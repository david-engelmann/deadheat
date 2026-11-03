//! Pins for the adjudication seat and its envelope readers,
//! DH-7 in CONTRACT.md, translated from the Python corpus.
//!
//! Every pin here runs offline over synthetic envelopes built
//! in the test. The summary and prompt pins hold the exact
//! stated forms from DH-7, the reader pins walk the failure
//! enumeration in DH-7, and the recording pins hold the entry
//! path into the race record and its reading under DH-5.
//!
//! One Python pin has no Rust test form. The pin that the seat
//! number test is the races number test (an identity check on
//! two module attributes in Python) is represented by
//! construction in the port. The seats module calls the one
//! shared is_number definition in the races module, so there
//! is no second predicate to drift.

mod shared;

use deadheat::json::JsonValue;
use deadheat::races::{decide_winner, load_race_data, race_to_json, Race};
use deadheat::seats::*;
use shared::*;

const PATCH_A: &str = "--- a/retry.py\n+++ b/retry.py\n@@ -1,1 +1,1 @@\n-budget = 5\n+budget = 3\n";
const PATCH_B: &str = "--- a/retry.py\n+++ b/retry.py\n@@ -1,1 +1,1 @@\n-budget = 5\n+budget = 2\n";

fn race_data() -> JsonValue {
    obj(vec![
        ("task", s("tighten the retry budget")),
        (
            "attempts",
            arr(vec![
                obj(vec![
                    ("model", s("prime-1")),
                    ("environment", s("home worktree")),
                    ("result", s("green")),
                    ("finished_at", f(300.0)),
                    ("tokens", i(88100)),
                    ("patch", s(PATCH_A)),
                ]),
                obj(vec![
                    ("model", s("shadow-1")),
                    ("environment", s("shadow worktree")),
                    ("result", s("green")),
                    ("finished_at", f(288.0)),
                    ("tokens", i(91400)),
                    ("patch", s(PATCH_B)),
                ]),
            ]),
        ),
    ])
}

fn race() -> Race {
    let (race, defects) = load_race_data(&race_data());
    assert_eq!(defects, Vec::<String>::new());
    race.expect("a clean race")
}

fn jev_envelope() -> JsonValue {
    jev_envelope_with("shadow-1", f(0.85))
}

fn jev_envelope_with(choice: &str, confidence: JsonValue) -> JsonValue {
    obj(vec![
        ("model", s("jev-1.13.0")),
        (
            "answers",
            obj(vec![(
                "decision",
                obj(vec![
                    ("type", s("choice")),
                    ("choice", s(choice)),
                    ("confidence", confidence),
                    (
                        "probabilities",
                        obj(vec![("prime-1", f(0.15)), ("shadow-1", f(0.85))]),
                    ),
                ]),
            )]),
        ),
        (
            "usage",
            obj(vec![("input_tokens", i(437)), ("output_tokens", i(37))]),
        ),
    ])
}

fn chat_envelope() -> JsonValue {
    chat_envelope_with("prime-1", 0.9)
}

fn chat_envelope_with(choice: &str, confidence: f64) -> JsonValue {
    let decision = obj(vec![
        ("choice", s(choice)),
        ("confidence", f(confidence)),
        (
            "probabilities",
            obj(vec![("prime-1", f(0.9)), ("shadow-1", f(0.1))]),
        ),
    ]);
    obj(vec![
        ("model", s("openai/gpt-6-luna")),
        (
            "choices",
            arr(vec![obj(vec![(
                "message",
                obj(vec![
                    ("role", s("assistant")),
                    ("content", s(&deadheat::json::dumps(&decision, None, true))),
                ]),
            )])]),
        ),
        (
            "usage",
            obj(vec![
                ("prompt_tokens", i(227)),
                ("completion_tokens", i(101)),
            ]),
        ),
    ])
}

