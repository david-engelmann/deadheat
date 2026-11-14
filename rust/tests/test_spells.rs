//! Pins for DH-50 and DH-51 in the Round 25 section of
//! CONTRACT.md.
//!
//! The discrimination record of this file stands pin by
//! pin, and no blanket claim stands over it. The spells
//! reading, its library function, and its command do
//! not stand on the pre round tree, so every pin in
//! this file that names the spells reading is absent
//! there at compile time. That absence is recorded
//! once here and is not a discrimination claim. The
//! discrimination each pin carries is against the
//! stated mutation of DH-50 beside it. Nine mutations
//! are named for the whole file. V-1, the class cuts
//! exchanged, a series classing unbroken where the
//! shipped reading classes it broken and broken where
//! it classes it unbroken, the never behind class
//! unmoved. V-2, the longest spell read as the behind
//! count in place of the longest run. V-3, the longest
//! spell read as the depth of DH-48 in place of the
//! longest run. V-4, the standings read over every
//! decided race of the full recorded record in place
//! of the decided record the walk leaves, so races
//! recorded after the decision count in the spell and
//! the count. V-5, the longest spell read as the
//! winner's longest run of wins, the streak figure of
//! DH-27, in place of the longest run of behind
//! standings. V-6, the behind count read as the
//! winner's front count of DH-46 in place of the count
//! of behind standings. V-7, the figure line printing
//! the behind count in place of the longest spell,
//! the classes standing true. V-8, a series holding
//! no winner placed in the never behind class in
//! place of standing in no class. V-9, the section
//! standing for every decided series in place of the
//! broken series alone.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{load_championship_file, Championship};
use deadheat::depths::depths_report;
use deadheat::json::JsonValue;
use deadheat::spells::{spell_class, spells_report, SpellClass};
use shared::*;

const ALPHA: &str = "alpha-1";
const BETA: &str = "beta-1";
const HARRIER: &str = "harrier-1";
const CURLEW: &str = "curlew-1";
const GANNET: &str = "gannet-1";
const AVOCET: &str = "avocet-1";

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

fn spells_for(dir: &Path, files: Vec<(&str, JsonValue)>, names: Vec<&str>) -> Vec<String> {
    spells_report(&load(dir, files, names))
}

fn load_example(name: &str) -> Championship {
    let (championship, defects) = load_championship_file(&examples().join(name));
    assert_eq!(defects, Vec::<String>::new());
    championship.expect("a championship")
}

fn golden(name: &str) -> String {
    std::fs::read_to_string(examples().join(name)).expect("a golden")
}

fn spell_figures(lines: &[String]) -> Vec<u64> {
    lines
        .iter()
        .filter_map(|line| {
            line.strip_prefix("longest spell ")
                .and_then(|rest| rest.split(", behind count ").next())
                .and_then(|figure| figure.parse::<u64>().ok())
        })
        .collect()
}

fn class_spell(series: &deadheat::series::Series) -> Option<(SpellClass, u64)> {
    spell_class(series).map(|(class, spell, _)| (class, spell))
}

#[test]
fn the_standing_championship_stands_never_behind_in_every_series() {
    // W-1. The standing championship golden, no section
    // standing, closing "spells, 2 series, 0 unbroken, 0
    // broken, 2 never behind, 0 no winner", the pilot
    // and the panel both standing never behind at a
    // spell of nought in a behind count of nought.
    // Fails under V-6, the behind count of each series
    // reading as its winner's front count, four for
    // the pilot and three for the panel, so both
    // series class broken and gain sections. Fails under V-9, both never behind
    // series gaining sections. Passes under V-1 by
    // design, the never behind class standing
    // unmoved by the exchange of the other two cuts.
    // Passes under V-2 and V-3, the spell, the count,
    // and the depth agreeing at nought on both series.
    // Passes under V-4 by the recorded count, neither
    // series gaining a behind standing over its full
    // record. Passes under V-5, the winners' runs of
    // wins moving no class whose behind count stands
    // at nought. Passes under V-7 and V-8 by design,
    // no section standing and no series holding no
    // winner.
    let championship = load_example("championship.json");
    let lines = spells_report(&championship);
    assert_eq!(
        lines,
        vec![
            "spells",
            "spells, 2 series, 0 unbroken, 0 broken, 2 never behind, 0 no winner"
        ]
    );
    assert_eq!(lines.join("\n") + "\n", golden("championship-spells.txt"));
}

