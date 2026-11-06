//! Pins for DH-17 and DH-18 in the Round 8 section of
//! CONTRACT.md.
//!
//! The counting walk is pinned against hand computed figures
//! including the races beyond a decision counting for
//! nothing, the totals over one series are reconciled
//! against the outcome counts of the series reading itself,
//! the winner figures are pinned through the adjudication
//! step and the adjusted finishes so a reading deciding
//! races on the scratch finishes alone fails, the spend sum
//! is pinned over reported counts alone with the not
//! reported form standing where no count was reported, the
//! model order is pinned at first standing as an attempt's
//! model across series, and the fencing of the printed
//! fields is pinned at the loaders that refuse a label
//! carrying text across lines.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{load_championship_file, Championship};
use deadheat::json::JsonValue;
use deadheat::series::series_report;
use deadheat::totals::totals_report;
use shared::*;

const ALPHA: &str = "alpha-1";
const BETA: &str = "beta-1";

fn attempt(model: &str, result: &str, finished: f64, tokens: Option<i64>) -> JsonValue {
    obj(vec![
        ("model", s(model)),
        ("environment", s("lane one")),
        ("result", s(result)),
        ("finished_at", f(finished)),
        (
            "tokens",
            match tokens {
                Some(count) => i(count),
                None => JsonValue::Null,
            },
        ),
        ("patch", s("a diff")),
    ])
}

fn attempt_two(model: &str, result: &str, finished: f64, tokens: Option<i64>) -> JsonValue {
    let mut value = attempt(model, result, finished, tokens);
    value = set(&value, "environment", s("lane two"));
    value
}

fn race_json(task: &str, attempts: Vec<JsonValue>) -> JsonValue {
    obj(vec![("task", s(task)), ("attempts", arr(attempts))])
}

fn clear_win(
    task: &str,
    winner: &str,
    loser: &str,
    winner_tokens: i64,
    loser_tokens: i64,
) -> JsonValue {
    race_json(
        task,
        vec![
            attempt(winner, "green", 100.0, Some(winner_tokens)),
            attempt_two(loser, "red", 130.0, Some(loser_tokens)),
        ],
    )
}

fn no_result(task: &str, first_tokens: i64, second_tokens: i64) -> JsonValue {
    race_json(
        task,
        vec![
            attempt(ALPHA, "red", 100.0, Some(first_tokens)),
            attempt_two(BETA, "red", 140.0, Some(second_tokens)),
        ],
    )
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

fn championship_json(names: Vec<&str>) -> JsonValue {
    obj(vec![(
        "series",
        arr(names.into_iter().map(s).collect::<Vec<_>>()),
    )])
}

fn load(dir: &Path, files: Vec<(&str, JsonValue)>, names: Vec<&str>) -> Championship {
    for (name, data) in &files {
        write_json(dir, name, data);
    }
    let path = write_json(dir, "championship.json", &championship_json(names));
    let (championship, defects) = load_championship_file(&path);
    assert_eq!(defects, Vec::<String>::new());
    championship.expect("a clean championship")
}

fn totals_for(dir: &Path, files: Vec<(&str, JsonValue)>, names: Vec<&str>) -> Vec<String> {
    totals_report(&load(dir, files, names))
}

/// The five race series of the walk pin. The decision lands
/// on the fourth race, and the fifth stands beyond it with
/// token counts large enough to expose any walk that reads
/// past the decision.
fn walk_series() -> JsonValue {
    series_json(
        "the walk saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", ALPHA, BETA, 10, 20),
            no_result("race two", 30, 40),
            clear_win("race three", BETA, ALPHA, 50, 60),
            clear_win("race four", ALPHA, BETA, 70, 80),
            clear_win("race five", BETA, ALPHA, 900, 900),
        ],
    )
}

// The counting walk and the reading, DH-17 and DH-18

