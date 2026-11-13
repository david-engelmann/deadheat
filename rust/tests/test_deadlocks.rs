//! Pins for DH-29 and DH-30 in the Round 13 section of
//! CONTRACT.md.
//!
//! The discrimination record of this file stands pin by
//! pin, and no blanket claim stands over it. The deadlock
//! reading, its library function, and its command do not
//! stand on the pre round tree, so every pin in this file
//! is absent there at compile time. That absence is
//! recorded once here and is not a discrimination claim.
//! The discrimination each pin carries is against the
//! stated mutation of DH-29 beside it. Five mutations are
//! named for the whole file. M-A, the tallies read over
//! every recorded race, races beyond the decision among
//! them. On the folded tree M-A fails D-2, D-3, and
//! D-9. The folded helper counts the level standing at
//! the record's final position, where the pre fold loop
//! set the last entry aside before counting. The two
//! forms agree on every record the true walk leaves
//! and part only under the mutant walk past the
//! decision, where D-9's fixture stands level at its
//! final position past the decision. The Round 24
//! execution record proves this by its pre fold
//! control, which reproduces the stated set on the
//! pre fold tree. M-B, the standing before the first
//! decided race
//! read as a level standing, nought against nought counted
//! as level. M-C, stood level read as stood behind, the
//! rally test of DH-25 substituted for the level test of
//! DH-29. M-D, a no result race counted as a level
//! standing where the tallies stand equal after it, or
//! counted among a section's decided races. M-E, the
//! section figure read as the count of the decided race
//! at which the tallies first stood level instead of the
//! count of the decided races that left them level. One
//! pin, the constructed file under the rally command,
//! passes against the pre round surface by design, as its
//! own comment states.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{championship_report, load_championship_file, Championship};
use deadheat::comebacks::comeback_report;
use deadheat::deadlocks::deadlock_report;
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
    let (championship, defects) = load_championship_file(&path);
    assert_eq!(defects, Vec::<String>::new());
    championship.expect("a clean championship")
}

fn deadlock_for(dir: &Path, files: Vec<(&str, JsonValue)>, names: Vec<&str>) -> Vec<String> {
    deadlock_report(&load(dir, files, names))
}

fn load_example(name: &str) -> Championship {
    let (championship, defects) = load_championship_file(&examples().join(name));
    assert_eq!(defects, Vec::<String>::new());
    championship.expect("a championship")
}

fn golden(name: &str) -> String {
    std::fs::read_to_string(examples().join(name)).expect("a golden")
}

#[test]
fn the_standing_championship_renders_the_empty_form() {
    // D-1. The two standing series never stand level, so
    // the golden is the empty form. Fails under M-B, under
    // which both standing series stand deadlocked at the
    // opening standing and the report gains two sections.
    let championship = load_example("championship.json");
    let lines = deadlock_report(&championship);
    assert_eq!(
        lines,
        vec![
            "deadlock",
            "deadlock, 2 series, 0 deadlocked, 2 clear, 0 no winner",
        ]
    );
    assert_eq!(lines.join("\n") + "\n", golden("championship-deadlock.txt"));
}

#[test]
fn the_survey_championship_matches_its_deadlock_golden() {
    // D-2. The section form over the survey championship,
    // pinned byte for byte against the golden the real
    // command produced. Its stated failure mode is golden
    // drift, and the mutation discrimination over this
    // fixture stands in the relations pin below.
    let championship = load_example("survey-championship.json");
    let lines = deadlock_report(&championship);
    assert!(lines.len() > 2, "the survey golden stands in section form");
    assert_eq!(
        lines.join("\n") + "\n",
        golden("survey-championship-deadlock.txt")
    );
}

