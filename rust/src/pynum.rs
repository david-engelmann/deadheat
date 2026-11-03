//! Numbers held the way the Python reading holds them.
//!
//! The Python JSON reading classifies a number from its raw
//! literal. An integer literal becomes an arbitrary precision
//! integer, and a literal with a fraction or an exponent becomes
//! a double. The loaders depend on that classification, a token
//! count spelled 5.0 is not an integer and an integer too large
//! for a finite double is not a finite number, so the port holds
//! the same two shapes in the Num type and never lets one become
//! the other.

use num_bigint::BigInt;
use num_rational::Ratio;
use std::cmp::Ordering;

/// A recorded number, an exact integer or a double, as the
/// Python reading classified its literal.
#[derive(Clone, Debug)]
pub enum Num {
    Int(BigInt),
    Float(f64),
}

/// The bound an integer must sit under in absolute value to
/// convert to a finite double, probed against the Python reading.
/// The largest finite double is 2^1024 - 2^971, the midpoint
/// between it and 2^1024 is 2^1024 - 2^970, and an integer at or
/// past that midpoint rounds to infinity, which the Python
/// conversion refuses with an OverflowError the finite test
/// reads as not finite.
fn finite_int_bound() -> BigInt {
    (BigInt::from(1) << 1024) - (BigInt::from(1) << 970)
}

impl Num {
    pub fn from_i64(value: i64) -> Num {
        Num::Int(BigInt::from(value))
    }

    pub fn from_f64(value: f64) -> Num {
        Num::Float(value)
    }

    /// Return whether this number is an integer, the test the
    /// loaders apply to the token counts, the standings counts,
    /// and the wins target.
    pub fn is_int(&self) -> bool {
        matches!(self, Num::Int(_))
    }

    /// Return whether this number is finite, without ever
    /// failing. An integer too large to convert to a double
    /// counts as not finite, so the loaders name it with the
    /// finite number defect instead of failing on it.
    pub fn is_finite_number(&self) -> bool {
        match self {
            Num::Int(value) => {
                let bound = finite_int_bound();
                *value < bound && *value > -bound
            }
            Num::Float(value) => value.is_finite(),
        }
    }

    /// Return the exact fraction this number names. A double
    /// converts to the fraction of its exact binary value and an
    /// integer to itself, so comparisons built on this never
    /// round. Every caller passes a finite number, as the Python
    /// code does, because the loaders refuse non finite figures
    /// before any reading reaches this.
    pub fn to_fraction(&self) -> Ratio<BigInt> {
        match self {
            Num::Int(value) => Ratio::from(value.clone()),
            Num::Float(value) => float_to_fraction(*value),
        }
    }

    /// Return whether this number is below zero, compared
    /// exactly, as the Python comparisons compare.
    pub fn is_negative(&self) -> bool {
        match self {
            Num::Int(value) => *value < BigInt::from(0),
            Num::Float(value) => *value < 0.0,
        }
    }

    /// Return the text the Python reading prints for this number
    /// where it renders one plainly, the integer as its digits
    /// and the double as its shortest round trip text.
    pub fn plain_text(&self) -> String {
        match self {
            Num::Int(value) => value.to_string(),
            Num::Float(value) => py_float_repr(*value),
        }
    }

    /// Add two recorded numbers the way the Python figures add,
    /// an integer sum staying an integer and a sum touching a
    /// double becoming a double. The run record counting reads
    /// this for the usage figures of the envelopes.
    pub fn added(&self, other: &Num) -> Num {
        match (self, other) {
            (Num::Int(a), Num::Int(b)) => Num::Int(a + b),
            _ => Num::Float(self.as_f64() + other.as_f64()),
        }
    }

    fn as_f64(&self) -> f64 {
        match self {
            Num::Int(value) => {
                use num_traits::ToPrimitive;
                value.to_f64().unwrap_or(f64::INFINITY)
            }
            Num::Float(value) => *value,
        }
    }
}

impl PartialEq for Num {
    /// Python number equality, exact across the two shapes. An
    /// integer equals a double when their exact values are equal,
    /// and non finite doubles equal only themselves of their kind.
    fn eq(&self, other: &Num) -> bool {
        match (self, other) {
            (Num::Int(a), Num::Int(b)) => a == b,
            (Num::Float(a), Num::Float(b)) => a == b,
            (Num::Int(a), Num::Float(b)) | (Num::Float(b), Num::Int(a)) => {
                if !b.is_finite() {
                    return false;
                }
                float_to_fraction(*b) == Ratio::from(a.clone())
            }
        }
    }
}

