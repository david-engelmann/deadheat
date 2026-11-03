//! Pins for DH-5 in the Round 2 section of CONTRACT.md,
//! translated from the Python corpus.
//!
//! The band edges are pinned on both sides and exactly on the
//! margin, the confidence threshold is pinned at and below
//! four fifths, every adjudication defect phrase is pinned at
//! its exact text, and the Round 1 outcomes for races the band
//! does not touch are pinned again here so a drift in the new
//! step shows against them.

mod shared;

use deadheat::json::JsonValue;
use deadheat::races::*;
use shared::*;

fn attempt() -> JsonValue {
    obj(vec![
        ("model", s("model-a")),
        ("environment", s("lane one")),
        ("result", s("green")),
        ("finished_at", f(100.0)),
        ("tokens", i(100)),
        ("patch", s("diff text")),
    ])
}

fn other() -> JsonValue {
    obj(vec![
        ("model", s("model-b")),
        ("environment", s("lane two")),
        ("result", s("green")),
        ("finished_at", f(96.0)),
        ("tokens", i(200)),
        ("patch", s("other diff")),
    ])
}

fn entry() -> JsonValue {
    obj(vec![
        ("adjudicator", s("judge-1")),
        ("chosen", s("model-a")),
        ("confidence", f(0.9)),
        ("reason", s("the steadier diff")),
    ])
}

fn race_with(attempts: Vec<JsonValue>, adjudication: Option<JsonValue>) -> JsonValue {
    let mut data = obj(vec![
        ("task", s("fix the parser")),
        ("attempts", arr(attempts)),
    ]);
    if let Some(value) = adjudication {
        data = set(&data, "adjudication", value);
    }
    data
}

fn loaded(data: &JsonValue) -> Race {
    let (race, defects) = load_race_data(data);
    assert_eq!(defects, Vec::<String>::new());
    race.expect("a clean race")
}

fn defects_of(data: &JsonValue) -> Vec<String> {
    load_race_data(data).1
}

fn winner_model(race: &Race) -> String {
    let (winner, _) = decide_winner(race);
    race.attempts[winner.expect("a winner")].model.display()
}

fn loser_model(race: &Race) -> String {
    let (_, loser) = decide_winner(race);
    race.attempts[loser.expect("a loser")].model.display()
}

// The band, DH-5

#[test]
fn both_green_well_inside_the_margin_is_a_close_call() {
    let race = loaded(&race_with(vec![attempt(), other()], None));
    assert!(is_close_call(&race));
}

#[test]
fn both_green_exactly_at_the_margin_is_a_close_call() {
    let race = loaded(&race_with(
        vec![attempt(), set(&other(), "finished_at", f(95.0))],
        None,
    ));
    assert!(is_close_call(&race));
}

#[test]
fn both_green_a_decimal_case_inside_the_margin() {
    let race = loaded(&race_with(
        vec![
            set(&attempt(), "finished_at", f(200.5)),
            set(&other(), "finished_at", f(190.5)),
        ],
        None,
    ));
    assert!(is_close_call(&race));
}

#[test]
fn both_green_just_outside_the_margin_is_not_a_close_call() {
    let race = loaded(&race_with(
        vec![attempt(), set(&other(), "finished_at", f(94.9))],
        None,
    ));
    assert!(!is_close_call(&race));
}

#[test]
fn both_green_a_decimal_case_outside_the_margin() {
    let race = loaded(&race_with(
        vec![
            set(&attempt(), "finished_at", f(200.5)),
            set(&other(), "finished_at", f(190.4)),
        ],
        None,
    ));
    assert!(!is_close_call(&race));
}

#[test]
fn both_green_a_sub_precision_gap_is_a_close_call() {
    let race = loaded(&race_with(
        vec![
            set(&attempt(), "finished_at", f(1.0)),
            set(&other(), "finished_at", f(1.0000004)),
        ],
        None,
    ));
    assert!(is_close_call(&race));
}

#[test]
fn an_exact_tie_of_green_finishes_is_not_a_close_call() {
    let race = loaded(&race_with(
        vec![attempt(), set(&other(), "finished_at", f(100.0))],
        None,
    ));
    assert!(!is_close_call(&race));
}

