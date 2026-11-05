//! Pins for DH-13 and DH-14 in the Round 6 section of
//! CONTRACT.md.
//!
//! Every championship defect phrase is pinned at its exact
//! text, the whole or nothing refusal is pinned with a good
//! series standing first in the collection, the counting walk
//! is pinned against hand computed figures including the races
//! beyond a decision counting for nothing, the aggregate
//! margin is pinned in exact arithmetic, and the presentation
//! order is pinned at each tie depth.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{championship_report, load_championship_file, load_championship_text};
use deadheat::json::JsonValue;
use deadheat::series::{load_series_file, series_report};
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

fn series_json(
    task: &str,
    models: Vec<&str>,
    wins_target: i64,
    races: Vec<JsonValue>,
) -> JsonValue {
    obj(vec![
        ("task", s(task)),
        ("models", arr(models.into_iter().map(s).collect::<Vec<_>>())),
        ("wins_target", i(wins_target)),
        ("races", arr(races)),
    ])
}

fn write_json(dir: &Path, name: &str, data: &JsonValue) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, deadheat::json::dumps(data, None, true)).expect("write");
    path
}

fn write_text(dir: &Path, name: &str, text: &str) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, text).expect("write");
    path
}

fn championship_json(names: Vec<&str>) -> JsonValue {
    obj(vec![(
        "series",
        arr(names.into_iter().map(s).collect::<Vec<_>>()),
    )])
}

/// Write the named series files and one championship file
/// naming them into a fresh folder and return the loaded
/// championship report lines.
fn report_for(dir: &Path, files: Vec<(&str, JsonValue)>, names: Vec<&str>) -> Vec<String> {
    for (name, data) in &files {
        write_json(dir, name, data);
    }
    let path = write_json(dir, "championship.json", &championship_json(names));
    let (championship, defects) = load_championship_file(&path);
    assert_eq!(defects, Vec::<String>::new());
    championship_report(&championship.expect("a clean championship"))
}

fn defects_for(dir: &Path, files: Vec<(&str, JsonValue)>, names: Vec<&str>) -> Vec<String> {
    for (name, data) in &files {
        write_json(dir, name, data);
    }
    let path = write_json(dir, "championship.json", &championship_json(names));
    let (championship, defects) = load_championship_file(&path);
    assert!(championship.is_none());
    defects
}

fn table_lines(report: &[String]) -> Vec<String> {
    let start = report
        .iter()
        .position(|line| line == "championship table")
        .expect("a table");
    report[start + 1..].to_vec()
}

// The loader, DH-13

#[test]
fn a_valid_championship_loads_its_series_in_collection_order() {
    let dir = temp_dir("champ-loads");
    let first = series_json("the alpha saga", vec![PRIME, SHADOW], 2, vec![]);
    let second = series_json("the beta saga", vec![SHADOW, PRIME], 1, vec![]);
    write_json(&dir, "a.json", &first);
    write_json(&dir, "b.json", &second);
    let path = write_json(
        &dir,
        "championship.json",
        &championship_json(vec!["b.json", "a.json"]),
    );
    let (championship, defects) = load_championship_file(&path);
    assert_eq!(defects, Vec::<String>::new());
    let championship = championship.expect("a clean championship");
    assert_eq!(championship.series.len(), 2);
    assert_eq!(championship.series[0].task.display(), "the beta saga");
    assert_eq!(championship.series[1].task.display(), "the alpha saga");
}

#[test]
fn non_json_and_non_object_inputs_are_named() {
    let dir = temp_dir("champ-shape");
    let (_, defects) = load_championship_text("not json", &dir);
    assert_eq!(defects, vec!["input is not valid JSON"]);
    let (_, defects) = load_championship_text("[1, 2]", &dir);
    assert_eq!(defects, vec!["input is not a JSON object"]);
}

#[test]
fn load_championship_file_names_an_unreadable_path() {
    let dir = temp_dir("champ-unreadable");
    let (championship, defects) = load_championship_file(&dir.join("nope.json"));
    assert!(championship.is_none());
    assert_eq!(defects, vec!["file cannot be read"]);
}