/// Convert a finite double to the fraction of its exact binary
/// value.
fn float_to_fraction(value: f64) -> Ratio<BigInt> {
    assert!(value.is_finite(), "a non finite figure has no fraction");
    if value == 0.0 {
        return Ratio::from(BigInt::from(0));
    }
    let bits = value.to_bits();
    let negative = bits >> 63 == 1;
    let biased = ((bits >> 52) & 0x7FF) as i64;
    let stored = bits & ((1u64 << 52) - 1);
    let (mantissa, exponent) = if biased == 0 {
        (stored, -1022 - 52)
    } else {
        (stored | (1u64 << 52), biased - 1023 - 52)
    };
    let mut numerator = BigInt::from(mantissa);
    if negative {
        numerator = -numerator;
    }
    if exponent >= 0 {
        Ratio::from(numerator << (exponent as usize))
    } else {
        Ratio::new(numerator, BigInt::from(1) << ((-exponent) as usize))
    }
}

/// Render a double the way the Python repr renders it, the
/// shortest decimal text that parses back to the same number,
/// in plain notation inside the stated exponent range and in
/// scientific notation outside it.
pub fn py_float_repr(value: f64) -> String {
    if value.is_nan() {
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value < 0.0 {
            "-inf".to_string()
        } else {
            "inf".to_string()
        };
    }
    if value == 0.0 {
        return if value.is_sign_negative() {
            "-0.0".to_string()
        } else {
            "0.0".to_string()
        };
    }
    let negative = value < 0.0;
    // The shortest digits and the decimal exponent, taken from
    // the scientific rendering, whose digit sequence is the same
    // shortest round trip sequence the Python repr computes.
    let scientific = format!("{:e}", value.abs());
    let (mantissa, exponent_text) = scientific
        .split_once('e')
        .expect("the scientific form carries an exponent");
    let exponent: i64 = exponent_text.parse().expect("the exponent is a number");
    let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
    // The value is 0.digits times ten to the decpt power.
    let decpt = exponent + 1;
    let body = if decpt <= -4 || decpt > 16 {
        // Scientific notation, one digit before the point and
        // the exponent signed and padded to two digits.
        let mut out = String::new();
        out.push(digits.as_bytes()[0] as char);
        if digits.len() > 1 {
            out.push('.');
            out.push_str(&digits[1..]);
        }
        out.push('e');
        let shown = decpt - 1;
        out.push(if shown < 0 { '-' } else { '+' });
        let magnitude = shown.abs();
        if magnitude < 10 {
            out.push('0');
        }
        out.push_str(&magnitude.to_string());
        out
    } else if decpt <= 0 {
        let mut out = String::from("0.");
        for _ in 0..(-decpt) {
            out.push('0');
        }
        out.push_str(&digits);
        out
    } else if (decpt as usize) >= digits.len() {
        let mut out = digits.clone();
        for _ in 0..(decpt as usize - digits.len()) {
            out.push('0');
        }
        out.push_str(".0");
        out
    } else {
        let split = decpt as usize;
        format!("{}.{}", &digits[..split], &digits[split..])
    };
    if negative {
        format!("-{}", body)
    } else {
        body
    }
}

/// Compare a recorded number against a small integer bound
/// exactly, the shape of the confidence range test.
pub fn cmp_int(num: &Num, bound: i64) -> Ordering {
    match num {
        Num::Int(value) => value.cmp(&BigInt::from(bound)),
        Num::Float(value) => value
            .partial_cmp(&(bound as f64))
            .unwrap_or(Ordering::Equal),
    }
}

/// An exact decimal figure, the shape the pilot run record sums
/// its costs in. The value is a coefficient times a power of ten,
/// and the operations the makers use are parsing the text of a
/// recorded figure, adding, rounding half up to six places, and
/// rendering in the text form the Python decimal type renders.
#[derive(Clone, Debug)]
pub struct PyDecimal {
    coefficient: BigInt,
    exponent: i32,
}

