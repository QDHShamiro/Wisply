use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use serde_json::{Value, json};
use wisply_core::Fail;
use wisply_memory::{
    BENCH_TOP, CLASSES, CROSS, DbError, NO_ANSWER, Question, Row, SMALL_CLASS, Store, Sums, by_class, claude_dirs,
    date, gain, judge, now, oneline, paired, questions, rows_json, rows_of, sums,
};

use super::super::game;
use super::{hook, shown, todo_hits};

mod label;

const USAGE: &str = "wisply memory bench [freeze [--again] | run [--hold] [--vectors] [--tag name] [--misses] | \
                     compare <old> <new> | open | judge <question> good|bad|old <note> ...]";
const QUESTIONS: &str = "questions.jsonl";
const VECTORS: &str = "vectors.jsonl";
const FROZEN: &str = "frozen.json";
const RESULTS: &str = "results";
const OPEN: &str = "unjudged.tsv";
const DB: &str = "memory.db";
const COMMIT: &str = env!("WISPLY_COMMIT");
const TAG_MAX: usize = 24;
const TITLE_CHARS: usize = 160;
const BODY_CHARS: usize = 300;
const QUERY_CHARS: usize = 70;
const BARE_SHOWN: usize = 20;
const LABELS: [&str; 3] = ["good", "bad", "old"];

pub fn run(home: &Path, args: &[String]) -> ExitCode {
    match command(home, args) {
        Ok(text) => {
            println!("{text}");
            ExitCode::SUCCESS
        }
        Err(fail) => {
            println!("{}", fail.text());
            ExitCode::from(1)
        }
    }
}

fn command(home: &Path, args: &[String]) -> Result<String, Fail> {
    let base = home.join("bench");
    let dir = base.join("memory");
    let rest = args.get(1..).unwrap_or_default();
    match args.first().map(String::as_str) {
        None => status(&dir),
        Some("freeze") => freeze(home, &dir, rest),
        Some("run") => measure(&base, &dir, rest),
        Some("compare") => compare(&dir, rest),
        Some("open") => label::open(&base, &dir),
        Some("judge") => label::label(&dir, rest),
        Some(other) => Err(Fail::hint(format!("unknown bench command '{other}'"), USAGE)),
    }
}

fn db_fail(e: DbError) -> Fail {
    Fail::new(format!("memory.db: {e}"))
}

fn file_fail(path: &Path, e: std::io::Error) -> Fail {
    Fail::new(format!("{}: {e}", shown_path(path)))
}

fn shown_path(path: &Path) -> String {
    path.display().to_string().replace('\\', "/")
}

fn pct(part: usize, all: usize) -> String {
    if all == 0 { "-".into() } else { format!("{:.1} %", part as f64 * 100.0 / all as f64) }
}

fn frozen(dir: &Path) -> Result<Value, Fail> {
    let none = || Fail::hint("no frozen copy of the memory yet", "wisply memory bench freeze");
    let text = fs::read_to_string(dir.join(FROZEN)).map_err(|_| none())?;
    serde_json::from_str::<Value>(&text).ok().filter(|v| v["at"].is_f64() && dir.join(DB).is_file()).ok_or_else(none)
}

fn load(dir: &Path) -> Result<Vec<Question>, Fail> {
    let path = dir.join(QUESTIONS);
    let text = fs::read_to_string(&path).map_err(|_| {
        Fail::hint(
            format!("no questions in {}", shown_path(&path)),
            "one JSON object per line: id, class, query, project, split (tune|hold), cross, good, bad, old",
        )
    })?;
    questions(&text).map_err(|e| Fail::new(format!("{QUESTIONS} {e}")))
}

fn vectors(dir: &Path) -> HashMap<String, Vec<f32>> {
    let text = fs::read_to_string(dir.join(VECTORS)).unwrap_or_default();
    text.lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .filter_map(|v| {
            let vec: Vec<f32> = v["vec"].as_array()?.iter().filter_map(Value::as_f64).map(|x| x as f32).collect();
            Some((v["query"].as_str()?.to_string(), vec))
        })
        .collect()
}

