//! Recorded races, the winner rule, the funeral, and the
//! standings.
//!
//! Every function in this module is a pure function of its
//! inputs, and the loaders never raise on malformed content. The
//! obligations this module carries are DH-1 to DH-4 in
//! CONTRACT.md, in the full form section, DH-5 in the Round 2
//! section, and the panel shape and reading of DH-9 in the
//! Round 4 section. The Python module of the same name is the
//! behavioral spec, as PORT-1 states.

use std::path::{Path, PathBuf};

use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{Signed, Zero};

use crate::json::{self, JsonValue};
use crate::pynum::{cmp_int, Num};
use crate::pytext::PText;

/// One suite result word of a recorded attempt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SuiteResult {
    Green,
    Red,
    Unfinished,
}

impl SuiteResult {
    pub fn as_str(&self) -> &'static str {
        match self {
            SuiteResult::Green => "green",
            SuiteResult::Red => "red",
            SuiteResult::Unfinished => "unfinished",
        }
    }

    fn from_text(text: &PText) -> Option<SuiteResult> {
        if *text == PText::from_text("green") {
            Some(SuiteResult::Green)
        } else if *text == PText::from_text("red") {
            Some(SuiteResult::Red)
        } else if *text == PText::from_text("unfinished") {
            Some(SuiteResult::Unfinished)
        } else {
            None
        }
    }
}

/// One recorded attempt, in the clean shape the loader returns.
#[derive(Clone, Debug, PartialEq)]
pub struct Attempt {
    pub model: PText,
    pub environment: PText,
    pub result: SuiteResult,
    pub tokens: Option<BigInt>,
    pub patch: PText,
    pub finished_at: Option<Num>,
}

/// One adjudication entry, the shape DH-5 validates, which is
/// also the shape of one panel entry under DH-9.
#[derive(Clone, Debug, PartialEq)]
pub struct Adjudication {
    pub adjudicator: PText,
    pub chosen: PText,
    pub confidence: Num,
    pub reason: PText,
}

/// One recorded race in its clean shape.
#[derive(Clone, Debug, PartialEq)]
pub struct Race {
    pub task: PText,
    pub attempts: Vec<Attempt>,
    pub adjudication: Option<Adjudication>,
    pub panel: Option<Vec<Adjudication>>,
}

/// Return whether a value the reading produced is a number, the
/// test the Python reading spells with its isinstance checks,
/// which a boolean never passes.
pub fn is_number(value: &JsonValue) -> bool {
    matches!(value, JsonValue::Num(_))
}

/// Return the integer a value holds, when it holds one. A
/// boolean is not an integer and a double is not an integer,
/// however integral its value.
pub fn as_int(value: &JsonValue) -> Option<&BigInt> {
    match value {
        JsonValue::Num(Num::Int(digits)) => Some(digits),
        _ => None,
    }
}

/// Return the exact fraction a recorded number names. A double
/// converts to the fraction of its exact binary value and an
/// integer to itself, so comparisons built on this never round.
pub fn exact(num: &Num) -> Ratio<BigInt> {
    num.to_fraction()
}

fn ratio_int(value: i64) -> Ratio<BigInt> {
    Ratio::from_integer(BigInt::from(value))
}

fn floor_ratio(value: &Ratio<BigInt>) -> BigInt {
    let numerator = value.numer();
    let denominator = value.denom();
    let quotient = numerator / denominator;
    let remainder = numerator % denominator;
    let signs_differ = (remainder < BigInt::from(0)) != (*denominator < BigInt::from(0));
    if !remainder.is_zero() && signs_differ {
        quotient - 1
    } else {
        quotient
    }
}

/// Return the count of units a fraction rounds to at a
/// precision. The count is taken on the exact value, and an
/// exact half rounds up.
fn round_half_up_units(fraction: &Ratio<BigInt>, places: u32) -> BigInt {
    let scale = Ratio::from_integer(BigInt::from(10).pow(places));
    let scaled = fraction * &scale;
    let mut units = floor_ratio(&scaled);
    let remainder = &scaled - Ratio::from_integer(units.clone());
    if remainder * ratio_int(2) >= ratio_int(1) {
        units += 1;
    }
    units
}

fn py_divmod(value: &BigInt, divisor: &BigInt) -> (BigInt, BigInt) {
    let quotient = value / divisor;
    let remainder = value % divisor;
    let signs_differ = (remainder < BigInt::from(0)) != (*divisor < BigInt::from(0));
    if !remainder.is_zero() && signs_differ {
        (quotient - 1, remainder + divisor)
    } else {
        (quotient, remainder)
    }
}

/// Render a count of units at a precision in plain decimal.
fn render_units(units: &BigInt, places: u32) -> String {
    let scale = BigInt::from(10).pow(places);
    let (whole, part) = py_divmod(units, &scale);
    if part.is_zero() {
        return whole.to_string();
    }
    let mut digits = part.to_string();
    while digits.len() < places as usize {
        digits.insert(0, '0');
    }
    let trimmed = digits.trim_end_matches('0');
    format!("{}.{}", whole, trimmed)
}

/// Return the exact decimal expansion of a fraction, or None.
/// The expansion terminates exactly when the denominator of the
/// fraction in its lowest terms carries no prime factor other
/// than two and five.
fn exact_expansion(fraction: &Ratio<BigInt>) -> Option<String> {
    let mut denominator = fraction.denom().clone();
    let two = BigInt::from(2);
    let five = BigInt::from(5);
    let mut twos: u32 = 0;
    while (&denominator % &two).is_zero() {
        denominator /= &two;
        twos += 1;
    }
    let mut fives: u32 = 0;
    while (&denominator % &five).is_zero() {
        denominator /= &five;
        fives += 1;
    }
    if denominator != BigInt::from(1) {
        return None;
    }
    let places = twos.max(fives);
    let scaled = fraction * &Ratio::from_integer(BigInt::from(10).pow(places));
    Some(render_units(&scaled.to_integer(), places))
}