#[test]
fn the_summary_states_the_stated_form() {
    let expected = format!(
        "task tighten the retry budget\nattempt, model prime-1, environment home worktree, result green, finished in 300 seconds, 88100 tokens\npatch of prime-1\n{PATCH_A}\nattempt, model shadow-1, environment shadow worktree, result green, finished in 288 seconds, 91400 tokens\npatch of shadow-1\n{PATCH_B}"
    );
    assert_eq!(attempt_summary(&race()), expected);
}

#[test]
fn the_summary_renders_fractional_seconds_exactly() {
    let mut data = race_data();
    data = set_path(&data, &["attempts", "0", "finished_at"], f(505.25));
    data = set_path(&data, &["attempts", "1", "finished_at"], f(398.5));
    let (race, defects) = load_race_data(&data);
    assert_eq!(defects, Vec::<String>::new());
    let summary = attempt_summary(&race.expect("a race"));
    assert!(summary.contains("finished in 505.25 seconds"));
    assert!(summary.contains("finished in 398.5 seconds"));
}

#[test]
fn the_summary_states_an_unfinished_attempt() {
    let mut data = race_data();
    let unfinished = obj(vec![
        ("model", s("shadow-1")),
        ("environment", s("shadow worktree")),
        ("result", s("unfinished")),
        ("tokens", JsonValue::Null),
        ("patch", s(PATCH_B)),
    ]);
    data = set_path(&data, &["attempts", "1"], unfinished);
    let (race, defects) = load_race_data(&data);
    assert_eq!(defects, Vec::<String>::new());
    let summary = attempt_summary(&race.expect("a race"));
    assert!(summary.contains(
        "attempt, model shadow-1, environment shadow worktree, result unfinished, never finished, tokens not reported"
    ));
}

#[test]
fn the_summary_is_a_pure_function_of_the_race() {
    let race = race();
    let snapshot = race.clone();
    assert_eq!(attempt_summary(&race), attempt_summary(&race));
    assert_eq!(race, snapshot);
}

#[test]
fn the_chat_prompt_wraps_the_summary_as_stated() {
    let race = race();
    let expected = format!(
        "Adjudicate the recorded close call race below as one judge, choosing exactly one of the two attempts as the winner.\n\n{}\n\nReply with one JSON object and nothing else. The object holds the field choice, exactly one of the two model labels, the field confidence, your confidence in the choice as a number from 0 to 1, and the field probabilities, an object holding one number for each of the two model labels.",
        attempt_summary(&race)
    );
    assert_eq!(chat_prompt(&race), expected);
}

#[test]
fn a_race_with_two_model_labels_has_a_seat() {
    assert!(has_seat(&race()));
}

#[test]
fn a_race_with_one_model_label_has_no_seat() {
    let data = set_path(&race_data(), &["attempts", "1", "model"], s("prime-1"));
    let (race, defects) = load_race_data(&data);
    assert_eq!(defects, Vec::<String>::new());
    assert!(!has_seat(&race.expect("a race")));
}

#[test]
fn the_jev_reader_builds_the_entry_by_the_stated_construction() {
    let entry = entry_from_jev_envelope(&race(), &jev_envelope()).expect("an entry");
    assert_eq!(entry.adjudicator.display(), "jev-1.13.0");
    assert_eq!(entry.chosen.display(), "shadow-1");
    assert_eq!(entry.confidence, deadheat::pynum::Num::from_f64(0.85));
    assert_eq!(
        entry.reason.display(),
        "jev-1.13.0 chose shadow-1 with confidence 0.85"
    );
}

#[test]
fn the_jev_reader_renders_an_integer_confidence_plainly() {
    let entry =
        entry_from_jev_envelope(&race(), &jev_envelope_with("shadow-1", i(1))).expect("an entry");
    assert_eq!(entry.confidence, deadheat::pynum::Num::from_i64(1));
    assert!(entry.reason.display().ends_with("with confidence 1"));
}

#[test]
fn the_reader_records_a_below_threshold_entry() {
    let entry =
        entry_from_jev_envelope(&race(), &jev_envelope_with("shadow-1", f(0.3))).expect("an entry");
    assert_eq!(entry.confidence, deadheat::pynum::Num::from_f64(0.3));
}