#[test]
fn the_constructed_example_stands_deadlocked_without_standing_behind() {
    // D-3. The round's headline discrimination. The
    // constructed winner won its first decided race, stood
    // level after its second, and never stood behind, so
    // the series stands deadlocked here. Fails under M-C,
    // under which the series renders no section and the
    // report reads as the empty form. This pin exists to
    // part DH-29 from DH-25.
    let championship = load_example("constructed-deadlock.json");
    let lines = deadlock_report(&championship);
    assert_eq!(
        lines,
        vec![
            "deadlock",
            "series 1, the constructed deadlock drill",
            "kappa-1 against lambda-1, first to 3 wins",
            "race 1, deadlock drill race one, kappa-1 wins",
            "race 2, deadlock drill race two, lambda-1 wins",
            "race 3, deadlock drill race three, kappa-1 wins",
            "race 4, deadlock drill race four, kappa-1 wins",
            "stood level after 1 of its decided races",
            "series winner, kappa-1, kappa-1 3 race wins, lambda-1 1 race wins, 0 no results",
            "deadlock, 1 series, 1 deadlocked, 0 clear, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-deadlock-deadlock.txt")
    );
}

#[test]
fn the_constructed_example_stands_wire_to_wire_under_rally() {
    // D-4. The same file under the rally command, the
    // empty form. Passes on the pre round tree by design,
    // in the posture of DH-28's goldens over standing
    // commands, the rally reading and its command standing
    // built there. The pin holds that the round's new
    // fixture does not move a standing reading and that
    // the two readings part over the same file, read
    // against the pin above.
    let championship = load_example("constructed-deadlock.json");
    let lines = rally_report(&championship);
    assert_eq!(
        lines,
        vec![
            "rally",
            "rally, 1 series, 0 rally, 1 wire to wire, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-deadlock-rally.txt")
    );
}

#[test]
fn a_series_of_target_one_stands_clear() {
    // D-5. The only decided race of a decided series of
    // target one is its deciding race, so no decided race
    // stands before it for the tallies to stand level on.
    // Fails under M-B, under which the opening standing
    // makes it deadlocked.
    let dir = temp_dir("deadlocks-target-one");
    let series = series_json(
        "the single saga",
        vec![ALPHA, BETA],
        1,
        vec![clear_win("race one", BETA, ALPHA)],
    );
    let lines = deadlock_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "deadlock",
            "deadlock, 1 series, 0 deadlocked, 1 clear, 0 no winner",
        ]
    );
}

#[test]
fn an_opening_no_result_makes_no_level_standing() {
    // D-6. The first recorded race holds no result and
    // moves no tally, and the winner then takes two
    // straight to the target of two. Fails under M-D,
    // under which the opening no result leaves the tallies
    // at nought against nought counted as a level standing
    // and the series stands deadlocked.
    let dir = temp_dir("deadlocks-opening-no-result");
    let series = series_json(
        "the quiet start saga",
        vec![ALPHA, BETA],
        2,
        vec![
            no_result("race one", ALPHA, BETA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", ALPHA, BETA),
        ],
    );
    let lines = deadlock_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "deadlock",
            "deadlock, 1 series, 0 deadlocked, 1 clear, 0 no winner",
        ]
    );
}

#[test]
fn an_interposed_no_result_leaves_the_figure_unchanged() {
    // D-7. The shape of the constructed example with a no
    // result recorded between its second and third decided
    // races. The series stands deadlocked with its section
    // figure unchanged and its section naming four decided
    // races and one no result. Fails under M-D in its
    // second form, a no result counted among the section's
    // decided races, which moves the section's race lines
    // and its closing split.
    let dir = temp_dir("deadlocks-interposed-no-result");
    let series = series_json(
        "the interposed deadlock saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            no_result("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
        ],
    );
    let lines = deadlock_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "deadlock",
            "series 1, the interposed deadlock saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, beta-1 wins",
            "race 4, race four, alpha-1 wins",
            "race 5, race five, alpha-1 wins",
            "stood level after 1 of its decided races",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 1 race wins, 1 no results",
            "deadlock, 1 series, 1 deadlocked, 0 clear, 0 no winner",
        ]
    );
}

