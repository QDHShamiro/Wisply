use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::time::Instant;

use image::DynamicImage;
use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;

use super::super::web::sheet;
use super::scenes;
use super::watch::{Job, Kind};

fn jpeg(img: &DynamicImage, q: u8) -> Vec<u8> {
    let mut buf = Cursor::new(Vec::new());
    let _ = JpegEncoder::new_with_quality(&mut buf, q).encode_image(&img.to_rgb8());
    buf.into_inner()
}

fn old_fit(img: &DynamicImage, width: u32) -> DynamicImage {
    if std::env::var("MEASURE_TAG").is_ok_and(|t| t != "old") {
        return super::super::web::fitted(img, width);
    }
    if img.width() <= width {
        return img.clone();
    }
    let h = (f64::from(img.height()) * f64::from(width) / f64::from(img.width())).round().max(1.0) as u32;
    img.resize_exact(width, h, FilterType::Lanczos3)
}

fn tokens(img: &DynamicImage) -> u32 {
    img.width().div_ceil(28) * img.height().div_ceil(28)
}

fn job(path: &Path) -> Job {
    Job {
        kind: Kind::File(path.to_path_buf()),
        key: path.display().to_string(),
        lo: None,
        hi: None,
        at: Vec::new(),
        full: false,
        text: false,
        lang: None,
        local: false,
        count: 12,
        question: None,
        fps: None,
        frames: true,
    }
}

#[test]
#[ignore]
fn measure() {
    let tag = std::env::var("MEASURE_TAG").unwrap_or_else(|_| "old".into());
    let root = PathBuf::from(std::env::var("MEASURE_ROOT").expect("MEASURE_ROOT"));
    let out = root.join("out").join(&tag);
    let _ = fs::remove_dir_all(&out);
    fs::create_dir_all(&out).unwrap();
    let home = PathBuf::from(std::env::var("USERPROFILE").unwrap_or_default()).join(".wisply");
    let mut clips: Vec<PathBuf> = fs::read_dir(root.join("clips")).unwrap().flatten().map(|e| e.path()).collect();
    clips.sort();
    for clip in clips {
        let name = clip.file_name().unwrap().to_string_lossy().into_owned();
        let dir = std::env::temp_dir().join(format!("wisply-measure-{tag}-{name}"));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let j = job(&clip);
        let t0 = Instant::now();
        let meta = scenes::meta_of(&home, &j, &dir).unwrap();
        let (tiles, readable, how) = scenes::pictures(&home, &j, &meta, &dir, 0.0, None).unwrap();
        let imgs: Vec<DynamicImage> = tiles.iter().map(|(_, i)| DynamicImage::ImageRgb8(i.clone())).collect();
        let stamps: Vec<String> = tiles.iter().map(|(t, _)| super::tools::clock(*t)).collect();
        let sh = DynamicImage::ImageRgb8(sheet(&imgs, &stamps));
        let fit = old_fit(&sh, 1024);
        let bytes = jpeg(&fit, 70);
        let secs = t0.elapsed().as_secs_f64();
        let t1 = Instant::now();
        let text = if readable { scenes::read_text(&tiles) } else { Vec::new() };
        let ocr = t1.elapsed().as_secs_f64();
        let chars: usize = text.iter().map(String::len).sum();
        fs::write(out.join(format!("{name}.jpg")), &bytes).unwrap();
        println!(
            "CASE {name} how={how} tiles={} sheet={}x{} tokens={} bytes={} secs={secs:.2} ocr_secs={ocr:.2} ocr_chars={chars}",
            tiles.len(),
            fit.width(),
            fit.height(),
            tokens(&fit),
            bytes.len()
        );
        let _ = fs::remove_dir_all(&dir);
    }
    let shots = root.join("shots");
    let mut files: Vec<PathBuf> =
        fs::read_dir(&shots).map(|d| d.flatten().map(|e| e.path()).collect()).unwrap_or_default();
    files.sort();
    for f in files {
        let name = f.file_name().unwrap().to_string_lossy().into_owned();
        let t0 = Instant::now();
        let img = image::open(&f).unwrap();
        let fit = old_fit(&img, 1024);
        let bytes = jpeg(&fit, 70);
        let secs = t0.elapsed().as_secs_f64();
        fs::write(out.join(format!("shot-{name}.jpg")), &bytes).unwrap();
        println!(
            "SHOT {name} {}x{} -> {}x{} tokens={} bytes={} secs={secs:.3}",
            img.width(),
            img.height(),
            fit.width(),
            fit.height(),
            tokens(&fit),
            bytes.len()
        );
    }
}
