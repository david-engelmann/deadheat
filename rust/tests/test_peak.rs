//! Pins for DH-56 and DH-57 in the Round 29 section of
//! CONTRACT.md.
//!
//! The discrimination record of this file stands pin by
//! pin, and no blanket claim stands over it. The peak
//! reading, its library function, and its command do
//! not stand on the pre round tree, so every pin in
//! this file that names the peak reading is absent
//! there at compile time. That absence is recorded
//! once here and is not a discrimination claim. The
//! discrimination each pin carries is against the
//! stated mutation of DH-56 beside it. Ten mutations
//! are named for the whole file. W-1, the class cuts
//! exchanged, a series classing late where the shipped
//! reading classes it early and early where it classes
//! it late. W-2, the place read as the last attainment,
//! the count of decided races standing up to and
//! including the last decided race after which the lead
//! stood at the largest lead, in place of the first.
//! W-3, the place read as the count of the attainment
//! race in the record, a race holding no winner
//! counting in the count, in place of the count of
//! decided races standing up to it. W-4, the standings
//! read over a second winner walk, the decided record
//! rebuilt inside the peak module from the winner
//! decision read by attempts index in place of the
//! decided record the counting walk leaves, so races
//! recorded after the decision count in the lead and
//! the place. W-5, the lead read from the loser's seat,
//! the deficit of DH-48 in place of the winner's lead,
//! the entry returning the place of the depth and the
//! depth as its lead. W-6, the comparison read against
//! the decided count, a series classing early where its
//! place stands at or below half its decided count, the
//! half standing rounded down, and late where it stands
//! above it. W-7, the figure line printing the place in
//! place of the lead and the lead in place of the
//! place, the classes standing true. W-8, a series
//! holding no winner classed early at the
//! classification entry in place of standing in no
//! class. W-9, the section standing for every decided
//! series in place of the late series alone. W-10, the
//! decided races of a section renumbered as their order
//! among the decided races, the first decided race
//! standing as race 1 and each later decided race
//! standing one above the last, in place of the count
//! of the race in the record the walk leaves.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{load_championship_file, Championship};
use deadheat::depths::depths_report;
use deadheat::json::JsonValue;
use deadheat::peak::{peak_class, peak_report, PeakClass};
use deadheat::place::{place_class, PlaceClass};
use shared::*;

const ALPHA: &str = "alpha-1";
const BETA: &str = "beta-1";
const FINCH: &str = "finch-1";
const HARRIER: &str = "harrier-1";

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

