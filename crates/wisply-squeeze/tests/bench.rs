use std::fs;
use std::path::PathBuf;
use std::time::Instant;

const REPS: usize = 300;
const SMALLEST_REAL: usize = 3000;

fn real() -> Vec<(String, String)> {
    let dir = match std::env::var_os("WISPLY_BENCH_DIR") {
        Some(d) => PathBuf::from(d),
        None => PathBuf::from(std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_default())
            .join(".wisply")
            .join("headroom")
            .join("out"),
    };
    let mut files: Vec<_> = fs::read_dir(&dir).map(|d| d.flatten().map(|e| e.path()).collect()).unwrap_or_default();
    files.sort();
    files
        .into_iter()
        .filter_map(|p| {
            let text = fs::read_to_string(&p).ok()?;
            if text.len() < SMALLEST_REAL {
                return None;
            }
            wisply_squeeze::squeeze(&text)?;
            Some((p.file_name()?.to_string_lossy().into_owned(), text))
        })
        .collect()
}

fn made() -> Vec<(String, String)> {
    let mut v = Vec::new();
    v.push((
        "gen-ticks".into(),
        (0..5000).map(|i| format!("[12:00:{:02} INFO]: tick {i}", i % 60)).collect::<Vec<_>>().join("\n"),
    ));
    let mut mvn: Vec<String> =
        (0..800).map(|i| format!("Downloaded from central: https://repo/x{i}.pom (2 kB at 9 kB/s)")).collect();
    mvn.insert(400, "[ERROR] Failed to execute goal compile on project shop".into());
    v.push(("gen-maven".into(), mvn.join("\n")));
    let mut trace: Vec<String> = (0..300).map(|i| format!("[21:00:{:02} INFO]: row {i}", i % 60)).collect();
    trace.push("java.lang.NullPointerException: x".into());
    trace.extend((0..400).map(|i| format!("\tat java.base/java.lang.reflect.M{i}.invoke(M.java:{i})")));
    v.push(("gen-trace".into(), trace.join("\n")));
    let rows: Vec<String> =
        (0..2000).map(|i| format!("{{\"id\":{i},\"name\":\"user{i}\",\"error\":null,\"score\":{}}}", i % 97)).collect();
    v.push(("gen-jsonl".into(), rows.join("\n")));
    let arr: Vec<String> = (0..1500).map(|i| format!("{{\"id\":{i},\"ok\":true,\"msg\":\"item {i}\"}}")).collect();
    v.push(("gen-json".into(), format!("[{}]", arr.join(","))));
    v.push((
        "gen-ansi".into(),
        (0..3000).map(|i| format!("\x1b[32m{i:05}\x1b[0m test case_{i} ... ok")).collect::<Vec<_>>().join("\r\n"),
    ));
    v.push((
        "gen-long".into(),
        (0..600).map(|i| format!("line {i} {}", "word ".repeat(200))).collect::<Vec<_>>().join("\n"),
    ));
    v
}

fn pct(sorted: &[f64], p: f64) -> f64 {
    sorted[((sorted.len() - 1) as f64 * p).round() as usize]
}

#[test]
#[ignore]
fn squeeze_speed() {
    let mut set = real();
    let n_real = set.len();
    set.extend(made());
    let (mut times, mut chars) = (Vec::new(), 0usize);
    println!("{:<18} {:>9} {:>9} {:>9}", "input", "bytes", "med_ms", "chars_out");
    for (name, text) in &set {
        let mut runs = Vec::with_capacity(REPS);
        let mut out = 0;
        for _ in 0..REPS {
            let t = Instant::now();
            let s = wisply_squeeze::squeeze(std::hint::black_box(text));
            runs.push(t.elapsed().as_secs_f64() * 1000.0);
            out = s.map_or(wisply_squeeze::clean(text).chars().count(), |s| s.chars_out);
        }
        runs.sort_by(f64::total_cmp);
        let med = pct(&runs, 0.5);
        println!("{name:<18} {:>9} {med:>9.3} {out:>9}", text.len());
        times.push(med);
        chars += out;
    }
    let mut sorted = times.clone();
    sorted.sort_by(f64::total_cmp);
    println!(
        "inputs={} (real {n_real}) max_ms={:.3} median_ms={:.3} chars_out_total={chars}",
        set.len(),
        sorted[sorted.len() - 1],
        pct(&sorted, 0.5)
    );
}
