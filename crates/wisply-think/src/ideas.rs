use serde_json::{Value, json};
use wisply_core::Call;

use crate::flow::counted;
use crate::run::{Home, Run, STATUSES, cut, int, points, table, text};
use crate::safety;

const TOP_DEFAULT: usize = 15;
const EXHAUSTED_RATIO: f64 = 0.5;
const EXHAUSTED_BATCH: usize = 4;

fn parse_idea(line: &str) -> Result<Value, String> {
    let mut parts: Vec<String> = line.split('|').map(|p| p.trim().to_string()).collect();
    if parts[0].is_empty() {
        return Err("title missing".into());
    }
    let defaults = ["", "-", "-", "2", "2", "2", "safe"];
    while parts.len() < 7 {
        parts.push(defaults[parts.len()].into());
    }
    let mut nums = Vec::new();
    for (name, v) in ["impact", "sure", "effort"].iter().zip(&parts[3..6]) {
        match v.as_str() {
            "1" | "2" | "3" => nums.push(v.parse::<i64>().unwrap_or(2)),
            _ => return Err(format!("{name} must be 1, 2 or 3, not '{v}'")),
        }
    }
    let kind = match parts[6].to_lowercase().as_str() {
        "safe" | "sicher" => "safe",
        "big" | "gross" | "groß" => "big",
        other => return Err(format!("kind must be safe or big, not '{other}'")),
    };
    Ok(json!({"title": parts[0], "lens": parts[1], "tool": parts[2], "impact": nums[0], "sure": nums[1],
              "effort": nums[2], "kind": kind, "by": "claude"}))
}

pub fn idea(home: &Home, c: &Call) -> Result<String, String> {
    let sub = c.pos.get(1).map(String::as_str).unwrap_or("list");
    let mut r = home.load(c.val("run"))?;
    match sub {
        "add" => {
            let mut lines: Vec<String> = c.pos.iter().skip(2).cloned().collect();
            if lines.is_empty() {
                lines = c.val("input").unwrap_or("").lines().map(str::to_string).collect();
            }
            lines.retain(|l| !l.trim().is_empty() && !l.trim_start().starts_with('#'));
            if lines.is_empty() {
                return Err(
                    "no ideas. Format: \"title | lens | tool | impact 1-3 | sure 1-3 | effort 1-3 | safe/big\"".into(),
                );
            }
            let (mut added, mut dupes, mut bad) = (0, Vec::new(), Vec::new());
            for l in &lines {
                match parse_idea(l) {
                    Ok(v) => match r.add_idea(v, c.on("force")) {
                        Ok(_) => added += 1,
                        Err(twin) => dupes.push(format!("{} ~ {twin}", cut(l.split('|').next().unwrap_or(""), 40))),
                    },
                    Err(e) => bad.push(format!("{}: {e}", cut(l, 40))),
                }
            }
            let exhausted =
                lines.len() >= EXHAUSTED_BATCH && dupes.len() as f64 / lines.len() as f64 >= EXHAUSTED_RATIO;
            if exhausted {
                let round = r.round();
                r.map().insert("ideas_out".into(), json!(round));
            }
            r.save()?;
            let mut out = vec![format!(
                "+{added} ideas (total {}, {}/{} toward the goal, wisply findings count half){}",
                r.ideas().len(),
                counted(&r.ideas(), None),
                int(&r.s, "idea_goal"),
                if dupes.is_empty() { String::new() } else { format!(", {} duplicates", dupes.len()) }
            )];
            out.extend(dupes.iter().map(|d| format!("  duplicate: {d}")));
            out.extend(bad.iter().map(|b| format!("  ERROR: {b}")));
            if exhausted {
                out.push("ideas are running out (many duplicates): switch tool or role, or honestly fewer".into());
            }
            Ok(out.join("\n"))
        }
        "out" => {
            let why = c.pos[2..].join(" ");
            if why.trim().is_empty() {
                return Err("wisply think idea out \"<why no more real ideas come>\"".into());
            }
            let round = r.round();
            r.map().insert("ideas_out".into(), json!(round));
            r.s["ideas_out_why"][round.to_string()] = json!(why);
            r.save()?;
            Ok(format!(
                "round {round}: ideas honestly ran out ({} total), the reason goes into the report",
                r.ideas().len()
            ))
        }
        "set" => idea_set(&mut r, c),
        _ => Ok(idea_list(&r, c)),
    }
}

