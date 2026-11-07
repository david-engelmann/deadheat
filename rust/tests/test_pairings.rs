//! Pins for DH-19 and DH-20 in the Round 9 section of
//! CONTRACT.md.
//!
//! The pairing reading is pinned against hand computed
//! figures over the standing example and over constructed
//! championships, its closing figures are reconciled against
//! the championship table and the totals closing over the
//! same championship, the partition of the collection by
//! pair is pinned where one model stands in two pairs, pair
//! membership is pinned to the models field rather than the
//! attempt models, and the panel decision of DH-9 is pinned
//! on this surface.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{championship_report, load_championship_file, Championship};
use deadheat::json::JsonValue;
use deadheat::pairings::pairing_report;
use deadheat::totals::totals_report;
use shared::*;

const ALPHA: &str = "alpha-1";
const BETA: &str = "beta-1";
const GAMMA: &str = "gamma-1";

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

fn clear_win(task: &str, winner: &str, loser: &str) -> JsonValue {
    race_json(
        task,
        vec![
            attempt(winner, "green", 100.0, Some(10)),
            attempt_two(loser, "red", 130.0, Some(20)),
        ],
    )
}

fn clear_win_by(task: &str, winner: &str, loser: &str, loser_finish: f64) -> JsonValue {
    race_json(
        task,
        vec![
            attempt(winner, "green", 100.0, Some(10)),
            attempt_two(loser, "red", loser_finish, Some(20)),
        ],
    )
}

