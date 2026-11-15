//! Pins for DH-52 and DH-53 in the Round 26 section of
//! CONTRACT.md.
//!
//! The discrimination record of this file stands pin by
//! pin, and no blanket claim stands over it. The reigns
//! reading, its library function, and its command do
//! not stand on the pre round tree, so every pin in
//! this file that names the reigns reading is absent
//! there at compile time. That absence is recorded
//! once here and is not a discrimination claim. The
//! discrimination each pin carries is against the
//! stated mutation of DH-52 beside it, and the pin of
//! the record count carries its discrimination against
//! the mutation of DH-53 its comment names beside the
//! nine. Ten mutations are named for the whole file. V-1, the class cuts
//! exchanged, a series classing broken where the
//! shipped reading classes it unbroken and unbroken
//! where it classes it broken. V-2, the longest reign
//! read as the front count in place of the longest
//! run. V-3, the longest reign read as the winner's
//! largest lead of DH-42 in place of the longest run.
//! V-4, the standings read over every decided race of
//! the full recorded record in place of the decided
//! record the walk leaves, so races recorded after
//! the decision count in the reign and the count. V-5,
//! the longest reign read as the winner's longest run
//! of wins, the streak figure of DH-27, in place of
//! the longest run of ahead standings. V-6, the front
//! count read as the winner's behind count of DH-50
//! in place of the count of ahead standings. V-7, the
//! figure line printing the front count in place of
//! the longest reign, the classes standing true. V-8,
//! a series holding no winner placed in the unbroken
//! class in place of standing in no class. V-9, the
//! section standing for every decided series in place
//! of the broken series alone. V-10, the decided races
//! of a section renumbered as their order among the
//! decided races, the first decided race standing as
//! race 1 and each later decided race standing one
//! above the last, in place of the count of the race
//! in the record the walk leaves.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{load_championship_file, Championship};
use deadheat::deadlocks::deadlock_report;
use deadheat::depths::depths_report;
use deadheat::json::JsonValue;
use deadheat::reigns::{reign_class, reigns_report, ReignClass};
use deadheat::spells::{spell_class, SpellClass};
use deadheat::tenure::tenure_report;
use shared::*;

const ALPHA: &str = "alpha-1";
const BETA: &str = "beta-1";
const PLOVER: &str = "plover-1";
const DUNLIN: &str = "dunlin-1";
const GODWIT: &str = "godwit-1";
const STILT: &str = "stilt-1";

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

fn reigns_for(dir: &Path, files: Vec<(&str, JsonValue)>, names: Vec<&str>) -> Vec<String> {
    reigns_report(&load(dir, files, names))
}

fn load_example(name: &str) -> Championship {
    let (championship, defects) = load_championship_file(&examples().join(name));
    assert_eq!(defects, Vec::<String>::new());
    championship.expect("a championship")
}

fn golden(name: &str) -> String {
    std::fs::read_to_string(examples().join(name)).expect("a golden")
}

fn reign_figures(lines: &[String]) -> Vec<u64> {
    lines
        .iter()
        .filter_map(|line| {
            line.strip_prefix("longest reign ")
                .and_then(|rest| rest.split(", front count ").next())
                .and_then(|figure| figure.parse::<u64>().ok())
        })
        .collect()
}

fn drill_one() -> JsonValue {
    series_json(
        "the constructed reigns drill one",
        vec![PLOVER, DUNLIN],
        5,
        vec![
            clear_win("reigns drill one race one", PLOVER, DUNLIN),
            clear_win("reigns drill one race two", DUNLIN, PLOVER),
            clear_win("reigns drill one race three", PLOVER, DUNLIN),
            clear_win("reigns drill one race four", DUNLIN, PLOVER),
            clear_win("reigns drill one race five", PLOVER, DUNLIN),
            clear_win("reigns drill one race six", PLOVER, DUNLIN),
            clear_win("reigns drill one race seven", PLOVER, DUNLIN),
        ],
    )
}

fn drill_two() -> JsonValue {
    series_json(
        "the constructed reigns drill two",
        vec![GODWIT, STILT],
        5,
        vec![
            clear_win("reigns drill two race one", STILT, GODWIT),
            clear_win("reigns drill two race two", GODWIT, STILT),
            clear_win("reigns drill two race three", GODWIT, STILT),
            clear_win("reigns drill two race four", GODWIT, STILT),
            clear_win("reigns drill two race five", STILT, GODWIT),
            clear_win("reigns drill two race six", GODWIT, STILT),
            clear_win("reigns drill two race seven", GODWIT, STILT),
        ],
    )
}

