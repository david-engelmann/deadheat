//! Pins for DH-23 and DH-24 in the Round 11 section of
//! CONTRACT.md.
//!
//! The distance reading is pinned against hand computed
//! figures over constructed championships. The core case
//! stands a series gone the distance whose loser traded
//! races with its winner and holds a win beyond the
//! decision, the edges stand a series standing without
//! reply, a series standing in between, an unfinished
//! series, a series holding no decided race, and a series
//! holding no races, and the closing counts are pinned in
//! their stated relation over a mixed championship. The
//! target one series is pinned at the library surface, a
//! series whose loser reaches the target less one only
//! through races beyond the decision stands in between,
//! and a handicap series stands the distance on its
//! scratch finishes and without reply under its
//! adjustments, against the same record read both ways.
//! The split a section states is reconciled against the
//! series report of the same series.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{championship_report, load_championship_file, Championship};
use deadheat::comebacks::comeback_report;
use deadheat::distances::distance_report;
use deadheat::json::JsonValue;
use deadheat::series::series_report;
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

fn distance_for(dir: &Path, files: Vec<(&str, JsonValue)>, names: Vec<&str>) -> Vec<String> {
    distance_report(&load(dir, files, names))
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

/// A series gone the distance, its loser trading races
/// with its winner to the target less one, decided at the
/// fifth race with a sixth race standing beyond the
/// decision in the loser's favour.
fn distance_series() -> JsonValue {
    series_json(
        "the distance saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", BETA, ALPHA),
            clear_win("race five", ALPHA, BETA),
            clear_win("race six", BETA, ALPHA),
        ],
    )
}

/// A series its winner took without its loser winning a
/// decided race.
fn reply_series() -> JsonValue {
    series_json(
        "the reply saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", ALPHA, BETA),
        ],
    )
}

/// A series whose loser won one race under a target of
/// three, above none and below the target less one when
/// the decision fell.
fn between_series() -> JsonValue {
    series_json(
        "the between saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
        ],
    )
}

