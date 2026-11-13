//! Pins for DH-48 and DH-49 in the Round 23 section of
//! CONTRACT.md.
//!
//! The discrimination record of this file stands pin by
//! pin, and no blanket claim stands over it. The depths
//! reading, its library function, and its command do
//! not stand on the pre round tree, so every pin in
//! this file that names the depths reading is absent
//! there at compile time. That absence is recorded
//! once here and is not a discrimination claim. The
//! discrimination each pin carries is against the
//! stated mutation of DH-48 beside it. Nine mutations
//! are named for the whole file. N-1, the class cuts
//! swapped, deep read as depth above margin and
//! shallow read as depth below margin standing
//! reversed. N-2, the height read in place of the
//! depth, the larger of the winner's largest lead and
//! the depth standing as the figure. N-3, the margin read
//! as the wins of the loser in place of the wins of
//! the winner less the wins of the loser. N-4, the
//! walk does not stop, the decided record read whole
//! to the end of the series file. N-5, the sign of the
//! deficit reversed in the counting, the depth reading
//! nought over every record the winner ever stood
//! behind in. N-6, the behind count read in place of
//! the depth, the count of the decided races the
//! winner stood behind in standing as the figure. N-7,
//! the figure line printing the behind count in place
//! of the depth, the depth computed rightly and the
//! report reading the count. N-8, a series holding no
//! winner placed in the shallow class in place of
//! standing in no class. N-9, the section standing for
//! every decided series in place of the deep series
//! alone.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{load_championship_file, Championship};
use deadheat::crossings::crossings_report;
use deadheat::depths::depths_report;
use deadheat::json::JsonValue;
use deadheat::leads::lead_report;
use shared::*;

const ALPHA: &str = "alpha-1";
const BETA: &str = "beta-1";
const MERLIN: &str = "merlin-1";
const SAKER: &str = "saker-1";

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

fn depths_for(dir: &Path, files: Vec<(&str, JsonValue)>, names: Vec<&str>) -> Vec<String> {
    depths_report(&load(dir, files, names))
}

fn load_example(name: &str) -> Championship {
    let (championship, defects) = load_championship_file(&examples().join(name));
    assert_eq!(defects, Vec::<String>::new());
    championship.expect("a championship")
}

fn golden(name: &str) -> String {
    std::fs::read_to_string(examples().join(name)).expect("a golden")
}

fn depth_figures(lines: &[String]) -> Vec<u64> {
    lines
        .iter()
        .filter_map(|line| {
            line.strip_prefix("deepest deficit ")
                .and_then(|rest| rest.strip_suffix(" over the decided record"))
                .and_then(|figure| figure.parse::<u64>().ok())
        })
        .collect()
}

#[test]
fn the_standing_championship_stands_shallow_in_every_series() {
    // M-1. The standing championship golden, no section
    // standing, closing "depths, 2 series, 0 deep, 0
    // matched, 2 shallow, 0 no winner", the pilot
    // standing shallow at a depth of nought against a
    // margin of two and the panel standing shallow at
    // a depth of nought against a margin of three.
    // Fails under N-1, both series moving to deep.
    // Fails under N-2, the pilot's height of two
    // standing equal to its margin and the panel's
    // height of three standing equal to its margin, so
    // both stand matched. Fails under N-3, the margins
    // reading as the losers' wins of one and nought.
    // Fails under N-5, the winners' largest leads of
    // two and three standing at their margins as
    // matched. Fails under N-9, both shallow series
    // gaining sections. Passes under N-4 by the
    // recorded count, the two series' depths standing
    // at nought over their full records as over their
    // decided records. Passes under N-6, neither
    // winner ever standing behind, so the behind count
    // and the depth agree at nought. Passes under N-7
    // and N-8 by design, no section standing and no
    // series holding no winner.
    let championship = load_example("championship.json");
    let lines = depths_report(&championship);
    assert_eq!(
        lines,
        vec![
            "depths",
            "depths, 2 series, 0 deep, 0 matched, 2 shallow, 0 no winner"
        ]
    );
    assert_eq!(lines.join("\n") + "\n", golden("championship-depths.txt"));
}