#[test]
fn the_series_field_is_named_when_missing_misshapen_or_empty() {
    let dir = temp_dir("champ-field");
    let (_, defects) = load_championship_text("{}", &dir);
    assert_eq!(defects, vec!["missing field series"]);
    let text = deadheat::json::dumps(&obj(vec![("series", s("pilot.json"))]), None, true);
    let (_, defects) = load_championship_text(&text, &dir);
    assert_eq!(defects, vec!["field series has the wrong type"]);
    let text = deadheat::json::dumps(&championship_json(vec![]), None, true);
    let (_, defects) = load_championship_text(&text, &dir);
    assert_eq!(defects, vec!["field series holds no series files"]);
}

#[test]
fn series_entries_are_named_by_index() {
    let dir = temp_dir("champ-entries");
    let text = deadheat::json::dumps(
        &obj(vec![("series", arr(vec![i(7), s("ok.json")]))]),
        None,
        true,
    );
    let (_, defects) = load_championship_text(&text, &dir);
    assert_eq!(defects, vec!["series entry 0 has the wrong type"]);
    let text = deadheat::json::dumps(&championship_json(vec![""]), None, true);
    let (_, defects) = load_championship_text(&text, &dir);
    assert_eq!(defects, vec!["series entry 0 is empty"]);
    let text = "{\"series\": [\"bad\\ud800x.json\"]}";
    let (_, defects) = load_championship_text(text, &dir);
    assert_eq!(defects, vec!["series entry 0 holds an unpaired surrogate"]);
    let text = deadheat::json::dumps(&championship_json(vec!["two\nlines.json"]), None, true);
    let (_, defects) = load_championship_text(&text, &dir);
    assert_eq!(defects, vec!["series entry 0 holds a newline"]);
}

#[test]
fn an_unreadable_named_series_refuses_the_championship_whole() {
    let dir = temp_dir("champ-missing-series");
    let good = series_json("the alpha saga", vec![PRIME, SHADOW], 2, vec![]);
    let defects = defects_for(&dir, vec![("a.json", good)], vec!["a.json", "gone.json"]);
    assert_eq!(defects, vec!["series 1 file cannot be read"]);
}

#[test]
fn a_malformed_named_series_refuses_the_championship_whole() {
    let dir = temp_dir("champ-bad-series");
    let good = series_json("the alpha saga", vec![PRIME, SHADOW], 2, vec![]);
    let bad = series_json(
        "the beta saga",
        vec![PRIME, SHADOW],
        2,
        vec![clear_win("race one", PRIME, "intruder-9")],
    );
    let defects = defects_for(
        &dir,
        vec![("a.json", good), ("b.json", bad)],
        vec!["a.json", "b.json"],
    );
    assert_eq!(
        defects,
        vec!["series 1 race 0 names a model outside the series"]
    );
}

#[test]
fn a_relative_name_reads_against_the_championship_folder() {
    // The series stands beside the championship file in a
    // nested folder, and the working folder of the test
    // process stands elsewhere, so only the folder relative
    // reading finds it.
    let dir = temp_dir("champ-relative");
    let nested = dir.join("nested");
    std::fs::create_dir_all(&nested).expect("a nested folder");
    write_json(
        &nested,
        "a.json",
        &series_json("the alpha saga", vec![PRIME, SHADOW], 2, vec![]),
    );
    let path = write_json(
        &nested,
        "championship.json",
        &championship_json(vec!["a.json"]),
    );
    let (championship, defects) = load_championship_file(&path);
    assert_eq!(defects, Vec::<String>::new());
    assert_eq!(championship.expect("a championship").series.len(), 1);
}

#[test]
fn an_absolute_name_reads_as_it_stands() {
    let dir = temp_dir("champ-absolute");
    let series_path = write_json(
        &dir,
        "a.json",
        &series_json("the alpha saga", vec![PRIME, SHADOW], 2, vec![]),
    );
    let absolute = series_path.to_str().expect("utf8").to_string();
    let text = format!("{{\"series\": [\"{}\"]}}", absolute);
    let (championship, defects) = load_championship_text(&text, &dir.join("elsewhere"));
    assert_eq!(defects, Vec::<String>::new());
    assert_eq!(championship.expect("a championship").series.len(), 1);
}

