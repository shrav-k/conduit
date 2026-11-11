//! The formats and the patterns of the profile: the grammar of `uuid` that SCH-004 cites, RFC
//! 9562; the other formats asserted, not only annotated (SCH-001); and `pattern` matched as
//! ECMA-262 matches it with the `u` flag (SCH-016), whatever library another engine uses.

mod common;

use common::*;
use conduit_loader::{
    Document, Role, Syntax, compile, formats::is_duration, formats::is_uuid, read_text, rules,
};
use serde_json::{Value, json};

fn valid(schema: &Value, instance: &str) -> bool {
    compile(schema)
        .unwrap_or_else(|fault| panic!("{schema}: {fault}"))
        .is_valid(&json!(instance))
}

fn uuid(text: &str) -> bool {
    let checked = valid(&json!({"type": "string", "format": "uuid"}), text);
    assert_eq!(
        checked,
        is_uuid(text),
        "{text:?}: the validator and the grammar disagree"
    );
    checked
}

#[test]
fn a_uuid_matches_the_grammar_of_rfc_9562() {
    for text in [
        // The example of RFC 9562, section 4, and one of version 7, from its appendix A.
        "f81d4fae-7dec-11d0-a765-00a0c91e6bf6",
        "017f22e2-79b0-7cc3-98c4-dc0c0c07398f",
        // The letters are case-insensitive on input.
        "F81D4FAE-7DEC-11D0-A765-00A0C91E6BF6",
        "f81D4fAE-7dec-11D0-a765-00A0c91e6bf6",
        // The Nil UUID and the Max UUID, which RFC 9562 defines (sections 5.9 and 5.10).
        "00000000-0000-0000-0000-000000000000",
        "ffffffff-ffff-ffff-ffff-ffffffffffff",
        "FFFFFFFF-FFFF-FFFF-FFFF-FFFFFFFFFFFF",
    ] {
        assert!(uuid(text), "{text}");
    }
}

#[test]
fn the_grammar_fixes_no_version_and_no_variant() {
    // The grammar is the string form alone: a version that RFC 9562 does not define (0, 9,
    // 15), and a variant it reserves (the NCS variant 0xxx, Microsoft's 110x and the future
    // 111x), each match it.
    for text in [
        "f81d4fae-7dec-01d0-a765-00a0c91e6bf6",
        "f81d4fae-7dec-91d0-a765-00a0c91e6bf6",
        "f81d4fae-7dec-f1d0-a765-00a0c91e6bf6",
        "f81d4fae-7dec-11d0-0765-00a0c91e6bf6",
        "f81d4fae-7dec-11d0-c765-00a0c91e6bf6",
        "f81d4fae-7dec-11d0-e765-00a0c91e6bf6",
    ] {
        assert!(uuid(text), "{text}");
    }
}

#[test]
fn other_forms_of_a_uuid_are_not_the_grammar() {
    for text in [
        // Forms that some libraries also parse.
        "{f81d4fae-7dec-11d0-a765-00a0c91e6bf6}",
        "urn:uuid:f81d4fae-7dec-11d0-a765-00a0c91e6bf6",
        "f81d4fae7dec11d0a76500a0c91e6bf6",
        // Hyphens out of place, too few or too many digits, a letter that is not hexadecimal.
        "f81d4fae7-dec-11d0-a765-00a0c91e6bf6",
        "f81d4fae-7dec-11d0-a765-00a0c91e6bf",
        "f81d4fae-7dec-11d0-a765-00a0c91e6bf6a",
        "g81d4fae-7dec-11d0-a765-00a0c91e6bf6",
        // White space, and digits and letters outside ASCII: fullwidth, and Arabic-Indic.
        " f81d4fae-7dec-11d0-a765-00a0c91e6bf6",
        "f81d4fae-7dec-11d0-a765-00a0c91e6bf6\n",
        "\u{ff46}81d4fae-7dec-11d0-a765-00a0c91e6bf",
        "\u{0660}81d4fae-7dec-11d0-a765-00a0c91e6bf",
        "",
    ] {
        assert!(!uuid(text), "{text:?}");
    }
    // A format constrains a string alone.
    let schema = json!({"format": "uuid"});
    assert!(compile(&schema).unwrap().is_valid(&json!(15)));
}