#[test]
fn the_totals_count_the_walk_and_stop_at_the_decision() {
    let dir = temp_dir("totals-walk");
    let lines = totals_for(&dir, vec![("a.json", walk_series())], vec!["a.json"]);
    assert_eq!(
        lines,
        vec![
            "totals",
            "alpha-1, 4 races, 2 wins, 1 losses, 1 no results, 170 tokens",
            "beta-1, 4 races, 1 wins, 2 losses, 1 no results, 190 tokens",
            "totals, 1 series, 4 races, 2 models",
        ]
    );
}

#[test]
fn the_totals_over_one_series_equal_the_series_reading_outcome_counts() {
    // The reconciliation the design names. The outcome
    // counts are read off the series report itself, race
    // line by race line, and the totals over a championship
    // holding that one series must state the same counts.
    let dir = temp_dir("totals-reconcile");
    let series = walk_series();
    let path = write_json(&dir, "a.json", &series);
    let (loaded, defects) = deadheat::series::load_series_file(&path);
    assert_eq!(defects, Vec::<String>::new());
    let report = series_report(&loaded.expect("a clean series"));
    let mut counted = 0usize;
    let mut wins: Vec<(&str, usize)> = vec![(ALPHA, 0), (BETA, 0)];
    let mut no_results = 0usize;
    for line in &report {
        if line.contains("beyond the decision") {
            continue;
        }
        if line.starts_with("race ") {
            counted += 1;
            if line.ends_with("no result") {
                no_results += 1;
            } else {
                for (model, count) in wins.iter_mut() {
                    if line.ends_with(&format!("{} wins", model)) {
                        *count += 1;
                    }
                }
            }
        }
    }
    let lines = totals_for(&dir, vec![("b.json", series)], vec!["b.json"]);
    let closing = lines.last().expect("a closing line").clone();
    assert_eq!(
        closing,
        format!("totals, 1 series, {} races, 2 models", counted)
    );
    for (model, count) in &wins {
        let line = lines
            .iter()
            .find(|line| line.starts_with(&format!("{}, ", model)))
            .expect("a model line")
            .clone();
        assert!(line.contains(&format!(", {} wins,", count)), "{}", line);
        assert!(
            line.contains(&format!(", {} no results,", no_results)),
            "{}",
            line
        );
    }
    // The closing figure relation: the races won summed over
    // the models are the races read less the no result races,
    // and the entered counts sum to two per counted race.
    let total_won: usize = wins.iter().map(|(_, count)| count).sum();
    assert_eq!(total_won + no_results, counted);
}

#[test]
fn the_winner_figures_read_the_adjudication_step_and_the_adjusted_finishes() {
    // Race one sits in the close call band and its entry
    // decides it for the slower finisher. Race two sits in
    // the band with no entry and stands as no result. Race
    // three is decided on the adjusted finishes against the
    // scratch order. A totals reading deciding any of these
    // on the scratch finishes alone states other figures.
    let dir = temp_dir("totals-steps");
    let adjudicated = {
        let mut race = race_json(
            "race one",
            vec![
                attempt(ALPHA, "green", 100.0, Some(11)),
                attempt_two(BETA, "green", 104.0, Some(12)),
            ],
        );
        race = set(
            &race,
            "adjudication",
            obj(vec![
                ("adjudicator", s("judge-1")),
                ("chosen", s(BETA)),
                ("confidence", f(0.8)),
                ("reason", s("the steadier run")),
            ]),
        );
        race
    };
    let undecided = race_json(
        "race two",
        vec![
            attempt(ALPHA, "green", 100.0, Some(13)),
            attempt_two(BETA, "green", 103.0, Some(14)),
        ],
    );
    let handicapped = {
        let mut loser = attempt(ALPHA, "green", 100.0, Some(15));
        loser = set(&loser, "environment", s("lane one"));
        let mut winner = attempt(BETA, "green", 120.0, Some(16));
        winner = set(&winner, "environment", s("lane two"));
        winner = set(&winner, "adjustment", i(-30));
        race_json("race three", vec![loser, winner])
    };
    let beyond = clear_win("race four", ALPHA, BETA, 900, 900);
    let series = series_json(
        "the steps saga",
        vec![ALPHA, BETA],
        2,
        vec![adjudicated, undecided, handicapped, beyond],
    );
    let lines = totals_for(&dir, vec![("a.json", series)], vec!["a.json"]);
    assert_eq!(
        lines,
        vec![
            "totals",
            "alpha-1, 3 races, 0 wins, 2 losses, 1 no results, 39 tokens",
            "beta-1, 3 races, 2 wins, 0 losses, 1 no results, 42 tokens",
            "totals, 1 series, 3 races, 2 models",
        ]
    );
}