#[test]
fn the_same_name_standing_twice_counts_the_series_twice() {
    let dir = temp_dir("champ-twice");
    let only = series_json(
        "the alpha saga",
        vec![PRIME, SHADOW],
        1,
        vec![clear_win("race one", PRIME, SHADOW)],
    );
    let report = report_for(&dir, vec![("a.json", only)], vec!["a.json", "a.json"]);
    assert_eq!(
        table_lines(&report),
        vec![
            "prime-1, 2 series won, 2 series stood in, 2 race wins, 0 no results, aggregate margin 60 seconds",
            "shadow-1, 0 series won, 2 series stood in, 0 race wins, 0 no results, aggregate margin 0 seconds",
        ]
    );
}

// The counting walk and the table, DH-13

#[test]
fn the_report_counts_counted_races_and_nothing_beyond_a_decision() {
    let dir = temp_dir("champ-counting");
    let alpha = series_json(
        "the alpha saga",
        vec![PRIME, SHADOW],
        2,
        vec![
            clear_win("race one", PRIME, SHADOW),
            no_result("race two", PRIME, SHADOW),
            clear_win("race three", SHADOW, PRIME),
            clear_win("race four", PRIME, SHADOW),
            clear_win("race five", SHADOW, PRIME),
        ],
    );
    let beta = series_json(
        "the beta saga",
        vec![SHADOW, PRIME],
        1,
        vec![
            clear_win("race one", SHADOW, PRIME),
            clear_win("race two", PRIME, SHADOW),
        ],
    );
    let report = report_for(
        &dir,
        vec![("a.json", alpha), ("b.json", beta)],
        vec!["a.json", "b.json"],
    );
    assert_eq!(
        report,
        vec![
            "championship",
            "series 1, the alpha saga",
            "prime-1 against shadow-1, first to 2 wins",
            "series winner, prime-1",
            "series 2, the beta saga",
            "shadow-1 against prime-1, first to 1 wins",
            "series winner, shadow-1",
            "championship table",
            "prime-1, 1 series won, 2 series stood in, 2 race wins, 1 no results, aggregate margin 60 seconds",
            "shadow-1, 1 series won, 2 series stood in, 2 race wins, 1 no results, aggregate margin 60 seconds",
        ]
    );
}

#[test]
fn an_unfinished_series_states_its_form_and_wins_no_series() {
    let dir = temp_dir("champ-unfinished");
    let alpha = series_json(
        "the alpha saga",
        vec![PRIME, SHADOW],
        3,
        vec![
            clear_win("race one", PRIME, SHADOW),
            no_result("race two", PRIME, SHADOW),
        ],
    );
    let report = report_for(&dir, vec![("a.json", alpha)], vec!["a.json"]);
    assert_eq!(
        report,
        vec![
            "championship",
            "series 1, the alpha saga",
            "prime-1 against shadow-1, first to 3 wins",
            "series unfinished, the target of 3 wins is not reached",
            "championship table",
            "prime-1, 0 series won, 1 series stood in, 1 race wins, 1 no results, aggregate margin 30 seconds",
            "shadow-1, 0 series won, 1 series stood in, 0 race wins, 1 no results, aggregate margin 0 seconds",
        ]
    );
}

#[test]
fn the_per_series_lines_equal_the_series_command_readings() {
    // For each recorded series the championship block carries
    // the models line and the closing line of that series own
    // report byte for byte.
    for name in ["pilot-series", "panel-series", "handicap-series"] {
        let source = examples().join(format!("{name}.json"));
        let (series, defects) = load_series_file(&source);
        assert_eq!(defects, Vec::<String>::new());
        let series = series.expect("a series");
        let dir = temp_dir("champ-mirror");
        let file_name = format!("{name}.json");
        std::fs::copy(&source, dir.join(&file_name)).expect("a copy");
        let path = write_json(
            &dir,
            "championship.json",
            &championship_json(vec![&file_name]),
        );
        let (championship, defects) = load_championship_file(&path);
        assert_eq!(defects, Vec::<String>::new());
        let report = championship_report(&championship.expect("a championship"));
        let own = series_report(&series);
        assert_eq!(report[2], own[1], "{name}");
        assert_eq!(report[3], own[own.len() - 1], "{name}");
    }
}

