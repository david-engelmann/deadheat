//! Pins for DH-1 to DH-4 in the full form section of
//! CONTRACT.md, translated from the Python corpus.
//!
//! Every defect phrase the loader can name is pinned against
//! the malformed shape that draws it, the winner rule is pinned
//! in each of its outcomes, and the standings update is pinned
//! as exact values.

mod shared;

use deadheat::json::JsonValue;
use deadheat::pynum::Num;
use deadheat::pytext::PText;
use deadheat::races::*;
use shared::*;

fn valid_attempt() -> JsonValue {
    obj(vec![
        ("model", s("model-a")),
        ("environment", s("lane one")),
        ("result", s("green")),
        ("finished_at", f(120.5)),
        ("tokens", i(4500)),
        ("patch", s("diff text")),
    ])
}

fn other_attempt() -> JsonValue {
    obj(vec![
        ("model", s("model-b")),
        ("environment", s("lane two")),
        ("result", s("red")),
        ("finished_at", f(90.25)),
        ("tokens", JsonValue::Null),
        ("patch", s("other diff")),
    ])
}

fn valid_race() -> JsonValue {
    obj(vec![
        ("task", s("fix the parser")),
        ("attempts", arr(vec![valid_attempt(), other_attempt()])),
    ])
}

fn attempt_with(base: JsonValue, pairs: Vec<(&str, JsonValue)>) -> JsonValue {
    pairs
        .into_iter()
        .fold(base, |value, (key, item)| set(&value, key, item))
}

fn race_with_attempt(index: usize, attempt: JsonValue) -> JsonValue {
    let mut attempts = vec![valid_attempt(), other_attempt()];
    attempts[index] = attempt;
    obj(vec![
        ("task", s("fix the parser")),
        ("attempts", arr(attempts)),
    ])
}

fn defects_of(data: &JsonValue) -> Vec<String> {
    load_race_data(data).1
}

fn loaded(data: &JsonValue) -> Race {
    let (race, defects) = load_race_data(data);
    assert_eq!(defects, Vec::<String>::new());
    race.expect("a clean race")
}

fn winner_model(race: &Race) -> String {
    let (winner, _) = decide_winner(race);
    race.attempts[winner.expect("a winner")].model.display()
}

fn loser_model(race: &Race) -> String {
    let (_, loser) = decide_winner(race);
    race.attempts[loser.expect("a loser")].model.display()
}

// The loader, DH-1

#[test]
fn a_valid_race_loads_with_its_shape_kept() {
    let race = loaded(&valid_race());
    assert_eq!(race.task.display(), "fix the parser");
    assert_eq!(race.attempts[0].model.display(), "model-a");
    assert_eq!(race.attempts[0].finished_at, Some(Num::from_f64(120.5)));
    assert_eq!(race.attempts[1].tokens, None);
}

#[test]
fn fields_the_shape_does_not_name_are_ignored() {
    let mut data = set(&valid_race(), "extra", s("ignored"));
    let attempts = arr(vec![
        set(&valid_attempt(), "surprise", i(1)),
        other_attempt(),
    ]);
    data = set(&data, "attempts", attempts);
    assert_eq!(defects_of(&data), Vec::<String>::new());
}

#[test]
fn text_that_is_not_json_draws_the_not_valid_json_defect() {
    let (race, defects) = load_race_text("{not json");
    assert!(race.is_none());
    assert_eq!(defects, vec!["input is not valid JSON"]);
}

#[test]
fn deeply_nested_input_never_raises() {
    let text = "[".repeat(100000) + &"]".repeat(100000);
    let (race, defects) = load_race_text(&text);
    assert!(race.is_none());
    assert!(!defects.is_empty());
}

#[test]
fn a_non_object_root_draws_the_not_an_object_defect() {
    let (race, defects) = load_race_text("[1, 2]");
    assert!(race.is_none());
    assert_eq!(defects, vec!["input is not a JSON object"]);
}

#[test]
fn missing_task_is_named() {
    assert_eq!(
        defects_of(&without(&valid_race(), &["task"])),
        vec!["missing field task"]
    );
}

#[test]
fn a_task_of_the_wrong_type_is_named() {
    let data = set(&valid_race(), "task", i(7));
    assert_eq!(defects_of(&data), vec!["field task has the wrong type"]);
}

#[test]
fn an_empty_task_is_named() {
    let data = set(&valid_race(), "task", s(""));
    assert_eq!(defects_of(&data), vec!["field task is empty"]);
}

