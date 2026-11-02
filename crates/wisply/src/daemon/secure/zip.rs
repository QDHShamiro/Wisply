use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use flate2::read::DeflateDecoder;

const EOCD: u32 = 0x0605_4b50;
const CENTRAL: u32 = 0x0201_4b50;
const LOCAL: u32 = 0x0403_4b50;
const TAIL: u64 = 65_557;
const MAX_ENTRIES: usize = 200_000;

pub struct Entry {
    pub name: String,
    method: u16,
    packed: u64,
    pub size: u64,
    offset: u64,
}

pub struct Zip {
    file: File,
    pub entries: Vec<Entry>,
}

fn u16_at(b: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([b[at], b[at + 1]])
}

fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}

impl Zip {
    pub fn open(path: &Path) -> Result<Zip, String> {
        let mut file = File::open(path).map_err(|e| format!("cannot open {}: {e}", path.display()))?;
        let len = file.metadata().map_err(|e| e.to_string())?.len();
        let from = len.saturating_sub(TAIL);
        file.seek(SeekFrom::Start(from)).map_err(|e| e.to_string())?;
        let mut tail = Vec::new();
        file.read_to_end(&mut tail).map_err(|e| e.to_string())?;
        let at = (0..tail.len().saturating_sub(21))
            .rev()
            .find(|i| u32_at(&tail, *i) == EOCD)
            .ok_or("not a zip archive (jar/apk)")?;
        let count = usize::from(u16_at(&tail, at + 10));
        let (dir_size, dir_at) = (u64::from(u32_at(&tail, at + 12)), u64::from(u32_at(&tail, at + 16)));
        if dir_at + dir_size > len || count > MAX_ENTRIES {
            return Err("zip directory is damaged or too large".into());
        }
        file.seek(SeekFrom::Start(dir_at)).map_err(|e| e.to_string())?;
        let mut dir = vec![0u8; dir_size as usize];
        file.read_exact(&mut dir).map_err(|e| e.to_string())?;
        let mut entries = Vec::new();
        let mut p = 0;
        while p + 46 <= dir.len() && u32_at(&dir, p) == CENTRAL && entries.len() < count {
            let (nl, xl, cl) = (
                usize::from(u16_at(&dir, p + 28)),
                usize::from(u16_at(&dir, p + 30)),
                usize::from(u16_at(&dir, p + 32)),
            );
            let Some(raw) = dir.get(p + 46..p + 46 + nl) else { break };
            entries.push(Entry {
                name: String::from_utf8_lossy(raw).into_owned(),
                method: u16_at(&dir, p + 10),
                packed: u64::from(u32_at(&dir, p + 20)),
                size: u64::from(u32_at(&dir, p + 24)),
                offset: u64::from(u32_at(&dir, p + 42)),
            });
            p += 46 + nl + xl + cl;
        }
        Ok(Zip { file, entries })
    }

    pub fn read(&mut self, at: usize, limit: u64) -> Option<Vec<u8>> {
        let e = self.entries.get(at)?;
        if e.size > limit || e.name.ends_with('/') {
            return None;
        }
        let (offset, packed, method, size) = (e.offset, e.packed, e.method, e.size);
        self.file.seek(SeekFrom::Start(offset)).ok()?;
        let mut head = [0u8; 30];
        self.file.read_exact(&mut head).ok()?;
        if u32_at(&head, 0) != LOCAL {
            return None;
        }
        let skip = i64::from(u16_at(&head, 26)) + i64::from(u16_at(&head, 28));
        self.file.seek(SeekFrom::Current(skip)).ok()?;
        let mut data = Vec::new();
        let source = (&mut self.file).take(packed);
        match method {
            0 => {
                source.take(limit).read_to_end(&mut data).ok()?;
            }
            8 => {
                DeflateDecoder::new(source).take(limit.min(size.max(1))).read_to_end(&mut data).ok()?;
            }
            _ => return None,
        }
        Some(data)
    }
}

#[cfg(test)]
pub fn build(files: &[(&str, &[u8])]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut dir = Vec::new();
    for (name, data) in files {
        let offset = out.len() as u32;
        let hdr = |out: &mut Vec<u8>, sig: u32, central: bool| {
            out.extend(sig.to_le_bytes());
            if central {
                out.extend([20u8, 0]);
            }
            out.extend([20u8, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            out.extend(0u32.to_le_bytes());
            out.extend((data.len() as u32).to_le_bytes());
            out.extend((data.len() as u32).to_le_bytes());
            out.extend((name.len() as u16).to_le_bytes());
            out.extend([0u8, 0]);
            if central {
                out.extend([0u8; 10]);
                out.extend(offset.to_le_bytes());
            }
            out.extend(name.as_bytes());
        };
        hdr(&mut out, LOCAL, false);
        out.extend(*data);
        hdr(&mut dir, CENTRAL, true);
    }
    let at = out.len() as u32;
    out.extend(&dir);
    out.extend(EOCD.to_le_bytes());
    out.extend([0u8; 4]);
    out.extend((files.len() as u16).to_le_bytes());
    out.extend((files.len() as u16).to_le_bytes());
    out.extend((dir.len() as u32).to_le_bytes());
    out.extend(at.to_le_bytes());
    out.extend([0u8; 2]);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stored_entries_are_listed_and_read() {
        let path = std::env::temp_dir().join(format!("wsp-zip-{}.zip", std::process::id()));
        std::fs::write(&path, build(&[("plugin.yml", b"name: X\n"), ("a/b.txt", b"hello")])).unwrap();
        let mut zip = Zip::open(&path).unwrap();
        let names: Vec<&str> = zip.entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, ["plugin.yml", "a/b.txt"]);
        assert_eq!(zip.read(1, 100).unwrap(), b"hello");
        assert!(zip.read(1, 2).is_none());
        std::fs::remove_file(&path).unwrap();
    }
}