fn results(dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(dir.join(RESULTS)) else { return Vec::new() };
    let mut all: Vec<PathBuf> =
        entries.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "json")).collect();
    all.sort();
    all
}

fn status(dir: &Path) -> Result<String, Fail> {
    let f = frozen(dir)?;
    let mut lines = vec![format!(
        "memory bench: copy of {} ({} notes, {} with vectors) in {}",
        f["day"].as_str().unwrap_or("-"),
        f["notes"],
        f["vecs"],
        shown_path(dir)
    )];
    match load(dir) {
        Ok(all) => {
            let hold = all.iter().filter(|q| q.hold).count();
            lines.push(format!(
                "questions: {} ({} tuning, {hold} hold-out), {} without a label, {} asked in German and noted in English",
                all.len(),
                all.len() - hold,
                all.iter().filter(|q| !q.labelled()).count(),
                all.iter().filter(|q| q.cross).count()
            ));
            let per: Vec<String> = CLASSES
                .iter()
                .map(|c| format!("{c} {}", all.iter().filter(|q| q.class == *c).count()))
                .filter(|c| !c.ends_with(" 0"))
                .collect();
            lines.push(format!("classes: {}", per.join(", ")));
            let cache = vectors(dir);
            lines.push(format!(
                "cached query vectors: {} of {}",
                all.iter().filter(|q| cache.contains_key(&q.query)).count(),
                all.len()
            ));
        }
        Err(fail) => lines.push(fail.text()),
    }
    let done = results(dir);
    if let Some(last) = done.last().and_then(|p| p.file_name()) {
        lines.push(format!("results: {}, last {}", done.len(), last.to_string_lossy()));
    }
    lines.push(USAGE.into());
    Ok(lines.join("\n"))
}

fn freeze(home: &Path, dir: &Path, args: &[String]) -> Result<String, Fail> {
    let to = dir.join(DB);
    if to.exists() && !args.iter().any(|a| a == "--again") {
        return Err(Fail::hint(
            "a frozen copy exists, and results only compare on the same copy",
            "wisply memory bench freeze --again replaces it",
        ));
    }
    fs::create_dir_all(dir).map_err(|e| file_fail(dir, e))?;
    for tail in ["", "-wal", "-shm"] {
        let _ = fs::remove_file(dir.join(format!("{DB}{tail}")));
    }
    let live = Store::open(home).map_err(db_fail)?;
    live.copy_to(&to).map_err(db_fail)?;
    let c = live.counts().map_err(db_fail)?;
    let at = now();
    let note = json!({"at": at, "day": date(at), "notes": c.notes, "vecs": c.vecs});
    fs::write(dir.join(FROZEN), note.to_string()).map_err(|e| file_fail(dir, e))?;
    Ok(format!("froze {} notes ({} with vectors) into {}", c.notes, c.vecs, shown_path(&to)))
}

fn tokens(text: &str) -> usize {
    text.chars().count() / 4
}

fn start_info(s: &mut Store, asked: &[&Question]) -> Result<Value, DbError> {
    let projects: BTreeSet<&str> = asked.iter().map(|q| q.project.as_str()).chain([""]).collect();
    hook::start_text(s, "", Vec::new())?;
    let mut sizes = Vec::new();
    for p in &projects {
        sizes.push((tokens(&hook::start_text(s, p, Vec::new())?), *p));
    }
    let top = sizes.iter().max().copied().unwrap_or_default();
    let files: Vec<usize> = claude_dirs(&wisply_proto::user_dir())
        .iter()
        .filter_map(|d| fs::read_to_string(d.join("MEMORY.md")).ok())
        .map(|t| tokens(&t))
        .collect();
    Ok(json!({
        "wisply": sizes.iter().map(|s| s.0).sum::<usize>() / sizes.len().max(1),
        "wisply_max": top.0,
        "wisply_max_project": top.1,
        "claude_max": files.iter().max().copied().unwrap_or(0),
        "claude_files": files.len(),
    }))
}

