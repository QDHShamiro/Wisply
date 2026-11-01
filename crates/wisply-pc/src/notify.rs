use std::thread;
use std::time::Duration;

use crate::plat::shell;

const WAIT: Duration = Duration::from_secs(20);
const APP: &str = "{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\\WindowsPowerShell\\v1.0\\powershell.exe";
const MAX_CHARS: usize = 220;

fn clip(text: &str) -> String {
    text.chars().filter(|c| !c.is_control()).take(MAX_CHARS).collect()
}

pub fn windows_script(title: &str, body: &str) -> String {
    let quote = |t: &str| clip(t).replace('\'', "''");
    format!(
        "[Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime] | Out-Null; \
         $t = [Windows.UI.Notifications.ToastNotificationManager]::GetTemplateContent([Windows.UI.Notifications.ToastTemplateType]::ToastText02); \
         $n = $t.GetElementsByTagName('text'); \
         $n.Item(0).AppendChild($t.CreateTextNode('{}')) | Out-Null; \
         $n.Item(1).AppendChild($t.CreateTextNode('{}')) | Out-Null; \
         [Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier('{APP}').Show([Windows.UI.Notifications.ToastNotification]::new($t))",
        quote(title),
        quote(body)
    )
}

pub fn mac_script(title: &str, body: &str) -> String {
    let quote = |t: &str| clip(t).replace('\\', "\\\\").replace('"', "\\\"");
    format!("display notification \"{}\" with title \"{}\"", quote(body), quote(title))
}

fn run(title: &str, body: &str) {
    if cfg!(windows) {
        let script = windows_script(title, body);
        shell::run(
            "powershell",
            &["-NoProfile", "-NonInteractive", "-WindowStyle", "Hidden", "-Command", &script],
            WAIT,
        );
    } else if cfg!(target_os = "macos") {
        shell::run("osascript", &["-e", &mac_script(title, body)], WAIT);
    } else {
        shell::run("notify-send", &["-a", "Wisply", "-u", "normal", &clip(title), &clip(body)], WAIT);
    }
}

pub fn send(title: &str, body: &str) {
    let (title, body) = (title.to_string(), body.to_string());
    let _ = thread::Builder::new().name("pc-notify".into()).spawn(move || run(&title, &body));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn one_real_notification() {
        run("Wisply PC", "test notification from wisply pc (it never takes focus)");
    }

    #[test]
    fn quotes_cannot_break_out_of_the_scripts() {
        let w = windows_script("it's", "a'b'; Remove-Item x #");
        assert!(
            w.contains("CreateTextNode('it''s')") && w.contains("CreateTextNode('a''b''; Remove-Item x #')"),
            "{w}"
        );
        let m = mac_script("t\"x", "b\\\"; do shell script \"x");
        assert_eq!(m, "display notification \"b\\\\\\\"; do shell script \\\"x\" with title \"t\\\"x\"");
        assert!(clip(&"x".repeat(500)).len() == MAX_CHARS && clip("a\nb") == "ab");
    }
}