#[test]
fn every_format_of_the_profile_is_asserted() {
    for (format, good, bad) in [
        ("date-time", "2026-11-09T10:00:00Z", "2026-11-09 10:00:00"),
        (
            "date-time",
            "2026-11-09T10:00:00.5+01:00",
            "2026-11-09T10:00:00",
        ),
        ("date", "2026-11-09", "2026-02-30"),
        ("duration", "P1DT2H", "1D"),
        ("uri", "https://example.org/files", "/files"),
        ("uri", "https://example.org/files", "https://exa mple.org"),
        (
            "uri-reference",
            "../history/history-1.4.0.tar.gz",
            "https://exa mple.org",
        ),
        ("uuid", "f81d4fae-7dec-11d0-a765-00a0c91e6bf6", "f81d4fae"),
    ] {
        let schema = json!({"type": "string", "format": format});
        assert!(valid(&schema, good), "{format}: {good}");
        assert!(!valid(&schema, bad), "{format}: {bad}");
    }
}

fn duration(text: &str) -> bool {
    let checked = valid(&json!({"type": "string", "format": "duration"}), text);
    assert_eq!(
        checked,
        is_duration(text),
        "{text:?}: the validator and the grammar disagree"
    );
    checked
}

#[test]
fn a_duration_matches_the_grammar_of_rfc_3339_appendix_a() {
    for text in [
        "P1D",
        "P1M",
        "P1Y",
        "P1W",
        "P1M2D",
        "P1Y2M",
        "P1Y2M3D",
        "PT1H",
        "PT1M",
        "PT1S",
        "PT1H2M",
        "PT1H2M3S",
        "PT2M3S",
        "P1DT2H",
        "P1Y2M3DT4H5M6S",
        "P0D",
        "P007D",
        // The quoted letters of ABNF match either case (RFC 5234, section 2.3).
        "p1d",
        "P1d",
        "PT1h",
        "pt1H",
        "P1w",
    ] {
        assert!(duration(text), "{text}");
    }
    for text in [
        // Units out of the grammar's order, or skipped where it nests them.
        "P1Y3D",
        "PT1H30S",
        "P1D1D",
        "P1M2Y",
        "P1W1D",
        "P1WT1H",
        // A fraction, a sign, a comma, a missing number, a missing unit and an empty time.
        "PT0.5S",
        "-P1D",
        "P1,5D",
        "P.5W",
        "P-0Y",
        "PD",
        "P1",
        "P",
        "PT",
        "P1DT",
        // White space, and digits outside ASCII.
        " P1D",
        "P1D\n",
        "P\u{0661}D",
        "P1 D",
        "",
    ] {
        assert!(!duration(text), "{text:?}");
    }
}

fn matches(pattern: &str, text: &str) -> bool {
    valid(&json!({"type": "string", "pattern": pattern}), text)
}

#[test]
fn a_pattern_is_matched_as_ecma_262_with_the_u_flag() {
    // `$` is the end of the string: a final line feed is not before it. Python's `re` would
    // match here, and so did `tools/check_schemas.py` before the first build's log of
    // increment 1.
    assert!(matches("^[a-z][a-z0-9-]*$", "console"));
    assert!(!matches("^[a-z][a-z0-9-]*$", "console\n"));
    // Unicode classes and literal characters retain ECMA-262's meaning.
    assert!(!matches("^[0-9]$", "\u{0663}"));
    assert!(!matches("^[A-Za-z0-9_]+$", "é"));
    assert!(matches("^[\u{feff}]$", "\u{feff}"));
    assert!(!matches("^[\u{feff}]$", "\u{1c}"));
    assert!(matches("^[😀]$", "😀"));
    // Patterns are unanchored, and escaped punctuation stands for itself.
    assert!(matches("a", "cat"));
    assert!(matches(r"^a\+b\.$", "a+b."));
    assert!(matches("^[-a-z]+$", "a-b"));
}

#[test]
fn the_specifications_host_constraint_is_matched() {
    // `address.v1` excludes a host whose last label is all digits with `not`.
    let address = specification()
        .contract("address.v1")
        .expect("address.v1")
        .clone();
    let schema = json!({"$defs": {"address": address}, "$ref": "#/$defs/address"});
    assert!(valid(&schema, "example.org/files"));
    assert!(!valid(&schema, "127.0.0.1/acme/history"));
    assert!(!valid(&schema, "example.org/../history"));
}

