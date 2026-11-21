use std::path::Path;

pub fn parse_csv(raw: &str) -> Result<Vec<String>, String> {
    let mut names = Vec::new();
    for (number, line) in raw.lines().enumerate().skip(1) {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.splitn(3, ',');
        let index = parts
            .next()
            .and_then(|p| p.parse::<usize>().ok())
            .ok_or_else(|| format!("line {}: no index", number + 1))?;
        parts.next();
        let name =
            parts.next().ok_or_else(|| format!("line {}: no name", number + 1))?.trim().trim_matches('"').to_string();
        if index != names.len() {
            return Err(format!("line {}: index {index} out of order", number + 1));
        }
        names.push(name);
    }
    if names.is_empty() {
        return Err("no labels".into());
    }
    Ok(names)
}

pub fn load(path: &Path) -> Result<Vec<String>, String> {
    let raw = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    parse_csv(&raw)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_quoted_names_with_commas() {
        let raw = "index,mid,display_name\n0,/m/09x0r,\"Speech\"\n1,/m/05zppz,\"Male speech, man speaking\"\n";
        assert_eq!(parse_csv(raw).unwrap(), vec!["Speech", "Male speech, man speaking"]);
    }

    #[test]
    fn rejects_gaps() {
        let raw = "index,mid,display_name\n0,/m/a,\"A\"\n2,/m/b,\"B\"\n";
        assert!(parse_csv(raw).is_err());
    }

    #[test]
    fn rejects_empty() {
        assert!(parse_csv("index,mid,display_name\n").is_err());
    }
}