#[test]
fn the_constructed_example_carries_the_first_series_alone() {
    // M-2. The constructed depths golden, the first
    // series standing in section form at the figure
    // two, its winner merlin-1 standing deep at a
    // depth of two against a margin of one, the second
    // series standing matched at a depth of one
    // against a margin of one and taking no section,
    // closing "depths, 2 series, 1 deep, 1 matched, 0
    // shallow, 0 no winner". Fails under N-1, the deep
    // and shallow cuts standing swapped on the first
    // series. Fails under N-3, the first series'
    // margin reading as its loser's three wins. Fails
    // under N-5, the first series' depth reading as
    // its winner's largest lead of one and standing
    // matched. Fails under N-6, the first series'
    // figure reading as its behind count of five.
    // Fails under N-7, its figure line printing the
    // five in place of the two. Fails under N-9, the
    // matched second series gaining a section. Passes
    // under N-2 by the stated coincidence, the first
    // series' height standing at its depth of two.
    // Passes under N-4 by design, neither series of
    // the pair carrying a race past its decision.
    // Passes under N-8 by design, no series holding no
    // winner.
    let championship = load_example("constructed-depths.json");
    let lines = depths_report(&championship);
    assert_eq!(
        lines,
        vec![
            "depths",
            "series 1, the constructed depths drill one",
            "merlin-1 against saker-1, first to 4 wins",
            "race 1, depths drill one race one, saker-1 wins",
            "race 2, depths drill one race two, saker-1 wins",
            "race 3, depths drill one race three, merlin-1 wins",
            "race 4, depths drill one race four, saker-1 wins",
            "race 5, depths drill one race five, merlin-1 wins",
            "race 6, depths drill one race six, merlin-1 wins",
            "race 7, depths drill one race seven, merlin-1 wins",
            "deepest deficit 2 over the decided record",
            "series winner, merlin-1, merlin-1 4 race wins, saker-1 3 race wins, 0 no results",
            "depths, 2 series, 1 deep, 1 matched, 0 shallow, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-depths-depths.txt")
    );
}

#[test]
fn the_survey_championship_matches_its_depths_golden() {
    // M-3. The survey championship golden, no section
    // standing, its six series standing shallow,
    // matched, matched, shallow, shallow, shallow at
    // depths of nought, one, one, nought, one, and one
    // against margins of two, one, one, three, three,
    // and two, closing "depths, 6 series, 0 deep, 2
    // matched, 4 shallow, 0 no winner". Fails under
    // N-1 and N-5 and N-9 on its classes and counts.
    // Fails under N-2, heights moving series between
    // the classes, the apex against crest series'
    // height of three standing above its margin of
    // two. Fails under N-3, margins reading as the
    // losers' wins. Fails under N-4, the full records
    // of the survey series carrying depths the walk
    // stops short of, the apex against bolt series
    // reading three over its full record against one
    // over its decided record, on the warm-up count.
    // Passes under N-6, the behind count and the depth
    // agreeing on every series of the six at the
    // figures its classes read. Passes under N-7 by
    // design, no section standing for its figure to
    // move in. Passes under N-8 by design, no series
    // holding no winner.
    let championship = load_example("survey-championship.json");
    let lines = depths_report(&championship);
    assert_eq!(depth_figures(&lines), Vec::<u64>::new());
    assert_eq!(
        lines,
        vec![
            "depths",
            "depths, 6 series, 0 deep, 2 matched, 4 shallow, 0 no winner"
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("survey-championship-depths.txt")
    );
}

#[test]
fn the_classification_reads_the_depth_against_the_margin() {
    // M-4. The classes at hand scale. A hand-built
    // series at a target of four won by beta, beta,
    // beta, alpha, alpha, alpha, alpha stands deep at
    // a depth of three against a margin of one, and a
    // second at the same target won by beta, beta,
    // alpha, alpha, alpha, alpha stands matched at a
    // depth of two against a margin of two, the two
    // asserted through the closing and the one section
    // standing, closing "depths, 2 series, 1 deep, 1
    // matched, 0 shallow, 0 no winner". Fails under
    // N-1, N-3, and N-5 on the classes. Fails under
    // N-6, the first series' figure reading as its
    // behind count of five and the second series
    // moving to deep at a behind count of three
    // against its margin of two, so two sections
    // stand. Fails under N-7, the first series' figure
    // line printing the five. Fails under N-9, the
    // matched series gaining its section. Passes under
    // N-2 by the stated coincidence, the two series'
    // heights standing at their depths of three and
    // two. Passes under N-4 and N-8 by design, no race
    // standing beyond either decision and no series
    // holding no winner.
    let dir = temp_dir("depths-classes");
    let deep = series_json(
        "the deep saga",
        vec![ALPHA, BETA],
        4,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
            clear_win("race six", ALPHA, BETA),
            clear_win("race seven", ALPHA, BETA),
        ],
    );
    let matched = series_json(
        "the matched saga",
        vec![ALPHA, BETA],
        4,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
            clear_win("race six", ALPHA, BETA),
        ],
    );
    let lines = depths_for(
        &dir,
        vec![("one.json", deep), ("two.json", matched)],
        vec!["one.json", "two.json"],
    );
    assert_eq!(depth_figures(&lines), vec![3]);
    assert_eq!(
        lines,
        vec![
            "depths",
            "series 1, the deep saga",
            "alpha-1 against beta-1, first to 4 wins",
            "race 1, race one, beta-1 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, beta-1 wins",
            "race 4, race four, alpha-1 wins",
            "race 5, race five, alpha-1 wins",
            "race 6, race six, alpha-1 wins",
            "race 7, race seven, alpha-1 wins",
            "deepest deficit 3 over the decided record",
            "series winner, alpha-1, alpha-1 4 race wins, beta-1 3 race wins, 0 no results",
            "depths, 2 series, 1 deep, 1 matched, 0 shallow, 0 no winner",
        ]
    );
}