#[test]
fn the_constructed_example_carries_the_first_series_alone() {
    // W-2. The constructed spells golden, the first
    // series standing in section form at the figure
    // three in a behind count of five, its winner
    // harrier-1 standing broken, the second series
    // standing unbroken at a spell of five in a behind
    // count of five and taking no section, closing
    // "spells, 2 series, 1 unbroken, 1 broken, 0 never
    // behind, 0 no winner". Fails under V-1, the two
    // series exchanging classes and the section moving
    // to the second series. Fails under V-2, the first
    // series' spell reading as its count of five and
    // classing unbroken. Fails under V-3, the spells
    // reading as the depths of two and three. Fails
    // under V-5, the first series' spell reading as
    // its winner's run of two. Fails under V-6, the
    // second series' behind count reading as its
    // winner's front count of two and classing broken.
    // Fails under V-7, the figure line printing the
    // five in place of the three. Fails under V-9, the
    // unbroken second series gaining a section. Passes
    // under V-4 by design, neither series of the pair
    // carrying a race past its decision. Passes under
    // V-8 by design, no series holding no winner.
    let championship = load_example("constructed-spells.json");
    let lines = spells_report(&championship);
    assert_eq!(
        lines,
        vec![
            "spells",
            "series 1, the constructed spells drill one",
            "harrier-1 against curlew-1, first to 5 wins",
            "race 1, spells drill one race one, curlew-1 wins",
            "race 2, spells drill one race two, curlew-1 wins",
            "race 3, spells drill one race three, harrier-1 wins",
            "race 4, spells drill one race four, harrier-1 wins",
            "race 5, spells drill one race five, curlew-1 wins",
            "race 6, spells drill one race six, harrier-1 wins",
            "race 7, spells drill one race seven, curlew-1 wins",
            "race 8, spells drill one race eight, harrier-1 wins",
            "race 9, spells drill one race nine, harrier-1 wins",
            "longest spell 3, behind count 5 over the decided record",
            "series winner, harrier-1, harrier-1 5 race wins, curlew-1 4 race wins, 0 no results",
            "spells, 2 series, 1 unbroken, 1 broken, 0 never behind, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-spells-spells.txt")
    );
}