#[test]
fn a_task_holding_an_unpaired_surrogate_is_named() {
    // The surrogate arrives through JSON text, as it does in
    // the Python pin through the dumps of a dict.
    let text = "{\"task\": \"bad\\ud800x\", \"attempts\": []}";
    let (_, defects) = load_race_text(text);
    assert!(defects.contains(&"field task holds an unpaired surrogate".to_string()));
}

#[test]
fn a_task_holding_a_newline_is_named() {
    let data = set(&valid_race(), "task", s("two\nlines"));
    assert_eq!(defects_of(&data), vec!["field task holds a newline"]);
}

#[test]
fn a_task_holding_a_carriage_return_is_named() {
    let data = set(&valid_race(), "task", s("two\rlines"));
    assert_eq!(defects_of(&data), vec!["field task holds a newline"]);
}

#[test]
fn missing_attempts_is_named() {
    assert_eq!(
        defects_of(&without(&valid_race(), &["attempts"])),
        vec!["missing field attempts"]
    );
}

#[test]
fn attempts_of_the_wrong_type_is_named() {
    let data = set(&valid_race(), "attempts", s("two"));
    assert_eq!(defects_of(&data), vec!["field attempts has the wrong type"]);
}

#[test]
fn one_attempt_is_named_as_not_exactly_two() {
    let data = obj(vec![
        ("task", s("fix the parser")),
        ("attempts", arr(vec![valid_attempt()])),
    ]);
    assert_eq!(
        defects_of(&data),
        vec!["field attempts must hold exactly two attempts"]
    );
}

#[test]
fn three_attempts_are_named_as_not_exactly_two() {
    let data = obj(vec![
        ("task", s("fix the parser")),
        (
            "attempts",
            arr(vec![
                valid_attempt(),
                other_attempt(),
                set(&valid_attempt(), "model", s("model-c")),
            ]),
        ),
    ]);
    assert_eq!(
        defects_of(&data),
        vec!["field attempts must hold exactly two attempts"]
    );
}

#[test]
fn an_attempt_that_is_not_an_object_is_named() {
    let data = race_with_attempt(0, s("fast"));
    assert_eq!(defects_of(&data), vec!["attempt 0 is not an object"]);
}

#[test]
fn a_missing_model_is_named_with_its_position() {
    let data = race_with_attempt(1, without(&other_attempt(), &["model"]));
    assert_eq!(defects_of(&data), vec!["attempt 1 is missing field model"]);
}

#[test]
fn a_model_of_the_wrong_type_is_named() {
    let data = race_with_attempt(0, set(&valid_attempt(), "model", i(3)));
    assert_eq!(
        defects_of(&data),
        vec!["attempt 0 field model has the wrong type"]
    );
}

#[test]
fn a_missing_environment_is_named_with_its_position() {
    let data = race_with_attempt(0, without(&valid_attempt(), &["environment"]));
    assert_eq!(
        defects_of(&data),
        vec!["attempt 0 is missing field environment"]
    );
}

#[test]
fn an_environment_of_the_wrong_type_is_named() {
    let data = race_with_attempt(0, set(&valid_attempt(), "environment", i(3)));
    assert_eq!(
        defects_of(&data),
        vec!["attempt 0 field environment has the wrong type"]
    );
}

#[test]
fn an_empty_environment_is_named() {
    let data = race_with_attempt(0, set(&valid_attempt(), "environment", s("")));
    assert_eq!(
        defects_of(&data),
        vec!["attempt 0 field environment is empty"]
    );
}

#[test]
fn an_environment_holding_an_unpaired_surrogate_is_named() {
    let data = "{\"task\": \"t\", \"attempts\": [{\"model\": \"model-a\", \"environment\": \"lane\\ud800\", \"result\": \"green\", \"finished_at\": 1, \"tokens\": 1, \"patch\": \"p\"}, {\"model\": \"model-b\", \"environment\": \"lane two\", \"result\": \"red\", \"finished_at\": 2, \"tokens\": null, \"patch\": \"q\"}]}";
    let (_, defects) = load_race_text(data);
    assert_eq!(
        defects,
        vec!["attempt 0 field environment holds an unpaired surrogate"]
    );
}

#[test]
fn a_model_holding_a_newline_is_named() {
    let data = race_with_attempt(0, set(&valid_attempt(), "model", s("two\nlines")));
    assert_eq!(
        defects_of(&data),
        vec!["attempt 0 field model holds a newline"]
    );
}

#[test]
fn a_model_holding_a_carriage_return_is_named_with_its_position() {
    let data = race_with_attempt(1, set(&other_attempt(), "model", s("two\rlines")));
    assert_eq!(
        defects_of(&data),
        vec!["attempt 1 field model holds a newline"]
    );
}