#[test]
fn the_reader_reads_probabilities_for_shape_only() {
    let envelope = set_path(
        &jev_envelope(),
        &["answers", "decision", "probabilities"],
        obj(vec![
            ("prime-1", f(0.99)),
            ("shadow-1", f(0.01)),
            ("a third model", f(0.5)),
        ]),
    );
    let entry = entry_from_jev_envelope(&race(), &envelope).expect("an entry");
    assert_eq!(entry.chosen.display(), "shadow-1");
}

// The jev failure enumeration, one test per mutation in the
// Python parametrized pin.

fn jev_fails(envelope: &JsonValue) -> bool {
    entry_from_jev_envelope(&race(), envelope).is_none()
}

#[test]
fn jev_fails_when_the_model_is_null() {
    assert!(jev_fails(&set(&jev_envelope(), "model", JsonValue::Null)));
}

#[test]
fn jev_fails_when_the_model_is_empty() {
    assert!(jev_fails(&set(&jev_envelope(), "model", s(""))));
}

#[test]
fn jev_fails_when_the_model_is_a_number() {
    assert!(jev_fails(&set(&jev_envelope(), "model", i(7))));
}

#[test]
fn jev_fails_when_the_model_holds_a_newline() {
    assert!(jev_fails(&set(&jev_envelope(), "model", s("jev\n1.13.0"))));
}

#[test]
fn jev_fails_when_the_model_holds_a_surrogate() {
    let text = "{\"model\": \"jev\\ud800\", \"answers\": {\"decision\": {\"type\": \"choice\", \"choice\": \"shadow-1\", \"confidence\": 0.85, \"probabilities\": {\"prime-1\": 0.15, \"shadow-1\": 0.85}}}, \"usage\": {\"input_tokens\": 437, \"output_tokens\": 37}}";
    let envelope = deadheat::json::parse(text).expect("an envelope");
    assert!(jev_fails(&envelope));
}

#[test]
fn jev_fails_without_answers() {
    assert!(jev_fails(&without(&jev_envelope(), &["answers"])));
}

#[test]
fn jev_fails_when_answers_is_a_list() {
    assert!(jev_fails(&set(&jev_envelope(), "answers", arr(vec![]))));
}

#[test]
fn jev_fails_without_a_decision() {
    assert!(jev_fails(&remove_path(
        &jev_envelope(),
        &["answers", "decision"]
    )));
}

#[test]
fn jev_fails_when_the_decision_is_a_string() {
    assert!(jev_fails(&set_path(
        &jev_envelope(),
        &["answers", "decision"],
        s("shadow-1")
    )));
}

#[test]
fn jev_fails_without_a_choice() {
    assert!(jev_fails(&remove_path(
        &jev_envelope(),
        &["answers", "decision", "choice"]
    )));
}

#[test]
fn jev_fails_when_the_choice_names_no_attempt() {
    assert!(jev_fails(&set_path(
        &jev_envelope(),
        &["answers", "decision", "choice"],
        s("third-1")
    )));
}

#[test]
fn jev_fails_when_the_choice_holds_a_newline() {
    assert!(jev_fails(&set_path(
        &jev_envelope(),
        &["answers", "decision", "choice"],
        s("prime\n1")
    )));
}

#[test]
fn jev_fails_when_the_choice_holds_a_surrogate() {
    let text = "{\"model\": \"jev-1.13.0\", \"answers\": {\"decision\": {\"type\": \"choice\", \"choice\": \"prime\\ud8001\", \"confidence\": 0.85, \"probabilities\": {\"prime-1\": 0.15, \"shadow-1\": 0.85}}}, \"usage\": {\"input_tokens\": 437, \"output_tokens\": 37}}";
    let envelope = deadheat::json::parse(text).expect("an envelope");
    assert!(jev_fails(&envelope));
}

#[test]
fn jev_fails_without_a_confidence() {
    assert!(jev_fails(&remove_path(
        &jev_envelope(),
        &["answers", "decision", "confidence"]
    )));
}

