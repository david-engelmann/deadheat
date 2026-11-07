//! Pins for DH-21 and DH-22 in the Round 10 section of
//! CONTRACT.md.
//!
//! The comeback reading is pinned against hand computed
//! figures over constructed championships. The core case
//! stands a series won from behind whose first race holds
//! no result and whose second race its winner lost, the
//! edges stand a series won from the front, an unfinished
//! series holding decided races, a series holding no
//! decided race, and a series holding no races, each in
//! neither class, and the closing counts are pinned in
//! their stated relation over a mixed championship. The
//! split a section states is reconciled against the series
//! report of the same series, the handicap series of
//! DH-12 is pinned read under its adjustments, and a
//! series named twice in one championship stands twice.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{championship_report, load_championship_file, Championship};
use deadheat::comebacks::comeback_report;
use deadheat::json::JsonValue;
use deadheat::series::{load_series_file, series_report};
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

fn comeback_for(dir: &Path, files: Vec<(&str, JsonValue)>, names: Vec<&str>) -> Vec<String> {
    comeback_report(&load(dir, files, names))
}

/// The section opening lines of a report, the lines of the
/// form series {n}, {task}, with the section closing lines
/// and the unfinished lines of other readings left out.
fn section_openings(lines: &[String]) -> Vec<&String> {
    lines
        .iter()
        .filter(|line| {
            line.starts_with("series ")
                && !line.starts_with("series winner")
                && !line.starts_with("series unfinished")
        })
        .collect()
}

/// A series its winner lost the second race of, the first
/// race holding no result, decided at the fourth race with
/// a fifth race standing beyond the decision.
fn behind_series() -> JsonValue {
    series_json(
        "the comeback saga",
        vec![ALPHA, BETA],
        2,
        vec![
            no_result("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", BETA, ALPHA),
        ],
    )
}

/// A series its winner won the first decided race of.
fn front_series() -> JsonValue {
    series_json(
        "the front saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
        ],
    )
}

/// A series holding decided races whose target stands
/// unreached, so it holds no winner.
fn unfinished_series() -> JsonValue {
    series_json(
        "the unfinished saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", ALPHA, BETA),
            no_result("race three", ALPHA, BETA),
        ],
    )
}

/// A series holding no decided race at all.
fn silent_series() -> JsonValue {
    series_json(
        "the silent saga",
        vec![ALPHA, BETA],
        2,
        vec![
            no_result("race one", ALPHA, BETA),
            no_result("race two", ALPHA, BETA),
        ],
    )
}

/// A series holding no races.
fn empty_series() -> JsonValue {
    series_json("the empty saga", vec![ALPHA, BETA], 2, vec![])
}

/// A second series won from behind, between another pair.
fn second_behind_series() -> JsonValue {
    series_json(
        "the second comeback saga",
        vec![BETA, GAMMA],
        2,
        vec![
            clear_win("race one", GAMMA, BETA),
            clear_win("race two", BETA, GAMMA),
            clear_win("race three", BETA, GAMMA),
        ],
    )
}

#[test]
fn a_winner_losing_the_second_race_after_a_no_result_first_race_stands_won_from_behind() {
    let dir = temp_dir("comebacks-behind");
    let lines = comeback_for(&dir, vec![("a.json", behind_series())], vec!["a.json"]);
    assert_eq!(
        lines,
        vec![
            "comebacks",
            "series 1, the comeback saga",
            "alpha-1 against beta-1, first to 2 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, alpha-1 wins",
            "race 4, race four, alpha-1 wins",
            "first decided race, race 2, alpha-1 lost to beta-1",
            "series winner, alpha-1, alpha-1 2 race wins, beta-1 1 race wins, 1 no results",
            "comebacks, 1 series, 1 from behind, 0 from the front",
        ]
    );
}

#[test]
fn a_series_won_from_the_front_stands_in_no_section() {
    let dir = temp_dir("comebacks-front");
    let lines = comeback_for(&dir, vec![("a.json", front_series())], vec!["a.json"]);
    assert_eq!(
        lines,
        vec![
            "comebacks",
            "comebacks, 1 series, 0 from behind, 1 from the front"
        ]
    );
}

#[test]
fn an_unfinished_series_holding_decided_races_stands_in_neither_class() {
    let dir = temp_dir("comebacks-unfinished");
    let lines = comeback_for(&dir, vec![("a.json", unfinished_series())], vec!["a.json"]);
    assert_eq!(
        lines,
        vec![
            "comebacks",
            "comebacks, 1 series, 0 from behind, 0 from the front"
        ]
    );
}

