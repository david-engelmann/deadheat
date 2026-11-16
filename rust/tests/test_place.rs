//! Pins for DH-54 and DH-55 in the Round 28 section of
//! CONTRACT.md.
//!
//! The discrimination record of this file stands pin by
//! pin, and no blanket claim stands over it. The place
//! reading, its library function, and its command do
//! not stand on the pre round tree, so every pin in
//! this file that names the place reading is absent
//! there at compile time. That absence is recorded
//! once here and is not a discrimination claim. The
//! discrimination each pin carries is against the
//! stated mutation of DH-54 beside it. Ten mutations
//! are named for the whole file. V-1, the class cuts
//! exchanged, a series classing late where the
//! shipped reading classes it early and early where
//! it classes it late, the never behind class
//! unmoved. V-2, the place read as the last
//! attainment, the count of decided races standing up
//! to and including the last decided race after which
//! the deficit stood at the depth, in place of the
//! first. V-3, the place read as the count of the
//! attainment race in the record, a race holding no
//! winner counting in the count, in place of the
//! count of decided races standing up to it. V-4, the
//! standings read over every decided race of the full
//! recorded record in place of the decided record the
//! walk leaves, so races recorded after the decision
//! count in the depth and the place. V-5, the deficit
//! read from the winner's seat, the depth standing as
//! the winner's largest lead and the place as the
//! first standing at which that lead stood at its
//! largest. V-6, the comparison read against the
//! decided count, a series classing early where its
//! place stands at or below half its decided count,
//! the half standing rounded down, and late where it
//! stands above it, the never behind class unmoved.
//! V-7, the figure line printing the place in place
//! of the depth and the depth in place of the place,
//! the classes standing true. V-8, a series holding
//! no winner placed in the never behind class in
//! place of standing in no class. V-9, the section
//! standing for every series bearing a depth in
//! place of the late series alone. V-10, the decided
//! races of a section renumbered as their order among
//! the decided races, the first decided race standing
//! as race 1 and each later decided race standing one
//! above the last, in place of the count of the race
//! in the record the walk leaves.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{load_championship_file, Championship};
use deadheat::depths::depths_report;
use deadheat::json::JsonValue;
use deadheat::openings::opening_report;
use deadheat::place::{place_class, place_report, PlaceClass};
use deadheat::reigns::{reign_class, ReignClass};
use deadheat::spells::{spell_class, SpellClass};
use shared::*;

const ALPHA: &str = "alpha-1";
const BETA: &str = "beta-1";
const OSPREY: &str = "osprey-1";
const KESTREL: &str = "kestrel-1";
const HOBBY: &str = "hobby-1";
const MERLIN: &str = "merlin-1";

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

fn place_for(dir: &Path, files: Vec<(&str, JsonValue)>, names: Vec<&str>) -> Vec<String> {
    place_report(&load(dir, files, names))
}

fn load_example(name: &str) -> Championship {
    let (championship, defects) = load_championship_file(&examples().join(name));
    assert_eq!(defects, Vec::<String>::new());
    championship.expect("a championship")
}

fn golden(name: &str) -> String {
    std::fs::read_to_string(examples().join(name)).expect("a golden")
}

fn place_figures(lines: &[String]) -> Vec<u64> {
    lines
        .iter()
        .filter_map(|line| {
            line.strip_prefix("deepest deficit ")
                .and_then(|rest| rest.split(", place ").nth(1))
                .and_then(|rest| rest.split(' ').next())
                .and_then(|figure| figure.parse::<u64>().ok())
        })
        .collect()
}

fn drill_one() -> JsonValue {
    series_json(
        "the constructed place drill one",
        vec![OSPREY, KESTREL],
        5,
        vec![
            clear_win("place drill one race one", OSPREY, KESTREL),
            clear_win("place drill one race two", KESTREL, OSPREY),
            clear_win("place drill one race three", KESTREL, OSPREY),
            clear_win("place drill one race four", KESTREL, OSPREY),
            clear_win("place drill one race five", OSPREY, KESTREL),
            clear_win("place drill one race six", KESTREL, OSPREY),
            clear_win("place drill one race seven", OSPREY, KESTREL),
            clear_win("place drill one race eight", OSPREY, KESTREL),
            clear_win("place drill one race nine", OSPREY, KESTREL),
        ],
    )
}

fn drill_two() -> JsonValue {
    series_json(
        "the constructed place drill two",
        vec![HOBBY, MERLIN],
        3,
        vec![
            clear_win("place drill two race one", HOBBY, MERLIN),
            clear_win("place drill two race two", HOBBY, MERLIN),
            clear_win("place drill two race three", MERLIN, HOBBY),
            clear_win("place drill two race four", MERLIN, HOBBY),
            clear_win("place drill two race five", MERLIN, HOBBY),
        ],
    )
}