#[test]
fn the_standing_championship_stands_unbroken_in_every_series() {
    // R-1. The standing championship golden, no section
    // standing, closing "reigns, 2 series, 2 unbroken, 0
    // broken, 0 no winner", the pilot and the panel both
    // standing unbroken, the pilot at a reign of four in
    // a front count of four and the panel at three in
    // three. Fails under V-1, the closing counts
    // exchanging. Fails under V-3, the pilot's reign
    // reading as its largest lead of two in a front
    // count of four and the series classing broken with
    // a section. Fails under V-5, the pilot's reign
    // reading as its longest win run of two in a front
    // count of four, the same parting. Fails under
    // V-6, both front counts reading as behind counts
    // of nought and both series classing broken. Fails
    // under V-9, both unbroken series gaining sections.
    // Passes under V-2, both series standing unbroken
    // at a reign read as the front count. Passes under
    // V-4, the full recorded record classing both
    // series as the decided record does and no section
    // standing to carry a figure. Passes under V-7, no
    // section standing. Passes under V-8, no series
    // holding no winner.
    let championship = load_example("championship.json");
    let lines = reigns_report(&championship);
    assert_eq!(
        lines,
        vec![
            "reigns",
            "reigns, 2 series, 2 unbroken, 0 broken, 0 no winner"
        ]
    );
    assert_eq!(lines.join("\n") + "\n", golden("championship-reigns.txt"));
}

#[test]
fn the_constructed_example_carries_the_first_series_alone() {
    // R-2. The constructed reigns golden, the first
    // series standing in section form at the figure
    // three in a front count of five, its winner
    // plover-1 standing broken, the second series
    // standing unbroken at a reign of five in a front
    // count of five and taking no section, closing
    // "reigns, 2 series, 1 unbroken, 1 broken, 0 no
    // winner". Fails under V-1, the two series
    // exchanging classes and the section moving to the
    // second series. Fails under V-2, the first series'
    // reign reading as its front count of five and
    // classing unbroken. Fails under V-3 and V-5, the
    // second series' reign reading as its largest lead
    // of three and as its longest win run of three
    // against its front count of five and classing
    // broken. Fails under V-6, the front counts reading
    // as behind counts of nought and one and both
    // series classing broken. Fails under V-7, the
    // figure line printing the five in place of the
    // three. Fails under V-9, the unbroken second
    // series gaining a section. Passes under V-4 by
    // design, neither series of the pair carrying a
    // race past its decision. Passes under V-8 by
    // design, no series holding no winner.
    let championship = load_example("constructed-reigns.json");
    let lines = reigns_report(&championship);
    assert_eq!(
        lines,
        vec![
            "reigns",
            "series 1, the constructed reigns drill one",
            "plover-1 against dunlin-1, first to 5 wins",
            "race 1, reigns drill one race one, plover-1 wins",
            "race 2, reigns drill one race two, dunlin-1 wins",
            "race 3, reigns drill one race three, plover-1 wins",
            "race 4, reigns drill one race four, dunlin-1 wins",
            "race 5, reigns drill one race five, plover-1 wins",
            "race 6, reigns drill one race six, plover-1 wins",
            "race 7, reigns drill one race seven, plover-1 wins",
            "longest reign 3, front count 5 over the decided record",
            "series winner, plover-1, plover-1 5 race wins, dunlin-1 2 race wins, 0 no results",
            "reigns, 2 series, 1 unbroken, 1 broken, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-reigns-reigns.txt")
    );
}