#[test]
fn a_pattern_that_is_not_ecma_262_in_unicode_mode_is_rejected() {
    // Python's named groups, an inline flag, a lone brace, an identity escape that Unicode
    // mode forbids, and a property that does not exist.
    for pattern in [
        "(?P<x>a)",
        "(?i)a",
        "a{",
        r"\-",
        r"\p{Foo}",
        "[z-a]",
        // Syntax of the edition of 2025, later than the one JSON Schema 2020-12 cites, which
        // `regress` accepts: a group that sets flags, and a name given to two groups.
        "(?i:a)",
        "(?-i:a)",
        "(?<a>x)|(?<a>y)",
        r"(?<\u0061>x)|(?<a>y)",
    ] {
        let fault = compile(&json!({"type": "string", "pattern": pattern}))
            .err()
            .unwrap_or_else(|| panic!("{pattern} compiled"));
        assert_eq!(fault.rule, rules::SCH_016, "{pattern}: {fault}");
    }
}

#[test]
fn only_interoperable_tokens_enter_a_pattern() {
    for pattern in [
        "(?:a)",
        "(?=a)a",
        "(?!b)a",
        "(?<=a)b",
        "(?<!a)b",
        "(?<a>x)|(?<b>y)",
        r"\d",
        r"\w",
        r"\s",
        r"\b",
        r"(a)\1",
        r"\p{Lu}",
        ".",
        r"\u0041",
        "a++",
        "a*+",
        "a{1}+",
        "a???",
        "[a-z",
        "a{,2}",
        "a{2,1}",
    ] {
        let fault = compile(&json!({"pattern": pattern})).expect_err(pattern);
        assert_eq!(fault.rule, rules::SCH_016, "{pattern}: {fault}");
    }
    for pattern in [
        "",
        "a",
        "😀",
        "[a-z]",
        "[^a-z]",
        "[]",
        "[^]",
        "[[]",
        "[a&&b]",
        "a+?",
        "a*?",
        "a??",
        "a{2}",
        "a{2,3}?",
        "a{2,}",
        "^(a|b)$",
        "[(?i:a)]",
        r"\(\?i:a\)",
        r"[\-]",
        r"[\]]",
        r"\^\$\\\.\*\+\?\(\)\[\]\{\}\|",
    ] {
        compile(&json!({"pattern": pattern})).unwrap_or_else(|fault| panic!("{pattern}: {fault}"));
    }
}

#[test]
fn a_document_whose_schema_holds_such_a_pattern_is_rejected() {
    let contract = |pattern: &str| {
        format!(
            "apiVersion: {api}\nkind: Schema\nmetadata:\n  name: label.v1\nspec:\n  type: object\n  properties:\n    tag: {{type: string, pattern: '{pattern}'}}\n  patternProperties:\n    '^x-': {{type: string}}\n",
            api = api()
        )
    };
    submit(&[("label.yaml", &contract("^[a-z]+$"))]).unwrap_or_else(|faults| panic!("{faults}"));
    let faults = submit(&[("label.yaml", &contract("(?P<x>a)"))]).expect_err("rejected");
    assert_eq!(faults.rules(), vec![rules::SCH_016], "{faults}");
    assert_eq!(
        faults.0[0].pointer.as_deref(),
        Some("/spec/properties/tag/pattern")
    );
    // A property named `pattern` is a name, and its schema is not taken for a pattern.
    let named = format!(
        "apiVersion: {api}\nkind: Schema\nmetadata:\n  name: find.v1\nspec:\n  type: object\n  properties:\n    pattern: {{type: string, minLength: 1}}\n",
        api = api()
    );
    submit(&[("find.yaml", &named)]).unwrap_or_else(|faults| panic!("{faults}"));
    // A key of patternProperties follows exactly the same token list.
    let keyed = contract("^[a-z]+$").replace("'^x-'", "'(?=x)x'");
    let faults = submit(&[("label.yaml", &keyed)]).expect_err("lookahead key");
    assert_eq!(faults.rules(), vec![rules::SCH_016]);
    assert_eq!(
        faults.0[0].pointer.as_deref(),
        Some("/spec/patternProperties/(?=x)x")
    );
    // The rule is part of the verdict that `check` gives, as reading a project gives it, and
    // not of `validate`, which applies the schemas alone.
    let text = contract("(?P<x>a)");
    let root = read_text(Syntax::Yaml, &text).unwrap().remove(0);
    let document = Document {
        file: "label.yaml".to_string(),
        index: 1,
        root,
    };
    specification()
        .validate(&document, Role::Document)
        .unwrap_or_else(|fault| panic!("{fault}"));
    let faults = specification()
        .check(&document, Role::Document)
        .expect_err("rejected");
    assert_eq!(faults.rules(), vec![rules::SCH_016], "{faults}");
}