#[test]
fn the_survey_championship_matches_its_spells_golden() {
    // W-3. The survey championship golden, no section
    // standing, its six series standing unbroken at a
    // spell of one in a count of one four times over
    // and never behind twice, closing "spells, 6
    // series, 4 unbroken, 0 broken, 2 never behind, 0
    // no winner". Fails under V-1, the four unbroken
    // series moving to broken and gaining sections.
    // Fails under V-4, the full records of the survey
    // series carrying behind standings the walk stops
    // short of, the apex against bolt series reading a
    // spell of three in a count of four over its full
    // record and the apex against crest series gaining
    // a behind standing past its decision. Fails under
    // V-5 and V-6 on the classes, the winners' runs and
    // front counts parting from the spells and counts.
    // Fails under V-9, its decided series gaining
    // sections. Passes under V-2, the spell and the
    // behind count agreeing on every series of the
    // six. Passes under V-3, the spell and the depth
    // agreeing on every series of the six. Passes
    // under V-7 by design, no section standing for its
    // figure to move in. Passes under V-8 by design,
    // no series holding no winner.
    let championship = load_example("survey-championship.json");
    let lines = spells_report(&championship);
    assert_eq!(spell_figures(&lines), Vec::<u64>::new());
    assert_eq!(
        lines,
        vec![
            "spells",
            "spells, 6 series, 4 unbroken, 0 broken, 2 never behind, 0 no winner"
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("survey-championship-spells.txt")
    );
}

#[test]
fn the_classification_reads_the_spell_against_the_behind_count() {
    // W-4. The classes at championship scale, the two
    // series of the constructed pair in hand built
    // form. The first, at a target of five won by
    // curlew, curlew, harrier, harrier, curlew,
    // harrier, curlew, harrier, harrier, stands broken
    // at a spell of three in a behind count of five,
    // and the second, won by avocet three times and
    // then gannet five times, stands unbroken at a
    // spell of five in a behind count of five, the two
    // asserted through the classification entry with
    // their figures stated beside their classes, and
    // the first series' figure line standing in the
    // report. Fails under V-1, the two classes
    // exchanging at the entry. Fails under V-2, the
    // first series' spell reading as its behind count
    // of five and classing unbroken. Fails under V-3,
    // the spells reading as the depths of two and
    // three at the entry. Fails under V-5, the first
    // series' spell reading as its winner's longest
    // run of two. Fails under V-6, the second series'
    // behind count reading as its winner's front
    // count of two and classing broken. Fails under
    // V-7, the figure line printing the behind count
    // of five in place of the spell of three. Passes
    // under V-4 by design, neither series carrying a
    // race past its decision. Passes under V-8 by
    // design, no series holding no winner. Passes
    // under V-9, the entry and the figure standing
    // unmoved, the sections the mutant adds standing
    // outside the assertions.
    let dir = temp_dir("spells-classification");
    let first = series_json(
        "the constructed spells drill one",
        vec![HARRIER, CURLEW],
        5,
        vec![
            clear_win("spells drill one race one", CURLEW, HARRIER),
            clear_win("spells drill one race two", CURLEW, HARRIER),
            clear_win("spells drill one race three", HARRIER, CURLEW),
            clear_win("spells drill one race four", HARRIER, CURLEW),
            clear_win("spells drill one race five", CURLEW, HARRIER),
            clear_win("spells drill one race six", HARRIER, CURLEW),
            clear_win("spells drill one race seven", CURLEW, HARRIER),
            clear_win("spells drill one race eight", HARRIER, CURLEW),
            clear_win("spells drill one race nine", HARRIER, CURLEW),
        ],
    );
    let second = series_json(
        "the constructed spells drill two",
        vec![GANNET, AVOCET],
        5,
        vec![
            clear_win("spells drill two race one", AVOCET, GANNET),
            clear_win("spells drill two race two", AVOCET, GANNET),
            clear_win("spells drill two race three", AVOCET, GANNET),
            clear_win("spells drill two race four", GANNET, AVOCET),
            clear_win("spells drill two race five", GANNET, AVOCET),
            clear_win("spells drill two race six", GANNET, AVOCET),
            clear_win("spells drill two race seven", GANNET, AVOCET),
            clear_win("spells drill two race eight", GANNET, AVOCET),
        ],
    );
    let championship = load(
        &dir,
        vec![("one.json", first), ("two.json", second)],
        vec!["one.json", "two.json"],
    );
    assert_eq!(
        class_spell(&championship.series[0]),
        Some((SpellClass::Broken, 3))
    );
    assert_eq!(
        class_spell(&championship.series[1]),
        Some((SpellClass::Unbroken, 5))
    );
    let lines = spells_report(&championship);
    assert!(lines.contains(&"longest spell 3, behind count 5 over the decided record".to_string()));
}

#[test]
fn a_series_at_the_smallest_count_above_nought_stands_unbroken() {
    // W-5. The boundary of the classes at the smallest
    // count above nought. A hand-built series at a
    // target of two won by beta, alpha, alpha stands
    // behind in one spell of one and then level and
    // decided, asserted unbroken at a spell of one in
    // a behind count of one through the classification
    // entry. Fails under V-1, the series classing
    // broken at the entry. Fails under V-5, its spell
    // reading as its winner's longest run of two and
    // classing broken. Passes under V-2, its spell and
    // its behind count agreeing at one. Passes under
    // V-3, its depth standing at one beside its spell.
    // Passes under V-6, its winner's front count of
    // one standing in place of its behind count of one
    // at the entry. Passes under V-4, V-7, V-8, and
    // V-9 by design, the entry standing unmoved by
    // each and no race standing past the decision.
    let dir = temp_dir("spells-smallest-count");
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
        class_spell(&championship.series[0]),
        Some((SpellClass::Unbroken, 1))
    );
}