/// Render a number the same way everywhere it appears.
///
/// Seconds and confidence figures both render through here, from
/// the exact value and never through a double. A figure that
/// needs six or fewer decimal places renders exactly. A figure
/// that needs more renders rounded at six decimal places, with
/// an exact half at the seventh place rounding up on the exact
/// value. A nonzero figure that rounds to zero at six places
/// renders its exact decimal expansion when that expansion
/// terminates, and otherwise renders rounded at twelve decimal
/// places with halves rounding up. Every figure renders in
/// plain decimal notation and never in scientific notation, so
/// a nonzero margin never prints as zero. The rule is FIX-DH-8
/// in CONTRACT.md, amending DH-3 and DH-5.
pub fn format_seconds_fraction(fraction: &Ratio<BigInt>) -> String {
    let million = Ratio::from_integer(BigInt::from(1_000_000));
    let scaled = fraction * &million;
    if scaled.denom() == &BigInt::from(1) {
        return render_units(&scaled.to_integer(), 6);
    }
    if !fraction.is_zero() && *fraction < Ratio::new(BigInt::from(1), BigInt::from(2_000_000)) {
        if let Some(expansion) = exact_expansion(fraction) {
            return expansion;
        }
        return render_units(&round_half_up_units(fraction, 12), 12);
    }
    render_units(&round_half_up_units(fraction, 6), 6)
}

/// Render one recorded number through the seconds rule.
pub fn format_seconds(value: &Num) -> String {
    format_seconds_fraction(&exact(value))
}

fn check_text_field(data: &JsonValue, name: &str, defects: &mut Vec<String>) -> Option<PText> {
    let value = match data.get(name) {
        None => {
            defects.push(format!("missing field {}", name));
            return None;
        }
        Some(value) => value,
    };
    let text = match value.as_text() {
        None => {
            defects.push(format!("field {} has the wrong type", name));
            return None;
        }
        Some(text) => text,
    };
    if text.is_empty() {
        defects.push(format!("field {} is empty", name));
        return None;
    }
    if text.holds_unpaired_surrogate() {
        defects.push(format!("field {} holds an unpaired surrogate", name));
        return None;
    }
    if text.holds_line_break() {
        defects.push(format!("field {} holds a newline", name));
        return None;
    }
    Some(text.clone())
}

/// The text field check of the series loader, which shares the
/// race loader's field discipline for its own fields.
pub fn check_text_field_shared(
    data: &JsonValue,
    name: &str,
    defects: &mut Vec<String>,
) -> Option<PText> {
    check_text_field(data, name, defects)
}

fn check_attempt_text(
    entry: &JsonValue,
    index: usize,
    name: &str,
    defects: &mut Vec<String>,
) -> Option<PText> {
    let value = match entry.get(name) {
        None => {
            defects.push(format!("attempt {} is missing field {}", index, name));
            return None;
        }
        Some(value) => value,
    };
    let text = match value.as_text() {
        None => {
            defects.push(format!(
                "attempt {} field {} has the wrong type",
                index, name
            ));
            return None;
        }
        Some(text) => text,
    };
    if text.is_empty() {
        defects.push(format!("attempt {} field {} is empty", index, name));
        return None;
    }
    if text.holds_unpaired_surrogate() {
        defects.push(format!(
            "attempt {} field {} holds an unpaired surrogate",
            index, name
        ));
        return None;
    }
    if text.holds_line_break() {
        defects.push(format!("attempt {} field {} holds a newline", index, name));
        return None;
    }
    Some(text.clone())
}

fn check_finished_at(
    entry: &JsonValue,
    index: usize,
    result: Option<SuiteResult>,
    defects: &mut Vec<String>,
) -> Option<Num> {
    let present = entry.has("finished_at");
    if present {
        let value = entry.get("finished_at").expect("a present field");
        let num = match value.as_num() {
            None => {
                defects.push(format!(
                    "attempt {} field finished_at has the wrong type",
                    index
                ));
                return None;
            }
            Some(num) => num,
        };
        if !num.is_finite_number() {
            defects.push(format!(
                "attempt {} field finished_at is not a finite number",
                index
            ));
            return None;
        }
        if num.is_negative() {
            defects.push(format!("attempt {} field finished_at is negative", index));
            return None;
        }
    }
    match result {
        Some(SuiteResult::Green) | Some(SuiteResult::Red) => {
            if !present {
                defects.push(format!("attempt {} is missing field finished_at", index));
                return None;
            }
            Some(
                entry
                    .get("finished_at")
                    .and_then(|value| value.as_num().cloned())
                    .expect("a validated figure"),
            )
        }
        Some(SuiteResult::Unfinished) => {
            if present {
                defects.push(format!(
                    "attempt {} field finished_at is present but the suite did not finish",
                    index
                ));
            }
            None
        }
        None => None,
    }
}

fn load_attempt(entry: &JsonValue, index: usize, defects: &mut Vec<String>) -> Option<Attempt> {
    if entry.as_object().is_none() {
        defects.push(format!("attempt {} is not an object", index));
        return None;
    }
    let before = defects.len();
    let model = check_attempt_text(entry, index, "model", defects);
    let environment = check_attempt_text(entry, index, "environment", defects);
    let mut result = None;
    match entry.get("result") {
        None => defects.push(format!("attempt {} is missing field result", index)),
        Some(value) => match value.as_text() {
            None => defects.push(format!("attempt {} field result has the wrong type", index)),
            Some(text) => match SuiteResult::from_text(text) {
                None => defects.push(format!(
                    "attempt {} field result is not a suite result",
                    index
                )),
                Some(found) => result = Some(found),
            },
        },
    }
    let mut tokens = None;
    match entry.get("tokens") {
        None => defects.push(format!("attempt {} is missing field tokens", index)),
        Some(JsonValue::Null) => {}
        Some(value) => match as_int(value) {
            None => defects.push(format!("attempt {} field tokens has the wrong type", index)),
            Some(digits) => {
                if *digits < BigInt::from(0) {
                    defects.push(format!("attempt {} field tokens is negative", index));
                } else {
                    tokens = Some(digits.clone());
                }
            }
        },
    }
    let mut patch = None;
    match entry.get("patch") {
        None => defects.push(format!("attempt {} is missing field patch", index)),
        Some(value) => match value.as_text() {
            None => defects.push(format!("attempt {} field patch has the wrong type", index)),
            Some(text) => {
                if text.holds_unpaired_surrogate() {
                    defects.push(format!(
                        "attempt {} field patch holds an unpaired surrogate",
                        index
                    ));
                } else {
                    patch = Some(text.clone());
                }
            }
        },
    }
    let finished = check_finished_at(entry, index, result, defects);
    if defects.len() != before {
        return None;
    }
    Some(Attempt {
        model: model?,
        environment: environment?,
        result: result?,
        tokens,
        patch: patch?,
        finished_at: finished,
    })
}