#[test]
fn the_survey_championship_matches_its_reigns_golden() {
    // R-3. The survey championship golden, sections
    // standing for apex and bolt at a reign of one in a
    // front count of two, apex and drift at three in
    // five, and bolt and crest at three in four, its
    // other three series standing unbroken, closing
    // "reigns, 6 series, 3 unbroken, 3 broken, 0 no
    // winner". Fails under V-1, V-2, V-3, V-5, V-6, and
    // V-9 on the classes and the sections. Fails under
    // V-4, the full recorded record classing apex and
    // crest broken at a reign of seven in a front count
    // of nine where the decided record classes it
    // unbroken at six in six, and bolt and crest's
    // reign reading nine in ten over its full record.
    // Fails under V-7, the figure lines of apex and
    // bolt, apex and drift, and bolt and crest printing
    // their front counts in place of their reigns.
    // Passes under V-8 alone, no series of the six
    // holding no winner.
    let championship = load_example("survey-championship.json");
    let lines = reigns_report(&championship);
    assert_eq!(reign_figures(&lines), vec![1, 3, 3]);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("reigns, 6 series, 3 unbroken, 3 broken, 0 no winner")
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("survey-championship-reigns.txt")
    );
}

#[test]
fn the_classification_reads_the_reign_against_the_front_count() {
    // R-4. The classes at championship scale at hand
    // scale. A hand-built championship of three series,
    // the first on the constructed drill one's shape
    // standing broken at a reign of three in a front
    // count of five, the second on the drill two's
    // shape standing unbroken at five in five, and the
    // third standing at two wins each under a target of
    // five and holding no winner, its whole report
    // asserted with the closing "reigns, 3 series, 1
    // unbroken, 1 broken, 1 no winner" and the one
    // section on the first series, and the
    // classification entry asserted on all three.
    // Fails under V-1, V-2, V-3, V-5, V-6, and V-7 on
    // the first two series as R-7 states them. Fails
    // under V-8, the third series gaining the unbroken
    // class at the entry and the closing reading two
    // unbroken and none holding no winner. Fails under
    // V-9, the second series gaining a section. Passes
    // under V-4, no series recording past its decision.
    let dir = temp_dir("reigns-classification");
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
        reign_class(&championship.series[0]),
        Some((ReignClass::Broken, 3, 5))
    );
    assert_eq!(
        reign_class(&championship.series[1]),
        Some((ReignClass::Unbroken, 5, 5))
    );
    assert_eq!(reign_class(&championship.series[2]), None);
    let lines = reigns_report(&championship);
    assert_eq!(reign_figures(&lines), vec![3]);
    assert!(lines.contains(&"series 1, the constructed reigns drill one".to_string()));
    assert!(!lines.iter().any(|line| line.starts_with("series 2,")));
    assert!(!lines.iter().any(|line| line.starts_with("series 3,")));
    assert_eq!(
        lines.last().map(String::as_str),
        Some("reigns, 3 series, 1 unbroken, 1 broken, 1 no winner")
    );
}

#[test]
fn a_series_at_the_smallest_front_count_stands_unbroken() {
    // R-5. The boundary of the classes at the smallest
    // front count. A hand-built series at a target of
    // two won by beta, alpha, alpha stands ahead at one
    // standing only, the deciding standing, asserted
    // unbroken at a reign of one in a front count of
    // one through the classification entry. Fails under
    // V-1, the series classing broken at the entry.
    // Fails under V-5, its stated reign reading as its
    // winner's longest run of two wins and classing
    // broken. Passes under V-2 and V-3, reign, front
    // count, and largest lead agreeing at one. Passes
    // under V-6, its behind count of one agreeing with
    // its front count at the boundary. Passes under
    // V-4, V-7, V-8, and V-9 by design, the entry
    // standing unmoved by each.
    let dir = temp_dir("reigns-smallest-count");
    let series = series_json(
        "the smallest count saga",
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
        reign_class(&championship.series[0]),
        Some((ReignClass::Unbroken, 1, 1))
    );
}

#[test]
fn a_winner_first_ahead_at_the_decision_stands_unbroken() {
    // R-6. The first lead at the deciding race. A
    // hand-built series at a target of three won by
    // beta, beta, alpha, alpha, alpha, its winner
    // standing behind through three standings and level
    // at the fourth and ahead only at the deciding
    // standing, asserted unbroken at a reign of one in
    // a front count of one, witnessing that the reading
    // carries no never ahead class beside its two, a
    // series whose winner first stands ahead at the
    // decision standing in the unbroken class and not
    // in a class of its own. Fails under V-1, the
    // series classing broken. Fails under V-5, its
    // stated reign reading as its winner's closing run
    // of three wins. Fails under V-6, its front count
    // reading as its behind count of three. Passes
    // under V-2 and V-3, the figures agreeing at one.
    // Passes under V-4 by design, and under V-7, V-8,
    // and V-9 by design, the entry standing unmoved.
    let dir = temp_dir("reigns-first-lead");
    let series = series_json(
        "the first lead saga",
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
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        reign_class(&championship.series[0]),
        Some((ReignClass::Unbroken, 1, 1))
    );
}

