//! The adjudication seat over decision models, and its readers.
//!
//! The seat presents one recorded close call to a decision
//! model in one stated summary form, reads the envelope the
//! call returns into the adjudication entry DH-5 validates, and
//! records that entry into the race as data. The readers are
//! pure functions of their inputs and make no call themselves.
//! The obligations this module carries are DH-7 and DH-8 in
//! CONTRACT.md, in the Round 3 section. The Python module of
//! the same name is the behavioral spec.

use crate::json::{self, JsonValue};
use crate::pynum::Num;
use crate::pytext::PText;
use crate::races::{format_seconds, Adjudication, Race, SuiteResult};

/// Return whether a string holds a newline or a carriage
/// return. The seat module carries its own copy of this reading
/// in the Python spec, and the port keeps the copy here, as the
/// registry records.
fn holds_line_break(text: &PText) -> bool {
    text.holds_line_break()
}

/// Return whether a race can be seated under DH-7. The seat
/// presents the two attempts under their model labels, so a
/// race whose attempts carry the same model label has no seat.
pub fn has_seat(race: &Race) -> bool {
    race.attempts[0].model != race.attempts[1].model
}

/// Return the summary text the seat presents for one race. The
/// form is the one DH-7 states.
pub fn attempt_summary(race: &Race) -> String {
    let mut parts = vec![format!("task {}", race.task.display_line())];
    for attempt in &race.attempts {
        let finish = if attempt.result == SuiteResult::Unfinished {
            "never finished".to_string()
        } else {
            format!(
                "finished in {} seconds",
                format_seconds(attempt.finished_at.as_ref().expect("a finish"))
            )
        };
        let spend = match &attempt.tokens {
            None => "tokens not reported".to_string(),
            Some(tokens) => format!("{} tokens", tokens),
        };
        parts.push(format!(
            "attempt, model {}, environment {}, result {}, {}, {}",
            attempt.model.display_line(),
            attempt.environment.display_line(),
            attempt.result.as_str(),
            finish,
            spend
        ));
        parts.push(format!("patch of {}", attempt.model.display_line()));
        parts.push(attempt.patch.display());
    }
    parts.join("\n")
}

const INSTRUCTION_LINE: &str = "Adjudicate the recorded close call race below as one judge, choosing exactly one of the two attempts as the winner.";

const REPLY_LINE: &str = "Reply with one JSON object and nothing else. The object holds the field choice, exactly one of the two model labels, the field confidence, your confidence in the choice as a number from 0 to 1, and the field probabilities, an object holding one number for each of the two model labels.";

/// Return the prompt the chat route sends for one race.
pub fn chat_prompt(race: &Race) -> String {
    format!(
        "{}\n\n{}\n\n{}",
        INSTRUCTION_LINE,
        attempt_summary(race),
        REPLY_LINE
    )
}

/// Render a confidence figure as its shortest round trip text.
/// An integer confidence renders with no decimal point, and a
/// fractional one renders as the shortest decimal text that
/// parses back to the same value.
fn render_confidence(confidence: &Num) -> String {
    confidence.plain_text()
}

/// Return the envelope model string, or None when unreadable.
fn read_model(envelope: &JsonValue) -> Option<PText> {
    envelope.as_object()?;

    let model = envelope.get("model")?.as_text()?;
    if model.is_empty() {
        return None;
    }
    if model.holds_unpaired_surrogate() {
        return None;
    }
    if holds_line_break(model) {
        return None;
    }
    Some(model.clone())
}

/// Build the adjudication entry from one decision, or None. The
/// decision must hold a choice naming one attempt of the race,
/// a finite confidence inside the range from zero to one, and
/// probabilities holding a finite number for each of the two
/// model labels, with every probability a finite number.
fn entry_from_decision(race: &Race, model: &PText, decision: &JsonValue) -> Option<Adjudication> {
    decision.as_object()?;

    let labels: Vec<&PText> = race.attempts.iter().map(|attempt| &attempt.model).collect();
    let choice = decision.get("choice")?.as_text()?;
    if choice.holds_unpaired_surrogate() {
        return None;
    }
    if holds_line_break(choice) {
        return None;
    }
    if !labels.contains(&choice) {
        return None;
    }
    let confidence_value = decision.get("confidence")?;
    if !crate::races::is_number(confidence_value) {
        return None;
    }
    let confidence = confidence_value.as_num().expect("a number");
    if !confidence.is_finite_number() {
        return None;
    }
    if crate::pynum::cmp_int(confidence, 0) == std::cmp::Ordering::Less
        || crate::pynum::cmp_int(confidence, 1) == std::cmp::Ordering::Greater
    {
        return None;
    }
    let probabilities = decision.get("probabilities")?;
    let probability_pairs = probabilities.as_object()?;
    for (_, value) in probability_pairs {
        if !crate::races::is_number(value) {
            return None;
        }
        let num = value.as_num().expect("a number");
        if !num.is_finite_number() {
            return None;
        }
    }
    for label in &labels {
        if !probability_pairs.iter().any(|(key, _)| key == *label) {
            return None;
        }
    }
    let reason = PText::from_text(&format!(
        "{} chose {} with confidence {}",
        model.display(),
        choice.display(),
        render_confidence(confidence)
    ));
    Some(Adjudication {
        adjudicator: model.clone(),
        chosen: choice.clone(),
        confidence: confidence.clone(),
        reason,
    })
}

/// Return the entry from one Jev envelope, or None. The
/// envelope is the Typesafe System One response as parsed JSON,
/// holding the model string and an answers object whose
/// decision object holds the decision.
pub fn entry_from_jev_envelope(race: &Race, envelope: &JsonValue) -> Option<Adjudication> {
    let model = read_model(envelope)?;
    let answers = envelope.get("answers")?;
    answers.as_object()?;

    let decision = answers.get("decision").cloned().unwrap_or(JsonValue::Null);
    entry_from_decision(race, &model, &decision)
}

/// Return the entry from one chat envelope, or None. The
/// envelope is an OpenAI compatible chat completion as parsed
/// JSON, holding the model string and a choices list whose
/// first entry holds a message whose content is one JSON
/// object holding the decision. Content that does not parse as
/// one JSON object is a failed call.
pub fn entry_from_chat_envelope(race: &Race, envelope: &JsonValue) -> Option<Adjudication> {
    let model = read_model(envelope)?;
    let choices = envelope.get("choices")?.as_array()?;
    if choices.is_empty() {
        return None;
    }
    let first = &choices[0];
    first.as_object()?;
    let message = first.get("message")?;
    message.as_object()?;

    let content = message.get("content")?.as_text()?;
    let decision = json::parse_units(content.units()).ok()?;
    entry_from_decision(race, &model, &decision)
}

/// Return a new race holding the entry under adjudication. The
/// input race is never mutated.
pub fn record_adjudication(race: &Race, entry: &Adjudication) -> Race {
    let mut recorded = race.clone();
    recorded.adjudication = Some(entry.clone());
    recorded
}

/// Return the race with the envelope entry recorded, or None.
/// The route word is jev or openai, naming the envelope shape.
/// None stands for a failed call, which records no entry. An
/// unknown route word is a programming error and panics,
/// because it is not malformed content, as the Python reading
/// raises on it.
pub fn adjudicated_race(race: &Race, envelope: &JsonValue, route: &str) -> Option<Race> {
    let entry = match route {
        "jev" => entry_from_jev_envelope(race, envelope),
        "openai" => entry_from_chat_envelope(race, envelope),
        _ => panic!("unknown adjudication route '{}'", route),
    };
    entry.map(|entry| record_adjudication(race, &entry))
}
