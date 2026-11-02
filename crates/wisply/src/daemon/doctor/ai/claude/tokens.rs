use std::collections::BTreeSet;

use serde_json::Value;

use super::tokens_files::{self as files, tokens};
use super::tokens_scan::{TokenScan, WINDOW_DAYS};
use super::usage::{is_off, mcp_names, normalized, skill_modes};
use super::{Env, Level, Row, clean_name, names_line, read_json, row};

const LONG_DESCRIPTION: usize = 250;
const BIG_BODY_TOKENS: usize = 3_000;
const LONG_LINE: usize = 120;
const MEMORY_TOKENS: usize = 2_000;
const MIN_LONG_LINE_TOKENS: usize = 200;
const NAME: &str = "tokens";

fn short(n: usize) -> String {
    if n >= 10_000 {
        format!("{}k", n / 1000)
    } else if n >= 1_000 {
        format!("{}.{}k", n / 1000, n % 1000 / 100)
    } else {
        n.to_string()
    }
}

fn cost(scan: &TokenScan, per_request: usize) -> String {
    format!("~{} tok/request, ~{} tok/day", short(per_request), short(scan.per_day(per_request)))
}

fn ranked(items: &mut [(String, usize)]) -> Vec<String> {
    items.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    items.iter().map(|(n, t)| format!("{n} ~{}", short(*t))).collect()
}

fn mcp_rows(env: &Env, scan: &TokenScan) -> Vec<Row> {
    let config = read_json(&env.config).unwrap_or(Value::Null);
    let mut servers: BTreeSet<String> = scan.mcp_cost.keys().cloned().collect();
    servers.extend(mcp_names(env, &config).iter().map(|n| normalized(n)));
    let mut idle: Vec<(String, usize)> = servers
        .iter()
        .filter(|s| clean_name(s) && !scan.mcp_used.contains_key(*s))
        .map(|s| {
            let c = scan.mcp_cost.get(s);
            (s.clone(), tokens(c.map_or(0, |c| c.tool_chars + c.instruction_chars)))
        })
        .collect();
    let total: usize = idle.iter().map(|i| i.1).sum();
    if idle.is_empty() {
        return vec![row(
            Level::Ok,
            NAME,
            format!("{} MCP servers and connectors, each called in {WINDOW_DAYS} days", servers.len()),
            "",
        )];
    }
    let count = idle.len();
    vec![row(
        Level::Warn,
        NAME,
        format!(
            "{count} of {} MCP servers/connectors without a call in {WINDOW_DAYS} days, {}: {}",
            servers.len(),
            cost(scan, total),
            names_line(&ranked(&mut idle))
        ),
        "turn the claude.ai connector off in Settings > Connectors or run /mcp disable <name> (no change made)",
    )]
}

fn idle_skill_row(env: &Env, scan: &TokenScan, infos: &[files::SkillInfo]) -> Option<Row> {
    let modes = skill_modes(env);
    let mut universe: BTreeSet<String> = scan.skill_cost.keys().cloned().collect();
    universe.extend(infos.iter().map(|i| i.name.clone()));
    let mut idle: Vec<(String, usize)> = Vec::new();
    for name in universe.iter().filter(|n| clean_name(n) && !is_off(&modes, n) && !scan.used_skill(n)) {
        let chars = scan.skill_cost.get(name).copied().or_else(|| {
            infos
                .iter()
                .find(|i| &i.name == name)
                .map(|i| i.name.chars().count() + i.description.min(LONG_DESCRIPTION) + 4)
        });
        idle.push((name.clone(), tokens(chars.unwrap_or(0))));
    }
    if idle.is_empty() {
        return None;
    }
    let total: usize = idle.iter().map(|i| i.1).sum();
    Some(row(
        Level::Warn,
        NAME,
        format!(
            "{} of {} listed skills not called in {WINDOW_DAYS} days, {}: {}",
            idle.len(),
            universe.len(),
            cost(scan, total),
            names_line(&ranked(&mut idle))
        ),
        "set \"skillOverrides\": {\"<name>\": \"off\"} in ~/.claude/settings.json (undo: remove the entry; no change made)",
    ))
}