fn sums_line(s: &Sums) -> String {
    format!(
        "hit@1 {} ({}/{})   hit@{BENCH_TOP} {} ({}/{})   MRR {:.3}   old note in the top {BENCH_TOP}: {}",
        pct(s.hit1, s.n),
        s.hit1,
        s.n,
        pct(s.hit3, s.n),
        s.hit3,
        s.n,
        s.mrr,
        s.stale
    )
}

fn class_lines(parts: &BTreeMap<String, Sums>) -> Vec<String> {
    let mut lines = vec![format!("by class (under {SMALL_CLASS} questions = a pointer, not a proof):")];
    for (name, s) in parts {
        let small = if s.n < SMALL_CLASS { "  (pointer)" } else { "" };
        lines.push(format!(
            "  {name:<9} n {:<4} hit@1 {:<8} hit@{BENCH_TOP} {:<8} MRR {:.3}{small}",
            s.n,
            pct(s.hit1, s.n),
            pct(s.hit3, s.n),
            s.mrr
        ));
    }
    lines
}

fn tag_of(args: &[String]) -> String {
    args.iter()
        .position(|a| a == "--tag")
        .and_then(|i| args.get(i + 1))
        .map(|t| t.chars().filter(|c| c.is_ascii_alphanumeric() || *c == '_').take(TAG_MAX).collect())
        .filter(|t: &String| !t.is_empty())
        .unwrap_or_else(|| "run".into())
}

fn answers(s: &mut Store, asked: &[&Question], cache: &HashMap<String, Vec<f32>>) -> Result<Vec<Row>, DbError> {
    s.ask(asked[0], None)?;
    let mut rows = Vec::with_capacity(asked.len());
    for q in asked {
        let vec = cache.get(&q.query).map(Vec::as_slice);
        let (hits, ms) = s.ask(q, vec)?;
        let mut row = judge(q, &hits.iter().map(|h| h.note.id).collect::<Vec<_>>());
        row.ms = ms;
        let todos = todo_hits(s, &q.query)?;
        row.chars = shown(&hits, &todos, &q.project, &q.query, "").chars().count();
        let hinted = hook::hints(s, &q.query, &q.project, vec)?;
        row.hint(q, &hinted.iter().take(hook::HINTS).map(|h| h.note.id).collect::<Vec<_>>());
        rows.push(row);
    }
    Ok(rows)
}

fn report(total: &Sums, parts: &BTreeMap<String, Sums>, start: &Value, game: &str) -> Vec<String> {
    let mut lines = vec![
        sums_line(total),
        format!(
            "hints: {} ({} wrong = {}), {} of the {} questions without an answer got one",
            total.hints,
            total.wrong_hints,
            pct(total.wrong_hints, total.hints),
            total.hinted_without_answer,
            total.without_answer
        ),
        format!(
            "answer: {} chars on average   time p50 {:.1} ms, p95 {:.1} ms{}",
            total.chars,
            total.p50,
            total.p95,
            if game.is_empty() {
                String::new()
            } else {
                format!(" (times do not count while {game} runs, measure them again without a game)")
            }
        ),
        format!(
            "start info: Wisply est. {} tokens (largest: {} for {}), Claude MEMORY.md est. up to {} tokens ({} files)",
            start["wisply"],
            start["wisply_max"],
            start["wisply_max_project"].as_str().filter(|p| !p.is_empty()).unwrap_or("no project"),
            start["claude_max"],
            start["claude_files"]
        ),
    ];
    lines.extend(class_lines(parts));
    lines
}

fn missed(asked: &[&Question], rows: &[Row]) -> Vec<String> {
    let miss = |q: &Question, r: &Row| q.class != NO_ANSWER && r.rank.is_none_or(|k| k > BENCH_TOP);
    let list: Vec<String> = asked
        .iter()
        .zip(rows)
        .filter(|(q, r)| miss(q, r))
        .map(|(q, r)| {
            let top: Vec<String> = r.top.iter().take(BENCH_TOP).map(|id| format!("#{id}")).collect();
            let rank = r.rank.map_or("-".into(), |k| k.to_string());
            let query = oneline(&q.query, QUERY_CHARS);
            format!("  {} {} rank {rank} \"{query}\" got {}", q.id, q.class, top.join(" "))
        })
        .collect();
    let head =
        format!("missed, good note not in the top {BENCH_TOP}: {} (question, class, rank, query, hits)", list.len());
    [head].into_iter().chain(list).collect()
}