#[test]
fn both_red_is_a_close_call_at_any_times() {
    let race = loaded(&race_with(
        vec![
            set(&set(&attempt(), "result", s("red")), "finished_at", f(10.0)),
            set(&set(&other(), "result", s("red")), "finished_at", f(900.0)),
        ],
        None,
    ));
    assert!(is_close_call(&race));
}

#[test]
fn both_unfinished_is_a_close_call() {
    let race = loaded(&race_with(
        vec![
            without(
                &set(&attempt(), "result", s("unfinished")),
                &["finished_at"],
            ),
            without(&set(&other(), "result", s("unfinished")), &["finished_at"]),
        ],
        None,
    ));
    assert!(is_close_call(&race));
}

#[test]
fn mixed_outcomes_are_not_close_calls() {
    let red_unfinished = loaded(&race_with(
        vec![
            set(&attempt(), "result", s("red")),
            without(&set(&other(), "result", s("unfinished")), &["finished_at"]),
        ],
        None,
    ));
    assert!(!is_close_call(&red_unfinished));
    let green_red = loaded(&race_with(
        vec![attempt(), set(&other(), "result", s("red"))],
        None,
    ));
    assert!(!is_close_call(&green_red));
}

// The adjudication entry and its defects, DH-5

#[test]
fn a_valid_entry_loads_and_is_kept_on_the_race() {
    let race = loaded(&race_with(vec![attempt(), other()], Some(entry())));
    let entry = race.adjudication.expect("an entry");
    assert_eq!(entry.adjudicator.display(), "judge-1");
    assert_eq!(entry.chosen.display(), "model-a");
    assert_eq!(entry.confidence, deadheat::pynum::Num::from_f64(0.9));
    assert_eq!(entry.reason.display(), "the steadier diff");
}

#[test]
fn fields_the_entry_does_not_name_are_ignored() {
    // The Python pin asserts the loaded entry holds exactly the
    // four named fields; the Rust entry is a fixed struct, so
    // the pin stands by construction and by the loader accepting
    // the extra field without a defect.
    let with_extra = set(&entry(), "probabilities", obj(vec![("model-a", f(0.9))]));
    let race = loaded(&race_with(vec![attempt(), other()], Some(with_extra)));
    let entry = race.adjudication.expect("an entry");
    assert_eq!(entry.chosen.display(), "model-a");
}

#[test]
fn an_entry_that_is_not_an_object_is_named() {
    let data = race_with(vec![attempt(), other()], Some(s("a verdict")));
    assert_eq!(
        defects_of(&data),
        vec!["field adjudication is not an object"]
    );
}

#[test]
fn each_missing_entry_field_is_named() {
    for field in ["adjudicator", "chosen", "confidence", "reason"] {
        let data = race_with(vec![attempt(), other()], Some(without(&entry(), &[field])));
        assert_eq!(
            defects_of(&data),
            vec![format!("adjudication is missing field {field}")]
        );
    }
}

#[test]
fn entry_text_fields_of_the_wrong_type_are_named() {
    for field in ["adjudicator", "chosen", "reason"] {
        let data = race_with(vec![attempt(), other()], Some(set(&entry(), field, i(7))));
        assert_eq!(
            defects_of(&data),
            vec![format!("adjudication field {field} has the wrong type")]
        );
    }
}

#[test]
fn empty_entry_text_fields_are_named() {
    for field in ["adjudicator", "chosen", "reason"] {
        let data = race_with(vec![attempt(), other()], Some(set(&entry(), field, s(""))));
        assert_eq!(
            defects_of(&data),
            vec![format!("adjudication field {field} is empty")]
        );
    }
}

#[test]
fn entry_text_fields_holding_a_surrogate_are_named() {
    for field in ["adjudicator", "chosen", "reason"] {
        let entry_text = format!(
            "{{\"adjudicator\": {}, \"chosen\": {}, \"confidence\": 0.9, \"reason\": {}}}",
            if field == "adjudicator" {
                "\"bad\\ud800x\""
            } else {
                "\"judge-1\""
            },
            if field == "chosen" {
                "\"bad\\ud800x\""
            } else {
                "\"model-a\""
            },
            if field == "reason" {
                "\"bad\\ud800x\""
            } else {
                "\"the steadier diff\""
            },
        );
        let full = format!(
            "{{\"task\": \"fix the parser\", \"attempts\": [{}, {}], \"adjudication\": {}}}",
            deadheat::json::dumps(&attempt(), None, true),
            deadheat::json::dumps(&other(), None, true),
            entry_text,
        );
        let (_, defects) = load_race_text(&full);
        assert_eq!(
            defects,
            vec![format!(
                "adjudication field {field} holds an unpaired surrogate"
            )]
        );
    }
}