#[test]
fn a_win_with_an_unfinished_loser_counts_and_carries_no_margin() {
    let dir = temp_dir("champ-unfinished-loser");
    let open_win = obj(vec![
        ("task", s("race one")),
        (
            "attempts",
            arr(vec![
                attempt(PRIME, "green", 100.0, 10, "winner diff"),
                obj(vec![
                    ("model", s(SHADOW)),
                    ("environment", s("lane two")),
                    ("result", s("unfinished")),
                    ("tokens", i(20)),
                    ("patch", s("loser diff")),
                ]),
            ]),
        ),
    ]);
    let alpha = series_json("the alpha saga", vec![PRIME, SHADOW], 1, vec![open_win]);
    let report = report_for(&dir, vec![("a.json", alpha)], vec!["a.json"]);
    assert_eq!(
        table_lines(&report),
        vec![
            "prime-1, 1 series won, 1 series stood in, 1 race wins, 0 no results, aggregate margin 0 seconds",
            "shadow-1, 0 series won, 1 series stood in, 0 race wins, 0 no results, aggregate margin 0 seconds",
        ]
    );
}

#[test]
fn the_margin_of_a_handicapped_race_reads_adjusted_finishes() {
    let dir = temp_dir("champ-handicap-margin");
    let mut winner = attempt(SHADOW, "green", 120.0, 10, "winner diff");
    winner = set(&winner, "adjustment", f(-30.0));
    let race = obj(vec![
        ("task", s("race one")),
        (
            "attempts",
            arr(vec![
                winner,
                attempt_two(PRIME, "green", 100.0, 20, "loser diff"),
            ]),
        ),
    ]);
    // The scratch finishes give prime-1 the race by 20 seconds
    // and the adjustment gives shadow-1 the adjusted finish of
    // 90 seconds, so the counted margin is the 10 seconds
    // between the adjusted finishes and not the 20 between the
    // recorded finishes.
    let alpha = series_json("the alpha saga", vec![PRIME, SHADOW], 1, vec![race]);
    let report = report_for(&dir, vec![("a.json", alpha)], vec!["a.json"]);
    assert_eq!(
        table_lines(&report),
        vec![
            "shadow-1, 1 series won, 1 series stood in, 1 race wins, 0 no results, aggregate margin 10 seconds",
            "prime-1, 0 series won, 1 series stood in, 0 race wins, 0 no results, aggregate margin 0 seconds",
        ]
    );
}

// Exact arithmetic, DH-13 and DH-11

fn text_race(task: &str, winner: &str, loser: &str, win_finish: &str, lose_finish: &str) -> String {
    format!(
        "{{\"task\": \"{task}\", \"attempts\": [{{\"model\": \"{winner}\", \"environment\": \"lane one\", \"result\": \"green\", \"finished_at\": {win_finish}, \"tokens\": 10, \"patch\": \"winner diff\"}}, {{\"model\": \"{loser}\", \"environment\": \"lane two\", \"result\": \"red\", \"finished_at\": {lose_finish}, \"tokens\": 20, \"patch\": \"loser diff\"}}]}}"
    )
}

fn text_series(task: &str, target: i64, races: &[String]) -> String {
    format!(
        "{{\"task\": \"{task}\", \"models\": [\"prime-1\", \"shadow-1\"], \"wins_target\": {target}, \"races\": [{}]}}",
        races.join(", ")
    )
}

#[test]
fn the_aggregate_margin_sums_decimals_exactly() {
    let dir = temp_dir("champ-exact-decimals");
    // Two counted wins with margins of 0.1 and 0.2 seconds.
    // The exact sum is 0.3 seconds, and no float reading of
    // those literals carries that sum in its decimal text.
    let races = vec![
        text_race("race one", PRIME, SHADOW, "0.1", "0.2"),
        text_race("race two", PRIME, SHADOW, "0.3", "0.5"),
    ];
    write_text(&dir, "a.json", &text_series("the alpha saga", 2, &races));
    let path = write_json(
        &dir,
        "championship.json",
        &championship_json(vec!["a.json"]),
    );
    let (championship, defects) = load_championship_file(&path);
    assert_eq!(defects, Vec::<String>::new());
    let report = championship_report(&championship.expect("a championship"));
    assert_eq!(
        table_lines(&report),
        vec![
            "prime-1, 1 series won, 1 series stood in, 2 race wins, 0 no results, aggregate margin 0.3 seconds",
            "shadow-1, 0 series won, 1 series stood in, 0 race wins, 0 no results, aggregate margin 0 seconds",
        ]
    );
}