#[test]
fn a_series_at_a_depth_equal_to_its_margin_stands_matched() {
    // M-5. The tie on the deciding figure at hand
    // scale. A hand-built series at a target of two
    // won by beta, alpha, alpha stands at a depth of
    // one against a margin of one and stands matched,
    // the boundary between matched and deep read from
    // the matched side, closing "depths, 1 series, 0
    // deep, 1 matched, 0 shallow, 0 no winner". Fails
    // under N-9 alone, its decided series gaining a
    // section. Passes under N-1 by design, the swapped
    // cuts meeting at the equality the series stands
    // on. Passes under N-2, its height standing at its
    // depth of one. Passes under N-3, its margin
    // reading as its loser's one win, the equality
    // standing. Passes under N-5, its winner's largest
    // lead standing at its depth of one. Passes under
    // N-6, its behind count standing at its depth of
    // one. Passes under N-4, N-7, and N-8 by design.
    let dir = temp_dir("depths-tied");
    let series = series_json(
        "the tied figure saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", ALPHA, BETA),
        ],
    );
    let lines = depths_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "depths",
            "depths, 1 series, 0 deep, 1 matched, 0 shallow, 0 no winner"
        ]
    );
}

#[test]
fn a_champion_who_never_stood_behind_stands_shallow_at_depth_nought() {
    // M-6. Depth nought at hand scale. A hand-built
    // series at a target of two won by alpha, alpha,
    // its winner winning the first race and never
    // standing behind, stands shallow at a depth of
    // nought against a margin of two, closing
    // "depths, 1 series, 0 deep, 0 matched, 1 shallow,
    // 0 no winner". Fails under N-1, the series moving
    // to deep. Fails under N-2, its height of two
    // standing equal to its margin as matched. Fails
    // under N-3, its margin reading as its loser's
    // nought wins, the depth standing equal to it as
    // matched. Fails under N-5, its winner's largest
    // lead of two standing at its margin as matched.
    // Fails under N-9, its decided series gaining a
    // section. Passes under N-4 and N-8 by design.
    // Passes under N-6, its behind count standing at
    // its depth of nought. Passes under N-7 by design,
    // no section standing.
    let dir = temp_dir("depths-wire");
    let series = series_json(
        "the wire saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", ALPHA, BETA),
        ],
    );
    let lines = depths_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "depths",
            "depths, 1 series, 0 deep, 0 matched, 1 shallow, 0 no winner"
        ]
    );
}

