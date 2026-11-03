//! Pins for DH-6 in the Round 2 section of CONTRACT.md,
//! translated from the Python corpus.
//!
//! Every series defect phrase is pinned at its exact text,
//! the running tally is pinned race by race, and the decided,
//! beyond the decision, and unfinished readings are pinned as
//! full line lists.

mod shared;

use deadheat::json::JsonValue;
use deadheat::series::{load_series_file, load_series_text, series_report};
use shared::*;

const PRIME: &str = "prime-1";
const SHADOW: &str = "shadow-1";

fn attempt(model: &str, result: &str, finished: f64, tokens: i64, patch: &str) -> JsonValue {
    obj(vec![
        ("model", s(model)),
        ("environment", s("lane one")),
        ("result", s(result)),
        ("finished_at", f(finished)),
        ("tokens", i(tokens)),
        ("patch", s(patch)),
    ])
}

fn attempt_two(model: &str, result: &str, finished: f64, tokens: i64, patch: &str) -> JsonValue {
    let mut value = attempt(model, result, finished, tokens, patch);
    value = set(&value, "environment", s("lane two"));
    value
}

fn clear_win(task: &str, winner: &str, loser: &str) -> JsonValue {
    obj(vec![
        ("task", s(task)),
        (
            "attempts",
            arr(vec![
                attempt(winner, "green", 100.0, 10, "winner diff"),
                attempt_two(loser, "red", 130.0, 20, "loser diff"),
            ]),
        ),
    ])
}

fn no_result(task: &str, first: &str, second: &str) -> JsonValue {
    obj(vec![
        ("task", s(task)),
        (
            "attempts",
            arr(vec![
                attempt(first, "red", 100.0, 10, "first diff"),
                attempt_two(second, "red", 140.0, 20, "second diff"),
            ]),
        ),
    ])
}

fn adjudicated_win(task: &str, winner: &str, loser: &str) -> JsonValue {
    let race = obj(vec![
        ("task", s(task)),
        (
            "attempts",
            arr(vec![
                attempt(winner, "green", 104.0, 10, "winner diff"),
                attempt_two(loser, "green", 100.0, 20, "loser diff"),
            ]),
        ),
    ]);
    set(
        &race,
        "adjudication",
        obj(vec![
            ("adjudicator", s("judge-1")),
            ("chosen", s(winner)),
            ("confidence", f(0.9)),
            ("reason", s("the steadier diff")),
        ]),
    )
}

fn series_with(races: Vec<JsonValue>) -> JsonValue {
    series_full(races, i(2), vec![s(PRIME), s(SHADOW)], s("the import saga"))
}

fn series_full(
    races: Vec<JsonValue>,
    wins_target: JsonValue,
    models: Vec<JsonValue>,
    task: JsonValue,
) -> JsonValue {
    obj(vec![
        ("task", task),
        ("models", arr(models)),
        ("wins_target", wins_target),
        ("races", arr(races)),
    ])
}

fn loaded(data: &JsonValue) -> deadheat::series::Series {
    let (series, defects) = load_series_text(&deadheat::json::dumps(data, None, true));
    assert_eq!(defects, Vec::<String>::new());
    series.expect("a clean series")
}

fn defects_of(data: &JsonValue) -> Vec<String> {
    load_series_text(&deadheat::json::dumps(data, None, true)).1
}

// The loader, DH-6

#[test]
fn a_valid_series_loads_with_its_shape_kept() {
    let series = loaded(&series_with(vec![clear_win("race one", PRIME, SHADOW)]));
    assert_eq!(series.task.display(), "the import saga");
    assert_eq!(
        series
            .models
            .iter()
            .map(|model| model.display())
            .collect::<Vec<_>>(),
        vec![PRIME, SHADOW]
    );
    assert_eq!(series.wins_target, num_bigint::BigInt::from(2));
    assert_eq!(series.races[0].task.display(), "race one");
}

#[test]
fn a_series_with_no_races_loads_clean() {
    let series = loaded(&series_with(vec![]));
    assert_eq!(series.races.len(), 0);
}