fn adjudicated_win(task: &str, first: &str, second: &str, chosen: &str) -> JsonValue {
    let mut race = no_result(task, first, second);
    race = set(
        &race,
        "adjudication",
        obj(vec![
            ("adjudicator", s("judge-1")),
            ("chosen", s(chosen)),
            ("confidence", f(0.94)),
            ("reason", s("the steadier diff")),
        ]),
    );
    race
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

fn peak_for(dir: &Path, files: Vec<(&str, JsonValue)>, names: Vec<&str>) -> Vec<String> {
    peak_report(&load(dir, files, names))
}

fn load_example(name: &str) -> Championship {
    let (championship, defects) = load_championship_file(&examples().join(name));
    assert_eq!(defects, Vec::<String>::new());
    championship.expect("a championship")
}

fn golden(name: &str) -> String {
    std::fs::read_to_string(examples().join(name)).expect("a golden")
}

fn peak_figures(lines: &[String]) -> Vec<u64> {
    lines
        .iter()
        .filter_map(|line| {
            line.strip_prefix("largest lead ")
                .and_then(|rest| rest.split(", place ").nth(1))
                .and_then(|rest| rest.split(' ').next())
                .and_then(|figure| figure.parse::<u64>().ok())
        })
        .collect()
}

fn drill_one() -> JsonValue {
    series_json(
        "the constructed peak drill one",
        vec![FINCH, HARRIER],
        3,
        vec![
            no_result("peak drill one race one", FINCH, HARRIER),
            clear_win("peak drill one race two", HARRIER, FINCH),
            clear_win("peak drill one race three", FINCH, HARRIER),
            clear_win("peak drill one race four", FINCH, HARRIER),
            clear_win("peak drill one race five", HARRIER, FINCH),
            clear_win("peak drill one race six", FINCH, HARRIER),
        ],
    )
}

#[test]
fn the_standing_championship_stands_early_in_every_series() {
    // PK-1. The standing championship golden, no
    // section standing, closing "peak, 2 series, 2
    // early, 0 late, 0 no winner", the pilot standing
    // early at a place of two in a lead of two and the
    // panel early at a place of three in a lead of
    // three, neither winner's loser having won a
    // decided race before the lead stood first
    // attained. Fails under W-1, the two series
    // classing late and the closing reading no early
    // and two late. Fails under W-2, the lead of each
    // series standing attained at several standings
    // and the last attainment moving its place.
    // Fails under W-3, the attainment of each series
    // standing behind a race holding no winner inside
    // its counted record, so the count in the record
    // reads above the count of decided races. Fails
    // under W-4, the panel reading a lead of four
    // first standing at its sixth decided race over
    // the record its races after the decision extend.
    // Fails under W-6, the panel standing late at the
    // half count cut, its place of three standing
    // above half its decided count. Fails under W-9,
    // the two early series gaining sections. Passes
    // under W-5 by design, both series standing early
    // under both figures with no section and the same
    // closing. Passes under W-7 and W-10, no figure
    // line and no section standing for either to
    // move. Passes under W-8, no series holding no
    // winner.
    let championship = load_example("championship.json");
    let lines = peak_report(&championship);
    assert_eq!(
        lines,
        vec!["peak", "peak, 2 series, 2 early, 0 late, 0 no winner"]
    );
    assert_eq!(lines.join("\n") + "\n", golden("championship-peak.txt"));
}

#[test]
fn the_constructed_example_carries_the_late_series_alone() {
    // PK-2. The constructed peak golden, the first
    // series standing in section form at a place of
    // three in a lead of one, its winner finch-1
    // standing late, the second series standing early
    // at a place of two in a lead of two and taking
    // no section, closing "peak, 2 series, 1 early, 1
    // late, 0 no winner". Fails under W-1, the two
    // series exchanging classes and the section
    // moving to the second series. Fails under W-2,
    // the first series' place reading five, the last
    // of its two attainments of the lead of one.
    // Fails under W-3, the first series' attainment
    // standing behind the race holding no winner that
    // stands first in its record, its place reading
    // four by the count in the record. Fails under
    // W-5, the lead reading as the loser's largest
    // lead on both series, the first series' depth of
    // one first standing at its first decided race
    // and the second's depth of two first standing at
    // its sixth, the two series exchanging classes
    // and the section moving to the second series.
    // Fails under W-7, the figure line printing the
    // three in place of the one and the one in place
    // of the three. Fails under W-9, the early second
    // series gaining a section. Fails under W-10, the
    // section's races renumbering as their decided
    // order, the race holding no winner standing
    // inside the counted record before them. Passes
    // under W-4 by design, neither series recording
    // past its decision. Passes under W-6, both
    // classes standing unmoved at the half count cut
    // with the figures unchanged. Passes under W-8,
    // no series holding no winner.
    let championship = load_example("constructed-peak.json");
    let lines = peak_report(&championship);
    assert_eq!(
        lines,
        vec![
            "peak",
            "series 1, the constructed peak drill one",
            "finch-1 against harrier-1, first to 3 wins",
            "race 2, peak drill one race two, harrier-1 wins",
            "race 3, peak drill one race three, finch-1 wins",
            "race 4, peak drill one race four, finch-1 wins",
            "race 5, peak drill one race five, harrier-1 wins",
            "race 6, peak drill one race six, finch-1 wins",
            "largest lead 1, place 3 over the decided record",
            "series winner, finch-1, finch-1 3 race wins, harrier-1 2 race wins, 1 no results",
            "peak, 2 series, 1 early, 1 late, 0 no winner",
        ]
    );
    assert_eq!(lines.join("\n") + "\n", golden("constructed-peak-peak.txt"));
}

#[test]
fn the_survey_championship_matches_its_peak_golden() {
    // PK-3. The survey championship golden, sections
    // standing for apex and drift at a place of four,
    // bolt and crest at a place of five, bolt and
    // drift at a place of five, and crest and drift
    // at a place of four, its other two series
    // standing early, closing "peak, 6 series, 2
    // early, 4 late, 0 no winner". Fails under W-1,
    // the early and late series exchanging classes
    // and the sections moving to the two early
    // series. Fails under W-2, the lead of a sectioned
    // series standing attained at several standings
    // and its place moving to the last attainment.
    // Fails under W-3, an attainment standing behind
    // a race holding no winner inside its counted
    // record. Fails under W-4, a series reading a
    // larger lead over the record its races after the
    // decision extend. Fails under W-5, the sections
    // and figures reading from the loser's seat.
    // Fails under W-6, the apex and drift series
    // standing early at the half count cut, its place
    // of four standing at or below half its decided
    // count, while it stands late at its lead. Fails
    // under W-7, the figure lines printing their
    // places in place of their leads. Fails under
    // W-9, the two early series gaining sections.
    // Fails under W-10, a section's races renumbering
    // over a race holding no winner inside its
    // counted record. Passes under W-8, no series of
    // the six holding no winner.
    let championship = load_example("survey-championship.json");
    let lines = peak_report(&championship);
    assert_eq!(peak_figures(&lines), vec![4, 5, 5, 4]);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("peak, 6 series, 2 early, 4 late, 0 no winner")
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("survey-championship-peak.txt")
    );
}