#[test]
fn a_series_holding_no_decided_race_stands_in_neither_class() {
    let dir = temp_dir("comebacks-silent");
    let lines = comeback_for(&dir, vec![("a.json", silent_series())], vec!["a.json"]);
    assert_eq!(
        lines,
        vec![
            "comebacks",
            "comebacks, 1 series, 0 from behind, 0 from the front"
        ]
    );
}

#[test]
fn a_series_holding_no_races_stands_in_neither_class() {
    let dir = temp_dir("comebacks-empty");
    let lines = comeback_for(&dir, vec![("a.json", empty_series())], vec!["a.json"]);
    assert_eq!(
        lines,
        vec![
            "comebacks",
            "comebacks, 1 series, 0 from behind, 0 from the front"
        ]
    );
}

#[test]
fn a_collection_holding_no_series_renders_the_empty_form_at_the_library_surface() {
    // No championship file carries an empty collection,
    // because the loader of DH-13 refuses a series list
    // holding no names, so this form stands at the library
    // surface alone, over a collection built by hand.
    let championship = Championship { series: vec![] };
    let lines = comeback_report(&championship);
    assert_eq!(
        lines,
        vec![
            "comebacks",
            "comebacks, 0 series, 0 from behind, 0 from the front"
        ]
    );
}

#[test]
fn the_closing_counts_stand_in_their_stated_relation_over_a_mixed_championship() {
    let dir = temp_dir("comebacks-mixed");
    let championship = load(
        &dir,
        vec![
            ("a.json", behind_series()),
            ("b.json", front_series()),
            ("c.json", unfinished_series()),
            ("d.json", silent_series()),
            ("e.json", empty_series()),
        ],
        vec!["a.json", "b.json", "c.json", "d.json", "e.json"],
    );
    let lines = comeback_report(&championship);
    let closing = lines.last().expect("a closing line").clone();
    assert_eq!(
        closing,
        "comebacks, 5 series, 1 from behind, 1 from the front"
    );
    // The stated relation of DH-22. The series holding no
    // winner are read off the championship report over the
    // same collection, whose blocks close unfinished where
    // no winner stands, and the three classes sum to the
    // series count of the closing line.
    let table = championship_report(&championship);
    let unfinished = table
        .iter()
        .filter(|line| line.starts_with("series unfinished"))
        .count() as u64;
    assert_eq!(unfinished, 3);
    let fields: Vec<&str> = closing.split(", ").collect();
    let total: u64 = fields[1]
        .split(' ')
        .next()
        .expect("a count")
        .parse()
        .expect("a number");
    let behind: u64 = fields[2]
        .split(' ')
        .next()
        .expect("a count")
        .parse()
        .expect("a number");
    let front: u64 = fields[3]
        .split(' ')
        .next()
        .expect("a count")
        .parse()
        .expect("a number");
    assert_eq!(behind + front + unfinished, total);
    assert_eq!(section_openings(&lines).len(), 1);
    assert!(lines.contains(&"series 1, the comeback saga".to_string()));
}

#[test]
fn two_series_won_from_behind_stand_two_sections_in_championship_order() {
    let dir = temp_dir("comebacks-two");
    let lines = comeback_for(
        &dir,
        vec![
            ("a.json", front_series()),
            ("b.json", behind_series()),
            ("c.json", second_behind_series()),
        ],
        vec!["a.json", "b.json", "c.json"],
    );
    let openings = section_openings(&lines);
    assert_eq!(
        openings,
        vec![
            "series 2, the comeback saga",
            "series 3, the second comeback saga"
        ]
    );
    assert!(lines.contains(
        &"series winner, beta-1, beta-1 2 race wins, gamma-1 1 race wins, 0 no results".to_string()
    ));
    assert_eq!(
        lines.last().expect("a closing line"),
        "comebacks, 3 series, 2 from behind, 1 from the front"
    );
}