#[test]
fn two_series_at_one_front_count_part_on_the_reign() {
    // R-7. The twin of the tie in the count. Two
    // hand-built series sharing the front count of
    // five, the first on the constructed drill one's
    // shape standing broken at a reign of three, the
    // second on the drill two's shape standing unbroken
    // at a reign of five, both asserted through the
    // classification entry. Fails under V-1 on both.
    // Fails under V-2 on the first, its reign reading
    // as its front count and the series classing
    // unbroken. Fails under V-3 and V-5 on the second,
    // its reign reading as its largest lead of three
    // and as its longest win run of three, the series
    // classing broken both ways. Fails under V-6 on
    // both, the front counts reading as behind counts
    // of nought and one. Passes under V-4, neither
    // series recording past its decision. Passes under
    // V-7, V-8, and V-9 by design, the entry standing
    // unmoved by each.
    let dir = temp_dir("reigns-twin-count");
    let championship = load(
        &dir,
        vec![("one.json", drill_one()), ("two.json", drill_two())],
        vec!["one.json", "two.json"],
    );
    assert_eq!(
        reign_class(&championship.series[0]),
        Some((ReignClass::Broken, 3, 5))
    );
    assert_eq!(
        reign_class(&championship.series[1]),
        Some((ReignClass::Unbroken, 5, 5))
    );
}

#[test]
fn the_longest_reign_stands_at_its_longest_not_its_last() {
    // R-8. The longest reign standing at its longest,
    // not its last. A hand-built series at a target of
    // three won by alpha, alpha, beta, beta, alpha, its
    // reign runs standing at three and one, asserted
    // broken at a reign of three in a front count of
    // four through the classification entry. Fails
    // under V-1, the series classing unbroken. Fails
    // under V-2, its reign reading as its front count
    // of four and classing unbroken. Fails under V-3,
    // its reign reading as its largest lead of two.
    // Fails under V-5, its reign reading as its longest
    // win run of two. Fails under V-6, its front count
    // reading as a behind count of nought. Passes under
    // V-4 by design, and under V-7, V-8, and V-9 by
    // design, the entry standing unmoved by each.
    let dir = temp_dir("reigns-longest-not-last");
    let series = series_json(
        "the longest reign saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", BETA, ALPHA),
            clear_win("race five", ALPHA, BETA),
        ],
    );
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        reign_class(&championship.series[0]),
        Some((ReignClass::Broken, 3, 4))
    );
}

#[test]
fn the_reign_reached_early_stands_against_the_later_races() {
    // R-9. The reign reached early standing against
    // the later races. A hand-built series at a target
    // of five won by alpha, alpha, beta, beta, alpha,
    // beta, alpha, alpha, its reign runs standing at
    // three, one, and two over eight decided races, the
    // longest reign reached at its third decided race
    // and standing against the five decided races after
    // it, asserted broken at a reign of three in a
    // front count of six through the classification
    // entry. Fails under V-1, V-2, V-3, its reign
    // reading as its largest lead of two, V-5, its
    // reign reading as its longest win run of two, and
    // V-6, its front count reading as a behind count of
    // nought. Passes under V-4 by design, and under
    // V-7, V-8, and V-9 by design, the entry standing
    // unmoved by each.
    let dir = temp_dir("reigns-reached-early");
    let series = series_json(
        "the early reign saga",
        vec![ALPHA, BETA],
        5,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", BETA, ALPHA),
            clear_win("race five", ALPHA, BETA),
            clear_win("race six", BETA, ALPHA),
            clear_win("race seven", ALPHA, BETA),
            clear_win("race eight", ALPHA, BETA),
        ],
    );
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        reign_class(&championship.series[0]),
        Some((ReignClass::Broken, 3, 6))
    );
}