#[test]
fn the_classification_reads_the_place_against_the_lead() {
    // PK-4. The classes at championship scale at hand
    // scale. A hand-built championship of three
    // series, the first won by alpha, beta, alpha
    // under a target of two standing early at a place
    // of one in a lead of one, the second won by
    // beta, alpha, alpha under a target of two
    // standing late at a place of three in a lead of
    // one, and the third holding decided races under
    // a target of three with its target unreached and
    // holding no winner, its whole report asserted
    // with the closing "peak, 3 series, 1 early, 1
    // late, 1 no winner" and the one section on the
    // second series, and the classification entry
    // asserted on all three. Fails under W-1, the
    // first two series exchanging classes and the
    // section moving to the first series. Fails under
    // W-2, the first series' lead standing attained
    // at its first and third decided races and its
    // place reading three, the series classing late.
    // Fails under W-5, the entries reading from the
    // loser's seat on the first two series. Fails
    // under W-8, the third series gaining the early
    // class at the entry. Passes under W-3, no race
    // of any series holding no winner, so the count
    // in the record and the count of decided races
    // agree at each attainment. Passes under W-4, no
    // series recording past its decision. Passes
    // under W-6, W-7, W-9, and W-10 by the fixture's
    // shape and the pin's seats, the classes standing
    // unmoved at the half count cut and the one
    // section already standing where W-9 would put
    // it, its races numbered as the record numbers
    // them.
    let dir = temp_dir("peak-classification");
    let first = series_json(
        "the early peak saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
        ],
    );
    let second = series_json(
        "the late peak saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", ALPHA, BETA),
        ],
    );
    let third = series_json(
        "the unreached saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
        ],
    );
    let championship = load(
        &dir,
        vec![
            ("one.json", first),
            ("two.json", second),
            ("three.json", third),
        ],
        vec!["one.json", "two.json", "three.json"],
    );
    assert_eq!(
        peak_class(&championship.series[0]),
        Some((PeakClass::Early, 1, 1))
    );
    assert_eq!(
        peak_class(&championship.series[1]),
        Some((PeakClass::Late, 3, 1))
    );
    assert_eq!(peak_class(&championship.series[2]), None);
    let lines = peak_report(&championship);
    assert_eq!(peak_figures(&lines), vec![3]);
    assert!(lines.contains(&"series 2, the late peak saga".to_string()));
    assert!(!lines.iter().any(|line| line.starts_with("series 1,")));
    assert!(!lines.iter().any(|line| line.starts_with("series 3,")));
    assert_eq!(
        lines.last().map(String::as_str),
        Some("peak, 3 series, 1 early, 1 late, 1 no winner")
    );
}

#[test]
fn a_series_at_the_least_place_stands_early() {
    // PK-5. The boundary of the classes at the least
    // place. A hand-built series at a target of two
    // won by alpha, beta, alpha stands at a lead of
    // one after its first decided race and never
    // leads by more, asserted early at a place of one
    // in a lead of one through the classification
    // entry. Fails under W-1, the series classing
    // late. Fails under W-2, its lead standing
    // attained at its first and third decided races
    // and its place reading three, the series classing
    // late. Fails under W-5, its entry reading a
    // place of nought in a lead of nought, its winner
    // never standing behind at any standing, where
    // the shipped reading classes it early at a place
    // of one in a lead of one. Passes
    // under W-3, its attainment race standing first in
    // the record as in the decided count. Passes under
    // W-6, its place of one standing at or below half
    // its decided count of three. Passes under W-4,
    // W-7, W-8, W-9, and W-10 by design, the entry
    // standing unmoved by each.
    let dir = temp_dir("peak-least-early");
    let series = series_json(
        "the least place saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
        ],
    );
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        peak_class(&championship.series[0]),
        Some((PeakClass::Early, 1, 1))
    );
}

#[test]
fn a_series_whose_loser_won_first_stands_late_at_lead_one() {
    // PK-6. The least late shape. A hand-built series
    // at a target of two won by beta, alpha, alpha,
    // its loser having won the first decided race and
    // its winner's lead of one standing first attained
    // only at the deciding race, asserted late at a
    // place of three in a lead of one through the
    // classification entry. Fails under W-1, the
    // series classing early. Fails under W-5, its
    // entry reading the place of its depth of one,
    // first standing at its first decided race, and
    // the series classing early at a place of one in
    // a lead of one. Passes under W-2, its lead
    // standing attained at one standing alone. Passes
    // under W-3, its attainment race standing third in
    // the record as in the decided count. Passes under
    // W-6, its place of three standing above half its
    // decided count of three. Passes under W-4, W-7,
    // W-8, W-9, and W-10 by design, the entry standing
    // unmoved by each.
    let dir = temp_dir("peak-least-late");
    let series = series_json(
        "the least late saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", ALPHA, BETA),
        ],
    );
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        peak_class(&championship.series[0]),
        Some((PeakClass::Late, 3, 1))
    );
}