impl PyDecimal {
    pub fn zero() -> PyDecimal {
        PyDecimal {
            coefficient: BigInt::from(0),
            exponent: 0,
        }
    }

    /// Parse the text of a recorded figure, the shortest round
    /// trip text of a double or the digits of an integer, into
    /// the exact decimal it names.
    pub fn parse(text: &str) -> PyDecimal {
        let (negative, rest) = match text.strip_prefix('-') {
            Some(stripped) => (true, stripped),
            None => (false, text.strip_prefix('+').unwrap_or(text)),
        };
        let (mantissa, exponent_part) = match rest.split_once(['e', 'E']) {
            Some((head, tail)) => (
                head,
                tail.parse::<i32>().expect("a decimal exponent is a number"),
            ),
            None => (rest, 0),
        };
        let (whole, fraction) = match mantissa.split_once('.') {
            Some((head, tail)) => (head, tail),
            None => (mantissa, ""),
        };
        let digits = format!("{}{}", whole, fraction);
        let mut coefficient =
            BigInt::parse_bytes(digits.as_bytes(), 10).unwrap_or_else(|| BigInt::from(0));
        if negative {
            coefficient = -coefficient;
        }
        PyDecimal {
            coefficient,
            exponent: exponent_part - fraction.len() as i32,
        }
    }

    pub fn added(&self, other: &PyDecimal) -> PyDecimal {
        let exponent = self.exponent.min(other.exponent);
        let mine = &self.coefficient * pow10(self.exponent - exponent);
        let theirs = &other.coefficient * pow10(other.exponent - exponent);
        PyDecimal {
            coefficient: mine + theirs,
            exponent,
        }
    }

    /// Return the count of digits in the coefficient, the
    /// width the Python decimal context measures a figure by
    /// when it rounds one.
    pub fn coefficient_digits(&self) -> usize {
        use num_traits::Signed;
        self.coefficient.abs().to_string().len()
    }

    /// Round to six decimal places, an exact half rounding away
    /// from zero, the ROUND_HALF_UP rule of the run record.
    pub fn quantized_six(&self) -> PyDecimal {
        let target = -6;
        if self.exponent >= target {
            return PyDecimal {
                coefficient: &self.coefficient * pow10(self.exponent - target),
                exponent: target,
            };
        }
        let divisor = pow10(target - self.exponent);
        let negative = self.coefficient < BigInt::from(0);
        let magnitude = if negative {
            -&self.coefficient
        } else {
            self.coefficient.clone()
        };
        let mut quotient = &magnitude / &divisor;
        let remainder = &magnitude % &divisor;
        if &remainder * 2 >= divisor {
            quotient += 1;
        }
        PyDecimal {
            coefficient: if negative { -quotient } else { quotient },
            exponent: target,
        }
    }

    /// Render the figure the way the Python decimal type renders
    /// it, in scientific form when the stored exponent is
    /// positive or the adjusted exponent falls below minus six,
    /// and in plain form otherwise.
    pub fn render(&self) -> String {
        let negative = self.coefficient < BigInt::from(0);
        let digits = if negative {
            (-&self.coefficient).to_string()
        } else {
            self.coefficient.to_string()
        };
        let exponent = self.exponent;
        let adjusted = exponent + digits.len() as i32 - 1;
        let body = if exponent > 0 || adjusted < -6 {
            let mut out = String::new();
            out.push_str(&digits[..1]);
            if digits.len() > 1 {
                out.push('.');
                out.push_str(&digits[1..]);
            }
            out.push('E');
            out.push(if adjusted < 0 { '-' } else { '+' });
            out.push_str(&adjusted.abs().to_string());
            out
        } else {
            let point = digits.len() as i32 + exponent;
            if point <= 0 {
                format!("0.{}{}", "0".repeat((-point) as usize), digits)
            } else if (point as usize) >= digits.len() {
                format!("{}{}", digits, "0".repeat((point as usize) - digits.len()))
            } else {
                format!(
                    "{}.{}",
                    &digits[..point as usize],
                    &digits[point as usize..]
                )
            }
        };
        if negative {
            format!("-{}", body)
        } else {
            body
        }
    }
}

