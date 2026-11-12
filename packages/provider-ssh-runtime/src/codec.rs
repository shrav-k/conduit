use std::collections::BTreeMap;

/// POSIX shell words, including empty words. Used only for program arguments and paths.
pub fn quote(word: &str) -> String {
    format!("'{}'", word.replace('\'', "'\\''"))
}
pub fn command(words: &[String]) -> String {
    words.iter().map(|s| quote(s)).collect::<Vec<_>>().join(" ")
}
/// OpenSSH's configuration parser, then its percent expansion (not shell quoting).
pub fn option_path(path: &str) -> Result<String, &'static str> {
    if !path.starts_with('/') || path.contains(['\n', '\r', '\0', '"', '\\']) {
        return Err("SSH path");
    }
    Ok(format!("\"{}\"", path.replace('%', "%%")))
}
pub fn variable(name: &str) -> bool {
    let mut bytes = name.bytes();
    bytes
        .next()
        .is_some_and(|b| b.is_ascii_alphabetic() || b == b'_')
        && bytes.all(|b| b.is_ascii_alphanumeric() || b == b'_')
}
/// systemd.exec EnvironmentFile double-quoted values: only backslash, double quote,
/// dollar and backtick need escaping. Literal newlines stay inside the quoted value.
/// UTF-8 only; NUL, BOM and Unicode noncharacters are not accepted by systemd.
pub fn environment(values: &BTreeMap<String, String>) -> Result<Vec<u8>, String> {
    let mut out = String::new();
    for (name, value) in values {
        if !variable(name)
            || value.chars().any(|c| {
                c == '\0'
                    || c == '\u{feff}'
                    || (0xfdd0..=0xfdef).contains(&(c as u32))
                    || (c as u32 & 0xffff) >= 0xfffe
            })
        {
            return Err(name.clone());
        }
        out.push_str(name);
        out.push_str("=\"");
        for c in value.chars() {
            if matches!(c, '\\' | '"' | '$' | '`') {
                out.push('\\');
            }
            out.push(c);
        }
        out.push_str("\"\n");
    }
    Ok(out.into_bytes())
}
/// Octal literal for printf %b; data travels in /bin/sh -s stdin, never argv.
pub fn octal(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("\\{b:03o}")).collect()
}
pub fn write_file(script: &mut String, path: &str, bytes: &[u8]) {
    script.push_str(&format!(
        "printf '%b' '{}' > {}\n/bin/chmod -- 600 {}\n",
        octal(bytes),
        quote(path),
        quote(path)
    ));
}
