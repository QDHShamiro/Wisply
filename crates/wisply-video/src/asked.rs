use crate::fail::{Fail, Kind};
use crate::source::{self, Source};
use crate::youtube::Video;

const FLAG_PREFIX: &str = "--";

#[derive(Debug)]
pub struct Asked {
    verb: String,
    source: Source,
    flags: Vec<(String, String)>,
}

fn quoted(text: &str) -> String {
    if text.contains(char::is_whitespace) { format!("\"{text}\"") } else { text.to_string() }
}

impl Asked {
    pub fn read(verb: &str, usage: &str, args: &[String], known: &[&str]) -> Result<Asked, Fail> {
        let (mut link, mut flags): (Option<&String>, Vec<(String, String)>) = (None, Vec::new());
        let mut rest = args.iter();
        while let Some(arg) = rest.next() {
            match arg.strip_prefix(FLAG_PREFIX) {
                Some(name) if known.contains(&name) => {
                    let Some(value) = rest.next() else {
                        return Err(Fail::hint(Kind::Input, format!("{arg} needs a value"), usage));
                    };
                    flags.push((name.to_string(), value.clone()));
                }
                Some(_) => return Err(Fail::hint(Kind::Input, format!("{verb} does not know {arg}"), usage)),
                None if link.is_some() => {
                    let message = format!("{verb} takes one video, {arg} is one too many");
                    return Err(Fail::hint(Kind::Input, message, "put words with spaces in quotes"));
                }
                None => link = Some(arg),
            }
        }
        let source = source::parse(link.map(String::as_str).unwrap_or_default())?;
        Ok(Asked { verb: verb.to_string(), source, flags })
    }

    pub fn flag(&self, name: &str) -> Option<&str> {
        self.flags.iter().rev().find(|(known, _)| known == name).map(|(_, value)| value.as_str())
    }

    pub fn video(&self) -> Result<Video, Fail> {
        let verb = &self.verb;
        match &self.source {
            Source::Local(path) => {
                let hint = format!("wisply pupil hear {}", quoted(&path.to_string_lossy()));
                Err(Fail::hint(Kind::Source, format!("{verb} reads YouTube links, not files"), hint))
            }
            Source::Page(link) => {
                let message = format!("{verb} reads YouTube links, not other pages");
                Err(Fail::hint(Kind::Source, message, format!("wisply listen {link}")))
            }
            source => {
                Video::of(source).ok_or_else(|| Fail::new(Kind::Source, format!("{verb} could not read this link")))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINK: &str = "https://youtu.be/abcdefghijk";
    const USAGE: &str = "wisply pupil captions <YouTube link>";
    const FILE: &str = if cfg!(windows) { "C:/clips/my clip.mp4" } else { "/clips/my clip.mp4" };

    fn args(words: &[&str]) -> Vec<String> {
        words.iter().map(|word| word.to_string()).collect()
    }

    fn refused(words: &[&str]) -> Fail {
        Asked::read("captions", USAGE, &args(words), &["start", "find"]).unwrap_err()
    }

    #[test]
    fn a_link_and_its_flags_are_read_in_any_order() {
        let words = args(&["--start", "1:00", LINK, "--find", "bevel edge"]);
        let asked = Asked::read("captions", USAGE, &words, &["start", "find"]).unwrap();
        assert_eq!(
            (asked.flag("start"), asked.flag("find"), asked.flag("end")),
            (Some("1:00"), Some("bevel edge"), None)
        );
        let video = asked.video().unwrap();
        assert_eq!(
            (video.id.as_str(), video.url.as_str()),
            ("abcdefghijk", "https://www.youtube.com/watch?v=abcdefghijk")
        );
        let twice = Asked::read("captions", USAGE, &args(&[LINK, "--start", "1", "--start", "2"]), &["start"]).unwrap();
        assert_eq!(twice.flag("start"), Some("2"));
    }

    #[test]
    fn wrong_arguments_are_named() {
        assert_eq!(refused(&[LINK, "--loud", "1"]).message, "captions does not know --loud");
        assert_eq!(refused(&[LINK, "--start"]).message, "--start needs a value");
        assert_eq!(refused(&[LINK, "second"]).message, "captions takes one video, second is one too many");
        let none = refused(&[]);
        assert_eq!((none.kind, none.message.as_str()), (Kind::Source, "no video was given"));
        assert_eq!(
            (refused(&[LINK, "--loud"]).hint.as_str(), refused(&[LINK, "--find"]).hint.as_str()),
            (USAGE, USAGE)
        );
    }

    #[test]
    fn files_and_other_pages_point_to_the_verb_that_reads_them() {
        let page = Asked::read("info", USAGE, &args(&["https://example.test/clip"]), &[]).unwrap().video().unwrap_err();
        assert_eq!(page.message, "info reads YouTube links, not other pages");
        assert_eq!(page.hint, "wisply listen https://example.test/clip");
        let file = Asked::read("info", USAGE, &args(&[FILE]), &[]).unwrap().video().unwrap_err();
        assert_eq!(file.message, "info reads YouTube links, not files");
        assert!(file.hint.starts_with("wisply pupil hear \"") && file.hint.ends_with("clip.mp4\""), "{}", file.hint);
    }
}
