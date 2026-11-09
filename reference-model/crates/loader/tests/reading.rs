//! Reading one file strictly, case by case: what SCH-012, SCH-017, SCH-006, SCH-021 and the
//! syntax rules reject, and what YAML's core schema and JSON read a value as (SCH-005, SCH-007,
//! SCH-011, SCH-013, SCH-018, SCH-020).

use conduit_loader::{
    Diagnostic, Diagnostics, MAX_DEPTH, Node, Position, Syntax, Value, read_bytes, read_text, rules,
};

fn yaml(text: &str) -> Result<Vec<Node>, Diagnostics> {
    read_text(Syntax::Yaml, text)
}

fn json(text: &str) -> Result<Vec<Node>, Diagnostics> {
    read_text(Syntax::Json, text)
}

/// The one rejection of a text, which must rest on this rule.
fn rejected(result: Result<Vec<Node>, Diagnostics>, rule: &str, text: &str) -> Diagnostic {
    match result {
        Ok(read) => panic!("{text:?} was read as {read:?}, and {rule} rejects it"),
        Err(faults) => {
            assert_eq!(faults.0.len(), 1, "{text:?}: {faults}");
            assert_eq!(faults.0[0].rule, rule, "{text:?}: {faults}");
            faults.0[0].clone()
        }
    }
}

/// The value of the key `a` of a text's one document.
fn value_of(result: Result<Vec<Node>, Diagnostics>, text: &str) -> Value {
    let documents = result.unwrap_or_else(|faults| panic!("{text:?}: {faults}"));
    assert_eq!(documents.len(), 1, "{text:?}");
    documents[0]
        .get("a")
        .unwrap_or_else(|| panic!("{text:?} has no `a`"))
        .value
        .clone()
}

fn yaml_value(text: &str) -> Value {
    value_of(yaml(text), text)
}

fn json_value(text: &str) -> Value {
    value_of(json(text), text)
}

fn string(text: &str) -> Value {
    Value::String(text.to_string())
}

/// What a rejection says, apart from where in the file it is: its rule, its document and its
/// place in the document.
fn faults_of(result: Result<Vec<Node>, Diagnostics>) -> Vec<(String, Option<usize>, String)> {
    let mut faults: Vec<_> = result
        .expect_err("rejected")
        .0
        .into_iter()
        .map(|d| {
            (
                d.rule.to_string(),
                d.document,
                d.pointer.unwrap_or_default(),
            )
        })
        .collect();
    faults.sort();
    faults
}

