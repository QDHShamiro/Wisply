use std::fs::Metadata;
use std::path::Path;

#[cfg(unix)]
pub fn shared(_path: &Path, meta: &Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;
    meta.is_file() && meta.nlink() > 1
}

#[cfg(windows)]
#[repr(C)]
#[derive(Default)]
struct FileInfo {
    attributes: u32,
    created: [u32; 2],
    accessed: [u32; 2],
    written: [u32; 2],
    volume: u32,
    size_high: u32,
    size_low: u32,
    links: u32,
    index_high: u32,
    index_low: u32,
}

#[cfg(windows)]
#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetFileInformationByHandle(handle: isize, info: *mut FileInfo) -> i32;
}

#[cfg(windows)]
pub fn shared(path: &Path, meta: &Metadata) -> bool {
    use std::os::windows::io::AsRawHandle;
    if !meta.is_file() {
        return false;
    }
    let Ok(file) = std::fs::File::open(path) else { return false };
    let mut info = FileInfo::default();
    let ok = unsafe { GetFileInformationByHandle(file.as_raw_handle() as isize, &mut info) };
    ok != 0 && info.links > 1
}

#[cfg(not(any(unix, windows)))]
pub fn shared(_path: &Path, _meta: &Metadata) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn a_second_name_for_the_same_file_counts_as_shared() {
        let dir = std::env::temp_dir().join(format!("wpc-links-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let (a, b, c) = (dir.join("a"), dir.join("b"), dir.join("c"));
        fs::write(&a, b"x").unwrap();
        fs::write(&c, b"y").unwrap();
        fs::hard_link(&a, &b).unwrap();
        assert!(shared(&a, &fs::symlink_metadata(&a).unwrap()));
        assert!(shared(&b, &fs::symlink_metadata(&b).unwrap()));
        assert!(!shared(&c, &fs::symlink_metadata(&c).unwrap()));
        fs::remove_dir_all(&dir).unwrap();
    }
}
