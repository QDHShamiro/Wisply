use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Args {
    pub video: PathBuf,
    pub start: Option<f64>,
    pub end: Option<f64>,
    pub config: Option<PathBuf>,
    pub speech_model: Option<String>,
    pub sound_model: Option<String>,
    pub only: Option<Part>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Part {
    Speech,
    Sound,
}

fn number(flag: &str, value: Option<String>) -> Result<f64, String> {
    let value = value.ok_or_else(|| format!("{flag} needs a value"))?;
    value
        .parse::<f64>()
        .ok()
        .filter(|v| v.is_finite() && *v >= 0.0)
        .ok_or_else(|| format!("{flag} needs a number of seconds, got {value}"))
}

pub fn parse(items: impl IntoIterator<Item = String>) -> Result<Args, String> {
    let mut args = Args::default();
    let mut video: Option<PathBuf> = None;
    let mut iter = items.into_iter();
    while let Some(item) = iter.next() {
        match item.as_str() {
            "--start" => args.start = Some(number("--start", iter.next())?),
            "--end" => args.end = Some(number("--end", iter.next())?),
            "--config" => args.config = Some(PathBuf::from(iter.next().ok_or("--config needs a file")?)),
            "--speech" => args.speech_model = Some(iter.next().ok_or("--speech needs a name")?),
            "--sound" => args.sound_model = Some(iter.next().ok_or("--sound needs a name")?),
            "--only" => {
                args.only = Some(match iter.next().as_deref() {
                    Some("speech") => Part::Speech,
                    Some("sound") => Part::Sound,
                    other => return Err(format!("--only needs speech or sound, got {other:?}")),
                })
            }
            flag if flag.starts_with("--") => return Err(format!("unknown option {flag}")),
            _ => {
                if video.replace(PathBuf::from(&item)).is_some() {
                    return Err("only one video".into());
                }
            }
        }
    }
    args.video = video.ok_or("no video given")?;
    if let (Some(start), Some(end)) = (args.start, args.end)
        && end <= start
    {
        return Err("--end must be after --start".into());
    }
    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_str(line: &str) -> Result<Args, String> {
        parse(line.split_whitespace().map(String::from))
    }

    #[test]
    fn video_only() {
        let args = parse_str("a.mp4").unwrap();
        assert_eq!(args.video, PathBuf::from("a.mp4"));
        assert_eq!(args.start, None);
    }

    #[test]
    fn full_set() {
        let args = parse_str("a.mp4 --start 5 --end 20.5 --only sound --speech small --sound ced-mini --config x.toml")
            .unwrap();
        assert_eq!(args.start, Some(5.0));
        assert_eq!(args.end, Some(20.5));
        assert_eq!(args.only, Some(Part::Sound));
        assert_eq!(args.speech_model.as_deref(), Some("small"));
        assert_eq!(args.sound_model.as_deref(), Some("ced-mini"));
        assert_eq!(args.config, Some(PathBuf::from("x.toml")));
    }

    #[test]
    fn rejects_bad_input() {
        assert!(parse_str("").is_err());
        assert!(parse_str("a.mp4 b.mp4").is_err());
        assert!(parse_str("a.mp4 --start").is_err());
        assert!(parse_str("a.mp4 --start x").is_err());
        assert!(parse_str("a.mp4 --start -3").is_err());
        assert!(parse_str("a.mp4 --start 9 --end 4").is_err());
        assert!(parse_str("a.mp4 --only music").is_err());
        assert!(parse_str("a.mp4 --wat").is_err());
    }
}
