//! UTC times in RFC 3339, from the standard library's clock.

use std::time::{SystemTime, UNIX_EPOCH};

/// The time now, as `YYYY-MM-DDTHH:MM:SSZ`.
pub fn now() -> String {
    format_utc(SystemTime::now())
}

/// A time as `YYYY-MM-DDTHH:MM:SSZ`, in UTC; a time before 1970 is written as 1970.
pub fn format_utc(time: SystemTime) -> String {
    let seconds = time
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs())
        .unwrap_or(0);
    let seconds = i64::try_from(seconds).unwrap_or(i64::MAX / 2);
    let days = seconds.div_euclid(86_400);
    let rest = seconds.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        rest / 3600,
        rest % 3600 / 60,
        rest % 60
    )
}

/// The proleptic Gregorian date of a count of days since 1970-01-01.
fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let day_of_era = z.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted_month = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * shifted_month + 2) / 5 + 1;
    let month = if shifted_month < 10 {
        shifted_month + 3
    } else {
        shifted_month - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    (year, month, day)
}

#[cfg(test)]
mod tests {
    use super::format_utc;
    use std::time::{Duration, UNIX_EPOCH};

    #[test]
    fn formats_known_times() {
        assert_eq!(format_utc(UNIX_EPOCH), "1970-01-01T00:00:00Z");
        let leap = UNIX_EPOCH + Duration::from_secs(951_782_400);
        assert_eq!(format_utc(leap), "2000-02-29T00:00:00Z");
        let late = UNIX_EPOCH + Duration::from_secs(1_795_000_000);
        assert_eq!(format_utc(late), "2026-11-18T11:06:40Z");
    }
}