#[test]
fn a_champion_who_never_stood_behind_stands_never_behind_at_nought() {
    // W-6. Spell and count at nought at hand scale. A
    // hand-built series at a target of three won by
    // alpha, beta, alpha, alpha, its winner winning
    // the first decided race and never standing
    // behind, stands never behind with the spell and
    // the count stated as nought through the
    // classification entry. Fails under V-5, its
    // stated spell of nought reading as its winner's
    // longest run of two. Fails under V-6, its behind
    // count reading as its winner's front count of
    // three and classing broken. Passes under V-1 by
    // design, the never behind class standing unmoved
    // by the exchange. Passes under V-2 and V-3, its
    // spell, count, and depth agreeing at nought.
    // Passes under V-4, V-7, V-8, and V-9 by design,
    // the entry standing unmoved by each.
    let dir = temp_dir("spells-wire");
    let series = series_json(
        "the never behind saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
        ],
    );
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        class_spell(&championship.series[0]),
        Some((SpellClass::NeverBehind, 0))
    );
}

#[test]
fn two_series_at_one_behind_count_part_on_the_spell() {
    // W-7. The twin of the tie in the count. Two
    // hand-built series sharing the behind count of
    // three, the first at a target of three won by
    // beta, beta, alpha, alpha, alpha standing in one
    // spell of three and asserted unbroken, the second
    // at a target of four won by beta, alpha, beta,
    // alpha, beta, alpha, alpha standing in three
    // spells of one and asserted broken, the two
    // through the classification entry and the second
    // series' figure line standing in the report.
    // Fails under V-1, the two classes exchanging.
    // Fails under V-2, the second series' spell
    // reading as its count of three and classing
    // unbroken. Fails under V-3, the first series'
    // spell reading as its depth of two and classing
    // broken. Fails under V-5, the second series'
    // spell reading as its winner's run of two at the
    // entry. Fails under V-6, the first series' behind
    // count reading as its winner's front count of one
    // and classing broken, and the second series'
    // behind count reading as its front count of one
    // standing equal to its spell and classing
    // unbroken. Fails under V-7, the second series'
    // figure line printing the three in place of the
    // one. Passes under V-4, V-8, and V-9 by design.
    let dir = temp_dir("spells-one-count");
    let first = series_json(
        "the one run saga",
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
        "the three runs saga",
        vec![ALPHA, BETA],
        4,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", BETA, ALPHA),
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
        class_spell(&championship.series[0]),
        Some((SpellClass::Unbroken, 3))
    );
    assert_eq!(
        class_spell(&championship.series[1]),
        Some((SpellClass::Broken, 1))
    );
    let lines = spells_report(&championship);
    assert!(spell_figures(&lines).contains(&1));
}

#[test]
fn the_longest_spell_stands_at_its_longest_not_its_last() {
    // W-8. The longest standing at hand scale. A
    // hand-built series at a target of four won by
    // beta, beta, alpha, alpha, beta, alpha, alpha,
    // its behind standings growing in a run of three,
    // level, and growing again in a run of one,
    // asserted broken at the longest standing of
    // three and not the last standing of one and not
    // the count of four, through the classification
    // entry and its figure line. Fails under V-1, the
    // series classing unbroken. Fails under V-2, its
    // spell reading as its behind count of four and
    // classing unbroken. Fails under V-3, its stated
    // spell reading as its depth of two. Fails under
    // V-5, its stated spell reading as its winner's
    // longest run of two. Fails under V-7, its figure
    // line printing the four in place of the three.
    // Passes under V-6, its behind count reading as
    // its winner's front count of one at the entry
    // while its class and its stated spell stand, the
    // assertions reading neither. Passes under V-4,
    // V-8, and V-9 by design.
    let dir = temp_dir("spells-longest");
    let series = series_json(
        "the longest standing saga",
        vec![ALPHA, BETA],
        4,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", BETA, ALPHA),
            clear_win("race six", ALPHA, BETA),
            clear_win("race seven", ALPHA, BETA),
        ],
    );
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        class_spell(&championship.series[0]),
        Some((SpellClass::Broken, 3))
    );
    let lines = spells_report(&championship);
    assert!(spell_figures(&lines).contains(&3));
}

