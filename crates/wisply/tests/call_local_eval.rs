use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

const EXE: &str = env!("CARGO_BIN_EXE_wisply-full");
const DIR_ENV: &str = "WISPLY_LOCAL_DIR";
const ROWS: &str = include_str!("data/call-answers.tsv");
const ROWS_ENV: &str = "CALL_EVAL_ROWS";
const MIN_ROWS: usize = 150;
const UNCLEAR: &str = "unclear";
const FAR: f64 = 4_000_000_000.0;
const MIN_FULL: f64 = 95.0;
const MAX_WRONG: f64 = 1.0;
const SPOKEN: &str = "ja mach das aber erst morgen";
const WHISPER_RATE: u32 = 16_000;
const WAV_HEADER: usize = 44;
const KEYS: [&str; 3] = ["GEMINI_API_KEY", "GROQ_API_KEY", "OPENAI_API_KEY"];

struct Row {
    question: String,
    choices: Vec<String>,
    reply: String,
    expect: String,
    note: String,
}

struct Got {
    choice: String,
    note: String,
    ms: f64,
}

struct Home(PathBuf);

impl Home {
    fn run(&self, args: &[&str]) -> (i32, String, f64) {
        let began = Instant::now();
        let mut cmd = Command::new(EXE);
        cmd.args(args)
            .env("WISPLY_HOME", &self.0)
            .env(if cfg!(windows) { "USERPROFILE" } else { "HOME" }, &self.0)
            .env("CLAUDE_CODE_SESSION_ID", "it-call-local-eval")
            .env_remove("MSYSTEM")
            .stdin(Stdio::null());
        for key in KEYS {
            cmd.env_remove(key);
        }
        let out = cmd.output().unwrap();
        let ms = began.elapsed().as_secs_f64() * 1000.0;
        (out.status.code().unwrap_or(-1), String::from_utf8_lossy(&out.stdout).trim_end().to_string(), ms)
    }

    fn asks(&self) -> PathBuf {
        self.0.join("call").join("asks")
    }
}

impl Drop for Home {
    fn drop(&mut self) {
        let _ = self.run(&["local", "stop"]);
        let _ = self.run(&["stop"]);
    }
}

fn rows() -> Vec<Row> {
    let own = std::env::var(ROWS_ENV).ok().map(|p| fs::read_to_string(&p).unwrap_or_else(|e| panic!("{p}: {e}")));
    let text = own.as_deref().unwrap_or(ROWS);
    if own.is_none() {
        assert!(
            text.lines().filter(|l| !l.trim().is_empty()).count() >= MIN_ROWS,
            "the check list needs at least {MIN_ROWS} rows"
        );
    }
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let c: Vec<&str> = l.split('\t').collect();
            assert_eq!(c.len(), 5, "bad row: {l}");
            Row {
                question: c[0].to_string(),
                choices: c[1].split('|').map(str::to_string).collect(),
                reply: c[2].to_string(),
                expect: c[3].to_string(),
                note: c[4].to_string(),
            }
        })
        .collect()
}

fn write_ask(home: &Home, id: &str, row: &Row) {
    fs::create_dir_all(home.asks()).unwrap();
    let ask = serde_json::json!({
        "id": id, "question": row.question, "choices": row.choices, "created": 1.0,
        "deadline": FAR, "state": "open", "answer": null, "via": "",
    });
    fs::write(home.asks().join(format!("{id}.json")), ask.to_string()).unwrap();
}

fn read_line(id: &str, code: i32, out: &str, ms: f64) -> Got {
    let none = Got { choice: UNCLEAR.to_string(), note: String::new(), ms };
    let prefix = format!("call: {id} answered ");
    let Some(rest) = out.strip_prefix(&prefix).filter(|_| code == 0) else { return none };
    match rest.rsplit_once(" (") {
        Some((choice, note)) => Got { choice: choice.to_string(), note: note.trim_end_matches(')').to_string(), ms },
        None => Got { choice: rest.trim().to_string(), note: String::new(), ms },
    }
}

fn pass(home: &Home, mode: &str, rows: &[Row]) -> Vec<Got> {
    let (code, out, _) = home.run(&["call", "set", &format!("understand={mode}")]);
    assert_eq!(code, 0, "{out}");
    rows.iter()
        .enumerate()
        .map(|(n, row)| {
            let id = format!("{mode}{n:04}");
            write_ask(home, &id, row);
            let (code, out, ms) = home.run(&["call", "answer", &id, &row.reply]);
            read_line(&id, code, &out, ms)
        })
        .collect()
}

fn percent(part: usize, whole: usize) -> f64 {
    part as f64 * 100.0 / whole.max(1) as f64
}

fn right(rows: &[Row], got: &[Got]) -> usize {
    rows.iter().zip(got).filter(|(r, g)| r.expect == g.choice).count()
}

fn wrong(rows: &[Row], got: &[Got]) -> usize {
    rows.iter().zip(got).filter(|(r, g)| g.choice != UNCLEAR && r.expect != g.choice).count()
}

fn quantile(mut ms: Vec<f64>, q: f64) -> f64 {
    ms.sort_by(f64::total_cmp);
    let at = ((ms.len() as f64 - 1.0) * q).round() as usize;
    ms.get(at).copied().unwrap_or(0.0)
}