#[test]
fn an_environment_holding_a_newline_is_named() {
    let data = race_with_attempt(0, set(&valid_attempt(), "environment", s("two\nlines")));
    assert_eq!(
        defects_of(&data),
        vec!["attempt 0 field environment holds a newline"]
    );
}

#[test]
fn an_environment_holding_a_carriage_return_is_named_with_its_position() {
    let data = race_with_attempt(1, set(&other_attempt(), "environment", s("two\rlines")));
    assert_eq!(
        defects_of(&data),
        vec!["attempt 1 field environment holds a newline"]
    );
}

#[test]
fn an_empty_model_is_named() {
    let data = race_with_attempt(0, set(&valid_attempt(), "model", s("")));
    assert_eq!(defects_of(&data), vec!["attempt 0 field model is empty"]);
}

#[test]
fn a_model_holding_an_unpaired_surrogate_is_named() {
    let data = "{\"task\": \"t\", \"attempts\": [{\"model\": \"m\\ud800\", \"environment\": \"lane one\", \"result\": \"green\", \"finished_at\": 1, \"tokens\": 1, \"patch\": \"p\"}, {\"model\": \"model-b\", \"environment\": \"lane two\", \"result\": \"red\", \"finished_at\": 2, \"tokens\": null, \"patch\": \"q\"}]}";
    let (_, defects) = load_race_text(data);
    assert_eq!(
        defects,
        vec!["attempt 0 field model holds an unpaired surrogate"]
    );
}

#[test]
fn a_missing_result_is_named() {
    let data = race_with_attempt(0, without(&valid_attempt(), &["result"]));
    assert_eq!(defects_of(&data), vec!["attempt 0 is missing field result"]);
}

#[test]
fn a_result_of_the_wrong_type_is_named() {
    let data = race_with_attempt(0, set(&valid_attempt(), "result", i(1)));
    assert_eq!(
        defects_of(&data),
        vec!["attempt 0 field result has the wrong type"]
    );
}

#[test]
fn a_result_outside_the_three_words_is_named() {
    let data = race_with_attempt(0, set(&valid_attempt(), "result", s("blue")));
    assert_eq!(
        defects_of(&data),
        vec!["attempt 0 field result is not a suite result"]
    );
}

#[test]
fn missing_tokens_is_named() {
    let data = race_with_attempt(0, without(&valid_attempt(), &["tokens"]));
    assert_eq!(defects_of(&data), vec!["attempt 0 is missing field tokens"]);
}

#[test]
fn fractional_tokens_are_named_as_the_wrong_type() {
    let data = race_with_attempt(0, set(&valid_attempt(), "tokens", f(1.5)));
    assert_eq!(
        defects_of(&data),
        vec!["attempt 0 field tokens has the wrong type"]
    );
}

#[test]
fn boolean_tokens_are_named_as_the_wrong_type() {
    let data = race_with_attempt(0, set(&valid_attempt(), "tokens", JsonValue::Bool(true)));
    assert_eq!(
        defects_of(&data),
        vec!["attempt 0 field tokens has the wrong type"]
    );
}

#[test]
fn negative_tokens_are_named() {
    let data = race_with_attempt(0, set(&valid_attempt(), "tokens", i(-1)));
    assert_eq!(
        defects_of(&data),
        vec!["attempt 0 field tokens is negative"]
    );
}

#[test]
fn zero_tokens_load_clean() {
    let data = race_with_attempt(0, set(&valid_attempt(), "tokens", i(0)));
    assert_eq!(defects_of(&data), Vec::<String>::new());
}

#[test]
fn missing_patch_is_named() {
    let data = race_with_attempt(1, without(&other_attempt(), &["patch"]));
    assert_eq!(defects_of(&data), vec!["attempt 1 is missing field patch"]);
}

#[test]
fn a_patch_of_the_wrong_type_is_named() {
    let data = race_with_attempt(0, set(&valid_attempt(), "patch", i(9)));
    assert_eq!(
        defects_of(&data),
        vec!["attempt 0 field patch has the wrong type"]
    );
}

#[test]
fn a_patch_holding_an_unpaired_surrogate_is_named() {
    let data = "{\"task\": \"t\", \"attempts\": [{\"model\": \"model-a\", \"environment\": \"lane one\", \"result\": \"green\", \"finished_at\": 1, \"tokens\": 1, \"patch\": \"d\\ud800\"}, {\"model\": \"model-b\", \"environment\": \"lane two\", \"result\": \"red\", \"finished_at\": 2, \"tokens\": null, \"patch\": \"q\"}]}";
    let (_, defects) = load_race_text(data);
    assert_eq!(
        defects,
        vec!["attempt 0 field patch holds an unpaired surrogate"]
    );
}