#[test]
fn a_reason_holding_a_newline_is_named() {
    let data = race_with(
        vec![attempt(), other()],
        Some(set(&entry(), "reason", s("two\nlines"))),
    );
    assert_eq!(
        defects_of(&data),
        vec!["adjudication field reason holds a newline"]
    );
}

#[test]
fn a_reason_holding_a_carriage_return_is_named() {
    let data = race_with(
        vec![attempt(), other()],
        Some(set(&entry(), "reason", s("two\rlines"))),
    );
    assert_eq!(
        defects_of(&data),
        vec!["adjudication field reason holds a newline"]
    );
}

#[test]
fn an_adjudicator_holding_a_newline_is_named() {
    let data = race_with(
        vec![attempt(), other()],
        Some(set(&entry(), "adjudicator", s("two\nlines"))),
    );
    assert_eq!(
        defects_of(&data),
        vec!["adjudication field adjudicator holds a newline"]
    );
}

#[test]
fn an_adjudicator_holding_a_carriage_return_is_named() {
    let data = race_with(
        vec![attempt(), other()],
        Some(set(&entry(), "adjudicator", s("two\rlines"))),
    );
    assert_eq!(
        defects_of(&data),
        vec!["adjudication field adjudicator holds a newline"]
    );
}

#[test]
fn a_chosen_label_holding_a_newline_is_named() {
    let data = race_with(
        vec![attempt(), other()],
        Some(set(&entry(), "chosen", s("two\nlines"))),
    );
    assert_eq!(
        defects_of(&data),
        vec!["adjudication field chosen holds a newline"]
    );
}

#[test]
fn a_chosen_label_holding_a_carriage_return_is_named() {
    let data = race_with(
        vec![attempt(), other()],
        Some(set(&entry(), "chosen", s("two\rlines"))),
    );
    assert_eq!(
        defects_of(&data),
        vec!["adjudication field chosen holds a newline"]
    );
}

#[test]
fn a_confidence_of_the_wrong_type_is_named() {
    let data = race_with(
        vec![attempt(), other()],
        Some(set(&entry(), "confidence", s("high"))),
    );
    assert_eq!(
        defects_of(&data),
        vec!["adjudication field confidence has the wrong type"]
    );
}

#[test]
fn a_non_finite_confidence_is_named() {
    for value in [f64::NAN, f64::INFINITY] {
        let data = race_with(
            vec![attempt(), other()],
            Some(set(&entry(), "confidence", f(value))),
        );
        assert_eq!(
            defects_of(&data),
            vec!["adjudication field confidence is not a finite number"]
        );
    }
}

#[test]
fn a_confidence_outside_the_range_is_named() {
    for value in [-0.1, 1.1] {
        let data = race_with(
            vec![attempt(), other()],
            Some(set(&entry(), "confidence", f(value))),
        );
        assert_eq!(
            defects_of(&data),
            vec!["adjudication field confidence is outside the range zero to one"]
        );
    }
}

#[test]
fn the_confidence_range_edges_load_clean() {
    for value in [0, 1] {
        let race = loaded(&race_with(
            vec![attempt(), other()],
            Some(set(&entry(), "confidence", i(value))),
        ));
        assert_eq!(
            race.adjudication.expect("an entry").confidence,
            deadheat::pynum::Num::from_i64(value)
        );
    }
}

#[test]
fn a_chosen_label_naming_no_attempt_is_named() {
    let data = race_with(
        vec![attempt(), other()],
        Some(set(&entry(), "chosen", s("model-c"))),
    );
    assert_eq!(
        defects_of(&data),
        vec!["adjudication field chosen names no attempt in this race"]
    );
}

// The winner rule step, DH-5

#[test]
fn an_entry_at_the_threshold_decides_for_the_chosen_attempt() {
    let race = loaded(&race_with(
        vec![attempt(), other()],
        Some(set(
            &set(&entry(), "chosen", s("model-b")),
            "confidence",
            f(0.8),
        )),
    ));
    assert_eq!(winner_model(&race), "model-b");
    assert_eq!(loser_model(&race), "model-a");
}

