const HOUR: i64 = 3600;
const DAY: i64 = 86_400;
const WEEK: i64 = 7;
const THURSDAY: i64 = 4;
const STANDARD: i64 = -8 * HOUR;
const SUMMER: i64 = -7 * HOUR;
const SUMMER_FROM: (i64, i64, i64) = (3, 2, 10);
const SUMMER_UNTIL: (i64, i64, i64) = (11, 1, 9);

pub(crate) fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let (era, of_era) = (year.div_euclid(400), year.rem_euclid(400));
    let of_year = (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + day - 1;
    era * 146_097 + of_era * 365 + of_era / 4 - of_era / 100 + of_year - 719_468
}

fn year_of(days: i64) -> i64 {
    let shifted = days + 719_468;
    let (era, of_era) = (shifted.div_euclid(146_097), shifted.rem_euclid(146_097));
    let year = (of_era - of_era / 1460 + of_era / 36_524 - of_era / 146_096) / 365;
    let of_year = of_era - (365 * year + year / 4 - year / 100);
    year + era * 400 + i64::from((5 * of_year + 2) / 153 >= 10)
}

fn turn(year: i64, (month, nth, hour): (i64, i64, i64)) -> i64 {
    let first = days_from_civil(year, month, 1);
    let to_sunday = (WEEK - (first + THURSDAY).rem_euclid(WEEK)) % WEEK;
    (first + to_sunday + (nth - 1) * WEEK) * DAY + hour * HOUR
}

fn offset(at: i64) -> i64 {
    let year = year_of(at.div_euclid(DAY));
    if (turn(year, SUMMER_FROM)..turn(year, SUMMER_UNTIL)).contains(&at) { SUMMER } else { STANDARD }
}

fn midnight(day: i64) -> i64 {
    let summer = day * DAY - SUMMER;
    if offset(summer) == SUMMER { summer } else { day * DAY - STANDARD }
}

fn local_day(at: f64) -> i64 {
    let whole = at.floor() as i64;
    (whole + offset(whole)).div_euclid(DAY)
}

pub fn day_start(at: f64) -> f64 {
    midnight(local_day(at)) as f64
}

pub fn next_day_start(at: f64) -> f64 {
    midnight(local_day(at) + 1) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn utc(year: i64, month: i64, day: i64, hour: i64) -> f64 {
        (days_from_civil(year, month, day) * DAY + hour * HOUR) as f64
    }

    #[test]
    fn the_day_starts_at_pacific_midnight_in_winter_and_summer() {
        assert_eq!(day_start(utc(2026, 11, 21, 20)), utc(2026, 11, 21, 8));
        assert_eq!(next_day_start(utc(2026, 11, 21, 20)), utc(2026, 11, 22, 8));
        assert_eq!(day_start(utc(2026, 7, 1, 12)), utc(2026, 7, 1, 7));
        assert_eq!(next_day_start(utc(2026, 7, 1, 12)), utc(2026, 7, 2, 7));
        assert_eq!(day_start(utc(2026, 7, 1, 6)), utc(2026, 6, 30, 7));
    }

    #[test]
    fn the_day_before_new_year_still_belongs_to_the_old_year() {
        assert_eq!(day_start(utc(2026, 1, 1, 3)), utc(2025, 12, 31, 8));
        assert_eq!(next_day_start(utc(2026, 1, 1, 3)), utc(2026, 1, 1, 8));
    }

    #[test]
    fn the_days_of_the_clock_change_are_23_and_25_hours_long() {
        let spring = utc(2026, 3, 8, 12);
        assert_eq!(day_start(spring), utc(2026, 3, 8, 8));
        assert_eq!(next_day_start(spring), utc(2026, 3, 9, 7));
        let autumn = utc(2026, 11, 1, 12);
        assert_eq!(day_start(autumn), utc(2026, 11, 1, 7));
        assert_eq!(next_day_start(autumn), utc(2026, 11, 2, 8));
        assert_eq!(day_start(utc(2024, 3, 10, 9)), utc(2024, 3, 10, 8));
        assert_eq!(day_start(utc(2024, 11, 3, 8)), utc(2024, 11, 3, 7));
    }

    #[test]
    fn a_day_start_is_its_own_day_start() {
        for at in [utc(2026, 3, 8, 8), utc(2026, 3, 9, 7), utc(2026, 11, 1, 7), utc(2026, 11, 2, 8)] {
            assert_eq!(day_start(at), at);
            assert_eq!(day_start(at - 1.0), day_start(at - 3600.0));
            assert!(next_day_start(at) > at);
        }
    }
}