fn load_attempts(data: &JsonValue, defects: &mut Vec<String>) -> Option<Vec<Attempt>> {
    let raw = match data.get("attempts") {
        None => {
            defects.push("missing field attempts".to_string());
            return None;
        }
        Some(value) => value,
    };
    let entries = match raw.as_array() {
        None => {
            defects.push("field attempts has the wrong type".to_string());
            return None;
        }
        Some(entries) => entries,
    };
    if entries.len() != 2 {
        defects.push("field attempts must hold exactly two attempts".to_string());
    }
    let mut clean = Vec::new();
    for (index, entry) in entries.iter().enumerate() {
        if let Some(attempt) = load_attempt(entry, index, defects) {
            clean.push(attempt);
        }
    }
    if entries.len() != 2 || clean.len() != entries.len() {
        return None;
    }
    Some(clean)
}

fn check_adjudication_text(
    entry: &JsonValue,
    name: &str,
    defects: &mut Vec<String>,
) -> Option<PText> {
    let value = match entry.get(name) {
        None => {
            defects.push(format!("adjudication is missing field {}", name));
            return None;
        }
        Some(value) => value,
    };
    let text = match value.as_text() {
        None => {
            defects.push(format!("adjudication field {} has the wrong type", name));
            return None;
        }
        Some(text) => text,
    };
    if text.is_empty() {
        defects.push(format!("adjudication field {} is empty", name));
        return None;
    }
    if text.holds_unpaired_surrogate() {
        defects.push(format!(
            "adjudication field {} holds an unpaired surrogate",
            name
        ));
        return None;
    }
    if text.holds_line_break() {
        defects.push(format!("adjudication field {} holds a newline", name));
        return None;
    }
    Some(text.clone())
}

fn check_confidence(
    entry: &JsonValue,
    missing: String,
    wrong_type: String,
    not_finite: String,
    outside: String,
    defects: &mut Vec<String>,
) -> Option<Num> {
    let value = match entry.get("confidence") {
        None => {
            defects.push(missing);
            return None;
        }
        Some(value) => value,
    };
    let num = match value.as_num() {
        None => {
            defects.push(wrong_type);
            return None;
        }
        Some(num) => num,
    };
    if !num.is_finite_number() {
        defects.push(not_finite);
        return None;
    }
    if cmp_int(num, 0) == std::cmp::Ordering::Less || cmp_int(num, 1) == std::cmp::Ordering::Greater
    {
        defects.push(outside);
        return None;
    }
    Some(num.clone())
}

fn load_adjudication(
    data: &JsonValue,
    attempts: Option<&Vec<Attempt>>,
    defects: &mut Vec<String>,
) -> Option<Adjudication> {
    let entry = data.get("adjudication")?;
    if entry.as_object().is_none() {
        defects.push("field adjudication is not an object".to_string());
        return None;
    }
    let before = defects.len();
    let adjudicator = check_adjudication_text(entry, "adjudicator", defects);
    let chosen = check_adjudication_text(entry, "chosen", defects);
    let reason = check_adjudication_text(entry, "reason", defects);
    let confidence = check_confidence(
        entry,
        "adjudication is missing field confidence".to_string(),
        "adjudication field confidence has the wrong type".to_string(),
        "adjudication field confidence is not a finite number".to_string(),
        "adjudication field confidence is outside the range zero to one".to_string(),
        defects,
    );
    if let (Some(chosen), Some(attempts)) = (&chosen, attempts) {
        if !attempts.iter().any(|attempt| attempt.model == *chosen) {
            defects.push("adjudication field chosen names no attempt in this race".to_string());
        }
    }
    if defects.len() != before {
        return None;
    }
    Some(Adjudication {
        adjudicator: adjudicator?,
        chosen: chosen?,
        confidence: confidence?,
        reason: reason?,
    })
}

fn check_panel_entry_text(
    entry: &JsonValue,
    index: usize,
    name: &str,
    defects: &mut Vec<String>,
) -> Option<PText> {
    let value = match entry.get(name) {
        None => {
            defects.push(format!("panel entry {} is missing field {}", index, name));
            return None;
        }
        Some(value) => value,
    };
    let text = match value.as_text() {
        None => {
            defects.push(format!(
                "panel entry {} field {} has the wrong type",
                index, name
            ));
            return None;
        }
        Some(text) => text,
    };
    if text.is_empty() {
        defects.push(format!("panel entry {} field {} is empty", index, name));
        return None;
    }
    if text.holds_unpaired_surrogate() {
        defects.push(format!(
            "panel entry {} field {} holds an unpaired surrogate",
            index, name
        ));
        return None;
    }
    if text.holds_line_break() {
        defects.push(format!(
            "panel entry {} field {} holds a newline",
            index, name
        ));
        return None;
    }
    Some(text.clone())
}

fn load_panel_entry(
    entry: &JsonValue,
    index: usize,
    attempts: Option<&Vec<Attempt>>,
    defects: &mut Vec<String>,
) -> Option<Adjudication> {
    if entry.as_object().is_none() {
        defects.push(format!("panel entry {} is not an object", index));
        return None;
    }
    let before = defects.len();
    let adjudicator = check_panel_entry_text(entry, index, "adjudicator", defects);
    let chosen = check_panel_entry_text(entry, index, "chosen", defects);
    let reason = check_panel_entry_text(entry, index, "reason", defects);
    let confidence = check_confidence(
        entry,
        format!("panel entry {} is missing field confidence", index),
        format!("panel entry {} field confidence has the wrong type", index),
        format!(
            "panel entry {} field confidence is not a finite number",
            index
        ),
        format!(
            "panel entry {} field confidence is outside the range zero to one",
            index
        ),
        defects,
    );
    if let (Some(chosen), Some(attempts)) = (&chosen, attempts) {
        if !attempts.iter().any(|attempt| attempt.model == *chosen) {
            defects.push(format!(
                "panel entry {} field chosen names no attempt in this race",
                index
            ));
        }
    }
    if defects.len() != before {
        return None;
    }
    Some(Adjudication {
        adjudicator: adjudicator?,
        chosen: chosen?,
        confidence: confidence?,
        reason: reason?,
    })
}

