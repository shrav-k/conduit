//! IFC-054: keep a login's report and its first end, independently of records.
use crate::{
    Context, State, Transition, access, inventory, limits, login_resource, requests, usage,
};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;

/// IFC-054's ends. The first end stands, even when another cause follows it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum End {
    /// The keep time plus the report's validity has passed.
    ValidForEnded,
    /// Its one call was admitted; the ask's order will set this end.
    SingleUse,
    /// A later call kept a limits member for this login.
    NewerLimits,
    /// A control sent a value at the levers place succeeded. This creates no hold.
    LeversChanged,
}

/// The login's newest report, replaced only by a newer report. `keptAt` is its
/// end record's time, never `observedAt`; removing the record does not remove this.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Kept {
    report: Value,
    invocation: String,
    #[serde(rename = "keptAt")]
    kept_at: String,
    ended: Option<End>,
}

impl Kept {
    fn valid_until(&self) -> Option<String> {
        if self.report["singleUse"] == true {
            return None;
        }
        // OPS-007 already validated this bounded integer, including JSON's 3600.0.
        let seconds = self.report["validForSeconds"].as_f64()? as i64;
        let (kept, _) = limits::instant(&self.kept_at)?;
        let until = kept
            .checked_add_signed(Duration::seconds(seconds))?
            .with_timezone(&Utc);
        // Adding whole seconds preserves every fractional digit, including those
        // beyond Chrono's nanoseconds. Write the resulting absolute time in UTC.
        let fraction = self
            .kept_at
            .split_once('.')
            .and_then(|(_, suffix)| suffix.split(|ch: char| !ch.is_ascii_digit()).next())
            .unwrap_or_default();
        let fraction = if fraction.is_empty() {
            String::new()
        } else {
            format!(".{fraction}")
        };
        Some(format!("{}{fraction}Z", until.format("%Y-%m-%dT%H:%M:%S")))
    }

    fn end_at(&self, time: &str) -> Option<End> {
        self.ended.or_else(|| {
            self.valid_until()
                .as_deref()
                .and_then(limits::instant)
                .zip(limits::instant(time))
                .filter(|(until, now)| until <= now)
                .map(|_| End::ValidForEnded)
        })
    }
}

pub(crate) struct Report {
    report: Value,
    login: String,
}

impl Report {
    pub(crate) fn take(
        completion: &requests::Completion,
        invocation: &Value,
        c: &Context,
    ) -> Option<Self> {
        if invocation["operation"] != c.qualified("admission-hook.v1")
            || !invocation["conduit"].is_null()
            || invocation["discovery"] == "pending"
        {
            return None;
        }
        let target = &invocation["held_target"];
        if !login_type(target, c).is_some_and(|ty| ty["spec"].get("admission").is_some()) {
            return None;
        }
        let requests::Completion::Result { result } = completion else {
            return None;
        };
        Some(Self {
            report: result["structuredContent"].get("admission")?.clone(),
            login: target["id"].as_str()?.into(),
        })
    }

    pub(crate) fn record_end(self, t: &mut Transition, first_record: usize, invocation: &str) {
        let Some((ended, stored)) = usage::end_records(
            &mut t.records[first_record..],
            &mut t.state.records,
            invocation,
        ) else {
            return;
        };
        // OPS-007's output validation, not the implementation's claimed outcome,
        // decides whether this is a successful report.
        if ended["data"]["outcome"] != "succeeded" {
            return;
        }
        let Some(kept_at) = ended["time"].as_str() else {
            return;
        };
        t.state.admission_reports.insert(
            self.login,
            Kept {
                report: self.report.clone(),
                invocation: invocation.into(),
                kept_at: kept_at.into(),
                ended: None,
            },
        );
        ended["data"]["admission"] = self.report;
        stored.clone_from(ended);
    }
}

fn login_type(instance: &Value, c: &Context) -> Option<Value> {
    if instance["kind"] != "Resource" || instance.get("spec").is_some() {
        return None;
    }
    inventory::instance_type(instance, c)
        .ok()
        .filter(|ty| ty["spec"].get("signIn").is_some())
}

/// An early end never replaces the first end, including an elapsed validity.
pub(crate) fn end(
    reports: &mut BTreeMap<String, Kept>,
    login: &str,
    time: &str,
    cause: End,
    except_invocation: Option<&str>,
) {
    let Some(kept) = reports.get_mut(login) else {
        return;
    };
    if except_invocation == Some(kept.invocation.as_str()) {
        return;
    }
    kept.ended = kept.end_at(time).or(Some(cause));
}

pub(crate) fn control_succeeded(state: &mut State, invocation: &Value, time: &str, c: &Context) {
    if invocation["state"] != "succeeded" || invocation["reconciled_control"] == true {
        return;
    }
    let target = &invocation["held_target"];
    let Some(ty) = login_type(target, c) else {
        return;
    };
    let Some(place) = ty["spec"]["admission"]["levers"].as_str() else {
        return;
    };
    let Some(id) = target["id"].as_str() else {
        return;
    };
    // REV-007 stores nothing after retirement, even when an in-flight control succeeds.
    if access::object_by_id(state, &target["id"]).is_none_or(|o| o["state"] == "retired") {
        return;
    }
    let Some(changes) = invocation["contract"]["spec"]["changes"].as_object() else {
        return;
    };
    let supplied = invocation["control_inputs"].as_array();
    let changed = changes.iter().any(|(input, pointer)| {
        supplied.is_some_and(|inputs| inputs.iter().any(|name| name == input))
            && pointer.as_str().is_some_and(|pointer| {
                place.is_empty()
                    || pointer == place
                    || pointer
                        .strip_prefix(place)
                        .is_some_and(|tail| tail.starts_with('/'))
            })
    });
    if changed {
        // This build stores no control settings. Its successful supplied value is
        // the boundary for ending this report; no hold or ask is produced here.
        end(
            &mut state.admission_reports,
            id,
            time,
            End::LeversChanged,
            None,
        );
    }
}

pub(crate) fn status(
    state: &State,
    instance: &Value,
    actor: &Value,
    time: &str,
    c: &Context,
) -> Option<Value> {
    if instance.get("spec").is_some() {
        return None;
    }
    let login = match instance["kind"].as_str()? {
        "Resource" => instance,
        "Agent" => login_resource::resource(state, instance)?,
        _ => return None,
    };
    let ty = login_type(login, c)?;
    if ty["spec"].get("admission").is_none() {
        return Some(json!({"checked":false}));
    }
    let Some(kept) = state.admission_reports.get(login["id"].as_str()?) else {
        return Some(
            json!({"checked":true,"report":null,"invocation":null,"keptAt":null,
            "validUntil":null,"state":"none","ended":null,"asking":null,"noReport":null}),
        );
    };
    let ended = kept.end_at(time);
    let invocation = if instance["kind"] != "Agent"
        || access::may_inspect(state, actor, login, c).unwrap_or(false)
    {
        json!(kept.invocation)
    } else {
        Value::Null
    };
    // OBS-006: derive expiry without mutating the report or starting work.
    // The report remains whole for an Agent-only inspector, including its lever.
    Some(
        json!({"checked":true,"report":kept.report,"invocation":invocation,
        "keptAt":kept.kept_at,"validUntil":kept.valid_until(),
        "state":if ended.is_some() {"ended"} else {"current"},
        "ended":ended,"asking":null,"noReport":null}),
    )
}