#[test]
fn a_series_holding_no_winner_stands_in_neither_class_even_level() {
    // D-8. The decided winners run first model, second
    // model, and then no decided race, the target of three
    // unreached and the tallies stood level after the
    // second decided race. No section stands for it and
    // the closing counts it under no winner alone. Fails
    // under the mutation that drops the champion
    // requirement and classifies by the level test alone,
    // under which it stands deadlocked with a section.
    // This pin carries the comeback no winner form of
    // DH-21 and DH-29.
    let dir = temp_dir("deadlocks-open");
    let series = series_json(
        "the open saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            no_result("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
        ],
    );
    let lines = deadlock_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "deadlock",
            "deadlock, 1 series, 0 deadlocked, 0 clear, 1 no winner",
        ]
    );
}

#[test]
fn a_series_decided_early_keeps_its_class_against_its_later_races() {
    // D-9. The M-A discriminator in the form of the rally
    // build's decided early pin. The series is decided at
    // a target of two by two decided wins for its winner,
    // with two further recorded races beyond the decision
    // won by the loser, and stands clear, because the walk
    // counts nothing beyond the decision. Fails under
    // M-A, under which the recorded tallies pass through
    // two to one to two to two and the series stands
    // deadlocked. On the folded tree this failure
    // stands, the folded helper counting the level
    // standing at the record's final position where
    // the pre fold loop set the last entry aside, so
    // the level standing past the decision is counted
    // here and was set aside there. The two forms
    // agree on every record the true walk leaves and
    // part only under the mutant walk past the
    // decision, as the Round 24 execution record
    // proves by its pre fold control.
    let dir = temp_dir("deadlocks-beyond");
    let series = series_json(
        "the early saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", BETA, ALPHA),
        ],
    );
    let lines = deadlock_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "deadlock",
            "deadlock, 1 series, 0 deadlocked, 1 clear, 0 no winner",
        ]
    );
}

/// The handicap twin of the rally pins, read under this
/// reading. On scratch finishes alpha-1 takes the first
/// two races and the series stands clear. The first
/// race's beta-1 attempt carries an adjustment that moves
/// its finish ahead of alpha-1's, so the adjusted record
/// opens with a beta-1 win, the tallies stand level after
/// the second decided race, and alpha-1 closes the series
/// out in the third.
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
fn a_handicap_adjustment_moves_a_series_between_clear_and_deadlocked() {
    // D-10. The twin stands clear on its scratch winners
    // and deadlocked on its adjusted winners. Fails under
    // the mutation reading the scratch winners in place of
    // the adjusted winners the walk records, the standing
    // discipline every reading of this repo states.
    let dir = temp_dir("deadlocks-handicap");
    let plain = deadlock_for(
        &dir,
        vec![("one.json", handicap_series(false))],
        vec!["one.json"],
    );
    assert_eq!(
        plain,
        vec![
            "deadlock",
            "deadlock, 1 series, 0 deadlocked, 1 clear, 0 no winner",
        ]
    );
    let adjusted = deadlock_for(
        &dir,
        vec![("one.json", handicap_series(true))],
        vec!["one.json"],
    );
    assert_eq!(
        adjusted,
        vec![
            "deadlock",
            "series 1, the handicap saga",
            "alpha-1 against beta-1, first to 2 wins",
            "race 1, race one, beta-1 wins",
            "race 2, race two, alpha-1 wins",
            "race 3, race three, alpha-1 wins",
            "stood level after 1 of its decided races",
            "series winner, alpha-1, alpha-1 2 race wins, beta-1 1 race wins, 0 no results",
            "deadlock, 1 series, 1 deadlocked, 0 clear, 0 no winner",
        ]
    );
}