#[test]
fn a_series_whose_reign_parts_from_its_depth_stands_broken() {
    // R-10. The reign parted from the depth at hand
    // scale. A hand-built series on the constructed
    // drill one's shape, asserted broken at a reign of
    // three in a front count of five through the
    // classification entry, with its DH-48 class
    // asserted shallow at a depth of nought beside it
    // through the depths closing over it alone, the two
    // readings parting on a series that never stood
    // behind. Fails under V-1, V-2, and V-6 on the
    // reigns half, the series classing unbroken under
    // V-2 and its figures moving under V-6. Passes
    // under V-3 and V-5, its largest lead and its
    // longest win run both standing at three with its
    // reign. Passes under V-4 by design. The depths
    // half stands under every reigns mutation by
    // design.
    let dir = temp_dir("reigns-parted-depth");
    let championship = load(&dir, vec![("one.json", drill_one())], vec!["one.json"]);
    assert_eq!(
        reign_class(&championship.series[0]),
        Some((ReignClass::Broken, 3, 5))
    );
    let depth_lines = depths_report(&championship);
    assert_eq!(
        depth_lines.last().map(String::as_str),
        Some("depths, 1 series, 0 deep, 0 matched, 1 shallow, 0 no winner")
    );
}

#[test]
fn the_duration_readings_part_on_one_series() {
    // R-11. The non-implication witness at hand scale.
    // A hand-built series at a target of five won by
    // beta, alpha, beta, beta, alpha, alpha, alpha,
    // alpha, its winner behind in two runs of one and
    // three standings and ahead in one closing run of
    // two, asserted unbroken at a reign of two in a
    // front count of two here and broken under DH-50 at
    // a spell of three in a behind count of four beside
    // it, the two duration readings parting on one
    // series in the direction R-10 does not witness.
    // Fails under V-1, the series classing broken here.
    // Fails under V-5, its reign reading as its winner's
    // closing run of four wins. Fails under V-6, its
    // front count reading as its behind count of four.
    // Passes under V-2, reign and front count agreeing
    // at two, and under V-3, its largest lead of two
    // agreeing with its reign. Passes under V-4 by
    // design. The spells half stands under every
    // reigns mutation by design.
    let dir = temp_dir("reigns-duration-witness");
    let series = series_json(
        "the duration witness saga",
        vec![ALPHA, BETA],
        5,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", BETA, ALPHA),
            clear_win("race five", ALPHA, BETA),
            clear_win("race six", ALPHA, BETA),
            clear_win("race seven", ALPHA, BETA),
            clear_win("race eight", ALPHA, BETA),
        ],
    );
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        reign_class(&championship.series[0]),
        Some((ReignClass::Unbroken, 2, 2))
    );
    assert_eq!(
        spell_class(&championship.series[0]),
        Some((SpellClass::Broken, 3, 4))
    );
}

#[test]
fn two_series_part_at_the_least_front_count() {
    // R-12. The smallest parting of the classes. Two
    // hand-built series under a target of two, the
    // first won by alpha and alpha standing unbroken at
    // a reign of two in a front count of two, the
    // second won by alpha, beta, alpha standing broken
    // at a reign of one in a front count of two, the
    // least front count at which the classes part.
    // Fails under V-1 on both. Fails under V-2 on the
    // second, its reign reading as its front count and
    // the series classing unbroken. Fails under V-6 on
    // both, the front counts reading as behind counts
    // of nought. Passes under V-3 and V-5, the largest
    // leads and the longest win runs agreeing with the
    // reigns on both series. Passes under V-4 by
    // design, and under V-7, V-8, and V-9 by design,
    // the entry standing unmoved by each.
    let dir = temp_dir("reigns-least-parting");
    let first = series_json(
        "the whole reign saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", ALPHA, BETA),
        ],
    );
    let second = series_json(
        "the parted reign saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
        ],
    );
    let championship = load(
        &dir,
        vec![("one.json", first), ("two.json", second)],
        vec!["one.json", "two.json"],
    );
    assert_eq!(
        reign_class(&championship.series[0]),
        Some((ReignClass::Unbroken, 2, 2))
    );
    assert_eq!(
        reign_class(&championship.series[1]),
        Some((ReignClass::Broken, 1, 2))
    );
}