#[test]
fn a_green_attempt_without_finished_at_is_named() {
    let data = race_with_attempt(0, without(&valid_attempt(), &["finished_at"]));
    assert_eq!(
        defects_of(&data),
        vec!["attempt 0 is missing field finished_at"]
    );
}

#[test]
fn a_red_attempt_without_finished_at_is_named() {
    let data = race_with_attempt(1, without(&other_attempt(), &["finished_at"]));
    assert_eq!(
        defects_of(&data),
        vec!["attempt 1 is missing field finished_at"]
    );
}

#[test]
fn an_unfinished_attempt_with_finished_at_is_named() {
    let attempt = attempt_with(
        other_attempt(),
        vec![("result", s("unfinished")), ("finished_at", f(12.0))],
    );
    let data = race_with_attempt(1, attempt);
    assert_eq!(
        defects_of(&data),
        vec!["attempt 1 field finished_at is present but the suite did not finish"]
    );
}

#[test]
fn an_unfinished_attempt_without_finished_at_loads_clean() {
    let attempt = without(
        &set(&other_attempt(), "result", s("unfinished")),
        &["finished_at"],
    );
    let data = race_with_attempt(1, attempt);
    let race = loaded(&data);
    assert_eq!(race.attempts[1].finished_at, None);
}

#[test]
fn a_finished_at_of_the_wrong_type_is_named() {
    let data = race_with_attempt(0, set(&valid_attempt(), "finished_at", s("soon")));
    assert_eq!(
        defects_of(&data),
        vec!["attempt 0 field finished_at has the wrong type"]
    );
}

#[test]
fn an_infinite_finished_at_is_named_as_not_finite() {
    let data = race_with_attempt(0, set(&valid_attempt(), "finished_at", f(f64::INFINITY)));
    assert_eq!(
        defects_of(&data),
        vec!["attempt 0 field finished_at is not a finite number"]
    );
}

#[test]
fn a_huge_integer_finished_at_is_named_as_not_finite() {
    let huge = format!("1{}", "0".repeat(400));
    let data = race_with_attempt(0, set(&valid_attempt(), "finished_at", bigint(&huge)));
    assert_eq!(
        defects_of(&data),
        vec!["attempt 0 field finished_at is not a finite number"]
    );
}

#[test]
fn a_negative_finished_at_is_named() {
    let data = race_with_attempt(0, set(&valid_attempt(), "finished_at", f(-0.5)));
    assert_eq!(
        defects_of(&data),
        vec!["attempt 0 field finished_at is negative"]
    );
}

#[test]
fn several_defects_arrive_in_one_pass() {
    let data = without(&valid_race(), &["task", "attempts"]);
    assert_eq!(
        defects_of(&data),
        vec!["missing field task", "missing field attempts"]
    );
}

#[test]
fn load_race_file_names_an_unreadable_path() {
    let dir = temp_dir("races-unreadable");
    let (race, defects) = load_race_file(&dir.join("nope.json"));
    assert!(race.is_none());
    assert_eq!(defects, vec!["file cannot be read"]);
}

#[test]
fn load_race_file_reads_a_written_race() {
    let dir = temp_dir("races-written");
    let path = dir.join("race.json");
    std::fs::write(&path, deadheat::json::dumps(&valid_race(), None, true)).expect("write");
    let (race, defects) = load_race_file(&path);
    assert_eq!(defects, Vec::<String>::new());
    assert_eq!(race.expect("a race").task.display(), "fix the parser");
}

// The winner rule, DH-2

#[test]
fn only_the_first_attempt_green_wins() {
    let race = loaded(&valid_race());
    assert_eq!(winner_model(&race), "model-a");
    assert_eq!(loser_model(&race), "model-b");
}

#[test]
fn only_the_second_attempt_green_wins() {
    let data = obj(vec![
        ("task", s("fix the parser")),
        (
            "attempts",
            arr(vec![
                attempt_with(
                    valid_attempt(),
                    vec![("result", s("red")), ("finished_at", f(30.0))],
                ),
                attempt_with(
                    other_attempt(),
                    vec![
                        ("result", s("green")),
                        ("finished_at", f(44.0)),
                        ("tokens", i(10)),
                    ],
                ),
            ]),
        ),
    ]);
    let race = loaded(&data);
    assert_eq!(winner_model(&race), "model-b");
    assert_eq!(loser_model(&race), "model-a");
}

#[test]
fn both_green_the_earlier_finish_wins() {
    let data = obj(vec![
        ("task", s("fix the parser")),
        (
            "attempts",
            arr(vec![
                set(&valid_attempt(), "finished_at", f(200.0)),
                attempt_with(
                    other_attempt(),
                    vec![("result", s("green")), ("finished_at", f(150.0))],
                ),
            ]),
        ),
    ]);
    let race = loaded(&data);
    assert_eq!(winner_model(&race), "model-b");
}

