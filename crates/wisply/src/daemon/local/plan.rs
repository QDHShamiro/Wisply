use super::Kind;

pub const BUSY_S: f64 = 10.0;
const MAX_THREADS: usize = 4;

#[derive(Debug, PartialEq, Eq)]
pub enum Budget {
    Fits,
    Kill(Vec<Kind>),
    Refuse,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Live {
    pub kind: Kind,
    pub mb: u64,
    pub used: f64,
}

pub fn expect(seen: u64, start: u64) -> u64 {
    if seen > 0 { seen } else { start }
}

pub fn budget(limit_mb: u64, running: &[Live], want: Kind, expect_mb: u64, now: f64) -> Budget {
    let others: Vec<&Live> = running.iter().filter(|l| l.kind != want).collect();
    let held: u64 = others.iter().map(|l| l.mb).sum();
    if others.is_empty() || held + expect_mb <= limit_mb {
        return Budget::Fits;
    }
    if others.iter().any(|l| now - l.used < BUSY_S) {
        return Budget::Refuse;
    }
    Budget::Kill(others.iter().map(|l| l.kind).collect())
}

pub fn gaming(mode: &str, playing: &str) -> bool {
    mode != "allow" && !playing.is_empty()
}

pub fn due(running: &[Live], now: f64, idle_s: u64, paused: bool) -> Vec<Kind> {
    running.iter().filter(|l| paused || now - l.used > idle_s as f64).map(|l| l.kind).collect()
}

pub fn threads(setting: i64, cores: usize) -> usize {
    match usize::try_from(setting) {
        Ok(n) if n > 0 => n,
        _ => (cores / 2).clamp(1, MAX_THREADS),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prio {
    Low,
    Idle,
}

pub fn priority(kind: Kind, mode: &str, playing: &str) -> Prio {
    let idle = kind == Kind::Llm && mode == "allow" && !playing.is_empty();
    if idle { Prio::Idle } else { Prio::Low }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn live(kind: Kind, mb: u64, used: f64) -> Live {
        Live { kind, mb, used }
    }

    #[test]
    fn two_programs_that_fit_run_side_by_side() {
        let run = [live(Kind::Asr, 300, 0.0)];
        assert_eq!(budget(1024, &run, Kind::Llm, 600, 100.0), Budget::Fits);
        assert_eq!(budget(1024, &[], Kind::Llm, 2000, 100.0), Budget::Fits);
    }

    #[test]
    fn the_idle_other_program_is_ended_to_make_room() {
        let run = [live(Kind::Asr, 600, 50.0)];
        assert_eq!(budget(1024, &run, Kind::Llm, 700, 100.0), Budget::Kill(vec![Kind::Asr]));
    }

    #[test]
    fn a_busy_other_program_is_not_ended_the_start_is_refused() {
        let run = [live(Kind::Asr, 600, 95.0)];
        assert_eq!(budget(1024, &run, Kind::Llm, 700, 100.0), Budget::Refuse);
    }

    #[test]
    fn the_same_program_never_counts_against_itself() {
        let run = [live(Kind::Llm, 900, 0.0)];
        assert_eq!(budget(1024, &run, Kind::Llm, 900, 100.0), Budget::Fits);
    }

    #[test]
    fn idle_programs_end_after_the_idle_time_and_everything_ends_for_a_game() {
        let run = [live(Kind::Llm, 600, 0.0), live(Kind::Asr, 300, 90.0)];
        assert_eq!(due(&run, 130.0, 120, false), vec![Kind::Llm]);
        assert_eq!(due(&run, 100.0, 120, false), Vec::<Kind>::new());
        assert_eq!(due(&run, 100.0, 120, true), vec![Kind::Llm, Kind::Asr]);
    }

    #[test]
    fn a_game_pauses_unless_the_user_allows_it() {
        assert!(gaming("pause", "javaw.exe"));
        assert!(!gaming("pause", ""));
        assert!(!gaming("allow", "javaw.exe"));
        assert_eq!(priority(Kind::Llm, "allow", "javaw.exe"), Prio::Idle);
        assert_eq!(priority(Kind::Llm, "allow", ""), Prio::Low);
        assert_eq!(priority(Kind::Llm, "pause", ""), Prio::Low);
    }

    #[test]
    fn speech_stays_below_normal_while_a_game_runs() {
        assert_eq!(priority(Kind::Asr, "allow", "javaw.exe"), Prio::Low);
        assert_eq!(priority(Kind::Asr, "allow", ""), Prio::Low);
    }

    #[test]
    fn threads_are_a_half_of_the_cores_up_to_four() {
        assert_eq!(threads(0, 16), 4);
        assert_eq!(threads(0, 4), 2);
        assert_eq!(threads(0, 1), 1);
        assert_eq!(threads(3, 16), 3);
        assert_eq!(threads(-1, 8), 4);
    }

    #[test]
    fn the_expected_size_is_the_last_measurement_or_the_start_value() {
        assert_eq!(expect(0, 600), 600);
        assert_eq!(expect(640, 600), 640);
    }
}