/// How a piece of text reads as a Python decimal figure, the
/// reading the Python decimal constructor applies to the text
/// of a cost figure the run record sums. A finite figure reads
/// as its exact decimal. The two non finite spellings read as
/// their own kinds, because the maker treats them apart, an
/// infinite cost fails where the Python rounding fails and a
/// not a number cost sums and renders as itself.
pub enum DecimalLiteral {
    Finite(PyDecimal),
    Infinite,
    NotANumber { negative: bool },
}

/// Read text the way the Python decimal constructor reads it.
///
/// The constructor strips the whitespace set of the Python
/// string reading from both ends and then ignores every
/// underscore in the text, both probed against the constructor
/// on the build machine, where even a doubled underscore or
/// one at either end reads clean. A sign may lead. The word
/// inf or infinity, in any letter case, reads as an infinite
/// figure, and the word nan reads as a not a number figure.
/// Any other text must spell a decimal figure, digits with an
/// optional point and an optional exponent, where a digit is
/// any code point of category Nd at its decimal value. An
/// exponent past the stored bound settles at the bound, which
/// cannot change an outcome the reading reaches, because a
/// positive exponent that large rounds into the digit refusal
/// when the figure is rounded and a negative one rounds the
/// figure to zero. Any other shape reads as no figure at all,
/// where the Python constructor raises.
pub fn read_decimal_literal(text: &str) -> Option<DecimalLiteral> {
    let held: Vec<char> = text.chars().filter(|point| *point != '_').collect();
    let mut start = 0;
    let mut end = held.len();
    while start < end && crate::pyunicode::is_stripped_space(held[start] as u32) {
        start += 1;
    }
    while end > start && crate::pyunicode::is_stripped_space(held[end - 1] as u32) {
        end -= 1;
    }
    let mut rest: &[char] = &held[start..end];
    let mut negative = false;
    if let Some(first) = rest.first() {
        if *first == '+' || *first == '-' {
            negative = *first == '-';
            rest = &rest[1..];
        }
    }
    if rest.is_empty() {
        return None;
    }
    let word: String = rest.iter().collect::<String>().to_ascii_lowercase();
    if word == "inf" || word == "infinity" {
        return Some(DecimalLiteral::Infinite);
    }
    if word == "nan" {
        return Some(DecimalLiteral::NotANumber { negative });
    }
    let digit_of = |point: char| crate::pyunicode::nd_digit_value(point as u32);
    let mut index = 0;
    let mut digits: Vec<u8> = Vec::new();
    while index < rest.len() {
        match digit_of(rest[index]) {
            Some(value) => {
                digits.push(b'0' + value as u8);
                index += 1;
            }
            None => break,
        }
    }
    let whole_digits = digits.len();
    let mut fraction_digits = 0usize;
    if index < rest.len() && rest[index] == '.' {
        index += 1;
        while index < rest.len() {
            match digit_of(rest[index]) {
                Some(value) => {
                    digits.push(b'0' + value as u8);
                    fraction_digits += 1;
                    index += 1;
                }
                None => break,
            }
        }
    }
    if whole_digits + fraction_digits == 0 {
        return None;
    }
    let mut exponent: i64 = 0;
    if index < rest.len() && (rest[index] == 'e' || rest[index] == 'E') {
        index += 1;
        let mut exponent_negative = false;
        if index < rest.len() && (rest[index] == '+' || rest[index] == '-') {
            exponent_negative = rest[index] == '-';
            index += 1;
        }
        let mut seen = 0usize;
        let mut magnitude: i64 = 0;
        while index < rest.len() {
            match digit_of(rest[index]) {
                Some(value) => {
                    magnitude = magnitude
                        .saturating_mul(10)
                        .saturating_add(value as i64)
                        .min(1_000_000);
                    seen += 1;
                    index += 1;
                }
                None => break,
            }
        }
        if seen == 0 {
            return None;
        }
        exponent = if exponent_negative {
            -magnitude
        } else {
            magnitude
        };
    }
    if index != rest.len() {
        return None;
    }
    let mut coefficient = BigInt::parse_bytes(&digits, 10)?;
    if negative {
        coefficient = -coefficient;
    }
    Some(DecimalLiteral::Finite(PyDecimal {
        coefficient,
        exponent: (exponent - fraction_digits as i64) as i32,
    }))
}

fn pow10(exponent: i32) -> BigInt {
    BigInt::from(10).pow(exponent as u32)
}
