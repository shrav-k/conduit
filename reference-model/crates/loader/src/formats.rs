//! What the validator is given beyond the crate's defaults: the grammars of `uuid` and
//! `duration` that SCH-004 cites, and `pattern` evaluated as ECMA-262 with the `u` flag
//! (SCH-016).

use jsonschema::{Keyword, ValidationError, paths::Location};
use serde_json::{Map, Value};

/// Whether a string matches the grammar of a UUID in RFC 9562, section 4:
///
/// ```text
/// UUID     = 4hexOctet "-" 2hexOctet "-" 2hexOctet "-" 2hexOctet "-" 6hexOctet
/// hexOctet = HEXDIG HEXDIG
/// HEXDIG   = DIGIT / "A" / "B" / "C" / "D" / "E" / "F"
/// ```
///
/// The quoted letters of ABNF match either case (RFC 5234, section 2.3), and RFC 9562 says the
/// letters are case-insensitive on input. The grammar fixes no version and no variant: the Nil
/// UUID and the Max UUID, which RFC 9562 defines, match it, and so does a UUID whose variant
/// field is one that RFC 9562 reserves. JSON Schema 2020-12 cites RFC 4122 for this format;
/// SCH-004 has the grammar of RFC 9562 checked, and the two grammars are the same.
pub fn is_uuid(text: &str) -> bool {
    let bytes = text.as_bytes();
    bytes.len() == 36
        && bytes.iter().enumerate().all(|(at, byte)| match at {
            8 | 13 | 18 | 23 => *byte == b'-',
            _ => byte.is_ascii_hexdigit(),
        })
}

/// Whether a string matches the grammar of a duration in RFC 3339, appendix A:
///
/// ```text
/// dur-second = 1*DIGIT "S"
/// dur-minute = 1*DIGIT "M" [dur-second]
/// dur-hour   = 1*DIGIT "H" [dur-minute]
/// dur-time   = "T" (dur-hour / dur-minute / dur-second)
/// dur-day    = 1*DIGIT "D"
/// dur-week   = 1*DIGIT "W"
/// dur-month  = 1*DIGIT "M" [dur-day]
/// dur-year   = 1*DIGIT "Y" [dur-month]
/// dur-date   = (dur-day / dur-month / dur-year) [dur-time]
/// duration   = "P" (dur-date / dur-time / dur-week)
/// ```
///
/// The quoted letters of ABNF match either case (RFC 5234, section 2.3), as for [`is_uuid`]
/// and as the schema profile says, so `p1d` matches; the `jsonschema` crate's own check takes
/// only capitals. A digit is ASCII, and a fraction, a sign and a week beside another unit are
/// not in the grammar.
pub fn is_duration(text: &str) -> bool {
    /// The designators of a run of units, each after one or more digits, or none where the
    /// run is not of that form.
    fn units(text: &[u8]) -> Option<Vec<u8>> {
        let mut designators = Vec::new();
        let mut digits = 0;
        for byte in text {
            if byte.is_ascii_digit() {
                digits += 1;
            } else if digits > 0 && byte.is_ascii_alphabetic() {
                designators.push(byte.to_ascii_uppercase());
                digits = 0;
            } else {
                return None;
            }
        }
        (digits == 0).then_some(designators)
    }
    let bytes = text.as_bytes();
    let Some(rest) = bytes
        .strip_prefix(b"P")
        .or_else(|| bytes.strip_prefix(b"p"))
    else {
        return false;
    };
    let (date, time) = match rest.iter().position(|b| b.eq_ignore_ascii_case(&b'T')) {
        Some(at) => (&rest[..at], Some(&rest[at + 1..])),
        None => (rest, None),
    };
    let (Some(date), time) = (units(date), time.map(units)) else {
        return false;
    };
    let date_ok = matches!(
        date.as_slice(),
        b"" | b"D" | b"M" | b"MD" | b"Y" | b"YM" | b"YMD"
    );
    match time {
        None => date.as_slice() == b"W" || (date_ok && !date.is_empty()),
        Some(None) => false,
        Some(Some(time)) => {
            date_ok && matches!(time.as_slice(), b"H" | b"HM" | b"HMS" | b"M" | b"MS" | b"S")
        }
    }
}

/// A pattern using the interoperable tokens of JSON Schema 2020-12, core, section 6.4,
/// plus escaped pattern punctuation, read as ECMA-262 in Unicode mode (SCH-016).
pub fn ecma_regex(pattern: &str) -> Result<regress::Regex, String> {
    interoperable_tokens(pattern)?;
    regress::Regex::with_flags(pattern, "u").map_err(|error| error.to_string())
}

