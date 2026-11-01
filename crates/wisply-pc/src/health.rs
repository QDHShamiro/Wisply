use std::time::Duration;

use crate::model::{Sample, gb};
use crate::plat::shell;

const WAIT: Duration = Duration::from_secs(25);
const SEARCH_WAIT: Duration = Duration::from_secs(90);
const OLD_DRIVER_DAYS: i64 = 365;
const UTF8: &str = "[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; ";
const DISKS_PS: &str = "Get-PhysicalDisk | ForEach-Object { $_.FriendlyName + '|' + $_.MediaType + '|' + $_.HealthStatus + '|' + $_.OperationalStatus + '|' + $_.Size }";
const GPU_PS: &str = "Get-CimInstance Win32_VideoController | ForEach-Object { $_.Name + '|' + $_.DriverVersion + '|' + $_.DriverDate.ToString('yyyyMMdd') }";
const UPDATES_PS: &str = "$s = New-Object -ComObject Microsoft.Update.Session; $r = $s.CreateUpdateSearcher().Search('IsInstalled=0 and IsHidden=0'); $r.Updates | ForEach-Object { $_.Title }";

#[derive(Debug, PartialEq)]
pub struct Drive {
    pub name: String,
    pub media: String,
    pub health: String,
    pub size: u64,
}

pub fn parse_drives(text: &str) -> Vec<Drive> {
    text.lines()
        .filter_map(|l| {
            let f: Vec<&str> = l.trim().split('|').collect();
            (f.len() >= 5).then(|| Drive {
                name: f[0].trim().to_string(),
                media: f[1].trim().to_string(),
                health: f[2].trim().to_string(),
                size: f[4].trim().parse().unwrap_or(0),
            })
        })
        .collect()
}

pub fn parse_lsblk(text: &str) -> Vec<Drive> {
    text.lines()
        .filter_map(|l| {
            let f: Vec<&str> = l.split_whitespace().collect();
            if f.len() < 3 {
                return None;
            }
            let model = f[3..].join(" ");
            Some(Drive {
                name: if model.is_empty() { f[0].to_string() } else { model },
                media: if f[1] == "1" { "HDD".into() } else { "SSD".into() },
                health: "unknown (install smartmontools for SMART)".into(),
                size: f[2].parse().unwrap_or(0),
            })
        })
        .collect()
}

pub fn parse_diskutil(text: &str) -> Option<String> {
    text.lines().find(|l| l.contains("SMART Status")).map(|l| l.split(':').nth(1).unwrap_or("").trim().to_string())
}

fn powershell(script: &str, limit: Duration) -> Option<String> {
    let full = format!("{UTF8}{script}");
    shell::run("powershell", &["-NoProfile", "-NonInteractive", "-Command", &full], limit)
}

pub fn disks() -> Vec<Drive> {
    if cfg!(windows) {
        powershell(DISKS_PS, WAIT).map(|t| parse_drives(&t)).unwrap_or_default()
    } else if cfg!(target_os = "macos") {
        let status = shell::run("diskutil", &["info", "disk0"], WAIT)
            .and_then(|t| parse_diskutil(&t))
            .unwrap_or_else(|| "unknown".into());
        vec![Drive { name: "disk0".into(), media: String::new(), health: status, size: 0 }]
    } else {
        shell::run("lsblk", &["-dnb", "-o", "NAME,ROTA,SIZE,MODEL"], WAIT).map(|t| parse_lsblk(&t)).unwrap_or_default()
    }
}

pub fn render_disks(drives: &[Drive], s: &Sample) -> String {
    let mut lines = vec![format!("pc disk | {} drives", drives.len())];
    for d in drives {
        let size = if d.size > 0 { format!(" | {}", gb(d.size)) } else { String::new() };
        let slow =
            if d.media.eq_ignore_ascii_case("hdd") { " | HDD: a busy disk is the usual cause of lag here" } else { "" };
        lines.push(format!(
            "{} | {} | health {}{size}{slow}",
            d.name,
            if d.media.is_empty() { "?" } else { &d.media },
            d.health
        ));
    }
    for d in &s.disks {
        let used = if d.total > 0 { (d.total - d.free.min(d.total)) as f64 * 100.0 / d.total as f64 } else { 0.0 };
        lines.push(format!("{} | {} free of {} ({used:.0} % used)", d.mount, gb(d.free), gb(d.total)));
    }
    if let Some(b) = s.disk_busy {
        lines.push(format!("activity right now | {b:.0} %"));
    }
    if drives.is_empty() {
        lines.push("could not read drive health on this system".into());
    }
    lines.join("\n")
}

pub fn driver_age_days(date: &str, today_days: i64) -> Option<i64> {
    let y: i64 = date.get(0..4)?.parse().ok()?;
    let m: i64 = date.get(4..6)?.parse().ok()?;
    let d: i64 = date.get(6..8)?.parse().ok()?;
    Some(today_days - days_from_civil(y, m, d))
}