fn load_panel(
    data: &JsonValue,
    attempts: Option<&Vec<Attempt>>,
    defects: &mut Vec<String>,
) -> Option<Vec<Adjudication>> {
    let raw = data.get("panel")?;
    if data.has("adjudication") {
        defects.push("field panel stands beside an adjudication entry".to_string());
    }
    let entries = match raw.as_array() {
        None => {
            defects.push("field panel has the wrong type".to_string());
            return None;
        }
        Some(entries) => entries,
    };
    let before = defects.len();
    if entries.is_empty() {
        defects.push("field panel holds no entries".to_string());
    }
    if entries.len() > 2 {
        defects.push("field panel holds more than two entries".to_string());
    }
    let mut clean = Vec::new();
    for (index, entry) in entries.iter().enumerate() {
        if let Some(loaded) = load_panel_entry(entry, index, attempts, defects) {
            clean.push(loaded);
        }
    }
    if clean.len() == 2 && clean[0].adjudicator == clean[1].adjudicator {
        defects.push("field panel names the same adjudicator twice".to_string());
    }
    if defects.len() != before || clean.len() != entries.len() {
        return None;
    }
    Some(clean)
}

/// Build a race from a parsed JSON object, naming every defect.
///
/// Returns a pair of the race and the defect list. A malformed
/// object returns no race, and this function never fails on bad
/// content. The text and file entry points parse first and then
/// read the race through here.
pub fn load_race_data(data: &JsonValue) -> (Option<Race>, Vec<String>) {
    let mut defects = Vec::new();
    let task = check_text_field(data, "task", &mut defects);
    let attempts = load_attempts(data, &mut defects);
    let adjudication = load_adjudication(data, attempts.as_ref(), &mut defects);
    let panel = load_panel(data, attempts.as_ref(), &mut defects);
    if !defects.is_empty() {
        return (None, defects);
    }
    (
        Some(Race {
            task: task.expect("a clean task"),
            attempts: attempts.expect("clean attempts"),
            adjudication,
            panel,
        }),
        Vec::new(),
    )
}

/// Parse recorded race text into a race, naming every defect
/// found. A malformed input returns no race.
pub fn load_race_text(text: &str) -> (Option<Race>, Vec<String>) {
    let data = match json::parse(text) {
        Ok(data) => data,
        Err(_) => return (None, vec!["input is not valid JSON".to_string()]),
    };
    if data.as_object().is_none() {
        return (None, vec!["input is not a JSON object".to_string()]);
    }
    load_race_data(&data)
}

/// Read a recorded race from disk, with the same contract as
/// text.
pub fn load_race_file(path: &Path) -> (Option<Race>, Vec<String>) {
    let text = match read_text_file(path) {
        None => return (None, vec!["file cannot be read".to_string()]),
        Some(text) => text,
    };
    load_race_text(&text)
}

/// Read a file as UTF-8 text, or None when it cannot be read,
/// the failure the loaders name as the file cannot be read
/// defect. A directory and an undecodable file both land here,
/// as they do under the Python reading.
pub fn read_text_file(path: &Path) -> Option<String> {
    let bytes = std::fs::read(path).ok()?;
    String::from_utf8(bytes).ok()
}

/// Return whether a race sits in the close call band of DH-5.
///
/// Both attempts holding the same non green outcome is a close
/// call. Both attempts green is a close call when the finish
/// times differ and the gap is no more than five percent of the
/// slower finish. The comparison multiplies the exact gap by
/// twenty and compares it against the exact slower finish, so
/// no rounding at any precision can move a race across the band
/// edge. An exact tie of green finish times is not a close call.
pub fn is_close_call(race: &Race) -> bool {
    let first = &race.attempts[0];
    let second = &race.attempts[1];
    if first.result != second.result {
        return false;
    }
    if first.result != SuiteResult::Green {
        return true;
    }
    let first_finish = exact(first.finished_at.as_ref().expect("a green finish"));
    let second_finish = exact(second.finished_at.as_ref().expect("a green finish"));
    let gap = (first_finish.clone() - second_finish.clone()).abs();
    let slower = first_finish.max(second_finish);
    !gap.is_zero() && gap * ratio_int(20) <= slower
}

fn confidence_threshold() -> Ratio<BigInt> {
    Ratio::new(BigInt::from(4), BigInt::from(5))
}

/// Return whether a recorded confidence meets the DH-5
/// threshold. The recorded figure is compared against four
/// fifths on exact fractions, so a recorded 0.8 meets the
/// threshold and no rounding can move a verdict across it.
pub fn confidence_meets(confidence: &Num) -> bool {
    exact(confidence) >= confidence_threshold()
}

/// Render a confidence for a line reporting it below the
/// threshold, under FIX-DH-10. A confidence below the threshold
/// whose rendering at six places would print the threshold
/// figure itself renders in full precision instead, so a line
/// reporting an abstention never shows a confidence equal to
/// the threshold.
fn format_confidence(confidence: &Num) -> String {
    let fraction = exact(confidence);
    if fraction < confidence_threshold()
        && format_seconds_fraction(&fraction) == format_seconds_fraction(&confidence_threshold())
    {
        if let Some(expansion) = exact_expansion(&fraction) {
            return expansion;
        }
        return render_units(&round_half_up_units(&fraction, 12), 12);
    }
    format_seconds_fraction(&fraction)
}

fn adjudication_state(race: &Race) -> &'static str {
    let entry = &race.adjudication;
    if !is_close_call(race) {
        return if entry.is_some() {
            "not needed"
        } else {
            "absent"
        };
    }
    match entry {
        None => "unrecorded",
        Some(entry) => {
            if confidence_meets(&entry.confidence) {
                "decided"
            } else {
                "abstained"
            }
        }
    }
}

/// Return the entries of a race panel that decide under DH-9.
fn panel_deciders(race: &Race) -> Vec<&Adjudication> {
    race.panel
        .as_ref()
        .expect("a panel race")
        .iter()
        .filter(|entry| confidence_meets(&entry.confidence))
        .collect()
}