#[test]
fn two_series_at_one_depth_part_on_the_margin() {
    // M-7. The same depth at two margins. A hand-built
    // series at a target of four won by beta, beta,
    // alpha, beta, alpha, alpha, alpha stands deep at
    // a depth of two against a margin of one, and a
    // second at the same target won by beta, beta,
    // alpha, alpha, alpha, alpha stands matched at a
    // depth of two against a margin of two, the pair
    // asserted both ways, closing "depths, 2 series, 1
    // deep, 1 matched, 0 shallow, 0 no winner". Fails
    // under N-1, N-3, and N-5 on the classes. Fails
    // under N-6, the first series' figure reading as
    // its behind count of four and the second moving
    // to deep at a behind count of three. Fails under
    // N-7, its figure line printing the four. Fails
    // under N-9, the matched series gaining a section.
    // Passes under N-2 by the stated coincidence, the
    // two series' heights standing at their depths of
    // two. Passes under N-4 and N-8 by design.
    let dir = temp_dir("depths-twins");
    let deep = series_json(
        "the deep twin saga",
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
    let matched = series_json(
        "the matched twin saga",
        vec![ALPHA, BETA],
        4,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
            clear_win("race six", ALPHA, BETA),
        ],
    );
    let lines = depths_for(
        &dir,
        vec![("one.json", deep), ("two.json", matched)],
        vec!["one.json", "two.json"],
    );
    assert_eq!(depth_figures(&lines), vec![2]);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("depths, 2 series, 1 deep, 1 matched, 0 shallow, 0 no winner")
    );
}

#[test]
fn the_depth_stands_at_its_largest_not_its_last() {
    // M-8. The largest deficit, not the last, at hand
    // scale. A hand-built series at a target of four
    // won by beta, beta, alpha, alpha, beta, alpha,
    // alpha, its winner's deficit standing at two,
    // then nought at the level standing, then one
    // again before the win, stands deep at a depth of
    // two against a margin of one, its figure standing
    // at the largest standing and not at the one
    // standing before the decision. Fails under N-1
    // and N-3 and N-5 on its class. Fails under N-6,
    // its figure reading as its behind count of four.
    // Fails under N-7, its figure line printing the
    // four in place of the two. Passes under N-2 by
    // the stated coincidence, its height standing at
    // its depth of two. Passes under N-9 by design,
    // its series standing deep and taking its section
    // under either reading of the section rule. Passes
    // under N-4 and N-8 by design.
    let dir = temp_dir("depths-largest");
    let series = series_json(
        "the returning deficit saga",
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
    let lines = depths_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "depths",
            "series 1, the returning deficit saga",
            "alpha-1 against beta-1, first to 4 wins",
            "race 1, race one, beta-1 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, alpha-1 wins",
            "race 4, race four, alpha-1 wins",
            "race 5, race five, beta-1 wins",
            "race 6, race six, alpha-1 wins",
            "race 7, race seven, alpha-1 wins",
            "deepest deficit 2 over the decided record",
            "series winner, alpha-1, alpha-1 4 race wins, beta-1 3 race wins, 0 no results",
            "depths, 1 series, 1 deep, 0 matched, 0 shallow, 0 no winner",
        ]
    );
}

#[test]
fn the_depth_reached_early_stands_against_the_later_races() {
    // M-9. The depth reached partway at hand scale. A
    // hand-built series at a target of four won by
    // beta, beta, beta, with one no result race
    // standing fourth in the record, and then alpha,
    // alpha, alpha, alpha, its deficit standing at
    // its largest after three races and the record
    // running four more decided races, stands deep at
    // a depth of three against a margin of one, its
    // figure standing at the earlier figure and not
    // at the tally at the decision, and its section's
    // race lines skipping the no result race's number.
    // Fails under N-1 and N-3 and N-5 on its class.
    // Fails under N-6, its figure reading as its
    // behind count of five. Fails under N-7, its
    // figure line printing the five in place of the
    // three. Passes under N-2 by the stated
    // coincidence, its height standing at its depth of
    // three. Passes under N-9 by design, its series
    // standing deep. Passes under N-4 and N-8 by
    // design, no race standing beyond its decision.
    let dir = temp_dir("depths-early");
    let series = series_json(
        "the early depth saga",
        vec![ALPHA, BETA],
        4,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", BETA, ALPHA),
            no_result("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
            clear_win("race six", ALPHA, BETA),
            clear_win("race seven", ALPHA, BETA),
            clear_win("race eight", ALPHA, BETA),
        ],
    );
    let lines = depths_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "depths",
            "series 1, the early depth saga",
            "alpha-1 against beta-1, first to 4 wins",
            "race 1, race one, beta-1 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, beta-1 wins",
            "race 5, race five, alpha-1 wins",
            "race 6, race six, alpha-1 wins",
            "race 7, race seven, alpha-1 wins",
            "race 8, race eight, alpha-1 wins",
            "deepest deficit 3 over the decided record",
            "series winner, alpha-1, alpha-1 4 race wins, beta-1 3 race wins, 1 no results",
            "depths, 1 series, 1 deep, 0 matched, 0 shallow, 0 no winner",
        ]
    );
}