#[test]
fn both_green_an_exact_tie_goes_to_the_first_attempt() {
    let data = obj(vec![
        ("task", s("fix the parser")),
        (
            "attempts",
            arr(vec![
                set(&valid_attempt(), "finished_at", f(100.0)),
                attempt_with(
                    other_attempt(),
                    vec![("result", s("green")), ("finished_at", f(100.0))],
                ),
            ]),
        ),
    ]);
    let race = loaded(&data);
    assert_eq!(winner_model(&race), "model-a");
    assert_eq!(loser_model(&race), "model-b");
}

#[test]
fn green_beats_unfinished() {
    let unfinished = without(
        &attempt_with(
            other_attempt(),
            vec![("result", s("unfinished")), ("model", s("model-b"))],
        ),
        &["finished_at"],
    );
    let data = obj(vec![
        ("task", s("fix the parser")),
        ("attempts", arr(vec![unfinished, valid_attempt()])),
    ]);
    let race = loaded(&data);
    assert_eq!(winner_model(&race), "model-a");
}

#[test]
fn red_against_unfinished_has_no_winner() {
    let data = obj(vec![
        ("task", s("fix the parser")),
        (
            "attempts",
            arr(vec![
                attempt_with(
                    valid_attempt(),
                    vec![("result", s("red")), ("finished_at", f(10.0))],
                ),
                without(
                    &set(&other_attempt(), "result", s("unfinished")),
                    &["finished_at"],
                ),
            ]),
        ),
    ]);
    let race = loaded(&data);
    assert_eq!(decide_winner(&race), (None, None));
}

#[test]
fn both_red_has_no_winner() {
    let data = obj(vec![
        ("task", s("fix the parser")),
        (
            "attempts",
            arr(vec![
                set(&valid_attempt(), "result", s("red")),
                other_attempt(),
            ]),
        ),
    ]);
    let race = loaded(&data);
    assert_eq!(decide_winner(&race), (None, None));
}

#[test]
fn both_unfinished_has_no_winner() {
    let data = obj(vec![
        ("task", s("fix the parser")),
        (
            "attempts",
            arr(vec![
                without(
                    &set(&valid_attempt(), "result", s("unfinished")),
                    &["finished_at"],
                ),
                without(
                    &set(&other_attempt(), "result", s("unfinished")),
                    &["finished_at"],
                ),
            ]),
        ),
    ]);
    let race = loaded(&data);
    assert_eq!(decide_winner(&race), (None, None));
}

// The report and the funeral, DH-3

#[test]
fn the_report_lines_for_a_decided_race() {
    assert_eq!(
        race_report(&loaded(&valid_race())),
        vec![
            "race fix the parser",
            "model-a in lane one, green in 120.5 seconds, 4500 tokens",
            "model-b in lane two, red in 90.25 seconds, tokens unreported",
            "verdict, model-a wins",
        ]
    );
}

#[test]
fn the_report_states_no_winner_when_neither_goes_green() {
    let data = obj(vec![
        ("task", s("fix the parser")),
        (
            "attempts",
            arr(vec![
                set(&valid_attempt(), "result", s("red")),
                other_attempt(),
            ]),
        ),
    ]);
    let lines = race_report(&loaded(&data));
    assert_eq!(lines[lines.len() - 2], "verdict, no winner");
    assert_eq!(
        lines[lines.len() - 1],
        "close call, no adjudication on record"
    );
}

#[test]
fn the_funeral_lines_for_a_decided_race() {
    assert_eq!(
        funeral(&loaded(&valid_race())),
        vec![
            "funeral for fix the parser",
            "winner model-a in lane one went green in 120.5 seconds",
            "loser model-b in lane two went red in 90.25 seconds",
            "margin 30.25 seconds between the two finishes",
        ]
    );
}

#[test]
fn the_funeral_lines_for_a_race_with_no_winner() {
    let data = obj(vec![
        ("task", s("fix the parser")),
        (
            "attempts",
            arr(vec![
                attempt_with(
                    valid_attempt(),
                    vec![("result", s("red")), ("finished_at", f(10.0))],
                ),
                without(
                    &set(&other_attempt(), "result", s("unfinished")),
                    &["finished_at"],
                ),
            ]),
        ),
    ]);
    assert_eq!(
        funeral(&loaded(&data)),
        vec![
            "funeral for fix the parser",
            "no winner, neither suite went green",
            "model-a in lane one went red in 10 seconds",
            "model-b in lane two never finished",
        ]
    );
}

