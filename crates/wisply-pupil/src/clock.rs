#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Moment {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

impl Moment {
    pub fn text(&self) -> String {
        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }

    pub fn file(&self) -> String {
        format!("{:04}{:02}{:02}-{:02}{:02}{:02}", self.year, self.month, self.day, self.hour, self.minute, self.second)
    }
}

#[cfg(windows)]
pub fn local() -> Moment {
    use windows_sys::Win32::Foundation::SYSTEMTIME;
    use windows_sys::Win32::System::SystemInformation::GetLocalTime;
    let mut found: SYSTEMTIME = unsafe { std::mem::zeroed() };
    unsafe { GetLocalTime(&mut found) };
    Moment {
        year: found.wYear.into(),
        month: found.wMonth.into(),
        day: found.wDay.into(),
        hour: found.wHour.into(),
        minute: found.wMinute.into(),
        second: found.wSecond.into(),
    }
}

#[cfg(not(windows))]
pub fn local() -> Moment {
    from_epoch(crate::server::now() as i64)
}

#[cfg(any(test, not(windows)))]
pub fn from_epoch(seconds: i64) -> Moment {
    let days = seconds.div_euclid(86_400);
    let rest = seconds.rem_euclid(86_400);
    let shifted = days + 719_468;
    let era = shifted.div_euclid(146_097);
    let day_of_era = shifted.rem_euclid(146_097);
    let year_of_era = (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let spot = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * spot + 2) / 5 + 1;
    let month = if spot < 10 { spot + 3 } else { spot - 9 };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    Moment {
        year: year as u32,
        month: month as u32,
        day: day as u32,
        hour: (rest / 3600) as u32,
        minute: (rest % 3600 / 60) as u32,
        second: (rest % 60) as u32,
    }
}

#[cfg(test)]
mod tests {
    use super::{from_epoch, local};

    #[test]
    fn epoch_seconds_become_calendar_dates() {
        assert_eq!(from_epoch(0).text(), "1970-01-01 00:00:00");
        assert_eq!(from_epoch(951_782_400).text(), "2000-02-29 00:00:00");
        assert_eq!(from_epoch(1_794_781_405).file(), "20261115-222325");
    }

    #[test]
    fn the_local_clock_is_a_real_date() {
        let now = local();
        assert!(now.year >= 2026 && (1..=12).contains(&now.month) && (1..=31).contains(&now.day));
    }
}