#[test]
fn the_standing_championship_stands_never_behind_in_every_series() {
    // PL-1. The standing championship golden, no
    // section standing, closing "place, 2 series, 0
    // early, 0 late, 2 never behind, 0 no winner", the
    // pilot and the panel both standing never behind,
    // neither winner having stood behind at any
    // standing. Fails under V-5, the depth reading as
    // the winner's largest lead on both series, the
    // pilot's lead of two first standing at its second
    // decided race and the panel's lead of three at
    // its third, both series classing early at a place
    // standing equal to the lead. Passes under V-1 by
    // design, the never behind class standing unmoved
    // and no series of the two bearing a depth to
    // exchange. Passes under V-9 by design, no series
    // bearing a depth to gain a section. Passes under
    // V-2 and V-3, neither series attaining a depth
    // for a place to stand at. Passes under V-4, the
    // full recorded record standing at a depth of
    // nought on both series as the decided record
    // does. Passes under V-6, V-7, V-8, and V-10, no
    // figure standing in the report for any of them
    // to move, no series holding no winner, and no
    // section standing whose races could renumber.
    let championship = load_example("championship.json");
    let lines = place_report(&championship);
    assert_eq!(
        lines,
        vec![
            "place",
            "place, 2 series, 0 early, 0 late, 2 never behind, 0 no winner"
        ]
    );
    assert_eq!(lines.join("\n") + "\n", golden("championship-place.txt"));
}

#[test]
fn the_constructed_example_carries_the_first_series_alone() {
    // PL-2. The constructed place golden, the first
    // series standing in section form at a depth of
    // two and a place of four, its winner osprey-1
    // standing late, the second series standing early
    // at a place of two in a depth of two and taking
    // no section, closing "place, 2 series, 1 early, 1
    // late, 0 never behind, 0 no winner". Fails under
    // V-1, the two series exchanging classes and the
    // section moving to the second series. Fails under
    // V-2, the first series' place reading six, the
    // last of its two attainments. Fails under V-5,
    // the depth reading as the winner's largest lead
    // on both series, osprey's of one first standing
    // at its first decided race and merlin's of one
    // first standing at its fifth, the two series
    // exchanging classes and the section moving to
    // the second series. Fails under V-6, the first
    // series' place of four
    // standing at or below half its decided count of
    // nine, the series classing early. Fails under
    // V-7, the figure line printing the four in place
    // of the two and the two in place of the four.
    // Fails under V-9, the early second series gaining
    // a section. Passes under V-3, no race of either
    // series holding no winner, so the count in the
    // record and the count of decided races agree at
    // the attainment. Passes under V-4 by design,
    // neither series recording past its decision.
    // Passes under V-8, no series holding no winner,
    // and under V-10, the section's races standing
    // numbered as the record numbers them and the
    // decided order agreeing with it here.
    let championship = load_example("constructed-place.json");
    let lines = place_report(&championship);
    assert_eq!(
        lines,
        vec![
            "place",
            "series 1, the constructed place drill one",
            "osprey-1 against kestrel-1, first to 5 wins",
            "race 1, place drill one race one, osprey-1 wins",
            "race 2, place drill one race two, kestrel-1 wins",
            "race 3, place drill one race three, kestrel-1 wins",
            "race 4, place drill one race four, kestrel-1 wins",
            "race 5, place drill one race five, osprey-1 wins",
            "race 6, place drill one race six, kestrel-1 wins",
            "race 7, place drill one race seven, osprey-1 wins",
            "race 8, place drill one race eight, osprey-1 wins",
            "race 9, place drill one race nine, osprey-1 wins",
            "deepest deficit 2, place 4 over the decided record",
            "series winner, osprey-1, osprey-1 5 race wins, kestrel-1 4 race wins, 0 no results",
            "place, 2 series, 1 early, 1 late, 0 never behind, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-place-place.txt")
    );
}

#[test]
fn the_survey_championship_matches_its_place_golden() {
    // PL-3. The survey championship golden, sections
    // standing for apex and bolt at a place of three
    // in a depth of one and apex and drift at a place
    // of seven in a depth of one, its other four series
    // standing early or never behind, closing "place,
    // 6 series, 2 early, 2 late, 2 never behind, 0 no
    // winner". Fails under V-1, the early and late
    // series exchanging classes and the sections
    // moving to bolt and drift and crest and drift.
    // Fails under V-3, apex and bolt's place reading
    // as four, the count of its attainment race in
    // the record, a race holding no winner standing
    // before it inside the counted record. Fails
    // under V-4, apex and bolt reading a depth of
    // three at a place of eleven over its full
    // recorded record, and apex and crest gaining a
    // depth of one at a place of nine over its full
    // record and classing late with a section. Fails
    // under V-5, apex and bolt classing early at its
    // winner's lead of one standing at its first
    // decided race, and apex and drift's figure
    // reading a depth of two at a place of four.
    // Fails under V-7, the figure lines printing
    // their places in place of their depths. Fails
    // under V-9, the early series gaining sections.
    // Fails under V-10, apex and bolt's section
    // renumbering its decided races, a race holding
    // no winner standing inside its counted record.
    // Passes under V-2, each series of the six
    // attaining its depth at one standing alone, so
    // the first and last attainments agree. Passes
    // under V-6, the comparison against the decided
    // count classing every series of the six as the
    // comparison against the depth classes it.
    // Passes under V-8, no series of the six holding
    // no winner.
    let championship = load_example("survey-championship.json");
    let lines = place_report(&championship);
    assert_eq!(place_figures(&lines), vec![3, 7]);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("place, 6 series, 2 early, 2 late, 2 never behind, 0 no winner")
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("survey-championship-place.txt")
    );
}