fn measure(base: &Path, dir: &Path, args: &[String]) -> Result<String, Fail> {
    let on = |name: &str| args.iter().any(|a| a == name);
    let (hold, cached, misses) = (on("--hold"), on("--vectors"), on("--misses"));
    if hold && misses {
        return Err(Fail::new("the hold-out set shows sums only, never single questions"));
    }
    let f = frozen(dir)?;
    let all = load(dir)?;
    let part = all.iter().filter(|q| q.hold == hold).count();
    let asked: Vec<&Question> = all.iter().filter(|q| q.hold == hold && q.labelled()).collect();
    let split = if hold { "hold" } else { "tune" };
    if asked.is_empty() {
        return Err(Fail::hint(format!("no labelled question in the {split} set"), "wisply memory bench open"));
    }
    let cache = if cached { vectors(dir) } else { HashMap::new() };
    let bare = asked.iter().filter(|q| !cache.contains_key(&q.query)).count();
    if cached && bare > 0 {
        return Err(Fail::new(format!("{bare} questions have no cached vector in {VECTORS}")));
    }
    let mut s = Store::open(base).map_err(db_fail)?;
    s.set_clock(f["at"].as_f64().unwrap_or(0.0));
    let rows = answers(&mut s, &asked, &cache).map_err(db_fail)?;
    let start = start_info(&mut s, &asked).map_err(db_fail)?;
    let (total, parts) = (sums(&rows), by_class(&rows));
    let game = game::running();
    let (tag, variant, at) = (tag_of(args), if cached { "vectors" } else { "words" }, now());
    let name = format!("{}-{tag}-{variant}-{split}.json", at as u64);
    let result = json!({
        "at": at, "day": date(at), "commit": COMMIT, "tag": tag, "variant": variant, "split": split,
        "frozen": f["at"], "game": game, "skipped": part - asked.len(), "start": start, "sums": total.json(),
        "classes": parts.iter().map(|(k, v)| (k.clone(), v.json())).collect::<BTreeMap<_, _>>(),
        "rows": rows_json(&rows),
    });
    let to = dir.join(RESULTS);
    fs::create_dir_all(&to).map_err(|e| file_fail(&to, e))?;
    fs::write(to.join(&name), result.to_string()).map_err(|e| file_fail(&to, e))?;
    let skipped =
        if part == asked.len() { String::new() } else { format!(" ({} without a label skipped)", part - asked.len()) };
    let mut lines = vec![format!(
        "memory bench, {}, {split} set: {} questions{skipped}, copy of {} ({} notes), commit {}",
        if cached { "words and cached vectors" } else { "words only" },
        total.asked,
        f["day"].as_str().unwrap_or("-"),
        f["notes"],
        if COMMIT.is_empty() { "unknown" } else { COMMIT }
    )];
    lines.extend(report(&total, &parts, &start, &game));
    if misses {
        lines.extend(missed(&asked, &rows));
    }
    let unjudged: usize = rows.iter().map(|r| r.open.len()).sum();
    if unjudged > 0 {
        lines.push(format!("{unjudged} hits are not judged yet: wisply memory bench open"));
    }
    lines.push(format!("saved {RESULTS}/{name}"));
    Ok(lines.join("\n"))
}

