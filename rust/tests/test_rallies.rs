//! Pins for DH-25 and DH-26 in the Round 12 section of
//! CONTRACT.md.
//!
//! The rally reading is pinned against hand computed
//! figures over constructed championships. The core case
//! stands a rally whose winner won the first decided race
//! and dipped below later, the class the comeback reading
//! leaves in its from the front count, beside a rally won
//! from behind with no result races interposed between its
//! decided races. The edges stand a series that only ever
//! stood level, a series of target one, a series holding
//! no winner, and a handicap series whose adjustment moves
//! it between wire to wire and a rally against its
//! unadjusted twin. The closing counts are pinned in their
//! stated relations against the comeback, distance, and
//! championship readings over one mixed championship, and
//! a series named twice in one championship is counted
//! once for each time it stands.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{championship_report, Championship};
use deadheat::comebacks::comeback_report;
use deadheat::distances::distance_report;
use deadheat::json::JsonValue;
use deadheat::rallies::rally_report;
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
    let (championship, defects) = deadheat::championship::load_championship_file(&path);
    assert_eq!(defects, Vec::<String>::new());
    championship.expect("a clean championship")
}

fn rally_for(dir: &Path, files: Vec<(&str, JsonValue)>, names: Vec<&str>) -> Vec<String> {
    rally_report(&load(dir, files, names))
}

/// A rally won from behind, its no result races standing
/// interposed between its decided races and moving no
/// tally. The winner loses the first decided race, then
/// takes three straight to the target of three.
fn interposed_series() -> JsonValue {
    series_json(
        "the interposed saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", BETA, ALPHA),
            no_result("race two", ALPHA, BETA),
            clear_win("race three", ALPHA, BETA),
            no_result("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
            clear_win("race six", ALPHA, BETA),
        ],
    )
}

/// A rally its winner did not win from behind. The winner
/// takes the first decided race, falls below over the next
/// two, and closes the series out, so the series stands a
/// rally and won from the front at once, the mid series
/// dip the DH-25 relations name.
fn dipped_series() -> JsonValue {
    series_json(
        "the dipped saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
        ],
    )
}

/// A series its winner never stood below in, standing
/// level after the second decided race and taking the
/// third. A level tally is not a deficit, so the series
/// stands wire to wire.
fn level_series() -> JsonValue {
    series_json(
        "the level saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
        ],
    )
}

/// A series of target one. Its only decided race is the
/// deciding race, so it stands wire to wire however it
/// falls.
fn target_one_series() -> JsonValue {
    series_json(
        "the single saga",
        vec![ALPHA, BETA],
        1,
        vec![clear_win("race one", BETA, ALPHA)],
    )
}

/// A series holding no winner, its two models trading one
/// race each under a target of three beside a race with no
/// result.
fn open_series() -> JsonValue {
    series_json(
        "the open saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            no_result("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
        ],
    )
}

#[test]
fn a_rally_with_no_results_interposed_keeps_its_classification() {
    let dir = temp_dir("rallies-interposed");
    let lines = rally_for(
        &dir,
        vec![("one.json", interposed_series())],
        vec!["one.json"],
    );
    assert_eq!(
        lines,
        vec![
            "rally",
            "series 1, the interposed saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, beta-1 wins",
            "race 3, race three, alpha-1 wins",
            "race 5, race five, alpha-1 wins",
            "race 6, race six, alpha-1 wins",
            "stood behind by 1 race wins at the deepest",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 1 race wins, 2 no results",
            "rally, 1 series, 1 rally, 0 wire to wire, 0 no winner",
        ]
    );
}

#[test]
fn a_series_won_from_the_front_can_stand_a_rally() {
    let dir = temp_dir("rallies-dipped");
    let championship = load(&dir, vec![("one.json", dipped_series())], vec!["one.json"]);
    let lines = rally_report(&championship);
    assert_eq!(
        lines,
        vec![
            "rally",
            "series 1, the dipped saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, beta-1 wins",
            "race 4, race four, alpha-1 wins",
            "race 5, race five, alpha-1 wins",
            "stood behind by 1 race wins at the deepest",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 2 race wins, 0 no results",
            "rally, 1 series, 1 rally, 0 wire to wire, 0 no winner",
        ]
    );
    // The same series stands won from the front under
    // DH-21, so the rally count of this championship stands
    // above its from behind count by the one dipped series,
    // the difference DH-25 states.
    let comebacks = comeback_report(&championship);
    assert_eq!(
        comebacks.last().map(|line| line.as_str()),
        Some("comebacks, 1 series, 0 from behind, 1 from the front")
    );
}

#[test]
fn a_series_that_only_stood_level_stands_wire_to_wire() {
    let dir = temp_dir("rallies-level");
    let lines = rally_for(&dir, vec![("one.json", level_series())], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "rally",
            "rally, 1 series, 0 rally, 1 wire to wire, 0 no winner"
        ]
    );
}

#[test]
fn a_series_of_target_one_stands_wire_to_wire() {
    let dir = temp_dir("rallies-target-one");
    let lines = rally_for(
        &dir,
        vec![("one.json", target_one_series())],
        vec!["one.json"],
    );
    assert_eq!(
        lines,
        vec![
            "rally",
            "rally, 1 series, 0 rally, 1 wire to wire, 0 no winner"
        ]
    );
}