fn panel_state(race: &Race) -> &'static str {
    if !is_close_call(race) {
        return "not needed";
    }
    let deciders = panel_deciders(race);
    if deciders.len() == 2 {
        if deciders[0].chosen == deciders[1].chosen {
            return "confirmed";
        }
        return "divided";
    }
    if deciders.len() == 1 {
        return "follows";
    }
    "abstained"
}

/// Return the entry clauses of a panel line, in entry order.
fn panel_body(race: &Race) -> String {
    let panel = race.panel.as_ref().expect("a panel race");
    let mut clauses = Vec::new();
    for entry in panel.iter() {
        if confidence_meets(&entry.confidence) {
            clauses.push(format!(
                "adjudication by {} chose {} at confidence {}",
                entry.adjudicator.display(),
                entry.chosen.display(),
                format_seconds(&entry.confidence)
            ));
        } else {
            clauses.push(format!(
                "adjudication by {} abstained at confidence {}",
                entry.adjudicator.display(),
                format_confidence(&entry.confidence)
            ));
        }
    }
    if panel.len() == 1 {
        clauses.push("the other judge is absent".to_string());
    }
    clauses.join(" and ")
}

fn panel_report_line(race: &Race) -> String {
    let state = panel_state(race);
    if state == "not needed" {
        return "panel not needed, the race stands outside the close call band".to_string();
    }
    let body = panel_body(race);
    match state {
        "confirmed" => {
            let (winner, _) = decide_winner(race);
            let winner = &race.attempts[winner.expect("a confirmed winner")];
            format!(
                "close call, panel, {}, the panel confirms {}",
                body,
                winner.model.display()
            )
        }
        "follows" => {
            let decider = panel_deciders(race)[0];
            format!(
                "close call, panel, {}, the panel follows {}",
                body,
                decider.adjudicator.display()
            )
        }
        "divided" => format!("close call, panel, {}, the panel stands divided", body),
        _ => format!("close call, panel, {}", body),
    }
}

fn panel_funeral_line(race: &Race) -> Option<String> {
    let state = panel_state(race);
    if state == "not needed" {
        return None;
    }
    let body = panel_body(race);
    match state {
        "confirmed" | "follows" => Some(format!("decided by the panel, {}", body)),
        "divided" => Some(format!(
            "close call, panel, {}, the panel stands divided, the race stands as a dead heat",
            body
        )),
        _ => Some(format!(
            "close call, panel, {}, the race stands as a dead heat",
            body
        )),
    }
}

fn panel_close_call_line(race: &Race) -> String {
    let state = panel_state(race);
    if state == "not needed" {
        return "panel not needed, the race stands outside the close call band".to_string();
    }
    let body = panel_body(race);
    match state {
        "confirmed" | "follows" => {
            let (winner, _) = decide_winner(race);
            let winner = &race.attempts[winner.expect("a decided winner")];
            format!(
                "panel, {}, the race is decided for {}",
                body,
                winner.model.display()
            )
        }
        "divided" => format!("panel, {}, the panel stands divided", body),
        _ => format!(
            "panel, {}, below the threshold of {}",
            body,
            format_seconds_fraction(&confidence_threshold())
        ),
    }
}

/// Return the winning and losing attempts of a race, as the
/// indices of the attempts in the record, or None on a side
/// that has no attempt.
///
/// A close call carrying a panel under DH-9 is decided for the
/// chosen attempt the panel confirms or follows, and stands
/// with no winner when the panel stands divided or no entry
/// decides. A close call under DH-5 is decided for the chosen
/// attempt when its adjudication entry meets the confidence
/// threshold, and stands with no winner otherwise. Outside the
/// band a green attempt beats a red or unfinished attempt. When
/// both attempts are green the earlier finish wins, and an
/// exact tie of finish times goes to the attempt that stands
/// first in the record. When neither attempt is green both
/// returns are None.
pub fn decide_winner(race: &Race) -> (Option<usize>, Option<usize>) {
    let first = &race.attempts[0];
    let second = &race.attempts[1];
    if is_close_call(race) {
        if race.panel.is_some() {
            let deciders = panel_deciders(race);
            if deciders.is_empty() {
                return (None, None);
            }
            let chosen = &deciders[0].chosen;
            if deciders.iter().any(|entry| entry.chosen != *chosen) {
                return (None, None);
            }
            let winner = if first.model == *chosen { 0 } else { 1 };
            return (Some(winner), Some(1 - winner));
        }
        if let Some(entry) = &race.adjudication {
            if confidence_meets(&entry.confidence) {
                let winner = if first.model == entry.chosen { 0 } else { 1 };
                return (Some(winner), Some(1 - winner));
            }
        }
        return (None, None);
    }
    let greens: Vec<usize> = race
        .attempts
        .iter()
        .enumerate()
        .filter(|(_, attempt)| attempt.result == SuiteResult::Green)
        .map(|(index, _)| index)
        .collect();
    if greens.is_empty() {
        return (None, None);
    }
    let winner = if greens.len() == 1 {
        greens[0]
    } else {
        let first_finish = exact(first.finished_at.as_ref().expect("a green finish"));
        let second_finish = exact(second.finished_at.as_ref().expect("a green finish"));
        if first_finish <= second_finish {
            0
        } else {
            1
        }
    };
    (Some(winner), Some(1 - winner))
}

fn outcome_phrase(attempt: &Attempt) -> String {
    if attempt.result == SuiteResult::Unfinished {
        return "never finished".to_string();
    }
    format!(
        "went {} in {} seconds",
        attempt.result.as_str(),
        format_seconds(attempt.finished_at.as_ref().expect("a finish"))
    )
}

fn report_line(attempt: &Attempt) -> String {
    let outcome = if attempt.result == SuiteResult::Unfinished {
        "unfinished".to_string()
    } else {
        format!(
            "{} in {} seconds",
            attempt.result.as_str(),
            format_seconds(attempt.finished_at.as_ref().expect("a finish"))
        )
    };
    let spend = match &attempt.tokens {
        None => "tokens unreported".to_string(),
        Some(tokens) => format!("{} tokens", tokens),
    };
    format!(
        "{} in {}, {}, {}",
        attempt.model.display(),
        attempt.environment.display(),
        outcome,
        spend
    )
}