#[test]
fn the_spell_reached_early_stands_against_the_later_races() {
    // W-9. The spell reached partway at hand scale.
    // Two hand-built series. The first, at a target of
    // three won by beta, beta, alpha, alpha, alpha,
    // stands in one spell of three early in its
    // decided record, and its fixture's file carries
    // two recorded races past its decision, both won
    // by beta, asserted unbroken at a spell of three
    // in a behind count of three over the decided
    // record the walk leaves. The second, at a target
    // of five won by beta, beta, alpha, alpha, alpha,
    // beta, beta, alpha, alpha, stands in a run of
    // three early and a single behind standing late,
    // asserted broken at the early spell of three and
    // not the late standing of one. Fails under V-1,
    // the two classes exchanging. Fails under V-2,
    // the second series' spell reading as its count
    // of four and classing unbroken. Fails under V-3,
    // the first series' spell reading as its depth of
    // two and classing broken. Fails under V-4, the
    // first series' two recorded races past its
    // decision counting in its standings, its behind
    // count reading four over the full record and
    // classing broken. Fails under V-6, the first
    // series' behind count reading as its winner's
    // front count of one and classing broken. Fails
    // under V-7, the second series' figure line
    // printing the four in place of the three. Passes
    // under V-5, the two winners' longest runs of
    // wins standing at three beside their spells.
    // Passes under V-8 and V-9 by design.
    let dir = temp_dir("spells-early");
    let first = series_json(
        "the early spell saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
            clear_win("race six", BETA, ALPHA),
            clear_win("race seven", BETA, ALPHA),
        ],
    );
    let second = series_json(
        "the late standing saga",
        vec![ALPHA, BETA],
        5,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
            clear_win("race six", BETA, ALPHA),
            clear_win("race seven", BETA, ALPHA),
            clear_win("race eight", ALPHA, BETA),
            clear_win("race nine", ALPHA, BETA),
        ],
    );
    let championship = load(
        &dir,
        vec![("one.json", first), ("two.json", second)],
        vec!["one.json", "two.json"],
    );
    assert_eq!(
        class_spell(&championship.series[0]),
        Some((SpellClass::Unbroken, 3))
    );
    assert_eq!(
        class_spell(&championship.series[1]),
        Some((SpellClass::Broken, 3))
    );
    let lines = spells_report(&championship);
    assert!(spell_figures(&lines).contains(&3));
}

#[test]
fn a_series_whose_spell_parts_from_its_depth_stands_broken() {
    // W-10. The spell parted from the depth at hand
    // scale. A hand-built series at a target of five
    // won by beta, beta, alpha, alpha, beta, alpha,
    // alpha, alpha stands at a longest spell of three
    // against a depth of two in a behind count of
    // four, asserted broken with the figure stated at
    // three through the classification entry and its
    // figure line. Fails under V-1, the series
    // classing unbroken. Fails under V-2, its spell
    // reading as its behind count of four. Fails under
    // V-3, its stated spell reading as its depth of
    // two. Fails under V-7, its figure line printing
    // the four in place of the three. Passes under
    // V-5 by the stated coincidence, its winner's
    // longest run of wins standing at three beside
    // its spell. Passes under V-6, its behind count
    // reading as its winner's front count of two at
    // the entry while its class and its stated spell
    // stand. Passes under V-4, V-8, and V-9 by design.
    let dir = temp_dir("spells-parted-depth");
    let series = series_json(
        "the parted figures saga",
        vec![ALPHA, BETA],
        5,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", BETA, ALPHA),
            clear_win("race six", ALPHA, BETA),
            clear_win("race seven", ALPHA, BETA),
            clear_win("race eight", ALPHA, BETA),
        ],
    );
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        class_spell(&championship.series[0]),
        Some((SpellClass::Broken, 3))
    );
    let lines = spells_report(&championship);
    assert!(spell_figures(&lines).contains(&3));
}