#[test]
fn the_section_figure_counts_every_level_standing() {
    // D-11. A series under a target of four whose decided
    // winners alternate first model, second model, first
    // model, second model, first model, second model,
    // first model, the first model winning the series four
    // to three and never standing behind, stands
    // deadlocked with its section line reading stood level
    // after 3 of its decided races. Fails under M-E, which
    // prints 2, the count of the decided race at the first
    // level standing. Fails under M-C, under which the
    // series stands in no section at all.
    let dir = temp_dir("deadlocks-alternating");
    let series = series_json(
        "the alternating saga",
        vec![ALPHA, BETA],
        4,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", BETA, ALPHA),
            clear_win("race five", ALPHA, BETA),
            clear_win("race six", BETA, ALPHA),
            clear_win("race seven", ALPHA, BETA),
        ],
    );
    let lines = deadlock_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "deadlock",
            "series 1, the alternating saga",
            "alpha-1 against beta-1, first to 4 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, alpha-1 wins",
            "race 4, race four, beta-1 wins",
            "race 5, race five, alpha-1 wins",
            "race 6, race six, beta-1 wins",
            "race 7, race seven, alpha-1 wins",
            "stood level after 3 of its decided races",
            "series winner, alpha-1, alpha-1 4 race wins, beta-1 3 race wins, 0 no results",
            "deadlock, 1 series, 1 deadlocked, 0 clear, 0 no winner",
        ]
    );
}

/// The stated relations of DH-30 over one championship,
/// read from the printed readings alone, in the form the
/// survey relations pin states them.
fn check_relations(championship: &Championship, series_count: u64) {
    let table = championship_report(championship);
    let decided = table
        .iter()
        .filter(|line| line.starts_with("series winner,"))
        .count() as u64;
    assert!(decided > 0, "the fixture holds decided series");
    let deadlock = deadlock_report(championship);
    let counts = closing_counts(deadlock.last().expect("a closing"));
    assert_eq!(counts[0], series_count);
    assert_eq!(counts[1] + counts[2], decided, "deadlocked and clear");
    assert_eq!(counts[1] + counts[2] + counts[3], series_count);
    // The no winner count equals the distance closing's
    // figure outright and the count neither comeback class
    // holds.
    let distance = distance_report(championship);
    let distance_counts = closing_counts(distance.last().expect("a closing"));
    assert_eq!(counts[3], distance_counts[4], "the no winner figure");
    let comebacks = comeback_report(championship);
    let comeback_counts = closing_counts(comebacks.last().expect("a closing"));
    assert_eq!(
        counts[3],
        series_count - (comeback_counts[1] + comeback_counts[2]),
        "the comeback leftover"
    );
    // Every rally series stood behind and so crossed a
    // level standing before the decision, so the rally
    // count stands at or below the deadlocked count.
    let rally = rally_report(championship);
    let rally_counts = closing_counts(rally.last().expect("a closing"));
    assert!(
        rally_counts[1] <= counts[1],
        "the rally count stands at or below the deadlocked count"
    );
    // No series standing without reply stands deadlocked,
    // its loser holding no decided win, so the deadlocked
    // count stands at or below the decided series less the
    // without reply count. This is the disjointness clause
    // that fails under M-B, under which every without
    // reply series stands deadlocked at the opening
    // standing.
    assert!(
        counts[1] <= decided - distance_counts[2],
        "no without reply series stands deadlocked"
    );
}

#[test]
fn the_closing_counts_stand_in_their_stated_relations() {
    // D-12. The relations of DH-30 over the survey
    // championship and the league championship of DH-31,
    // computed over the same readings. Fails under M-B in
    // its disjointness clause and under M-A on any fixture
    // series decided early whose recorded races beyond
    // the decision level its tallies, the decided early
    // pin above carrying that mutation on a hand built
    // series whether or not a fixture series does.
    check_relations(&load_example("survey-championship.json"), 6);
    check_relations(&load_example("league-championship.json"), 10);
}