fn notes(rows: &[Row], got: &[Got]) -> (usize, usize) {
    let wanted: Vec<(&Row, &Got)> = rows.iter().zip(got).filter(|(r, _)| !r.note.is_empty()).collect();
    let hit = wanted.iter().filter(|(r, g)| g.note.to_lowercase().contains(&r.note.to_lowercase())).count();
    (hit, wanted.len())
}

fn resample(wav: &[u8]) -> Vec<u8> {
    let rate = u32::from_le_bytes(wav[24..28].try_into().unwrap());
    let samples: Vec<i16> = wav[WAV_HEADER..].as_chunks::<2>().0.iter().map(|b| i16::from_le_bytes(*b)).collect();
    let total = (samples.len() as u64 * u64::from(WHISPER_RATE) / u64::from(rate)) as usize;
    let pcm: Vec<u8> = (0..total)
        .flat_map(|i| {
            let at = i as f64 * f64::from(rate) / f64::from(WHISPER_RATE);
            let (lo, frac) = (at as usize, at.fract());
            let a = f64::from(samples[lo.min(samples.len() - 1)]);
            let b = f64::from(samples[(lo + 1).min(samples.len() - 1)]);
            ((a + (b - a) * frac) as i16).to_le_bytes()
        })
        .collect();
    let mut out = Vec::with_capacity(pcm.len() + WAV_HEADER);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + pcm.len() as u32).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&WHISPER_RATE.to_le_bytes());
    out.extend_from_slice(&(WHISPER_RATE * 2).to_le_bytes());
    out.extend_from_slice(&2u16.to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&(pcm.len() as u32).to_le_bytes());
    out.extend_from_slice(&pcm);
    out
}

fn speak(dir: &Path) -> Option<Vec<u8>> {
    let file = dir.join("probe.wav");
    let ok =
        Command::new("espeak-ng").args(["-v", "de", "-w"]).arg(&file).arg(SPOKEN).status().is_ok_and(|s| s.success());
    ok.then(|| fs::read(&file).ok()).flatten().filter(|w| w.len() > WAV_HEADER).map(|w| resample(&w))
}

fn multipart(audio: &[u8]) -> (Vec<u8>, String) {
    let boundary = "wisplyevalboundary";
    let mut body = Vec::new();
    for (name, value) in [("language", "de"), ("response_format", "json")] {
        body.extend_from_slice(
            format!("--{boundary}\r\nContent-Disposition: form-data; name=\"{name}\"\r\n\r\n{value}\r\n").as_bytes(),
        );
    }
    body.extend_from_slice(
        format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"u.wav\"\r\nContent-Type: audio/wav\r\n\r\n"
        )
        .as_bytes(),
    );
    body.extend_from_slice(audio);
    body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
    (body, format!("multipart/form-data; boundary={boundary}"))
}

fn probe(home: &Home) -> Result<String, String> {
    let Some(audio) = speak(&home.0) else { return Ok("skipped (no espeak-ng)".to_string()) };
    let (code, url, _) = home.run(&["local-url", "asr"]);
    if code != 0 {
        return Err(format!("whisper did not start: {url}"));
    }
    let (body, ctype) = multipart(&audio);
    let mut resp = ureq::post(&format!("{}/v1/audio/transcriptions", url.trim()))
        .config()
        .timeout_global(Some(std::time::Duration::from_secs(120)))
        .build()
        .header("Content-Type", ctype)
        .send(&body[..])
        .map_err(|e| e.to_string())?;
    let data: serde_json::Value = resp.body_mut().read_json().map_err(|e| e.to_string())?;
    let said = data["text"].as_str().unwrap_or("").to_lowercase();
    if said.contains("ja") && said.contains("morgen") { Ok(said) } else { Err(format!("heard '{said}'")) }
}

#[test]
#[ignore = "needs the downloaded local models, run: wisply local get; WISPLY_LOCAL_DIR=<home> cargo test -p wisply --test call_local_eval -- --ignored --nocapture"]
fn local_understanding_meets_the_bar() {
    let dir = std::env::var(DIR_ENV).unwrap_or_else(|_| panic!("set {DIR_ENV} to a home where wisply local get ran"));
    let home = Home(PathBuf::from(dir));
    let rows = rows();
    let rules = pass(&home, "off", &rows);
    let full = pass(&home, "auto", &rows);
    let ms: Vec<f64> = full.iter().map(|g| g.ms).collect();
    let (full_pct, rules_pct) = (percent(right(&rows, &full), rows.len()), percent(right(&rows, &rules), rows.len()));
    let wrong_n = wrong(&rows, &full);
    println!(
        "EVAL rules-only={rules_pct:.0}% full={full_pct:.0}% wrong={wrong_n} p50={:.0}ms p95={:.0}ms",
        quantile(ms.clone(), 0.5),
        quantile(ms, 0.95)
    );
    let (hit, asked) = notes(&rows, &full);
    println!("EVAL notes={hit}/{asked} rows={}", rows.len());
    for ((row, got), at) in rows.iter().zip(&full).zip(1..) {
        if row.expect != got.choice {
            println!("EVAL miss row {at}: '{}' -> {} (wanted {})", row.reply, got.choice, row.expect);
        }
    }
    let heard = probe(&home);
    println!("EVAL whisper {}", heard.as_ref().map_or_else(|e| format!("FAILED {e}"), |s| format!("ok '{s}'")));
    assert!(full_pct >= MIN_FULL, "full {full_pct:.1}% is below {MIN_FULL}%");
    assert!(percent(wrong_n, rows.len()) <= MAX_WRONG, "{wrong_n} confident wrong answers");
    heard.unwrap();
}
