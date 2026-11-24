//! IFC-052 reports on invocation end records and IFC-053 observations for inspection.
use crate::{
    Context, State, Transition, inventory, login_resource, requests::Completion, schemas, usage,
};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Observations {
    newest: Value,
    windows: BTreeMap<String, Value>,
    /// IFC-053 signals in limit-status.v1 form: usageAllowed beside paidUsage's
    /// possible and inUse. Each holds value, observedAt, source, instance and
    /// invocation; paid signals also hold resetsAt. Empty for an entry an earlier
    /// engine wrote, and then not serialized, preserving its stored form (DL2).
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    signals: BTreeMap<String, Value>,
}

pub(crate) struct Report {
    value: Value,
    instance: Option<Value>,
    login: Option<String>,
}

impl Report {
    pub(crate) fn take(completion: &mut Completion, target: &Value, c: &Context) -> Option<Self> {
        let value = usage::take_member(completion, c, "limits")?;
        // Use the instance and login place held when this call was dispatched.
        let instance = eligible(target, c).then(|| {
            let mut instance =
                json!({"kind":target["kind"],"name":target["name"],"id":target["id"]});
            if let Some(nested) = target.get("nested") {
                instance["nested"] = json!({"instance":nested["parent"],"name":nested["name"]});
            }
            instance
        });
        let login = login_resource::id(target)
            .and_then(Value::as_str)
            .map(str::to_owned);
        Some(Self {
            value,
            instance,
            login,
        })
    }

    pub(crate) fn record_end(
        self,
        t: &mut Transition,
        c: &Context,
        first_record: usize,
        invocation: &str,
    ) {
        let Some((ended, stored)) = usage::end_records(
            &mut t.records[first_record..],
            &mut t.state.records,
            invocation,
        ) else {
            return;
        };
        if let Some(warning) = warning(c, &self.value) {
            ended["data"]["limitsDropped"] = warning;
        } else {
            if let Some(instance) = self.instance.as_ref()
                && let Some(id) = instance["id"].as_str()
            {
                let evidence = json!({
                    "observedAt":self.value["observedAt"],"source":ended["source"],
                    "instance":instance,"invocation":invocation,
                });
                let mut newest = evidence.clone();
                newest["usageAllowed"] = self.value["usageAllowed"].clone();
                if let Some(reason) = self.value.get("noWindows") {
                    newest["noWindows"] = reason.clone();
                }
                for key in ["paidUsage", "windowsOmitted"] {
                    if let Some(value) = self.value.get(key) {
                        newest[key] = value.clone();
                    }
                }
                let windows = self.value["windows"].as_array().expect("validated windows");
                for id in std::iter::once(id).chain(self.login.as_deref()) {
                    let observations = t.state.limit_observations.entry(id.to_owned()).or_default();
                    keep_latest(&mut observations.newest, newest.clone());
                    // IFC-053 / OBS-002: signals advance independently, only on non-null values.
                    for (key, value, paid) in [
                        ("usageAllowed", &self.value["usageAllowed"], false),
                        ("possible", &self.value["paidUsage"]["possible"], true),
                        ("inUse", &self.value["paidUsage"]["inUse"], true),
                    ] {
                        if value.is_null() {
                            continue;
                        }
                        let mut signal = evidence.clone();
                        signal["value"] = value.clone();
                        let current = if paid {
                            signal["resetsAt"] = self.value["paidUsage"]["resetsAt"].clone();
                            let paid_usage = observations
                                .signals
                                .entry("paidUsage".into())
                                .or_insert_with(|| json!({}));
                            &mut paid_usage[key]
                        } else {
                            observations.signals.entry(key.into()).or_default()
                        };
                        keep_latest(current, signal);
                    }
                    for reported in windows {
                        let key = reported["id"].as_str().expect("validated window ID");
                        let mut window = reported.as_object().expect("validated window").clone();
                        window.extend(evidence.as_object().expect("observation evidence").clone());
                        keep_latest(
                            observations.windows.entry(key.to_owned()).or_default(),
                            Value::Object(window),
                        );
                    }
                }
            }
            ended["data"]["limits"] = self.value;
        }
        stored.clone_from(ended);
    }
}

