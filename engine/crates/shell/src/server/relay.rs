use crate::{Result, identity::random_text};
use chrono::{DateTime, Utc};
use rmcp::model::InputRequiredResult;
use serde_json::{Value, json};
use std::collections::BTreeMap;

struct Issued {
    identity: String,
    tool: String,
    invocation: String,
    attempt: u64,
    expires: DateTime<Utc>,
}

#[derive(Default)]
pub(super) struct States {
    issued: BTreeMap<String, Issued>,
    consumed: BTreeMap<String, (String, DateTime<Utc>)>,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Taken {
    Accepted { invocation: String, attempt: u64 },
    NotIssued,
    Consumed,
    Expired,
    OtherTool,
}

impl Taken {
    pub(super) fn into_result(self) -> std::result::Result<(String, u64), &'static str> {
        match self {
            Self::Accepted {
                invocation,
                attempt,
            } => Ok((invocation, attempt)),
            Self::NotIssued => Err(
                "requestState is not one this engine issued to this Interface: it was altered, issued to another Interface, or forgotten at a restart",
            ),
            Self::Consumed => Err(
                "requestState was already used: each retry consumes its state, and an answer that asks again carries a fresh one",
            ),
            Self::Expired => Err("requestState expired with its URL action's wait (DL18)"),
            Self::OtherTool => Err(
                "requestState was issued for another tool: a retry repeats the tool of the request it answers",
            ),
        }
    }
}

impl States {
    fn remove_expired(&mut self, now: DateTime<Utc>) {
        self.issued.retain(|_, entry| entry.expires > now);
        self.consumed.retain(|_, (_, expires)| *expires > now);
    }

    fn issue(
        &mut self,
        identity: &str,
        tool: &str,
        invocation: &str,
        attempt: u64,
        expires: DateTime<Utc>,
        now: DateTime<Utc>,
    ) -> Result<String> {
        self.remove_expired(now);
        let state = loop {
            let candidate = random_text(43)?;
            if !self.issued.contains_key(&candidate) && !self.consumed.contains_key(&candidate) {
                break candidate;
            }
        };
        self.issued.insert(
            state.clone(),
            Issued {
                identity: identity.into(),
                tool: tool.into(),
                invocation: invocation.into(),
                attempt,
                expires,
            },
        );
        Ok(state)
    }

    pub(super) fn take(
        &mut self,
        state: &str,
        identity: &str,
        tool: &str,
        now: DateTime<Utc>,
    ) -> Taken {
        let taken = match self.issued.get(state) {
            Some(entry) if entry.identity != identity => Taken::NotIssued,
            Some(entry) if entry.expires <= now => Taken::Expired,
            Some(entry) if entry.tool != tool => Taken::OtherTool,
            Some(_) => {
                let entry = self.issued.remove(state).expect("state was just found");
                self.consumed
                    .insert(state.into(), (entry.identity, entry.expires));
                Taken::Accepted {
                    invocation: entry.invocation,
                    attempt: entry.attempt,
                }
            }
            None if self
                .consumed
                .get(state)
                .is_some_and(|(owner, expires)| owner == identity && *expires > now) =>
            {
                Taken::Consumed
            }
            None => Taken::NotIssued,
        };
        self.remove_expired(now);
        taken
    }
}