fn idea_list(r: &Run, c: &Call) -> String {
    let mut ideas: Vec<Value> = r.ideas();
    if let Some(st) = c.val("status") {
        ideas.retain(|i| text(i, "status") == st);
    }
    if let Some(n) = c.val("round").and_then(|n| n.parse::<i64>().ok()) {
        ideas.retain(|i| int(i, "round") == n);
    }
    ideas.sort_by(|a, b| points(b).total_cmp(&points(a)).then(int(a, "id").cmp(&int(b, "id"))));
    let top = if c.on("all") { ideas.len() } else { c.val("top").and_then(|n| n.parse().ok()).unwrap_or(TOP_DEFAULT) };
    let rows: Vec<Vec<String>> = ideas
        .iter()
        .take(top)
        .map(|i| {
            let place = if text(i, "place").is_empty() { String::new() } else { format!(" ({})", text(i, "place")) };
            vec![
                int(i, "id").to_string(),
                cut(&format!("{}{place}", text(i, "title")), 70),
                text(i, "lens").into(),
                text(i, "kind").into(),
                points(i).to_string(),
                text(i, "status").into(),
                if text(i, "by") == "wisply" { "wisply".into() } else { "claude".into() },
            ]
        })
        .collect();
    let mut out = table(&["#", "Idea", "Lens", "Kind", "Points", "Status", "By"], &rows);
    if out.is_empty() {
        out = "no ideas".into();
    }
    if ideas.len() > top {
        out.push_str(&format!("\n... +{} more (--all)", ideas.len() - top));
    }
    out
}

fn idea_set(r: &mut Run, c: &Call) -> Result<String, String> {
    let status = c
        .val("status")
        .ok_or("wisply think idea set <ids> --status done|unchecked|dropped|suggestion|new")?
        .to_string();
    if !STATUSES.contains(&status.as_str()) {
        return Err(format!("status is one of {}", STATUSES.join(", ")));
    }
    let ids: Vec<i64> = c.pos.iter().skip(2).filter_map(|s| s.trim_start_matches('#').parse().ok()).collect();
    if ids.is_empty() {
        return Err("which ideas? wisply think idea set 3 4 --status done --proof \"...\"".into());
    }
    for id in &ids {
        let Some(i) = r.ideas().into_iter().find(|i| int(i, "id") == *id) else {
            return Err(format!("idea #{id} does not exist"));
        };
        if status == "done" && c.val("proof").is_none() && text(&i, "proof").is_empty() {
            return Err(format!("#{id}: done only with proof -> --proof \"<command + result>\""));
        }
        if status == "dropped" && c.val("why").is_none() && text(&i, "why").is_empty() {
            return Err(format!("#{id}: dropped only with a reason -> --why \"<reason>\""));
        }
    }
    let round = r.round();
    for id in &ids {
        let i = r.idea_mut(*id).expect("checked above");
        i["status"] = json!(status);
        for k in ["proof", "why", "source"] {
            if let Some(v) = c.val(k) {
                i[k] = json!(v);
            }
        }
        if let Some(g) = c.val("grade").and_then(|g| g.parse::<i64>().ok()) {
            i["grade"] = json!(g.clamp(1, 10));
        }
        i["round_status"] = json!(round);
        if status == "done" || status == "unchecked" {
            safety::mark_after(r, *id);
        }
    }
    r.save()?;
    Ok(format!("{} idea(s) -> {status}", ids.len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ideas_parse_with_defaults_and_limits() {
        let v = parse_idea("Cache rules | perf | measure | 3 | 2 | 1 | safe").unwrap();
        assert_eq!((text(&v, "title"), int(&v, "impact"), text(&v, "kind")), ("Cache rules", 3, "safe"));
        let v = parse_idea("Just a title").unwrap();
        assert_eq!((int(&v, "impact"), text(&v, "kind"), text(&v, "by")), (2, "safe", "claude"));
        assert_eq!(parse_idea("x | a | b | 4").unwrap_err(), "impact must be 1, 2 or 3, not '4'");
        assert_eq!(parse_idea("x | a | b | 1 | 1 | 1 | huge").unwrap_err(), "kind must be safe or big, not 'huge'");
        assert_eq!(parse_idea("x | a | b | 1 | 1 | 1 | gross").unwrap()["kind"], "big");
        let ideas: Vec<Value> = [("claude", 1), ("wisply", 1), ("wisply", 1), ("wisply", 1), ("claude", 2)]
            .iter()
            .map(|(by, round)| json!({"by": by, "round": round}))
            .collect();
        assert_eq!((counted(&ideas, Some(1)), counted(&ideas, None)), (2, 3));
    }
}