pub fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let doy = (153 * (m + if m > 2 { -3 } else { 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

pub fn parse_gpus(text: &str, today_days: i64) -> Vec<String> {
    text.lines()
        .filter_map(|l| {
            let f: Vec<&str> = l.trim().split('|').collect();
            if f.len() < 3 {
                return None;
            }
            let age = driver_age_days(f[2].trim(), today_days);
            let note = match age {
                Some(a) if a > OLD_DRIVER_DAYS => format!("{} days old, look for a newer one from the maker", a),
                Some(a) => format!("{a} days old, fine"),
                None => "date unknown".into(),
            };
            Some(format!("{} | driver {} | {note}", f[0].trim(), f[1].trim()))
        })
        .collect()
}

pub fn parse_titles(text: &str) -> Vec<String> {
    text.lines().map(str::trim).filter(|l| !l.is_empty()).map(String::from).collect()
}

pub fn parse_softwareupdate(text: &str) -> Vec<String> {
    text.lines().filter_map(|l| l.trim().strip_prefix("* Label:").map(|r| r.trim().to_string())).collect()
}

pub fn parse_apt(text: &str) -> Vec<String> {
    text.lines().filter(|l| l.contains("[upgradable")).map(|l| l.split('/').next().unwrap_or("").to_string()).collect()
}

pub fn updates(today_days: i64) -> String {
    let mut lines = vec!["pc updates | read only, nothing is installed".to_string()];
    if cfg!(windows) {
        let gpu = powershell(GPU_PS, WAIT).map(|t| parse_gpus(&t, today_days));
        lines.extend(gpu.unwrap_or_default().into_iter().map(|g| format!("graphics | {g}")));
        match powershell(UPDATES_PS, SEARCH_WAIT) {
            Some(t) => {
                let list = parse_titles(&t);
                lines.push(format!("windows update | {} open", list.len()));
                lines.extend(list.iter().take(8).map(|u| format!("  {u}")));
            }
            None => lines.push("windows update | could not ask (timed out or blocked)".into()),
        }
    } else if cfg!(target_os = "macos") {
        let open = shell::run("softwareupdate", &["-l"], SEARCH_WAIT).map(|t| parse_softwareupdate(&t));
        match open {
            Some(list) => {
                lines.push(format!("software update | {} open", list.len()));
                lines.extend(list.iter().take(8).map(|u| format!("  {u}")));
            }
            None => lines.push("software update | could not ask".into()),
        }
    } else {
        if let Some(t) = shell::run("nvidia-smi", &["--query-gpu=name,driver_version", "--format=csv,noheader"], WAIT) {
            lines.push(format!("graphics | {}", t.trim()));
        }
        match shell::run("apt", &["list", "--upgradable"], SEARCH_WAIT) {
            Some(t) => {
                let list = parse_apt(&t);
                lines.push(format!("apt | {} open", list.len()));
                lines.extend(list.iter().take(8).map(|u| format!("  {u}")));
            }
            None => lines.push("system updates | apt not found, check your package manager".into()),
        }
    }
    lines.push("to install, use your system's updater; Wisply only shows what is open".into());
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drive_health_lines_are_read() {
        let d =
            parse_drives("Samsung SSD 980|SSD|Healthy|OK|500107862016\nSeagate|HDD|Warning|OK|2000398934016\n\nbad");
        assert_eq!(d.len(), 2);
        assert_eq!(
            (d[0].name.as_str(), d[0].media.as_str(), d[0].health.as_str()),
            ("Samsung SSD 980", "SSD", "Healthy")
        );
        let s = Sample {
            disks: vec![crate::model::Disk { mount: "C:".into(), total: 500 << 30, free: 100 << 30 }],
            disk_busy: Some(12.0),
            ..Sample::default()
        };
        let text = render_disks(&d, &s);
        assert!(
            text.contains("Seagate | HDD | health Warning") && text.contains("busy disk is the usual cause"),
            "{text}"
        );
        assert!(
            text.contains("C: | 100.0 GB free of 500.0 GB (80 % used)") && text.contains("activity right now | 12 %"),
            "{text}"
        );
        assert_eq!(parse_diskutil("   SMART Status:             Verified\n").as_deref(), Some("Verified"));
        let lsblk = parse_lsblk("sda 1 1000204886016 WDC WD10\nnvme0n1 0 500107862016 Samsung SSD\n");
        assert_eq!(
            (lsblk[0].media.as_str(), lsblk[1].media.as_str(), lsblk[0].name.as_str()),
            ("HDD", "SSD", "WDC WD10")
        );
    }

    #[test]
    fn driver_age_is_counted_in_days() {
        let today = days_from_civil(2026, 11, 1);
        assert_eq!(driver_age_days("20251101", today), Some(365));
        assert_eq!(driver_age_days("20260101", today), Some(304));
        let lines = parse_gpus("NVIDIA GeForce RTX|32.0.15.6094|20240105\nOld GPU|1.0|20260920", today);
        assert!(
            lines[0].contains("days old, look for a newer one") && lines[1].contains("42 days old, fine"),
            "{lines:?}"
        );
        assert_eq!(days_from_civil(1970, 1, 1), 0);
    }

    #[test]
    fn update_lists_are_read() {
        assert_eq!(parse_titles("KB1 Security\n\nKB2 Driver \n"), ["KB1 Security", "KB2 Driver"]);
        assert_eq!(parse_softwareupdate("   * Label: macOS 15.1-24B83\n\tTitle: x\n"), ["macOS 15.1-24B83"]);
        assert_eq!(parse_apt("Listing...\nbash/stable 5.2 amd64 [upgradable from: 5.1]\n"), ["bash"]);
    }
}