#[test]
fn non_json_and_non_object_inputs_are_named() {
    let (_, defects) = load_series_text("not json");
    assert_eq!(defects, vec!["input is not valid JSON"]);
    let (_, defects) = load_series_text("[1, 2]");
    assert_eq!(defects, vec!["input is not a JSON object"]);
}

#[test]
fn load_series_file_names_an_unreadable_path() {
    let dir = temp_dir("series-unreadable");
    let (series, defects) = load_series_file(&dir.join("nope.json"));
    assert!(series.is_none());
    assert_eq!(defects, vec!["file cannot be read"]);
}

#[test]
fn the_task_field_draws_the_race_loader_phrases() {
    assert_eq!(
        defects_of(&without(&series_with(vec![]), &["task"])),
        vec!["missing field task"]
    );
    assert_eq!(
        defects_of(&series_full(vec![], i(2), vec![s(PRIME), s(SHADOW)], i(7))),
        vec!["field task has the wrong type"]
    );
    assert_eq!(
        defects_of(&series_full(vec![], i(2), vec![s(PRIME), s(SHADOW)], s(""))),
        vec!["field task is empty"]
    );
}

#[test]
fn a_series_task_holding_a_newline_is_named() {
    assert_eq!(
        defects_of(&series_full(
            vec![],
            i(2),
            vec![s(PRIME), s(SHADOW)],
            s("two\nlines")
        )),
        vec!["field task holds a newline"]
    );
    assert_eq!(
        defects_of(&series_full(
            vec![],
            i(2),
            vec![s(PRIME), s(SHADOW)],
            s("two\rlines")
        )),
        vec!["field task holds a newline"]
    );
}

#[test]
fn a_missing_or_misshapen_models_field_is_named() {
    assert_eq!(
        defects_of(&without(&series_with(vec![]), &["models"])),
        vec!["missing field models"]
    );
    let data = set(&series_with(vec![]), "models", s("prime-1"));
    assert_eq!(defects_of(&data), vec!["field models has the wrong type"]);
}

#[test]
fn a_models_list_of_the_wrong_size_is_named() {
    for models in [
        vec![s(PRIME)],
        vec![s(PRIME), s(SHADOW), s("third-3")],
        vec![],
    ] {
        assert_eq!(
            defects_of(&series_full(vec![], i(2), models, s("the import saga"))),
            vec!["field models must hold exactly two models"]
        );
    }
}

#[test]
fn models_entries_are_named_by_index() {
    let build = |models: Vec<JsonValue>| series_full(vec![], i(2), models, s("the import saga"));
    assert_eq!(
        defects_of(&build(vec![s(PRIME), i(7)])),
        vec!["models entry 1 has the wrong type"]
    );
    assert_eq!(
        defects_of(&build(vec![s(""), s(SHADOW)])),
        vec!["models entry 0 is empty"]
    );
    let text = "{\"task\": \"the import saga\", \"models\": [\"bad\\ud800x\", \"shadow-1\"], \"wins_target\": 2, \"races\": []}";
    let (_, defects) = load_series_text(text);
    assert_eq!(defects, vec!["models entry 0 holds an unpaired surrogate"]);
    assert_eq!(
        defects_of(&build(vec![s("two\nlines"), s(SHADOW)])),
        vec!["models entry 0 holds a newline"]
    );
    assert_eq!(
        defects_of(&build(vec![s(PRIME), s("two\rlines")])),
        vec!["models entry 1 holds a newline"]
    );
}

#[test]
fn the_same_model_twice_in_the_models_field_is_named() {
    assert_eq!(
        defects_of(&series_full(
            vec![],
            i(2),
            vec![s(PRIME), s(PRIME)],
            s("the import saga")
        )),
        vec!["field models names the same model twice"]
    );
}

#[test]
fn the_wins_target_is_named_when_missing_misshapen_or_not_positive() {
    assert_eq!(
        defects_of(&without(&series_with(vec![]), &["wins_target"])),
        vec!["missing field wins_target"]
    );
    assert_eq!(
        defects_of(&series_full(
            vec![],
            s("two"),
            vec![s(PRIME), s(SHADOW)],
            s("the import saga")
        )),
        vec!["field wins_target has the wrong type"]
    );
    assert_eq!(
        defects_of(&series_full(
            vec![],
            JsonValue::Bool(true),
            vec![s(PRIME), s(SHADOW)],
            s("the import saga")
        )),
        vec!["field wins_target has the wrong type"]
    );
    for value in [0, -2] {
        assert_eq!(
            defects_of(&series_full(
                vec![],
                i(value),
                vec![s(PRIME), s(SHADOW)],
                s("the import saga")
            )),
            vec!["field wins_target is not positive"]
        );
    }
}