#[test]
fn two_series_at_one_lead_part_on_the_place() {
    // PK-7. The twin of the tie in the lead. Two
    // hand-built series sharing the lead of two, the
    // first won by alpha, alpha, beta, alpha under a
    // target of three standing early at a place of
    // two, the second won by alpha, beta, alpha,
    // alpha under a target of three standing late at
    // a place of four, both asserted through the
    // classification entry. Fails under W-1 on both.
    // Fails under W-5 on both, neither series' winner
    // standing behind at any standing, so each entry
    // reads a place of nought in a lead of nought, the
    // first series' figures moving from two in two
    // and the second classing early where the shipped
    // reading classes it late at a place of four in a
    // lead of two. Fails under W-2 on the
    // first alone, its lead standing attained at its
    // second and fourth decided races and its place
    // reading four, the series classing late. Passes
    // under W-3, no race of either series holding no
    // winner. Passes under W-6, both classes standing
    // unmoved at the half count cut, the first series'
    // place of two standing at or below half its
    // decided count of four and the second's place of
    // four standing above half its decided count of
    // four. Passes under W-4, W-7, W-8, W-9, and W-10
    // by design, the entry standing unmoved by each.
    let dir = temp_dir("peak-twin-lead");
    let first = series_json(
        "the early lead saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
        ],
    );
    let second = series_json(
        "the late lead saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
        ],
    );
    let championship = load(
        &dir,
        vec![("one.json", first), ("two.json", second)],
        vec!["one.json", "two.json"],
    );
    assert_eq!(
        peak_class(&championship.series[0]),
        Some((PeakClass::Early, 2, 2))
    );
    assert_eq!(
        peak_class(&championship.series[1]),
        Some((PeakClass::Late, 4, 2))
    );
}

#[test]
fn the_place_names_the_first_attainment_not_the_last() {
    // PK-8. The first attainment against the last. A
    // hand-built series won by beta, alpha, alpha,
    // beta, alpha under a target of three, its lead of
    // one standing attained at its third decided race
    // and again at its fifth, asserted late at a place
    // of three through the classification entry and
    // through the figure line of its whole report.
    // Fails under W-1, the series classing early and
    // taking no section. Fails under W-2, its place
    // reading five, the last attainment. Fails under
    // W-5, its entry reading the place of its depth of
    // one, first standing at its first decided race,
    // and the series classing early. Passes under W-3,
    // no race holding no winner, so the count in the
    // record agrees with the count of decided races at
    // the attainment. Passes under W-6, its place of
    // three standing above half its decided count of
    // five. Passes under W-4 by design, the series
    // recording no race past its decision. Passes
    // under W-7, W-8, W-9, and W-10, the series already
    // standing sectioned and its races standing
    // numbered as the record numbers them.
    let dir = temp_dir("peak-first-attainment");
    let series = series_json(
        "the first attainment saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", BETA, ALPHA),
            clear_win("race five", ALPHA, BETA),
        ],
    );
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        peak_class(&championship.series[0]),
        Some((PeakClass::Late, 3, 1))
    );
    let lines = peak_report(&championship);
    assert!(lines.contains(&"largest lead 1, place 3 over the decided record".to_string()));
}

#[test]
fn a_series_whose_lead_first_stands_at_the_decision_stands_late() {
    // PK-9. The lead first standing at the decision.
    // A hand-built series won by beta, alpha, beta,
    // alpha, alpha under a target of three, its
    // winner's tally standing level with its loser's
    // after each of its first four decided races and
    // its lead of one standing first attained at the
    // deciding race, asserted late at a place of five
    // in a lead of one, its place standing at the count
    // of its decided races. Fails under W-1, the series
    // classing early. Fails under W-5, its entry
    // reading the place of its depth of one, first
    // standing at its first decided race, and the
    // series classing early at a place of one. Passes
    // under W-2, its lead standing attained at one
    // standing alone. Passes under W-6, its place of
    // five standing above half its decided count of
    // five. Passes under W-3, W-4, W-7, W-8, W-9, and
    // W-10 by design, the entry standing unmoved by
    // each.
    let dir = temp_dir("peak-lead-at-decision");
    let series = series_json(
        "the level saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
        ],
    );
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        peak_class(&championship.series[0]),
        Some((PeakClass::Late, 5, 1))
    );
}