#[test]
fn a_duplicate_key_is_rejected() {
    for text in [
        "a: 1\na: 2\n",
        "a: 1\n\"a\": 2\n",
        "a: 1\n'a': 2\n",
        "{a: 1, a: 2}\n",
        "x:\n  b: 1\n  b: 2\n",
        // One key, spelled once plainly and once with an escape.
        "ab: 1\n\"a\\x62\": 2\n",
    ] {
        let fault = rejected(yaml(text), rules::SCH_012, text);
        assert!(fault.message.contains("duplicate key"), "{fault}");
    }
    let fault = rejected(yaml("a: 1\nb: 2\na: 3\n"), rules::SCH_012, "");
    assert_eq!(fault.position, Some(Position { line: 3, column: 1 }));
    assert_eq!(fault.document, Some(1));
    // JSON keeps no member silently: two members with one key are rejected too (I-JSON).
    for text in [r#"{"a": 1, "a": 2}"#, r#"{"x": {"b": 1, "b": 2}}"#] {
        rejected(json(text), rules::SCH_012, text);
    }
    let fault = rejected(
        json(r#"[{"a": 1}, {"x": {"b": 1, "b": 2}}]"#),
        rules::SCH_012,
        "",
    );
    assert_eq!(fault.document, Some(2));
    assert_eq!(fault.pointer.as_deref(), Some("/x/b"));
}

#[test]
fn anchors_aliases_tags_and_merge_keys_are_rejected() {
    // An anchor and the alias that refers to it are each rejected.
    let both = yaml("a: &x 1\nb: *x\n").expect_err("an anchor and an alias");
    assert_eq!(both.rules(), vec![rules::SCH_012], "{both}");
    assert_eq!(both.0.len(), 2, "{both}");
    assert!(both.0[1].message.contains("alias"), "{both}");
    for (text, what) in [
        ("a: &x 1\nb: 2\n", "anchor"),
        ("&x a: 1\n", "anchor"),
        ("a: !!str 1\n", "tag"),
        ("a: ! b\n", "tag"),
        ("a: !local b\n", "tag"),
        ("a: !!map {b: 1}\n", "tag"),
        ("a: !!seq [1]\n", "tag"),
        ("!!str a: 1\n", "tag"),
        ("%TAG !e! tag:example.org,2026:\n---\na: !e!x 1\n", "tag"),
        ("<<: {b: 1}\n", "merge key"),
        ("a:\n  <<: {b: 1}\n  c: 2\n", "merge key"),
    ] {
        let fault = rejected(yaml(text), rules::SCH_012, text);
        assert!(fault.message.contains(what), "{text:?}: {fault}");
    }
    // A quoted `<<` is a string key, never a merge key.
    assert_eq!(yaml_value("\"<<\": 1\na: 2\n"), Value::Integer(2));
}

#[test]
fn a_key_that_is_not_a_string_is_rejected() {
    for text in [
        "1: a\n",
        "1.5: a\n",
        "true: a\n",
        "null: a\n",
        "~: a\n",
        ": a\n",
        "0x1F: a\n",
        ".inf: a\n",
        "[a]: b\n",
        "? [a, b]\n: c\n",
        "? {a: b}\n: c\n",
        "{[a]: b}\n",
    ] {
        let fault = rejected(yaml(text), rules::SCH_012, text);
        assert!(fault.message.contains("not a string"), "{text:?}: {fault}");
    }
    // A key the core schema reads as a string, or one that is quoted, is a string.
    for text in [
        "\"1\": x\na: 1\n",
        "yes: x\na: 1\n",
        "'true': x\na: 1\n",
        "? |\n  b\n: x\na: 1\n",
    ] {
        assert_eq!(yaml_value(text), Value::Integer(1), "{text:?}");
    }
}

#[test]
fn a_number_with_no_json_form_is_rejected() {
    for text in [
        ".inf", "-.inf", "+.INF", ".Inf", ".nan", ".NaN", ".NAN", "0x1F", "0xff", "0o17",
    ] {
        rejected(yaml(&format!("a: {text}\n")), rules::SCH_017, text);
    }
    // The core schema of YAML 1.2 reads these as strings, so nothing is rejected.
    for text in [
        "-.nan",
        "0b101",
        "0x",
        "0o8",
        ".infinity",
        "0X1F",
        "1_000",
        "+0x1F",
    ] {
        assert_eq!(yaml_value(&format!("a: {text}\n")), string(text), "{text}");
    }
    // Quoted, every one of them is a string.
    assert_eq!(yaml_value("a: \".inf\"\n"), string(".inf"));
    assert_eq!(yaml_value("a: '0x1F'\n"), string("0x1F"));
    // JSON has none of them: each is not JSON.
    for text in [r#"{"a": NaN}"#, r#"{"a": Infinity}"#, r#"{"a": 0x1F}"#] {
        rejected(json(text), rules::SUB_005, text);
    }
}

#[test]
fn numbers_out_of_range_are_rejected() {
    let largest = format!("{:.0}", f64::MAX);
    for (text, rule) in [
        ("9007199254740992", rules::SCH_006),
        ("-9007199254740992", rules::SCH_006),
        ("1e16", rules::SCH_006),
        ("1e400", rules::SCH_006),
        ("1.5e300", rules::SCH_006),
        // Written with a fraction, and read as a binary64 value that is whole and outside the
        // range: no integer outside it is read (SCH-006, SCH-020).
        ("9007199254740993.5", rules::SCH_006),
        ("-9007199254740993.5", rules::SCH_006),
        ("9007199254740991.5", rules::SCH_006),
        (&format!("{largest}.5") as &str, rules::SCH_021),
    ] {
        let fault = rejected(yaml(&format!("a: {text}\n")), rule, text);
        assert_eq!(
            fault.position,
            Some(Position { line: 1, column: 4 }),
            "{text}"
        );
        rejected(json(&format!("{{\"a\": {text}}}")), rule, text);
    }
    assert_eq!(
        yaml_value("a: 9007199254740991\n"),
        Value::Integer(9_007_199_254_740_991)
    );
    assert_eq!(
        json_value(r#"{"a": -9007199254740991}"#),
        Value::Integer(-9_007_199_254_740_991)
    );
}

#[test]
fn a_number_is_read_by_its_value() {
    // SCH-005 and SCH-007: 15, 15.0 and 1.5e1 are one integer, in either syntax.
    for text in ["15", "15.0", "1.5e1", "150e-1", "+15", "015"] {
        assert_eq!(
            yaml_value(&format!("a: {text}\n")),
            Value::Integer(15),
            "{text}"
        );
    }
    for text in ["15", "15.0", "1.5e1", "150e-1"] {
        assert_eq!(
            json_value(&format!("{{\"a\": {text}}}")),
            Value::Integer(15),
            "{text}"
        );
    }
    assert_eq!(Value::Integer(1), Value::Float(1.0));
    // SCH-020: a number with a fractional part is the nearest binary64 value.
    assert_eq!(yaml_value("a: 0.7\n"), Value::Float(0.7));
    assert_eq!(yaml_value("a: .5\n"), Value::Float(0.5));
    assert_eq!(json_value(r#"{"a": 15.5}"#), Value::Float(15.5));
    assert_eq!(yaml_value("a: 1.\n"), Value::Integer(1));
    assert_eq!(yaml_value("a: -0\n"), Value::Integer(0));
    // SCH-020 and SCH-006: a binary64 value read from a number written with a fraction, and
    // that has no fractional part, is an integer within the range.
    for (text, whole) in [
        ("0.99999999999999999999", 1),
        ("1e-400", 0),
        ("9007199254740990.5", 9_007_199_254_740_990),
    ] {
        assert_eq!(
            yaml_value(&format!("a: {text}\n")),
            Value::Integer(whole),
            "{text}"
        );
        assert_eq!(
            json_value(&format!("{{\"a\": {text}}}")),
            Value::Integer(whole),
            "{text}"
        );
    }
}

#[test]
fn a_plain_scalar_is_read_by_the_core_schema_of_yaml_1_2() {
    for text in ["a:\n", "a: ~\n", "a: null\n", "a: Null\n", "a: NULL\n"] {
        assert_eq!(yaml_value(text), Value::Null, "{text:?}");
    }
    for (text, b) in [
        ("true", true),
        ("True", true),
        ("TRUE", true),
        ("false", false),
        ("FALSE", false),
    ] {
        assert_eq!(
            yaml_value(&format!("a: {text}\n")),
            Value::Bool(b),
            "{text}"
        );
    }
    // What YAML 1.1 read as booleans and numbers are strings here, and none is converted
    // (SCH-011): `yes` is the string "yes", and "15" is a string.
    for text in [
        "yes", "no", "on", "off", "Yes", "y", "n", "tRUE", "nULL", "1:30",
    ] {
        assert_eq!(yaml_value(&format!("a: {text}\n")), string(text), "{text}");
    }
    assert_eq!(yaml_value("a: \"15\"\n"), string("15"));
    assert_eq!(yaml_value("a: 'true'\n"), string("true"));
    assert_eq!(yaml_value("a: |\n  15\n"), string("15\n"));
}

#[test]
fn a_yaml_directive_names_version_1() {
    rejected(yaml("%YAML 2.0\n---\na: 1\n"), rules::SUB_005, "%YAML 2.0");
    // YAML 1.2.2 reads a document that says it is version 1.1 as version 1.2.
    assert_eq!(yaml_value("%YAML 1.1\n---\na: yes\n"), string("yes"));
    assert_eq!(yaml_value("%YAML 1.2\n---\na: 1\n"), Value::Integer(1));
    assert_eq!(
        yaml_value("%TAG !e! tag:example.org,2026:\n---\na: 1\n"),
        Value::Integer(1)
    );
    rejected(
        yaml("a: 1\n...\n%YAML 3.1\n---\nb: 2\n"),
        rules::SUB_005,
        "a later %YAML 3.1",
    );
}

#[test]
fn a_yaml_file_holds_any_number_of_documents() {
    assert_eq!(yaml("").unwrap().len(), 0);
    assert_eq!(yaml("# only a comment\n").unwrap().len(), 0);
    assert_eq!(yaml("a: 1\n---\nb: 2\n").unwrap().len(), 2);
    let empty = yaml("---\n").unwrap();
    assert_eq!(empty.len(), 1);
    assert_eq!(empty[0].value, Value::Null);
    // A `---` at the end starts one more document, which is empty: the schemas reject it.
    let trailing = yaml("a: 1\n---\n").unwrap();
    assert_eq!(trailing.len(), 2);
    assert_eq!(trailing[1].value, Value::Null);
    let fault = rejected(yaml("a: 1\n---\nb: 1\nb: 2\n"), rules::SCH_012, "");
    assert_eq!(fault.document, Some(2));
}

#[test]
fn a_json_file_holds_one_document_or_an_array_of_them() {
    assert_eq!(json("[]").unwrap().len(), 0);
    assert_eq!(json(r#"[{"a": 1}, {"b": 2}]"#).unwrap().len(), 2);
    assert_eq!(json(r#"{"a": 1}"#).unwrap().len(), 1);
    // Anything else is one document, which the schemas then judge, as they judge a YAML
    // document that is not a map.
    assert_eq!(json(r#""text""#).unwrap()[0].value, string("text"));
    assert!(matches!(json("[[1]]").unwrap()[0].value, Value::Array(_)));
}

#[test]
fn text_that_is_not_yaml_or_json_is_rejected() {
    for text in ["a: b: c\n", "a: [1, 2\n", "a:\n\t- b\n", "\"a\n"] {
        rejected(yaml(text), rules::SUB_005, text);
    }
    let fault = rejected(yaml("a: 1\nb: c: d\n"), rules::SUB_005, "");
    assert_eq!(fault.position.map(|p| p.line), Some(2));
    for text in [
        "",
        "{",
        r#"{"a": 1,}"#,
        r#"{"a": 1} // note"#,
        "{'a': 1}",
        r#"{"a": 01}"#,
        "{} {}",
        "\u{feff}{}",
    ] {
        rejected(json(text), rules::SUB_005, text);
    }
    // A byte order mark is YAML's to allow, and the loader allows it there.
    assert_eq!(yaml_value("\u{feff}a: 1\n"), Value::Integer(1));
}

#[test]
fn text_that_is_not_utf8_is_rejected() {
    let fault = rejected(
        read_bytes(Syntax::Yaml, b"a: 1\nb: \xff\n"),
        rules::SUB_005,
        "not UTF-8",
    );
    assert_eq!(fault.position, Some(Position { line: 2, column: 4 }));
    read_bytes(Syntax::Json, b"{\"a\": \"\xc3\"}").expect_err("not UTF-8");
    // UTF-16 is YAML's to allow, and the loader reads UTF-8 alone.
    let utf16: Vec<u8> = "a: 1\n".encode_utf16().flat_map(u16::to_le_bytes).collect();
    read_bytes(Syntax::Yaml, &[[0xff, 0xfe].as_slice(), &utf16].concat()).expect_err("UTF-16");
}

#[test]
fn values_nest_no_deeper_than_the_limit() {
    let lists = |depth: usize| format!("{}{}", "[".repeat(depth), "]".repeat(depth));
    let maps = |depth: usize| format!("{}1{}", r#"{"a": "#.repeat(depth), "}".repeat(depth));
    assert!(yaml(&lists(MAX_DEPTH)).is_ok());
    assert!(json(&maps(MAX_DEPTH)).is_ok());
    rejected(yaml(&lists(MAX_DEPTH + 1)), rules::SUB_011, "YAML");
    rejected(json(&maps(MAX_DEPTH + 1)), rules::SUB_011, "JSON");
    // Block style, by indentation, is held to the same limit.
    let block = |depth: usize| {
        (0..depth)
            .map(|level| format!("{}a:\n", "  ".repeat(level)))
            .collect::<String>()
    };
    assert!(yaml(&block(MAX_DEPTH)).is_ok());
    rejected(yaml(&block(MAX_DEPTH + 1)), rules::SUB_011, "block YAML");
    // Far deeper input is rejected as a value, and does not exhaust the stack.
    rejected(yaml(&lists(100_000)), rules::SUB_011, "YAML");
    rejected(json(&maps(100_000)), rules::SUB_011, "JSON");
}

/// SCH-013: the JSON form and the YAML form of one document read as the same values.
#[test]
fn json_and_yaml_forms_of_one_document_are_equal() {
    let from_yaml =
        yaml("a: 15\nb: [x, 'y', 1.5, true, null]\nc:\n  d: {e: f}\n  g: \"\\u00e9\\U0001F600\"\n")
            .unwrap();
    let from_json = json(r#"{"c": {"g": "\u00e9\ud83d\ude00", "d": {"e": "f"}}, "b": ["x", "y", 1.5, true, null], "a": 15.0}"#)
        .unwrap();
    assert_eq!(from_yaml, from_json);
    // Lists keep their order; maps do not (SCH-007).
    assert_ne!(
        yaml("a: [1, 2]\n").unwrap(),
        json(r#"{"a": [2, 1]}"#).unwrap()
    );
}

/// YAML 1.2.2, section 5.1: a stream holds the printable characters, `c-printable`. A C0
/// control character other than tab, line feed and carriage return is allowed nowhere, and DEL,
/// a C1 control character other than NEL, U+FFFE and U+FFFF only inside a quoted scalar, which
/// takes any character but a C0 control (`nb-json`). The parser does not check this, so the
/// loader does.
#[test]
fn characters_that_yaml_does_not_allow_are_rejected() {
    for text in [
        "a: x\u{1}y\n",
        "a: \"x\u{1}y\"\n",
        "a: 'x\u{1b}y'\n",
        "\"a\u{1}\": x\n",
        "a: |\n  x\u{1}\n",
        "# a comment\u{1}\na: 1\n",
        "a: x\u{7f}y\n",
        "a: x\u{80}y\n",
        "a: x\u{9f}y\n",
        "a: x\u{fffe}y\n",
        "a: x\u{ffff}y\n",
        "a: |\n  x\u{7f}\n",
        "a: >\n  x\u{9f}\n",
        "x\u{7f}: 1\n",
        "# a comment\u{7f}\na: 1\n",
        "a: 1 # a comment\u{fffe}\n",
    ] {
        let fault = rejected(yaml(text), rules::SUB_005, text);
        assert!(
            fault.message.contains("does not allow"),
            "{text:?}: {fault}"
        );
    }
    let fault = rejected(yaml("a: 1\nb: x\u{7f}\n"), rules::SUB_005, "DEL");
    assert_eq!(fault.position, Some(Position { line: 2, column: 5 }));
    // Only a C0 control is excluded from a quoted scalar; tab, NEL and the line and paragraph
    // separators are content anywhere; and an escape writes any character.
    assert_eq!(
        yaml_value("a: \"x\u{7f}\u{80}\u{9f}\u{fffe}\u{ffff}y\"\n"),
        string("x\u{7f}\u{80}\u{9f}\u{fffe}\u{ffff}y")
    );
    assert_eq!(yaml_value("a: 'x\u{7f}'\n"), string("x\u{7f}"));
    assert_eq!(yaml_value("a: x\ty\n"), string("x\ty"));
    assert_eq!(yaml_value("a: x\u{85}y\n"), string("x\u{85}y"));
    assert_eq!(yaml_value("a: x\u{2028}y\n"), string("x\u{2028}y"));
    assert_eq!(yaml_value("a: \"\\x01\\x7f\"\n"), string("\u{1}\u{7f}"));
    // JSON allows the same in a string, and a C0 control only escaped (RFC 8259).
    assert_eq!(
        json_value("{\"a\": \"x\u{7f}\u{fffe}y\"}"),
        string("x\u{7f}\u{fffe}y")
    );
    rejected(
        json("{\"a\": \"x\u{1}y\"}"),
        rules::SUB_005,
        "a C0 control in JSON",
    );
}

/// The schema profile's row for I-JSON: of its rules only two apply, no duplicate keys and whole
/// numbers in range. A noncharacter may stand in a string. A surrogate that is not one of a pair
/// is not text, so the file is not YAML or JSON (SUB-005); a YAML escape writes one code point,
/// so a pair written as two escapes is two such surrogates, where JSON reads it as one
/// character.
#[test]
fn of_i_json_only_two_rules_apply_to_strings() {
    // A noncharacter, written out or escaped, in either syntax.
    for (text, value) in [
        ("a: x\u{fdd0}y\n", "x\u{fdd0}y"),
        ("a: \"x\u{fdd0}y\"\n", "x\u{fdd0}y"),
        (
            "a: \"\\uFDD0\\uFFFF\\U0010FFFF\"\n",
            "\u{fdd0}\u{ffff}\u{10ffff}",
        ),
        ("a: 'x\u{ffff}y'\n", "x\u{ffff}y"),
    ] {
        assert_eq!(yaml_value(text), string(value), "{text:?}");
    }
    for (text, value) in [
        ("{\"a\": \"x\u{fdd0}y\"}", "x\u{fdd0}y"),
        (
            r#"{"a": "\ufdd0\uffff\udbff\udfff"}"#,
            "\u{fdd0}\u{ffff}\u{10ffff}",
        ),
    ] {
        assert_eq!(json_value(text), string(value), "{text:?}");
    }
    // A surrogate that is not one of a pair, escaped, is not text.
    for text in [
        r#"{"a": "\ud800"}"#,
        r#"{"a": "\udc00"}"#,
        r#"{"a": "x\ud83dy"}"#,
        r#"{"a": "\ude00\ud83d"}"#,
        r#"{"\ud800": 1}"#,
    ] {
        rejected(json(text), rules::SUB_005, text);
    }
    for text in [r#"a: "\ud800""#, r#"a: "\udc00""#, r#""\ud800": 1"#] {
        rejected(yaml(text), rules::SUB_005, text);
    }
    // In YAML an escape writes one code point, so a pair written as two escapes is rejected;
    // the character is written with one escape. In JSON the pair is one character.
    rejected(
        yaml(r#"a: "\ud83d\ude00""#),
        rules::SUB_005,
        "an escaped pair in YAML",
    );
    assert_eq!(yaml_value(r#"a: "\U0001F600""#), string("\u{1f600}"));
    assert_eq!(json_value(r#"{"a": "\ud83d\ude00"}"#), string("\u{1f600}"));
    // Encoded in the file, a surrogate is not UTF-8.
    rejected(
        read_bytes(Syntax::Yaml, b"a: \"\xed\xa0\x80\"\n"),
        rules::SUB_005,
        "an encoded surrogate",
    );
}

/// SUB-001: the order of documents, within a file or across files, has no meaning. A fault in
/// a value does not end the reading of a file, so every fault of every document is reported,
/// in whatever order they are written.
#[test]
fn every_fault_of_a_file_is_reported_in_any_order() {
    let a = "kind: Interface\nmetadata:\n  name: a\n  name: b\n";
    let b = "kind: Interface\nspec:\n  type: t\n  type: u\n";
    let ab = faults_of(yaml(&format!("{a}---\n{b}")));
    let ba = faults_of(yaml(&format!("{b}---\n{a}")));
    let places = |faults: &[(String, Option<usize>, String)]| {
        let mut places: Vec<_> = faults
            .iter()
            .map(|(r, _, p)| (r.clone(), p.clone()))
            .collect();
        places.sort();
        places
    };
    assert_eq!(ab.len(), 2, "{ab:?}");
    assert_eq!(places(&ab), places(&ba));
    assert_eq!(
        ab,
        vec![
            ("SCH-012".to_string(), Some(1), "/metadata/name".to_string()),
            ("SCH-012".to_string(), Some(2), "/spec/type".to_string()),
        ]
    );
    // The same in a JSON array.
    let a = r#"{"metadata": {"name": "a", "name": "b"}}"#;
    let b = r#"{"spec": {"type": "t", "type": "u"}}"#;
    let ab = faults_of(json(&format!("[{a}, {b}]")));
    let ba = faults_of(json(&format!("[{b}, {a}]")));
    assert_eq!(ab.len(), 2, "{ab:?}");
    assert_eq!(places(&ab), places(&ba));
    // And the members of one map: the value of a duplicate key is read too.
    for text in ["a: 0x1F\na: 1\n", "a: 1\na: 0x1F\n"] {
        assert_eq!(
            places(&faults_of(yaml(text))),
            vec![
                ("SCH-012".to_string(), "/a".to_string()),
                ("SCH-017".to_string(), "/a".to_string()),
            ],
            "{text:?}"
        );
    }
    // Text that is not YAML is rejected for that alone, wherever it is.
    for text in ["a: 1\na: 2\n---\nb: [\n", "b: [\n---\na: 1\na: 2\n"] {
        rejected(yaml(text), rules::SUB_005, text);
    }
}

/// SCH-013: the JSON form and the YAML form of one document give the same diagnostics. A
/// fault found when a file is read names its rule, its document and its place as a JSON
/// Pointer, alike in either syntax.
#[test]
fn a_fault_has_the_same_rule_and_place_in_yaml_and_json() {
    let largest = format!("{:.0}", f64::MAX);
    let nested = |depth: usize| format!("{}{}", "[".repeat(depth), "]".repeat(depth));
    for (from_yaml, from_json) in [
        (
            "x:\n  b: 1\n  b: 2\n".to_string(),
            r#"{"x": {"b": 1, "b": 2}}"#.to_string(),
        ),
        (
            "a: [1, 9007199254740992]\n".to_string(),
            r#"{"a": [1, 9007199254740992]}"#.to_string(),
        ),
        (
            "a: {b~c/d: 1e400}\n".to_string(),
            r#"{"a": {"b~c/d": 1e400}}"#.to_string(),
        ),
        (
            format!("a: [{largest}.5]\n"),
            format!("{{\"a\": [{largest}.5]}}"),
        ),
        (
            "a: 1\n---\nb: [1, 1e400]\nc: {d: 1, d: 2}\n".to_string(),
            r#"[{"a": 1}, {"b": [1, 1e400], "c": {"d": 1, "d": 2}}]"#.to_string(),
        ),
        (
            format!("a: {}\n", nested(MAX_DEPTH)),
            format!("{{\"a\": {}}}", nested(MAX_DEPTH)),
        ),
    ] {
        let expected = faults_of(json(&from_json));
        assert!(
            expected.iter().all(|(_, _, p)| !p.is_empty()),
            "{expected:?}"
        );
        assert_eq!(faults_of(yaml(&from_yaml)), expected, "{from_yaml:?}");
    }
}

#[test]
fn a_reserved_directive_is_ignored() {
    // A directive whose name only begins with YAML is reserved, and YAML 1.2.2 (section 6.8)
    // has a reader ignore it; `01.2` is version 1.2.
    for text in [
        "%YAMLX 2.0\n---\na: 1\n",
        "%YAML-EXT 1\n---\na: 1\n",
        "%YAML 01.2\n---\na: 1\n",
    ] {
        assert_eq!(yaml_value(text), Value::Integer(1), "{text:?}");
    }
    let fault = rejected(
        yaml("# a comment\n%YAML 2.0\n---\na: 1\n"),
        rules::SUB_005,
        "",
    );
    assert_eq!(fault.position, Some(Position { line: 2, column: 1 }));
}

/// YAML 1.2.2, `c-forbidden`: a line that begins with `---` or `...` marks a document, and is
/// never content. The parser reads one into a block scalar at the root, so the loader rejects
/// such a text rather than read it otherwise than YAML does.
#[test]
fn a_document_marker_is_not_read_as_content() {
    for text in ["--- |\nx\n---\nb\n", "--- >\nx\n---\n"] {
        rejected(yaml(text), rules::SUB_005, text);
    }
    // The parser ends the scalar at `...`, as YAML does.
    assert_eq!(yaml("--- |\nx\n...\n").unwrap()[0].value, string("x\n"));
    // Indented, or followed by another character, it is content.
    assert_eq!(
        yaml("--- |\n  x\n  ---\n").unwrap()[0].value,
        string("x\n---\n")
    );
    assert_eq!(
        yaml("--- |\nx\n----\n").unwrap()[0].value,
        string("x\n----\n")
    );
    assert_eq!(yaml_value("a: |\n  x\n  ...\n"), string("x\n...\n"));
}

#[test]
fn a_place_in_json_is_counted_in_characters() {
    let fault = rejected(
        json(r#"{"é": 1, "ü": 2,}"#),
        rules::SUB_005,
        "a trailing comma",
    );
    assert_eq!(
        fault.position,
        Some(Position {
            line: 1,
            column: 17
        })
    );
    let fault = rejected(json("{\"a\": 1,\n\"é\": x}"), rules::SUB_005, "x");
    assert_eq!(fault.position, Some(Position { line: 2, column: 6 }));
    let fault = rejected(
        json(r#"{"é": 1, "é": 2}"#),
        rules::SCH_012,
        "a duplicate key",
    );
    assert_eq!(
        fault.position,
        Some(Position {
            line: 1,
            column: 15
        })
    );
}

/// Reading takes time in proportion to the size of a file, in either syntax: a file of some
/// megabytes is read in well under a second when built with optimizations, where a reader
/// whose time grew with the square of the size would take minutes.
#[test]
fn a_large_file_is_read_in_time_proportional_to_its_size() {
    let bound = std::time::Duration::from_secs(20);
    let items: Vec<String> = (0..40_000)
        .map(|i| format!("{{\"name\": \"item-{i}\", \"value\": {i}, \"list\": [1, 2.5, \"é\"]}}"))
        .collect();
    let text = format!("[{}]", items.join(",\n"));
    assert!(text.len() > 2_000_000);
    let start = std::time::Instant::now();
    assert_eq!(json(&text).unwrap().len(), 40_000);
    assert!(start.elapsed() < bound, "JSON: {:?}", start.elapsed());
    let one_line = text.replace('\n', "");
    let start = std::time::Instant::now();
    assert_eq!(json(&one_line).unwrap().len(), 40_000);
    assert!(
        start.elapsed() < bound,
        "JSON on one line: {:?}",
        start.elapsed()
    );
    let text = "---\nname: é\nvalue: 1\n".repeat(60_000);
    let start = std::time::Instant::now();
    assert_eq!(yaml(&text).unwrap().len(), 60_000);
    assert!(start.elapsed() < bound, "YAML: {:?}", start.elapsed());
}