#[test]
fn a_model_that_enters_and_never_wins_reads_zero_wins() {
    let dir = temp_dir("totals-never-wins");
    let series = series_json(
        "the one sided saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", ALPHA, BETA, 10, 20),
            clear_win("race two", ALPHA, BETA, 10, 20),
        ],
    );
    let lines = totals_for(&dir, vec![("a.json", series)], vec!["a.json"]);
    assert_eq!(
        lines[2],
        "beta-1, 2 races, 0 wins, 2 losses, 0 no results, 40 tokens"
    );
}

// The spend figures, DH-18

#[test]
fn the_spend_sums_the_reported_counts_alone() {
    let dir = temp_dir("totals-spend");
    let silent = race_json(
        "race one",
        vec![
            attempt(ALPHA, "green", 100.0, None),
            attempt_two(BETA, "red", 130.0, None),
        ],
    );
    let counted = clear_win("race two", ALPHA, BETA, 25, 35);
    let series = series_json(
        "the spend saga",
        vec![ALPHA, BETA],
        2,
        vec![silent, counted],
    );
    let lines = totals_for(&dir, vec![("a.json", series)], vec!["a.json"]);
    assert_eq!(
        lines[1],
        "alpha-1, 2 races, 2 wins, 0 losses, 0 no results, 25 tokens"
    );
    assert_eq!(
        lines[2],
        "beta-1, 2 races, 0 wins, 2 losses, 0 no results, 35 tokens"
    );
}

#[test]
fn a_model_reporting_no_count_reads_tokens_not_reported() {
    let dir = temp_dir("totals-unreported");
    let first = race_json(
        "race one",
        vec![
            attempt(ALPHA, "green", 100.0, None),
            attempt_two(BETA, "red", 130.0, Some(20)),
        ],
    );
    let second = race_json(
        "race two",
        vec![
            attempt(ALPHA, "green", 100.0, None),
            attempt_two(BETA, "red", 130.0, Some(20)),
        ],
    );
    let series = series_json("the quiet saga", vec![ALPHA, BETA], 2, vec![first, second]);
    let lines = totals_for(&dir, vec![("a.json", series)], vec!["a.json"]);
    assert_eq!(
        lines[1],
        "alpha-1, 2 races, 2 wins, 0 losses, 0 no results, tokens not reported"
    );
    assert!(!lines[1].contains(", 0 tokens"), "{}", lines[1]);
    assert_eq!(
        lines[2],
        "beta-1, 2 races, 0 wins, 2 losses, 0 no results, 40 tokens"
    );
}

// The model order and the collection shapes, DH-18