#[test]
fn the_margin_names_the_gap_when_the_loser_finished_first() {
    let data = obj(vec![
        ("task", s("fix the parser")),
        (
            "attempts",
            arr(vec![
                attempt_with(
                    valid_attempt(),
                    vec![
                        ("result", s("red")),
                        ("finished_at", f(40.0)),
                        ("model", s("model-b")),
                    ],
                ),
                attempt_with(
                    other_attempt(),
                    vec![
                        ("result", s("green")),
                        ("finished_at", f(70.0)),
                        ("model", s("model-a")),
                    ],
                ),
            ]),
        ),
    ]);
    let lines = funeral(&loaded(&data));
    assert_eq!(
        lines[lines.len() - 1],
        "margin 30 seconds between the two finishes"
    );
}

#[test]
fn no_margin_line_when_the_loser_never_finished() {
    let data = obj(vec![
        ("task", s("fix the parser")),
        (
            "attempts",
            arr(vec![
                valid_attempt(),
                without(
                    &set(&other_attempt(), "result", s("unfinished")),
                    &["finished_at"],
                ),
            ]),
        ),
    ]);
    let lines = funeral(&loaded(&data));
    assert!(!lines.iter().any(|line| line.starts_with("margin")));
    assert_eq!(lines[2], "loser model-b in lane two never finished");
}

#[test]
fn a_sub_precision_margin_renders_at_full_precision() {
    // The pin as altered under FIX-DH-8 in the Python corpus.
    let data = obj(vec![
        ("task", s("fix the parser")),
        (
            "attempts",
            arr(vec![
                set(&valid_attempt(), "finished_at", f(1.0)),
                attempt_with(
                    other_attempt(),
                    vec![("result", s("green")), ("finished_at", f(1.0000004))],
                ),
            ]),
        ),
    ]);
    let lines = funeral(&loaded(&data));
    let last = &lines[lines.len() - 1];
    assert_eq!(
        last,
        "margin 0.0000004000000000115022658064845018088817596435546875 seconds between the two finishes"
    );
    assert!(!last.contains("margin 0 seconds"));
}

#[test]
fn format_seconds_renders_plain_values() {
    assert_eq!(format_seconds(&Num::from_f64(30.25)), "30.25");
    assert_eq!(format_seconds(&Num::from_f64(10.0)), "10");
    assert_eq!(format_seconds(&Num::from_i64(0)), "0");
}

#[test]
fn format_seconds_renders_a_large_integer_digit_for_digit() {
    assert_eq!(
        format_seconds(&Num::Int(num_bigint::BigInt::from(123456789012345678i64))),
        "123456789012345678"
    );
    assert_eq!(
        format_seconds(&Num::Int(
            "100000000000000000001"
                .parse::<num_bigint::BigInt>()
                .expect("digits")
        )),
        "100000000000000000001"
    );
}

#[test]
fn format_seconds_rounds_an_exact_half_up_on_the_exact_value() {
    assert_eq!(format_seconds(&Num::from_f64(0.0078125)), "0.007813");
}

#[test]
fn format_seconds_renders_a_terminating_sub_floor_figure_in_full() {
    assert_eq!(
        format_seconds(&Num::from_f64(1.0 / 4194304.0)),
        "0.0000002384185791015625"
    );
}

#[test]
fn format_seconds_rounds_a_non_terminating_sub_floor_figure() {
    let fraction = num_rational::Ratio::new(
        num_bigint::BigInt::from(1),
        num_bigint::BigInt::from(30000000),
    );
    assert_eq!(format_seconds_fraction(&fraction), "0.000000033333");
}

// The loser patch operations, DH-3

#[test]
fn slugify_reduces_labels() {
    assert_eq!(slugify(&PText::from_text("Model A!")), "model-a");
    assert_eq!(slugify(&PText::from_text("weird__name")), "weird-name");
    assert_eq!(slugify(&PText::from_text("a  b")), "a-b");
    assert_eq!(slugify(&PText::from_text("!!!")), "unnamed");
}

#[test]
fn the_patch_file_name_joins_the_slugs() {
    assert_eq!(
        patch_file_name(&loaded(&valid_race())),
        "fix-the-parser-model-b.patch"
    );
}

#[test]
fn adopt_writes_the_loser_patch_verbatim() {
    let dir = temp_dir("races-adopt");
    let race = loaded(&valid_race());
    let target = adopt_patch(&race, &dir.join("patches")).expect("a target");
    assert_eq!(std::fs::read(&target).expect("bytes"), b"other diff");
    assert_eq!(
        target.file_name().expect("a name").to_string_lossy(),
        "fix-the-parser-model-b.patch"
    );
}