/// Tokenize before compiling: constructs accepted by a regex library but outside SCH-016
/// must not enter an authored schema. The Python checker uses the same token list.
fn interoperable_tokens(pattern: &str) -> Result<(), String> {
    let chars: Vec<char> = pattern.chars().collect();
    let mut at = 0;
    let mut can_quantify = false;
    while at < chars.len() {
        let c = chars[at];
        match c {
            '\\' => {
                at += 1;
                if !chars
                    .get(at)
                    .is_some_and(|c| "^$\\.*+?()[]{}|".contains(*c))
                {
                    return Err("only escaped pattern punctuation is allowed".into());
                }
                can_quantify = true;
            }
            '[' => {
                at += 1;
                if chars.get(at) == Some(&'^') {
                    at += 1;
                }
                while at < chars.len() && chars[at] != ']' {
                    if chars[at] == '\\' {
                        at += 1;
                        if !chars
                            .get(at)
                            .is_some_and(|c| "^$\\.*+?()[]{}|-".contains(*c))
                        {
                            return Err(
                                "only escaped pattern punctuation is allowed in a class".into()
                            );
                        }
                    }
                    at += 1;
                }
                if chars.get(at) != Some(&']') {
                    return Err("an unclosed character class".into());
                }
                can_quantify = true;
            }
            '(' => {
                if chars.get(at + 1) == Some(&'?') {
                    return Err("only simple grouping is allowed".into());
                }
                can_quantify = false;
            }
            '|' | '^' | '$' => can_quantify = false,
            '*' | '+' | '?' | '{' => {
                if !can_quantify {
                    return Err("a quantifier without an atom".into());
                }
                if c == '{' {
                    at += 1;
                    let begin = at;
                    while chars.get(at).is_some_and(char::is_ascii_digit) {
                        at += 1;
                    }
                    if at == begin {
                        return Err("a range quantifier needs a decimal count".into());
                    }
                    if chars.get(at) == Some(&',') {
                        at += 1;
                        while chars.get(at).is_some_and(char::is_ascii_digit) {
                            at += 1;
                        }
                    }
                    if chars.get(at) != Some(&'}') {
                        return Err("an unclosed range quantifier".into());
                    }
                }
                if chars.get(at + 1) == Some(&'?') {
                    at += 1;
                }
                can_quantify = false;
            }
            '.' => return Err("the wildcard is outside the interoperable tokens".into()),
            _ => can_quantify = true,
        }
        at += 1;
    }
    Ok(())
}

/// The keyword `pattern`, evaluated by an ECMA-262 engine. It takes the place of the crate's
/// own `pattern`, which translates a pattern into the syntax of another engine as far as it
/// can, and passes one with a lookaround to that engine untranslated.
struct EcmaPattern {
    pattern: String,
    regex: regress::Regex,
}

impl<'i> Keyword<'i> for EcmaPattern {
    fn validate(&self, instance: &'i Value) -> Result<(), ValidationError<'i>> {
        if self.is_valid(instance) {
            Ok(())
        } else {
            Err(ValidationError::custom(format!(
                "{instance} does not match the pattern '{}'",
                self.pattern
            )))
        }
    }

    fn is_valid(&self, instance: &'i Value) -> bool {
        // A pattern is not anchored: it is valid where the expression matches anywhere.
        match instance {
            Value::String(text) => self.regex.find(text).is_some(),
            _ => true,
        }
    }
}

/// The prefix of the message of a schema that holds a `pattern` an ECMA-262 engine rejects.
pub(crate) const NOT_ECMA: &str = "not a regular expression of ECMA-262";

fn pattern_keyword<'a>(
    _parent: &'a Map<String, Value>,
    value: &'a Value,
    _location: Location,
) -> Result<Box<dyn for<'i> Keyword<'i>>, ValidationError<'a>> {
    let pattern = value
        .as_str()
        .ok_or_else(|| ValidationError::custom("`pattern` is not a string"))?;
    let regex = ecma_regex(pattern)
        .map_err(|why| ValidationError::custom(format!("{NOT_ECMA}: '{pattern}': {why}")))?;
    Ok(Box::new(EcmaPattern {
        pattern: pattern.to_string(),
        regex,
    }))
}

/// The validator's options: JSON Schema 2020-12 (SCH-001), every format the profile lists
/// asserted and not only annotated (SCH-001, SCH-004), `uuid` by [`is_uuid`], `duration` by
/// [`is_duration`], a format the crate does not know rejected rather than ignored, and
/// `pattern` as ECMA-262 (SCH-016).
///
/// The crate checks `date-time` and `date` by RFC 3339, and `uri` and `uri-reference` by RFC
/// 3986, the grammars the profile cites. `patternProperties` is still matched by the crate's
/// own engine, which cannot be replaced; no schema of the specification uses it, and the core
/// matches it itself when it validates the schemas that authors write, from increment 2, as
/// the first build's log of increment 1 records.
pub fn options() -> jsonschema::ValidationOptions<'static> {
    jsonschema::draft202012::options()
        .should_validate_formats(true)
        .should_ignore_unknown_formats(false)
        .with_format("uuid", is_uuid)
        .with_format("duration", is_duration)
        .with_keyword("pattern", pattern_keyword)
}