#[test]
fn the_constructed_drill_two_shape_stands_broken_here_and_matched_there() {
    // W-11. The non implication witness at hand
    // scale, in the constructed depths drill two's
    // shape. A hand-built series at a target of three
    // won by beta, alpha, beta, alpha, alpha stands
    // at a spell of one in a behind count of two,
    // asserted broken here through the classification
    // entry and its figure line, while the same
    // loaded series stands matched under DH-48 at a
    // depth of one against a margin of one, neither
    // reading's class implying the other's. Fails
    // under V-1, the series classing unbroken here.
    // Fails under V-2, its spell reading as its behind
    // count of two and classing unbroken. Fails under
    // V-5, its spell reading as its winner's run of
    // two and classing unbroken. Fails under V-6, its
    // behind count reading as its winner's front
    // count of one standing equal to its spell and
    // classing unbroken. Fails under V-7, its figure
    // line printing the two in place of the one.
    // Passes under V-3 by the stated coincidence, its
    // depth standing at one beside its spell. Passes
    // under V-4, V-8, and V-9 by design, and its
    // depths assertions standing outside the
    // mutations.
    let dir = temp_dir("spells-witness");
    let series = series_json(
        "the matched witness saga",
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
        class_spell(&championship.series[0]),
        Some((SpellClass::Broken, 1))
    );
    let lines = spells_report(&championship);
    assert!(spell_figures(&lines).contains(&1));
    let depth_lines = depths_report(&championship);
    assert_eq!(
        depth_lines.last().map(String::as_str),
        Some("depths, 1 series, 0 deep, 1 matched, 0 shallow, 0 no winner")
    );
}

#[test]
fn two_series_part_at_one_margin_on_the_count_against_nought() {
    // W-12. The boundary pair at one margin, the tie
    // of the count against nought read from both
    // sides. Two hand-built series at a target of
    // three, the first won by alpha, beta, alpha,
    // beta, alpha standing never behind at a margin
    // of one, the second won by beta, alpha, alpha,
    // beta, alpha standing unbroken at a spell of one
    // in a behind count of one at the same margin of
    // one, the two asserted through the classification
    // entry. Fails under V-1, the second series
    // classing broken. Fails under V-5, the second
    // series' spell reading as its winner's run of
    // two and classing broken. Fails under V-6, the
    // first series' behind count reading as its
    // winner's front count of three and classing
    // broken, and the second series' behind count
    // reading as its front count of two and classing
    // broken. Passes under V-2 and V-3, the spells,
    // counts, and depths of the two standing as
    // stated. Passes under V-4, V-7, V-8, and V-9 by
    // design.
    let dir = temp_dir("spells-one-margin");
    let first = series_json(
        "the front margin saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", BETA, ALPHA),
            clear_win("race five", ALPHA, BETA),
        ],
    );
    let second = series_json(
        "the behind margin saga",
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
    let championship = load(
        &dir,
        vec![("one.json", first), ("two.json", second)],
        vec!["one.json", "two.json"],
    );
    assert_eq!(
        spell_class(&championship.series[0]).map(|(class, _, _)| class),
        Some(SpellClass::NeverBehind)
    );
    assert_eq!(
        spell_class(&championship.series[1]).map(|(class, _, _)| class),
        Some(SpellClass::Unbroken)
    );
}

#[test]
fn a_winner_behind_through_five_standings_stands_unbroken() {
    // W-13. The reaching form at a target of four. A
    // hand-built series won by beta, beta, alpha,
    // beta, alpha, alpha, alpha, its winner standing
    // behind through five consecutive decided races
    // and taking the series at the last, asserted
    // unbroken at a spell of five in a behind count of
    // five through the classification entry. Fails
    // under V-1, the series classing broken. Fails
    // under V-3, its stated spell reading as its depth
    // of two and classing broken. Fails under V-5, its
    // stated spell reading as its winner's longest run
    // of three and classing broken. Fails under V-6,
    // its behind count reading as its winner's front
    // count of one and classing broken. Passes under
    // V-2, its spell and its behind count agreeing at
    // five. Passes under V-4, V-7, V-8, and V-9 by
    // design.
    let dir = temp_dir("spells-reaching");
    let series = series_json(
        "the reaching saga",
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
        class_spell(&championship.series[0]),
        Some((SpellClass::Unbroken, 5))
    );
}

