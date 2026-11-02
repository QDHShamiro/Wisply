use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use serde_json::Value;

use super::app::scan_text;
use super::report::Report;
use super::rules;

const FILE_LIMIT: u64 = 2_000_000;
const HEADER_LIMIT: usize = 64_000_000;
const SCANNED: &[&str] = &["js", "mjs", "cjs", "ts", "json", "html", "yml", "yaml", "env", "properties", "txt"];

struct Item {
    path: String,
    offset: u64,
    size: u64,
}

fn walk(node: &Value, path: &str, out: &mut Vec<Item>) {
    let Some(files) = node["files"].as_object() else { return };
    for (name, child) in files {
        let here = if path.is_empty() { name.clone() } else { format!("{path}/{name}") };
        if child["files"].is_object() {
            walk(child, &here, out);
        } else if child["unpacked"].as_bool() != Some(true)
            && let (Some(offset), Some(size)) =
                (child["offset"].as_str().and_then(|o| o.parse().ok()), child["size"].as_u64())
        {
            out.push(Item { path: here, offset, size });
        }
    }
}

fn header(file: &mut File) -> Result<(Value, u64), String> {
    let mut head = [0u8; 16];
    file.read_exact(&mut head).map_err(|e| format!("not an asar archive ({e})"))?;
    let pickle = u32::from_le_bytes([head[4], head[5], head[6], head[7]]) as usize;
    let json_len = u32::from_le_bytes([head[12], head[13], head[14], head[15]]) as usize;
    if json_len == 0 || json_len > HEADER_LIMIT || json_len + 8 > pickle + 8 {
        return Err("not an asar archive (bad header)".into());
    }
    let mut json = vec![0u8; json_len];
    file.read_exact(&mut json).map_err(|e| e.to_string())?;
    let value = serde_json::from_slice(&json).map_err(|_| "asar header is not JSON")?;
    Ok((value, 8 + pickle as u64))
}

pub fn scan(path: &Path, report: &mut Report) -> Result<(), String> {
    let mut file = File::open(path).map_err(|e| format!("cannot open {}: {e}", path.display()))?;
    let (tree, base) = header(&mut file)?;
    let mut items = Vec::new();
    walk(&tree, "", &mut items);
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("app.asar").to_string();
    let mut read = 0;
    for item in &items {
        let ext = item.path.rsplit_once('.').map(|(_, e)| e.to_lowercase()).unwrap_or_default();
        let base_name = item.path.rsplit('/').next().unwrap_or("");
        if !(SCANNED.contains(&ext.as_str()) || base_name.starts_with(".env")) || item.size > FILE_LIMIT {
            continue;
        }
        let mut data = vec![0u8; item.size as usize];
        if file.seek(SeekFrom::Start(base + item.offset)).is_err() || file.read_exact(&mut data).is_err() {
            continue;
        }
        read += 1;
        let config = !rules::code_file(&item.path);
        scan_text(&mut report.found, &format!("{name}!{}", item.path), &String::from_utf8_lossy(&data), config, true);
    }
    report.checked.push(format!("{name}: {read} of {} files", items.len()));
    Ok(())
}

#[cfg(test)]
pub fn build(files: &[(&str, &str)]) -> Vec<u8> {
    let mut entries = serde_json::Map::new();
    let mut data: Vec<u8> = Vec::new();
    for (name, body) in files {
        entries.insert((*name).into(), serde_json::json!({"size": body.len(), "offset": data.len().to_string()}));
        data.extend(body.as_bytes());
    }
    let json = serde_json::json!({"files": entries}).to_string().into_bytes();
    let padded = json.len().div_ceil(4) * 4;
    let mut out = Vec::new();
    out.extend(4u32.to_le_bytes());
    out.extend(((padded + 8) as u32).to_le_bytes());
    out.extend(((padded + 4) as u32).to_le_bytes());
    out.extend((json.len() as u32).to_le_bytes());
    out.extend(&json);
    out.extend(vec![0u8; padded - json.len()]);
    out.extend(data);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asar_files_are_read_and_judged() {
        let path = std::env::temp_dir().join(format!("wsp-asar-{}.asar", std::process::id()));
        let main = "new BrowserWindow({ webPreferences: { nodeIntegration: true } })\n";
        std::fs::write(&path, build(&[("main.js", main), ("logo.png", "xx")])).unwrap();
        let mut report = Report::new("t");
        scan(&path, &mut report).unwrap();
        assert_eq!(report.found.len(), 1);
        assert!(report.found[0].file.ends_with("!main.js") && report.found[0].why.starts_with("Electron window"));
        assert_eq!(report.checked, [format!("{}: 1 of 2 files", path.file_name().unwrap().to_str().unwrap())]);
        std::fs::remove_file(&path).unwrap();
    }
}