#[test]
fn the_section_of_a_series_states_the_split_the_series_report_states() {
    let dir = temp_dir("comebacks-split");
    let championship = load(&dir, vec![("a.json", behind_series())], vec!["a.json"]);
    let lines = comeback_report(&championship);
    let report = series_report(&championship.series[0]);
    // The decided race lines of the section are the winner
    // lines of the series report over the counted races,
    // the report naming no winner past the deciding race.
    let report_winners: Vec<&String> = report
        .iter()
        .filter(|line| line.starts_with("race ") && line.ends_with(" wins"))
        .collect();
    let section_races: Vec<&String> = lines
        .iter()
        .filter(|line| line.starts_with("race "))
        .collect();
    assert_eq!(section_races, report_winners);
    assert_eq!(section_races.len(), 3);
    // The final tally of the series report states the split
    // the section closing line states.
    let tally = report
        .iter()
        .rev()
        .find(|line| line.starts_with("tally, "))
        .expect("a final tally")
        .clone();
    assert_eq!(tally, "tally, alpha-1 2 wins, beta-1 1 wins, 1 no results");
    assert!(lines.contains(
        &"series winner, alpha-1, alpha-1 2 race wins, beta-1 1 race wins, 1 no results"
            .to_string()
    ));
}

#[test]
fn the_handicap_series_reads_won_from_behind_under_its_adjustments() {
    // The handicap series of DH-12 stands alone in a
    // championship built by hand here. On the scratch
    // finishes its second race goes to prime-1, which would
    // take the series from the front at the second race.
    // Under the adjustments DH-11 states, shadow-1 wins
    // that race and the series at the third race, having
    // lost the first decided race, so the section stands.
    let (series, defects) = load_series_file(&examples().join("handicap-series.json"));
    assert_eq!(defects, Vec::<String>::new());
    let championship = Championship {
        series: vec![series.expect("a clean series")],
    };
    let lines = comeback_report(&championship);
    assert_eq!(
        lines,
        vec![
            "comebacks",
            "series 1, the ledger import season",
            "prime-1 against shadow-1, first to 2 wins",
            "race 1, ledger import one, prime-1 wins",
            "race 2, balance the ledger import, shadow-1 wins",
            "race 3, ledger import three, shadow-1 wins",
            "first decided race, race 1, shadow-1 lost to prime-1",
            "series winner, shadow-1, prime-1 1 race wins, shadow-1 2 race wins, 0 no results",
            "comebacks, 1 series, 1 from behind, 0 from the front",
        ]
    );
}

#[test]
fn the_same_series_named_twice_in_one_championship_stands_twice() {
    let dir = temp_dir("comebacks-twice");
    let lines = comeback_for(
        &dir,
        vec![("a.json", behind_series())],
        vec!["a.json", "a.json"],
    );
    let openings = section_openings(&lines);
    assert_eq!(
        openings,
        vec!["series 1, the comeback saga", "series 2, the comeback saga"]
    );
    assert_eq!(
        lines.last().expect("a closing line"),
        "comebacks, 2 series, 2 from behind, 0 from the front"
    );
}

#[test]
fn the_comeback_command_prints_the_reading() {
    let dir = temp_dir("comebacks-cli");
    write_json(&dir, "a.json", &behind_series());
    let path = write_json(
        &dir,
        "championship.json",
        &championship_json(vec!["a.json"]),
    );
    let (out, err, code) = run_cli(&["comeback", path.to_str().expect("utf8")]);
    assert_eq!(code, 0);
    assert_eq!(err, "");
    assert!(out.starts_with("comebacks\n"), "{}", out);
    assert!(out
        .trim_end()
        .ends_with("comebacks, 1 series, 1 from behind, 0 from the front"));
}

#[test]
fn the_comeback_command_refuses_a_malformed_championship() {
    let dir = temp_dir("comebacks-cli-bad");
    let path = write_json(
        &dir,
        "championship.json",
        &championship_json(vec!["gone.json"]),
    );
    let (out, _, code) = run_cli(&["comeback", path.to_str().expect("utf8")]);
    assert_eq!(code, 1);
    assert_eq!(
        out,
        "invalid championship file\ndefect series 0 file cannot be read\n"
    );
    let missing = dir.join("nope.json");
    let (out, _, code) = run_cli(&["comeback", missing.to_str().expect("utf8")]);
    assert_eq!(code, 1);
    assert_eq!(
        out,
        "invalid championship file\ndefect file cannot be read\n"
    );
}

#[test]
fn the_comeback_reading_mutates_nothing() {
    let dir = temp_dir("comebacks-pure");
    let championship = load(&dir, vec![("a.json", behind_series())], vec!["a.json"]);
    let held = championship.clone();
    let first = comeback_report(&championship);
    let second = comeback_report(&championship);
    assert_eq!(first, second);
    assert_eq!(championship, held);
}
