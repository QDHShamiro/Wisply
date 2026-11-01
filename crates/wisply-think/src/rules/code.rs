use std::collections::{BTreeSet, HashMap};
use std::sync::LazyLock;

use regex::Regex;

use super::Finding;
use crate::files::Pack;

const FILE_LINES: usize = 500;
const FN_LINES: usize = 60;
const FN_BRANCHES: usize = 15;
const NEST_MAX: usize = 5;
const LINE_CHARS: usize = 160;
const DUP_WINDOW: usize = 8;
const DUP_MIN_CHARS: usize = 6;

fn re(p: &str) -> Regex {
    Regex::new(p).expect("code rule pattern")
}

static RUST_FN: LazyLock<Regex> =
    LazyLock::new(|| re(r#"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:(?:const|async|unsafe|extern\s+"C")\s+)*fn\s+(\w+)"#));
static JAVA_FN: LazyLock<Regex> = LazyLock::new(|| {
    re(
        r"^\s*(?:(?:public|private|protected|static|final|abstract|synchronized|override|open|internal|suspend|default|inline|operator)\s+)*(?:fun\s+(?:<[^>]*>\s*)?(?:\w+\.)?(\w+)|[\w<>\[\]?,.]+(?:<[^>]*>)?\s+(\w+))\s*\(",
    )
});
static JS_FN: LazyLock<Regex> = LazyLock::new(|| {
    re(
        r"^\s*(?:export\s+)?(?:default\s+)?(?:async\s+)?function\s*\*?\s*(\w+)|^\s*(?:export\s+)?(?:const|let)\s+(\w+)\s*=\s*(?:async\s+)?(?:\([^)]*\)|\w+)\s*(?::\s*[^=]+)?=>|^\s+(?:async\s+|static\s+|private\s+|public\s+)*(\w+)\s*\([^)]*\)\s*(?::\s*[^{]+)?\{\s*$",
    )
});
static GO_FN: LazyLock<Regex> = LazyLock::new(|| re(r"^func\s+(?:\([^)]*\)\s*)?(\w+)"));
static PY_FN: LazyLock<Regex> = LazyLock::new(|| re(r"^(\s*)(?:async\s+)?def\s+(\w+)"));
static BRANCH: LazyLock<Regex> =
    LazyLock::new(|| re(r"\b(?:if|for|while|case|catch|match|elif|except|when)\b|&&|\|\||\?\s"));
static KEYWORD: LazyLock<Regex> =
    LazyLock::new(|| re(r"^\s*(?:if|for|while|switch|catch|return|new|else|do|try|synchronized|throw)\b"));
static TODO: LazyLock<Regex> = LazyLock::new(|| re(r"(?://|#|/\*|\*|--)\s*(?:TODO|FIXME|XXX|HACK)\b"));

pub struct Function {
    pub name: String,
    pub line: usize,
    pub lines: usize,
    pub branches: usize,
    pub depth: usize,
}

fn name_of(caps: &regex::Captures) -> Option<String> {
    caps.iter().skip(1).flatten().last().map(|m| m.as_str().to_string())
}

fn starts(lang: &str, line: &str) -> Option<String> {
    let caps = match lang {
        "rs" => RUST_FN.captures(line),
        "java" | "kt" | "cs" => {
            if KEYWORD.is_match(line) || line.trim_end().ends_with(';') || line.contains('=') && !line.contains("fun ")
            {
                return None;
            }
            JAVA_FN.captures(line)
        }
        "js" | "ts" | "tsx" | "jsx" | "mjs" | "cjs" | "vue" | "svelte" => {
            if KEYWORD.is_match(line) {
                return None;
            }
            JS_FN.captures(line)
        }
        "go" => GO_FN.captures(line),
        _ => None,
    }?;
    name_of(&caps)
}

static CHARLIT: LazyLock<Regex> = LazyLock::new(|| re(r"'(?:\\.|[^'\\])'"));

fn strip_strings(line: &str) -> String {
    let line = CHARLIT.replace_all(line, "' '");
    let mut out = String::with_capacity(line.len());
    let mut quote: Option<char> = None;
    let mut prev = ' ';
    for c in line.chars() {
        match quote {
            Some(q) if c == q && prev != '\\' => quote = None,
            Some(_) => {}
            None if c == '"' || c == '`' => quote = Some(c),
            None if c == '/' && prev == '/' => {
                out.pop();
                break;
            }
            None => out.push(c),
        }
        prev = if prev == '\\' && c == '\\' { ' ' } else { c };
    }
    out
}

pub fn functions(lang: &str, text: &str) -> Vec<Function> {
    let lines: Vec<&str> = text.lines().collect();
    if lang == "py" {
        return python_functions(&lines);
    }
    let mut out = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let Some(name) = starts(lang, lines[i]) else {
            i += 1;
            continue;
        };
        let (mut depth, mut opened, mut deepest, mut branches) = (0usize, false, 0usize, 0usize);
        let mut j = i;
        while j < lines.len() {
            let code = strip_strings(lines[j]);
            branches += BRANCH.find_iter(&code).count();
            for c in code.chars() {
                if c == '{' {
                    depth += 1;
                    opened = true;
                    deepest = deepest.max(depth);
                } else if c == '}' {
                    depth = depth.saturating_sub(1);
                }
            }
            if !opened && (code.trim_end().ends_with(';') || j > i + 6) {
                break;
            }
            if opened && depth == 0 {
                break;
            }
            j += 1;
        }
        if opened {
            out.push(Function { name, line: i + 1, lines: j - i + 1, branches, depth: deepest.saturating_sub(1) });
            i += 1;
        } else {
            i = j + 1;
        }
    }
    out
}

fn python_functions(lines: &[&str]) -> Vec<Function> {
    let indent = |l: &str| l.len() - l.trim_start().len();
    let mut out = Vec::new();
    for (i, l) in lines.iter().enumerate() {
        let Some(c) = PY_FN.captures(l) else { continue };
        let base = c[1].len();
        let (mut end, mut branches, mut deepest) = (i, 0, 0);
        for (j, m) in lines.iter().enumerate().skip(i + 1) {
            if m.trim().is_empty() {
                continue;
            }
            if indent(m) <= base {
                break;
            }
            end = j;
            branches += BRANCH.find_iter(m).count();
            deepest = deepest.max((indent(m) - base) / 4);
        }
        out.push(Function {
            name: c[2].to_string(),
            line: i + 1,
            lines: end - i + 1,
            branches,
            depth: deepest.saturating_sub(1),
        });
    }
    out
}

fn first_line(text: &str, pat: &Regex, test_cut: usize) -> Option<(usize, usize)> {
    let mut first = None;
    let mut n = 0;
    for (i, l) in text.lines().enumerate() {
        if i + 1 >= test_cut {
            break;
        }
        if pat.is_match(l) && !l.trim_start().starts_with("//") {
            n += 1;
            first.get_or_insert(i + 1);
        }
    }
    first.map(|f| (f, n))
}

const SMELL_TABLE: &str = r"
rs | rust | debug-output | 2 3 1 | dbg! left in | remove it | \bdbg!\(
rs | rust | todo-macro | 3 3 2 | todo!/unimplemented! panics | finish it | \b(?:todo|unimplemented)!\(
rs | rust | unwrap | 2 2 1 | unwrap() can panic | use ? or expect with the reason | \.unwrap\(\)
java,kt | code | debug-output | 2 3 1 | System.out/err print | use the plugin logger | System\.(?:out|err)\.print
java,kt | code | stack-trace | 2 3 1 | printStackTrace() | log it with the logger and the exception | \.printStackTrace\(\)
java,kt | java-plugin | bukkit-scheduler | 3 3 2 | Bukkit scheduler breaks on Folia | use the global/region/entity schedulers | Bukkit\.getScheduler\(\)|\.runTaskTimer\(|BukkitRunnable
java,kt | java-plugin | shared-hashmap | 2 2 1 | HashMap field shared across threads | use ConcurrentHashMap | ^\s*(?:final\s+)?(?:private|protected|public|static)\s+(?:(?:static|final|volatile|transient)\s+)*(?:Map|HashMap)<[^;]*=\s*new\s+HashMap<|^(?:\t| {0,4})(?:final\s+)?(?:Map|HashMap)<[^;]*=\s*new\s+HashMap<
java,kt | java-plugin | gui-by-title | 2 3 2 | GUI recognised by its title | give the GUI its own InventoryHolder and check instanceof | getView\(\)\.getTitle\(\)|\.getTitle\(\)\s*\.\s*(?:equals|contains|startsWith)
java,kt | java-plugin | legacy-color | 1 2 2 | legacy ChatColor/§ colors | use Adventure components or a color util | \bChatColor\.|§[0-9a-fk-or]
py | python | bare-except | 2 3 1 | bare except hides every error | catch the real exception | ^\s*except\s*:
py | python | mutable-default | 2 3 1 | mutable default argument is shared between calls | default to None and create it inside | def\s+\w+\([^)]*=\s*(?:\[\]|\{\}|set\(\))
js,ts,tsx,jsx,mjs,cjs,vue,svelte | web | debug-output | 1 3 1 | console.log left in | remove it | \bconsole\.log\(
ts,tsx | web | any-type | 1 2 2 | any turns type checks off | give it a real type | :\s*any\b|<any>|\bas\s+any\b
js,ts,tsx,jsx,mjs,cjs,vue,svelte | web | loose-equal | 1 2 1 | == / != convert types | use === / !== | [^=!<>]\s(?:==|!=)\s[^=]
";

struct Smell {
    langs: Vec<&'static str>,
    pack: Pack,
    rule: &'static str,
    rate: (u8, u8, u8),
    what: &'static str,
    fix: &'static str,
    pat: Regex,
}

static SMELLS: LazyLock<Vec<Smell>> = LazyLock::new(|| {
    SMELL_TABLE
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let f: Vec<&str> = l.splitn(7, '|').map(str::trim).collect();
            let n: Vec<u8> = f[3].split_whitespace().filter_map(|x| x.parse().ok()).collect();
            Smell {
                langs: f[0].split(',').collect(),
                pack: Pack::from_name(f[1]).expect("smell pack"),
                rule: f[2],
                rate: (n[0], n[1], n[2]),
                what: f[4],
                fix: f[5],
                pat: re(f[6]),
            }
        })
        .collect()
});

fn counted(
    rule: &str,
    pack: Pack,
    hit: Option<(usize, usize)>,
    what: &str,
    fix: &str,
    rate: (u8, u8, u8),
) -> Option<Finding> {
    let (line, n) = hit?;
    let why = if n > 1 { format!("{what} ({n} times)") } else { what.to_string() };
    Some(Finding::new(rule, pack, line, why, fix).rate(rate.0, rate.1, rate.2))
}

pub fn check(lang: &str, text: &str, test: bool, packs: &BTreeSet<Pack>) -> Vec<Finding> {
    let mut out = Vec::new();
    let total = text.lines().count();
    if total > FILE_LINES {
        out.push(
            Finding::new("long-file", Pack::Code, 0, format!("{total} lines (over {FILE_LINES})"), "split it by job")
                .rate(2, 2, 2),
        );
    }
    for f in functions(lang, text) {
        if f.lines > FN_LINES {
            out.push(
                Finding::new(
                    "long-function",
                    Pack::Code,
                    f.line,
                    format!("{} is {} lines", f.name, f.lines),
                    "split it into named steps",
                )
                .rate(2, 2, 2),
            );
        }
        if f.branches > FN_BRANCHES {
            out.push(
                Finding::new(
                    "complex-function",
                    Pack::Code,
                    f.line,
                    format!("{} has {} branches", f.name, f.branches),
                    "pull branches into small functions or a table",
                )
                .rate(2, 2, 2),
            );
        }
        if f.depth > NEST_MAX {
            out.push(
                Finding::new(
                    "deep-nesting",
                    Pack::Code,
                    f.line,
                    format!("{} nests {} levels", f.name, f.depth),
                    "return early or move the inner part out",
                )
                .rate(2, 2, 2),
            );
        }
    }
    let long: Vec<usize> =
        text.lines().enumerate().filter(|(_, l)| l.chars().count() > LINE_CHARS).map(|(i, _)| i + 1).collect();
    if let Some(first) = long.first() {
        out.push(
            Finding::new(
                "long-line",
                Pack::Code,
                *first,
                format!("{} lines over {LINE_CHARS} chars", long.len()),
                "wrap them",
            )
            .rate(1, 2, 1),
        );
    }
    let todo = text.lines().enumerate().filter(|(_, l)| TODO.is_match(l)).map(|(i, _)| i + 1).collect::<Vec<_>>();
    if let Some(first) = todo.first() {
        out.push(
            Finding::new(
                "todo",
                Pack::Code,
                *first,
                format!("{} TODO/FIXME markers", todo.len()),
                "do it or track it as a to-do",
            )
            .rate(1, 2, 2),
        );
    }
    if test {
        return out;
    }
    let cut = if lang == "rs" {
        text.lines().position(|l| l.trim() == "#[cfg(test)]").map_or(usize::MAX, |i| i + 1)
    } else {
        usize::MAX
    };
    for m in active(lang, packs) {
        out.extend(counted(m.rule, m.pack, first_line(text, &m.pat, cut), m.what, m.fix, m.rate));
    }
    out
}

fn active<'a>(lang: &'a str, packs: &'a BTreeSet<Pack>) -> impl Iterator<Item = &'static Smell> + 'a {
    let plugin = packs.contains(&Pack::JavaPlugin);
    SMELLS.iter().filter(move |m| m.langs.contains(&lang) && (m.pack != Pack::JavaPlugin || plugin))
}

pub fn smells_on(lang: &str, line: &str, packs: &BTreeSet<Pack>) -> Vec<&'static str> {
    if !packs.contains(&Pack::Code) || line.trim_start().starts_with("//") {
        return Vec::new();
    }
    active(lang, packs).filter(|m| m.pat.is_match(line)).map(|m| m.rule).collect()
}

fn significant(line: &str) -> Option<String> {
    let t = line.trim();
    let bare = t.trim_matches(|c: char| "{}()[];,".contains(c));
    if bare.len() < DUP_MIN_CHARS
        || t.starts_with("//")
        || t.starts_with('#')
        || t.starts_with("import ")
        || t.starts_with("use ")
    {
        return None;
    }
    Some(t.split_whitespace().collect::<Vec<_>>().join(" "))
}

pub fn duplicates(files: &[(String, String)]) -> Vec<Finding> {
    let mut seen: HashMap<u64, (usize, usize)> = HashMap::new();
    let mut out = Vec::new();
    let mut said: BTreeSet<(usize, usize)> = BTreeSet::new();
    for (fi, (_, text)) in files.iter().enumerate() {
        let lines: Vec<(usize, String)> =
            text.lines().enumerate().filter_map(|(i, l)| significant(l).map(|s| (i + 1, s))).collect();
        let mut k = 0;
        while k + DUP_WINDOW <= lines.len() {
            let mut h: u64 = 0xcbf2_9ce4_8422_2325;
            for (_, s) in &lines[k..k + DUP_WINDOW] {
                for b in s.bytes().chain(*b"\n") {
                    h = (h ^ u64::from(b)).wrapping_mul(0x100_0000_01b3);
                }
            }
            match seen.get(&h) {
                Some(&(of, at)) if !(of == fi && at + DUP_WINDOW > lines[k].0) => {
                    if said.insert((fi, of)) {
                        out.push(
                            Finding::new(
                                "duplicate",
                                Pack::Code,
                                lines[k].0,
                                format!("same {DUP_WINDOW}+ lines as {}:{at}", files[of].0),
                                "keep one copy and call it",
                            )
                            .rate(2, 2, 2),
                        );
                        out.last_mut().expect("just pushed").file = files[fi].0.clone();
                    }
                    k += DUP_WINDOW;
                }
                Some(_) => k += 1,
                None => {
                    seen.insert(h, (fi, lines[k].0));
                    k += 1;
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn packs(list: &[Pack]) -> BTreeSet<Pack> {
        list.iter().copied().collect()
    }

    #[test]
    fn functions_are_measured_per_language() {
        let body = "    if a { x(); }\n".repeat(70);
        let rust = format!("pub fn big(a: bool) {{\n{body}}}\nfn small() {{ let s = \"{{\"; }}\n");
        let f = functions("rs", &rust);
        assert_eq!(f.len(), 2);
        assert_eq!((f[0].name.as_str(), f[0].lines, f[0].branches), ("big", 72, 70));
        assert_eq!((f[1].name.as_str(), f[1].lines), ("small", 1));
        let java = "public class A {\n  private void run(int a) {\n    if (a > 1) {\n      go();\n    }\n  }\n  abstract void x();\n}\n";
        let f = functions("java", java);
        assert_eq!(f.iter().map(|f| (f.name.as_str(), f.lines)).collect::<Vec<_>>(), [("run", 5)]);
        let py = "def a(x):\n    if x:\n        return 1\n    return 2\n\ndef b():\n    pass\n";
        let f = functions("py", py);
        assert_eq!(
            f.iter().map(|f| (f.name.as_str(), f.lines, f.branches)).collect::<Vec<_>>(),
            [("a", 4, 1), ("b", 2, 0)]
        );
        let ts = "export function go(a: number): void {\n  console.log(a);\n}\nconst f = (x) => {\n  return x;\n};\n";
        assert_eq!(functions("ts", ts).iter().map(|f| f.name.as_str()).collect::<Vec<_>>(), ["go", "f"]);
    }

    #[test]
    fn language_rules_fire_outside_tests_only() {
        let rust = "fn a() { let x = y.unwrap(); dbg!(x); }\n#[cfg(test)]\nmod tests { fn t() { z.unwrap(); } }\n";
        let got: Vec<String> =
            check("rs", rust, false, &packs(&[Pack::Code, Pack::Rust])).iter().map(|f| f.rule.clone()).collect();
        assert_eq!(got, ["debug-output", "unwrap"]);
        let java = "class A {\n  private final Map<UUID, Integer> m = new HashMap<>();\n  void a() { Bukkit.getScheduler().runTask(p, r); System.out.println(1); e.printStackTrace(); }\n}\n";
        let got: Vec<String> = check("java", java, false, &packs(&[Pack::Code, Pack::JavaPlugin]))
            .iter()
            .map(|f| f.rule.clone())
            .collect();
        assert_eq!(got, ["debug-output", "stack-trace", "bukkit-scheduler", "shared-hashmap"]);
        assert!(check("java", java, true, &packs(&[Pack::Code, Pack::JavaPlugin])).is_empty());
        let plugin = packs(&[Pack::Code, Pack::JavaPlugin]);
        let field = |line: &str| smells_on("java", line, &plugin).contains(&"shared-hashmap");
        assert!(
            field("    Map<UUID, Integer> m = new HashMap<>();") && field("\tprivate Map<A, B> m = new HashMap<>();")
        );
        assert!(field("        private static final Map<A, B> M = new HashMap<>();"));
        assert!(!field("        Map<String, String> ph = new HashMap<>();"));
        assert!(!field("        final Map<Location, BlockData> changes = new HashMap<>();"));
        let ts = "let a: any = 1;\nif (a == 2) console.log(a);\n";
        let got: Vec<String> = check("ts", ts, false, &packs(&[Pack::Code])).iter().map(|f| f.rule.clone()).collect();
        assert_eq!(got, ["debug-output", "any-type", "loose-equal"]);
    }

    #[test]
    fn every_smell_row_parses() {
        assert_eq!(SMELLS.len(), SMELL_TABLE.lines().filter(|l| !l.trim().is_empty()).count());
        assert!(SMELLS.iter().all(|m| m.rate.0 > 0 && m.rate.1 > 0 && m.rate.2 > 0 && !m.fix.is_empty()));
    }

    #[test]
    fn copied_blocks_are_found_once() {
        let block: String = (0..10).map(|i| format!("let value_{i} = compute({i});\n")).collect();
        let files = vec![("a.rs".to_string(), block.clone()), ("b.rs".to_string(), format!("fn x() {{}}\n{block}"))];
        let d = duplicates(&files);
        assert_eq!(d.len(), 1);
        assert_eq!((d[0].file.as_str(), d[0].line), ("b.rs", 2));
        assert!(d[0].why.contains("a.rs:1"));
    }
}