/// A series whose loser stands at the target less two
/// when the deciding race falls and reaches the target
/// less one only through the races beyond the decision,
/// which count for nothing.
fn late_series() -> JsonValue {
    series_json(
        "the late saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", BETA, ALPHA),
            clear_win("race six", BETA, ALPHA),
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

/// A second series gone the distance, between another
/// pair under a target of two.
fn second_distance_series() -> JsonValue {
    series_json(
        "the second distance saga",
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
fn a_series_whose_loser_traded_to_the_target_less_one_stands_the_distance() {
    let dir = temp_dir("distances-distance");
    let lines = distance_for(&dir, vec![("a.json", distance_series())], vec!["a.json"]);
    assert_eq!(
        lines,
        vec![
            "distance",
            "series 1, the distance saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, alpha-1 wins",
            "race 4, race four, beta-1 wins",
            "race 5, race five, alpha-1 wins",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 2 race wins, 0 no results",
            "distance, 1 series, 1 the distance, 0 without reply, 0 in between, 0 no winner",
        ]
    );
}

#[test]
fn a_series_taken_without_a_loser_win_stands_without_reply_and_in_no_section() {
    let dir = temp_dir("distances-reply");
    let lines = distance_for(&dir, vec![("a.json", reply_series())], vec!["a.json"]);
    assert_eq!(
        lines,
        vec![
            "distance",
            "distance, 1 series, 0 the distance, 1 without reply, 0 in between, 0 no winner"
        ]
    );
}

#[test]
fn a_series_whose_loser_stood_below_the_target_less_one_stands_in_between() {
    let dir = temp_dir("distances-between");
    let lines = distance_for(&dir, vec![("a.json", between_series())], vec!["a.json"]);
    assert_eq!(
        lines,
        vec![
            "distance",
            "distance, 1 series, 0 the distance, 0 without reply, 1 in between, 0 no winner"
        ]
    );
}

#[test]
fn a_loser_reaching_the_target_less_one_only_beyond_the_decision_stands_in_between() {
    // The wins of the loser are read over the counted
    // races alone, as the walk leaves them when the
    // deciding race fell. A reading over the whole record
    // would count the two wins beyond the decision, stand
    // the loser at the target, and misread the series at
    // every figure.
    let dir = temp_dir("distances-late");
    let championship = load(&dir, vec![("a.json", late_series())], vec!["a.json"]);
    let lines = distance_report(&championship);
    assert_eq!(
        lines,
        vec![
            "distance",
            "distance, 1 series, 0 the distance, 0 without reply, 1 in between, 0 no winner"
        ]
    );
    let report = series_report(&championship.series[0]);
    let tally = report
        .iter()
        .rev()
        .find(|line| line.starts_with("tally, "))
        .expect("a final tally")
        .clone();
    assert_eq!(tally, "tally, alpha-1 3 wins, beta-1 1 wins, 0 no results");
}

#[test]
fn an_unfinished_series_holding_decided_races_holds_no_winner() {
    let dir = temp_dir("distances-unfinished");
    let lines = distance_for(&dir, vec![("a.json", unfinished_series())], vec!["a.json"]);
    assert_eq!(
        lines,
        vec![
            "distance",
            "distance, 1 series, 0 the distance, 0 without reply, 0 in between, 1 no winner"
        ]
    );
}

#[test]
fn a_series_holding_no_decided_race_holds_no_winner() {
    let dir = temp_dir("distances-silent");
    let lines = distance_for(&dir, vec![("a.json", silent_series())], vec!["a.json"]);
    assert_eq!(
        lines,
        vec![
            "distance",
            "distance, 1 series, 0 the distance, 0 without reply, 0 in between, 1 no winner"
        ]
    );
}

#[test]
fn a_series_holding_no_races_holds_no_winner() {
    let dir = temp_dir("distances-empty");
    let lines = distance_for(&dir, vec![("a.json", empty_series())], vec!["a.json"]);
    assert_eq!(
        lines,
        vec![
            "distance",
            "distance, 1 series, 0 the distance, 0 without reply, 0 in between, 1 no winner"
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
    let lines = distance_report(&championship);
    assert_eq!(
        lines,
        vec![
            "distance",
            "distance, 0 series, 0 the distance, 0 without reply, 0 in between, 0 no winner"
        ]
    );
}

#[test]
fn a_series_of_target_one_stands_without_reply_and_never_the_distance() {
    // The loser of a decided series of target one stands
    // at none when the first decided race decides, and
    // none is the target less one of a target of one, so a
    // reading that matched the target less one before it
    // read the reply would stand this series the distance.
    // DH-23 stands it without reply.
    let dir = temp_dir("distances-single");
    let single = series_json(
        "the single saga",
        vec![ALPHA, BETA],
        1,
        vec![
            no_result("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
        ],
    );
    let lines = distance_for(&dir, vec![("a.json", single)], vec!["a.json"]);
    assert_eq!(
        lines,
        vec![
            "distance",
            "distance, 1 series, 0 the distance, 1 without reply, 0 in between, 0 no winner"
        ]
    );
}

#[test]
fn the_closing_counts_stand_in_their_stated_relation_over_a_mixed_championship() {
    let dir = temp_dir("distances-mixed");
    let championship = load(
        &dir,
        vec![
            ("a.json", distance_series()),
            ("b.json", reply_series()),
            ("c.json", between_series()),
            ("d.json", unfinished_series()),
            ("e.json", silent_series()),
        ],
        vec!["a.json", "b.json", "c.json", "d.json", "e.json"],
    );
    let lines = distance_report(&championship);
    let closing = lines.last().expect("a closing line").clone();
    assert_eq!(
        closing,
        "distance, 5 series, 1 the distance, 1 without reply, 1 in between, 2 no winner"
    );
    // The stated relations of DH-24. The no winner count
    // equals the unfinished blocks of the championship
    // report over the same collection and the no winner
    // figure the comeback closing leaves unstated, read
    // there as the series count less its two stated
    // classes, and the three classes of DH-23 sum to the
    // decided series both readings count.
    let fields: Vec<&str> = closing.split(", ").collect();
    let count = |field: &str| -> u64 {
        field
            .split(' ')
            .next()
            .expect("a count")
            .parse()
            .expect("a number")
    };
    let total = count(fields[1]);
    let the_distance = count(fields[2]);
    let without_reply = count(fields[3]);
    let in_between = count(fields[4]);
    let no_winner = count(fields[5]);
    let table = championship_report(&championship);
    let unfinished = table
        .iter()
        .filter(|line| line.starts_with("series unfinished"))
        .count() as u64;
    assert_eq!(unfinished, 2);
    assert_eq!(no_winner, unfinished);
    assert_eq!(the_distance + without_reply + in_between + no_winner, total);
    let comeback_closing = comeback_report(&championship)
        .last()
        .expect("a closing line")
        .clone();
    assert_eq!(
        comeback_closing,
        "comebacks, 5 series, 0 from behind, 3 from the front"
    );
    let decided = the_distance + without_reply + in_between;
    assert_eq!(decided, 3);
    assert_eq!(section_openings(&lines).len(), 1);
    assert!(lines.contains(&"series 1, the distance saga".to_string()));
}

#[test]
fn two_series_gone_the_distance_stand_two_sections_in_championship_order() {
    let dir = temp_dir("distances-two");
    let lines = distance_for(
        &dir,
        vec![
            ("a.json", reply_series()),
            ("b.json", distance_series()),
            ("c.json", second_distance_series()),
        ],
        vec!["a.json", "b.json", "c.json"],
    );
    let openings = section_openings(&lines);
    assert_eq!(
        openings,
        vec![
            "series 2, the distance saga",
            "series 3, the second distance saga"
        ]
    );
    assert!(lines.contains(
        &"series winner, beta-1, beta-1 2 race wins, gamma-1 1 race wins, 0 no results".to_string()
    ));
    assert_eq!(
        lines.last().expect("a closing line"),
        "distance, 3 series, 2 the distance, 1 without reply, 0 in between, 0 no winner"
    );
}

#[test]
fn the_section_of_a_series_states_the_split_the_series_report_states() {
    let dir = temp_dir("distances-split");
    let championship = load(&dir, vec![("a.json", distance_series())], vec!["a.json"]);
    let lines = distance_report(&championship);
    let report = series_report(&championship.series[0]);
    // The decided race lines of the section are the winner
    // lines of the series report over the counted races,
    // the report naming the sixth race beyond the decision
    // and never as a win.
    let report_winners: Vec<&String> = report
        .iter()
        .filter(|line| line.starts_with("race ") && line.ends_with(" wins"))
        .collect();
    let section_races: Vec<&String> = lines
        .iter()
        .filter(|line| line.starts_with("race "))
        .collect();
    assert_eq!(section_races, report_winners);
    assert_eq!(section_races.len(), 5);
    // The final tally of the series report states the split
    // the section closing line states.
    let tally = report
        .iter()
        .rev()
        .find(|line| line.starts_with("tally, "))
        .expect("a final tally")
        .clone();
    assert_eq!(tally, "tally, alpha-1 3 wins, beta-1 2 wins, 0 no results");
    assert!(lines.contains(
        &"series winner, alpha-1, alpha-1 3 race wins, beta-1 2 race wins, 0 no results"
            .to_string()
    ));
}

/// The twin series of the handicap pin. On its scratch
/// finishes the series stands the distance, beta-1 taking
/// the second race and alpha-1 the third at a target of
/// two. Under the adjustment the second race carries,
/// alpha-1 takes that race on its adjusted finish and
/// the target at the second race, leaving beta-1 at none.
fn twin_series(adjusted: bool) -> JsonValue {
    let mut second_alpha = attempt_two(ALPHA, "green", 110.0, Some(20));
    if adjusted {
        second_alpha = set(&second_alpha, "adjustment", i(-20));
    }
    series_json(
        "the twin saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", ALPHA, BETA),
            race_json(
                "race two",
                vec![attempt(BETA, "green", 100.0, Some(10)), second_alpha],
            ),
            clear_win("race three", ALPHA, BETA),
        ],
    )
}

#[test]
fn a_handicap_series_moves_between_classes_against_its_unadjusted_twin() {
    let dir = temp_dir("distances-twin-plain");
    let lines = distance_for(&dir, vec![("a.json", twin_series(false))], vec!["a.json"]);
    assert_eq!(
        lines,
        vec![
            "distance",
            "series 1, the twin saga",
            "alpha-1 against beta-1, first to 2 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, alpha-1 wins",
            "series winner, alpha-1, alpha-1 2 race wins, beta-1 1 race wins, 0 no results",
            "distance, 1 series, 1 the distance, 0 without reply, 0 in between, 0 no winner",
        ]
    );
    let dir = temp_dir("distances-twin-adjusted");
    let lines = distance_for(&dir, vec![("a.json", twin_series(true))], vec!["a.json"]);
    assert_eq!(
        lines,
        vec![
            "distance",
            "distance, 1 series, 0 the distance, 1 without reply, 0 in between, 0 no winner"
        ]
    );
}

#[test]
fn the_same_series_named_twice_in_one_championship_stands_twice() {
    let dir = temp_dir("distances-twice");
    let lines = distance_for(
        &dir,
        vec![("a.json", distance_series())],
        vec!["a.json", "a.json"],
    );
    let openings = section_openings(&lines);
    assert_eq!(
        openings,
        vec!["series 1, the distance saga", "series 2, the distance saga"]
    );
    assert_eq!(
        lines.last().expect("a closing line"),
        "distance, 2 series, 2 the distance, 0 without reply, 0 in between, 0 no winner"
    );
}

#[test]
fn the_distance_command_prints_the_reading() {
    let dir = temp_dir("distances-cli");
    write_json(&dir, "a.json", &distance_series());
    let path = write_json(
        &dir,
        "championship.json",
        &championship_json(vec!["a.json"]),
    );
    let (out, err, code) = run_cli(&["distance", path.to_str().expect("utf8")]);
    assert_eq!(code, 0);
    assert_eq!(err, "");
    assert!(out.starts_with("distance\n"), "{}", out);
    assert!(out.trim_end().ends_with(
        "distance, 1 series, 1 the distance, 0 without reply, 0 in between, 0 no winner"
    ));
}

#[test]
fn the_distance_command_refuses_a_malformed_championship() {
    let dir = temp_dir("distances-cli-bad");
    let path = write_json(
        &dir,
        "championship.json",
        &championship_json(vec!["gone.json"]),
    );
    let (out, _, code) = run_cli(&["distance", path.to_str().expect("utf8")]);
    assert_eq!(code, 1);
    assert_eq!(
        out,
        "invalid championship file\ndefect series 0 file cannot be read\n"
    );
    let missing = dir.join("nope.json");
    let (out, _, code) = run_cli(&["distance", missing.to_str().expect("utf8")]);
    assert_eq!(code, 1);
    assert_eq!(
        out,
        "invalid championship file\ndefect file cannot be read\n"
    );
}

#[test]
fn the_distance_reading_mutates_nothing() {
    let dir = temp_dir("distances-pure");
    let championship = load(&dir, vec![("a.json", distance_series())], vec!["a.json"]);
    let held = championship.clone();
    let first = distance_report(&championship);
    let second = distance_report(&championship);
    assert_eq!(first, second);
    assert_eq!(championship, held);
}
