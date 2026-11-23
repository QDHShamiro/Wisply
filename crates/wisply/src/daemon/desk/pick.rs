use wisply_core::Fail;

use super::super::web::ref_kind;
use super::view::{score, short};
use super::{AGENT_EXES, BROWSER_EXES, CHAT_EXES, Desk, Res, SHELL_CLASSES, TERMINALS, win, windowish};

const OPEN_LISTED: usize = 12;

impl Desk<'_> {
    pub fn list_windows(&mut self) -> Vec<String> {
        let mut lines = Vec::new();
        for w in win::windows() {
            let r = match self.st.window_ref(w.hwnd) {
                Some(r) => r,
                None => {
                    let mut n = self.st.windows.len() + 1;
                    while self.st.windows.contains_key(&format!("w{n}")) {
                        n += 1;
                    }
                    self.st.windows.insert(format!("w{n}"), w.hwnd);
                    format!("w{n}")
                }
            };
            self.st.info.insert(r.clone(), (w.exe.to_lowercase(), w.title.clone()));
            if w.fg {
                self.st.seen_front = w.hwnd;
            }
            let flag = format!("{}{}", if w.fg { " *" } else { "" }, if w.min { " (min)" } else { "" });
            let who = if w.exe.is_empty() { &w.cls } else { &w.exe };
            let n: u64 = r[1..].parse().unwrap_or(0);
            lines.push((n, format!("{r} {} - {who}{flag}", short(&w.title, 60))));
        }
        lines.sort();
        lines.into_iter().map(|(_, l)| l).collect()
    }

    fn successor(&self, spec: &str) -> Option<isize> {
        let (exe, title) = self.st.info.get(spec)?;
        let mut same: Vec<win::Win> = win::windows().into_iter().filter(|w| w.exe.to_lowercase() == *exe).collect();
        if same.len() > 1 {
            same.retain(|w| w.title == *title);
        }
        (same.len() == 1).then(|| same[0].hwnd)
    }

    pub fn find_window(&mut self, spec: &str) -> Res<isize> {
        if ref_kind(spec) == Some("w") {
            if !self.st.windows.contains_key(spec) {
                self.list_windows();
            }
            let hwnd = self.st.windows.get(spec).copied().unwrap_or(0);
            if hwnd != 0
                && !win::alive(hwnd)
                && let Some(new) = self.successor(spec)
            {
                self.st.windows.insert(spec.to_string(), new);
                let exe = self.st.info.get(spec).map(|i| i.0.clone()).unwrap_or_default();
                self.note(&format!("the old {spec} was closed; {spec} is now the new {exe} window"));
                return Ok(new);
            }
            if hwnd == 0 || !win::alive(hwnd) {
                return Err(Fail::hint(format!("window {spec} is gone (closed)"), "wisply windows").into());
            }
            return Ok(hwnd);
        }
        let text = if spec.to_lowercase().starts_with("app:") { &spec[4..] } else { spec };
        let mut best: Option<(u8, isize)> = None;
        for w in win::windows() {
            let s = score(text, &w.title, &w.exe);
            if s > 0 && best.is_none_or(|(b, _)| s > b) {
                best = Some((s, w.hwnd));
            }
        }
        let Some((_, hwnd)) = best else {
            return Err(Fail::hint(format!("no window matches \"{text}\""), "wisply windows").into());
        };
        self.list_windows();
        Ok(hwnd)
    }

    fn front_app(&self) -> Option<isize> {
        let hwnd = win::foreground();
        let cls = win::class_of(hwnd);
        if hwnd == 0
            || hwnd != self.st.seen_front
            || !win::alive(hwnd)
            || TERMINALS.contains(&cls.as_str())
            || SHELL_CLASSES.contains(&cls.as_str())
        {
            return None;
        }
        let pid = win::pid_of(hwnd);
        let exe = win::exe_of(pid).to_lowercase();
        let skip = [AGENT_EXES, BROWSER_EXES, CHAT_EXES].iter().any(|list| list.contains(&exe.as_str()));
        (!exe.is_empty() && !skip && !win::blocked(pid)).then_some(hwnd)
    }

    pub fn target(&mut self, first: Option<&str>) -> Res<isize> {
        let hwnd = match first.filter(|p| windowish(p)) {
            Some(spec) => self.find_window(spec)?,
            None if self.st.hwnd != 0 && win::alive(self.st.hwnd) => self.st.hwnd,
            None => {
                let Some(hwnd) = self.front_app() else {
                    let open = self.open_windows();
                    return Err(Fail::hint(
                        "no window chosen",
                        format!(
                            "name one, like wisply press w3 Enter or wisply snap app:<name>{}",
                            if open.is_empty() { String::new() } else { format!("\nopen windows:\n{open}") }
                        ),
                    )
                    .into());
                };
                let label = self.label(hwnd);
                self.note(&format!("no window was chosen: used {label}, still in front since your last windows"));
                hwnd
            }
        };
        self.st.hwnd = hwnd;
        self.ctx.target = Some(("app".into(), hwnd.to_string()));
        Ok(hwnd)
    }

    fn open_windows(&mut self) -> String {
        self.list_windows();
        let mut lines = Vec::new();
        for w in win::windows().into_iter().filter(|w| !w.title.trim().is_empty()) {
            let Some(r) = self.st.window_ref(w.hwnd) else { continue };
            let exe = w.exe.to_lowercase();
            let own = TERMINALS.contains(&w.cls.as_str()) || AGENT_EXES.contains(&exe.as_str());
            let mark = if own { " (terminal: only when named)" } else { "" };
            let n: u64 = r[1..].parse().unwrap_or(0);
            lines.push((own, n, format!("{r} {} - {exe}{mark}", short(&w.title, 50))));
        }
        lines.sort();
        lines.into_iter().take(OPEN_LISTED).map(|(.., l)| l).collect::<Vec<_>>().join("\n")
    }
}
