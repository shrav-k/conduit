use provider_ssh_runtime::codec;
use std::collections::BTreeMap;

/// Independent EnvironmentFile reader: unquoted whitespace is stripped, single quotes
/// are literal, double quotes use POSIX backslash rules, and escaped newlines join lines.
fn decode(text: &str) -> BTreeMap<String, String> {
    #[derive(PartialEq)]
    enum Mode {
        Bare,
        Single,
        Double,
    }
    let mut lines = text.chars().peekable();
    let mut result = BTreeMap::new();
    while lines.peek().is_some() {
        while lines.peek().is_some_and(|c| c.is_whitespace()) {
            lines.next();
        }
        if lines.peek().is_some_and(|c| matches!(c, '#' | ';')) {
            for c in lines.by_ref() {
                if c == '\n' {
                    break;
                }
            }
            continue;
        }
        let mut name = String::new();
        let mut assignment = false;
        for c in lines.by_ref() {
            if c == '=' {
                assignment = true;
                break;
            }
            if c == '\n' {
                break;
            }
            name.push(c);
        }
        if !assignment {
            continue;
        }
        let mut mode = Mode::Bare;
        let mut value = String::new();
        let mut trailing = String::new();
        let mut begun = false;
        while let Some(c) = lines.next() {
            if mode == Mode::Bare && c == '\n' {
                break;
            }
            if mode == Mode::Single {
                if c == '\'' {
                    mode = Mode::Bare;
                } else {
                    value.push(c);
                }
                continue;
            }
            if mode == Mode::Double {
                if c == '"' {
                    mode = Mode::Bare;
                    continue;
                }
                if c == '\\' {
                    let n = lines.next().unwrap();
                    match n {
                        '\n' => (),
                        '\\' | '"' | '$' | '`' => value.push(n),
                        _ => {
                            value.push('\\');
                            value.push(n);
                        }
                    }
                } else {
                    value.push(c);
                }
                continue;
            }
            if matches!(c, ' ' | '\t' | '\r') {
                if begun {
                    trailing.push(c);
                }
                continue;
            }
            value.push_str(&std::mem::take(&mut trailing));
            begun = true;
            match c {
                '\'' => mode = Mode::Single,
                '"' => mode = Mode::Double,
                '\\' => {
                    let n = lines.next().unwrap();
                    if n != '\n' {
                        value.push(n);
                    }
                }
                _ => value.push(c),
            }
        }
        assert!(mode == Mode::Bare);
        result.insert(name.trim().into(), value);
    }
    result
}
#[test]
fn environment_file_round_trip_and_reference_syntax() {
    let reference = "# comment\n ; comment\nA=  unquoted tail  \nB=' single\nquoted '\nC=\" both ' \\\" \\\\ \\$ \\` \\q\"\nD=joined\\\nline\n";
    let parsed = decode(reference);
    assert_eq!(parsed["A"], "unquoted tail");
    assert_eq!(parsed["B"], " single\nquoted ");
    assert_eq!(parsed["C"], " both ' \" \\ $ ` \\q");
    assert_eq!(parsed["D"], "joinedline");
    let values: BTreeMap<String, String> = [
        "",
        "\n",
        "\"",
        "'",
        "\\",
        "$",
        " leading",
        "trailing ",
        "\tboth\t",
        "one\n'\"\\$`%\ntwo",
        "caf\u{e9} 🦀",
        "x\r\ny",
    ]
    .into_iter()
    .enumerate()
    .map(|(i, v)| (format!("VALUE_{i}"), v.into()))
    .collect();
    let encoded = codec::environment(&values).unwrap();
    assert_eq!(decode(std::str::from_utf8(&encoded).unwrap()), values);
    let value = (1u8..=127).map(char::from).collect::<String>();
    let map = BTreeMap::from([("ALL_ASCII".into(), value)]);
    assert_eq!(
        decode(std::str::from_utf8(&codec::environment(&map).unwrap()).unwrap()),
        map
    );
}
#[test]
fn environment_names_and_unrepresentable_values_are_refused_by_name() {
    for name in [
        "",
        "1NAME",
        "BAD-NAME",
        "WITH SPACE",
        "NONASCII_é",
        "A=B",
        "A\nB",
    ] {
        assert_eq!(
            codec::environment(&BTreeMap::from([(name.into(), "synthetic-value".into())]))
                .unwrap_err(),
            name
        );
    }
    for value in [
        "nul\0value",
        "\u{feff}",
        "\u{fdd0}",
        "\u{ffff}",
        "\u{10ffff}",
    ] {
        assert_eq!(
            codec::environment(&BTreeMap::from([("TEST_VALUE".into(), value.into())])).unwrap_err(),
            "TEST_VALUE"
        );
    }
    assert!(codec::environment(&BTreeMap::from([("_valid123".into(), "yes".into())])).is_ok());
}