#[test]
fn an_entry_above_the_threshold_can_pick_the_slower_attempt() {
    let race = loaded(&race_with(
        vec![attempt(), other()],
        Some(set(&entry(), "confidence", f(0.95))),
    ));
    assert_eq!(winner_model(&race), "model-a");
}

#[test]
fn an_entry_below_the_threshold_abstains() {
    for value in [0.79, 0.7999999, 0.5] {
        let race = loaded(&race_with(
            vec![attempt(), other()],
            Some(set(&entry(), "confidence", f(value))),
        ));
        assert_eq!(decide_winner(&race), (None, None));
    }
}

#[test]
fn the_double_just_below_the_threshold_abstains() {
    let race = loaded(&race_with(
        vec![attempt(), other()],
        Some(set(&entry(), "confidence", f(0.7999999999999999))),
    ));
    assert_eq!(decide_winner(&race), (None, None));
}

#[test]
fn a_close_call_with_no_entry_stands_as_no_result() {
    let race = loaded(&race_with(vec![attempt(), other()], None));
    assert_eq!(decide_winner(&race), (None, None));
}

// The abstention rendering, FIX-DH-10

const FULL_PRECISION: &str = "0.799999899999999986022203302127309143543243408203125";

#[test]
fn an_abstention_rounding_to_the_threshold_renders_in_full_precision() {
    let race = loaded(&race_with(
        vec![attempt(), other()],
        Some(set(&entry(), "confidence", f(0.7999999))),
    ));
    assert_eq!(decide_winner(&race), (None, None));
    assert!(race_report(&race).contains(&format!(
        "close call, adjudication by judge-1 abstained at confidence {FULL_PRECISION}"
    )));
    assert!(funeral(&race).contains(&format!(
        "close call, adjudication by judge-1 abstained at confidence {FULL_PRECISION}, the race stands as a dead heat"
    )));
    assert!(close_call_report(&race).contains(&format!(
        "adjudication by judge-1 abstained at confidence {FULL_PRECISION}, below the threshold of 0.8"
    )));
}

#[test]
fn a_confidence_at_the_threshold_renders_as_the_threshold_figure() {
    let race = loaded(&race_with(
        vec![attempt(), other()],
        Some(set(
            &set(&entry(), "chosen", s("model-b")),
            "confidence",
            f(0.8),
        )),
    ));
    assert!(close_call_report(&race).contains(
        &"adjudication by judge-1 chose model-b at confidence 0.8, the race is decided for model-b"
            .to_string()
    ));
    assert!(race_report(&race).contains(
        &"close call, adjudication by judge-1 chose model-b at confidence 0.8".to_string()
    ));
}

#[test]
fn an_abstention_rounding_below_the_threshold_keeps_its_rendering() {
    let race = loaded(&race_with(
        vec![attempt(), other()],
        Some(set(&entry(), "confidence", f(0.7999994))),
    ));
    assert_eq!(decide_winner(&race), (None, None));
    assert!(close_call_report(&race).contains(
        &"adjudication by judge-1 abstained at confidence 0.799999, below the threshold of 0.8"
            .to_string()
    ));
    assert!(race_report(&race).contains(
        &"close call, adjudication by judge-1 abstained at confidence 0.799999".to_string()
    ));
}

#[test]
fn a_both_red_close_call_is_decided_by_an_entry() {
    let race = loaded(&race_with(
        vec![
            set(&attempt(), "result", s("red")),
            set(&other(), "result", s("red")),
        ],
        Some(set(
            &set(&entry(), "chosen", s("model-b")),
            "confidence",
            f(0.85),
        )),
    ));
    assert_eq!(winner_model(&race), "model-b");
    assert_eq!(loser_model(&race), "model-a");
}

#[test]
fn a_chosen_label_carried_by_both_attempts_picks_the_first() {
    let twins = vec![
        set(&attempt(), "result", s("red")),
        set(&set(&other(), "model", s("model-a")), "result", s("red")),
    ];
    let race = loaded(&race_with(twins, Some(set(&entry(), "confidence", f(0.9)))));
    let (winner, _) = decide_winner(&race);
    assert_eq!(
        race.attempts[winner.expect("a winner")]
            .environment
            .display(),
        "lane one"
    );
}

