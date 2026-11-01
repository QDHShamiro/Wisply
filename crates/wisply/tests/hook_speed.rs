use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Instant;

const REPS: usize = 25;
const SMALLEST_REAL: u64 = 3000;

fn fnv(s: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.bytes() {
        h = (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3);
    }
    h
}

fn session() -> String {
    (0..).map(|i| format!("s{i}")).find(|s| !fnv(s).is_multiple_of(10)).expect("session")
}

fn real() -> Vec<(String, String)> {
    let home = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_default();
    let dir = PathBuf::from(home).join(".wisply").join("headroom").join("out");
    let mut files: Vec<_> = fs::read_dir(&dir).map(|d| d.flatten().map(|e| e.path()).collect()).unwrap_or_default();
    files.sort();
    files
        .into_iter()
        .filter(|p| fs::metadata(p).is_ok_and(|m| m.len() >= SMALLEST_REAL))
        .filter_map(|p| Some((p.file_name()?.to_string_lossy().into_owned(), fs::read_to_string(&p).ok()?)))
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
        (0..800).map(|i| format!("{{\"id\":{i},\"name\":\"user{i}\",\"error\":null,\"score\":{}}}", i % 97)).collect();
    v.push(("gen-jsonl".into(), rows.join("\n")));
    let arr: Vec<String> = (0..600).map(|i| format!("{{\"id\":{i},\"ok\":true,\"msg\":\"item {i}\"}}")).collect();
    v.push(("gen-json".into(), format!("[{}]", arr.join(","))));
    v.push((
        "gen-ansi".into(),
        (0..1500).map(|i| format!("\x1b[32m{i:05}\x1b[0m test case_{i} ... ok")).collect::<Vec<_>>().join("\r\n"),
    ));
    v.push((
        "gen-long".into(),
        (0..200).map(|i| format!("line {i} {}", "word ".repeat(100))).collect::<Vec<_>>().join("\n"),
    ));
    v
}

#[test]
#[ignore]
fn hook_process_time() {
    let home = std::env::temp_dir().join(format!("wisply-hook-speed-{}", std::process::id()));
    let _ = fs::remove_dir_all(&home);
    fs::create_dir_all(&home).expect("home");
    let sid = session();
    let mut set = real();
    let n_real = set.len();
    set.extend(made());
    let mut all = Vec::new();
    println!("{:<18} {:>9} {:>9} {:>9}", "input", "bytes", "med_ms", "p95_ms");
    for (name, text) in &set {
        let mut times = Vec::new();
        for rep in 0..REPS + 3 {
            let event = serde_json::json!({
                "session_id": sid, "tool_name": "Bash", "tool_use_id": format!("toolu_{name}_{rep}"),
                "tool_input": {"command": "npm test 2>&1"},
                "tool_response": {"stdout": text, "stderr": "", "interrupted": false, "isImage": false}
            })
            .to_string();
            let t = Instant::now();
            let mut child = Command::new(env!("CARGO_BIN_EXE_wisply"))
                .args(["hook", "post"])
                .env("WISPLY_HOME", &home)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
                .expect("spawn");
            child.stdin.take().expect("stdin").write_all(event.as_bytes()).expect("write");
            let out = child.wait_with_output().expect("wait");
            let ms = t.elapsed().as_secs_f64() * 1000.0;
            assert!(out.status.success());
            if rep >= 3 {
                times.push(ms);
            }
        }
        times.sort_by(f64::total_cmp);
        let med = times[times.len() / 2];
        let p95 = times[((times.len() - 1) as f64 * 0.95).round() as usize];
        println!("{name:<18} {:>9} {med:>9.2} {p95:>9.2}", text.len());
        all.extend(times);
    }
    all.sort_by(f64::total_cmp);
    println!(
        "inputs={} (real {n_real}) process_median_ms={:.2} process_p95_ms={:.2}",
        set.len(),
        all[all.len() / 2],
        all[((all.len() - 1) as f64 * 0.95).round() as usize]
    );
    let _ = fs::remove_dir_all(&home);
}