#[test]
fn a_series_wide_on_its_winners_side_stands_shallow() {
    // M-10. The depth parted from the height at hand
    // scale. A hand-built series at a target of eight
    // won by beta, beta, then alpha seven times, then
    // beta three times, then alpha, stands wide under
    // DH-42 at a height of five on its winner's side
    // and shallow here at a depth of two against a
    // margin of three, the two figures asserted apart
    // over the one loaded series. Fails under N-1, its
    // class moving to deep. Fails under N-2, its depth
    // reading as its height of five and standing deep.
    // Fails under N-5, its depth reading as its
    // winner's largest lead of five and standing deep.
    // Fails under N-6, its depth reading as its behind
    // count of three, standing equal to its margin as
    // matched. Fails under N-9, its shallow series
    // gaining a section. Passes under N-3 by the
    // stated coincidence, its margin reading as its
    // loser's five wins and the depth of two standing
    // below it as shallow either way. Passes under
    // N-7 by design, no figure standing in its report
    // for the count to print in, and its lead
    // assertions standing outside the mutations.
    // Passes under N-4 and N-8 by design.
    let dir = temp_dir("depths-height");
    let series = series_json(
        "the wide winner saga",
        vec![ALPHA, BETA],
        8,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
            clear_win("race six", ALPHA, BETA),
            clear_win("race seven", ALPHA, BETA),
            clear_win("race eight", ALPHA, BETA),
            clear_win("race nine", ALPHA, BETA),
            clear_win("race ten", BETA, ALPHA),
            clear_win("race eleven", BETA, ALPHA),
            clear_win("race twelve", BETA, ALPHA),
            clear_win("race thirteen", ALPHA, BETA),
        ],
    );
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    let lines = depths_report(&championship);
    assert_eq!(
        lines,
        vec![
            "depths",
            "depths, 1 series, 0 deep, 0 matched, 1 shallow, 0 no winner"
        ]
    );
    let lead_lines = lead_report(&championship);
    assert!(lead_lines.contains(&"largest lead 5 over the decided record".to_string()));
    assert_eq!(
        lead_lines.last().map(String::as_str),
        Some("lead, 1 series, 1 wide, 0 narrow, 0 no winner")
    );
}

#[test]
fn the_constructed_drill_one_stands_deep_and_short() {
    // M-11. The non implication witness of DH-48's
    // stated relations, in fixture form. The
    // constructed depths drill one, at a target of
    // four won by saker, saker, merlin, saker, merlin,
    // merlin, merlin, stands deep here at a depth of
    // two against a margin of one and short under
    // DH-44 over the same loaded series, the two
    // classes asserted over the one fixture. Fails
    // under N-1 and N-3 and N-5 on its depths class.
    // Fails under N-6, its figure reading as its
    // behind count of five. Fails under N-7, its
    // figure line printing the five in place of the
    // two. Passes under N-2 by the stated coincidence,
    // its height standing at its depth of two. Passes
    // under N-9 by design, its one series standing
    // deep and taking its section under either reading
    // of the section rule, and its crossings side
    // standing outside the mutations. Passes under
    // N-4 and N-8 by design
    let dir = temp_dir("depths-witness");
    let series = series_json(
        "the constructed depths drill one",
        vec![MERLIN, SAKER],
        4,
        vec![
            clear_win("depths drill one race one", SAKER, MERLIN),
            clear_win("depths drill one race two", SAKER, MERLIN),
            clear_win("depths drill one race three", MERLIN, SAKER),
            clear_win("depths drill one race four", SAKER, MERLIN),
            clear_win("depths drill one race five", MERLIN, SAKER),
            clear_win("depths drill one race six", MERLIN, SAKER),
            clear_win("depths drill one race seven", MERLIN, SAKER),
        ],
    );
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    let lines = depths_report(&championship);
    assert_eq!(depth_figures(&lines), vec![2]);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("depths, 1 series, 1 deep, 0 matched, 0 shallow, 0 no winner")
    );
    let crossing_lines = crossings_report(&championship);
    assert_eq!(
        crossing_lines.last().map(String::as_str),
        Some("crossings, 1 series, 0 through, 1 short, 0 no winner")
    );
}

