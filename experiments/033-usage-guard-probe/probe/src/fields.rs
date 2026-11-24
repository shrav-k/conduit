use crate::Output;
use serde_json::Value;
use std::collections::BTreeSet;

pub const CODEX_NAMES: &str = "ordinaryUsageAllowed rateLimits rateLimitsByLimitId rateLimitResetCredits accountId rateLimitUpsell limitId limitName normalModelSlug primary secondary usedPercent windowDurationMins resetsAt credits hasCredits unlimited balance individualLimit limit used remainingPercent spendControlReached planType rateLimitReachedType";
pub const CLAUDE_NAMES: &str = "message usage_report rate_limits limits kind group percent resets_at scope model surface severity is_active extra_usage is_enabled monthly_limit used_credits utilization currency rate_limit_info status resetsAt errorCode canUserPurchaseCredits hasChargeableSavedPaymentMethod overageEnabled rateLimitType overageStatus overageResetsAt overageDisabledReason isUsingOverage unifiedWindows five_hour seven_day";

// Keep object order without enabling serde_json's extra preserve_order dependency.
// serde_json validates the entire document first; this small walk only retains its ordering.
#[derive(Clone, Debug)]
pub enum Node {
    Object(Vec<(String, Node)>),
    Array(Vec<Node>),
    Leaf(Value),
}
impl Node {
    pub fn parse(text: &str) -> Option<Self> {
        serde_json::from_str::<Value>(text).ok()?;
        fn string_end(b: &[u8], mut i: usize) -> usize {
            i += 1;
            while b[i] != b'"' {
                if b[i] == b'\\' {
                    i += 1;
                }
                i += 1;
            }
            i + 1
        }
        fn ws(b: &[u8], i: &mut usize) {
            while *i < b.len() && b[*i].is_ascii_whitespace() {
                *i += 1;
            }
        }
        fn parse(t: &str, i: &mut usize) -> Node {
            let b = t.as_bytes();
            ws(b, i);
            match b[*i] {
                b'{' => {
                    *i += 1;
                    ws(b, i);
                    let mut entries = Vec::new();
                    while b[*i] != b'}' {
                        let end = string_end(b, *i);
                        let key = serde_json::from_str(&t[*i..end]).unwrap();
                        *i = end;
                        ws(b, i);
                        *i += 1;
                        let value = parse(t, i);
                        entries.push((key, value));
                        ws(b, i);
                        if b[*i] == b',' {
                            *i += 1;
                            ws(b, i);
                        } else {
                            break;
                        }
                    }
                    *i += 1;
                    Node::Object(entries)
                }
                b'[' => {
                    *i += 1;
                    ws(b, i);
                    let mut entries = Vec::new();
                    while b[*i] != b']' {
                        entries.push(parse(t, i));
                        ws(b, i);
                        if b[*i] == b',' {
                            *i += 1;
                        } else {
                            break;
                        }
                    }
                    *i += 1;
                    Node::Array(entries)
                }
                _ => {
                    let start = *i;
                    if b[*i] == b'"' {
                        *i = string_end(b, *i);
                    } else {
                        while *i < b.len() && !b",]} \n\r\t".contains(&b[*i]) {
                            *i += 1;
                        }
                    }
                    Node::Leaf(serde_json::from_str(&t[start..*i]).unwrap())
                }
            }
        }
        Some(parse(text, &mut 0))
    }
    pub fn get(&self, key: &str) -> Option<&Self> {
        if let Self::Object(entries) = self {
            entries.iter().find(|(k, _)| k == key).map(|(_, v)| v)
        } else {
            None
        }
    }
    pub fn find<'a>(&'a self, key: &str, out: &mut Vec<&'a Self>) {
        match self {
            Self::Object(entries) => {
                for (k, v) in entries {
                    if k == key {
                        out.push(v);
                    }
                    v.find(key, out);
                }
            }
            Self::Array(entries) => {
                for v in entries {
                    v.find(key, out);
                }
            }
            _ => {}
        }
    }
}
fn time_name(name: &str) -> bool {
    matches!(
        name,
        "resetsAt" | "resets_at" | "overageResetsAt" | "windowDurationMins"
    )
}
fn time_shape(text: &str) -> bool {
    let b = text.as_bytes();
    if b.len() < 20 {
        return false;
    }
    for (i, c) in b.iter().enumerate().take(19) {
        let fixed = match i {
            4 | 7 => Some(b'-'),
            10 => Some(b'T'),
            13 | 16 => Some(b':'),
            _ => None,
        };
        if fixed.map_or(!c.is_ascii_digit(), |v| v != *c) {
            return false;
        }
    }
    let mut i = 19;
    if b[i] == b'.' {
        i += 1;
        let start = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        if i == start {
            return false;
        }
    }
    if &b[i..] == b"Z" {
        return true;
    }
    let tail = &b[i..];
    tail.len() == 6
        && matches!(tail[0], b'+' | b'-')
        && tail[3] == b':'
        && [1, 2, 4, 5].iter().all(|j| tail[*j].is_ascii_digit())
}
fn words(name: &str) -> Option<&'static str> {
    match name {
        "kind" => Some("session weekly_all weekly_scoped"),
        "severity" => Some("normal warning critical"),
        "status" => Some("allowed allowed_warning rejected"),
        "rateLimitType" => Some("five_hour seven_day seven_day_opus seven_day_sonnet overage"),
        "rateLimitReachedType" => Some(
            "rate_limit_reached workspace_owner_credits_depleted workspace_member_credits_depleted workspace_owner_usage_limit_reached workspace_member_usage_limit_reached",
        ),
        _ => None,
    }
}
fn kind(node: &Node) -> &'static str {
    match node {
        Node::Object(_) => "object",
        Node::Array(_) => "array",
        Node::Leaf(v) => match v {
            Value::Null => "null",
            Value::Bool(_) => "boolean",
            Value::Number(_) => "number",
            _ => "string",
        },
    }
}
fn walk(
    node: &Node,
    prefix: &str,
    codex: bool,
    out: Option<&mut Output>,
) -> BTreeSet<(String, String)> {
    struct Walk<'a> {
        prefix: &'a str,
        names: &'a str,
        out: Option<&'a mut Output>,
        fields: BTreeSet<(String, String)>,
    }
    impl Walk<'_> {
        fn visit(&mut self, node: &Node, path: &str, name: &str, emit: bool) {
            if emit {
                let ty = kind(node);
                self.fields.insert((path.into(), ty.into()));
                let value = match node {
                    Node::Leaf(Value::Bool(v)) => format!(" {v}"),
                    Node::Leaf(Value::Number(v))
                        if time_name(name)
                            || matches!(
                                name,
                                "usedPercent" | "remainingPercent" | "percent" | "utilization"
                            ) =>
                    {
                        format!(" {v}")
                    }
                    Node::Leaf(Value::String(v)) if time_name(name) && time_shape(v) => {
                        format!(" {v}")
                    }
                    Node::Leaf(Value::String(v)) if words(name).is_some() => format!(
                        " word:{}",
                        words(name).unwrap().split_whitespace().any(|w| w == v)
                    ),
                    _ => String::new(),
                };
                if let Some(out) = self.out.as_mut() {
                    out.line(&format!("{}.field {}: {}{}", self.prefix, path, ty, value));
                }
            }
            match node {
                Node::Object(entries) => {
                    for (i, (key, value)) in entries.iter().enumerate() {
                        let k = if name == "rateLimitsByLimitId" {
                            format!("[{}]", i + 1)
                        } else if self.names.split_whitespace().any(|n| n == key) {
                            key.clone()
                        } else {
                            "?".into()
                        };
                        let p = if path.is_empty() {
                            k.clone()
                        } else if k.starts_with('[') {
                            format!("{path}{k}")
                        } else {
                            format!("{path}.{k}")
                        };
                        // Data keys cannot authorize a value, even if they look like an allowed name.
                        let value_name = if name == "rateLimitsByLimitId" {
                            ""
                        } else {
                            &k
                        };
                        self.visit(value, &p, value_name, true);
                    }
                }
                Node::Array(entries) => {
                    for (i, v) in entries.iter().enumerate() {
                        self.visit(v, &format!("{path}[{}]", i + 1), "", true);
                    }
                }
                _ => {}
            }
        }
    }
    let mut walk = Walk {
        prefix,
        names: if codex { CODEX_NAMES } else { CLAUDE_NAMES },
        out,
        fields: BTreeSet::new(),
    };
    walk.visit(node, "", "", false);
    walk.fields
}

pub fn print(
    node: &Node,
    prefix: &str,
    codex: bool,
    out: &mut Output,
) -> BTreeSet<(String, String)> {
    walk(node, prefix, codex, Some(out))
}
pub fn shape(node: &Node, codex: bool) -> BTreeSet<(String, String)> {
    walk(node, "", codex, None)
}