#[test]
fn an_entry_on_a_race_outside_the_band_is_ignored() {
    let race = loaded(&race_with(
        vec![
            set(&attempt(), "finished_at", f(200.0)),
            set(&other(), "finished_at", f(150.0)),
        ],
        Some(set(&entry(), "confidence", f(0.99))),
    ));
    assert_eq!(winner_model(&race), "model-b");
    let report = race_report(&race);
    assert_eq!(
        report[report.len() - 1],
        "adjudication by judge-1 not needed, the race stands outside the close call band"
    );
    assert!(!funeral(&race)
        .iter()
        .any(|line| line.contains("adjudication")));
}

// The close call rendering, DH-5

#[test]
fn the_report_and_funeral_of_a_decided_close_call() {
    let race = loaded(&race_with(
        vec![attempt(), other()],
        Some(set(
            &set(&entry(), "chosen", s("model-b")),
            "confidence",
            f(0.9),
        )),
    ));
    assert_eq!(
        race_report(&race),
        vec![
            "race fix the parser",
            "model-a in lane one, green in 100 seconds, 100 tokens",
            "model-b in lane two, green in 96 seconds, 200 tokens",
            "verdict, model-b wins",
            "close call, adjudication by judge-1 chose model-b at confidence 0.9",
        ]
    );
    assert_eq!(
        funeral(&race),
        vec![
            "funeral for fix the parser",
            "winner model-b in lane two went green in 96 seconds",
            "loser model-a in lane one went green in 100 seconds",
            "decided by adjudication, judge-1 chose model-b at confidence 0.9",
            "margin 4 seconds between the two finishes",
        ]
    );
}

#[test]
fn the_report_and_funeral_of_an_abstained_close_call() {
    let race = loaded(&race_with(
        vec![attempt(), other()],
        Some(set(&entry(), "confidence", f(0.62))),
    ));
    let report = race_report(&race);
    assert_eq!(
        report[report.len() - 2..],
        [
            "verdict, no winner".to_string(),
            "close call, adjudication by judge-1 abstained at confidence 0.62".to_string(),
        ]
    );
    let lines = funeral(&race);
    assert_eq!(
        lines[1],
        "no winner, both suites went green inside the close call band"
    );
    assert_eq!(
        lines[lines.len() - 2],
        "close call, adjudication by judge-1 abstained at confidence 0.62, the race stands as a dead heat"
    );
    assert_eq!(
        lines[lines.len() - 1],
        "margin 4 seconds between the two finishes"
    );
}

#[test]
fn the_funeral_of_an_unrecorded_both_red_close_call() {
    let race = loaded(&race_with(
        vec![
            set(&attempt(), "result", s("red")),
            set(&other(), "result", s("red")),
        ],
        None,
    ));
    let lines = funeral(&race);
    assert_eq!(lines[1], "no winner, neither suite went green");
    assert_eq!(
        lines[lines.len() - 2],
        "close call, no adjudication on record, the race stands as a dead heat"
    );
    assert_eq!(
        lines[lines.len() - 1],
        "margin 4 seconds between the two finishes"
    );
}

// The standings read adjudicated races through the winner rule, DH-5

#[test]
fn standings_record_an_adjudicated_win_and_an_abstention() {
    let decided = loaded(&race_with(
        vec![attempt(), other()],
        Some(set(
            &set(&entry(), "chosen", s("model-b")),
            "confidence",
            f(0.9),
        )),
    ));
    let standings = update_standings(&empty_standings(), &decided);
    let wins = |table: &Standings, name: &str, field: &str| -> num_bigint::BigInt {
        let entry = table
            .models
            .iter()
            .find(|(held, _)| held.display() == name)
            .map(|(_, entry)| entry.clone())
            .expect("an entry");
        match field {
            "wins" => entry.wins,
            "losses" => entry.losses,
            _ => entry.no_results,
        }
    };
    assert_eq!(
        wins(&standings, "model-b", "wins"),
        num_bigint::BigInt::from(1)
    );
    assert_eq!(
        wins(&standings, "model-a", "losses"),
        num_bigint::BigInt::from(1)
    );
    let abstained = loaded(&race_with(
        vec![attempt(), other()],
        Some(set(&entry(), "confidence", f(0.5))),
    ));
    let standings = update_standings(&standings, &abstained);
    assert_eq!(
        wins(&standings, "model-a", "no_results"),
        num_bigint::BigInt::from(1)
    );
    assert_eq!(
        wins(&standings, "model-b", "no_results"),
        num_bigint::BigInt::from(1)
    );
}