#[test]
fn the_two_place_readings_part_on_one_series() {
    // PK-10. The two place readings parting on one
    // series. A hand-built series won by alpha, beta,
    // beta, alpha, alpha under a target of three,
    // asserted early at a place of one in a lead of one
    // through the peak entry, its winner's lead
    // standing first attained at its first decided
    // race, and late at a place of three in a depth of
    // one through the place entry on the same series,
    // its winner's deepest deficit standing first
    // attained at its third decided race, the two
    // readings parting on the extreme each reads the
    // place of. Fails under W-1, W-2, and W-5 on the
    // peak half, its lead standing attained at its
    // first and fifth decided races so the last
    // attainment moves its place under W-2, and its
    // entry reading the place of its depth under W-5.
    // Passes under W-6 and the rest by shape on the
    // peak half. Its place half stands under every
    // peak mutation by design.
    let dir = temp_dir("peak-place-parting");
    let series = series_json(
        "the parted places saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
        ],
    );
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        peak_class(&championship.series[0]),
        Some((PeakClass::Early, 1, 1))
    );
    assert_eq!(
        place_class(&championship.series[0]),
        Some((PlaceClass::Late, 3, 1))
    );
}

#[test]
fn an_adjudicated_race_counts_in_the_place() {
    // PK-11. The adjudication held entry at hand
    // scale. A hand-built series at a target of three
    // whose fifth race's attempts stand both red and
    // named by an adjudication entry for alpha, the
    // race decided to the named attempt under DH-5,
    // its winner's race sequence standing lost, won,
    // lost, won, adjudicated won, its lead of one
    // standing first attained at the adjudicated race,
    // asserted late at a place of five in a lead of one
    // with its whole report asserted in the pin, the
    // adjudicated race numbered five among its decided
    // races. Fails under W-1, the series classing early
    // and taking no section. Fails under W-5, its entry
    // reading the place of its depth of one, first
    // standing at its first decided race, and the
    // series classing early. Passes under W-2, its lead
    // standing attained at one standing alone. Passes
    // under W-3, its attainment race standing fifth in
    // the record as in the decided count. Passes under
    // W-6, its place of five standing above half its
    // decided count of five. Passes under W-4 and W-8
    // by design, and under W-7, W-9, and W-10, its one
    // decided series already standing sectioned and
    // its races standing numbered as the record
    // numbers them.
    let dir = temp_dir("peak-adjudicated");
    let series = series_json(
        "the adjudicated peak saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
            adjudicated_win("race five", ALPHA, BETA, ALPHA),
        ],
    );
    let lines = peak_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "peak",
            "series 1, the adjudicated peak saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, beta-1 wins",
            "race 2, race two, alpha-1 wins",
            "race 3, race three, beta-1 wins",
            "race 4, race four, alpha-1 wins",
            "race 5, race five, alpha-1 wins",
            "largest lead 1, place 5 over the decided record",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 2 race wins, 0 no results",
            "peak, 1 series, 0 early, 1 late, 0 no winner",
        ]
    );
}

#[test]
fn the_races_after_the_decision_count_in_neither_figure() {
    // PK-12. The tailed fixture. A hand-built series
    // won by alpha, alpha, beta, alpha under a target
    // of three and recording two further races after
    // its decision, both won by alpha, asserted early
    // at a place of two in a lead of two over the
    // decided record the walk leaves, the two recorded
    // races after the decision counting in neither
    // figure. Fails under W-1, the series classing
    // late. Fails under W-2, its lead standing attained
    // at its second and fourth decided races over the
    // decided record and its place reading four, the
    // last attainment there. Fails under W-4, the
    // extended record's lead of four first standing at
    // its sixth decided race over the full recorded
    // record where the decided record reads two at
    // two. Fails under W-5, its entry reading a place
    // of nought in a lead of nought, its winner never
    // standing behind at any standing, where the
    // shipped reading classes it early at a place of
    // two in a lead of two.
    // Passes under W-3, its attainment race standing
    // second in the record as in the decided count.
    // Passes under W-6, its place of two standing at or
    // below half its decided count of four. Passes
    // under W-7, W-8, W-9, and W-10 by design, the entry
    // standing unmoved by each.
    let dir = temp_dir("peak-tailed");
    let tailed = series_json(
        "the tailed peak saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
            clear_win("race six", ALPHA, BETA),
        ],
    );
    let championship = load(&dir, vec![("one.json", tailed)], vec!["one.json"]);
    assert_eq!(
        peak_class(&championship.series[0]),
        Some((PeakClass::Early, 2, 2))
    );
}