#[test]
fn a_series_of_target_one_stands_never_behind() {
    // W-14. The target one sentence of DH-50 at hand
    // scale. A hand-built series decided at its first
    // race under a wins target of one stands never
    // behind, its only decided race being its deciding
    // race and its winner having won it, its whole
    // report asserted in the pin, closing "spells, 1
    // series, 0 unbroken, 0 broken, 1 never behind, 0
    // no winner". Fails under V-6, its behind count
    // reading as its winner's front count of one and
    // the series classing broken with a section.
    // Fails under V-9, its never behind decided series
    // gaining a section. Passes under V-1 by design,
    // the never behind class standing unmoved. Passes
    // under V-2, V-3, V-4, V-5, V-7, and V-8 by
    // design, its spell, count, and depth agreeing at
    // nought and no section standing.
    let dir = temp_dir("spells-target-one");
    let series = series_json(
        "the single race saga",
        vec![ALPHA, BETA],
        1,
        vec![clear_win("race one", ALPHA, BETA)],
    );
    let lines = spells_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "spells",
            "spells, 1 series, 0 unbroken, 0 broken, 1 never behind, 0 no winner"
        ]
    );
}

#[test]
fn a_championship_holding_no_winner_stands_in_no_class() {
    // W-15. The no winner forms at hand scale, two in
    // number. A hand-built series carrying no decided
    // race, its two races standing without a winner,
    // and a second whose target stands unreached, its
    // two decided races standing one to each model
    // under a target of three, each stand in no class
    // and count in the closing's no winner figure,
    // neither counting under never behind, closing
    // "spells, 2 series, 0 unbroken, 0 broken, 0 never
    // behind, 2 no winner". Fails under V-8 alone,
    // both series moving into the never behind class
    // and the closing reading two never behind against
    // no no winner. Passes under the eight remaining
    // mutations by design, no class, figure, or
    // section standing in either series for them to
    // move.
    let dir = temp_dir("spells-no-winner");
    let silent = series_json(
        "the silent saga",
        vec![ALPHA, BETA],
        2,
        vec![
            no_result("race one", ALPHA, BETA),
            no_result("race two", ALPHA, BETA),
        ],
    );
    let unreached = series_json(
        "the unreached saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
        ],
    );
    let lines = spells_for(
        &dir,
        vec![("one.json", silent), ("two.json", unreached)],
        vec!["one.json", "two.json"],
    );
    assert_eq!(
        lines,
        vec![
            "spells",
            "spells, 2 series, 0 unbroken, 0 broken, 0 never behind, 2 no winner"
        ]
    );
}

#[test]
fn a_series_broken_by_the_smallest_parting_stands_broken() {
    // W-16. The smallest parting of the spell and the
    // count. A hand-built series at a target of three
    // won by beta, alpha, beta, alpha, alpha stands at
    // a spell of one in a behind count of two, the
    // smallest count at which the spell and the count
    // part, a count of two standing in two runs of one
    // by the parity of the walk, asserted broken
    // through the classification entry and its figure
    // line. Fails under V-1, the series classing
    // unbroken. Fails under V-2, its spell reading as
    // its behind count of two and classing unbroken.
    // Fails under V-5, its spell reading as its
    // winner's run of two and classing unbroken. Fails
    // under V-6, its behind count reading as its
    // winner's front count of one standing equal to
    // its spell and classing unbroken. Fails under
    // V-7, its figure line printing the two in place
    // of the one. Passes under V-3 by the stated
    // coincidence, its depth standing at one beside
    // its spell. Passes under V-4, V-8, and V-9 by
    // design.
    let dir = temp_dir("spells-smallest-parting");
    let series = series_json(
        "the smallest parting saga",
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
        class_spell(&championship.series[0]),
        Some((SpellClass::Broken, 1))
    );
    let lines = spells_report(&championship);
    assert!(spell_figures(&lines).contains(&1));
}

#[test]
fn the_league_championship_matches_its_spells_closing() {
    // W-17. The league closing line alone, its one
    // broken series being the census of the broken
    // class on a shipped collection, asserted as a
    // line and not through the whole golden, the pin
    // that prices the league's four unbroken series
    // and its five never behind against a defect that
    // reads the whole collection, closing "spells, 10
    // series, 4 unbroken, 1 broken, 5 never behind, 0
    // no winner". Fails under V-1, its counts moving
    // between unbroken and broken. Fails under V-2,
    // the bolt against ember series classing unbroken
    // at a spell read as its count of four. Fails
    // under V-4, the full records of the league series
    // carrying behind standings the walk stops short
    // of, the bolt against crest series gaining two
    // behind standings past its decision among them.
    // Fails under V-5 and V-6 on the classes. Passes
    // under V-3 by execution, the league's classes
    // standing as the spells read as the depths, the
    // bolt against ember series standing broken at a
    // depth of two in its count of four. Passes under
    // V-7 by design, the closing only pin asserting no
    // figure. Passes under V-8 and V-9 by design, the
    // closing counting no section.
    let championship = load_example("league-championship.json");
    let lines = spells_report(&championship);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("spells, 10 series, 4 unbroken, 1 broken, 5 never behind, 0 no winner")
    );
}