#[test]
fn a_series_holding_no_winner_stands_in_neither_class() {
    let dir = temp_dir("rallies-open");
    let lines = rally_for(&dir, vec![("one.json", open_series())], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "rally",
            "rally, 1 series, 0 rally, 0 wire to wire, 1 no winner"
        ]
    );
}

/// The handicap twin. On scratch finishes alpha-1 takes
/// the first two races and the series stands wire to
/// wire. The first race's beta-1 attempt carries an
/// adjustment that moves its finish ahead of alpha-1's,
/// so the adjusted record opens with a beta-1 win and
/// alpha-1 stands behind before it closes the series
/// out in the third race.
fn handicap_series(adjusted: bool) -> JsonValue {
    let mut first_beta = attempt_two(BETA, "green", 110.0, Some(20));
    if adjusted {
        first_beta = set(&first_beta, "adjustment", i(-20));
    }
    series_json(
        "the handicap saga",
        vec![ALPHA, BETA],
        2,
        vec![
            race_json(
                "race one",
                vec![attempt(ALPHA, "green", 100.0, Some(10)), first_beta],
            ),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", ALPHA, BETA),
        ],
    )
}

#[test]
fn a_handicap_adjustment_moves_a_series_between_wire_to_wire_and_a_rally() {
    let dir = temp_dir("rallies-handicap");
    let plain = rally_for(
        &dir,
        vec![("one.json", handicap_series(false))],
        vec!["one.json"],
    );
    assert_eq!(
        plain,
        vec![
            "rally",
            "rally, 1 series, 0 rally, 1 wire to wire, 0 no winner"
        ]
    );
    let adjusted = rally_for(
        &dir,
        vec![("one.json", handicap_series(true))],
        vec!["one.json"],
    );
    assert_eq!(
        adjusted,
        vec![
            "rally",
            "series 1, the handicap saga",
            "alpha-1 against beta-1, first to 2 wins",
            "race 1, race one, beta-1 wins",
            "race 2, race two, alpha-1 wins",
            "race 3, race three, alpha-1 wins",
            "stood behind by 1 race wins at the deepest",
            "series winner, alpha-1, alpha-1 2 race wins, beta-1 1 race wins, 0 no results",
            "rally, 1 series, 1 rally, 0 wire to wire, 0 no winner",
        ]
    );
}

#[test]
fn the_closing_counts_stand_in_their_stated_relations() {
    // One mixed championship carrying every class, read by
    // the rally, comeback, distance, and championship
    // readings together, each stated relation of DH-25
    // checked against the figures the sibling readings
    // print over the same record.
    let dir = temp_dir("rallies-relations");
    let championship = load(
        &dir,
        vec![
            ("one.json", interposed_series()),
            ("two.json", dipped_series()),
            ("three.json", level_series()),
            ("four.json", target_one_series()),
            ("five.json", open_series()),
        ],
        vec![
            "one.json",
            "two.json",
            "three.json",
            "four.json",
            "five.json",
        ],
    );
    let rally = rally_report(&championship);
    assert_eq!(
        rally.last().map(|line| line.as_str()),
        Some("rally, 5 series, 2 rally, 2 wire to wire, 1 no winner")
    );
    // Every series won from behind stands a rally, and the
    // rally count stands above the from behind count by
    // the one dipped series and no more.
    let comebacks = comeback_report(&championship);
    assert_eq!(
        comebacks.last().map(|line| line.as_str()),
        Some("comebacks, 5 series, 1 from behind, 3 from the front")
    );
    // The no winner count is the count neither comeback
    // class holds, and the distance closing states the
    // same figure outright. No series standing without
    // reply stands a rally, the target one series standing
    // without reply here and wire to wire above.
    let distance = distance_report(&championship);
    assert_eq!(
        distance.last().map(|line| line.as_str()),
        Some("distance, 5 series, 2 the distance, 1 without reply, 1 in between, 1 no winner")
    );
    // The rally and wire to wire counts sum to the decided
    // series the championship reading counts as won.
    let table = championship_report(&championship);
    let won: Vec<&String> = table
        .iter()
        .filter(|line| line.starts_with("series winner,"))
        .collect();
    assert_eq!(won.len(), 4, "four decided series stand won");
}

#[test]
fn a_series_named_twice_is_counted_once_for_each_time_it_stands() {
    let dir = temp_dir("rallies-twice");
    let lines = rally_for(
        &dir,
        vec![("one.json", level_series())],
        vec!["one.json", "one.json"],
    );
    assert_eq!(
        lines,
        vec![
            "rally",
            "rally, 2 series, 0 rally, 2 wire to wire, 0 no winner"
        ]
    );
}

#[test]
fn a_series_decided_early_keeps_its_class_against_its_later_races() {
    // The decided record stops at the decision, so the
    // four beta-1 wins standing after alpha-1 closed the
    // series out move no tally. Read over every recorded
    // race instead, alpha-1 would stand behind by two
    // before the record ended and the series would stand
    // a rally, so this pin fails against that reading.
    let dir = temp_dir("rallies-beyond");
    let series = series_json(
        "the early saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", BETA, ALPHA),
            clear_win("race five", BETA, ALPHA),
            clear_win("race six", BETA, ALPHA),
        ],
    );
    let lines = rally_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "rally",
            "rally, 1 series, 0 rally, 1 wire to wire, 0 no winner"
        ]
    );
}