#[test]
fn jev_fails_when_the_confidence_is_a_boolean() {
    assert!(jev_fails(&set_path(
        &jev_envelope(),
        &["answers", "decision", "confidence"],
        JsonValue::Bool(true)
    )));
}

#[test]
fn jev_fails_when_the_confidence_is_a_string() {
    assert!(jev_fails(&set_path(
        &jev_envelope(),
        &["answers", "decision", "confidence"],
        s("0.9")
    )));
}

#[test]
fn jev_fails_when_the_confidence_is_above_one() {
    assert!(jev_fails(&set_path(
        &jev_envelope(),
        &["answers", "decision", "confidence"],
        f(1.5)
    )));
}

#[test]
fn jev_fails_when_the_confidence_is_below_zero() {
    assert!(jev_fails(&set_path(
        &jev_envelope(),
        &["answers", "decision", "confidence"],
        f(-0.1)
    )));
}

#[test]
fn jev_fails_without_probabilities() {
    assert!(jev_fails(&remove_path(
        &jev_envelope(),
        &["answers", "decision", "probabilities"]
    )));
}

#[test]
fn jev_fails_when_probabilities_is_a_list() {
    assert!(jev_fails(&set_path(
        &jev_envelope(),
        &["answers", "decision", "probabilities"],
        arr(vec![])
    )));
}

#[test]
fn jev_fails_when_probabilities_miss_a_label() {
    assert!(jev_fails(&set_path(
        &jev_envelope(),
        &["answers", "decision", "probabilities"],
        obj(vec![("prime-1", f(0.5))])
    )));
}

#[test]
fn jev_fails_when_a_probability_is_a_string() {
    assert!(jev_fails(&set_path(
        &jev_envelope(),
        &["answers", "decision", "probabilities"],
        obj(vec![("prime-1", f(0.5)), ("shadow-1", s("high"))])
    )));
}

#[test]
fn the_jev_reader_fails_a_non_object_envelope() {
    assert!(entry_from_jev_envelope(&race(), &arr(vec![s("not"), s("an"), s("object")])).is_none());
    assert!(entry_from_jev_envelope(&race(), &JsonValue::Null).is_none());
}

#[test]
fn the_jev_reader_fails_a_non_finite_figure() {
    for figure in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let by_confidence = jev_envelope_with("shadow-1", f(figure));
        assert!(entry_from_jev_envelope(&race(), &by_confidence).is_none());
        let by_probability = set_path(
            &jev_envelope(),
            &["answers", "decision", "probabilities", "prime-1"],
            f(figure),
        );
        assert!(entry_from_jev_envelope(&race(), &by_probability).is_none());
    }
}

#[test]
fn the_chat_reader_builds_the_entry() {
    let entry = entry_from_chat_envelope(&race(), &chat_envelope()).expect("an entry");
    assert_eq!(entry.adjudicator.display(), "openai/gpt-6-luna");
    assert_eq!(entry.chosen.display(), "prime-1");
    assert_eq!(entry.confidence, deadheat::pynum::Num::from_f64(0.9));
    assert_eq!(
        entry.reason.display(),
        "openai/gpt-6-luna chose prime-1 with confidence 0.9"
    );
}

#[test]
fn the_chat_reader_accepts_a_choice_either_way() {
    let entry =
        entry_from_chat_envelope(&race(), &chat_envelope_with("shadow-1", 0.5)).expect("an entry");
    assert_eq!(entry.chosen.display(), "shadow-1");
}

#[test]
fn the_chat_reader_fails_an_unreadable_reply() {
    let contents = [
        "I choose prime-1",
        "[\"prime-1\"]",
        "\"prime-1\"",
        "{\"choice\": \"third-1\", \"confidence\": 0.9, \"probabilities\": {\"prime-1\": 0.5, \"shadow-1\": 0.5}}",
        "{\"choice\": \"prime-1\", \"confidence\": 2, \"probabilities\": {\"prime-1\": 0.5, \"shadow-1\": 0.5}}",
    ];
    for content in contents {
        let envelope = set_path(
            &chat_envelope(),
            &["choices", "0", "message", "content"],
            s(content),
        );
        assert!(
            entry_from_chat_envelope(&race(), &envelope).is_none(),
            "{content}"
        );
    }
}