#[test]
fn the_classification_reads_the_place_against_the_depth() {
    // PL-4. The classes at championship scale at hand
    // scale. A hand-built championship of three
    // series, the first on the constructed drill one's
    // shape standing late at a place of four in a
    // depth of two, the second on the drill two's
    // shape standing early at a place of two in a
    // depth of two, and the third standing at two wins
    // each under a target of five and holding no
    // winner, its whole report asserted with the
    // closing "place, 3 series, 1 early, 1 late, 0
    // never behind, 1 no winner" and the one section
    // on the first series, and the classification
    // entry asserted on all three. Fails under V-1,
    // V-2, V-5, V-6, and V-7 on the first two series
    // as PL-2 states them. Fails under V-8, the third
    // series gaining the never behind class at the
    // entry and the closing reading two never behind
    // and none holding no winner. Fails under V-9, the
    // second series gaining a section. Passes under
    // V-3 and V-4, no series recording a race holding
    // no winner and none recording past its decision.
    // Passes under V-10, the section's races standing
    // in decided order as the record numbers them.
    let dir = temp_dir("place-classification");
    let third = series_json(
        "the unreached saga",
        vec![ALPHA, BETA],
        5,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", BETA, ALPHA),
        ],
    );
    let championship = load(
        &dir,
        vec![
            ("one.json", drill_one()),
            ("two.json", drill_two()),
            ("three.json", third),
        ],
        vec!["one.json", "two.json", "three.json"],
    );
    assert_eq!(
        place_class(&championship.series[0]),
        Some((PlaceClass::Late, 4, 2))
    );
    assert_eq!(
        place_class(&championship.series[1]),
        Some((PlaceClass::Early, 2, 2))
    );
    assert_eq!(place_class(&championship.series[2]), None);
    let lines = place_report(&championship);
    assert_eq!(place_figures(&lines), vec![4]);
    assert!(lines.contains(&"series 1, the constructed place drill one".to_string()));
    assert!(!lines.iter().any(|line| line.starts_with("series 2,")));
    assert!(!lines.iter().any(|line| line.starts_with("series 3,")));
    assert_eq!(
        lines.last().map(String::as_str),
        Some("place, 3 series, 1 early, 1 late, 0 never behind, 1 no winner")
    );
}

#[test]
fn a_series_at_the_least_place_stands_early() {
    // PL-5. The boundary of the classes at the least
    // place. A hand-built series at a target of two
    // won by beta, alpha, alpha stands at a deficit of
    // one after its first decided race and never
    // behind again, asserted early at a place of one
    // in a depth of one through the classification
    // entry. Fails under V-1, the series classing
    // late. Fails under V-5, its place reading as the
    // first standing of its winner's largest lead of
    // one, at its third decided race, and the series
    // classing late. Passes under V-2, its depth
    // standing attained at one standing alone. Passes
    // under V-3, its attainment race standing first in
    // the record as in the decided count. Passes under
    // V-6, its place of one standing at or below half
    // its decided count of three. Passes under V-4,
    // V-7, V-8, V-9, and V-10 by design, the entry
    // standing unmoved by each.
    let dir = temp_dir("place-least-early");
    let series = series_json(
        "the least place saga",
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
        place_class(&championship.series[0]),
        Some((PlaceClass::Early, 1, 1))
    );
}

#[test]
fn a_series_whose_winner_won_first_stands_late_at_depth_one() {
    // PL-6. The least late shape. A hand-built series
    // at a target of three won by alpha, beta, beta,
    // alpha, alpha, its winner having won the first
    // decided race and then standing at a deficit of
    // one after its third, asserted late at a place of
    // three in a depth of one through the
    // classification entry. Fails under V-1, the
    // series classing early. Fails under V-5, its
    // winner's largest lead of one first standing at
    // its first decided race and the series classing
    // early at a place of one in a depth of one.
    // Passes under V-2, its depth standing attained at
    // one standing alone. Passes under V-3, its
    // attainment race standing third in the record as
    // in the decided count. Passes under V-6, its
    // place of three standing above half its decided
    // count of five. Passes under V-4, V-7, V-8, V-9,
    // and V-10 by design, the entry standing unmoved
    // by each.
    let dir = temp_dir("place-least-late");
    let series = series_json(
        "the least late saga",
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
        place_class(&championship.series[0]),
        Some((PlaceClass::Late, 3, 1))
    );
}