/// Return the report lines for one race.
pub fn race_report(race: &Race) -> Vec<String> {
    let mut lines = vec![format!("race {}", race.task.display())];
    for attempt in &race.attempts {
        lines.push(report_line(attempt));
    }
    let (winner, _) = decide_winner(race);
    match winner {
        None => lines.push("verdict, no winner".to_string()),
        Some(winner) => lines.push(format!(
            "verdict, {} wins",
            race.attempts[winner].model.display()
        )),
    }
    if race.panel.is_some() {
        lines.push(panel_report_line(race));
        return lines;
    }
    let state = adjudication_state(race);
    if let Some(entry) = &race.adjudication {
        match state {
            "decided" => {
                let winner = winner.expect("a decided winner");
                lines.push(format!(
                    "close call, adjudication by {} chose {} at confidence {}",
                    entry.adjudicator.display(),
                    race.attempts[winner].model.display(),
                    format_seconds(&entry.confidence)
                ));
            }
            "abstained" => lines.push(format!(
                "close call, adjudication by {} abstained at confidence {}",
                entry.adjudicator.display(),
                format_confidence(&entry.confidence)
            )),
            "not needed" => lines.push(format!(
                "adjudication by {} not needed, the race stands outside the close call band",
                entry.adjudicator.display()
            )),
            _ => {}
        }
    } else if state == "unrecorded" {
        lines.push("close call, no adjudication on record".to_string());
    }
    lines
}

/// Return the funeral lines for one race.
pub fn funeral(race: &Race) -> Vec<String> {
    let mut lines = vec![format!("funeral for {}", race.task.display())];
    let (winner, loser) = decide_winner(race);
    let first = &race.attempts[0];
    let second = &race.attempts[1];
    match (winner, loser) {
        (None, _) => {
            if first.result == SuiteResult::Green && second.result == SuiteResult::Green {
                lines.push(
                    "no winner, both suites went green inside the close call band".to_string(),
                );
            } else {
                lines.push("no winner, neither suite went green".to_string());
            }
            for attempt in &race.attempts {
                lines.push(format!(
                    "{} in {} {}",
                    attempt.model.display(),
                    attempt.environment.display(),
                    outcome_phrase(attempt)
                ));
            }
        }
        (Some(winner), Some(loser)) => {
            let winner_attempt = &race.attempts[winner];
            let loser_attempt = &race.attempts[loser];
            lines.push(format!(
                "winner {} in {} {}",
                winner_attempt.model.display(),
                winner_attempt.environment.display(),
                outcome_phrase(winner_attempt)
            ));
            lines.push(format!(
                "loser {} in {} {}",
                loser_attempt.model.display(),
                loser_attempt.environment.display(),
                outcome_phrase(loser_attempt)
            ));
        }
        _ => unreachable!("a winner stands with a loser"),
    }
    if race.panel.is_some() {
        if let Some(line) = panel_funeral_line(race) {
            lines.push(line);
        }
    } else if let Some(entry) = &race.adjudication {
        let state = adjudication_state(race);
        match state {
            "decided" => {
                let winner = winner.expect("a decided winner");
                lines.push(format!(
                    "decided by adjudication, {} chose {} at confidence {}",
                    entry.adjudicator.display(),
                    race.attempts[winner].model.display(),
                    format_seconds(&entry.confidence)
                ));
            }
            "abstained" => lines.push(format!(
                "close call, adjudication by {} abstained at confidence {}, the race stands as a dead heat",
                entry.adjudicator.display(),
                format_confidence(&entry.confidence)
            )),
            "unrecorded" => lines.push(
                "close call, no adjudication on record, the race stands as a dead heat".to_string(),
            ),
            _ => {}
        }
    } else if adjudication_state(race) == "unrecorded" {
        lines.push(
            "close call, no adjudication on record, the race stands as a dead heat".to_string(),
        );
    }
    if let (Some(first_finish), Some(second_finish)) = (&first.finished_at, &second.finished_at) {
        let margin = (exact(first_finish) - exact(second_finish)).abs();
        lines.push(format!(
            "margin {} seconds between the two finishes",
            format_seconds_fraction(&margin)
        ));
    }
    lines
}

/// Return the closecall lines for one race.
pub fn close_call_report(race: &Race) -> Vec<String> {
    let first = &race.attempts[0];
    let second = &race.attempts[1];
    let mut lines = vec![format!("closecall {}", race.task.display())];
    if first.result == SuiteResult::Green && second.result == SuiteResult::Green {
        let first_finish = exact(first.finished_at.as_ref().expect("a green finish"));
        let second_finish = exact(second.finished_at.as_ref().expect("a green finish"));
        let gap = (first_finish.clone() - second_finish.clone()).abs();
        let slower = first_finish.max(second_finish);
        if gap.is_zero() {
            lines.push(
                "outside the band, the finish times are tied and the tie goes to the first attempt in the record"
                    .to_string(),
            );
        } else if is_close_call(race) {
            lines.push(format!(
                "in the band, both attempts went green and the gap of {} seconds is within five percent of the slower finish of {} seconds",
                format_seconds_fraction(&gap),
                format_seconds_fraction(&slower)
            ));
        } else {
            lines.push(format!(
                "outside the band, the gap of {} seconds is wider than five percent of the slower finish of {} seconds",
                format_seconds_fraction(&gap),
                format_seconds_fraction(&slower)
            ));
        }
    } else if first.result == second.result {
        if first.result == SuiteResult::Red {
            lines.push("in the band, both attempts went red".to_string());
        } else {
            lines.push("in the band, both attempts never finished".to_string());
        }
    } else {
        lines.push("outside the band, the two attempts hold different outcomes".to_string());
    }
    if race.panel.is_some() {
        lines.push(panel_close_call_line(race));
    } else if let Some(entry) = &race.adjudication {
        let state = adjudication_state(race);
        match state {
            "decided" => {
                let (winner, _) = decide_winner(race);
                let winner = winner.expect("a decided winner");
                lines.push(format!(
                    "adjudication by {} chose {} at confidence {}, the race is decided for {}",
                    entry.adjudicator.display(),
                    race.attempts[winner].model.display(),
                    format_seconds(&entry.confidence),
                    race.attempts[winner].model.display()
                ));
            }
            "abstained" => lines.push(format!(
                "adjudication by {} abstained at confidence {}, below the threshold of {}",
                entry.adjudicator.display(),
                format_confidence(&entry.confidence),
                format_seconds_fraction(&confidence_threshold())
            )),
            "not needed" => lines.push(format!(
                "adjudication by {} not needed, the race stands outside the close call band",
                entry.adjudicator.display()
            )),
            _ => {}
        }
    } else if adjudication_state(race) == "unrecorded" {
        lines.push("no adjudication on record, the race stands as no result".to_string());
    }
    let (winner, _) = decide_winner(race);
    match winner {
        None => lines.push("verdict, no winner".to_string()),
        Some(winner) => lines.push(format!(
            "verdict, {} wins",
            race.attempts[winner].model.display()
        )),
    }
    lines
}