#[test]
fn the_record_count_stands_skipped_over_a_no_result_race() {
    // PK-13. The record count at hand scale, the pin
    // DH-57's stated relations name for this file. A
    // hand-built series on the constructed drill one's
    // shape, its first recorded race holding no winner,
    // its winner finch-1 standing at a lead of one
    // first attained at its third decided race, stands
    // late at a place of three in a lead of one, and
    // its whole report stands asserted, its section
    // printing its decided races at their counts in
    // the record, race 2 through race 6, the count of
    // the race holding no winner standing skipped,
    // while its place stands at three, a count of
    // decided races and not of the record, as the
    // stated relations of DH-57 state the two counts.
    // Fails under W-1, the series classing early and
    // taking no section. Fails under W-2, its place
    // reading five, the last of its two attainments.
    // Fails under W-3, its place reading four, the
    // count of its attainment race in the record.
    // Fails under W-5, its entry reading the place of
    // its depth of one, first standing at its first
    // decided race, and the series classing early.
    // Fails under W-7, its figure line printing the
    // place of three in place of the lead of one.
    // Fails under W-10, its decided races standing
    // renumbered as race 1 through race 5. Passes under
    // W-4 by design, the series recording no race past
    // its decision. Passes under W-6, its place of
    // three standing above half its decided count of
    // five. Passes under W-8 and W-9, no series holding
    // no winner and its one decided series already
    // standing sectioned.
    let dir = temp_dir("peak-record-count");
    let championship = load(&dir, vec![("one.json", drill_one())], vec!["one.json"]);
    let lines = peak_report(&championship);
    assert_eq!(
        lines,
        vec![
            "peak",
            "series 1, the constructed peak drill one",
            "finch-1 against harrier-1, first to 3 wins",
            "race 2, peak drill one race two, harrier-1 wins",
            "race 3, peak drill one race three, finch-1 wins",
            "race 4, peak drill one race four, finch-1 wins",
            "race 5, peak drill one race five, harrier-1 wins",
            "race 6, peak drill one race six, finch-1 wins",
            "largest lead 1, place 3 over the decided record",
            "series winner, finch-1, finch-1 3 race wins, harrier-1 2 race wins, 1 no results",
            "peak, 1 series, 0 early, 1 late, 0 no winner"
        ]
    );
}

#[test]
fn a_series_of_target_one_stands_early() {
    // PK-14. The target one sentence of DH-56 at hand
    // scale. A hand-built series decided at its first
    // race under a wins target of one stands early, its
    // only decided race being its deciding race and its
    // winner having won it, its largest lead standing
    // at one and its place at one with it, asserted
    // through the classification entry, with its whole
    // report asserted beside it, closing "peak, 1
    // series, 1 early, 0 late, 0 no winner". Fails
    // under W-1, the series classing late and the
    // closing reading one late. Fails under W-5, its
    // entry reading a place of nought in a lead of
    // nought, its winner never standing behind.
    // Fails under W-6, its place of one standing above
    // half its decided count of one, the half standing
    // at nought rounded down, and the series classing
    // late. Passes under W-2, its lead standing attained
    // at one standing alone. Passes under W-3, its
    // attainment race standing first in the record as
    // in the decided count. Passes under W-4, W-7, W-8,
    // W-9, and W-10 by design, no section standing and
    // no figure standing in the report for any of them
    // to move.
    let dir = temp_dir("peak-target-one");
    let series = series_json(
        "the single race saga",
        vec![ALPHA, BETA],
        1,
        vec![clear_win("race one", ALPHA, BETA)],
    );
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        peak_class(&championship.series[0]),
        Some((PeakClass::Early, 1, 1))
    );
    let lines = peak_report(&championship);
    assert_eq!(
        lines,
        vec!["peak", "peak, 1 series, 1 early, 0 late, 0 no winner"]
    );
}

#[test]
fn the_league_championship_matches_its_peak_golden() {
    // PK-15. The league championship golden, sections
    // standing for apex and bolt at a place of three,
    // apex and crest at a place of six, apex and drift
    // at a place of five, apex and ember at a place of
    // three, bolt and ember at a place of eight, and
    // crest and ember at a place of five, its other
    // four series standing early, closing "peak, 10
    // series, 4 early, 6 late, 0 no winner". Fails
    // under W-1, the early series classing late and
    // gaining sections as the late series lose theirs.
    // Fails under W-2, the lead of a sectioned series
    // standing attained at several standings and its
    // place moving to the last attainment. Fails under
    // W-3, an attainment standing behind a race holding
    // no winner inside its counted record. Fails under
    // W-4, a series reading a larger lead over the
    // record its races after the decision extend, the
    // crest and drift series extending to a lead of
    // six. Fails under W-5, the sections and figures
    // reading from the loser's seat. Fails under W-6,
    // a sectioned series standing early at the half
    // count cut while it stands late at its lead.
    // Fails under W-7, the figure lines printing their
    // places in place of their leads. Fails under W-9,
    // the four early series gaining sections. Fails
    // under W-10, a section's races renumbering over a
    // race holding no winner inside its counted record.
    // Passes under W-8, no series of the ten holding no
    // winner.
    let championship = load_example("league-championship.json");
    let lines = peak_report(&championship);
    assert_eq!(peak_figures(&lines), vec![3, 6, 5, 3, 8, 5]);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("peak, 10 series, 4 early, 6 late, 0 no winner")
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("league-championship-peak.txt")
    );
}

