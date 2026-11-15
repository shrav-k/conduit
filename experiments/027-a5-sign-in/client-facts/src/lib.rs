//! Redacted facts only. Raw client output never enters a report or diagnostic.
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use url::{Host, Url};

pub mod run;

fn letters(s: &str) -> String {
    s.chars()
        .filter(char::is_ascii_alphabetic)
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

pub fn without_ansi(s: &str) -> String {
    let mut chars = s.chars().peekable();
    let mut out = String::new();
    while let Some(c) = chars.next() {
        if c != '\u{1b}' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('[') => {
                for c in chars.by_ref() {
                    if ('@'..='~').contains(&c) {
                        break;
                    }
                }
            }
            Some(']') => {
                while let Some(c) = chars.next() {
                    if c == '\u{7}' || (c == '\u{1b}' && chars.next_if_eq(&'\\').is_some()) {
                        break;
                    }
                }
            }
            _ => {}
        }
    }
    out
}

fn email(s: &str) -> bool {
    let mut parts = s.split('@');
    matches!((parts.next(), parts.next(), parts.next()), (Some(a), Some(b), None) if !a.is_empty() && b.contains('.'))
}

fn safe_key(s: &str) -> bool {
    let mut chars = s.chars();
    s.len() <= 32
        && matches!(chars.next(), Some(c) if c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn json_status(
    v: &Value,
    path: &mut Vec<String>,
    safe: bool,
    flags: &mut (bool, bool),
    account: &mut Option<String>,
) {
    match v {
        Value::Object(map) => {
            for (key, value) in map {
                let label = letters(key);
                if matches!(label.as_str(), "loggedin" | "signedin")
                    && let Some(b) = value.as_bool()
                {
                    if b {
                        flags.1 = true;
                    } else {
                        flags.0 = true;
                    }
                }
                path.push(key.clone());
                let safe = safe && safe_key(key);
                if account.is_none()
                    && let Some(s) = value.as_str()
                    && (email(s)
                        || (!s.is_empty()
                            && matches!(
                                label.as_str(),
                                "email"
                                    | "accountemail"
                                    | "useremail"
                                    | "accountid"
                                    | "accountuuid"
                                    | "userid"
                            )))
                {
                    *account = Some(if safe {
                        path.join(".")
                    } else {
                        "unnamed".into()
                    });
                }
                json_status(value, path, safe, flags, account);
                path.pop();
            }
        }
        Value::Array(values) => {
            for (i, value) in values.iter().enumerate() {
                path.push(i.to_string());
                json_status(value, path, safe, flags, account);
                path.pop();
            }
        }
        Value::String(s) if account.is_none() && email(s) => {
            *account = Some(if safe {
                path.join(".")
            } else {
                "unnamed".into()
            });
        }
        _ => {}
    }
}

pub fn status(stdout: &str, stderr: &str) -> Value {
    let mut flags = (false, false);
    let mut account = None;
    if let Ok(Value::Object(map)) = serde_json::from_str::<Value>(stdout.trim()) {
        json_status(
            &Value::Object(map),
            &mut vec![],
            true,
            &mut flags,
            &mut account,
        );
    } else {
        let text = without_ansi(&format!("{stdout}\n{stderr}"));
        let lower = text.to_ascii_lowercase();
        flags.0 = ["not logged in", "not signed in", "logged out"]
            .iter()
            .any(|s| lower.contains(s));
        flags.1 = ["logged in", "signed in"].iter().any(|s| lower.contains(s));
        for line in text.lines() {
            if line.split_whitespace().any(email) {
                let label = line.split_once(':').map(|(label, _)| letters(label));
                account = Some(match label {
                    Some(label)
                        if matches!(
                            label.as_str(),
                            "email"
                                | "emailaddress"
                                | "account"
                                | "accountemail"
                                | "user"
                                | "useremail"
                                | "login"
                                | "loggedinas"
                                | "signedinas"
                        ) =>
                    {
                        label
                    }
                    _ => "unnamed".into(),
                });
                break;
            }
        }
    }
    json!({"login": if flags.0 { "no" } else if flags.1 { "yes" } else { "unclear" },
        "account_named": if account.is_some() { "yes" } else { "no" }, "account_field": account})
}

pub fn version(stdout: &str, stderr: &str) -> String {
    let text = without_ansi(&format!("{stdout}\n{stderr}"));
    text.split_whitespace()
        .find(|token| {
            let numbers: Vec<_> = token.splitn(3, '.').collect();
            if numbers.len() != 3
                || numbers[..2]
                    .iter()
                    .any(|n| n.is_empty() || !n.bytes().all(|c| c.is_ascii_digit()))
            {
                return false;
            }
            let third_end = numbers[2]
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(numbers[2].len());
            let (third, suffix) = numbers[2].split_at(third_end);
            !third.is_empty()
                && (suffix.is_empty()
                    || (suffix.len() > 1
                        && matches!(suffix.as_bytes()[0], b'-' | b'+' | b'.')
                        && suffix[1..]
                            .bytes()
                            .all(|c| c.is_ascii_alphanumeric() || c == b'.' || c == b'-')))
        })
        .unwrap_or("unparsed")
        .into()
}

/// Only ephemeral keys hold the URLs. Callers discard these after producing redacted facts.
pub fn authorization_urls(printed: &str, shim: &str) -> BTreeMap<String, &'static str> {
    let mut found = BTreeMap::new();
    for (text, source) in [(printed, "printed"), (shim, "shim")] {
        let clean = without_ansi(text);
        let mut rest = clean.as_str();
        while let Some(start) = rest.find("https://") {
            rest = &rest[start..];
            let end = rest
                .find(|c: char| c.is_whitespace() || "\"'<>)]".contains(c))
                .unwrap_or(rest.len());
            let raw = &rest[..end];
            if let Ok(url) = Url::parse(raw)
                && url.query_pairs().any(|(k, _)| {
                    matches!(k.as_ref(), "client_id" | "response_type" | "redirect_uri")
                })
            {
                found
                    .entry(raw.into())
                    .and_modify(|old| {
                        if *old != source {
                            *old = "both";
                        }
                    })
                    .or_insert(source);
            }
            rest = &rest[end..];
        }
    }
    found
}

fn explicit_port(raw: &str) -> bool {
    let authority = raw
        .split_once("://")
        .map(|(_, tail)| tail.split(['/', '?', '#']).next().unwrap_or(""))
        .unwrap_or("");
    authority
        .rsplit_once(':')
        .is_some_and(|(_, port)| !port.is_empty() && port.bytes().all(|c| c.is_ascii_digit()))
}

/// Returns redacted facts plus the loopback-only target to test in memory.
pub fn url_facts(raw: &str, source: &str) -> (Value, Option<(String, u16)>) {
    let parsed = Url::parse(raw).expect("caller supplies a parsed URL");
    let pairs: Vec<_> = parsed.query_pairs().collect();
    let parameter = |name: &str| {
        pairs
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.as_ref())
            .unwrap_or("")
    };
    let redirect_raw = parameter("redirect_uri");
    let redirect = Url::parse(redirect_raw).ok();
    let host = redirect.as_ref().and_then(|u| match u.host() {
        Some(Host::Domain("localhost")) => Some("localhost"),
        Some(Host::Ipv4(ip)) if ip == std::net::Ipv4Addr::LOCALHOST => Some("127.0.0.1"),
        Some(Host::Ipv6(ip)) if ip == std::net::Ipv6Addr::LOCALHOST => Some("[::1]"),
        _ => None,
    });
    let return_kind = match redirect.as_ref().map(Url::scheme) {
        Some("http") if host.is_some() => "redirect",
        Some("https") => "page",
        _ => "none",
    };
    let unreserved = |c: u8| c.is_ascii_alphanumeric() || b"-._~".contains(&c);
    let challenge = parameter("code_challenge");
    let state = parameter("state");
    let authority = raw
        .split_once("://")
        .map(|(_, s)| s.split(['/', '?', '#']).next().unwrap_or(""))
        .unwrap_or("");
    let checks = [
        ("https", parsed.scheme() == "https"),
        ("no_userinfo", !authority.contains('@')),
        ("no_fragment", parsed.fragment().is_none()),
        ("no_repeated_parameter", {
            let mut keys = BTreeSet::new();
            pairs.iter().all(|(k, _)| keys.insert(k))
        }),
        ("client_id", !parameter("client_id").is_empty()),
        ("response_type_code", parameter("response_type") == "code"),
        ("s256", parameter("code_challenge_method") == "S256"),
        (
            "challenge_43",
            challenge.len() == 43
                && challenge
                    .bytes()
                    .all(|c| c.is_ascii_alphanumeric() || b"-_".contains(&c)),
        ),
        (
            "state_ok",
            state.len() >= 22 && state.bytes().all(unreserved),
        ),
    ];
    let mut fact = json!({"source": source, "return": return_kind,
        "redirect_host": if return_kind == "redirect" { host } else { None },
        "state_length": state.chars().count(), "listens": "not checked"});
    let mut failures = Vec::new();
    for (name, passes) in checks {
        fact[name] = json!(passes);
        if !passes {
            failures.push(name);
        }
    }
    let mut target = None;
    if return_kind == "redirect" {
        let redirect = redirect.as_ref().unwrap();
        let port = explicit_port(redirect_raw)
            .then(|| redirect.port_or_known_default())
            .flatten();
        for (name, passes) in [
            ("redirect_port", port.is_some()),
            ("redirect_no_query", redirect.query().is_none()),
        ] {
            fact[name] = json!(passes);
            if !passes {
                failures.push(name);
            }
        }
        if let Some(port) = port {
            target = Some((host.unwrap().into(), port));
        }
    } else if return_kind == "none" {
        failures.push("return");
    }
    fact["ifc_049"] = json!(failures);
    (fact, target)
}

#[cfg(test)]
mod parser_tests;