#[test]
fn two_series_part_on_the_depth_at_one_margin() {
    // M-12. The tie in the depth at one margin. A
    // hand-built series at a target of two won by
    // alpha, beta, alpha stands shallow at a depth of
    // nought against a margin of one, and a second at
    // the same target won by beta, alpha, alpha stands
    // matched at a depth of one against the same
    // margin of one, the boundary of deepest deficit
    // against wire to wire, closing "depths, 2 series,
    // 0 deep, 1 matched, 1 shallow, 0 no winner".
    // Fails under N-1, the first series moving to
    // deep. Fails under N-2, the first series' height
    // of one standing equal to its margin as matched.
    // Fails under N-5, the first series' depth reading
    // as its winner's largest lead of one as matched.
    // Fails under N-9, both decided series gaining
    // sections. Passes under N-3 by the stated
    // coincidence, the margins reading as the losers'
    // one win apiece and both classes standing.
    // Passes under N-6, the behind counts standing at
    // the depths of nought and one. Passes under N-4,
    // N-7, and N-8 by design.
    let dir = temp_dir("depths-depth-tie");
    let shallow = series_json(
        "the level depth saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
        ],
    );
    let matched = series_json(
        "the single depth saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", ALPHA, BETA),
        ],
    );
    let lines = depths_for(
        &dir,
        vec![("one.json", shallow), ("two.json", matched)],
        vec!["one.json", "two.json"],
    );
    assert_eq!(
        lines,
        vec![
            "depths",
            "depths, 2 series, 0 deep, 1 matched, 1 shallow, 0 no winner"
        ]
    );
}

#[test]
fn the_depth_reaches_the_wins_target_less_one() {
    // M-13. The depth reaching figure at hand scale. A
    // hand-built series at a target of four whose
    // loser wins three before the winner wins four
    // stands deep at a depth of three, the largest the
    // target admits for a decided series, the wins
    // target less one, against a margin of one. Fails
    // under N-1 and N-3 and N-5 on its class. Fails
    // under N-6, its figure reading as its behind
    // count of five, its winner standing behind after
    // five of its decided races and not three as a
    // first reading of the record suggests. Fails
    // under N-7, its figure line printing the five.
    // Passes under N-2 by the stated coincidence, its
    // height standing at its depth of three. Passes
    // under N-9 by design, its series standing deep.
    // Passes under N-4 and N-8 by design.
    let dir = temp_dir("depths-reaching");
    let series = series_json(
        "the reaching depth saga",
        vec![ALPHA, BETA],
        4,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
            clear_win("race six", ALPHA, BETA),
            clear_win("race seven", ALPHA, BETA),
        ],
    );
    let lines = depths_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(depth_figures(&lines), vec![3]);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("depths, 1 series, 1 deep, 0 matched, 0 shallow, 0 no winner")
    );
}

#[test]
fn a_series_of_target_one_stands_shallow() {
    // M-14. The target one sentence of DH-48 at hand
    // scale. A hand-built series decided at its first
    // race under a wins target of one stands shallow
    // at a depth of nought against a margin of one,
    // its only decided race being its deciding race
    // and its winner having won it, closing "depths, 1
    // series, 0 deep, 0 matched, 1 shallow, 0 no
    // winner". Fails under N-1, its class moving to
    // deep. Fails under N-2, its height of one standing
    // equal to its margin as matched. Fails under N-3,
    // its margin reading as its loser's nought wins as
    // matched. Fails under N-5, its winner's largest
    // lead of one standing at its margin as matched.
    // Fails under N-9, its shallow decided series
    // gaining the section the design's weakest
    // mutation prices. Passes under N-6, its behind
    // count standing at its depth of nought. Passes
    // under N-4, N-7, and N-8 by design.
    let dir = temp_dir("depths-target-one");
    let series = series_json(
        "the single race saga",
        vec![ALPHA, BETA],
        1,
        vec![clear_win("race one", ALPHA, BETA)],
    );
    let lines = depths_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "depths",
            "depths, 1 series, 0 deep, 0 matched, 1 shallow, 0 no winner"
        ]
    );
}