#[test]
fn adopt_keeps_an_unusual_patch_byte_for_byte() {
    let dir = temp_dir("races-adopt-unusual");
    let data = race_with_attempt(
        1,
        set(&other_attempt(), "patch", s("line one\nline two  \n")),
    );
    let race = loaded(&data);
    let target = adopt_patch(&race, &dir).expect("a target");
    assert_eq!(
        std::fs::read(&target).expect("bytes"),
        b"line one\nline two  \n"
    );
}

#[test]
fn discard_removes_an_adopted_patch() {
    let dir = temp_dir("races-discard");
    let race = loaded(&valid_race());
    let target = adopt_patch(&race, &dir).expect("a target");
    let returned = discard_patch(&race, &dir).expect("a path");
    assert_eq!(returned, target);
    assert!(!target.exists());
}

#[test]
fn discard_succeeds_when_no_patch_stands_there() {
    let dir = temp_dir("races-discard-empty");
    let race = loaded(&valid_race());
    let returned = discard_patch(&race, &dir).expect("a path");
    assert_eq!(returned, dir.join("fix-the-parser-model-b.patch"));
}

#[test]
fn adopt_and_discard_refuse_a_race_with_no_loser() {
    let dir = temp_dir("races-no-loser");
    let data = obj(vec![
        ("task", s("fix the parser")),
        (
            "attempts",
            arr(vec![
                set(&valid_attempt(), "result", s("red")),
                other_attempt(),
            ]),
        ),
    ]);
    let race = loaded(&data);
    assert!(adopt_patch(&race, &dir).is_none());
    assert!(discard_patch(&race, &dir).is_none());
    assert_eq!(std::fs::read_dir(&dir).expect("a listing").count(), 0);
}

// The standings, DH-4

fn entry(wins: i64, losses: i64, no_results: i64, tokens: i64, unreported: i64) -> StandingEntry {
    StandingEntry {
        wins: num_bigint::BigInt::from(wins),
        losses: num_bigint::BigInt::from(losses),
        no_results: num_bigint::BigInt::from(no_results),
        tokens: num_bigint::BigInt::from(tokens),
        tokens_unreported: num_bigint::BigInt::from(unreported),
    }
}

fn standing(standings: &Standings, name: &str) -> StandingEntry {
    standings
        .models
        .iter()
        .find(|(held, _)| held.display() == name)
        .map(|(_, entry)| entry.clone())
        .expect("an entry")
}

#[test]
fn recording_a_decided_race() {
    let standings = update_standings(&empty_standings(), &loaded(&valid_race()));
    assert_eq!(
        standings,
        Standings {
            models: vec![
                (PText::from_text("model-a"), entry(1, 0, 0, 4500, 0)),
                (PText::from_text("model-b"), entry(0, 1, 0, 0, 1)),
            ],
        }
    );
}

#[test]
fn standings_accumulate_across_two_races() {
    let first = update_standings(&empty_standings(), &loaded(&valid_race()));
    let second_data = obj(vec![
        ("task", s("fix the parser again")),
        (
            "attempts",
            arr(vec![
                attempt_with(
                    valid_attempt(),
                    vec![
                        ("result", s("red")),
                        ("finished_at", f(12.0)),
                        ("tokens", i(100)),
                    ],
                ),
                without(
                    &set(&other_attempt(), "result", s("unfinished")),
                    &["finished_at"],
                ),
            ]),
        ),
    ]);
    let standings = update_standings(&first, &loaded(&second_data));
    assert_eq!(standing(&standings, "model-a"), entry(1, 0, 1, 4600, 0));
    assert_eq!(standing(&standings, "model-b"), entry(0, 1, 1, 0, 2));
}

#[test]
fn null_tokens_count_as_unreported_and_never_as_zero() {
    let data = obj(vec![
        ("task", s("fix the parser")),
        (
            "attempts",
            arr(vec![
                set(&valid_attempt(), "tokens", JsonValue::Null),
                set(&other_attempt(), "tokens", JsonValue::Null),
            ]),
        ),
    ]);
    let standings = update_standings(&empty_standings(), &loaded(&data));
    assert_eq!(
        standing(&standings, "model-a").tokens,
        num_bigint::BigInt::from(0)
    );
    assert_eq!(
        standing(&standings, "model-a").tokens_unreported,
        num_bigint::BigInt::from(1)
    );
    assert_eq!(
        standing(&standings, "model-b").tokens_unreported,
        num_bigint::BigInt::from(1)
    );
}