#[test]
fn two_series_at_one_depth_part_on_the_place() {
    // PL-7. The twin of the tie in the depth. Two
    // hand-built series sharing the depth of two, the
    // first won by beta, beta, alpha, alpha, alpha
    // under a target of three standing early at a
    // place of two, the second won by alpha, beta,
    // beta, beta, alpha, alpha, alpha under a target
    // of four standing late at a place of four, both
    // asserted through the classification entry.
    // Fails under V-1 on both. Fails under V-5 on
    // both, the first series' winner lead of one first
    // standing at its fifth decided race and classing
    // it late, the second series' winner lead of one
    // first standing at its first decided race and
    // classing it early. Passes under V-2, each depth
    // standing attained at one standing alone.
    // Passes under V-3, no race of either series
    // holding no winner. Passes under V-6, the first
    // series' place of two standing at or below half
    // its decided count of five and the second's
    // place of four standing above half its decided
    // count of seven. Passes under V-4, V-7, V-8, V-9,
    // and V-10 by design, the entry standing unmoved
    // by each.
    let dir = temp_dir("place-twin-depth");
    let first = series_json(
        "the early depth saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
        ],
    );
    let second = series_json(
        "the late depth saga",
        vec![ALPHA, BETA],
        4,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", BETA, ALPHA),
            clear_win("race five", ALPHA, BETA),
            clear_win("race six", ALPHA, BETA),
            clear_win("race seven", ALPHA, BETA),
        ],
    );
    let championship = load(
        &dir,
        vec![("one.json", first), ("two.json", second)],
        vec!["one.json", "two.json"],
    );
    assert_eq!(
        place_class(&championship.series[0]),
        Some((PlaceClass::Early, 2, 2))
    );
    assert_eq!(
        place_class(&championship.series[1]),
        Some((PlaceClass::Late, 4, 2))
    );
}

#[test]
fn the_place_names_the_first_attainment_not_the_last() {
    // PL-8. The first attainment against the last. A
    // hand-built series on the constructed drill one's
    // shape, its depth of two standing attained at its
    // fourth decided race and again at its sixth,
    // asserted late at a place of four through the
    // classification entry and through the figure line
    // of its whole report. Fails under V-1, the series
    // classing early and taking no section. Fails
    // under V-2, its place reading six, the last
    // attainment. Fails under V-5, its winner's largest
    // lead of one first standing at its first decided
    // race and the series classing early. Fails under
    // V-6, its place of four standing at or below half
    // its decided count of nine and the series
    // classing early. Fails under V-7, its figure line
    // printing the place of four in place of the depth
    // of two. Passes under V-3, no race holding no
    // winner, so the count in the record agrees with
    // the count of decided races at the attainment.
    // Passes under V-4 by design, the series recording
    // no race past its decision. Passes under V-8,
    // V-9, and V-10, the series already standing
    // sectioned and its races standing numbered as
    // the record numbers them.
    let dir = temp_dir("place-first-attainment");
    let championship = load(&dir, vec![("one.json", drill_one())], vec!["one.json"]);
    assert_eq!(
        place_class(&championship.series[0]),
        Some((PlaceClass::Late, 4, 2))
    );
    let lines = place_report(&championship);
    assert!(lines.contains(&"deepest deficit 2, place 4 over the decided record".to_string()));
}

#[test]
fn a_series_whose_place_parts_from_its_margin_stands_late() {
    // PL-9. The place parted from the margin at hand
    // scale. A hand-built series on the constructed
    // drill one's shape, asserted late at a place of
    // four in a depth of two through the classification
    // entry, with its DH-48 class asserted deep beside
    // it through the depths closing over it alone, its
    // depth of two standing above its margin of one,
    // the two readings parting on the comparison each
    // reads its figure against. Fails under V-1, V-2,
    // V-5, and V-6 on the place half as PL-8 states
    // them. Passes under V-3, V-4, V-7, V-8, V-9, and
    // V-10 on the place half, the entry standing
    // unmoved by each. The depths half stands under
    // every place mutation by design.
    let dir = temp_dir("place-parted-margin");
    let championship = load(&dir, vec![("one.json", drill_one())], vec!["one.json"]);
    assert_eq!(
        place_class(&championship.series[0]),
        Some((PlaceClass::Late, 4, 2))
    );
    let depth_lines = depths_report(&championship);
    assert_eq!(
        depth_lines.last().map(String::as_str),
        Some("depths, 1 series, 1 deep, 0 matched, 0 shallow, 0 no winner")
    );
}