#[test]
fn a_championship_holding_no_winner_stands_in_no_class() {
    // M-15. The no winner forms at hand scale, two in
    // number. A hand-built series carrying no decided
    // race, its two races standing without a winner,
    // and a second whose target stands unreached, its
    // two decided races standing one to each model
    // under a target of three, each stand in no class
    // and count in the closing's no winner figure,
    // closing "depths, 2 series, 0 deep, 0 matched, 0
    // shallow, 2 no winner". Fails under N-8 alone,
    // both series moving into the shallow class and
    // the closing reading two shallow against no no
    // winner. Passes under N-4 by execution, neither
    // series carrying a race past a decision, there
    // being no decision in either, where the design
    // had priced a failure on a sibling form this
    // fixture does not carry. Passes under the seven
    // remaining mutations by design, no class, figure,
    // or section standing in either series for them to
    // move.
    let dir = temp_dir("depths-no-winner");
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
    let lines = depths_for(
        &dir,
        vec![("one.json", silent), ("two.json", unreached)],
        vec!["one.json", "two.json"],
    );
    assert_eq!(
        lines,
        vec![
            "depths",
            "depths, 2 series, 0 deep, 0 matched, 0 shallow, 2 no winner"
        ]
    );
}

#[test]
fn a_series_at_a_depth_one_above_its_margin_stands_deep() {
    // M-16. The deciding figure read from the other
    // side at hand scale. A hand-built series at a
    // target of five won by beta, beta, beta, alpha,
    // alpha, alpha, alpha, alpha stands deep at a
    // depth of three against a margin of two, its
    // depth standing above its margin by exactly one,
    // the boundary between matched and deep. Fails
    // under N-1 and N-3 and N-5 on its class. Fails
    // under N-6, its figure reading as its behind
    // count of five. Fails under N-7, its figure line
    // printing the five in place of the three. Passes
    // under N-2 by the stated coincidence, its height
    // standing at its depth of three. Passes under
    // N-9 by design, its series standing deep. Passes
    // under N-4 and N-8 by design.
    let dir = temp_dir("depths-one-above");
    let series = series_json(
        "the one above saga",
        vec![ALPHA, BETA],
        5,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
            clear_win("race six", ALPHA, BETA),
            clear_win("race seven", ALPHA, BETA),
            clear_win("race eight", ALPHA, BETA),
        ],
    );
    let lines = depths_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(depth_figures(&lines), vec![3]);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("depths, 1 series, 1 deep, 0 matched, 0 shallow, 0 no winner")
    );
}

#[test]
fn the_league_championship_matches_its_depths_closing() {
    // M-17. The league closing line alone, its three
    // matched series being the census of the matched
    // class on a shipped collection, asserted as a
    // line and not through the whole golden, the pin
    // that prices the league's three matched series
    // against a defect that reads the whole collection.
    // No series of the league stands deep, so no
    // section stands and the closing is the report's
    // second line. Fails under N-1, its counts moving
    // between deep and shallow. Fails under N-2,
    // heights moving series between the classes. Fails
    // under N-4, the full records of the league series
    // carrying depths the walk stops short of, the
    // bolt against drift series reading one over its
    // full record against nought over its decided
    // record, on the warm-up count. Fails under N-9,
    // its decided series gaining sections and the
    // report standing longer than two lines. Passes
    // under N-3 by execution, the margins reading as
    // the losers' wins and every league class
    // standing, the apex against crest series' loser
    // holding two wins against its depth of one.
    // Fails under N-5, a league series gaining a
    // section at its winner's largest lead of three.
    // Fails under N-6, a league series gaining a
    // section at a behind count of four. Passes under
    // N-7 by design, no section standing for its
    // figure to move in. Passes under N-8 by design.
    let championship = load_example("league-championship.json");
    let lines = depths_report(&championship);
    assert_eq!(depth_figures(&lines), Vec::<u64>::new());
    assert_eq!(
        lines.last().map(String::as_str),
        Some("depths, 10 series, 0 deep, 3 matched, 7 shallow, 0 no winner")
    );
}

