use std::fs;
use std::path::Path;

use image::{GrayImage, Luma};
use qrcode::render::unicode::Dense1x2;
use qrcode::{Color, QrCode};

const SCALE: u32 = 10;
const QUIET: u32 = 4;
const FILE: &str = "pair.png";

pub fn encode_component(text: &str) -> String {
    text.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => (b as char).to_string(),
            _ => format!("%{b:02X}"),
        })
        .collect()
}

pub fn pair_link(url: &str, code: &str) -> String {
    format!("wisply://pair?u={}&c={}", encode_component(url), encode_component(code))
}

pub fn terminal(data: &str) -> Option<String> {
    let code = QrCode::new(data.as_bytes()).ok()?;
    Some(code.render::<Dense1x2>().quiet_zone(true).build())
}

pub fn png(data: &str, dir: &Path) -> Option<std::path::PathBuf> {
    let code = QrCode::new(data.as_bytes()).ok()?;
    let width = code.width() as u32;
    let size = (width + QUIET * 2) * SCALE;
    let colors = code.to_colors();
    let img = GrayImage::from_fn(size, size, |x, y| {
        let (mx, my) = (x / SCALE, y / SCALE);
        let inside = mx >= QUIET && my >= QUIET && mx < width + QUIET && my < width + QUIET;
        let dark = inside && colors[((my - QUIET) * width + (mx - QUIET)) as usize] == Color::Dark;
        Luma([if dark { 0 } else { 255 }])
    });
    fs::create_dir_all(dir).ok()?;
    let path = dir.join(FILE);
    img.save(&path).ok()?;
    Some(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_link_carries_the_url_and_code_escaped() {
        let link = pair_link("http://100.64.1.2:47841", "ab12");
        assert_eq!(link, "wisply://pair?u=http%3A%2F%2F100.64.1.2%3A47841&c=ab12");
        assert!(!link[8..].contains("//"));
    }

    #[test]
    fn a_terminal_and_a_png_picture_are_made() {
        let link = pair_link("http://127.0.0.1:47841", &"f".repeat(32));
        let text = terminal(&link).unwrap();
        assert!(text.lines().count() > 10 && text.contains('█') || text.contains('▀') || text.contains('▄'));
        let dir = std::env::temp_dir().join(format!("wisply-agents-qr-{}", std::process::id()));
        let path = png(&link, &dir).unwrap();
        let img = image::open(&path).unwrap().to_luma8();
        assert!(img.width() == img.height() && img.width() > 200);
        assert!(img.pixels().any(|p| p.0[0] == 0) && img.pixels().any(|p| p.0[0] == 255));
        let _ = fs::remove_dir_all(&dir);
    }
}