#[test]
fn the_duration_readings_part_on_one_series() {
    // PL-10. The non-implication witness at hand
    // scale. A hand-built series on the constructed
    // drill one's shape, asserted late at a place of
    // four in a depth of two here, unbroken under
    // DH-50 beside it at a spell of five in a behind
    // count of five, its behind standings standing in
    // one run, and broken under DH-52 at a reign of
    // one in a front count of two, its ahead standings
    // standing in two runs of one, the three readings
    // parting on one series. Fails under V-1, V-2,
    // V-5, and V-6 on the place half as PL-8 states
    // them. Passes under V-3, V-4, V-7, V-8, V-9, and
    // V-10 on the place half, the entry standing
    // unmoved by each. The spells and reigns halves
    // stand under every place mutation by design.
    let dir = temp_dir("place-duration-witness");
    let championship = load(&dir, vec![("one.json", drill_one())], vec!["one.json"]);
    assert_eq!(
        place_class(&championship.series[0]),
        Some((PlaceClass::Late, 4, 2))
    );
    assert_eq!(
        spell_class(&championship.series[0]),
        Some((SpellClass::Unbroken, 5, 5))
    );
    assert_eq!(
        reign_class(&championship.series[0]),
        Some((ReignClass::Broken, 1, 2))
    );
}

#[test]
fn an_adjudicated_race_counts_in_the_place() {
    // PL-11. The adjudication held entry at hand
    // scale. A hand-built series at a target of three
    // whose third race's attempts stand both red and
    // named by an adjudication entry for beta, the
    // race decided to the named attempt under DH-5,
    // its winner's race sequence standing won, lost,
    // adjudicated lost, won, won, its depth of one
    // standing attained at the adjudicated race,
    // asserted late at a place of three in a depth of
    // one with its whole report asserted in the pin,
    // the adjudicated race numbered three among its
    // decided races. Fails under V-1, the series
    // classing early and taking no section. Fails
    // under V-5, its winner's largest lead of one
    // first standing at its first decided race and the
    // series classing early. Fails under V-7, its
    // figure line printing the place of three in
    // place of the depth of one. Passes under V-2, its
    // depth standing attained at one standing alone.
    // Passes under V-3, its attainment race standing
    // third in the record as in the decided count.
    // Passes under V-6, its place of three standing
    // above half its decided count of five. Passes
    // under V-4 and V-8 by design, and under V-9 and
    // V-10, its one decided series already standing
    // sectioned and its races standing numbered as
    // the record numbers them.
    let dir = temp_dir("place-adjudicated");
    let series = series_json(
        "the adjudicated place saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            adjudicated_win("race three", ALPHA, BETA, BETA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
        ],
    );
    let lines = place_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "place",
            "series 1, the adjudicated place saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, beta-1 wins",
            "race 4, race four, alpha-1 wins",
            "race 5, race five, alpha-1 wins",
            "deepest deficit 1, place 3 over the decided record",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 2 race wins, 0 no results",
            "place, 1 series, 0 early, 1 late, 0 never behind, 0 no winner",
        ]
    );
}

#[test]
fn the_races_after_the_decision_count_in_neither_figure() {
    // PL-12. The tailed fixture. A hand-built series
    // on the constructed drill one's shape recorded
    // with four further races after its decision, all
    // lost by its winner, asserted late at a place of
    // four in a depth of two over the decided record
    // the walk leaves, the four recorded races after
    // the decision counting in neither figure. Fails
    // under V-1, the series classing early. Fails
    // under V-2, its place reading six over the
    // decided record, the last of its two attainments
    // there. Fails under V-4, the full recorded record
    // reading a depth of three at a place of thirteen
    // over its thirteen recorded races where the
    // decided record reads two at four. Fails under
    // V-5, its winner's largest lead of one first
    // standing at its first decided race over the
    // decided record and the series classing early.
    // Fails under V-6, its place of four standing at
    // or below half its decided count of nine.
    // Passes under V-3, its attainment race standing
    // fourth in the record as in the decided count.
    // Passes under V-7, V-8, V-9, and V-10 by design,
    // the entry standing unmoved by each.
    let dir = temp_dir("place-tailed");
    let tailed = series_json(
        "the constructed place drill one",
        vec![OSPREY, KESTREL],
        5,
        vec![
            clear_win("place drill one race one", OSPREY, KESTREL),
            clear_win("place drill one race two", KESTREL, OSPREY),
            clear_win("place drill one race three", KESTREL, OSPREY),
            clear_win("place drill one race four", KESTREL, OSPREY),
            clear_win("place drill one race five", OSPREY, KESTREL),
            clear_win("place drill one race six", KESTREL, OSPREY),
            clear_win("place drill one race seven", OSPREY, KESTREL),
            clear_win("place drill one race eight", OSPREY, KESTREL),
            clear_win("place drill one race nine", OSPREY, KESTREL),
            clear_win("place drill one race ten", KESTREL, OSPREY),
            clear_win("place drill one race eleven", KESTREL, OSPREY),
            clear_win("place drill one race twelve", KESTREL, OSPREY),
            clear_win("place drill one race thirteen", KESTREL, OSPREY),
        ],
    );
    let championship = load(&dir, vec![("one.json", tailed)], vec!["one.json"]);
    assert_eq!(
        place_class(&championship.series[0]),
        Some((PlaceClass::Late, 4, 2))
    );
}

