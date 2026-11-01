pub const GB: u64 = 1 << 30;
pub const MB: u64 = 1 << 20;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Proc {
    pub pid: u32,
    pub ppid: u32,
    pub name: String,
    pub exe: String,
    pub rss: u64,
    pub cpu_ms: u64,
    pub io: u64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Disk {
    pub mount: String,
    pub total: u64,
    pub free: u64,
}

#[derive(Clone, Debug, Default)]
pub struct Sample {
    pub at_ms: u64,
    pub mem_total: u64,
    pub mem_used: u64,
    pub cpu: Option<(u64, u64)>,
    pub cores: u32,
    pub disk_busy: Option<f32>,
    pub idle_secs: Option<u64>,
    pub foreground: Option<u32>,
    pub procs: Vec<Proc>,
    pub disks: Vec<Disk>,
}

impl Sample {
    pub fn mem_pct(&self) -> f32 {
        if self.mem_total == 0 { 0.0 } else { self.mem_used as f32 * 100.0 / self.mem_total as f32 }
    }

    pub fn by_name(&self) -> Vec<(String, u64)> {
        let mut sums: Vec<(String, u64)> = Vec::new();
        for p in &self.procs {
            let key = p.name.to_lowercase();
            match sums.iter_mut().find(|(n, _)| *n == key) {
                Some((_, sum)) => *sum += p.rss,
                None => sums.push((key, p.rss)),
            }
        }
        sums.sort_by_key(|(_, rss)| std::cmp::Reverse(*rss));
        sums
    }

    pub fn proc(&self, pid: u32) -> Option<&Proc> {
        self.procs.iter().find(|p| p.pid == pid)
    }

    pub fn descendants(&self, root: u32) -> Vec<u32> {
        let mut out = vec![root];
        let mut i = 0;
        while i < out.len() {
            let parent = out[i];
            for p in &self.procs {
                if p.ppid == parent && !out.contains(&p.pid) {
                    out.push(p.pid);
                }
            }
            i += 1;
        }
        out
    }
}

pub fn short_name(name: &str) -> String {
    let lower = name.to_lowercase();
    lower.strip_suffix(".exe").unwrap_or(&lower).to_string()
}

pub fn gb(bytes: u64) -> String {
    format!("{:.1} GB", bytes as f64 / GB as f64)
}

pub fn size(bytes: u64) -> String {
    if bytes >= GB {
        gb(bytes)
    } else if bytes >= MB {
        format!("{} MB", bytes / MB)
    } else if bytes >= 1024 {
        format!("{} KB", bytes / 1024)
    } else {
        format!("{bytes} B")
    }
}

pub fn secs() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn processes_add_up_by_name_and_tree() {
        let p = |pid, ppid, name: &str, rss| Proc { pid, ppid, name: name.into(), rss, ..Proc::default() };
        let s = Sample {
            procs: vec![p(1, 0, "Brave.exe", 3), p(2, 1, "brave.exe", 4), p(3, 2, "gpu", 1), p(4, 0, "x", 9)],
            ..Sample::default()
        };
        assert_eq!(s.by_name()[0], ("x".into(), 9));
        assert_eq!(s.by_name()[1], ("brave.exe".into(), 7));
        assert_eq!(s.descendants(1), vec![1, 2, 3]);
        assert_eq!(short_name("Cargo.EXE"), "cargo");
        assert_eq!(size(3 * GB / 2), "1.5 GB");
        assert_eq!(size(5 * MB), "5 MB");
        assert_eq!(size(999), "999 B");
    }
}
