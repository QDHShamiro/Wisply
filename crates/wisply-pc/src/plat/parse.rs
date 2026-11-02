use crate::model::{Disk, Proc};

const KB: u64 = 1024;

pub fn proc_stat(text: &str) -> Option<(String, u32, u64)> {
    let open = text.find('(')?;
    let close = text.rfind(')')?;
    let name = text.get(open + 1..close)?.to_string();
    let rest: Vec<&str> = text.get(close + 2..)?.split_whitespace().collect();
    let ppid = rest.get(1)?.parse().ok()?;
    let ticks = rest.get(11)?.parse::<u64>().ok()? + rest.get(12)?.parse::<u64>().ok()?;
    Some((name, ppid, ticks))
}

pub fn meminfo(text: &str) -> Option<(u64, u64)> {
    let field = |key: &str| {
        let line = text.lines().find(|l| l.starts_with(key))?;
        line.split_whitespace().nth(1)?.parse::<u64>().ok().map(|kb| kb * KB)
    };
    Some((field("MemTotal:")?, field("MemAvailable:")?))
}

pub fn cpu_line(text: &str) -> Option<(u64, u64)> {
    let line = text.lines().find(|l| l.starts_with("cpu "))?;
    let n: Vec<u64> = line.split_whitespace().skip(1).filter_map(|v| v.parse().ok()).collect();
    if n.len() < 4 {
        return None;
    }
    let total: u64 = n.iter().take(8).sum();
    let idle = n[3] + n.get(4).copied().unwrap_or(0);
    Some((total.saturating_sub(idle), total))
}

fn whole_disk(name: &str) -> bool {
    let tail = name.chars().last().is_some_and(|c| c.is_ascii_digit());
    match name {
        n if n.starts_with("nvme") || n.starts_with("mmcblk") => !n.contains('p'),
        n if ["sd", "vd", "hd", "xvd"].iter().any(|p| n.starts_with(p)) => !tail,
        _ => false,
    }
}

pub fn io_ticks(text: &str) -> u64 {
    text.lines()
        .filter_map(|l| {
            let f: Vec<&str> = l.split_whitespace().collect();
            whole_disk(f.get(2)?).then(|| f.get(12)?.parse::<u64>().ok()).flatten()
        })
        .max()
        .unwrap_or(0)
}

fn clock(text: &str) -> u64 {
    let (days, rest) = match text.split_once('-') {
        Some((d, r)) => (d.parse::<u64>().unwrap_or(0), r),
        None => (0, text),
    };
    let mut secs = 0.0f64;
    for part in rest.split(':') {
        secs = secs * 60.0 + part.parse::<f64>().unwrap_or(0.0);
    }
    ((days * 86_400) as f64 * 1000.0 + secs * 1000.0) as u64
}

pub fn ps_rows(text: &str) -> Vec<Proc> {
    text.lines()
        .filter_map(|line| {
            let mut it = line.split_whitespace();
            let pid = it.next()?.parse().ok()?;
            let ppid = it.next()?.parse().ok()?;
            let rss = it.next()?.parse::<u64>().ok()? * KB;
            let cpu_ms = clock(it.next()?);
            let exe = it.collect::<Vec<_>>().join(" ");
            let name = exe.rsplit('/').next().unwrap_or(&exe).to_string();
            Some(Proc { pid, ppid, name, exe, rss, cpu_ms, io: 0 })
        })
        .collect()
}

pub fn df_rows(text: &str) -> Vec<Disk> {
    let mut seen: Vec<String> = Vec::new();
    text.lines()
        .skip(1)
        .filter_map(|line| {
            let f: Vec<&str> = line.split_whitespace().collect();
            if f.len() < 6 || !f[0].starts_with("/dev/") || seen.iter().any(|s| s == f[0]) {
                return None;
            }
            seen.push(f[0].to_string());
            let total = f[1].parse::<u64>().ok()? * KB;
            let free = f[3].parse::<u64>().ok()? * KB;
            Some(Disk { mount: f[5..].join(" "), total, free })
        })
        .collect()
}