pub(super) fn relay(
    value: &Value,
    states: &mut States,
    identity: &str,
    tool: &str,
    now: DateTime<Utc>,
) -> Result<InputRequiredResult> {
    let invalid = "the core's input request is not a URL elicitation with an https URL and an expiry; nothing was relayed";
    let request = &value["request"];
    let params = request["params"].as_object().ok_or(invalid)?;
    let url = params
        .get("url")
        .and_then(Value::as_str)
        .and_then(|url| url::Url::parse(url).ok())
        .ok_or(invalid)?;
    if request["method"] != "elicitation/create"
        || params.len() != 3
        || params.get("mode").and_then(Value::as_str) != Some("url")
        || params.get("message").and_then(Value::as_str).is_none()
        || url.scheme() != "https"
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return Err(invalid.into());
    }
    let invocation = value["invocation"].as_str().ok_or(invalid)?;
    let attempt = value["attempt"].as_u64().ok_or(invalid)?;
    let expires = DateTime::parse_from_rfc3339(value["expires"].as_str().ok_or(invalid)?)
        .map_err(|_| invalid)?;
    let state = states.issue(identity, tool, invocation, attempt, expires.to_utc(), now)?;
    let mut result = json!({
        "resultType": "input_required",
        "inputRequests": {"url": request},
        "requestState": state
    });
    if value["meta"].is_object() {
        result["_meta"] = value["meta"].clone();
    }
    serde_json::from_value(result).map_err(|_| {
        states.issued.remove(&state);
        "invalid core input request".into()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeDelta;

    const T0: &str = "2026-11-18T12:00:00Z";
    const X: &str = "2026-11-18T12:10:00Z";

    #[test]
    fn request_states_bind_interface_tool_and_single_use() {
        let t0 = T0.parse().unwrap();
        let x: DateTime<Utc> = X.parse().unwrap();
        let mut states = States::default();
        let s1 = states.issue("i1", "t1", "v1", 1, x, t0).unwrap();
        assert_eq!(s1.len(), 43);
        assert!(
            s1.bytes()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        );
        assert_eq!(states.take(&s1, "i2", "t1", t0), Taken::NotIssued);
        assert_eq!(
            states.take(&format!("{s1}x"), "i1", "t1", t0),
            Taken::NotIssued
        );
        assert_eq!(states.take(&s1, "i1", "t2", t0), Taken::OtherTool);
        assert_eq!(
            states.take(&s1, "i1", "t1", t0),
            Taken::Accepted {
                invocation: "v1".into(),
                attempt: 1
            }
        );
        assert_eq!(states.take(&s1, "i1", "t1", t0), Taken::Consumed);
        assert_eq!(states.take(&s1, "i2", "t1", t0), Taken::NotIssued);

        let s2 = states.issue("i1", "t1", "v1", 1, x, t0).unwrap();
        assert_ne!(s2, s1);
        assert_eq!(states.take(&s2, "i1", "t1", x), Taken::Expired);
        assert_eq!(states.take(&s2, "i1", "t1", t0), Taken::NotIssued);
        assert_eq!(states.take(&s1, "i1", "t1", t0), Taken::NotIssued);

        let s3 = states.issue("i1", "t1", "v2", 2, x, t0).unwrap();
        let s4 = states
            .issue("i1", "t1", "v2", 3, x + TimeDelta::minutes(10), x)
            .unwrap();
        assert_eq!(states.take(&s3, "i1", "t1", t0), Taken::NotIssued);
        assert_eq!(
            states.take(&s4, "i1", "t1", x),
            Taken::Accepted {
                invocation: "v2".into(),
                attempt: 3
            }
        );
    }

    #[test]
    fn relay_passes_only_a_url_elicitation() {
        let t0 = T0.parse().unwrap();
        let mut states = States::default();
        let value = json!({
            "invocation": "v1",
            "attempt": 1,
            "request": {
                "method": "elicitation/create",
                "params": {"mode":"url","message":"Sign in","url":"https://example.org/login"}
            },
            "meta": {"k":"v"},
            "expires": X
        });
        let result = relay(&value, &mut states, "i1", "t1", t0).unwrap();
        let state = result.request_state.clone().unwrap();
        assert_eq!(
            serde_json::to_value(result).unwrap(),
            json!({
                "resultType": "input_required",
                "inputRequests": {"url": {
                    "method": "elicitation/create",
                    "params": {
                        "mode": "url",
                        "message": "Sign in",
                        "url": "https://example.org/login",
                        "elicitationId": ""
                    }
                }},
                "requestState": state,
                "_meta": {"k":"v"}
            })
        );
        assert_eq!(
            states.take(&state, "i1", "t1", t0),
            Taken::Accepted {
                invocation: "v1".into(),
                attempt: 1
            }
        );
        let mut no_meta = value.clone();
        no_meta["meta"] = Value::Null;
        let result = relay(&no_meta, &mut states, "i1", "t1", t0).unwrap();
        assert!(serde_json::to_value(result).unwrap().get("_meta").is_none());

        let mut url = url::Url::parse("https://example.org/login").unwrap();
        url.set_username("user").unwrap();
        let mut extra_params = value["request"]["params"].clone();
        extra_params["requestedSchema"] = json!({});
        let mut states = States::default();
        let prevalidation_error = "the core's input request is not a URL elicitation with an https URL and an expiry; nothing was relayed";
        for (path, replacement) in [
            ("/request/params/mode", json!("form")),
            ("/request/params/url", json!("http://example.org/login")),
            ("/request/params/url", json!(url.as_str())),
            ("/request/params", extra_params),
            ("/request/method", json!("sampling/createMessage")),
            ("/expires", json!("soon")),
        ] {
            let mut invalid = value.clone();
            *invalid.pointer_mut(path).unwrap() = replacement;
            assert_eq!(
                relay(&invalid, &mut states, "i1", "t1", t0).unwrap_err(),
                prevalidation_error,
                "{path}"
            );
            assert!(states.issued.is_empty());
        }
        let mut missing_attempt = value;
        missing_attempt.as_object_mut().unwrap().remove("attempt");
        assert_eq!(
            relay(&missing_attempt, &mut states, "i1", "t1", t0).unwrap_err(),
            prevalidation_error
        );
        assert!(states.issued.is_empty());
    }
}