#[test]
fn a_winner_ahead_at_every_standing_stands_unbroken() {
    // R-13. The reaching form at a target of four. A
    // hand-built series won by alpha, alpha, alpha,
    // beta, alpha, its winner standing ahead at every
    // one of its five decided standings, asserted
    // unbroken at a reign of five in a front count of
    // five through the classification entry. Fails
    // under V-1, the series classing broken. Fails
    // under V-3, its stated reign reading as its
    // largest lead of three. Fails under V-5, its
    // stated reign reading as its opening run of three
    // wins. Fails under V-6, its front count reading as
    // a behind count of nought. Passes under V-2, reign
    // and front count agreeing at five. Passes under
    // V-4 by design, and under V-7, V-8, and V-9 by
    // design, the entry standing unmoved by each.
    let dir = temp_dir("reigns-reaching");
    let series = series_json(
        "the reaching saga",
        vec![ALPHA, BETA],
        4,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", BETA, ALPHA),
            clear_win("race five", ALPHA, BETA),
        ],
    );
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        reign_class(&championship.series[0]),
        Some((ReignClass::Unbroken, 5, 5))
    );
}

#[test]
fn a_series_of_target_one_stands_unbroken() {
    // R-14. The target one sentence of DH-52 at hand
    // scale. A hand-built series decided at its first
    // race under a wins target of one stands unbroken,
    // its only decided race being its deciding race and
    // its winner having won it, its whole report
    // asserted in the pin, closing "reigns, 1 series, 1
    // unbroken, 0 broken, 0 no winner". Fails under
    // V-1, the series classing broken with a section.
    // Fails under V-6, its front count reading as a
    // behind count of nought and the series classing
    // broken. Fails under V-9, its unbroken decided
    // series gaining a section. Passes under V-2, V-3,
    // and V-5, the figures agreeing at one, and under
    // V-7, no section standing whose figure line could
    // move. Passes under V-4 and V-8 by design.
    let dir = temp_dir("reigns-target-one");
    let series = series_json(
        "the single race saga",
        vec![ALPHA, BETA],
        1,
        vec![clear_win("race one", ALPHA, BETA)],
    );
    let lines = reigns_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "reigns",
            "reigns, 1 series, 1 unbroken, 0 broken, 0 no winner"
        ]
    );
}

#[test]
fn the_races_after_the_decision_count_in_neither_figure() {
    // R-15. The tailed fixture. A hand-built series on
    // the constructed drill one's shape recorded with
    // two further races after its decision, both lost
    // by its winner, asserted broken at a reign of
    // three in a front count of five over the decided
    // record the walk leaves, the two recorded races
    // after the decision counting in neither figure.
    // Fails under V-1, the series classing unbroken.
    // Fails under V-2, its reign reading as its front
    // count of five and classing unbroken. Fails under
    // V-6, its front count reading as a behind count of
    // nought. Fails under V-4, the full recorded record
    // reading a reign of five in a front count of seven
    // over its nine recorded races where the decided
    // record reads three in five. Passes under V-3, its
    // decided largest lead of three agreeing with its
    // reign, and under V-5, its winner's longest win
    // run over the decided record standing at three.
    // Passes under V-7, V-8, and V-9 by design, the
    // entry standing unmoved by each.
    let dir = temp_dir("reigns-tailed");
    let tailed = series_json(
        "the constructed reigns drill one",
        vec![PLOVER, DUNLIN],
        5,
        vec![
            clear_win("reigns drill one race one", PLOVER, DUNLIN),
            clear_win("reigns drill one race two", DUNLIN, PLOVER),
            clear_win("reigns drill one race three", PLOVER, DUNLIN),
            clear_win("reigns drill one race four", DUNLIN, PLOVER),
            clear_win("reigns drill one race five", PLOVER, DUNLIN),
            clear_win("reigns drill one race six", PLOVER, DUNLIN),
            clear_win("reigns drill one race seven", PLOVER, DUNLIN),
            clear_win("reigns drill one race eight", DUNLIN, PLOVER),
            clear_win("reigns drill one race nine", DUNLIN, PLOVER),
        ],
    );
    let championship = load(&dir, vec![("one.json", tailed)], vec!["one.json"]);
    assert_eq!(
        reign_class(&championship.series[0]),
        Some((ReignClass::Broken, 3, 5))
    );
}

