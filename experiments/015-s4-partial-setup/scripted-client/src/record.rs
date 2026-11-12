use chrono::{SecondsFormat, Utc};
use serde_json::{Value, json};
use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    os::unix::fs::{OpenOptionsExt, PermissionsExt},
    time::Instant,
};

#[derive(Default)]
pub struct Redactor(Vec<String>);
impl Redactor {
    pub fn new(markers: &[String]) -> Self {
        let mut r = Self::default();
        for m in markers {
            r.add(m);
        }
        r
    }
    pub fn add(&mut self, secret: &str) {
        let mut s = secret.to_owned();
        for _ in 0..4 {
            self.0.push(s.clone());
            let escaped = serde_json::to_string(&s).unwrap();
            s = escaped[1..escaped.len() - 1].to_owned();
        }
        self.0.sort_by_key(|s| std::cmp::Reverse(s.len()));
        self.0.dedup();
    }
    fn text(&self, mut s: String) -> String {
        for secret in &self.0 {
            s = s.replace(secret, "[redacted]");
        }
        s
    }
    pub fn value(&self, v: Value) -> Value {
        match v {
            Value::String(s) => Value::String(self.text(s)),
            Value::Number(n) if self.0.iter().any(|secret| n.to_string().contains(secret)) => {
                Value::String("[redacted]".into())
            }
            Value::Array(a) => Value::Array(a.into_iter().map(|v| self.value(v)).collect()),
            Value::Object(o) => Value::Object(
                o.into_iter()
                    .map(|(k, v)| (self.text(k), self.value(v)))
                    .collect(),
            ),
            other => other,
        }
    }
}
pub fn create(path: &std::path::Path) -> io::Result<File> {
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)?;
    file.set_permissions(std::fs::Permissions::from_mode(0o600))?;
    Ok(file)
}
pub fn emit(v: Value) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    writeln!(stdout, "{v}")?;
    stdout.flush()
}
pub struct Trace {
    file: File,
    pub redact: Redactor,
    started: Instant,
    seq: u64,
}
impl Trace {
    pub fn new(file: File, redact: Redactor, started: Instant) -> Self {
        Self {
            file,
            redact,
            started,
            seq: 0,
        }
    }
    pub fn event(
        &mut self,
        event: &str,
        step: Option<usize>,
        data: Value,
    ) -> Result<(), &'static str> {
        self.seq += 1;
        let mut line = json!({"seq":self.seq,"at":Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),"elapsedMs":self.started.elapsed().as_millis(),"event":event});
        if let Some(step) = step {
            line["step"] = json!(step);
        }
        line.as_object_mut()
            .unwrap()
            .extend(data.as_object().unwrap().clone());
        writeln!(self.file, "{}", self.redact.value(line))
            .and_then(|()| self.file.flush())
            .map_err(|_| "trace-failed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_markers_and_token_replace_whole_numbers() {
        let mut redact = Redactor::new(&["12345678".into()]);
        redact.add("87654321");
        let value = json!({"numbers":[9123456789_u64,-9123456789_i64,9123456789.25,9876543219_u64],"kept":[17,true,null]});
        let output = redact.value(value).to_string();
        let parsed: Value = serde_json::from_str(&output).unwrap();
        assert!(!output.contains("12345678") && !output.contains("87654321"));
        assert!(parsed["numbers"] == json!(vec!["[redacted]"; 4]));
        assert!(parsed["kept"] == json!([17, true, null]));
    }
}
