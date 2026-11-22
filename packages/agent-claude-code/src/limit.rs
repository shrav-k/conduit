use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) fn window(text: &str) -> Option<&'static str> {
    [
        ("hit your session limit", "session"),
        ("hit your weekly limit", "weekly"),
        ("hit your opus limit", "opus"),
        ("hit your sonnet limit", "sonnet"),
    ]
    .into_iter()
    .find_map(|(words, name)| text.contains(words).then_some(name))
}

pub(crate) fn reset_time(value: &Value, started: SystemTime) -> Option<String> {
    const EIGHT_DAYS: f64 = 8.0 * 86_400.0;
    let number = value.as_f64()?;
    let began = started.duration_since(UNIX_EPOCH).ok()?.as_secs_f64();
    let mut plausible = [number, number / 1000.0]
        .into_iter()
        .filter(|time| began < *time && *time <= began + EIGHT_DAYS);
    let time = plausible.next()?;
    if plausible.next().is_some() {
        return None;
    }
    let time = (time * 1000.0).round() as i64;
    utc(time)
}

fn utc(milliseconds: i64) -> Option<String> {
    const SECONDS_PER_DAY: i64 = 86_400;
    let seconds = milliseconds.div_euclid(1000);
    // Convert to a March-based civil calendar so February ends each year.
    let day = seconds.div_euclid(SECONDS_PER_DAY) + 719_468;
    let era = day.div_euclid(146_097);
    let day_of_era = day.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let march_month = (5 * day_of_year + 2) / 153;
    let date = day_of_year - (153 * march_month + 2) / 5 + 1;
    let month = if march_month < 10 {
        march_month + 3
    } else {
        march_month - 9
    };
    let year = era * 400 + year_of_era + i64::from(month <= 2);
    let time = seconds.rem_euclid(SECONDS_PER_DAY);
    (0..=9999).contains(&year).then(|| {
        let fraction = milliseconds.rem_euclid(1000);
        let fractional = if fraction == 0 {
            String::new()
        } else {
            format!(".{fraction:03}")
        };
        format!(
            "{year:04}-{month:02}-{date:02}T{:02}:{:02}:{:02}{fractional}Z",
            time / 3_600,
            time / 60 % 60,
            time % 60
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::time::Duration;

    #[test]
    fn reset_agrees_in_seconds_and_milliseconds_on_a_leap_day() {
        let started = UNIX_EPOCH + Duration::from_secs(1_709_164_800);
        for timestamp in [json!(1_709_168_400), json!(1_709_168_400_000_i64)] {
            assert_eq!(
                reset_time(&timestamp, started).as_deref(),
                Some("2024-02-29T01:00:00Z")
            );
        }
        assert_eq!(
            reset_time(&json!(1_709_168_400_123_i64), started).as_deref(),
            Some("2024-02-29T01:00:00.123Z")
        );
        assert_eq!(utc(0).as_deref(), Some("1970-01-01T00:00:00Z"));
    }
}
