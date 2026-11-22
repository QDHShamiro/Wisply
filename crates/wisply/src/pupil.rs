use std::path::{Path, PathBuf};
use std::process::ExitCode;

const PUPIL_VAR: &str = "PUPIL_CONFIG";
const HEAR_VAR: &str = "PUPIL_HEAR_CONFIG";

pub fn run(home: &Path, args: &[String]) -> ExitCode {
    let code = match args.first().map(String::as_str) {
        Some("hear") => wisply_hear::run(&config(home, HEAR_VAR, "hear.toml"), &args[1..]),
        Some("limits" | "info" | "captions") => wisply_video::run(home, args),
        _ => wisply_pupil::run(&config(home, PUPIL_VAR, "pupil.toml"), args),
    };
    ExitCode::from(u8::try_from(code).unwrap_or(1))
}

fn config(home: &Path, var: &str, name: &str) -> PathBuf {
    std::env::var_os(var).filter(|v| !v.is_empty()).map(PathBuf::from).unwrap_or_else(|| home.join("pupil").join(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configs_live_in_the_pupil_folder_of_the_home() {
        let home = Path::new("h");
        assert_eq!(config(home, "WISPLY_TEST_UNSET_PUPIL_VAR", "pupil.toml"), home.join("pupil").join("pupil.toml"));
        assert_eq!(config(home, "WISPLY_TEST_UNSET_HEAR_VAR", "hear.toml"), home.join("pupil").join("hear.toml"));
    }
}
