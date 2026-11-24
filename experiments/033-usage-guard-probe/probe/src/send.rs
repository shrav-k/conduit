use crate::{failure::Step, process};
use serde_json::{Value, json};
use std::io::{self, Write};

/// The three app-server messages the probe sends, as it sends them.
pub fn initialize() -> Value {
    json!({"id":1,"method":"initialize","params":{
        "clientInfo":{"name":"usage-guard-probe","version":"0.1.0"},
        "capabilities":{"experimentalApi":false}}})
}
pub fn initialized() -> Value {
    json!({"method":"initialized"})
}
pub fn rate_limits_read() -> Value {
    json!({"id":2,"method":"account/rateLimits/read","params":{"excludeResetCreditDetails":true}})
}

/// Nothing is written to a client once SIGINT, SIGTERM or SIGHUP has come.
fn withheld() -> io::Result<()> {
    if process::interrupted() {
        return Err(Step::Interrupted.error(io::Error::other("interrupted")));
    }
    Ok(())
}

/// Every app-server write passes this check before serialization or a write: the message must
/// be one of the three above, whole, with no other id, parameter, capability or field.
pub fn codex(out: &mut impl Write, message: &Value) -> io::Result<()> {
    let allowed = [initialize(), initialized(), rate_limits_read()].contains(message);
    if !allowed {
        // MUTATION_TARGET: the test must catch losing this refusal.
        return Err(io::Error::other("method_refused"));
    }
    withheld()?;
    writeln!(out, "{message}").map_err(|e| Step::Write.error(e))?;
    out.flush().map_err(|e| Step::Write.error(e))
}

pub fn claude(out: &mut impl Write, content: &str) -> io::Result<()> {
    if content != "/usage" {
        return Err(io::Error::other("content_refused"));
    }
    withheld()?;
    writeln!(
        out,
        "{}",
        json!({"type":"user", "message":{"role":"user","content":content},
        "parent_tool_use_id":null,"session_id":""})
    )
    .map_err(|e| Step::Write.error(e))?;
    out.flush().map_err(|e| Step::Write.error(e))
}