#[test]
fn the_bolt_and_ember_series_stands_unbroken_at_two_in_two() {
    // R-16. The tenure witness on the shipped record.
    // The league series bolt and ember loaded from the
    // examples, asserted unbroken at a reign of two in
    // a front count of two through the classification
    // entry, with its DH-46 class asserted behind
    // beside it through the tenure closing over it
    // alone, its winner's front count of two standing
    // below its loser's three, a series the counts
    // class by its loser's lead standing unbroken here
    // on its one late run of ahead standings. Fails
    // under V-1, the series classing broken. Fails
    // under V-4, the full recorded record reading its
    // figures at seven in seven. Fails under V-5, its
    // reign reading as its winner's longest win run of
    // three. Fails under V-6, its front count reading
    // as its behind count of four. Passes under V-2 and
    // under V-3, its largest lead of two agreeing with
    // its reign. The tenure half stands under every
    // reigns mutation by design.
    let championship = load_example("league-championship.json");
    let series = championship.series[6].clone();
    assert_eq!(reign_class(&series), Some((ReignClass::Unbroken, 2, 2)));
    let alone = Championship {
        series: vec![series],
    };
    let tenure_lines = tenure_report(&alone);
    assert_eq!(
        tenure_lines.last().map(String::as_str),
        Some("tenure, 1 series, 0 ahead, 0 level, 1 behind, 0 no winner")
    );
}

#[test]
fn the_constructed_drill_one_stands_broken_here_and_deadlocked_there() {
    // R-17. The deadlock witness on the shipped
    // record. The constructed reigns drill one loaded
    // from the examples, asserted broken at a reign of
    // three in a front count of five here, with the
    // deadlock reading classing the series deadlocked
    // beside it through the deadlock closing over it
    // alone, a level standing of its decided record
    // standing between two of its ahead standings at
    // its second and fourth decided races and so before
    // its deciding race, every broken series carrying
    // such a standing as DH-52's form requires. Fails
    // under V-1, V-2, and V-6 on the reigns half.
    // Passes under V-3 and V-5, its largest lead and its
    // longest win run both standing at three with its
    // reign, and under V-4, the series deciding at its
    // last recorded race. The deadlock half stands
    // under every reigns mutation by design.
    let championship = load_example("constructed-reigns.json");
    let series = championship.series[0].clone();
    assert_eq!(reign_class(&series), Some((ReignClass::Broken, 3, 5)));
    let alone = Championship {
        series: vec![series],
    };
    let deadlock_lines = deadlock_report(&alone);
    assert_eq!(
        deadlock_lines.last().map(String::as_str),
        Some("deadlock, 1 series, 1 deadlocked, 0 clear, 0 no winner")
    );
}

#[test]
fn an_adjudicated_race_counts_in_the_reign() {
    // R-18. The adjudication held entry at hand scale.
    // A hand-built series at a target of three whose
    // third race's attempts stand both red and named by
    // an adjudication entry for alpha, the race decided
    // to the named attempt under DH-5, its winner's
    // race sequence standing won, lost, adjudicated
    // won, lost, won, its standings standing ahead,
    // level, ahead, level, ahead, its whole report
    // asserted in the pin with its one section, the
    // series standing broken at a reign of one in a
    // front count of three, the adjudicated race
    // numbered three among its decided races. Fails
    // under V-1, the series classing unbroken. Fails
    // under V-2, its reign reading as its front count
    // of three and the series classing unbroken with no
    // section. Fails under V-6, its front count reading
    // as a behind count of nought. Fails under V-7, its
    // figure line printing the front count of three in
    // place of the reign of one. Passes under V-3 and
    // V-5, its largest lead and its longest win run both
    // standing at one with its reign, and under V-9,
    // its one decided series already standing
    // sectioned. Passes under V-4 and V-8 by design.
    let dir = temp_dir("reigns-adjudicated");
    let series = series_json(
        "the adjudicated reign saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            adjudicated_win("race three", ALPHA, BETA, ALPHA),
            clear_win("race four", BETA, ALPHA),
            clear_win("race five", ALPHA, BETA),
        ],
    );
    let lines = reigns_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "reigns",
            "series 1, the adjudicated reign saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, alpha-1 wins",
            "race 4, race four, beta-1 wins",
            "race 5, race five, alpha-1 wins",
            "longest reign 1, front count 3 over the decided record",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 2 race wins, 0 no results",
            "reigns, 1 series, 0 unbroken, 1 broken, 0 no winner",
        ]
    );
}

