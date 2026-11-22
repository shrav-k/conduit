use serde_json::{Value, json};
use std::time::Duration;
use tokio::time::Instant;

use super::{Config, client_command, client_version::probe};

pub(super) const MEMBER: &str = "io.github.shrav-k.conduit/usage";

#[derive(Clone, Copy)]
pub(super) struct Totals {
    input: u64,
    output: u64,
    cached: Option<u64>,
    reasoning: Option<u64>,
}

impl Totals {
    pub(super) fn reported(usage: &Value) -> Option<Self> {
        let totals = Self {
            input: usage["input_tokens"].as_u64()?,
            output: usage["output_tokens"].as_u64()?,
            cached: usage["cached_input_tokens"].as_u64(),
            reasoning: usage["reasoning_output_tokens"].as_u64(),
        };
        (totals.input != 0
            || totals.output != 0
            || totals.cached.unwrap_or(0) != 0
            || totals.reasoning.unwrap_or(0) != 0)
            .then_some(totals)
    }

    pub(super) fn since(self, previous: Option<Self>) -> Option<Value> {
        let input = self
            .input
            .checked_sub(previous.map_or(0, |totals| totals.input))?;
        let output = self
            .output
            .checked_sub(previous.map_or(0, |totals| totals.output))?;
        if input == 0 {
            return None;
        }
        let mut counts = json!({"input": input, "output": output});
        for (name, value, earlier) in [
            (
                "cachedInput",
                self.cached,
                previous.and_then(|totals| totals.cached),
            ),
            (
                "reasoning",
                self.reasoning,
                previous.and_then(|totals| totals.reasoning),
            ),
        ] {
            if let Some(value) = value
                && (previous.is_none() || earlier.is_some())
            {
                counts[name] = json!(value.checked_sub(earlier.unwrap_or(0))?);
            }
        }
        Some(counts)
    }
}

pub(super) async fn version(config: &Config, turn_deadline: Instant) -> Option<String> {
    let deadline = (Instant::now() + Duration::from_secs(5)).min(turn_deadline);
    let command = client_command(config, None, &["--version".into()]);
    let bytes = probe(command, deadline).await?;
    let version = std::str::from_utf8(&bytes)
        .ok()?
        .strip_prefix("codex-cli ")?
        .split_ascii_whitespace()
        .next()?;
    ((1..=64).contains(&version.len())
        && version
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b".+-".contains(&byte)))
    .then(|| version.to_owned())
}

fn windows(bucket: &Value) -> impl Iterator<Item = &Value> {
    [&bucket["primary"], &bucket["secondary"]]
        .into_iter()
        .filter(|window| window.is_object())
}

pub(super) fn limit(result: &Value) -> Value {
    let buckets = std::iter::once(&result["rateLimits"]).chain(
        result["rateLimitsByLimitId"]
            .as_object()
            .into_iter()
            .flat_map(|entries| entries.values()),
    );
    let reset = |window: &&Value| window["resetsAt"].as_i64();
    let selected = buckets
        .clone()
        .flat_map(windows)
        .filter(|window| {
            window["usedPercent"]
                .as_f64()
                .is_some_and(|percent| percent >= 100.0)
        })
        .max_by_key(reset)
        .or_else(|| {
            buckets
                .filter(|bucket| !bucket["rateLimitReachedType"].is_null())
                .filter_map(|bucket| {
                    windows(bucket)
                        .filter_map(|window| Some((window, window["usedPercent"].as_f64()?)))
                        .max_by(|(_, left), (_, right)| left.total_cmp(right))
                        .map(|(window, _)| window)
                })
                .max_by_key(reset)
        });
    let window_minutes = selected.and_then(|window| window["windowDurationMins"].as_u64());
    let resets_at = selected
        .and_then(|window| window["resetsAt"].as_i64())
        .and_then(utc);
    json!({"window":null, "windowMinutes":window_minutes, "resetsAt":resets_at})
}

fn utc(seconds: i64) -> Option<String> {
    const SECONDS_PER_DAY: i64 = 86_400;
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
        format!(
            "{year:04}-{month:02}-{date:02}T{:02}:{:02}:{:02}Z",
            time / 3_600,
            time / 60 % 60,
            time % 60
        )
    })
}

#[cfg(test)]
mod tests {
    use super::utc;

    #[test]
    fn formats_the_epoch_and_a_leap_day_in_utc() {
        for (seconds, expected) in [
            (0, "1970-01-01T00:00:00Z"),
            (-1, "1969-12-31T23:59:59Z"),
            (1_709_164_800, "2024-02-29T00:00:00Z"),
        ] {
            assert_eq!(utc(seconds).as_deref(), Some(expected));
        }
    }
}