#[test]
fn the_model_order_is_first_standing_as_an_attempt_model() {
    // The championship names the second series first. Its
    // first race names alpha before gamma in record order,
    // so the order is alpha, gamma, beta even though beta
    // stands in the models field of the later series beside
    // alpha, and gamma never stands in that series at all.
    let dir = temp_dir("totals-order");
    let later = series_json(
        "the later saga",
        vec![ALPHA, BETA],
        1,
        vec![clear_win("race one", BETA, ALPHA, 10, 20)],
    );
    let earlier = series_json(
        "the earlier saga",
        vec!["gamma-1", ALPHA],
        1,
        vec![race_json(
            "race one",
            vec![
                attempt(ALPHA, "green", 100.0, Some(10)),
                attempt_two("gamma-1", "red", 130.0, Some(20)),
            ],
        )],
    );
    let lines = totals_for(
        &dir,
        vec![("a.json", later), ("b.json", earlier)],
        vec!["b.json", "a.json"],
    );
    let order: Vec<&str> = lines[1..4]
        .iter()
        .map(|line| line.split(',').next().expect("a model"))
        .collect();
    assert_eq!(order, vec!["alpha-1", "gamma-1", "beta-1"]);
    assert_eq!(lines[4], "totals, 2 series, 2 races, 3 models");
}

#[test]
fn a_series_holding_no_races_names_no_model_into_the_reading() {
    let dir = temp_dir("totals-empty-series");
    let empty = series_json("the empty saga", vec!["zeta-1", "eta-1"], 2, vec![]);
    let full = series_json(
        "the full saga",
        vec![ALPHA, BETA],
        1,
        vec![clear_win("race one", ALPHA, BETA, 10, 20)],
    );
    let lines = totals_for(
        &dir,
        vec![("a.json", empty), ("b.json", full)],
        vec!["a.json", "b.json"],
    );
    assert_eq!(
        lines,
        vec![
            "totals",
            "alpha-1, 1 races, 1 wins, 0 losses, 0 no results, 10 tokens",
            "beta-1, 1 races, 0 wins, 1 losses, 0 no results, 20 tokens",
            "totals, 2 series, 1 races, 2 models",
        ]
    );
}

#[test]
fn a_championship_of_empty_series_reads_zero_counts() {
    let dir = temp_dir("totals-empty");
    let empty = series_json("the empty saga", vec![ALPHA, BETA], 2, vec![]);
    let lines = totals_for(&dir, vec![("a.json", empty)], vec!["a.json"]);
    assert_eq!(lines, vec!["totals", "totals, 1 series, 0 races, 0 models"]);
}

#[test]
fn a_name_standing_twice_in_the_list_counts_twice() {
    let dir = temp_dir("totals-twice");
    let series = series_json(
        "the single saga",
        vec![ALPHA, BETA],
        1,
        vec![clear_win("race one", ALPHA, BETA, 10, 20)],
    );
    let lines = totals_for(&dir, vec![("a.json", series)], vec!["a.json", "a.json"]);
    assert_eq!(
        lines,
        vec![
            "totals",
            "alpha-1, 2 races, 2 wins, 0 losses, 0 no results, 20 tokens",
            "beta-1, 2 races, 0 wins, 2 losses, 0 no results, 40 tokens",
            "totals, 2 series, 2 races, 2 models",
        ]
    );
}

// The fencing of the printed fields, DH-18 under FIX-DH-13

#[test]
fn a_label_carrying_a_newline_is_refused_before_any_reading() {
    let dir = temp_dir("totals-fencing");
    let bad = series_json("the bad saga", vec!["two\nlines", BETA], 1, vec![]);
    write_json(&dir, "a.json", &bad);
    let path = write_json(
        &dir,
        "championship.json",
        &championship_json(vec!["a.json"]),
    );
    let (championship, defects) = load_championship_file(&path);
    assert!(championship.is_none());
    assert_eq!(defects, vec!["series 0 models entry 0 holds a newline"]);
}

#[test]
fn a_model_named_totals_mints_no_second_closing_line() {
    // The model line opens with the model label, in the
    // form the standings table already prints, and a model
    // named totals brings its line as close to the closing
    // form as a label can. The output still holds exactly
    // its four lines, told apart by their unit words.
    let dir = temp_dir("totals-namesake");
    let series = series_json(
        "the namesake saga",
        vec!["totals", "other-1"],
        1,
        vec![clear_win("race one", "totals", "other-1", 10, 20)],
    );
    let lines = totals_for(&dir, vec![("a.json", series)], vec!["a.json"]);
    assert_eq!(
        lines,
        vec![
            "totals",
            "totals, 1 races, 1 wins, 0 losses, 0 no results, 10 tokens",
            "other-1, 1 races, 0 wins, 1 losses, 0 no results, 20 tokens",
            "totals, 1 series, 1 races, 2 models",
        ]
    );
}