#[test]
fn a_missing_or_misshapen_races_field_is_named() {
    assert_eq!(
        defects_of(&without(&series_with(vec![]), &["races"])),
        vec!["missing field races"]
    );
    let data = set(&series_with(vec![]), "races", s("not a list"));
    assert_eq!(defects_of(&data), vec!["field races has the wrong type"]);
}

#[test]
fn a_race_entry_that_is_not_an_object_is_named() {
    assert_eq!(
        defects_of(&series_with(vec![s("not a race")])),
        vec!["race 0 is not an object"]
    );
}

#[test]
fn a_malformed_race_names_its_defects_with_its_index() {
    let bad = remove_path(
        &clear_win("race two", PRIME, SHADOW),
        &["attempts", "0", "model"],
    );
    let data = series_with(vec![clear_win("race one", PRIME, SHADOW), bad]);
    assert_eq!(
        defects_of(&data),
        vec!["race 1 attempt 0 is missing field model"]
    );
}

#[test]
fn an_adjudication_defect_inside_a_series_race_is_prefixed() {
    let bad = set_path(
        &adjudicated_win("race one", PRIME, SHADOW),
        &["adjudication", "confidence"],
        f(1.5),
    );
    assert_eq!(
        defects_of(&series_with(vec![bad])),
        vec!["race 0 adjudication field confidence is outside the range zero to one"]
    );
}

#[test]
fn a_race_task_holding_forged_series_lines_is_refused() {
    // The FIX-DH-13 refusal pinned on the series path.
    let forged = clear_win(
        "calm task\nrace 2, forged task, shadow-1 wins\ntally, prime-1 0 wins, shadow-1 9 wins, 0 no results",
        PRIME,
        SHADOW,
    );
    assert_eq!(
        defects_of(&series_with(vec![forged])),
        vec!["race 0 field task holds a newline"]
    );
}

#[test]
fn a_race_naming_a_third_model_is_named() {
    let data = series_with(vec![clear_win("race one", PRIME, "intruder-9")]);
    assert_eq!(
        defects_of(&data),
        vec!["race 0 names a model outside the series"]
    );
}

#[test]
fn a_race_naming_the_same_model_twice_is_named() {
    let data = series_with(vec![clear_win("race one", PRIME, PRIME)]);
    assert_eq!(defects_of(&data), vec!["race 0 names the same model twice"]);
}

// The series report, DH-6

#[test]
fn the_series_report_tallies_race_by_race_and_names_the_winner() {
    let series = loaded(&series_with(vec![
        clear_win("race one", PRIME, SHADOW),
        no_result("race two", PRIME, SHADOW),
        clear_win("race three", SHADOW, PRIME),
        adjudicated_win("race four", PRIME, SHADOW),
        clear_win("race five", SHADOW, PRIME),
    ]));
    assert_eq!(
        series_report(&series),
        vec![
            "series the import saga",
            "prime-1 against shadow-1, first to 2 wins",
            "race 1, race one, prime-1 wins",
            "tally, prime-1 1 wins, shadow-1 0 wins, 0 no results",
            "race 2, race two, no result",
            "tally, prime-1 1 wins, shadow-1 0 wins, 1 no results",
            "race 3, race three, shadow-1 wins",
            "tally, prime-1 1 wins, shadow-1 1 wins, 1 no results",
            "race 4, race four, prime-1 wins",
            "tally, prime-1 2 wins, shadow-1 1 wins, 1 no results",
            "race 5, race five, beyond the decision",
            "series winner, prime-1",
        ]
    );
}