// Round 1 outcomes the band does not touch, pinned again under DH-5

#[test]
fn round_1_a_clear_both_green_margin_still_goes_to_the_earlier_finish() {
    let race = loaded(&race_with(
        vec![
            set(&attempt(), "finished_at", f(200.0)),
            set(&other(), "finished_at", f(150.0)),
        ],
        None,
    ));
    assert_eq!(winner_model(&race), "model-b");
    assert_eq!(
        race_report(&race),
        vec![
            "race fix the parser",
            "model-a in lane one, green in 200 seconds, 100 tokens",
            "model-b in lane two, green in 150 seconds, 200 tokens",
            "verdict, model-b wins",
        ]
    );
}

#[test]
fn round_1_an_exact_tie_still_goes_to_the_first_attempt() {
    let race = loaded(&race_with(
        vec![attempt(), set(&other(), "finished_at", f(100.0))],
        None,
    ));
    assert_eq!(winner_model(&race), "model-a");
    assert_eq!(loser_model(&race), "model-b");
    assert!(!race_report(&race)
        .iter()
        .any(|line| line.contains("close call")));
}

#[test]
fn round_1_a_green_against_a_red_still_goes_to_the_green() {
    let race = loaded(&race_with(
        vec![attempt(), set(&other(), "result", s("red"))],
        None,
    ));
    assert_eq!(winner_model(&race), "model-a");
    assert_eq!(
        funeral(&race),
        vec![
            "funeral for fix the parser",
            "winner model-a in lane one went green in 100 seconds",
            "loser model-b in lane two went red in 96 seconds",
            "margin 4 seconds between the two finishes",
        ]
    );
}

#[test]
fn round_1_both_red_with_no_entry_still_records_a_no_result() {
    let race = loaded(&race_with(
        vec![
            set(&attempt(), "result", s("red")),
            set(&other(), "result", s("red")),
        ],
        None,
    ));
    let standings = update_standings(&empty_standings(), &race);
    for name in ["model-a", "model-b"] {
        let entry = standings
            .models
            .iter()
            .find(|(held, _)| held.display() == name)
            .map(|(_, entry)| entry.clone())
            .expect("an entry");
        assert_eq!(entry.no_results, num_bigint::BigInt::from(1));
    }
}

// The closecall command, Round 2 command line surface

fn write_race(dir: &std::path::Path, data: &JsonValue) -> std::path::PathBuf {
    let path = dir.join("race.json");
    std::fs::write(&path, deadheat::json::dumps(data, None, true)).expect("write");
    path
}

#[test]
fn the_closecall_command_explains_a_decided_close_call() {
    let dir = temp_dir("cc-decided");
    let path = write_race(
        &dir,
        &race_with(
            vec![attempt(), other()],
            Some(set(
                &set(&entry(), "chosen", s("model-b")),
                "confidence",
                f(0.9),
            )),
        ),
    );
    let (out, _, code) = run_cli(&["closecall", path.to_str().expect("utf8")]);
    assert_eq!(code, 0);
    assert_eq!(
        lines_of(&out),
        vec![
            "closecall fix the parser",
            "in the band, both attempts went green and the gap of 4 seconds is within five percent of the slower finish of 100 seconds",
            "adjudication by judge-1 chose model-b at confidence 0.9, the race is decided for model-b",
            "verdict, model-b wins",
        ]
    );
}

#[test]
fn the_closecall_command_explains_an_abstention() {
    let dir = temp_dir("cc-abstained");
    let path = write_race(
        &dir,
        &race_with(
            vec![attempt(), other()],
            Some(set(&entry(), "confidence", f(0.62))),
        ),
    );
    let (out, _, code) = run_cli(&["closecall", path.to_str().expect("utf8")]);
    assert_eq!(code, 0);
    let lines = lines_of(&out);
    assert_eq!(
        lines[2],
        "adjudication by judge-1 abstained at confidence 0.62, below the threshold of 0.8"
    );
    assert_eq!(lines[lines.len() - 1], "verdict, no winner");
}

