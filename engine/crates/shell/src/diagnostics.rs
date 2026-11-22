//! Author-facing rendering of SARIF diagnostics.

use serde_json::Value;

pub(crate) fn diagnostics(reply: &mut Value) -> Option<&mut Vec<Value>> {
    for pointer in [
        "/structuredContent/diagnostics",
        "/diagnostics",
        "/error/data/diagnostics",
    ] {
        if reply.pointer(pointer).is_some_and(Value::is_array) {
            return reply.pointer_mut(pointer).and_then(Value::as_array_mut);
        }
    }
    None
}

/// Print one line for each diagnostic and one note for every other authored place.
/// `path` is the directory or archive argument as written on the command line.
pub fn print_diagnostics(diagnostics: &[Value], path: &str, archive: bool) {
    for diagnostic in diagnostics {
        let place = display_place(&diagnostic["locations"][0], path, archive);
        let level = diagnostic["level"].as_str().unwrap_or("error");
        let message = diagnostic["message"]["text"]
            .as_str()
            .unwrap_or("rejected")
            .replace(['\r', '\n'], " ");
        let rule = diagnostic["ruleId"].as_str().unwrap_or("?");
        let translation = &diagnostic["properties"]["translation"];
        if let (Some(origin), Some(name)) = (
            translation["rule"].as_str(),
            translation["document"].as_str(),
        ) {
            eprintln!("{place}: {level}: {message} [{rule}; {origin} {name}]");
        } else {
            eprintln!("{place}: {level}: {message} [{rule}]");
        }
        for related in diagnostic["relatedLocations"]
            .as_array()
            .into_iter()
            .flatten()
        {
            let place = display_place(related, path, archive);
            eprintln!("{place}: note: also here [{rule}]");
        }
    }
}

fn display_place(location: &Value, fallback: &str, archive: bool) -> String {
    let physical = &location["physicalLocation"];
    let uri = physical["artifactLocation"]["uri"].as_str();
    let path = match uri {
        Some(uri) if archive => format!("{fallback}({uri})"),
        Some(uri) => uri.to_owned(),
        None => fallback.to_owned(),
    };
    let region = &physical["region"];
    match (region["startLine"].as_u64(), region["startColumn"].as_u64()) {
        (Some(line), Some(column)) => format!("{path}:{line}:{column}"),
        _ => path,
    }
}