#[test]
fn the_record_count_stands_skipped_over_a_no_result_race() {
    // PL-13. The record count at hand scale, the pin
    // DH-55's stated relations name for this file. A
    // hand-built series at a target of four whose
    // first recorded race holds no winner, its winner
    // alpha-1 standing at a depth of two first
    // attained at its fourth decided race, stands
    // late at a place of four in a depth of two, and
    // its whole report stands asserted, its section
    // printing its decided races at their counts in
    // the record, race 2 through race 8, the count of
    // the race holding no winner standing skipped,
    // while its place stands at four, a count of
    // decided races and not of the record, as the
    // stated relations of DH-55 state the two counts.
    // Fails under V-1, the series classing early and
    // taking no section. Fails under V-3, its place
    // reading five, the count of its attainment race
    // in the record. Fails under V-5, its winner's
    // largest lead of one first standing at its first
    // decided race and the series classing early.
    // Fails under V-7, its figure line printing the
    // place of four in place of the depth of two.
    // Fails under V-10, its decided races standing
    // renumbered as race 1 through race 7. Passes
    // under V-2, its depth standing attained at one
    // decided standing alone. Passes under V-6, its
    // place of four standing above half its decided
    // count of seven. Passes under V-4 and V-8 by
    // design, and under V-9, its one decided series
    // already standing sectioned.
    let dir = temp_dir("place-record-count");
    let series = series_json(
        "the record count drill",
        vec![ALPHA, BETA],
        4,
        vec![
            no_result("record count drill race one", ALPHA, BETA),
            clear_win("record count drill race two", ALPHA, BETA),
            clear_win("record count drill race three", BETA, ALPHA),
            clear_win("record count drill race four", BETA, ALPHA),
            clear_win("record count drill race five", BETA, ALPHA),
            clear_win("record count drill race six", ALPHA, BETA),
            clear_win("record count drill race seven", ALPHA, BETA),
            clear_win("record count drill race eight", ALPHA, BETA),
        ],
    );
    let lines = place_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "place",
            "series 1, the record count drill",
            "alpha-1 against beta-1, first to 4 wins",
            "race 2, record count drill race two, alpha-1 wins",
            "race 3, record count drill race three, beta-1 wins",
            "race 4, record count drill race four, beta-1 wins",
            "race 5, record count drill race five, beta-1 wins",
            "race 6, record count drill race six, alpha-1 wins",
            "race 7, record count drill race seven, alpha-1 wins",
            "race 8, record count drill race eight, alpha-1 wins",
            "deepest deficit 2, place 4 over the decided record",
            "series winner, alpha-1, alpha-1 4 race wins, beta-1 3 race wins, 1 no results",
            "place, 1 series, 0 early, 1 late, 0 never behind, 0 no winner"
        ]
    );
}

#[test]
fn a_series_of_target_one_stands_never_behind() {
    // PL-14. The target one sentence of DH-54 at hand
    // scale. A hand-built series decided at its first
    // race under a wins target of one stands never
    // behind, its only decided race being its deciding
    // race and its winner having won it, its whole
    // report asserted in the pin, closing "place, 1
    // series, 0 early, 0 late, 1 never behind, 0 no
    // winner". Fails under V-5, its depth reading as
    // its winner's largest lead of one at a place of
    // one and the series classing early. Passes under
    // V-1, the never behind class standing unmoved by
    // the exchange of the other two. Passes under
    // V-2, V-3, and V-4, no depth standing for a place
    // to read over any of the three records. Passes
    // under V-6, V-7, V-8, V-9, and V-10 by design, no
    // section standing and no figure standing in the
    // report for any of them to move.
    let dir = temp_dir("place-target-one");
    let series = series_json(
        "the single race saga",
        vec![ALPHA, BETA],
        1,
        vec![clear_win("race one", ALPHA, BETA)],
    );
    let lines = place_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "place",
            "place, 1 series, 0 early, 0 late, 1 never behind, 0 no winner"
        ]
    );
}

#[test]
fn the_league_championship_matches_its_place_golden() {
    // PL-15. The league championship golden, a section
    // standing for apex and crest alone at a place of
    // three in a depth of one, its other nine series
    // standing early or never behind, closing "place,
    // 10 series, 4 early, 1 late, 5 never behind, 0 no
    // winner". Fails under V-1, the early series
    // classing late and gaining sections as apex and
    // crest loses its own. Fails under V-3, apex and
    // bolt's place reading as two, the count of its
    // attainment race in the record, and the series
    // classing late with a section. Fails under V-4,
    // bolt and crest, bolt and drift, and crest and
    // drift each gaining a depth over their full
    // recorded records and classing late with
    // sections. Fails under V-5, apex and crest's
    // figure reading a depth of two at a place of six
    // and apex and bolt classing late at its winner's
    // lead. Fails under V-6, apex and crest's place of
    // three standing at or below half its decided
    // count of six and the series classing early.
    // Fails under V-7, the figure line printing the
    // place in place of the depth. Fails under V-9,
    // the four early series gaining sections. Passes
    // under V-2, every series of the ten attaining its
    // depth at one standing alone. Passes under V-8,
    // no series of the ten holding no winner. Passes
    // under V-10, apex and crest's section standing
    // with no race holding no winner inside its
    // counted record, its races numbering as the
    // record numbers them and the decided order
    // agreeing with it there.
    let championship = load_example("league-championship.json");
    let lines = place_report(&championship);
    assert_eq!(place_figures(&lines), vec![3]);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("place, 10 series, 4 early, 1 late, 5 never behind, 0 no winner")
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("league-championship-place.txt")
    );
}

