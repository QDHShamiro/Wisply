use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use crate::model::{Sample, short_name, size};
use crate::plat::{Platform, shell};

const SECOND_SAMPLE: Duration = Duration::from_millis(1500);
const LISTED: usize = 5;

pub fn parse_ping(text: &str) -> Option<u64> {
    let b = text.as_bytes();
    for (i, &c) in b.iter().enumerate() {
        if c != b'=' && c != b'<' {
            continue;
        }
        let rest = &text[i + 1..];
        let digits: String = rest.chars().take_while(|ch| ch.is_ascii_digit() || *ch == '.').collect();
        let after = rest[digits.len()..].trim_start();
        if !digits.is_empty() && after.to_lowercase().starts_with("ms") {
            let ms: f64 = digits.parse().ok()?;
            return Some(if c == b'<' { 1 } else { ms.round() as u64 });
        }
    }
    None
}

pub fn ping(host: &str, limit_ms: u64) -> Option<u64> {
    let wait = limit_ms.to_string();
    let args: Vec<&str> = if cfg!(windows) {
        vec!["-n", "1", "-w", &wait, host]
    } else if cfg!(target_os = "macos") {
        vec!["-c", "1", "-t", "2", host]
    } else {
        vec!["-c", "1", "-W", "2", host]
    };
    parse_ping(&shell::run("ping", &args, Duration::from_millis(limit_ms + 3000))?)
}

const ESTABLISHED: [&str; 7] =
    ["established", "hergestellt", "etablie", "établie", "establecida", "stabilita", "gevestigd"];

fn established(state: &str) -> bool {
    ESTABLISHED.contains(&state.to_lowercase().as_str())
}

pub fn parse_netstat(text: &str) -> Vec<(u32, usize)> {
    let mut counts: HashMap<u32, usize> = HashMap::new();
    for line in text.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.len() < 5 || !f[0].eq_ignore_ascii_case("tcp") || !established(f[3]) {
            continue;
        }
        if local(f[2]) {
            continue;
        }
        if let Ok(pid) = f[4].parse() {
            *counts.entry(pid).or_default() += 1;
        }
    }
    ranked(counts)
}

pub fn parse_ss(text: &str) -> Vec<(u32, usize)> {
    let mut counts: HashMap<u32, usize> = HashMap::new();
    for line in text.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.len() < 5 || !f[0].starts_with("ESTAB") || local(f[4]) {
            continue;
        }
        let pid = line.split("pid=").nth(1).and_then(|r| r.split([',', ')']).next()).and_then(|p| p.parse().ok());
        if let Some(pid) = pid {
            *counts.entry(pid).or_default() += 1;
        }
    }
    ranked(counts)
}

pub fn parse_lsof(text: &str) -> Vec<(u32, usize)> {
    let mut counts: HashMap<u32, usize> = HashMap::new();
    for line in text.lines().skip(1) {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.len() < 9 || !line.contains("ESTABLISHED") {
            continue;
        }
        let remote = f.iter().find(|x| x.contains("->")).and_then(|x| x.split("->").nth(1)).unwrap_or("");
        if local(remote) {
            continue;
        }
        if let Ok(pid) = f[1].parse() {
            *counts.entry(pid).or_default() += 1;
        }
    }
    ranked(counts)
}

fn local(addr: &str) -> bool {
    let host = addr.trim_start_matches('[');
    host.starts_with("127.") || host.starts_with("::1") || host.starts_with("0.0.0.0") || host.starts_with("[::")
}

fn ranked(counts: HashMap<u32, usize>) -> Vec<(u32, usize)> {
    let mut v: Vec<(u32, usize)> = counts.into_iter().collect();
    v.sort_by_key(|(pid, n)| (std::cmp::Reverse(*n), *pid));
    v
}

pub fn connections() -> Vec<(u32, usize)> {
    let wait = Duration::from_secs(15);
    if cfg!(windows) {
        shell::run("netstat", &["-ano", "-p", "TCP"], wait).map(|t| parse_netstat(&t)).unwrap_or_default()
    } else if cfg!(target_os = "macos") {
        shell::run("lsof", &["-i", "TCP", "-nP", "-sTCP:ESTABLISHED"], wait).map(|t| parse_lsof(&t)).unwrap_or_default()
    } else {
        shell::run("ss", &["-tnpH"], wait).map(|t| parse_ss(&t)).unwrap_or_default()
    }
}

pub struct Row {
    pub name: String,
    pub pid: u32,
    pub links: usize,
    pub moved: u64,
}

