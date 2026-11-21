use std::{env, fs};

fn main() {
    let dir = env::args().nth(1).expect("dir arg");
    let (mut tin, mut tout, mut none, mut n) = (0usize, 0usize, 0usize, 0usize);
    let mut rows: Vec<(f64, String, usize, usize)> = Vec::new();
    for e in fs::read_dir(&dir).expect("read_dir") {
        let p = e.expect("entry").path();
        if p.extension().and_then(|x| x.to_str()) != Some("txt") {
            continue;
        }
        let Ok(text) = fs::read_to_string(&p) else { continue };
        n += 1;
        match wisply_squeeze::squeeze(&text) {
            Some(s) => {
                tin += s.chars_in;
                tout += s.chars_out;
                let r = s.chars_out as f64 / s.chars_in.max(1) as f64;
                rows.push((r, p.file_name().unwrap().to_string_lossy().into_owned(), s.chars_in, s.chars_out));
            }
            None => {
                none += 1;
                let c = text.chars().count();
                tin += c;
                tout += c;
            }
        }
    }
    rows.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    println!("files={n} squeezed={} untouched={none}", n - none);
    println!(
        "overall kept={:.1}% saved={:.1}% (chars_in={tin} chars_out={tout})",
        tout as f64 / tin.max(1) as f64 * 100.0,
        (1.0 - tout as f64 / tin.max(1) as f64) * 100.0
    );
    let med = rows.get(rows.len() / 2).map(|r| r.0).unwrap_or(0.0);
    println!("median kept (squeezed only) = {:.3}", med);
    println!("--- 12 least-compressed squeezed (engine barely helped) ---");
    for (r, name, ci, co) in rows.iter().take(12) {
        println!("kept {:.3}  {ci}->{co}  {name}", r);
    }
}