// The example, DH-18

#[test]
fn the_totals_example_figures_are_hand_checked() {
    let (championship, defects) = load_championship_file(&examples().join("championship.json"));
    assert_eq!(defects, Vec::<String>::new());
    let lines = totals_report(&championship.expect("a clean championship"));
    assert_eq!(
        lines,
        vec![
            "totals",
            "prime-1, 14 races, 6 wins, 1 losses, 7 no results, 1017000 tokens",
            "shadow-1, 14 races, 1 wins, 6 losses, 7 no results, 1073200 tokens",
            "totals, 2 series, 14 races, 2 models",
        ]
    );
}

// The command line surface, the Round 8 surface statement

#[test]
fn the_totals_command_prints_the_reading() {
    let dir = temp_dir("totals-cli");
    write_json(&dir, "a.json", &walk_series());
    let path = write_json(
        &dir,
        "championship.json",
        &championship_json(vec!["a.json"]),
    );
    let (out, err, code) = run_cli(&["totals", path.to_str().expect("utf8")]);
    assert_eq!(code, 0);
    assert_eq!(err, "");
    assert!(out.starts_with("totals\n"), "{}", out);
    assert!(out
        .trim_end()
        .ends_with("totals, 1 series, 4 races, 2 models"));
}

#[test]
fn the_totals_command_refuses_a_malformed_championship() {
    let dir = temp_dir("totals-cli-bad");
    let path = write_json(
        &dir,
        "championship.json",
        &championship_json(vec!["gone.json"]),
    );
    let (out, _, code) = run_cli(&["totals", path.to_str().expect("utf8")]);
    assert_eq!(code, 1);
    assert_eq!(
        out,
        "invalid championship file\ndefect series 0 file cannot be read\n"
    );
    let missing = dir.join("nope.json");
    let (out, _, code) = run_cli(&["totals", missing.to_str().expect("utf8")]);
    assert_eq!(code, 1);
    assert_eq!(
        out,
        "invalid championship file\ndefect file cannot be read\n"
    );
}

#[test]
fn the_totals_command_writes_nothing() {
    let dir = temp_dir("totals-cli-writes");
    let series_path = write_json(&dir, "a.json", &walk_series());
    let path = write_json(
        &dir,
        "championship.json",
        &championship_json(vec!["a.json"]),
    );
    let before: Vec<(String, Vec<u8>)> = [&series_path, &path]
        .iter()
        .map(|held| {
            (
                held.file_name()
                    .expect("a name")
                    .to_string_lossy()
                    .into_owned(),
                std::fs::read(held).expect("bytes"),
            )
        })
        .collect();
    let (_, _, code) = run_cli(&["totals", path.to_str().expect("utf8")]);
    assert_eq!(code, 0);
    let mut names: Vec<String> = std::fs::read_dir(&dir)
        .expect("a listing")
        .map(|entry| {
            entry
                .expect("an entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    names.sort();
    assert_eq!(names, vec!["a.json", "championship.json"]);
    for (name, bytes) in before {
        assert_eq!(std::fs::read(dir.join(name)).expect("bytes"), bytes);
    }
}

#[test]
fn the_totals_reading_mutates_nothing() {
    let dir = temp_dir("totals-pure");
    let championship = load(&dir, vec![("a.json", walk_series())], vec!["a.json"]);
    let held = championship.clone();
    let first = totals_report(&championship);
    let second = totals_report(&championship);
    assert_eq!(first, second);
    assert_eq!(championship, held);
}
