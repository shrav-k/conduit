//! Rewrites of the specification's patterns under ruling 48, point 3.
//! Every old expression is matched by regress, as are all patterns in its new allOf/not.
mod common;
use common::*;
use conduit_loader::{Node, Value, read_file};
use serde_json::Value as Json;

#[test]
fn every_rewritten_pattern_keeps_its_verdict() {
    let cases = [
        (
            "address.v1",
            "",
            r###"^([a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?\.)+(?![0-9]+(/|$))[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?(/[a-z0-9][a-z0-9._-]*)*$"###,
        ),
        (
            "qualified-reference.v1",
            "",
            r###"^([a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?\.)+(?![0-9]+([/#]|$))[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?(/[a-z0-9][a-z0-9._-]*)*#(?=.{1,63}$)[a-z][a-z0-9-]*(\.[a-z][a-z0-9-]*)*$"###,
        ),
        (
            "destination.v1",
            "",
            r###"^(/(?!-(/|$))([^/~]|~[01])*)+$"###,
        ),
        (
            "template-spec.v1",
            "/$defs/reaches/properties/to/items",
            r###"^/(conduits/[a-z][a-z0-9-]{0,62}/target|(config|conduits/[a-z][a-z0-9-]{0,62}/arguments)(/(?!-(/|$))([^/~]|~[01])*)*)$"###,
        ),
        (
            "pool-template-spec.v1",
            "/$defs/reaches/properties/to/items",
            r###"^/(entries/[a-z][a-z0-9-]{0,62}/(placement|runtime)|(config|entries/[a-z][a-z0-9-]{0,62}/arguments)(/(?!-(/|$))([^/~]|~[01])*)*)$"###,
        ),
        (
            "conduit-template-spec.v1",
            "/$defs/reaches/properties/to/items",
            r###"^/(operations|requirements/[a-z][a-z0-9-]{0,62}/arguments(/(?!-(/|$))([^/~]|~[01])*)*)$"###,
        ),
        (
            "server-package.v1",
            "/properties/transport/anyOf/1/properties/url",
            r###"^https?://[^\s]+$"###,
        ),
        (
            "server-remote.v1",
            "/properties/url",
            r###"^https?://[^\s]+$"###,
        ),
    ];

    fn strings(node: &Node, out: &mut Vec<String>) {
        match &node.value {
            Value::String(s) => out.push(s.clone()),
            Value::Array(items) => items.iter().for_each(|v| strings(v, out)),
            Value::Object(members) => members.iter().for_each(|(_, v)| strings(v, out)),
            _ => {}
        }
    }
    fn examples_in(dir: &std::path::Path, out: &mut Vec<String>) {
        for file in files_in(dir) {
            // A file rejected by the strict reader has no document value to match.
            if let Ok(documents) = read_file(&file) {
                for document in documents {
                    strings(&document.root, out);
                }
            }
        }
        for sub in directories_in(dir) {
            examples_in(&sub, out);
        }
    }
    enum Expression {
        Pattern(regress::Regex),
        All(Vec<Expression>),
        Not(Box<Expression>),
    }
    impl Expression {
        fn read(schema: &Json) -> Self {
            if let Some(p) = schema.get("pattern").and_then(Json::as_str) {
                Self::Pattern(regress::Regex::with_flags(p, "u").unwrap())
            } else if let Some(parts) = schema.get("allOf").and_then(Json::as_array) {
                Self::All(parts.iter().map(Self::read).collect())
            } else {
                Self::Not(Box::new(Self::read(&schema["not"])))
            }
        }
        fn matches(&self, s: &str) -> bool {
            match self {
                Self::Pattern(p) => {
                    regress::backends::find::<regress::backends::PikeVMExecutor>(p, s, 0)
                        .next()
                        .is_some()
                }
                Self::All(parts) => parts.iter().all(|p| p.matches(s)),
                Self::Not(p) => !p.matches(s),
            }
        }
    }
    let mut samples = Vec::new();
    examples_in(&examples(), &mut samples);
    let base = [
        "",
        "a",
        "-",
        "--",
        ".",
        "..",
        "~",
        "~0",
        "~1",
        "~2",
        "0",
        "123",
        "a.1",
        "a.org",
        "xn--a.org",
        "a_b",
        "a-b",
        "😀",
        "é",
        "a\n",
    ];
    for a in base {
        for b in base {
            for text in [
                format!("{a}.{b}"),
                format!("{a}.{b}/p"),
                format!("{a}.{b}#read.v1"),
                format!("/{a}/{b}"),
            ] {
                samples.push(text);
            }
        }
    }
    // Boundary lengths of DNS labels and export names; all template destinations, empty
    // reference tokens, dash tokens, escaped tokens, malformed escapes, line terminators.
    for n in [0, 1, 2, 62, 63, 64, 65] {
        for c in ['a', '0', '-'] {
            let part = c.to_string().repeat(n);
            samples.extend([
                format!("{part}.org"),
                format!("a.{part}"),
                format!("a.org/{part}"),
                format!("a.org#{part}"),
                format!("a.org#x.{part}"),
            ]);
        }
    }
    for token in base {
        for root in [
            "config",
            "conduits/a/arguments",
            "conduits/a/target",
            "entries/a/arguments",
            "entries/a/placement",
            "entries/a/runtime",
            "operations",
            "requirements/a/arguments",
        ] {
            for tail in ["", "/", "/x", "/-", "/--", "/~0", "/~1"] {
                samples.push(format!("/{root}/{token}{tail}"));
            }
        }
    }
    // Every ECMA-262 white space and line terminator, their near neighbors, and the code
    // points Python's shorthand used to include incorrectly.
    for code in [
        9, 10, 11, 12, 13, 32, 160, 0x1680, 0x2028, 0x2029, 0x202f, 0x205f, 0x3000, 0xfeff, 0x1c,
        0x1d, 0x1e, 0x1f, 0x85,
    ]
    .into_iter()
    .chain(0x2000..=0x200a)
    {
        for nearby in code - 1..=code + 1 {
            let c = char::from_u32(nearby).unwrap();
            for prefix in ["http://", "https://", "https://example.org/"] {
                samples.extend([format!("{prefix}{c}"), format!("{prefix}x{c}y")]);
            }
        }
    }
    // Deterministic generated strings, with useful stems so matches and refusals are both
    // common. No external RNG, locale, wall clock or network decides a test case.
    let alphabet: Vec<char> = "abc019-._~/#: 😀é\n\r\u{feff}".chars().collect();
    let stems = [
        "",
        "a.org/",
        "a.123/",
        "a.org#",
        "/config/",
        "/conduits/a/arguments/",
        "/entries/a/arguments/",
        "/requirements/a/arguments/",
        "https://",
    ];
    let mut state = 0x123456789abcdef_u64;
    for i in 0..6000 {
        let mut text = stems[i % stems.len()].to_string();
        for _ in 0..i % 70 {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            text.push(alphabet[(state >> 32) as usize % alphabet.len()]);
        }
        samples.push(text);
    }
    assert!(samples.len() > 10_000);
    for (name, pointer, old) in cases {
        let old = regress::Regex::with_flags(old, "u").unwrap();
        let schema = specification()
            .contract(name)
            .unwrap()
            .pointer(pointer)
            .unwrap();
        let new = Expression::read(schema);
        let mut accepted = 0;
        for text in &samples {
            let before =
                regress::backends::find::<regress::backends::PikeVMExecutor>(&old, text, 0)
                    .next()
                    .is_some();
            assert_eq!(before, new.matches(text), "{name}{pointer}: {text:?}");
            accepted += usize::from(before);
        }
        assert!(
            accepted > 0 && accepted < samples.len(),
            "{name}: vacuous comparison"
        );
    }
}