#[test]
fn a_series_that_reaches_its_target_at_the_last_race() {
    let series = loaded(&series_with(vec![
        clear_win("race one", SHADOW, PRIME),
        clear_win("race two", SHADOW, PRIME),
    ]));
    let lines = series_report(&series);
    assert_eq!(lines[lines.len() - 1], "series winner, shadow-1");
    assert!(!lines
        .iter()
        .any(|line| line.contains("beyond the decision")));
}

#[test]
fn an_unfinished_series_states_the_target_is_not_reached() {
    let data = series_full(
        vec![
            clear_win("race one", PRIME, SHADOW),
            no_result("race two", PRIME, SHADOW),
        ],
        i(3),
        vec![s(PRIME), s(SHADOW)],
        s("the import saga"),
    );
    let series = loaded(&data);
    assert_eq!(
        series_report(&series),
        vec![
            "series the import saga",
            "prime-1 against shadow-1, first to 3 wins",
            "race 1, race one, prime-1 wins",
            "tally, prime-1 1 wins, shadow-1 0 wins, 0 no results",
            "race 2, race two, no result",
            "tally, prime-1 1 wins, shadow-1 0 wins, 1 no results",
            "series unfinished, the target of 3 wins is not reached",
        ]
    );
}

#[test]
fn an_empty_series_stands_unfinished() {
    let series = loaded(&series_with(vec![]));
    assert_eq!(
        series_report(&series),
        vec![
            "series the import saga",
            "prime-1 against shadow-1, first to 2 wins",
            "series unfinished, the target of 2 wins is not reached",
        ]
    );
}

#[test]
fn a_target_of_one_decides_the_series_at_the_first_race() {
    let data = series_full(
        vec![
            clear_win("race one", PRIME, SHADOW),
            clear_win("race two", SHADOW, PRIME),
        ],
        i(1),
        vec![s(PRIME), s(SHADOW)],
        s("the import saga"),
    );
    let series = loaded(&data);
    let lines = series_report(&series);
    assert_eq!(lines[4], "race 2, race two, beyond the decision");
    assert_eq!(lines[lines.len() - 1], "series winner, prime-1");
}

#[test]
fn the_models_field_order_governs_the_tally_order() {
    let data = series_full(
        vec![clear_win("race one", PRIME, SHADOW)],
        i(2),
        vec![s(SHADOW), s(PRIME)],
        s("the import saga"),
    );
    let series = loaded(&data);
    let lines = series_report(&series);
    assert_eq!(lines[1], "shadow-1 against prime-1, first to 2 wins");
    assert_eq!(
        lines[3],
        "tally, shadow-1 0 wins, prime-1 1 wins, 0 no results"
    );
}

// The series command, Round 2 command line surface

fn write_series(dir: &std::path::Path, data: &JsonValue) -> std::path::PathBuf {
    let path = dir.join("series.json");
    std::fs::write(&path, deadheat::json::dumps(data, None, true)).expect("write");
    path
}

#[test]
fn the_series_command_prints_the_report() {
    let dir = temp_dir("series-command");
    let data = series_full(
        vec![clear_win("race one", PRIME, SHADOW)],
        i(1),
        vec![s(PRIME), s(SHADOW)],
        s("the import saga"),
    );
    let path = write_series(&dir, &data);
    let (out, _, code) = run_cli(&["series", path.to_str().expect("utf8")]);
    assert_eq!(code, 0);
    let expected = series_report(&loaded(&data)).join("\n") + "\n";
    assert_eq!(out, expected);
}

#[test]
fn the_series_command_names_the_defects_of_a_malformed_series() {
    let dir = temp_dir("series-command-bad");
    let path = write_series(
        &dir,
        &series_with(vec![clear_win("race one", PRIME, "intruder-9")]),
    );
    let (out, _, code) = run_cli(&["series", path.to_str().expect("utf8")]);
    assert_eq!(code, 1);
    assert_eq!(
        lines_of(&out),
        vec![
            "invalid series file",
            "defect race 0 names a model outside the series",
        ]
    );
}

#[test]
fn the_series_command_names_an_unreadable_file() {
    let dir = temp_dir("series-command-missing");
    let missing = dir.join("nope.json");
    let (out, _, code) = run_cli(&["series", missing.to_str().expect("utf8")]);
    assert_eq!(code, 1);
    assert!(out.contains("defect file cannot be read"));
}