#[test]
fn the_update_never_mutates_its_input() {
    let before = update_standings(&empty_standings(), &loaded(&valid_race()));
    let snapshot = before.clone();
    let other = set(&valid_race(), "task", s("another task"));
    let _ = update_standings(&before, &loaded(&other));
    assert_eq!(before, snapshot);
}

#[test]
fn one_model_in_both_lanes_records_both_sides() {
    let data = obj(vec![
        ("task", s("fix the parser")),
        (
            "attempts",
            arr(vec![
                attempt_with(
                    valid_attempt(),
                    vec![
                        ("model", s("solo")),
                        ("finished_at", f(10.0)),
                        ("tokens", i(5)),
                    ],
                ),
                attempt_with(
                    other_attempt(),
                    vec![
                        ("model", s("solo")),
                        ("result", s("green")),
                        ("finished_at", f(20.0)),
                        ("tokens", i(7)),
                    ],
                ),
            ]),
        ),
    ]);
    let standings = update_standings(&empty_standings(), &loaded(&data));
    assert_eq!(standings.models.len(), 1);
    assert_eq!(standings.models[0].1, entry(1, 1, 0, 12, 0));
}

#[test]
fn the_standings_table_renders_in_label_order() {
    let standings = update_standings(&empty_standings(), &loaded(&valid_race()));
    assert_eq!(
        standings_table(&standings),
        vec![
            "season standings",
            "model-a, 1 wins, 0 losses, 0 no results, 4500 tokens, 0 unreported",
            "model-b, 0 wins, 1 losses, 0 no results, 0 tokens, 1 unreported",
        ]
    );
}

#[test]
fn the_empty_standings_table_says_no_races_recorded() {
    assert_eq!(
        standings_table(&empty_standings()),
        vec!["season standings", "no races recorded"]
    );
}

#[test]
fn standings_text_round_trips_through_the_loader() {
    let standings = update_standings(&empty_standings(), &loaded(&valid_race()));
    let (parsed, defects) = load_standings_text(&standings_text(&standings));
    assert_eq!(defects, Vec::<String>::new());
    assert_eq!(parsed.expect("standings"), standings);
}

#[test]
fn standings_loader_names_a_missing_models_field() {
    let (_, defects) = load_standings_text("{}");
    assert_eq!(defects, vec!["missing field models"]);
}

#[test]
fn standings_loader_names_a_models_field_of_the_wrong_type() {
    let (_, defects) = load_standings_text("{\"models\": [1]}");
    assert_eq!(defects, vec!["field models has the wrong type"]);
}

#[test]
fn standings_loader_names_a_bad_entry() {
    let (_, defects) = load_standings_text("{\"models\": {\"model-a\": {\"wins\": 1}}}");
    assert!(defects.contains(&"standings entry model-a is missing field losses".to_string()));
}

#[test]
fn standings_loader_names_a_negative_count() {
    let text = "{\"models\": {\"m\": {\"wins\": -1, \"losses\": 0, \"no_results\": 0, \"tokens\": 0, \"tokens_unreported\": 0}}}";
    let (_, defects) = load_standings_text(text);
    assert_eq!(defects, vec!["standings entry m field wins is negative"]);
}

#[test]
fn standings_loader_names_an_entry_that_is_not_an_object() {
    let (_, defects) = load_standings_text("{\"models\": {\"model-a\": 7}}");
    assert_eq!(defects, vec!["standings entry model-a is not an object"]);
}

#[test]
fn standings_loader_names_an_entry_field_of_the_wrong_type() {
    let text = "{\"models\": {\"model-a\": {\"wins\": 0, \"losses\": 0, \"no_results\": 0, \"tokens\": \"many\", \"tokens_unreported\": 0}}}";
    let (_, defects) = load_standings_text(text);
    assert_eq!(
        defects,
        vec!["standings entry model-a field tokens has the wrong type"]
    );
}

#[test]
fn standings_loader_names_an_unpaired_surrogate_in_a_model_key() {
    let text = "{\"models\": {\"bad\\ud800x\": {\"wins\": 0, \"losses\": 0, \"no_results\": 0, \"tokens\": 0, \"tokens_unreported\": 0}}}";
    let (standings, defects) = load_standings_text(text);
    assert!(standings.is_none());
    assert_eq!(defects, vec!["field models holds an unpaired surrogate"]);
}

#[test]
fn standings_loader_names_a_newline_in_a_model_key() {
    let text = "{\"models\": {\"bad\\nkey\": {\"wins\": 0, \"losses\": 0, \"no_results\": 0, \"tokens\": 0, \"tokens_unreported\": 0}}}";
    let (standings, defects) = load_standings_text(text);
    assert!(standings.is_none());
    assert_eq!(defects, vec!["field models holds a newline"]);
}
