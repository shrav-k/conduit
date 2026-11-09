//! Reading a number from the text that writes it, in YAML or in JSON.
//!
//! Whether a number has a fractional part is decided by the decimal value its text writes,
//! not by the binary64 value nearest to it, as `tools/check_schemas.py` decides it: `1.5e1` has
//! none, and `9007199254740993.5` has one although its nearest binary64 value is whole.
//!
//! - A number with no fractional part is an integer however it is written (SCH-005), and one
//!   outside the range of I-JSON is rejected (SCH-006).
//! - A number with a fractional part is the binary64 value nearest to it (SCH-020), and one
//!   whose magnitude is greater than the largest finite binary64 value is rejected (SCH-021).
//!   That is judged on the decimal value: a number a little above the largest binary64 value
//!   that would round down to it is still rejected.

use crate::value::{LARGEST_WHOLE, Value};

/// Why a number is rejected.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum NumberFault {
    /// No fractional part, and outside the range of SCH-006.
    WholeOutOfRange,
    /// A fractional part, and outside the range of binary64 (SCH-021).
    OutsideBinary64,
}

/// Read a number from its text. The text has the form, which the caller has checked,
/// `[-+]?(\.[0-9]+|[0-9]+(\.[0-9]*)?)([eE][-+]?[0-9]+)?`: YAML 1.2's core schema writes an
/// integer and a float in it, and JSON's grammar (RFC 8259) is narrower.
pub(crate) fn read_number(text: &str) -> Result<Value, NumberFault> {
    let (negative, unsigned) = match text.as_bytes().first() {
        Some(b'-') => (true, &text[1..]),
        Some(b'+') => (false, &text[1..]),
        _ => (false, text),
    };
    let (mantissa, exponent) = match unsigned.find(['e', 'E']) {
        Some(at) => (&unsigned[..at], exponent_of(&unsigned[at + 1..])),
        None => (unsigned, 0),
    };
    let (whole_part, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let digits = format!("{whole_part}{fraction}");
    let digits = digits.trim_start_matches('0');
    if digits.is_empty() {
        return Ok(Value::Integer(0));
    }
    let significant = digits.trim_end_matches('0');
    let trailing = (digits.len() - significant.len()) as i64;
    // The value is `significant` times ten to the power `scale`.
    let scale = exponent - fraction.len() as i64 + trailing;
    if scale >= 0 {
        // No fractional part: an integer, within 2^53 - 1, which has 16 digits.
        if significant.len() as i64 + scale > 16 {
            return Err(NumberFault::WholeOutOfRange);
        }
        let mut value: u64 = significant
            .parse()
            .map_err(|_| NumberFault::WholeOutOfRange)?;
        for _ in 0..scale {
            value *= 10;
        }
        if value > LARGEST_WHOLE {
            return Err(NumberFault::WholeOutOfRange);
        }
        let value = value as i64;
        return Ok(Value::Integer(if negative { -value } else { value }));
    }
    // A fractional part. Its whole part, compared with the largest finite binary64 value,
    // decides SCH-021: the number is greater than that value exactly when its whole part is at
    // least that value, since the fraction is not zero.
    let whole_digits = significant.len() as i64 + scale;
    if whole_digits >= 309 {
        let largest = format!("{:.0}", f64::MAX);
        if whole_digits > largest.len() as i64 || &significant[..largest.len()] >= largest.as_str()
        {
            return Err(NumberFault::OutsideBinary64);
        }
    }
    let written = format!(
        "{}{}.{}e{}",
        if negative { "-" } else { "" },
        if whole_part.is_empty() {
            "0"
        } else {
            whole_part
        },
        if fraction.is_empty() { "0" } else { fraction },
        exponent
    );
    match written.parse::<f64>() {
        Ok(value) if value.is_finite() => Ok(Value::Float(value)),
        _ => Err(NumberFault::OutsideBinary64),
    }
}

/// The exponent, saturated far beyond any that can matter, so that no text overflows it.
fn exponent_of(text: &str) -> i64 {
    const LIMIT: i64 = 1 << 40;
    let (negative, digits) = match text.as_bytes().first() {
        Some(b'-') => (true, &text[1..]),
        Some(b'+') => (false, &text[1..]),
        _ => (false, text),
    };
    let mut value: i64 = 0;
    for digit in digits.bytes().filter(u8::is_ascii_digit) {
        value = (value * 10 + i64::from(digit - b'0')).min(LIMIT);
    }
    if negative { -value } else { value }
}

/// Whether a text has the form of a number that [`read_number`] reads: YAML 1.2's core schema
/// writes an integer `[-+]?[0-9]+` and a float
/// `[-+]?(\.[0-9]+|[0-9]+(\.[0-9]*)?)([eE][-+]?[0-9]+)?` (YAML 1.2.2, section 10.3.2).
pub(crate) fn is_decimal(text: &str) -> bool {
    let bytes = text.as_bytes();
    let mut at = usize::from(matches!(bytes.first(), Some(b'-' | b'+')));
    let digits = |from: usize| {
        bytes[from..]
            .iter()
            .take_while(|b| b.is_ascii_digit())
            .count()
    };
    let whole = digits(at);
    at += whole;
    let mut fraction = 0;
    if bytes.get(at) == Some(&b'.') {
        fraction = digits(at + 1);
        at += 1 + fraction;
    }
    // `.5` and `5.` and `5.5` are each a number; `.` alone is not.
    if whole == 0 && fraction == 0 {
        return false;
    }
    if matches!(bytes.get(at), Some(b'e' | b'E')) {
        at += 1;
        if matches!(bytes.get(at), Some(b'-' | b'+')) {
            at += 1;
        }
        let exponent = digits(at);
        if exponent == 0 {
            return false;
        }
        at += exponent;
    }
    at == bytes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn integer(text: &str) -> i64 {
        match read_number(text) {
            Ok(Value::Integer(i)) => i,
            other => panic!("{text}: {other:?}"),
        }
    }

    fn float(text: &str) -> f64 {
        match read_number(text) {
            Ok(Value::Float(f)) => f,
            other => panic!("{text}: {other:?}"),
        }
    }

    #[test]
    fn a_number_with_no_fractional_part_is_an_integer_however_written() {
        assert_eq!(integer("15"), 15);
        assert_eq!(integer("15.0"), 15);
        assert_eq!(integer("1.5e1"), 15);
        assert_eq!(integer("150e-1"), 15);
        assert_eq!(integer("+15"), 15);
        assert_eq!(integer("-0"), 0);
        assert_eq!(integer("0.0"), 0);
        assert_eq!(integer("007"), 7);
        assert_eq!(integer("9007199254740991"), 9_007_199_254_740_991);
        assert_eq!(integer("-9007199254740991"), -9_007_199_254_740_991);
        assert_eq!(integer("9.007199254740991e15"), 9_007_199_254_740_991);
    }

    #[test]
    fn a_whole_number_outside_the_range_is_rejected() {
        for text in [
            "9007199254740992",
            "-9007199254740992",
            "1e16",
            "1e400",
            "1.5e300",
        ] {
            assert_eq!(
                read_number(text),
                Err(NumberFault::WholeOutOfRange),
                "{text}"
            );
        }
        assert_eq!(
            read_number(&format!("1{}", "0".repeat(400))),
            Err(NumberFault::WholeOutOfRange)
        );
    }

    #[test]
    fn a_number_with_a_fractional_part_is_binary64() {
        assert_eq!(float("0.7"), 0.7);
        assert_eq!(float("15.5"), 15.5);
        assert_eq!(float(".5"), 0.5);
        assert_eq!(float("-1.25e-1"), -0.125);
        assert_eq!(float("1e-400"), 0.0);
        // Written with a fraction; its nearest binary64 value is whole, and it stays a float.
        assert_eq!(float("9007199254740993.5"), 9_007_199_254_740_994.0);
    }

    #[test]
    fn a_number_with_a_fractional_part_beyond_binary64_is_rejected() {
        let largest = format!("{:.0}", f64::MAX);
        // The largest value with a fraction beside it is greater than the largest value.
        assert_eq!(
            read_number(&format!("{largest}.5")),
            Err(NumberFault::OutsideBinary64)
        );
        assert_eq!(
            read_number(&format!("{largest}0.5")),
            Err(NumberFault::OutsideBinary64)
        );
        // One less, with a fraction, is not.
        let mut below = largest.clone().into_bytes();
        *below.last_mut().unwrap() -= 1;
        let below = String::from_utf8(below).unwrap();
        assert!(matches!(
            read_number(&format!("{below}.5")),
            Ok(Value::Float(_))
        ));
    }

    #[test]
    fn the_form_of_a_number() {
        for text in [
            "1", "-1", "+1", "1.", ".5", "1.5", "1e5", "1.e5", "-.5E-3", "007",
        ] {
            assert!(is_decimal(text), "{text}");
        }
        for text in [
            "", ".", "-", "e5", "1e", "1e+", "0x1F", "0o17", ".inf", "1_000", "1.5.5", " 1",
        ] {
            assert!(!is_decimal(text), "{text}");
        }
    }
}