#[test]
fn the_longest_reign_arrives_late_after_a_single_standing() {
    // R-19. The late reign. A hand-built series at a
    // target of five won by alpha, beta, alpha, alpha,
    // alpha, alpha, its reign runs standing at one and
    // four, the longest reign standing last, asserted
    // broken at a reign of four in a front count of
    // five through the classification entry, the mirror
    // of R-8 on the same count. Fails under V-1, the
    // series classing unbroken. Fails under V-2, its
    // reign reading as its front count of five and
    // classing unbroken. Fails under V-6, its front
    // count reading as a behind count of nought.
    // Passes under V-3, its largest lead of four
    // agreeing with its reign, and under V-5, its
    // closing run of four wins agreeing with its reign.
    // Passes under V-4 by design, and under V-7, V-8,
    // and V-9 by design, the entry standing unmoved by
    // each.
    let dir = temp_dir("reigns-late");
    let series = series_json(
        "the late reign saga",
        vec![ALPHA, BETA],
        5,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
            clear_win("race six", ALPHA, BETA),
        ],
    );
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        reign_class(&championship.series[0]),
        Some((ReignClass::Broken, 4, 5))
    );
}

#[test]
fn a_collection_holding_no_series_renders_the_empty_form_at_the_library_surface() {
    // R-20. The empty form, pinned from birth. No
    // championship file carries an empty collection,
    // because the loader of DH-13 refuses a series list
    // holding no names, so this form stands at the
    // library surface alone, over a collection built by
    // hand. The pin fails under no mutation of the
    // nine, the empty rendering being the two lines
    // DH-53 states and the mutation set carrying no
    // variant under which it changes.
    let championship = Championship { series: vec![] };
    let lines = reigns_report(&championship);
    assert_eq!(
        lines,
        vec![
            "reigns",
            "reigns, 0 series, 0 unbroken, 0 broken, 0 no winner"
        ]
    );
}

#[test]
fn the_record_count_stands_skipped_over_a_no_result_race() {
    // R-21. The record count at hand scale, the pin
    // FIX-DH-29 names for this file. A hand-built series
    // at a target of two whose first recorded race holds
    // no winner, its winner alpha-1 taking the second
    // and fourth recorded races against beta-1's third,
    // stands broken at a reign of one in a front count
    // of two, and its whole report stands asserted, its
    // section printing its decided races at their counts
    // in the record, race 2, race 3, and race 4, the
    // count of the race holding no winner standing
    // skipped, as the stated relations of DH-53 now
    // state the count's meaning. Fails under V-1, the
    // series classing unbroken and taking no section.
    // Fails under V-2, its reign reading as its front
    // count of two and the series classing unbroken.
    // Fails under V-6, its figure line printing a front
    // count of nought against the asserted two. Fails
    // under V-7, its figure line printing the front
    // count of two in the reign's place. Fails under
    // V-10, its decided races standing renumbered as
    // race 1, race 2, and race 3. Passes under V-3 and
    // V-5, its winner's largest lead and its winner's
    // longest run of wins both standing at one with its
    // reign, under V-4, its recorded record ending at
    // its decision, and under V-8 and V-9 by design.
    let dir = temp_dir("reigns-record-count");
    let series = series_json(
        "the record count drill",
        vec![ALPHA, BETA],
        2,
        vec![
            no_result("record count drill race one", ALPHA, BETA),
            clear_win("record count drill race two", ALPHA, BETA),
            clear_win("record count drill race three", BETA, ALPHA),
            clear_win("record count drill race four", ALPHA, BETA),
        ],
    );
    let lines = reigns_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "reigns",
            "series 1, the record count drill",
            "alpha-1 against beta-1, first to 2 wins",
            "race 2, record count drill race two, alpha-1 wins",
            "race 3, record count drill race three, beta-1 wins",
            "race 4, record count drill race four, alpha-1 wins",
            "longest reign 1, front count 2 over the decided record",
            "series winner, alpha-1, alpha-1 2 race wins, beta-1 1 race wins, 1 no results",
            "reigns, 1 series, 0 unbroken, 1 broken, 0 no winner"
        ]
    );
}
