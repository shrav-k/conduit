use serde_json::{Value, json};
use std::collections::BTreeMap;

pub const MEMBER: &str = "io.github.shrav-k.conduit/usage";

pub(crate) struct Usage {
    version: Option<String>,
    previous: Option<BTreeMap<String, Counts>>,
}

impl Default for Usage {
    fn default() -> Self {
        Self {
            version: None,
            // A new process has no prior counts. None represents an unreadable report.
            previous: Some(BTreeMap::new()),
        }
    }
}

impl Usage {
    pub(crate) fn init(&mut self, event: &Value) {
        self.version = event["claude_code_version"]
            .as_str()
            .filter(|version| {
                (1..=64).contains(&version.len())
                    && version
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || b".+-".contains(&byte))
            })
            .map(str::to_owned);
    }

    pub(crate) fn result(&mut self, event: &Value) -> Option<Value> {
        let current = event["modelUsage"].as_object().and_then(|report| {
            report
                .iter()
                .map(|(model, counts)| {
                    if !(1..=256).contains(&model.chars().count()) {
                        return None;
                    }
                    Some((model.clone(), Counts::read(counts)?))
                })
                .collect()
        });
        let previous = std::mem::replace(&mut self.previous, current)?;
        let version = self.version.as_ref()?;
        let mut models = serde_json::Map::new();
        for (model, current) in self.previous.as_ref()? {
            let delta = current.delta(previous.get(model))?;
            if let Some(value) = delta {
                models.insert(model.clone(), value);
            }
        }
        (1..=16)
            .contains(&models.len())
            .then(|| json!({"client":{"name":"claude-code","version":version},"models":models}))
    }
}

struct Counts {
    input: u64,
    read: Option<u64>,
    created: Option<u64>,
    output: u64,
    thinking: Option<u64>,
}

impl Counts {
    fn read(report: &Value) -> Option<Self> {
        let optional = |key| match report.get(key) {
            Some(count) => count.as_u64().map(Some),
            None => Some(None),
        };
        Some(Self {
            input: report["inputTokens"].as_u64()?,
            read: optional("cacheReadInputTokens")?,
            created: optional("cacheCreationInputTokens")?,
            output: report["outputTokens"].as_u64()?,
            thinking: optional("thinkingTokens")?,
        })
    }

    fn delta(&self, previous: Option<&Self>) -> Option<Option<Value>> {
        let optional_delta =
            |current: Option<u64>, before: Option<u64>| match (current, before, previous.is_none())
            {
                (Some(count), _, true) => Some(Some(count)),
                (Some(count), Some(earlier), false) => count.checked_sub(earlier).map(Some),
                _ => Some(None),
            };
        let uncached = self
            .input
            .checked_sub(previous.map_or(0, |counts| counts.input))?;
        let output = self
            .output
            .checked_sub(previous.map_or(0, |counts| counts.output))?;
        let old_read = previous.and_then(|counts| counts.read);
        let old_created = previous.and_then(|counts| counts.created);
        let cached = optional_delta(self.read, old_read)?;
        let created = optional_delta(self.created, old_created)?;
        let reasoning = optional_delta(self.thinking, previous.and_then(|counts| counts.thinking))?;
        if previous.is_some()
            && (self.read.is_some() != old_read.is_some()
                || self.created.is_some() != old_created.is_some())
        {
            // A change in reported cache counters makes total input incomparable.
            return Some(None);
        }
        let input = uncached
            .checked_add(cached.unwrap_or_default())?
            .checked_add(created.unwrap_or_default())?;
        if [
            input,
            output,
            cached.unwrap_or_default(),
            reasoning.unwrap_or_default(),
        ]
        .iter()
        .all(|count| *count == 0)
        {
            return Some(None);
        }
        let mut result = json!({"input":input,"output":output});
        if let Some(cached) = cached {
            result["cachedInput"] = json!(cached);
        }
        if let Some(reasoning) = reasoning {
            result["reasoning"] = json!(reasoning);
        }
        Some(Some(result))
    }
}