#[test]
fn an_adjudicated_race_counts_in_the_spell() {
    // W-18. The adjudication held entry at hand
    // scale. A hand-built series at a target of three
    // whose first race's attempts stand both red and
    // named by an adjudication entry for beta, the
    // race decided to the named attempt under DH-5,
    // and then won by alpha, alpha, beta, alpha,
    // stands unbroken at a spell of one in a behind
    // count of one, the spell read on the record DH-5
    // decided, asserted through the classification
    // entry, the series standing never behind on the
    // record without the adjudicated race. Fails under
    // V-1, the series classing broken. Fails under
    // V-5, its spell reading as its winner's run of
    // two and classing broken. Fails under V-6, its
    // behind count reading as its winner's front count
    // of two and classing broken. Passes under V-2 and
    // V-3, its spell, count, and depth agreeing at
    // one. Passes under V-4, V-7, V-8, and V-9 by
    // design.
    let dir = temp_dir("spells-adjudicated");
    let series = series_json(
        "the adjudicated spell saga",
        vec![ALPHA, BETA],
        3,
        vec![
            adjudicated_win("race one", ALPHA, BETA, BETA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", BETA, ALPHA),
            clear_win("race five", ALPHA, BETA),
        ],
    );
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        class_spell(&championship.series[0]),
        Some((SpellClass::Unbroken, 1))
    );
}

#[test]
fn the_longest_spell_arrives_late_after_two_single_standings() {
    // W-19. The longest spell arriving late at hand
    // scale. A hand-built series at a target of five
    // won by beta, alpha, beta, alpha, beta, beta,
    // alpha, alpha, alpha, two single behind standings
    // parted by level standings standing before a run
    // of three, asserted broken at the late spell of
    // three in a behind count of five through the
    // classification entry and its figure line. Fails
    // under V-1, the series classing unbroken. Fails
    // under V-2, its spell reading as its behind count
    // of five and classing unbroken. Fails under V-3,
    // its stated spell reading as its depth of two.
    // Fails under V-7, its figure line printing the
    // five in place of the three. Passes under V-5 by
    // the stated coincidence, its winner's longest run
    // of wins standing at three beside its spell.
    // Passes under V-6, its behind count reading as
    // its winner's front count of one at the entry
    // while its class and its stated spell stand.
    // Passes under V-4, V-8, and V-9 by design.
    let dir = temp_dir("spells-late");
    let series = series_json(
        "the late spell saga",
        vec![ALPHA, BETA],
        5,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", BETA, ALPHA),
            clear_win("race six", BETA, ALPHA),
            clear_win("race seven", ALPHA, BETA),
            clear_win("race eight", ALPHA, BETA),
            clear_win("race nine", ALPHA, BETA),
        ],
    );
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        class_spell(&championship.series[0]),
        Some((SpellClass::Broken, 3))
    );
    let lines = spells_report(&championship);
    assert!(spell_figures(&lines).contains(&3));
}

#[test]
fn a_collection_holding_no_series_renders_the_empty_form_at_the_library_surface() {
    // W-20. The empty form, pinned from birth. No
    // championship file carries an empty collection,
    // because the loader of DH-13 refuses a series
    // list holding no names, so this form stands at
    // the library surface alone, over a collection
    // built by hand. The pin fails under no mutation
    // of the nine, the empty rendering being the two
    // lines DH-51 states and the mutation set carrying
    // no variant under which it changes.
    let championship = Championship { series: vec![] };
    let lines = spells_report(&championship);
    assert_eq!(
        lines,
        vec![
            "spells",
            "spells, 0 series, 0 unbroken, 0 broken, 0 never behind, 0 no winner"
        ]
    );
}