pub fn busiest(before: &Sample, after: &Sample, conns: &[(u32, usize)]) -> Vec<Row> {
    let mut rows: Vec<Row> = conns
        .iter()
        .filter_map(|(pid, links)| {
            let now = after.proc(*pid)?;
            let moved = before.proc(*pid).map_or(0, |b| now.io.saturating_sub(b.io));
            Some(Row { name: short_name(&now.name), pid: *pid, links: *links, moved })
        })
        .collect();
    rows.sort_by_key(|r| (std::cmp::Reverse(r.moved), std::cmp::Reverse(r.links)));
    rows
}

pub fn render(host: &str, ms: Option<u64>, rows: &[Row]) -> String {
    let head = match ms {
        Some(ms) => format!("pc net | ping {host} {ms} ms"),
        None => format!("pc net | no answer from {host}"),
    };
    let mut lines = vec![head];
    if rows.is_empty() {
        lines.push("no program with an open internet connection found".into());
    }
    for r in rows.iter().take(LISTED) {
        let moved =
            if r.moved > 0 { format!(", {} of disk or network data in 1.5 s", size(r.moved)) } else { String::new() };
        lines.push(format!("{} (pid {}) | {} connections{moved}", r.name, r.pid, r.links));
    }
    lines.push("the busiest program is the one with the most data moved; per-program network counters are not exposed by the OS, so this is an estimate".into());
    lines.join("\n")
}

pub fn report(p: &mut dyn Platform, host: &str, limit_ms: u64) -> String {
    let before = p.sample();
    let ms = ping(host, limit_ms);
    let conns = connections();
    thread::sleep(SECOND_SAMPLE);
    let after = p.sample();
    render(host, ms, &busiest(&before, &after, &conns))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::proc;
    use crate::model::Proc;

    #[test]
    fn ping_times_are_found_in_every_output_style() {
        assert_eq!(parse_ping("Antwort von 1.1.1.1: Bytes=32 Zeit=34ms TTL=57"), Some(34));
        assert_eq!(parse_ping("Reply from 1.1.1.1: bytes=32 time=12ms TTL=57"), Some(12));
        assert_eq!(parse_ping("Reply from 1.1.1.1: bytes=32 time<1ms TTL=57"), Some(1));
        assert_eq!(parse_ping("64 bytes from 1.1.1.1: icmp_seq=1 ttl=57 time=12.6 ms"), Some(13));
        assert_eq!(parse_ping("Request timed out."), None);
    }

    #[test]
    fn connections_are_counted_per_program_and_loopback_is_ignored() {
        let ns = "  Proto  Local Address          Foreign Address        State           PID\n  TCP    10.0.0.2:50123    142.250.1.1:443   ESTABLISHED     1234\n  TCP    10.0.0.2:50124    142.250.1.2:443   ESTABLISHED     1234\n  TCP    10.0.0.2:50125    10.1.1.1:443   ESTABLISHED     77\n  TCP    127.0.0.1:5000    127.0.0.1:50126   ESTABLISHED     77\n  TCP    0.0.0.0:80    0.0.0.0:0   LISTENING     4\n";
        assert_eq!(parse_netstat(ns), vec![(1234, 2), (77, 1)]);
        let de = "  TCP    10.0.0.2:5    1.2.3.4:443   HERGESTELLT     9
  TCP    10.0.0.2:6    1.2.3.4:443   ABWARTEND     9
";
        assert_eq!(parse_netstat(de), vec![(9, 1)]);
        let ss = "ESTAB 0 0 192.168.0.5:5000 1.1.1.1:443 users:((\"firefox\",pid=123,fd=40))\nESTAB 0 0 127.0.0.1:1 127.0.0.1:2 users:((\"x\",pid=5,fd=1))\nLISTEN 0 1 *:22 *:*";
        assert_eq!(parse_ss(ss), vec![(123, 1)]);
        let lsof = "COMMAND PID USER FD TYPE DEVICE SIZE/OFF NODE NAME\nSafari 321 me 10u IPv4 0x1 0t0 TCP 10.0.0.2:5->1.1.1.1:443 (ESTABLISHED)\n";
        assert_eq!(parse_lsof(lsof), vec![(321, 1)]);
    }

    #[test]
    fn the_busiest_program_is_the_one_that_moved_the_most() {
        let mk = |a: u64, b: u64| Sample {
            procs: vec![Proc { io: a, ..proc(1, 0, "a.exe", 1) }, Proc { io: b, ..proc(2, 0, "b.exe", 1) }],
            ..Sample::default()
        };
        let rows = busiest(&mk(0, 0), &mk(10, 5000), &[(1, 9), (2, 1), (99, 4)]);
        assert_eq!(rows.iter().map(|r| r.name.as_str()).collect::<Vec<_>>(), ["b", "a"]);
        let text = render("host", Some(34), &rows);
        assert!(text.starts_with("pc net | ping host 34 ms\nb (pid 2) | 1 connections, 4 KB"), "{text}");
        assert!(render("h", None, &[]).contains("no answer from h"));
    }
}