#[test]
fn the_bolt_and_ember_series_stands_early_at_two_in_two() {
    // PL-16. The matched witness on the shipped
    // record. The league series bolt and ember loaded
    // from the examples, asserted early at a place of
    // two in a depth of two through the classification
    // entry, with its DH-48 class asserted matched
    // beside it through the depths closing over it
    // alone, its depth of two standing equal to its
    // margin of two, a series whose deepest deficit
    // was dug before its winner won a race and whose
    // depth the margin exactly answers. Fails under
    // V-1, the series classing late. Fails under V-5,
    // its winner's largest lead of two first standing
    // at its eighth decided race and the series
    // classing late at a place of eight. Passes under
    // V-2, its depth standing attained at one standing
    // alone, and under V-3, its attainment race
    // standing second in the record as in the decided
    // count, and under V-4, its full recorded record
    // reading the same depth at the same place.
    // Passes under V-6, its place of two standing at
    // or below half its decided count of eight.
    // Passes under V-7, V-8, V-9, and V-10 by design,
    // the entry standing unmoved by each. The depths
    // half stands under every place mutation by
    // design.
    let championship = load_example("league-championship.json");
    let series = championship.series[6].clone();
    assert_eq!(place_class(&series), Some((PlaceClass::Early, 2, 2)));
    let alone = Championship {
        series: vec![series],
    };
    let depth_lines = depths_report(&alone);
    assert_eq!(
        depth_lines.last().map(String::as_str),
        Some("depths, 1 series, 0 deep, 1 matched, 0 shallow, 0 no winner")
    );
}

#[test]
fn early_and_late_stand_across_the_opening_classes() {
    // PL-17. The opening relations of DH-54 at hand
    // scale. Four hand-built series, each loaded
    // alone, the first on the constructed drill two's
    // shape standing early with its winner standing
    // from behind under DH-38, the second won by beta,
    // alpha, beta, beta, alpha, alpha, alpha under a
    // target of four standing late with its winner
    // also standing from behind, its loser's opening
    // run of one standing below its depth of two where
    // the first series' loser's opening run of two
    // stands at it, the third on the constructed drill
    // one's shape standing late with its winner
    // standing alone under DH-38 at an opening run of
    // one, and the fourth won by alpha, alpha, beta,
    // beta, beta, alpha, alpha, alpha under a target
    // of five standing late with its winner standing
    // in a run at an opening run of two, the place
    // entry and the opening closing asserted on each,
    // witnessing that a series standing early stands
    // from behind and that a series standing late may
    // stand in any of the three opening classes.
    // Fails under V-1 on all four, the early and late
    // classes exchanging. Fails under V-5 on all four,
    // each series' figures reading from its winner's
    // seat. Fails under V-2 on the third series alone,
    // its depth standing attained at two standings
    // and its place reading six for four. Fails under
    // V-6 on the third series alone, its place of four
    // standing at or below half its decided count of
    // nine. Passes under V-3 and V-4, no series
    // recording a race holding no winner inside its
    // counted record before its attainment and none
    // recording past its decision. Passes under V-7,
    // V-8, V-9, and V-10 by design, the entries
    // standing unmoved by each. The opening halves
    // stand under every place mutation by design.
    let dir = temp_dir("place-opening-relations");
    let second = series_json(
        "the short opening saga",
        vec![ALPHA, BETA],
        4,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", BETA, ALPHA),
            clear_win("race five", ALPHA, BETA),
            clear_win("race six", ALPHA, BETA),
            clear_win("race seven", ALPHA, BETA),
        ],
    );
    let fourth = series_json(
        "the running opening saga",
        vec![ALPHA, BETA],
        5,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", BETA, ALPHA),
            clear_win("race five", BETA, ALPHA),
            clear_win("race six", ALPHA, BETA),
            clear_win("race seven", ALPHA, BETA),
            clear_win("race eight", ALPHA, BETA),
        ],
    );
    let first_ch = load(&dir, vec![("one.json", drill_two())], vec!["one.json"]);
    assert_eq!(
        place_class(&first_ch.series[0]),
        Some((PlaceClass::Early, 2, 2))
    );
    assert_eq!(
        opening_report(&first_ch).last().map(String::as_str),
        Some("opening, 1 series, 0 in a run, 0 alone, 1 from behind, 0 no winner")
    );
    let second_ch = load(&dir, vec![("two.json", second)], vec!["two.json"]);
    assert_eq!(
        place_class(&second_ch.series[0]),
        Some((PlaceClass::Late, 4, 2))
    );
    assert_eq!(
        opening_report(&second_ch).last().map(String::as_str),
        Some("opening, 1 series, 0 in a run, 0 alone, 1 from behind, 0 no winner")
    );
    let third_ch = load(&dir, vec![("three.json", drill_one())], vec!["three.json"]);
    assert_eq!(
        place_class(&third_ch.series[0]),
        Some((PlaceClass::Late, 4, 2))
    );
    assert_eq!(
        opening_report(&third_ch).last().map(String::as_str),
        Some("opening, 1 series, 0 in a run, 1 alone, 0 from behind, 0 no winner")
    );
    let fourth_ch = load(&dir, vec![("four.json", fourth)], vec!["four.json"]);
    assert_eq!(
        place_class(&fourth_ch.series[0]),
        Some((PlaceClass::Late, 5, 1))
    );
    assert_eq!(
        opening_report(&fourth_ch).last().map(String::as_str),
        Some("opening, 1 series, 1 in a run, 0 alone, 0 from behind, 0 no winner")
    );
}