fn warning(c: &Context, value: &Value) -> Option<Value> {
    let mut pointer = format!(
        "/_meta/{}",
        schemas::token(&format!("{}limits", c.meta_prefix))
    );
    let mut warning = json!({"ruleId":"IFC-052","level":"warning"});
    let detail = match c.core_environment.first_core_failure(
        &c.core_package,
        "limit-report.v1",
        value,
    ) {
        Ok(None) => {
            let mut seen = BTreeMap::new();
            let (first, second) = value["windows"]
                .as_array()
                .expect("validated windows")
                .iter()
                .enumerate()
                .find_map(|(index, window)| {
                    let id = window["id"].as_str().expect("validated window ID");
                    seen.insert(id, index).map(|previous| (previous, index))
                })?;
            pointer.push_str("/windows");
            format!("at {pointer} the windows at indices {first} and {second} have the same id")
        }
        Ok(Some((at, keyword))) => {
            pointer.push_str(&at);
            warning["properties"] = json!({"keywordLocation":keyword});
            let last = keyword
                .rsplit_once('/')
                .map_or(keyword.as_str(), |(_, last)| last);
            if last.is_empty() {
                format!("at {pointer} it does not conform to limit-report.v1")
            } else {
                format!(
                    "at {pointer} it does not conform to limit-report.v1, failing its {last} keyword"
                )
            }
        }
        Err(_) => "the engine could not check it against limit-report.v1".into(),
    };
    warning["locations"] = json!([{"logicalLocations":[{"fullyQualifiedName":pointer}]}]);
    warning["message"] = json!({"text":format!(
        "The result's limits member was dropped: {detail}. The record of this invocation's end keeps no part of it, and nothing else about the call changes (IFC-052)."
    )});
    Some(warning)
}

fn eligible(instance: &Value, c: &Context) -> bool {
    instance.get("spec").is_none()
        && match instance["kind"].as_str() {
            Some("Agent") => true,
            Some("Resource") => inventory::instance_type(instance, c)
                .is_ok_and(|ty| ty["spec"].get("signIn").is_some()),
            _ => false,
        }
}

fn instant(value: &str) -> Option<(DateTime<FixedOffset>, &str)> {
    let normalized = DateTime::parse_from_rfc3339(value).ok()?;
    let fraction = value
        .split_once('.')
        .and_then(|(_, suffix)| suffix.split(|ch: char| !ch.is_ascii_digit()).next())
        .unwrap_or_default();
    // Chrono orders the instant through nanoseconds. For equal nanoseconds, the
    // remaining digits sort as if zero-padded once trailing zeros are removed.
    let remainder = fraction.get(9..).unwrap_or_default().trim_end_matches('0');
    Some((normalized, remainder))
}

fn keep_latest(current: &mut Value, candidate: Value) {
    // A candidate seen here is from the end record being kept now, so it wins ties.
    if current["observedAt"].as_str().and_then(instant)
        <= candidate["observedAt"].as_str().and_then(instant)
    {
        *current = candidate;
    }
}

pub(crate) fn status(state: &State, instance: &Value, time: &str, c: &Context) -> Option<Value> {
    if !eligible(instance, c) {
        return None;
    }
    let id = instance["id"].as_str()?;
    let Some(observations) = state.limit_observations.get(id) else {
        return Some(json!({"state":"unknown","windows":[]}));
    };
    let now = instant(time);
    let windows: Vec<_> = observations
        .windows
        .values()
        .cloned()
        .map(|mut window| {
            if window["resetsAt"]
                .as_str()
                .and_then(instant)
                .zip(now)
                .is_some_and(|(reset, now)| reset <= now)
            {
                window["state"] = json!("unknown");
            }
            window
        })
        .collect();
    let mut status = json!({"state":"observed","newest":observations.newest,"windows":windows});
    if !observations.signals.is_empty() {
        status["signals"] = json!(observations.signals);
    }
    Some(status)
}