fn chat_fails(envelope: &JsonValue) -> bool {
    entry_from_chat_envelope(&race(), envelope).is_none()
}

#[test]
fn chat_fails_when_choices_is_empty() {
    assert!(chat_fails(&set(&chat_envelope(), "choices", arr(vec![]))));
}

#[test]
fn chat_fails_without_choices() {
    assert!(chat_fails(&without(&chat_envelope(), &["choices"])));
}

#[test]
fn chat_fails_when_the_first_choice_is_empty() {
    assert!(chat_fails(&set(
        &chat_envelope(),
        "choices",
        arr(vec![obj(vec![])])
    )));
}

#[test]
fn chat_fails_when_the_message_is_null() {
    assert!(chat_fails(&set_path(
        &chat_envelope(),
        &["choices", "0", "message"],
        JsonValue::Null
    )));
}

#[test]
fn chat_fails_when_the_content_is_an_object() {
    assert!(chat_fails(&set_path(
        &chat_envelope(),
        &["choices", "0", "message", "content"],
        obj(vec![("choice", s("x"))])
    )));
}

#[test]
fn chat_fails_when_the_model_is_empty() {
    assert!(chat_fails(&set(&chat_envelope(), "model", s(""))));
}

#[test]
fn recording_an_entry_never_mutates_the_race() {
    let race = race();
    let entry = entry_from_jev_envelope(&race, &jev_envelope()).expect("an entry");
    let recorded = record_adjudication(&race, &entry);
    assert!(race.adjudication.is_none());
    assert_eq!(recorded.adjudication.as_ref(), Some(&entry));
    let (loaded, defects) = load_race_data(&race_to_json(&recorded));
    assert_eq!(defects, Vec::<String>::new());
    assert_eq!(loaded.expect("a race"), recorded);
}

#[test]
fn a_recorded_deciding_entry_decides_under_dh_5() {
    let race = race();
    let entry =
        entry_from_jev_envelope(&race, &jev_envelope_with("shadow-1", f(0.85))).expect("an entry");
    let recorded = record_adjudication(&race, &entry);
    let (winner, _) = decide_winner(&recorded);
    assert_eq!(
        recorded.attempts[winner.expect("a winner")].model.display(),
        "shadow-1"
    );
}

#[test]
fn a_recorded_low_entry_abstains_under_dh_5() {
    let race = race();
    let entry =
        entry_from_jev_envelope(&race, &jev_envelope_with("shadow-1", f(0.3))).expect("an entry");
    let recorded = record_adjudication(&race, &entry);
    let (winner, _) = decide_winner(&recorded);
    assert_eq!(winner, None);
}

#[test]
fn adjudicated_race_records_through_either_route() {
    let race = race();
    let by_jev = adjudicated_race(&race, &jev_envelope(), "jev").expect("a race");
    assert_eq!(
        by_jev.adjudication.expect("an entry").adjudicator.display(),
        "jev-1.13.0"
    );
    let by_chat = adjudicated_race(&race, &chat_envelope(), "openai").expect("a race");
    assert_eq!(
        by_chat
            .adjudication
            .expect("an entry")
            .adjudicator
            .display(),
        "openai/gpt-6-luna"
    );
}

#[test]
fn adjudicated_race_returns_none_for_a_failed_call() {
    let envelope = set_path(
        &jev_envelope(),
        &["answers", "decision", "confidence"],
        i(7),
    );
    assert!(adjudicated_race(&race(), &envelope, "jev").is_none());
}

#[test]
#[should_panic]
fn adjudicated_race_rejects_an_unknown_route_word() {
    let _ = adjudicated_race(&race(), &jev_envelope(), "carrier-pigeon");
}