#[test]
fn a_collection_holding_no_series_renders_the_empty_form_at_the_library_surface() {
    // PL-18. The empty form, pinned from birth. No
    // championship file carries an empty collection,
    // because the loader of DH-13 refuses a series
    // list holding no names, so this form stands at
    // the library surface alone, over a collection
    // built by hand. The pin fails under no mutation
    // of the ten, the empty rendering being the two
    // lines DH-55 states and the mutation set carrying
    // no variant under which it changes.
    let championship = Championship { series: vec![] };
    let lines = place_report(&championship);
    assert_eq!(
        lines,
        vec![
            "place",
            "place, 0 series, 0 early, 0 late, 0 never behind, 0 no winner"
        ]
    );
}

#[test]
fn the_depth_attained_at_several_standings_classes_by_the_first() {
    // PL-19. The several standings case of DH-54 at
    // the class boundary. A hand-built series at a
    // target of four won by beta, beta, alpha, beta,
    // alpha, alpha, alpha, its depth of two standing
    // attained at its second decided race and again
    // at its fourth, asserted early at a place of two
    // through the classification entry, the class
    // standing on the first attainment alone as DH-54
    // states. Fails under V-1, the series classing
    // late. Fails under V-2, its place reading four,
    // the last attainment, and the series classing
    // late. Fails under V-5, its winner's largest lead
    // of one first standing at its seventh decided
    // race and the series classing late at a place of
    // seven. Passes under V-3, its first attainment
    // race standing second in the record as in the
    // decided count. Passes under V-6, its place of
    // two standing at or below half its decided count
    // of seven. Passes under V-4, V-7, V-8, V-9, and
    // V-10 by design, the entry standing unmoved by
    // each.
    let dir = temp_dir("place-several-standings");
    let series = series_json(
        "the several standings saga",
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
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        place_class(&championship.series[0]),
        Some((PlaceClass::Early, 2, 2))
    );
}

#[test]
fn two_series_part_at_the_least_depth() {
    // PL-20. The smallest parting of the classes. Two
    // hand-built series sharing the depth of one, the
    // first won by beta, alpha, alpha under a target
    // of two standing early at a place of one, the
    // second won by alpha, beta, beta, alpha, alpha
    // under a target of three standing late at a place
    // of three, the least depth at which the classes
    // part, both asserted through the classification
    // entry. Fails under V-1 on both. Fails under V-5
    // on both, the first series' winner lead of one
    // first standing at its third decided race and
    // classing it late, the second series' winner
    // lead of one first standing at its first decided
    // race and classing it early. Passes under V-2,
    // each depth standing attained at one standing
    // alone. Passes under V-3, no race of either
    // series holding no winner. Passes under V-6, the
    // first series' place of one standing at or below
    // half its decided count of three and the second's
    // place of three standing above half its decided
    // count of five. Passes under V-4, V-7, V-8, V-9,
    // and V-10 by design, the entry standing unmoved
    // by each.
    let dir = temp_dir("place-least-parting");
    let first = series_json(
        "the whole depth saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", ALPHA, BETA),
        ],
    );
    let second = series_json(
        "the parted depth saga",
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
    let championship = load(
        &dir,
        vec![("one.json", first), ("two.json", second)],
        vec!["one.json", "two.json"],
    );
    assert_eq!(
        place_class(&championship.series[0]),
        Some((PlaceClass::Early, 1, 1))
    );
    assert_eq!(
        place_class(&championship.series[1]),
        Some((PlaceClass::Late, 3, 1))
    );
}