/// Reduce a label to a slug of lower case letters and digits.
/// Runs of other characters become single dashes, dashes at the
/// edges are dropped, and a label that reduces to nothing
/// becomes the word unnamed.
pub fn slugify(label: &PText) -> String {
    let lowered: String = label
        .display()
        .chars()
        .flat_map(|c| c.to_lowercase())
        .collect();
    let mut slug = String::new();
    let mut dash_pending = false;
    for c in lowered.chars() {
        if c.is_ascii_lowercase() || c.is_ascii_digit() {
            if dash_pending && !slug.is_empty() {
                slug.push('-');
            }
            dash_pending = false;
            slug.push(c);
        } else {
            dash_pending = true;
        }
    }
    if slug.is_empty() {
        "unnamed".to_string()
    } else {
        slug
    }
}

/// Return the file name the loser patch lives under.
pub fn patch_file_name(race: &Race) -> String {
    let (_, loser) = decide_winner(race);
    let loser = &race.attempts[loser.expect("a race with a loser")];
    format!("{}-{}.patch", slugify(&race.task), slugify(&loser.model))
}

/// Write the loser patch verbatim into the patches folder.
/// Returns the path written, or None when the race has no
/// loser.
pub fn adopt_patch(race: &Race, patches_dir: &Path) -> Option<PathBuf> {
    let (_, loser) = decide_winner(race);
    let loser = &race.attempts[loser?];
    std::fs::create_dir_all(patches_dir).ok()?;
    let target = patches_dir.join(patch_file_name(race));
    std::fs::write(&target, loser.patch.display()).ok()?;
    Some(target)
}

/// Remove the loser patch file when it stands there. Returns
/// the path the patch lives under, or None when the race has no
/// loser. Removing a file that is not there still succeeds.
pub fn discard_patch(race: &Race, patches_dir: &Path) -> Option<PathBuf> {
    let (_, loser) = decide_winner(race);
    loser?;
    let target = patches_dir.join(patch_file_name(race));
    let _ = std::fs::remove_file(&target);
    Some(target)
}

/// One model's standing line in the season standings.
#[derive(Clone, Debug, PartialEq)]
pub struct StandingEntry {
    pub wins: BigInt,
    pub losses: BigInt,
    pub no_results: BigInt,
    pub tokens: BigInt,
    pub tokens_unreported: BigInt,
}

impl StandingEntry {
    fn blank() -> StandingEntry {
        StandingEntry {
            wins: BigInt::from(0),
            losses: BigInt::from(0),
            no_results: BigInt::from(0),
            tokens: BigInt::from(0),
            tokens_unreported: BigInt::from(0),
        }
    }

    fn to_json(&self) -> JsonValue {
        JsonValue::object(vec![
            (
                PText::from_text("wins"),
                JsonValue::Num(Num::Int(self.wins.clone())),
            ),
            (
                PText::from_text("losses"),
                JsonValue::Num(Num::Int(self.losses.clone())),
            ),
            (
                PText::from_text("no_results"),
                JsonValue::Num(Num::Int(self.no_results.clone())),
            ),
            (
                PText::from_text("tokens"),
                JsonValue::Num(Num::Int(self.tokens.clone())),
            ),
            (
                PText::from_text("tokens_unreported"),
                JsonValue::Num(Num::Int(self.tokens_unreported.clone())),
            ),
        ])
    }
}

/// The season standings, model entries in record order.
#[derive(Clone, Debug, PartialEq)]
pub struct Standings {
    pub models: Vec<(PText, StandingEntry)>,
}

/// Return the standings a season starts from.
pub fn empty_standings() -> Standings {
    Standings { models: Vec::new() }
}

impl Standings {
    fn entry_for(&mut self, model: &PText) -> &mut StandingEntry {
        if let Some(position) = self.models.iter().position(|(name, _)| name == model) {
            return &mut self.models[position].1;
        }
        self.models.push((model.clone(), StandingEntry::blank()));
        &mut self.models.last_mut().expect("a pushed entry").1
    }

    pub fn to_json(&self) -> JsonValue {
        let pairs: Vec<(PText, JsonValue)> = self
            .models
            .iter()
            .map(|(name, entry)| (name.clone(), entry.to_json()))
            .collect();
        JsonValue::object(vec![(PText::from_text("models"), JsonValue::object(pairs))])
    }
}

/// Return new standings with one race recorded. The input
/// standings are never mutated.
pub fn update_standings(standings: &Standings, race: &Race) -> Standings {
    let mut updated = standings.clone();
    let (winner, loser) = decide_winner(race);
    match (winner, loser) {
        (Some(winner), Some(loser)) => {
            let winner_model = race.attempts[winner].model.clone();
            let loser_model = race.attempts[loser].model.clone();
            updated.entry_for(&winner_model).wins += 1;
            updated.entry_for(&loser_model).losses += 1;
        }
        _ => {
            for attempt in &race.attempts {
                let model = attempt.model.clone();
                updated.entry_for(&model).no_results += 1;
            }
        }
    }
    for attempt in &race.attempts {
        let model = attempt.model.clone();
        let entry = updated.entry_for(&model);
        match &attempt.tokens {
            None => entry.tokens_unreported += 1,
            Some(tokens) => entry.tokens += tokens.clone(),
        }
    }
    updated
}

const STANDINGS_FIELDS: [&str; 5] = [
    "wins",
    "losses",
    "no_results",
    "tokens",
    "tokens_unreported",
];