#[test]
fn the_closecall_command_explains_a_race_outside_the_band() {
    let dir = temp_dir("cc-outside");
    let path = write_race(
        &dir,
        &race_with(
            vec![
                set(&attempt(), "finished_at", f(200.0)),
                set(&other(), "finished_at", f(150.0)),
            ],
            None,
        ),
    );
    let (out, _, code) = run_cli(&["closecall", path.to_str().expect("utf8")]);
    assert_eq!(code, 0);
    assert_eq!(
        lines_of(&out),
        vec![
            "closecall fix the parser",
            "outside the band, the gap of 50 seconds is wider than five percent of the slower finish of 200 seconds",
            "verdict, model-b wins",
        ]
    );
}

#[test]
fn the_closecall_command_explains_a_tie() {
    let dir = temp_dir("cc-tie");
    let path = write_race(
        &dir,
        &race_with(
            vec![attempt(), set(&other(), "finished_at", f(100.0))],
            None,
        ),
    );
    let (out, _, code) = run_cli(&["closecall", path.to_str().expect("utf8")]);
    assert_eq!(code, 0);
    let lines = lines_of(&out);
    assert_eq!(
        lines[1],
        "outside the band, the finish times are tied and the tie goes to the first attempt in the record"
    );
    assert_eq!(lines[lines.len() - 1], "verdict, model-a wins");
}

#[test]
fn the_closecall_command_explains_a_both_red_close_call() {
    let dir = temp_dir("cc-both-red");
    let path = write_race(
        &dir,
        &race_with(
            vec![
                set(&attempt(), "result", s("red")),
                set(&other(), "result", s("red")),
            ],
            None,
        ),
    );
    let (out, _, code) = run_cli(&["closecall", path.to_str().expect("utf8")]);
    assert_eq!(code, 0);
    let lines = lines_of(&out);
    assert_eq!(lines[1], "in the band, both attempts went red");
    assert_eq!(
        lines[2],
        "no adjudication on record, the race stands as no result"
    );
}

#[test]
fn the_closecall_command_reports_the_lines_it_prints() {
    let dir = temp_dir("cc-plain");
    let path = write_race(&dir, &race_with(vec![attempt(), other()], None));
    let (out, _, code) = run_cli(&["closecall", path.to_str().expect("utf8")]);
    assert_eq!(code, 0);
    assert_eq!(
        lines_of(&out),
        vec![
            "closecall fix the parser",
            "in the band, both attempts went green and the gap of 4 seconds is within five percent of the slower finish of 100 seconds",
            "no adjudication on record, the race stands as no result",
            "verdict, no winner",
        ]
    );
}

#[test]
fn a_gap_floats_cannot_see_is_not_reported_as_a_tie() {
    let race = loaded(&race_with(
        vec![
            set(&attempt(), "finished_at", bigint("100000000000000000001")),
            set(&other(), "finished_at", f(1e20)),
        ],
        Some(set(&entry(), "confidence", f(0.9))),
    ));
    assert_eq!(
        close_call_report(&race),
        vec![
            "closecall fix the parser",
            "in the band, both attempts went green and the gap of 1 seconds is within five percent of the slower finish of 100000000000000000001 seconds",
            "adjudication by judge-1 chose model-a at confidence 0.9, the race is decided for model-a",
            "verdict, model-a wins",
        ]
    );
    assert_eq!(
        funeral(&race),
        vec![
            "funeral for fix the parser",
            "winner model-a in lane one went green in 100000000000000000001 seconds",
            "loser model-b in lane two went green in 100000000000000000000 seconds",
            "decided by adjudication, judge-1 chose model-a at confidence 0.9",
            "margin 1 seconds between the two finishes",
        ]
    );
}

#[test]
fn the_closecall_command_names_the_defects_of_a_malformed_race() {
    let dir = temp_dir("cc-malformed");
    let path = write_race(
        &dir,
        &race_with(
            vec![attempt(), other()],
            Some(set(&entry(), "confidence", f(1.5))),
        ),
    );
    let (out, _, code) = run_cli(&["closecall", path.to_str().expect("utf8")]);
    assert_eq!(code, 1);
    assert_eq!(
        lines_of(&out),
        vec![
            "invalid recorded race",
            "defect adjudication field confidence is outside the range zero to one",
        ]
    );
}

#[test]
fn the_closecall_command_names_an_unreadable_file() {
    let dir = temp_dir("cc-unreadable");
    let missing = dir.join("nope.json");
    let (out, _, code) = run_cli(&["closecall", missing.to_str().expect("utf8")]);
    assert_eq!(code, 1);
    assert!(out.contains("defect file cannot be read"));
}