#[test]
fn the_aggregate_margin_holds_at_the_giant_scale() {
    let dir = temp_dir("champ-exact-giant");
    // Finishes at the 10^300 scale built by repetition, never
    // as a pasted literal. Each race is won by exactly one
    // second, and the exact sum over two races is two.
    let giant = format!("1{}", "0".repeat(300));
    let giant_plus = format!("1{}1", "0".repeat(299));
    let races = vec![
        text_race("race one", PRIME, SHADOW, &giant, &giant_plus),
        text_race("race two", PRIME, SHADOW, &giant, &giant_plus),
    ];
    write_text(&dir, "a.json", &text_series("the alpha saga", 2, &races));
    let path = write_json(
        &dir,
        "championship.json",
        &championship_json(vec!["a.json"]),
    );
    let (championship, defects) = load_championship_file(&path);
    assert_eq!(defects, Vec::<String>::new());
    let report = championship_report(&championship.expect("a championship"));
    assert_eq!(
        table_lines(&report),
        vec![
            "prime-1, 1 series won, 1 series stood in, 2 race wins, 0 no results, aggregate margin 2 seconds",
            "shadow-1, 0 series won, 1 series stood in, 0 race wins, 0 no results, aggregate margin 0 seconds",
        ]
    );
}

// The presentation order, DH-13

#[test]
fn series_won_orders_the_table_ahead_of_race_wins_and_margin() {
    let dir = temp_dir("champ-order-series");
    let one = series_json(
        "the alpha saga",
        vec![PRIME, SHADOW],
        1,
        vec![clear_win("race one", PRIME, SHADOW)],
    );
    let two = series_json(
        "the beta saga",
        vec![PRIME, SHADOW],
        1,
        vec![clear_win("race one", PRIME, SHADOW)],
    );
    let wide_win = obj(vec![
        ("task", s("race one")),
        (
            "attempts",
            arr(vec![
                attempt(SHADOW, "green", 100.0, 10, "winner diff"),
                attempt_two(PRIME, "red", 900.0, 20, "loser diff"),
            ]),
        ),
    ]);
    let three = series_json(
        "the gamma saga",
        vec![PRIME, SHADOW],
        3,
        vec![wide_win.clone(), wide_win.clone(), wide_win],
    );
    let report = report_for(
        &dir,
        vec![("a.json", one), ("b.json", two), ("c.json", three)],
        vec!["a.json", "b.json", "c.json"],
    );
    // Shadow-1 holds more race wins and the far larger margin
    // and still stands second, because prime-1 took two series
    // and shadow-1 took one.
    assert_eq!(
        table_lines(&report),
        vec![
            "prime-1, 2 series won, 3 series stood in, 2 race wins, 0 no results, aggregate margin 60 seconds",
            "shadow-1, 1 series won, 3 series stood in, 3 race wins, 0 no results, aggregate margin 2400 seconds",
        ]
    );
}

#[test]
fn race_wins_orders_the_table_ahead_of_margin() {
    let dir = temp_dir("champ-order-wins");
    let wide_win = obj(vec![
        ("task", s("race one")),
        (
            "attempts",
            arr(vec![
                attempt(SHADOW, "green", 100.0, 10, "winner diff"),
                attempt_two(PRIME, "red", 900.0, 20, "loser diff"),
            ]),
        ),
    ]);
    let alpha = series_json(
        "the alpha saga",
        vec![PRIME, SHADOW],
        5,
        vec![
            clear_win("race one", PRIME, SHADOW),
            clear_win("race two", PRIME, SHADOW),
            wide_win,
        ],
    );
    let report = report_for(&dir, vec![("a.json", alpha)], vec!["a.json"]);
    assert_eq!(
        table_lines(&report),
        vec![
            "prime-1, 0 series won, 1 series stood in, 2 race wins, 0 no results, aggregate margin 60 seconds",
            "shadow-1, 0 series won, 1 series stood in, 1 race wins, 0 no results, aggregate margin 800 seconds",
        ]
    );
}