pub fn vm_stat(text: &str, total: u64) -> Option<u64> {
    let page: u64 = text.lines().next()?.split("page size of ").nth(1)?.split_whitespace().next()?.parse().ok()?;
    let pages = |key: &str| {
        let line = text.lines().find(|l| l.starts_with(key))?;
        line.split(':').nth(1)?.trim().trim_end_matches('.').parse::<u64>().ok()
    };
    let free = pages("Pages free")? + pages("Pages inactive").unwrap_or(0) + pages("Pages speculative").unwrap_or(0);
    Some(total.saturating_sub(free * page))
}

pub fn hid_idle(text: &str) -> Option<u64> {
    let line = text.lines().find(|l| l.contains("HIDIdleTime"))?;
    let ns: u64 = line.split('=').nth(1)?.trim().parse().ok()?;
    Some(ns / 1_000_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linux_proc_files_are_read_even_with_odd_names() {
        let stat = "4242 (my (odd) app) S 1 4242 4242 0 -1 4194560 100 0 0 0 150 50 0 0 20 0 3 0 100 1000 200 0";
        assert_eq!(proc_stat(stat), Some(("my (odd) app".into(), 1, 200)));
        assert_eq!(proc_stat("garbage"), None);
        let mem = "MemTotal:       16000000 kB\nMemFree:  100 kB\nMemAvailable:    4000000 kB\n";
        assert_eq!(meminfo(mem), Some((16_000_000 * 1024, 4_000_000 * 1024)));
        assert_eq!(cpu_line("cpu  100 0 100 700 100 0 0 0 0 0\ncpu0 1 1 1 1"), Some((200, 1000)));
    }

    #[test]
    fn disk_activity_counts_whole_disks_only() {
        let t = "   8       0 sda 1 0 1 1 1 0 1 1 0 500 1\n   8       1 sda1 1 0 1 1 1 0 1 1 0 900 1\n   7 0 loop0 1 0 1 1 1 0 1 1 0 700 1\n 259 0 nvme0n1 1 0 1 1 1 0 1 1 0 300 1\n 259 1 nvme0n1p1 1 0 1 1 1 0 1 1 0 800 1";
        assert_eq!(io_ticks(t), 500);
    }

    #[test]
    fn mac_ps_df_vm_and_ioreg_are_read() {
        let ps = "  1     0   5000  0:01.50 /sbin/launchd\n 99     1 204800 1-02:03:04 /Applications/My App.app/Contents/MacOS/My App\nbad line\n";
        let procs = ps_rows(ps);
        assert_eq!(procs.len(), 2);
        assert_eq!((procs[0].name.as_str(), procs[0].rss, procs[0].cpu_ms), ("launchd", 5000 * 1024, 1500));
        assert_eq!(procs[1].name, "My App");
        assert_eq!(procs[1].cpu_ms, (86_400 + 2 * 3600 + 3 * 60 + 4) * 1000);
        let df = "Filesystem 1024-blocks Used Available Capacity Mounted on\n/dev/disk3s1 500000 300000 200000 60% /\n/dev/disk3s1 500000 300000 200000 60% /System/Volumes/Data\nmap auto 0 0 0 100% /home\n";
        let disks = df_rows(df);
        assert_eq!(disks, vec![Disk { mount: "/".into(), total: 500_000 * 1024, free: 200_000 * 1024 }]);
        let vm = "Mach Virtual Memory Statistics: (page size of 16384 bytes)\nPages free:                               1000.\nPages inactive:                           500.\nPages speculative:                        100.\n";
        assert_eq!(vm_stat(vm, 10_000 * 16384), Some(8400 * 16384));
        let ioreg = "    | |   \"HIDIdleTime\" = 7500000000";
        assert_eq!(hid_idle(ioreg), Some(7));
    }
}