fn load_standings_entry(
    name: &PText,
    entry: &JsonValue,
    defects: &mut Vec<String>,
) -> Option<StandingEntry> {
    if entry.as_object().is_none() {
        defects.push(format!(
            "standings entry {} is not an object",
            name.display()
        ));
        return None;
    }
    let before = defects.len();
    let mut values: Vec<BigInt> = Vec::new();
    for field in STANDINGS_FIELDS {
        match entry.get(field) {
            None => defects.push(format!(
                "standings entry {} is missing field {}",
                name.display(),
                field
            )),
            Some(value) => match as_int(value) {
                None => defects.push(format!(
                    "standings entry {} field {} has the wrong type",
                    name.display(),
                    field
                )),
                Some(digits) => {
                    if *digits < BigInt::from(0) {
                        defects.push(format!(
                            "standings entry {} field {} is negative",
                            name.display(),
                            field
                        ));
                    } else {
                        values.push(digits.clone());
                    }
                }
            },
        }
    }
    if defects.len() != before {
        return None;
    }
    Some(StandingEntry {
        wins: values[0].clone(),
        losses: values[1].clone(),
        no_results: values[2].clone(),
        tokens: values[3].clone(),
        tokens_unreported: values[4].clone(),
    })
}

/// Parse standings text, naming every defect found.
pub fn load_standings_text(text: &str) -> (Option<Standings>, Vec<String>) {
    let data = match json::parse(text) {
        Ok(data) => data,
        Err(_) => return (None, vec!["input is not valid JSON".to_string()]),
    };
    if data.as_object().is_none() {
        return (None, vec!["input is not a JSON object".to_string()]);
    }
    let mut defects = Vec::new();
    let mut models = None;
    match data.get("models") {
        None => defects.push("missing field models".to_string()),
        Some(value) => match value.as_object() {
            None => defects.push("field models has the wrong type".to_string()),
            Some(pairs) => {
                let mut loaded = Vec::new();
                for (name, entry) in pairs {
                    if name.holds_unpaired_surrogate() {
                        defects.push("field models holds an unpaired surrogate".to_string());
                        continue;
                    }
                    if name.holds_line_break() {
                        defects.push("field models holds a newline".to_string());
                        continue;
                    }
                    if let Some(clean) = load_standings_entry(name, entry, &mut defects) {
                        loaded.push((name.clone(), clean));
                    }
                }
                models = Some(loaded);
            }
        },
    }
    if !defects.is_empty() {
        return (None, defects);
    }
    (
        Some(Standings {
            models: models.expect("loaded models"),
        }),
        Vec::new(),
    )
}

/// Read standings from disk, with the same contract as text.
pub fn load_standings_file(path: &Path) -> (Option<Standings>, Vec<String>) {
    let text = match read_text_file(path) {
        None => return (None, vec!["file cannot be read".to_string()]),
        Some(text) => text,
    };
    load_standings_text(&text)
}

/// Render standings as the JSON text the standings file holds.
pub fn standings_text(standings: &Standings) -> String {
    let mut text = json::dumps(&standings.to_json(), Some(2), true);
    text.push('\n');
    text
}

/// Return the standings table lines, one per model in label
/// order.
pub fn standings_table(standings: &Standings) -> Vec<String> {
    let mut lines = vec!["season standings".to_string()];
    if standings.models.is_empty() {
        lines.push("no races recorded".to_string());
    }
    let mut sorted: Vec<&(PText, StandingEntry)> = standings.models.iter().collect();
    sorted.sort_by(|a, b| a.0.cmp_code_points(&b.0));
    for (name, entry) in sorted {
        lines.push(format!(
            "{}, {} wins, {} losses, {} no results, {} tokens, {} unreported",
            name.display(),
            entry.wins,
            entry.losses,
            entry.no_results,
            entry.tokens,
            entry.tokens_unreported
        ));
    }
    lines
}

/// Render a race as the JSON value the makers write, in the
/// shape the loader reads back.
pub fn race_to_json(race: &Race) -> JsonValue {
    let attempts: Vec<JsonValue> = race
        .attempts
        .iter()
        .map(|attempt| {
            let mut pairs = vec![
                (
                    PText::from_text("model"),
                    JsonValue::Str(attempt.model.clone()),
                ),
                (
                    PText::from_text("environment"),
                    JsonValue::Str(attempt.environment.clone()),
                ),
                (
                    PText::from_text("result"),
                    JsonValue::Str(PText::from_text(attempt.result.as_str())),
                ),
                (
                    PText::from_text("tokens"),
                    match &attempt.tokens {
                        None => JsonValue::Null,
                        Some(tokens) => JsonValue::Num(Num::Int(tokens.clone())),
                    },
                ),
                (
                    PText::from_text("patch"),
                    JsonValue::Str(attempt.patch.clone()),
                ),
            ];
            if let Some(finished) = &attempt.finished_at {
                pairs.push((
                    PText::from_text("finished_at"),
                    JsonValue::Num(finished.clone()),
                ));
            }
            JsonValue::object(pairs)
        })
        .collect();
    let mut pairs = vec![
        (PText::from_text("task"), JsonValue::Str(race.task.clone())),
        (PText::from_text("attempts"), JsonValue::Arr(attempts)),
    ];
    if let Some(entry) = &race.adjudication {
        pairs.push((
            PText::from_text("adjudication"),
            adjudication_to_json(entry),
        ));
    }
    if let Some(panel) = &race.panel {
        pairs.push((
            PText::from_text("panel"),
            JsonValue::Arr(panel.iter().map(adjudication_to_json).collect()),
        ));
    }
    JsonValue::object(pairs)
}

/// Render one adjudication entry as its JSON value.
pub fn adjudication_to_json(entry: &Adjudication) -> JsonValue {
    JsonValue::object(vec![
        (
            PText::from_text("adjudicator"),
            JsonValue::Str(entry.adjudicator.clone()),
        ),
        (
            PText::from_text("chosen"),
            JsonValue::Str(entry.chosen.clone()),
        ),
        (
            PText::from_text("confidence"),
            JsonValue::Num(entry.confidence.clone()),
        ),
        (
            PText::from_text("reason"),
            JsonValue::Str(entry.reason.clone()),
        ),
    ])
}