fn no_result(task: &str, first: &str, second: &str) -> JsonValue {
    race_json(
        task,
        vec![
            attempt(first, "red", 100.0, Some(10)),
            attempt_two(second, "red", 140.0, Some(20)),
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

fn pairing_for(dir: &Path, files: Vec<(&str, JsonValue)>, names: Vec<&str>) -> Vec<String> {
    pairing_report(&load(dir, files, names))
}

fn shared_pair_series() -> JsonValue {
    series_json(
        "the shared pair saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", ALPHA, BETA),
            no_result("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", BETA, ALPHA),
        ],
    )
}

fn second_pair_series() -> JsonValue {
    series_json(
        "the second pair saga",
        vec![GAMMA, ALPHA],
        1,
        vec![
            clear_win_by("race one", GAMMA, ALPHA, 110.0),
            clear_win("race two", ALPHA, GAMMA),
        ],
    )
}

fn leading_number(field: &str) -> u64 {
    field
        .split(' ')
        .next()
        .expect("a count")
        .parse()
        .expect("a number")
}

fn pairing_model_figures(lines: &[String]) -> Vec<(String, [u64; 3], String)> {
    let mut totals: Vec<(String, [u64; 3], String)> = Vec::new();
    for line in lines {
        if !line.contains(" series won, ") || !line.contains(" race wins, ") {
            continue;
        }
        if line.starts_with("pairings,") {
            continue;
        }
        let fields: Vec<&str> = line.split(", ").collect();
        if fields.len() != 5 {
            continue;
        }
        let model = fields[0].to_string();
        let figures = [
            leading_number(fields[1]),
            leading_number(fields[2]),
            leading_number(fields[3]),
        ];
        let margin = fields[4]
            .strip_prefix("aggregate margin ")
            .expect("a margin")
            .strip_suffix(" seconds")
            .expect("seconds")
            .to_string();
        match totals.iter_mut().find(|(held, _, _)| *held == model) {
            Some((_, held, held_margin)) => {
                held[0] += figures[0];
                held[1] += figures[1];
                held[2] += figures[2];
                // A model standing in two pairs in these pins
                // wins a margin in one pair alone, so the held
                // margin is the nonzero one.
                if *held_margin == "0" {
                    *held_margin = margin;
                } else {
                    assert_eq!(margin, "0");
                }
            }
            None => totals.push((model, figures, margin)),
        }
    }
    totals
}

fn championship_model_figures(lines: &[String]) -> Vec<(String, [u64; 3], String)> {
    let table = lines
        .iter()
        .position(|line| line == "championship table")
        .expect("a table");
    lines[table + 1..]
        .iter()
        .map(|line| {
            let fields: Vec<&str> = line.split(", ").collect();
            assert_eq!(fields.len(), 6, "{}", line);
            (
                fields[0].to_string(),
                [
                    leading_number(fields[1]),
                    leading_number(fields[3]),
                    leading_number(fields[4]),
                ],
                fields[5]
                    .strip_prefix("aggregate margin ")
                    .expect("a margin")
                    .strip_suffix(" seconds")
                    .expect("seconds")
                    .to_string(),
            )
        })
        .collect()
}

fn closing_counts(line: &str) -> (u64, u64) {
    // The closing forms read pairings, P pairings, S series,
    // R races and totals, S series, R races, M models. The
    // series and races counts are read off by their unit
    // words, since the two forms place them differently.
    let fields: Vec<&str> = line.split(", ").collect();
    let series = fields
        .iter()
        .find(|field| field.ends_with(" series"))
        .expect("a series count");
    let races = fields
        .iter()
        .find(|field| field.ends_with(" races"))
        .expect("a races count");
    (leading_number(series), leading_number(races))
}

// The example and the counting walk, DH-19 and DH-20

#[test]
fn the_pairing_example_figures_are_hand_checked() {
    let (championship, defects) = load_championship_file(&examples().join("championship.json"));
    assert_eq!(defects, Vec::<String>::new());
    let lines = pairing_report(&championship.expect("a clean championship"));
    assert_eq!(
        lines,
        vec![
            "pairings",
            "pairing, prime-1, against, shadow-1",
            "series read, 2",
            "races read, 14",
            "prime-1, 2 series won, 6 race wins, 7 no results, aggregate margin 45.25 seconds",
            "shadow-1, 0 series won, 1 race wins, 7 no results, aggregate margin 10 seconds",
            "pairings, 1 pairings, 2 series, 14 races",
        ]
    );
}

#[test]
fn the_pairing_reading_counts_only_its_own_pair_series() {
    let dir = temp_dir("pairings-two-pairs");
    let lines = pairing_for(
        &dir,
        vec![
            ("a.json", shared_pair_series()),
            ("b.json", second_pair_series()),
        ],
        vec!["a.json", "b.json"],
    );
    assert_eq!(
        lines,
        vec![
            "pairings",
            "pairing, alpha-1, against, beta-1",
            "series read, 1",
            "races read, 4",
            "alpha-1, 1 series won, 2 race wins, 1 no results, aggregate margin 60 seconds",
            "beta-1, 0 series won, 1 race wins, 1 no results, aggregate margin 30 seconds",
            "pairing, gamma-1, against, alpha-1",
            "series read, 1",
            "races read, 1",
            "gamma-1, 1 series won, 1 race wins, 0 no results, aggregate margin 10 seconds",
            "alpha-1, 0 series won, 0 race wins, 0 no results, aggregate margin 0 seconds",
            "pairings, 2 pairings, 2 series, 5 races",
        ]
    );
}

// The stated relations of DH-20

#[test]
fn the_pairing_figures_reconcile_with_the_championship_table_and_the_totals_closing() {
    let dir = temp_dir("pairings-relations");
    let championship = load(
        &dir,
        vec![
            ("a.json", shared_pair_series()),
            ("b.json", second_pair_series()),
        ],
        vec!["a.json", "b.json"],
    );
    let pairing_lines = pairing_report(&championship);
    let mut paired = pairing_model_figures(&pairing_lines);
    let mut tabled = championship_model_figures(&championship_report(&championship));
    paired.sort();
    tabled.sort();
    assert_eq!(paired, tabled);
    let totals_lines = totals_report(&championship);
    let pairing_closing = closing_counts(pairing_lines.last().expect("a closing line"));
    let totals_closing = closing_counts(totals_lines.last().expect("a closing line"));
    assert_eq!(pairing_closing.0, totals_closing.0);
    assert_eq!(pairing_closing.1, totals_closing.1);
    assert_eq!(pairing_closing, (2, 5));
}

#[test]
fn the_section_order_follows_first_standing_and_not_the_figures() {
    let dir = temp_dir("pairings-order");
    let quiet = series_json(
        "the quiet saga",
        vec!["zeta-1", "eta-1"],
        1,
        vec![no_result("race one", "zeta-1", "eta-1")],
    );
    let loud = series_json(
        "the loud saga",
        vec![ALPHA, BETA],
        1,
        vec![clear_win_by("race one", ALPHA, BETA, 1099.0)],
    );
    let lines = pairing_for(
        &dir,
        vec![("a.json", quiet), ("b.json", loud)],
        vec!["a.json", "b.json"],
    );
    let first_section = lines
        .iter()
        .position(|line| line.starts_with("pairing, "))
        .expect("a first section");
    let second_section = lines[first_section + 1..]
        .iter()
        .position(|line| line.starts_with("pairing, "))
        .expect("a second section")
        + first_section
        + 1;
    assert_eq!(lines[first_section], "pairing, zeta-1, against, eta-1");
    assert_eq!(lines[second_section], "pairing, alpha-1, against, beta-1");
    assert!(lines
        .iter()
        .any(|line| line.contains("aggregate margin 999 seconds")));
}

// Pair membership by the models field, DH-19

#[test]
fn a_series_holding_no_races_stands_its_pair_with_zero_counts() {
    let dir = temp_dir("pairings-empty-series");
    let empty = series_json("the empty saga", vec!["zeta-1", "eta-1"], 2, vec![]);
    let lines = pairing_for(&dir, vec![("a.json", empty)], vec!["a.json"]);
    assert_eq!(
        lines,
        vec![
            "pairings",
            "pairing, zeta-1, against, eta-1",
            "series read, 1",
            "races read, 0",
            "zeta-1, 0 series won, 0 race wins, 0 no results, aggregate margin 0 seconds",
            "eta-1, 0 series won, 0 race wins, 0 no results, aggregate margin 0 seconds",
            "pairings, 1 pairings, 1 series, 0 races",
        ]
    );
}

#[test]
fn a_series_naming_the_pair_in_reverse_joins_the_same_pair() {
    let dir = temp_dir("pairings-reversed");
    let first = series_json(
        "the first saga",
        vec![ALPHA, BETA],
        1,
        vec![clear_win("race one", ALPHA, BETA)],
    );
    let second = series_json(
        "the second saga",
        vec![BETA, ALPHA],
        1,
        vec![no_result("race one", BETA, ALPHA)],
    );
    let lines = pairing_for(
        &dir,
        vec![("a.json", first), ("b.json", second)],
        vec!["a.json", "b.json"],
    );
    assert_eq!(
        lines,
        vec![
            "pairings",
            "pairing, alpha-1, against, beta-1",
            "series read, 2",
            "races read, 2",
            "alpha-1, 1 series won, 1 race wins, 1 no results, aggregate margin 30 seconds",
            "beta-1, 0 series won, 0 race wins, 1 no results, aggregate margin 0 seconds",
            "pairings, 1 pairings, 2 series, 2 races",
        ]
    );
}

#[test]
fn a_panel_decided_race_counts_its_win_in_its_pair() {
    // Both attempts went green inside the close call band
    // and the faster attempt is alpha. The panel decides
    // for beta, so a reading deciding the race on finishes
    // alone states the win on the wrong side.
    let dir = temp_dir("pairings-panel");
    let mut race = race_json(
        "race one",
        vec![
            attempt(ALPHA, "green", 100.0, Some(10)),
            attempt_two(BETA, "green", 104.0, Some(20)),
        ],
    );
    race = set(
        &race,
        "panel",
        arr(vec![
            obj(vec![
                ("adjudicator", s("judge-a")),
                ("chosen", s(BETA)),
                ("confidence", f(0.9)),
                ("reason", s("the steadier run")),
            ]),
            obj(vec![
                ("adjudicator", s("judge-b")),
                ("chosen", s(BETA)),
                ("confidence", f(0.95)),
                ("reason", s("the cleaner patch")),
            ]),
        ]),
    );
    let series = series_json("the panel saga", vec![ALPHA, BETA], 1, vec![race]);
    let lines = pairing_for(&dir, vec![("a.json", series)], vec!["a.json"]);
    assert_eq!(
        lines,
        vec![
            "pairings",
            "pairing, alpha-1, against, beta-1",
            "series read, 1",
            "races read, 1",
            "alpha-1, 0 series won, 0 race wins, 0 no results, aggregate margin 0 seconds",
            "beta-1, 1 series won, 1 race wins, 0 no results, aggregate margin 4 seconds",
            "pairings, 1 pairings, 1 series, 1 races",
        ]
    );
}

// The loader refusal and the command surface

#[test]
fn a_label_carrying_a_newline_is_refused_before_any_reading() {
    let dir = temp_dir("pairings-fencing");
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
fn the_pairing_command_prints_the_reading() {
    let dir = temp_dir("pairings-cli");
    write_json(&dir, "a.json", &shared_pair_series());
    let path = write_json(
        &dir,
        "championship.json",
        &championship_json(vec!["a.json"]),
    );
    let (out, err, code) = run_cli(&["pairing", path.to_str().expect("utf8")]);
    assert_eq!(code, 0);
    assert_eq!(err, "");
    assert!(out.starts_with("pairings\n"), "{}", out);
    assert!(out
        .trim_end()
        .ends_with("pairings, 1 pairings, 1 series, 4 races"));
}

#[test]
fn the_pairing_command_refuses_a_malformed_championship() {
    let dir = temp_dir("pairings-cli-bad");
    let path = write_json(
        &dir,
        "championship.json",
        &championship_json(vec!["gone.json"]),
    );
    let (out, _, code) = run_cli(&["pairing", path.to_str().expect("utf8")]);
    assert_eq!(code, 1);
    assert_eq!(
        out,
        "invalid championship file\ndefect series 0 file cannot be read\n"
    );
    let missing = dir.join("nope.json");
    let (out, _, code) = run_cli(&["pairing", missing.to_str().expect("utf8")]);
    assert_eq!(code, 1);
    assert_eq!(
        out,
        "invalid championship file\ndefect file cannot be read\n"
    );
}

#[test]
fn the_pairing_reading_mutates_nothing() {
    let dir = temp_dir("pairings-pure");
    let championship = load(&dir, vec![("a.json", shared_pair_series())], vec!["a.json"]);
    let held = championship.clone();
    let first = pairing_report(&championship);
    let second = pairing_report(&championship);
    assert_eq!(first, second);
    assert_eq!(championship, held);
}