fn long_description_row(scan: &TokenScan, infos: &[files::SkillInfo]) -> Option<Row> {
    let mut long: Vec<(String, usize)> = infos
        .iter()
        .filter(|i| clean_name(&i.name) && i.description > LONG_DESCRIPTION)
        .map(|i| (i.name.clone(), tokens(i.description - LONG_DESCRIPTION)))
        .collect();
    if long.is_empty() {
        return None;
    }
    let total: usize = long.iter().map(|i| i.1).sum();
    Some(row(
        Level::Warn,
        NAME,
        format!(
            "{} skill descriptions over {LONG_DESCRIPTION} characters, {} above the limit: {}",
            long.len(),
            cost(scan, total),
            names_line(&ranked(&mut long))
        ),
        "shorten each description to one sentence plus its trigger words, move the rest into the body (no change made)",
    ))
}

fn big_body_row(infos: &[files::SkillInfo]) -> Option<Row> {
    let mut big: Vec<(String, usize)> = infos
        .iter()
        .filter(|i| clean_name(&i.name) && tokens(i.body) > BIG_BODY_TOKENS)
        .map(|i| (i.name.clone(), tokens(i.body)))
        .collect();
    if big.is_empty() {
        return None;
    }
    Some(row(
        Level::Warn,
        NAME,
        format!(
            "{} skill bodies over {} tokens, paid on every call and again after a compact: {}",
            big.len(),
            short(BIG_BODY_TOKENS),
            names_line(&ranked(&mut big))
        ),
        "split rarely needed parts into files the skill reads on demand (no change made)",
    ))
}

fn skill_rows(env: &Env, scan: &TokenScan) -> Vec<Row> {
    let infos = files::skills(env);
    [idle_skill_row(env, scan, &infos), long_description_row(scan, &infos), big_body_row(&infos)]
        .into_iter()
        .flatten()
        .collect()
}

fn memory_rows(env: &Env, scan: &TokenScan) -> Vec<Row> {
    let mut rows = Vec::new();
    for file in files::memory_files(env, LONG_LINE) {
        let size = tokens(file.chars);
        if size <= MEMORY_TOKENS && (file.long_lines == 0 || size < MIN_LONG_LINE_TOKENS) {
            continue;
        }
        let mut text = format!("{} is ~{} tokens, {}", file.label, short(size), cost(scan, size));
        if file.long_lines > 0 {
            text.push_str(&format!(", {} lines over {LONG_LINE} characters", file.long_lines));
        }
        rows.push(row(
            Level::Warn,
            NAME,
            text,
            "keep one line of at most 120 characters per entry and the file under 2k tokens, move details to the vault and read them with wisply recall (no change made)",
        ));
    }
    rows
}

fn hook_rows(env: &Env, scan: &TokenScan) -> Vec<Row> {
    let mut rows = Vec::new();
    for (tag, kind) in files::legacy_leftovers(env) {
        let seen = scan.legacy_runs.get(&tag);
        let measured = seen.map_or(String::new(), |(runs, chars)| {
            format!(", {runs} runs in {WINDOW_DAYS} days printed ~{} tokens", short(tokens(*chars)))
        });
        rows.push(row(
            Level::Warn,
            NAME,
            format!("leftover {tag} {kind} still configured{measured}"),
            "remove that entry from the hooks or enabledPlugins block in ~/.claude/settings.json (no change made)",
        ));
    }
    rows
}