#[test]
fn margin_orders_the_table_ahead_of_the_label() {
    let dir = temp_dir("champ-order-margin");
    let wide_win = obj(vec![
        ("task", s("race two")),
        (
            "attempts",
            arr(vec![
                attempt(SHADOW, "green", 100.0, 10, "winner diff"),
                attempt_two(PRIME, "red", 145.0, 20, "loser diff"),
            ]),
        ),
    ]);
    let alpha = series_json(
        "the alpha saga",
        vec![PRIME, SHADOW],
        5,
        vec![clear_win("race one", PRIME, SHADOW), wide_win],
    );
    let report = report_for(&dir, vec![("a.json", alpha)], vec!["a.json"]);
    // One win each and no series won, so the larger margin
    // stands first even though its label stands later.
    assert_eq!(
        table_lines(&report),
        vec![
            "shadow-1, 0 series won, 1 series stood in, 1 race wins, 0 no results, aggregate margin 45 seconds",
            "prime-1, 0 series won, 1 series stood in, 1 race wins, 0 no results, aggregate margin 30 seconds",
        ]
    );
}

#[test]
fn the_label_orders_the_table_when_every_figure_ties() {
    let dir = temp_dir("champ-order-label");
    // The models field names shadow-1 first and every figure
    // ties, so the standing label order alone places prime-1
    // first.
    let alpha = series_json(
        "the alpha saga",
        vec![SHADOW, PRIME],
        5,
        vec![
            clear_win("race one", PRIME, SHADOW),
            clear_win("race two", SHADOW, PRIME),
        ],
    );
    let report = report_for(&dir, vec![("a.json", alpha)], vec!["a.json"]);
    assert_eq!(
        table_lines(&report),
        vec![
            "prime-1, 0 series won, 1 series stood in, 1 race wins, 0 no results, aggregate margin 30 seconds",
            "shadow-1, 0 series won, 1 series stood in, 1 race wins, 0 no results, aggregate margin 30 seconds",
        ]
    );
}

// The example, DH-14

#[test]
fn the_championship_example_figures_are_the_hand_checked_figures() {
    // The figures stated in DH-14, hand checked against the
    // two series goldens and the records beneath them. The
    // pilot series counts three prime-1 wins against one
    // shadow-1 win with five no results and margins of 26.5
    // against 10, and the panel series counts three prime-1
    // wins with two no results and a prime-1 margin of 18.75.
    let (championship, defects) = load_championship_file(&examples().join("championship.json"));
    assert_eq!(defects, Vec::<String>::new());
    let report = championship_report(&championship.expect("a championship"));
    assert_eq!(
        report,
        vec![
            "championship",
            "series 1, the recorded close call pilot",
            "prime-1 against shadow-1, first to 3 wins",
            "series winner, prime-1",
            "series 2, the recorded close call panel",
            "prime-1 against shadow-1, first to 3 wins",
            "series winner, prime-1",
            "championship table",
            "prime-1, 2 series won, 2 series stood in, 6 race wins, 7 no results, aggregate margin 45.25 seconds",
            "shadow-1, 0 series won, 2 series stood in, 1 race wins, 7 no results, aggregate margin 10 seconds",
        ]
    );
}

// The championship command, Round 6 command line surface

#[test]
fn the_championship_command_prints_the_report() {
    let dir = temp_dir("champ-command");
    let alpha = series_json(
        "the alpha saga",
        vec![PRIME, SHADOW],
        1,
        vec![clear_win("race one", PRIME, SHADOW)],
    );
    write_json(&dir, "a.json", &alpha);
    let path = write_json(
        &dir,
        "championship.json",
        &championship_json(vec!["a.json"]),
    );
    let (out, _, code) = run_cli(&["championship", path.to_str().expect("utf8")]);
    assert_eq!(code, 0);
    let expected = report_for(
        &temp_dir("champ-command-again"),
        vec![("a.json", alpha)],
        vec!["a.json"],
    )
    .join("\n")
        + "\n";
    assert_eq!(out, expected);
}

#[test]
fn the_championship_command_names_the_defects_of_a_malformed_file() {
    let dir = temp_dir("champ-command-bad");
    let path = write_json(
        &dir,
        "championship.json",
        &championship_json(vec!["gone.json"]),
    );
    let (out, _, code) = run_cli(&["championship", path.to_str().expect("utf8")]);
    assert_eq!(code, 1);
    assert_eq!(
        lines_of(&out),
        vec![
            "invalid championship file",
            "defect series 0 file cannot be read",
        ]
    );
}

#[test]
fn the_championship_command_names_an_unreadable_file() {
    let dir = temp_dir("champ-command-missing");
    let missing = dir.join("nope.json");
    let (out, _, code) = run_cli(&["championship", missing.to_str().expect("utf8")]);
    assert_eq!(code, 1);
    assert!(out.contains("defect file cannot be read"));
}