#[test]
fn the_osprey_series_stands_deep_and_early() {
    // PK-16. The deep and early witness on the shipped
    // record. The first series of the constructed place
    // example loaded from the examples, asserted early
    // at a place of one in a lead of one through the
    // peak entry, its winner's lead standing first
    // attained at its first decided race, with its
    // DH-48 class asserted deep beside it through the
    // depths closing over it alone, its depth of two
    // standing above its margin of one, a series whose
    // winner led from its first race and still dug the
    // deepest deficit of its record on the way. Fails
    // under W-1, the series classing late. Fails under
    // W-2, its lead of one standing attained at its
    // first and ninth decided races and its place
    // reading nine, the series classing late. Fails
    // under W-5, its entry reading the place of its
    // depth of two, first standing at its fourth
    // decided race, and the series classing late at a
    // place of four in a lead of two. Passes under W-3,
    // its attainment race standing first in the record
    // as in the decided count. Passes under W-4, its
    // extended record reading the same lead at the same
    // place. Passes under W-6, its place of one standing
    // at or below half its decided count of nine.
    // Passes under W-7, W-8, W-9, and W-10 by design,
    // the entry standing unmoved by each. The depths
    // half stands under every peak mutation by design.
    let championship = load_example("constructed-place.json");
    let series = championship.series[0].clone();
    assert_eq!(peak_class(&series), Some((PeakClass::Early, 1, 1)));
    let alone = Championship {
        series: vec![series],
    };
    let depth_lines = depths_report(&alone);
    assert_eq!(
        depth_lines.last().map(String::as_str),
        Some("depths, 1 series, 1 deep, 0 matched, 0 shallow, 0 no winner")
    );
}

#[test]
fn early_and_late_stand_across_the_place_classes() {
    // PK-17. The DH-54 relations of DH-56 at hand
    // scale. Four hand-built series, each loaded alone,
    // the first on the PK-10 shape standing early here
    // and late under DH-54, the second won by beta,
    // beta, alpha, beta, alpha, alpha, alpha under a
    // target of four standing late here at a place of
    // seven in a lead of one and early under DH-54 at
    // a place of two in a depth of two, the third won
    // by alpha, alpha, beta, alpha under a target of
    // three standing early here at a place of two in a
    // lead of two and never behind there, and the
    // fourth won by alpha, beta, alpha, alpha under a
    // target of three standing late here at a place of
    // four in a lead of two and never behind there, the
    // peak entry and the place entry asserted on each,
    // witnessing that a series standing early under
    // DH-54 stands late here and that a series standing
    // never behind there may stand in either class
    // here. Fails under W-1 and W-5 on all four, the
    // early and late classes exchanging under W-1 and
    // each series' entry reading from its loser's seat
    // under W-5. Fails under W-2 on the first and third
    // alone, each series' lead standing attained at
    // two standings and its place moving to the last
    // attainment. Passes under W-3 and W-4, no series
    // recording a race holding no winner inside its
    // counted record before its attainment and none
    // recording past its decision. Passes under W-6,
    // W-7, W-8, W-9, and W-10 by design, the entries
    // standing unmoved by each. The place halves stand
    // under every peak mutation by design.
    let dir = temp_dir("peak-place-relations");
    let first = series_json(
        "the parted places saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
        ],
    );
    let second = series_json(
        "the deep early saga",
        vec![ALPHA, BETA],
        4,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", BETA, ALPHA),
            clear_win("race five", ALPHA, BETA),
            clear_win("race six", ALPHA, BETA),
            clear_win("race seven", ALPHA, BETA),
        ],
    );
    let third = series_json(
        "the front early saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
        ],
    );
    let fourth = series_json(
        "the front late saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
        ],
    );
    let first_ch = load(&dir, vec![("one.json", first)], vec!["one.json"]);
    assert_eq!(
        peak_class(&first_ch.series[0]),
        Some((PeakClass::Early, 1, 1))
    );
    assert_eq!(
        place_class(&first_ch.series[0]),
        Some((PlaceClass::Late, 3, 1))
    );
    let second_ch = load(&dir, vec![("two.json", second)], vec!["two.json"]);
    assert_eq!(
        peak_class(&second_ch.series[0]),
        Some((PeakClass::Late, 7, 1))
    );
    assert_eq!(
        place_class(&second_ch.series[0]),
        Some((PlaceClass::Early, 2, 2))
    );
    let third_ch = load(&dir, vec![("three.json", third)], vec!["three.json"]);
    assert_eq!(
        peak_class(&third_ch.series[0]),
        Some((PeakClass::Early, 2, 2))
    );
    assert_eq!(
        place_class(&third_ch.series[0]),
        Some((PlaceClass::NeverBehind, 0, 0))
    );
    let fourth_ch = load(&dir, vec![("four.json", fourth)], vec!["four.json"]);
    assert_eq!(
        peak_class(&fourth_ch.series[0]),
        Some((PeakClass::Late, 4, 2))
    );
    assert_eq!(
        place_class(&fourth_ch.series[0]),
        Some((PlaceClass::NeverBehind, 0, 0))
    );
}

