use super::*;

pub(super) fn open(base: &Path, dir: &Path) -> Result<String, Fail> {
    let f = frozen(dir)?;
    let all = load(dir)?;
    let mut pool: BTreeMap<String, BTreeSet<i64>> = BTreeMap::new();
    for path in results(dir) {
        let Some(v) = fs::read_to_string(&path).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()) else {
            continue;
        };
        for r in rows_of(&v["rows"]) {
            pool.entry(r.id).or_default().extend(r.open);
        }
    }
    let mut s = Store::open(base).map_err(db_fail)?;
    s.set_clock(f["at"].as_f64().unwrap_or(0.0));
    let cache = vectors(dir);
    let mut bare = Vec::new();
    for q in all.iter().filter(|q| !q.labelled()) {
        let (hits, _) = s.ask(q, cache.get(&q.query).map(Vec::as_slice)).map_err(db_fail)?;
        let ids: Vec<i64> = hits.iter().map(|h| h.note.id).collect();
        let got: Vec<String> = ids.iter().map(|id| format!("#{id}")).collect();
        let found = if got.is_empty() { "nothing".into() } else { got.join(" ") };
        bare.push(format!("  {} \"{}\" got {found}", q.id, oneline(&q.query, QUERY_CHARS)));
        pool.entry(q.id.clone()).or_default().extend(ids);
    }
    let mut lines = Vec::new();
    for q in &all {
        let judged = |id: &i64| q.good.contains(id) || q.bad.contains(id) || q.old.contains(id);
        for id in pool.get(&q.id).into_iter().flatten().filter(|id| !judged(id)) {
            let Some(n) = s.note(*id).map_err(db_fail)? else { continue };
            lines.push(format!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}",
                q.id,
                oneline(&q.query, TITLE_CHARS),
                n.id,
                n.kind,
                n.project,
                oneline(&n.title, TITLE_CHARS),
                oneline(&n.body, BODY_CHARS)
            ));
        }
    }
    let to = dir.join(OPEN);
    fs::write(&to, lines.join("\n")).map_err(|e| file_fail(&to, e))?;
    let mut said = vec![format!(
        "{} hits wait for a verdict in {} (question, query, note, kind, project, title, body)",
        lines.len(),
        shown_path(&to)
    )];
    if !bare.is_empty() {
        said.push(format!(
            "without a good note yet: {} (question, query, its hits; they are in the list too)",
            bare.len()
        ));
        said.extend(bare.iter().take(BARE_SHOWN).cloned());
        if bare.len() > BARE_SHOWN {
            said.push(format!("  and {} more", bare.len() - BARE_SHOWN));
        }
    }
    said.push("wisply memory bench judge <question> good|bad|old <note> ...".into());
    Ok(said.join("\n"))
}

pub(super) fn label(dir: &Path, args: &[String]) -> Result<String, Fail> {
    let usage = || Fail::hint("judge needs a question, a verdict and note numbers", USAGE);
    let [id, verdict, notes @ ..] = args else { return Err(usage()) };
    let ids: Vec<i64> = notes.iter().filter_map(|n| n.trim_start_matches('#').parse().ok()).collect();
    if !LABELS.contains(&verdict.as_str()) || ids.is_empty() || ids.len() != notes.len() {
        return Err(usage());
    }
    load(dir)?;
    let path = dir.join(QUESTIONS);
    let text = fs::read_to_string(&path).map_err(|e| file_fail(&path, e))?;
    let mut said = None;
    let mut out = Vec::new();
    for line in text.lines() {
        let mut v: Value = serde_json::from_str(line).unwrap_or(Value::Null);
        if v["id"].as_str() != Some(id.as_str()) {
            out.push(line.to_string());
            continue;
        }
        for key in LABELS {
            let mut list: Vec<i64> =
                v[key].as_array().map(|a| a.iter().filter_map(Value::as_i64).collect()).unwrap_or_default();
            list.retain(|n| !ids.contains(n));
            if key == verdict {
                list.extend(&ids);
            }
            v[key] = json!(list);
        }
        said = Some(format!("{id}: good {}, bad {}, old {}", v["good"], v["bad"], v["old"]));
        out.push(v.to_string());
    }
    let Some(said) = said else { return Err(Fail::new(format!("no question '{id}'"))) };
    fs::write(&path, out.join("\n") + "\n").map_err(|e| file_fail(&path, e))?;
    Ok(said)
}
