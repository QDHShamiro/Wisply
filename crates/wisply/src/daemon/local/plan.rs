use super::Kind;

pub const FLIGHT_MAX_S: f64 = 180.0;
pub const LIMIT_MIN_MB: u64 = 1024;
pub const LIMIT_MAX_MB: u64 = 3072;
const RAM_SHARE: u64 = 16;
const MAX_THREADS: usize = 4;
const LLM_BUFFER_MB: u64 = 350;
const ASR_BUFFER_MB: u64 = 300;
const MB: u64 = 1 << 20;

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
    pub busy: bool,
}

pub fn expect(seen: u64, start: u64) -> u64 {
    if seen > 0 { seen } else { start }
}

pub fn start_mb(kind: Kind, file_bytes: u64) -> u64 {
    let buffer = match kind {
        Kind::Llm => LLM_BUFFER_MB,
        Kind::Asr => ASR_BUFFER_MB,
    };
    file_bytes.div_ceil(MB) + buffer
}

pub fn limit(set: Option<u64>, total_mb: u64) -> u64 {
    match set {
        Some(mb) if mb > 0 => mb,
        _ => (total_mb / RAM_SHARE).clamp(LIMIT_MIN_MB, LIMIT_MAX_MB),
    }
}

pub fn flying(count: u32, since: f64, now: f64) -> bool {
    count > 0 && now - since < FLIGHT_MAX_S
}

pub fn budget(limit_mb: u64, running: &[Live], want: Kind, expect_mb: u64) -> Budget {
    let others: Vec<&Live> = running.iter().filter(|l| l.kind != want).collect();
    let held: u64 = others.iter().map(|l| l.mb).sum();
    if others.is_empty() || held + expect_mb <= limit_mb {
        return Budget::Fits;
    }
    if others.iter().any(|l| l.busy) {
        return Budget::Refuse;
    }
    Budget::Kill(others.iter().map(|l| l.kind).collect())
}

pub fn gaming(mode: &str, playing: &str) -> bool {
    mode != "allow" && !playing.is_empty()
}

pub fn due(running: &[Live], now: f64, idle_s: u64, paused: bool) -> Vec<Kind> {
    running.iter().filter(|l| paused || (!l.busy && now - l.used > idle_s as f64)).map(|l| l.kind).collect()
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
        Live { kind, mb, used, busy: false }
    }

    fn busy(kind: Kind, mb: u64) -> Live {
        Live { kind, mb, used: 0.0, busy: true }
    }

    #[test]
    fn two_programs_that_fit_run_side_by_side() {
        let run = [live(Kind::Asr, 300, 0.0)];
        assert_eq!(budget(1024, &run, Kind::Llm, 600), Budget::Fits);
        assert_eq!(budget(1024, &[], Kind::Llm, 2000), Budget::Fits);
    }

    #[test]
    fn the_idle_other_program_is_ended_to_make_room() {
        let run = [live(Kind::Asr, 600, 50.0)];
        assert_eq!(budget(1024, &run, Kind::Llm, 700), Budget::Kill(vec![Kind::Asr]));
    }

    #[test]
    fn the_chain_right_after_the_transcript_is_not_refused() {
        let just_used = [live(Kind::Asr, 480, 99.9)];
        assert_eq!(budget(2048, &just_used, Kind::Llm, 860), Budget::Fits);
        assert_eq!(budget(1024, &just_used, Kind::Llm, 860), Budget::Kill(vec![Kind::Asr]));
    }

    #[test]
    fn a_request_that_really_runs_refuses_the_start() {
        let run = [busy(Kind::Asr, 600)];
        assert_eq!(budget(1024, &run, Kind::Llm, 700), Budget::Refuse);
        assert_eq!(budget(4096, &run, Kind::Llm, 700), Budget::Fits);
    }

    #[test]
    fn the_same_program_never_counts_against_itself() {
        let run = [live(Kind::Llm, 900, 0.0)];
        assert_eq!(budget(1024, &run, Kind::Llm, 900), Budget::Fits);
    }

    #[test]
    fn the_limit_is_a_sixteenth_of_the_ram_between_one_and_three_gb() {
        assert_eq!(limit(None, 32 * 1024), 2048);
        assert_eq!(limit(None, 8 * 1024), LIMIT_MIN_MB);
        assert_eq!(limit(None, 128 * 1024), LIMIT_MAX_MB);
        assert_eq!(limit(None, 0), LIMIT_MIN_MB);
        assert_eq!(limit(Some(0), 32 * 1024), 2048);
    }

    #[test]
    fn a_set_limit_always_wins() {
        assert_eq!(limit(Some(512), 64 * 1024), 512);
        assert_eq!(limit(Some(9000), 8 * 1024), 9000);
    }

    #[test]
    fn both_models_fit_on_a_32_gb_machine() {
        let asr = start_mb(Kind::Asr, 190_085_487);
        let llm = start_mb(Kind::Llm, 532_517_120);
        assert!(asr > 190 && llm > 532);
        assert!(asr + llm <= limit(None, 32 * 1024));
        assert!(asr + llm > LIMIT_MIN_MB);
    }

    #[test]
    fn a_request_counts_only_while_it_runs_and_not_forever() {
        assert!(flying(1, 100.0, 120.0));
        assert!(!flying(0, 100.0, 120.0));
        assert!(!flying(1, 100.0, 100.0 + FLIGHT_MAX_S));
    }

    #[test]
    fn idle_programs_end_after_the_idle_time_and_everything_ends_for_a_game() {
        let run = [live(Kind::Llm, 600, 0.0), live(Kind::Asr, 300, 90.0)];
        assert_eq!(due(&run, 130.0, 120, false), vec![Kind::Llm]);
        assert_eq!(due(&run, 100.0, 120, false), Vec::<Kind>::new());
        assert_eq!(due(&run, 100.0, 120, true), vec![Kind::Llm, Kind::Asr]);
    }

    #[test]
    fn a_program_with_a_running_request_is_not_ended_for_idleness() {
        let run = [busy(Kind::Llm, 600)];
        assert_eq!(due(&run, 500.0, 120, false), Vec::<Kind>::new());
        assert_eq!(due(&run, 500.0, 120, true), vec![Kind::Llm]);
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