#[test]
fn a_collection_holding_no_series_renders_the_empty_form_at_the_library_surface() {
    // PK-18. The empty form, pinned from birth. No
    // championship file carries an empty collection,
    // because the loader of DH-13 refuses a series
    // list holding no names, so this form stands at
    // the library surface alone, over a collection
    // built by hand. The pin fails under no mutation
    // of the ten, the empty rendering being the two
    // lines DH-57 states and the mutation set carrying
    // no variant under which it changes.
    let championship = Championship { series: vec![] };
    let lines = peak_report(&championship);
    assert_eq!(
        lines,
        vec!["peak", "peak, 0 series, 0 early, 0 late, 0 no winner"]
    );
}

#[test]
fn the_lead_attained_at_several_standings_classes_by_the_first() {
    // PK-19. The several standings case of DH-56 at
    // the class boundary. A hand-built series at a
    // target of four won by alpha, alpha, beta, alpha,
    // beta, alpha, its lead of two standing attained at
    // its second, fourth, and sixth decided races,
    // asserted early at a place of two through the
    // classification entry, the class standing on the
    // first attainment alone as DH-56 states. Fails
    // under W-1, the series classing late. Fails under
    // W-2, its place reading six, the last attainment,
    // and the series classing late. Fails under W-5,
    // its entry reading a place of nought in a lead of
    // nought, its winner never standing behind.
    // Passes under W-3, its first attainment race
    // standing second in the record as in the decided
    // count. Passes under W-6, its place of two standing
    // at or below half its decided count of six. Passes
    // under W-4, W-7, W-8, W-9, and W-10 by design, the
    // entry standing unmoved by each.
    let dir = temp_dir("peak-several-standings");
    let series = series_json(
        "the several standings saga",
        vec![ALPHA, BETA],
        4,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", BETA, ALPHA),
            clear_win("race six", ALPHA, BETA),
        ],
    );
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        peak_class(&championship.series[0]),
        Some((PeakClass::Early, 2, 2))
    );
}

#[test]
fn two_series_part_at_the_least_lead() {
    // PK-20. The smallest parting of the classes. Two
    // hand-built series sharing the lead of one, the
    // first won by alpha, beta, alpha under a target
    // of two standing early at a place of one, the
    // second won by beta, alpha, alpha under a target
    // of two standing late at a place of three, the
    // least lead at which the classes part, both
    // asserted through the classification entry. Fails
    // under W-1 on both. Fails under W-5 on both, the
    // first series' entry reading a place of nought in
    // a lead of nought, its winner never standing
    // behind, and the second series' entry reading the
    // place of its depth of one, first standing at its
    // first decided race, and classing it early.
    // Fails under W-2 on the first alone, its lead
    // standing attained at its first and third decided
    // races and its place reading three, the series
    // classing late. Passes under W-3, no race of
    // either series holding no winner. Passes under
    // W-6, the first series' place of one standing at
    // or below half its decided count of three and the
    // second's place of three standing above half its
    // decided count of three. Passes under W-4, W-7,
    // W-8, W-9, and W-10 by design, the entry standing
    // unmoved by each.
    let dir = temp_dir("peak-least-parting");
    let first = series_json(
        "the whole lead saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
        ],
    );
    let second = series_json(
        "the parted lead saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", ALPHA, BETA),
        ],
    );
    let championship = load(
        &dir,
        vec![("one.json", first), ("two.json", second)],
        vec!["one.json", "two.json"],
    );
    assert_eq!(
        peak_class(&championship.series[0]),
        Some((PeakClass::Early, 1, 1))
    );
    assert_eq!(
        peak_class(&championship.series[1]),
        Some((PeakClass::Late, 3, 1))
    );
}
