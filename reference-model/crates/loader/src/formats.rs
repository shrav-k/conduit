//! What the validator is given beyond the crate's defaults: the grammars of `uuid` and
//! `duration` that SCH-004 cites, and `pattern` evaluated as ECMA-262 with the `u` flag
//! (SCH-016).

use std::collections::HashSet;

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
/// The quoted letters of ABNF match either case (RFC 5234, section 2.3), as for [`is_uuid`],
/// so `p1d` matches; the `jsonschema` crate's own check takes only capitals. A digit is ASCII,
/// and a fraction, a sign and a week beside another unit are not in the grammar. The first
/// build's log of increment 1 records the case of the letters as a finding.
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

/// A regular expression of ECMA-262, in Unicode mode, which is the `u` flag (SCH-016), in the
/// edition that JSON Schema 2020-12 cites, the eleventh, of 2020. `regress` implements the
/// current edition, and accepts two things that the eleventh does not, which are rejected here:
/// a group that sets flags, such as `(?i:a)`, and two groups of one name, such as
/// `(?<a>x)|(?<a>y)`, both of the edition of 2025. Nothing else that the later editions added
/// is syntax of a pattern; they added flags, and a pattern here takes the `u` flag alone.
pub fn ecma_regex(pattern: &str) -> Result<regress::Regex, String> {
    let regex = regress::Regex::with_flags(pattern, "u").map_err(|error| error.to_string())?;
    match later_syntax(pattern) {
        Some(why) => Err(why.to_string()),
        None => Ok(regex),
    }
}

/// Syntax of a pattern, which `regress` has accepted, that ECMA-262 added after the edition
/// of 2020: a group that sets flags, and a name given to two groups.
fn later_syntax(pattern: &str) -> Option<&'static str> {
    let chars: Vec<char> = pattern.chars().collect();
    let mut names = HashSet::new();
    let mut in_class = false;
    let mut at = 0;
    while at < chars.len() {
        match chars[at] {
            '\\' => at += 1,
            '[' => in_class = true,
            ']' => in_class = false,
            '(' if !in_class && chars.get(at + 1) == Some(&'?') => match chars.get(at + 2) {
                Some(':' | '=' | '!') => {}
                Some('<') if matches!(chars.get(at + 3), Some('=' | '!')) => {}
                Some('<') => {
                    let name: String = chars[at + 3..].iter().take_while(|c| **c != '>').collect();
                    if !names.insert(group_name(&name)) {
                        return Some(
                            "a name given to two groups, which ECMA-262 allows only from its \
                             edition of 2025",
                        );
                    }
                }
                _ => {
                    return Some(
                        "a group that sets flags, which ECMA-262 has only from its edition of \
                         2025",
                    );
                }
            },
            _ => {}
        }
        at += 1;
    }
    None
}

/// The name of a group as ECMA-262 compares it, with each `\uXXXX` and `\u{X}` escape read
/// as the character it writes, and a pair of escaped surrogates as one character.
fn group_name(written: &str) -> String {
    let mut units: Vec<u32> = Vec::new();
    let mut rest = written;
    while !rest.is_empty() {
        if let Some(after) = rest.strip_prefix("\\u{")
            && let Some(end) = after.find('}')
            && let Ok(code) = u32::from_str_radix(&after[..end], 16)
        {
            units.push(code);
            rest = &after[end + 1..];
        } else if let Some(after) = rest.strip_prefix("\\u")
            && let Some(hex) = after.get(..4)
            && let Ok(code) = u32::from_str_radix(hex, 16)
        {
            units.push(code);
            rest = &after[4..];
        } else {
            let c = rest.chars().next().unwrap_or_default();
            units.push(u32::from(c));
            rest = &rest[c.len_utf8()..];
        }
    }
    let mut name = String::new();
    let mut index = 0;
    while index < units.len() {
        let unit = units[index];
        let next = units.get(index + 1).copied().unwrap_or(0);
        if (0xD800..0xDC00).contains(&unit) && (0xDC00..0xE000).contains(&next) {
            let code = 0x10000 + ((unit - 0xD800) << 10) + (next - 0xDC00);
            name.extend(char::from_u32(code));
            index += 2;
        } else {
            name.push(char::from_u32(unit).unwrap_or('\u{fffd}'));
            index += 1;
        }
    }
    name
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
/// 3986, the grammars the profile cites. `patternProperties`
/// is still matched by the crate's own engine, which cannot be replaced; no schema of the
/// specification uses it, and the first build's log of increment 1 records what remains.
pub fn options() -> jsonschema::ValidationOptions<'static> {
    jsonschema::draft202012::options()
        .should_validate_formats(true)
        .should_ignore_unknown_formats(false)
        .with_format("uuid", is_uuid)
        .with_format("duration", is_duration)
        .with_keyword("pattern", pattern_keyword)
}