#[test]
fn an_adjudicated_race_counts_in_the_depth() {
    // M-18. The adjudication held entry at hand scale.
    // A hand-built series at a target of three whose
    // first race's attempts stand both red and named
    // by an adjudication entry for beta, the race
    // decided to the named attempt under DH-5, and
    // then won by beta, alpha, alpha, alpha, stands
    // deep at a depth of two against a margin of one,
    // the depth read on the record DH-5 decided, its
    // first section race line standing as the
    // adjudicated race. Fails under N-1 and N-3 and
    // N-5 on its class. Fails under N-6, its figure
    // reading as its behind count of three. Fails
    // under N-7, its figure line printing the three in
    // place of the two. Passes under N-2 by the stated
    // coincidence, its height standing at its depth of
    // two. Passes under N-9 by design, its series
    // standing deep. Passes under N-4 and N-8 by
    // design.
    let dir = temp_dir("depths-adjudicated");
    let series = series_json(
        "the adjudicated depth saga",
        vec![ALPHA, BETA],
        3,
        vec![
            adjudicated_win("race one", ALPHA, BETA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
        ],
    );
    let lines = depths_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "depths",
            "series 1, the adjudicated depth saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, beta-1 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, alpha-1 wins",
            "race 4, race four, alpha-1 wins",
            "race 5, race five, alpha-1 wins",
            "deepest deficit 2 over the decided record",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 2 race wins, 0 no results",
            "depths, 1 series, 1 deep, 0 matched, 0 shallow, 0 no winner",
        ]
    );
}

#[test]
fn the_deepest_deficit_stands_late_after_two_level_standings() {
    // M-19. The deepest deficit standing late at hand
    // scale. A hand-built series at a target of five
    // won by alpha, beta, alpha, beta, beta, beta,
    // alpha, alpha, alpha, its winner's deepest
    // deficit arriving only after the tallies stood
    // equal twice, stands deep at a depth of two
    // against a margin of one, the deficit standing at
    // two after the second level standing and the
    // winner standing behind after three of its
    // decided races. Fails under N-1 and N-3 and N-5
    // on its class. Fails under N-6, its figure
    // reading as its behind count of three. Fails
    // under N-7, its figure line printing the three in
    // place of the two, the count and the depth
    // parting on this record as they do not part on
    // M-5's. Passes under N-2 by the stated
    // coincidence, its height standing at its depth of
    // two. Passes under N-9 by design, its series
    // standing deep. Passes under N-4 and N-8 by
    // design.
    let dir = temp_dir("depths-late");
    let series = series_json(
        "the late depth saga",
        vec![ALPHA, BETA],
        5,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", BETA, ALPHA),
            clear_win("race five", BETA, ALPHA),
            clear_win("race six", BETA, ALPHA),
            clear_win("race seven", ALPHA, BETA),
            clear_win("race eight", ALPHA, BETA),
            clear_win("race nine", ALPHA, BETA),
        ],
    );
    let lines = depths_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "depths",
            "series 1, the late depth saga",
            "alpha-1 against beta-1, first to 5 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, alpha-1 wins",
            "race 4, race four, beta-1 wins",
            "race 5, race five, beta-1 wins",
            "race 6, race six, beta-1 wins",
            "race 7, race seven, alpha-1 wins",
            "race 8, race eight, alpha-1 wins",
            "race 9, race nine, alpha-1 wins",
            "deepest deficit 2 over the decided record",
            "series winner, alpha-1, alpha-1 5 race wins, beta-1 4 race wins, 0 no results",
            "depths, 1 series, 1 deep, 0 matched, 0 shallow, 0 no winner",
        ]
    );
}

#[test]
fn a_collection_holding_no_series_renders_the_empty_form_at_the_library_surface() {
    // M-20. The empty form, pinned from birth. No
    // championship file carries an empty collection,
    // because the loader of DH-13 refuses a series list
    // holding no names, so this form stands at the
    // library surface alone, over a collection built by
    // hand. The pin fails under no mutation of the
    // nine, the empty rendering being the two lines
    // DH-49 states and the mutation set carrying no
    // variant under which it changes.
    let championship = Championship { series: vec![] };
    let lines = depths_report(&championship);
    assert_eq!(
        lines,
        vec![
            "depths",
            "depths, 0 series, 0 deep, 0 matched, 0 shallow, 0 no winner"
        ]
    );
}