fn result(dir: &Path, name: &str) -> Result<Value, Fail> {
    let folder = dir.join(RESULTS);
    let plain = name.replace('\\', "/");
    let bare = plain.rsplit('/').next().unwrap_or(name).trim_end_matches(".json");
    let tagged = |p: &PathBuf| {
        let stem = p.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
        stem.split_once('-').is_some_and(|(_, rest)| rest == bare || rest.starts_with(&format!("{bare}-")))
    };
    let path = [PathBuf::from(name), folder.join(format!("{bare}.json"))]
        .into_iter()
        .find(|p| p.is_file())
        .or_else(|| results(dir).into_iter().rfind(tagged))
        .ok_or_else(|| {
            let hint =
                format!("the tag of a run (its newest result counts) or a file name from {}", shown_path(&folder));
            Fail::hint(format!("no result '{name}'"), hint)
        })?;
    let text = fs::read_to_string(&path).map_err(|e| file_fail(&path, e))?;
    serde_json::from_str(&text).map_err(|e| Fail::new(format!("{name}: {e}")))
}

fn compare(dir: &Path, args: &[String]) -> Result<String, Fail> {
    let [old, new] = args else { return Err(Fail::hint("compare needs two results", USAGE)) };
    let (a, b) = (result(dir, old)?, result(dir, new)?);
    if a["split"] != b["split"] || a["frozen"] != b["frozen"] {
        return Err(Fail::hint(
            "these two results are from different question sets or different frozen copies",
            "a tag stands for its newest result, name the set as well: base-words-tune or base-words-hold",
        ));
    }
    let (before, after, left) = paired(&rows_of(&a["rows"]), &rows_of(&b["rows"]), &load(dir)?);
    let (x, y) = (sums(&before), sums(&after));
    let g = gain(&before, &after);
    let mut lines = vec![
        format!(
            "{} {} ({}) -> {} {} ({}), {} set, {} pairs, judged with the labels of today",
            a["tag"].as_str().unwrap_or("-"),
            a["variant"].as_str().unwrap_or("-"),
            a["commit"].as_str().unwrap_or("-"),
            b["tag"].as_str().unwrap_or("-"),
            b["variant"].as_str().unwrap_or("-"),
            b["commit"].as_str().unwrap_or("-"),
            a["split"].as_str().unwrap_or("-"),
            g.pairs
        ),
        format!(
            "hit@1 {} -> {}   hit@{BENCH_TOP} {} -> {}   MRR {:.3} -> {:.3}",
            pct(x.hit1, x.n),
            pct(y.hit1, y.n),
            pct(x.hit3, x.n),
            pct(y.hit3, y.n),
            x.mrr,
            y.mrr
        ),
        format!(
            "old note in the top {BENCH_TOP} {} -> {}   wrong hints {}/{} -> {}/{}   answer chars {} -> {}",
            x.stale, y.stale, x.wrong_hints, x.hints, y.wrong_hints, y.hints, x.chars, y.chars
        ),
        format!(
            "won {}, lost {} (top {BENCH_TOP}): {}",
            g.wins,
            g.losses,
            if g.counts { "counts as a gain" } else { "no proven gain (needs 10 won and twice as many won as lost)" }
        ),
    ];
    if left != [0, 0] {
        let [single, bare] = left;
        lines.insert(1, format!("left out: {single} asked in only one of the two runs, {bare} without a label today"));
    }
    if a["split"] != "hold" {
        lines.extend(class_gains(&before, &after));
    }
    Ok(lines.join("\n"))
}

fn class_gains(before: &[Row], after: &[Row]) -> Vec<String> {
    let mut lines = Vec::new();
    let classes: BTreeSet<&str> = after.iter().map(|r| r.class.as_str()).filter(|c| *c != NO_ANSWER).collect();
    for class in classes {
        let pick = |rows: &[Row]| rows.iter().filter(|r| r.class == class).cloned().collect::<Vec<_>>();
        let part = gain(&pick(before), &pick(after));
        lines.push(format!("  {class:<9} won {}, lost {} of {}", part.wins, part.losses, part.pairs));
    }
    let pick = |rows: &[Row]| rows.iter().filter(|r| r.cross).cloned().collect::<Vec<_>>();
    let part = gain(&pick(before), &pick(after));
    lines.push(format!("  {CROSS:<9} won {}, lost {} of {}", part.wins, part.losses, part.pairs));
    lines
}

#[cfg(test)]
mod tests;