fn command_rows(env: &Env, scan: &TokenScan) -> Vec<Row> {
    let open = files::open_commands(env);
    if open.is_empty() {
        return Vec::new();
    }
    let mut each: Vec<(String, usize)> =
        open.iter().map(|c| (c.name.clone(), tokens(c.name.chars().count() + c.description + 4))).collect();
    let total: usize = each.iter().map(|i| i.1).sum();
    vec![row(
        Level::Warn,
        NAME,
        format!(
            "{} Wisply slash commands without disable-model-invocation, {}: {}",
            open.len(),
            cost(scan, total),
            names_line(&ranked(&mut each))
        ),
        "add disable-model-invocation: true to the frontmatter of commands you only run by hand (no change made)",
    )]
}

pub fn rows(env: &Env, scan: &TokenScan) -> Vec<Row> {
    let mut rows = Vec::new();
    if scan.sessions == 0 {
        rows.push(row(
            Level::Ok,
            NAME,
            format!("no transcripts in the last {WINDOW_DAYS} days, usage checks skipped"),
            "",
        ));
    } else {
        rows.extend(mcp_rows(env, scan));
        rows.extend(skill_rows(env, scan));
    }
    rows.extend(memory_rows(env, scan));
    rows.extend(hook_rows(env, scan));
    rows.extend(command_rows(env, scan));
    if rows.iter().all(|r| r.level == Level::Ok) {
        rows.push(row(Level::Ok, NAME, "no token waste found in skills, servers, memory files, hooks or commands", ""));
    }
    rows
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::super::tokens_scan::scan_with;
    use super::*;

    fn world(tag: &str) -> Env {
        let root = std::env::temp_dir().join(format!("wp-tok-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let claude = root.join("claude");
        fs::create_dir_all(claude.join("projects/p")).unwrap();
        fs::create_dir_all(root.join("proj/.claude-plugin")).unwrap();
        fs::create_dir_all(root.join("proj/commands")).unwrap();
        Env { user: root.clone(), claude, config: root.join(".claude.json"), cwd: root.join("proj") }
    }

    fn all(rows: &[Row]) -> String {
        rows.iter().map(|r| format!("{} {} | {}\n", r.name, r.text, r.fix)).collect()
    }

    #[test]
    fn numbers_are_short_and_ranked_by_cost() {
        assert_eq!((short(999), short(1500), short(12_345)), ("999".into(), "1.5k".into(), "12k".into()));
        let mut items = vec![("b".to_string(), 5), ("a".to_string(), 5), ("c".to_string(), 90)];
        assert_eq!(ranked(&mut items), ["c ~90", "a ~5", "b ~5"]);
    }

    #[test]
    fn invented_setup_gives_every_item_with_numbers() {
        let env = world("all");
        let long = "x".repeat(300);
        let body = "y".repeat(14_000);
        for (name, desc, text) in
            [("fat", long.as_str(), body.as_str()), ("lean", "short", "body"), ("used", "ok", "b")]
        {
            let dir = env.claude.join("skills").join(name);
            fs::create_dir_all(&dir).unwrap();
            fs::write(dir.join("SKILL.md"), format!("---\nname: {name}\ndescription: {desc}\n---\n{text}\n")).unwrap();
        }
        fs::write(env.claude.join("CLAUDE.md"), format!("{}\nshort\n", "z".repeat(900))).unwrap();
        fs::write(
            env.claude.join("settings.json"),
            r#"{"hooks":{"SessionStart":[{"hooks":[{"type":"command","command":"node claude-mem/start.js"}]}]}}"#,
        )
        .unwrap();
        fs::write(env.cwd.join("commands/hand.md"), "---\ndescription: by hand\n---\nrun\n").unwrap();
        fs::write(env.cwd.join("commands/safe.md"), "---\ndescription: x\ndisable-model-invocation: true\n---\n")
            .unwrap();
        let session = [
            r#"{"type":"assistant","requestId":"r1","message":{"content":[{"type":"tool_use","id":"a","name":"mcp__live__go","input":{}},{"type":"tool_use","id":"b","name":"Skill","input":{"skill":"used"}}]}}"#,
            r#"{"type":"assistant","requestId":"r2","message":{"content":[]}}"#,
            r#"{"type":"attachment","attachment":{"type":"deferred_tools_delta","addedNames":["mcp__live__go","mcp__idle_srv__one","mcp__idle_srv__two"]}}"#,
            r#"{"type":"attachment","attachment":{"type":"mcp_instructions_delta","addedNames":["idle srv"],"addedBlocks":["0123456789012345678901234567890123456789"]}}"#,
            r#"{"type":"attachment","attachment":{"type":"skill_listing","content":"- fat: a\n- lean: b\n- used: c","skillCount":3}}"#,
            r#"{"type":"attachment","attachment":{"type":"hook_success","command":"node claude-mem/start.js","stdout":"12345678"}}"#,
        ];
        fs::write(env.claude.join("projects/p/s.jsonl"), session.join("\n")).unwrap();
        let found = scan_with(&env, Some(&env.user));
        assert_eq!((found.sessions, found.requests), (1, 2));
        let text = all(&rows(&env, &found));
        assert!(
            text.contains("1 of 2 MCP servers/connectors without a call in 14 days") && text.contains("idle_srv ~"),
            "{text}"
        );
        assert!(
            text.contains("2 of 3 listed skills not called") && text.contains("fat") && text.contains("lean"),
            "{text}"
        );
        assert!(text.contains("1 skill descriptions over 250 characters"), "{text}");
        assert!(text.contains("1 skill bodies over 3.0k tokens") && text.contains("fat ~3.5k"), "{text}");
        assert!(text.contains("CLAUDE.md (user) is ~") && text.contains("1 lines over 120 characters"), "{text}");
        assert!(
            text.contains("leftover claude-mem hook still configured, 1 runs in 14 days printed ~2 tokens"),
            "{text}"
        );
        assert!(
            text.contains("1 Wisply slash commands without disable-model-invocation") && text.contains("hand"),
            "{text}"
        );
        assert!(!text.contains("safe"), "{text}");
        assert!(text.matches("no change made").count() >= 5, "{text}");
        fs::remove_dir_all(env.user).unwrap();
    }

    #[test]
    fn per_day_scales_with_sessions_over_the_window() {
        let found = TokenScan { sessions: 280, ..TokenScan::default() };
        assert_eq!(found.per_day(100), 2000);
        assert_eq!(cost(&found, 100), "~100 tok/request, ~2.0k tok/day");
    }

    #[test]
    fn a_clean_setup_and_no_transcripts_stay_green() {
        let env = world("clean");
        fs::write(env.claude.join("settings.json"), "{}").unwrap();
        let rows = rows(&env, &scan_with(&env, Some(&env.user)));
        assert!(rows.iter().all(|r| r.level == Level::Ok), "{}", all(&rows));
        assert!(all(&rows).contains("no transcripts"), "{}", all(&rows));
        fs::remove_dir_all(env.user).unwrap();
    }

    #[test]
    fn two_hundred_transcripts_scan_fast() {
        let env = world("speed");
        let line = r#"{"type":"user","message":{"content":"filler line that is long enough to look like a real transcript entry"}}"#;
        let call = r#"{"type":"assistant","requestId":"r","message":{"content":[{"type":"tool_use","id":"a","name":"Skill","input":{"skill":"s"}}]}}"#;
        let body = format!("{}\n{call}\n", vec![line; 600].join("\n"));
        for n in 0..200 {
            fs::write(env.claude.join(format!("projects/p/s{n}.jsonl")), &body).unwrap();
        }
        let cold = scan_with(&env, Some(&env.user));
        assert_eq!((cold.sessions, cold.requests), (200, 200));
        let start = std::time::Instant::now();
        let found = scan_with(&env, Some(&env.user));
        let took = start.elapsed();
        assert_eq!((found.sessions, found.requests, found.skills_used["s"]), (200, 200, 200));
        assert!(took < std::time::Duration::from_secs(1), "{took:?}");
        fs::remove_dir_all(env.user).unwrap();
    }
}
