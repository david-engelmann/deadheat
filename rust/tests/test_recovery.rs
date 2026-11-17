//! Pins for DH-58 and DH-59 in the Round 30 section of
//! CONTRACT.md.
//!
//! The discrimination record of this file stands pin by
//! pin, and no blanket claim stands over it. The recovery
//! reading, its library function, and its command do
//! not stand on the pre round tree, so every pin in
//! this file that names the recovery reading is absent
//! there at compile time. That absence is recorded
//! once here and is not a discrimination claim. The
//! discrimination each pin carries is against the
//! stated mutation of DH-58 beside it. Ten mutations
//! are named for the whole file. X-1, the class names
//! exchanged in the cut, a series classing delayed
//! where its answer stands at its depth plus one and
//! immediate where its answer stands above it. X-2,
//! the place set at the last attainment, the count of
//! decided races standing up to and including the
//! last decided race after which the deficit stood
//! at the depth, and the answer read from that race
//! in place of the first attainment. X-3, the answer
//! counted in record counts, the count of the answer
//! race in the record less the count of the
//! attainment race in the record, in place of the
//! count of decided races standing after the
//! attainment. X-4, the decided record rebuilt inside
//! the module from the winner decision read over
//! every race of the record, so races recorded after
//! the decision count in the depth and the answer.
//! X-5, the answer read to the first level standing,
//! the shipped answer less one. X-6, the cut read
//! against the depth itself, a series classing
//! immediate where its answer stands at or below its
//! depth, a cut no series bearing a depth can meet,
//! and delayed where it stands above it. X-7, the
//! figure line printing the answer in place of the
//! depth and the depth in place of the answer, the
//! classes standing true. X-8, a series holding no
//! winner classed never behind at an answer of
//! nought and a depth of nought at the classification
//! entry in place of standing in no class. X-9, the
//! section standing for the immediate series in place
//! of the delayed. X-10, the decided races of a
//! section renumbered as their order among the
//! decided races, the first decided race standing as
//! race 1 and each later decided race standing one
//! above the last, in place of the count of the race
//! in the record the walk leaves.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{load_championship_file, Championship};
use deadheat::json::JsonValue;
use deadheat::peak::{peak_class, PeakClass};
use deadheat::place::{place_class, PlaceClass};
use deadheat::recovery::{recovery_class, recovery_report, RecoveryClass};
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

fn recovery_for(dir: &Path, files: Vec<(&str, JsonValue)>, names: Vec<&str>) -> Vec<String> {
    recovery_report(&load(dir, files, names))
}

fn load_example(name: &str) -> Championship {
    let (championship, defects) = load_championship_file(&examples().join(name));
    assert_eq!(defects, Vec::<String>::new());
    championship.expect("a championship")
}

fn golden(name: &str) -> String {
    std::fs::read_to_string(examples().join(name)).expect("a golden")
}

fn recovery_figures(lines: &[String]) -> Vec<(u64, u64)> {
    lines
        .iter()
        .filter_map(|line| {
            let rest = line.strip_prefix("deepest deficit ")?;
            let (depth, rest) = rest.split_once(", answer ")?;
            let answer = rest.split(' ').next()?;
            Some((depth.parse::<u64>().ok()?, answer.parse::<u64>().ok()?))
        })
        .collect()
}

fn immediate_saga() -> JsonValue {
    series_json(
        "the immediate recovery saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", BETA, ALPHA),
        ],
    )
}

fn delayed_saga() -> JsonValue {
    series_json(
        "the delayed recovery saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
        ],
    )
}

fn open_saga() -> JsonValue {
    series_json(
        "the open recovery saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
        ],
    )
}

#[test]
fn the_standing_championship_stands_never_behind_in_every_series() {
    // RC-1. The standing championship golden, no
    // section standing, closing "recovery, 2 series,
    // 0 immediate, 0 delayed, 2 never behind, 0 no
    // winner", the pilot and the panel each standing
    // never behind, neither winner having stood
    // behind over its decided record. Passes under
    // every variant by design, no series of the two
    // bearing a depth for a variant to move, no
    // series holding no winner for X-8 to class, and
    // no section or figure line standing for X-7,
    // X-9, or X-10 to move, a characterization of the
    // shipped reading on the shipped record.
    let championship = load_example("championship.json");
    let lines = recovery_report(&championship);
    assert_eq!(
        lines,
        vec![
            "recovery",
            "recovery, 2 series, 0 immediate, 0 delayed, 2 never behind, 0 no winner"
        ]
    );
    assert_eq!(lines.join("\n") + "\n", golden("championship-recovery.txt"));
}

#[test]
fn the_constructed_example_carries_the_delayed_series_alone() {
    // RC-2. The constructed recovery golden, the
    // first series standing in section form at an
    // answer of five in a depth of two, its winner
    // stint-1 standing delayed, the second series
    // standing immediate at an answer of four in a
    // depth of three and taking no section, closing
    // "recovery, 2 series, 1 immediate, 1 delayed, 0
    // never behind, 0 no winner". Fails under X-1,
    // the two series exchanging classes and the
    // section moving to the second series. Fails
    // under X-2, the first series' depth standing
    // attained at its second and fourth decided
    // races and its answer reading three from the
    // last attainment. Fails under X-3, the first
    // series' attainment standing behind a race
    // holding no winner inside its counted record,
    // so its answer reads six by the counts in the
    // record. Fails under X-5, the two answers
    // reading four and three. Fails under X-6, the
    // second series standing delayed at the depth
    // cut, its answer of four standing above its
    // depth of three, and the closing reading no
    // immediate and two delayed. Fails under X-7,
    // the figure line printing the five in place of
    // the two and the two in place of the five.
    // Fails under X-9, the immediate second series
    // gaining the section as the delayed first
    // loses it. Fails under X-10, the section's
    // races renumbering as their decided order, the
    // race holding no winner standing inside the
    // counted record before them. Passes under X-4
    // by design, neither series recording past its
    // decision. Passes under X-8, no series holding
    // no winner.
    let championship = load_example("constructed-recovery.json");
    let lines = recovery_report(&championship);
    assert_eq!(
        lines,
        vec![
            "recovery",
            "series 1, the constructed recovery drill one",
            "avocet-1 against stint-1, first to 4 wins",
            "race 1, recovery drill one race one, avocet-1 wins",
            "race 2, recovery drill one race two, avocet-1 wins",
            "race 4, recovery drill one race four, stint-1 wins",
            "race 5, recovery drill one race five, avocet-1 wins",
            "race 6, recovery drill one race six, stint-1 wins",
            "race 7, recovery drill one race seven, stint-1 wins",
            "race 8, recovery drill one race eight, stint-1 wins",
            "deepest deficit 2, answer 5 over the decided record",
            "series winner, stint-1, avocet-1 3 race wins, stint-1 4 race wins, 1 no results",
            "recovery, 2 series, 1 immediate, 1 delayed, 0 never behind, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-recovery-recovery.txt")
    );
}

#[test]
fn the_survey_championship_matches_its_recovery_golden() {
    // RC-3. The survey championship golden, no
    // section standing, its four depth bearing
    // series standing immediate and its other two
    // standing never behind, closing "recovery, 6
    // series, 4 immediate, 0 delayed, 2 never
    // behind, 0 no winner". Fails under X-1, the
    // immediate series classing delayed and the
    // closing reading no immediate and four delayed.
    // Fails under X-3, an answer counted in record
    // counts over a race holding no winner standing
    // inside a counted record. Fails under X-4, a
    // series reading its depth and answer over the
    // record its races after the decision extend.
    // Fails under X-5, the answers reading one less
    // and the series at the floor classing delayed.
    // Fails under X-6, the immediate series standing
    // delayed at the depth cut, their answers
    // standing above their depths. Fails under X-9,
    // the four immediate series gaining sections.
    // Passes under X-2, the answer of each depth
    // bearing series reading the same from its last
    // attainment as from its first. Passes under
    // X-7 and X-10, no section standing in the true
    // report for either to move. Passes under X-8,
    // no series of the six holding no winner.
    let championship = load_example("survey-championship.json");
    let lines = recovery_report(&championship);
    assert_eq!(recovery_figures(&lines), Vec::<(u64, u64)>::new());
    assert_eq!(
        lines.last().map(String::as_str),
        Some("recovery, 6 series, 4 immediate, 0 delayed, 2 never behind, 0 no winner")
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("survey-championship-recovery.txt")
    );
}

#[test]
fn the_classification_reads_the_answer_against_the_depth_plus_one() {
    // RC-4. The classes at championship scale at
    // hand scale, asserted at the classification
    // entry alone. A hand-built championship of
    // three series, the first won by alpha, beta,
    // beta under a target of two standing immediate
    // at an answer of two in a depth of one, the
    // second won by beta, alpha, beta, alpha, alpha
    // under a target of three standing delayed at
    // an answer of four in a depth of one, and the
    // third standing at two race wins each under a
    // target of three with its target unreached and
    // holding no winner. Fails under X-1, the first
    // two series exchanging classes. Fails under
    // X-2, the second series' depth standing
    // attained at its first and third decided races
    // and its answer reading two from the last
    // attainment, the series classing immediate.
    // Fails under X-5, the answers reading one and
    // three, the first series classing delayed
    // below its floor. Fails under X-6, the first
    // series standing delayed at the depth cut, its
    // answer of two standing above its depth of
    // one. Fails under X-8 at the third series
    // alone, the series gaining the never behind
    // class at the entry in place of no class.
    // Passes under X-3, no race of any series
    // holding no winner, so the counts in the
    // record and the counts of decided races agree
    // at each answer. Passes under X-4, no series
    // recording past its decision. Passes under
    // X-7, X-9, and X-10 by design, the pin seating
    // at the entry with no report line asserted.
    let dir = temp_dir("recovery-classification");
    let championship = load(
        &dir,
        vec![
            ("immediate.json", immediate_saga()),
            ("delayed.json", delayed_saga()),
            ("open.json", open_saga()),
        ],
        vec!["immediate.json", "delayed.json", "open.json"],
    );
    assert_eq!(
        recovery_class(&championship.series[0]),
        Some((RecoveryClass::Immediate, 2, 1))
    );
    assert_eq!(
        recovery_class(&championship.series[1]),
        Some((RecoveryClass::Delayed, 4, 1))
    );
    assert_eq!(recovery_class(&championship.series[2]), None);
}

#[test]
fn the_immediate_series_alone_prints_no_section() {
    // RC-5. The least depth report pin. The
    // immediate depth one series of RC-4 standing
    // alone, its report of two lines, closing
    // "recovery, 1 series, 1 immediate, 0 delayed,
    // 0 never behind, 0 no winner". Fails under
    // X-1, the series classing delayed and gaining
    // a section. Fails under X-5, its answer reading
    // one and the series classing delayed below its
    // floor. Fails under X-6, its answer of two
    // standing above its depth of one at the depth
    // cut and the series classing delayed. Fails
    // under X-9, the series gaining a section as
    // the class the sections stand for. Passes
    // under X-2, its depth standing attained at
    // its first decided race alone. Passes under
    // X-3, no race holding no winner in its record.
    // Passes under X-4, nothing standing recorded
    // past its decision. Passes under X-7 and X-10,
    // no figure line and no section standing in the
    // true report. Passes under X-8, the series
    // holding a winner.
    let dir = temp_dir("recovery-immediate-report");
    let lines = recovery_for(
        &dir,
        vec![("immediate.json", immediate_saga())],
        vec!["immediate.json"],
    );
    assert_eq!(
        lines,
        vec![
            "recovery",
            "recovery, 1 series, 1 immediate, 0 delayed, 0 never behind, 0 no winner"
        ]
    );
}

#[test]
fn the_delayed_series_alone_prints_its_section() {
    // RC-6. The delayed report pin. The delayed
    // depth one series of RC-4 standing alone with
    // its section whole, figure "deepest deficit 1,
    // answer 4 over the decided record", closing
    // "recovery, 1 series, 0 immediate, 1 delayed,
    // 0 never behind, 0 no winner". Fails under X-1,
    // the series classing immediate and its section
    // standing gone. Fails under X-2, its depth
    // standing attained at its first and third
    // decided races and its answer reading two from
    // the last attainment, the series classing
    // immediate and its section standing gone.
    // Fails under X-5, its answer reading three in
    // the figure line. Fails under X-7, the figure
    // line printing the four in place of the one
    // and the one in place of the four. Fails under
    // X-9, its section standing gone, the sections
    // standing for the immediate class in its
    // place. Passes under X-3, no race holding no
    // winner in its record, so the counts in the
    // record and the counts of decided races agree
    // at its answer. Passes under X-4, nothing
    // standing recorded past its decision. Passes
    // under X-6, its answer of four standing above
    // its depth of one at either cut, its class and
    // figures unmoved. Passes under X-8, the series
    // holding a winner. Passes under X-10, its
    // races standing numbered by the record as by
    // their decided order, no race holding no
    // winner among them.
    let dir = temp_dir("recovery-delayed-report");
    let lines = recovery_for(
        &dir,
        vec![("delayed.json", delayed_saga())],
        vec!["delayed.json"],
    );
    assert_eq!(
        lines,
        vec![
            "recovery",
            "series 1, the delayed recovery saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, beta-1 wins",
            "race 2, race two, alpha-1 wins",
            "race 3, race three, beta-1 wins",
            "race 4, race four, alpha-1 wins",
            "race 5, race five, alpha-1 wins",
            "deepest deficit 1, answer 4 over the decided record",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 2 race wins, 0 no results",
            "recovery, 1 series, 0 immediate, 1 delayed, 0 never behind, 0 no winner",
        ]
    );
}

#[test]
fn two_series_at_one_depth_part_on_the_answer() {
    // RC-7. Two series at a depth of two parting on
    // the answer, asserted at the entry. The first
    // built by hand, won by alpha, alpha, beta,
    // beta, beta under a target of three, its
    // winner beta standing immediate at an answer
    // of three, the second the constructed recovery
    // drill one loaded from the examples, standing
    // delayed at an answer of five. Fails under
    // X-1, the two series exchanging classes. Fails
    // under X-2 at the drill one, its depth standing
    // attained at its second and fourth decided
    // races and its answer reading three from the
    // last attainment. Fails under X-3 at the drill
    // one, its answer reading six by the counts in
    // the record over the race holding no winner
    // inside its counted record. Fails under X-5,
    // the answers reading two and four, the hand
    // series classing delayed below its floor.
    // Fails under X-6 at the hand series, its answer
    // of three standing above its depth of two at
    // the depth cut and the series classing delayed.
    // Passes under X-4, neither series recording
    // past its decision. Passes under X-7, X-9, and
    // X-10 by design, the pin seating at the entry.
    // Passes under X-8, both series holding winners.
    let dir = temp_dir("recovery-depth-two");
    let hand = load(
        &dir,
        vec![(
            "deep.json",
            series_json(
                "the deep immediate recovery saga",
                vec![ALPHA, BETA],
                3,
                vec![
                    clear_win("race one", ALPHA, BETA),
                    clear_win("race two", ALPHA, BETA),
                    clear_win("race three", BETA, ALPHA),
                    clear_win("race four", BETA, ALPHA),
                    clear_win("race five", BETA, ALPHA),
                ],
            ),
        )],
        vec!["deep.json"],
    );
    assert_eq!(
        recovery_class(&hand.series[0]),
        Some((RecoveryClass::Immediate, 3, 2))
    );
    let pair = load_example("constructed-recovery.json");
    assert_eq!(
        recovery_class(&pair.series[0]),
        Some((RecoveryClass::Delayed, 5, 2))
    );
}

#[test]
fn the_answer_names_the_first_ahead_standing_not_the_deciding_race() {
    // RC-8. A series whose winner stood ahead
    // first at its fifth decided race and won the
    // series only at its eighth, won by beta,
    // beta, alpha, alpha, alpha, beta, alpha, alpha
    // under a target of five, asserted at the
    // entry. Its depth of two stands attained at
    // its second decided race, and its answer
    // stands at three, the count from the
    // attainment to the first ahead standing, and
    // not at six, the count to the deciding race.
    // Fails under X-1, the series classing delayed.
    // Fails under X-5, its answer reading two and
    // the series classing delayed below its floor
    // of three. Fails under X-6, its answer of
    // three standing above its depth of two at the
    // depth cut and the series classing delayed.
    // Passes under X-2, its depth standing attained
    // at its second decided race alone. Passes
    // under X-3, no race holding no winner in its
    // record. Passes under X-4, its deciding race
    // standing last in its record. Passes under
    // X-7, X-9, and X-10 by design, the pin seating
    // at the entry. Passes under X-8, the series
    // holding a winner.
    let dir = temp_dir("recovery-first-ahead");
    let championship = load(
        &dir,
        vec![(
            "level.json",
            series_json(
                "the level recovery saga",
                vec![ALPHA, BETA],
                5,
                vec![
                    clear_win("race one", BETA, ALPHA),
                    clear_win("race two", BETA, ALPHA),
                    clear_win("race three", ALPHA, BETA),
                    clear_win("race four", ALPHA, BETA),
                    clear_win("race five", ALPHA, BETA),
                    clear_win("race six", BETA, ALPHA),
                    clear_win("race seven", ALPHA, BETA),
                    clear_win("race eight", ALPHA, BETA),
                ],
            ),
        )],
        vec!["level.json"],
    );
    assert_eq!(
        recovery_class(&championship.series[0]),
        Some((RecoveryClass::Immediate, 3, 2))
    );
}

#[test]
fn a_deeper_deficit_after_a_shallower_answer_restarts_the_answer() {
    // RC-9. A series whose winner stood ahead at
    // its third decided race against a depth of
    // one and then stood at a depth of two at its
    // sixth decided race, won by beta, alpha,
    // alpha, beta, beta, beta, alpha, beta, alpha,
    // alpha, alpha under a target of six, asserted
    // at the entry. Its answer stands at five,
    // read from the deeper attainment, and not at
    // two, the answer the shallower depth carried
    // before the deeper deficit stood. Fails under
    // X-1, the series classing immediate. Fails
    // under X-2, its depth standing attained at
    // its sixth and eighth decided races and its
    // answer reading three from the last
    // attainment. Fails under X-5, its answer
    // reading four. Passes under X-3, no race
    // holding no winner in its record. Passes
    // under X-4, its deciding race standing last
    // in its record. Passes under X-6, its answer
    // of five standing above its depth of two at
    // either cut, its class and figures unmoved.
    // Passes under X-7, X-9, and X-10 by design,
    // the pin seating at the entry. Passes under
    // X-8, the series holding a winner.
    let dir = temp_dir("recovery-deeper-deficit");
    let championship = load(
        &dir,
        vec![(
            "deeper.json",
            series_json(
                "the deeper recovery saga",
                vec![ALPHA, BETA],
                6,
                vec![
                    clear_win("race one", BETA, ALPHA),
                    clear_win("race two", ALPHA, BETA),
                    clear_win("race three", ALPHA, BETA),
                    clear_win("race four", BETA, ALPHA),
                    clear_win("race five", BETA, ALPHA),
                    clear_win("race six", BETA, ALPHA),
                    clear_win("race seven", ALPHA, BETA),
                    clear_win("race eight", BETA, ALPHA),
                    clear_win("race nine", ALPHA, BETA),
                    clear_win("race ten", ALPHA, BETA),
                    clear_win("race eleven", ALPHA, BETA),
                ],
            ),
        )],
        vec!["deeper.json"],
    );
    assert_eq!(
        recovery_class(&championship.series[0]),
        Some((RecoveryClass::Delayed, 5, 2))
    );
}

#[test]
fn the_three_extreme_entries_part_on_the_constructed_drill_two() {
    // RC-10. The three extreme entries on the
    // constructed recovery drill two, loaded from
    // the examples. The series stands early under
    // DH-54 at a place of three in a depth of
    // three, late under DH-56 at a place of seven
    // in a lead of one, and immediate here at an
    // answer of four in the same depth of three,
    // the three readings of the extremes parting on
    // one record. Fails under X-1, X-5, and X-6 at
    // its recovery entry alone, the class
    // exchanging under the first, its answer
    // reading three and classing delayed below its
    // floor of four under the second, and its
    // answer of four standing above its depth of
    // three at the depth cut and classing delayed
    // under the third, the place and peak entries
    // standing unmoved by every variant of this
    // file by design. Passes under X-2, its depth
    // of three standing attained at its third
    // decided race alone. Passes under X-3, no
    // race holding no winner in its record. Passes
    // under X-4, its deciding race standing last
    // in its record. Passes under X-7, X-9, and
    // X-10 by design, the pin seating at the
    // entries. Passes under X-8, the series holding
    // a winner.
    let pair = load_example("constructed-recovery.json");
    let series = &pair.series[1];
    assert_eq!(place_class(series), Some((PlaceClass::Early, 3, 3)));
    assert_eq!(peak_class(series), Some((PeakClass::Late, 7, 1)));
    assert_eq!(
        recovery_class(series),
        Some((RecoveryClass::Immediate, 4, 3))
    );
}

#[test]
fn an_adjudicated_race_counts_in_the_answer() {
    // RC-11. The delayed shape of RC-4 with its
    // third race standing adjudicated, won by
    // beta on the adjudication of a race neither
    // attempt finished, then alpha, alpha, under a
    // target of three, asserted at the entry. The
    // adjudicated race stands as the loser win
    // inside the answer that classes the series
    // delayed at an answer of four in a depth of
    // one, an adjudicated race counting in the
    // answer as any decided race counts. Fails
    // under X-1, the series classing immediate.
    // Fails under X-2, its depth standing attained
    // at its first and third decided races and its
    // answer reading two from the last attainment.
    // Fails under X-5, its answer reading three.
    // Passes under X-3, the adjudicated race
    // holding a winner, so no race of the series
    // holds no winner and the counts in the record
    // agree with the counts of decided races.
    // Passes under X-4, its deciding race standing
    // last in its record. Passes under X-6, its
    // answer of four standing above its depth of
    // one at either cut. Passes under X-7, X-9,
    // and X-10 by design, the pin seating at the
    // entry. Passes under X-8, the series holding
    // a winner.
    let dir = temp_dir("recovery-adjudicated");
    let championship = load(
        &dir,
        vec![(
            "adjudicated.json",
            series_json(
                "the adjudicated recovery saga",
                vec![ALPHA, BETA],
                3,
                vec![
                    clear_win("race one", BETA, ALPHA),
                    clear_win("race two", ALPHA, BETA),
                    adjudicated_win("race three", BETA, ALPHA, BETA),
                    clear_win("race four", ALPHA, BETA),
                    clear_win("race five", ALPHA, BETA),
                ],
            ),
        )],
        vec!["adjudicated.json"],
    );
    assert_eq!(
        recovery_class(&championship.series[0]),
        Some((RecoveryClass::Delayed, 4, 1))
    );
}

#[test]
fn the_races_after_the_decision_count_in_neither_figure() {
    // RC-12. A series decided at its third decided
    // race, won by alpha, beta, beta under a target
    // of two, with three further races recorded
    // after the decision, each won by alpha,
    // asserted at the entry. The series stands
    // immediate at an answer of two in a depth of
    // one, the races after the decision counting
    // in neither the depth nor the answer, the
    // counting walk stopping at the decision.
    // Fails under X-1, the series classing delayed.
    // Fails under X-4, the record read whole
    // standing at a depth of two first stood at
    // its last race, with no answer standing after
    // that attainment, the entry parting whole.
    // Fails under X-5, its answer reading one and
    // the series classing delayed. Fails under
    // X-6, its answer of two standing above its
    // depth of one at the depth cut. Passes under
    // X-2, its depth standing attained at its
    // first decided race alone over the decided
    // record. Passes under X-3, no race holding no
    // winner in its record. Passes under X-7, X-9,
    // and X-10 by design, the pin seating at the
    // entry. Passes under X-8, the series holding
    // a winner.
    let dir = temp_dir("recovery-after-decision");
    let championship = load(
        &dir,
        vec![(
            "settled.json",
            series_json(
                "the settled recovery saga",
                vec![ALPHA, BETA],
                2,
                vec![
                    clear_win("race one", ALPHA, BETA),
                    clear_win("race two", BETA, ALPHA),
                    clear_win("race three", BETA, ALPHA),
                    clear_win("race four", ALPHA, BETA),
                    clear_win("race five", ALPHA, BETA),
                    clear_win("race six", ALPHA, BETA),
                ],
            ),
        )],
        vec!["settled.json"],
    );
    assert_eq!(
        recovery_class(&championship.series[0]),
        Some((RecoveryClass::Immediate, 2, 1))
    );
}

#[test]
fn the_constructed_drill_one_standing_alone() {
    // RC-13. The constructed recovery drill one
    // standing alone, its whole report asserted,
    // its section's races numbered 1, 2, 4, 5, 6,
    // 7, 8 by the record, the race holding no
    // winner standing third in its record and its
    // number standing skipped. Fails under X-1,
    // the series classing immediate and its section
    // standing gone. Fails under X-2, its depth
    // standing attained at its second and fourth
    // decided races and its answer reading three
    // from the last attainment. Fails under X-3,
    // its answer reading six by the counts in the
    // record. Fails under X-5, its answer reading
    // four in the figure line. Fails under X-7, the
    // figure line printing the five in place of the
    // two and the two in place of the five. Fails
    // under X-9, its section standing gone. Fails
    // under X-10, its races renumbering as their
    // decided order, standing 1 through 7 with no
    // number skipped. Passes under X-4, nothing
    // standing recorded past its decision. Passes
    // under X-6, its answer of five standing above
    // its depth of two at either cut, its class and
    // figures unmoved. Passes under X-8, the series
    // holding a winner.
    let pair = load_example("constructed-recovery.json");
    let alone = Championship {
        series: vec![pair.series[0].clone()],
    };
    let lines = recovery_report(&alone);
    assert_eq!(
        lines,
        vec![
            "recovery",
            "series 1, the constructed recovery drill one",
            "avocet-1 against stint-1, first to 4 wins",
            "race 1, recovery drill one race one, avocet-1 wins",
            "race 2, recovery drill one race two, avocet-1 wins",
            "race 4, recovery drill one race four, stint-1 wins",
            "race 5, recovery drill one race five, avocet-1 wins",
            "race 6, recovery drill one race six, stint-1 wins",
            "race 7, recovery drill one race seven, stint-1 wins",
            "race 8, recovery drill one race eight, stint-1 wins",
            "deepest deficit 2, answer 5 over the decided record",
            "series winner, stint-1, avocet-1 3 race wins, stint-1 4 race wins, 1 no results",
            "recovery, 1 series, 0 immediate, 1 delayed, 0 never behind, 0 no winner",
        ]
    );
}

#[test]
fn a_target_of_one_stands_never_behind() {
    // RC-14. A series under a target of one, its
    // only decided race the deciding race, won by
    // alpha, asserted at the entry and in its
    // report. Its winner never stood behind, its
    // depth stands at nought, and it classes never
    // behind at an answer of nought, as every
    // decided series under that target stands.
    // Passes under every variant by design, no
    // depth standing for a variant to move and no
    // answer standing for X-5 to lessen, a
    // characterization of the shipped reading at
    // the least target.
    let dir = temp_dir("recovery-target-one");
    let championship = load(
        &dir,
        vec![(
            "single.json",
            series_json(
                "the single race recovery saga",
                vec![ALPHA, BETA],
                1,
                vec![clear_win("race one", ALPHA, BETA)],
            ),
        )],
        vec!["single.json"],
    );
    assert_eq!(
        recovery_class(&championship.series[0]),
        Some((RecoveryClass::NeverBehind, 0, 0))
    );
    let lines = recovery_report(&championship);
    assert_eq!(
        lines,
        vec![
            "recovery",
            "recovery, 1 series, 0 immediate, 0 delayed, 1 never behind, 0 no winner"
        ]
    );
}

#[test]
fn the_league_championship_matches_its_recovery_golden() {
    // RC-15. The league championship golden, one
    // section standing for bolt and ember at an
    // answer of five in a depth of two, its other
    // nine series taking no section, four standing
    // immediate and five never behind, closing
    // "recovery, 10 series, 4 immediate, 1 delayed,
    // 5 never behind, 0 no winner". Fails under
    // X-1, the immediate and delayed series
    // exchanging classes and the section moving.
    // Fails under X-3, an answer counted in record
    // counts over a race holding no winner standing
    // inside a counted record. Fails under X-4, a
    // series reading its depth and answer over the
    // record its races after the decision extend.
    // Fails under X-5, the answers reading one
    // less, a series at its floor classing delayed.
    // Fails under X-6, an immediate series standing
    // delayed at the depth cut. Fails under X-7,
    // the one figure line printing its figures
    // exchanged. Fails under X-9, the four immediate
    // series gaining sections as the delayed series
    // loses its own. Fails under X-10, the one
    // section's races renumbering over a race
    // holding no winner inside its counted record.
    // Passes under X-2, the answer of each depth
    // bearing series reading the same from its last
    // attainment as from its first. Passes under
    // X-8, no series of the ten holding no winner.
    let championship = load_example("league-championship.json");
    let lines = recovery_report(&championship);
    assert_eq!(recovery_figures(&lines), vec![(2, 5)]);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("recovery, 10 series, 4 immediate, 1 delayed, 5 never behind, 0 no winner")
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("league-championship-recovery.txt")
    );
}

#[test]
fn the_league_bolt_and_ember_series_stands_delayed_at_five_in_two() {
    // RC-16. The league's bolt and ember series at
    // the entry, the one delayed series of the
    // league championship, standing delayed at an
    // answer of five in a depth of two, its place
    // under DH-54 standing at two and its winner
    // standing ahead first at its seventh decided
    // race. Fails under X-1, the series classing
    // immediate. Fails under X-3, its answer reading
    // six by the counts in the record over the race
    // holding no winner standing inside its counted
    // record between the attainment and the answer.
    // Fails under X-5, its answer reading four.
    // Passes under X-2, its depth standing attained
    // at one standing, so the answer reads the same
    // from the last attainment as from the first.
    // Passes under X-4, its record read whole
    // standing at the same depth and the same
    // answer. Passes under X-6, its answer of five
    // standing above its depth of two at either
    // cut. Passes under X-7, X-9, and X-10 by
    // design, the pin seating at the entry. Passes
    // under X-8, the series holding a winner.
    let championship = load_example("league-championship.json");
    assert_eq!(
        recovery_class(&championship.series[6]),
        Some((RecoveryClass::Delayed, 5, 2))
    );
}

#[test]
fn the_entries_stand_across_the_classes() {
    // RC-17. The entries across the classes on the
    // shipped record. The apex and bolt series of
    // the survey championship standing immediate at
    // an answer of two in a depth of one, the bolt
    // and ember series of the league championship
    // standing delayed at an answer of five in a
    // depth of two, and the pilot series of the
    // standing championship standing never behind
    // at an answer of nought in a depth of nought.
    // Fails under X-1, the first two entries
    // exchanging classes. Fails under X-3, an
    // answer counted in record counts over a race
    // holding no winner standing inside a counted
    // record. Fails under X-4, an entry reading
    // over the record its series' races after the
    // decision extend. Fails under X-5, the answers
    // reading one less, the immediate entry
    // classing delayed below its floor. Fails under
    // X-6, the immediate entry standing delayed at
    // the depth cut, its answer of two standing
    // above its depth of one. Passes under X-2,
    // each answer reading the same from its last
    // attainment as from its first. Passes under
    // X-7, X-9, and X-10 by design, the pin seating
    // at the entries. Passes under X-8, each series
    // holding a winner.
    let survey = load_example("survey-championship.json");
    assert_eq!(
        recovery_class(&survey.series[0]),
        Some((RecoveryClass::Immediate, 2, 1))
    );
    let league = load_example("league-championship.json");
    assert_eq!(
        recovery_class(&league.series[6]),
        Some((RecoveryClass::Delayed, 5, 2))
    );
    let standing = load_example("championship.json");
    assert_eq!(
        recovery_class(&standing.series[0]),
        Some((RecoveryClass::NeverBehind, 0, 0))
    );
}

#[test]
fn the_empty_championship_prints_the_empty_form() {
    // RC-18. The empty form at the library surface.
    // A championship holding no series renders the
    // first line and the closing alone, "recovery,
    // 0 series, 0 immediate, 0 delayed, 0 never
    // behind, 0 no winner". Passes under every
    // variant by design, no series standing for a
    // variant to move, a characterization of the
    // shipped reading at the empty boundary.
    let championship = Championship { series: vec![] };
    let lines = recovery_report(&championship);
    assert_eq!(
        lines,
        vec![
            "recovery",
            "recovery, 0 series, 0 immediate, 0 delayed, 0 never behind, 0 no winner"
        ]
    );
}

#[test]
fn the_answer_reads_from_the_first_attainment() {
    // RC-19. The depth attained at several
    // standings, over the constructed pair's first
    // series in its decided shape, its record
    // standing as its seven decided races alone,
    // won by alpha, alpha, beta, alpha, beta, beta,
    // beta under a target of four with beta the
    // winner, asserted at the entry. The depth of
    // two stands attained at its second and fourth
    // decided races, and the answer stands at
    // five, read from the first attainment, the
    // later standing at the same depth moving
    // neither the place nor the answer. Fails
    // under X-1, the series classing immediate.
    // Fails under X-2, its answer reading three
    // from the last attainment. Fails under X-5,
    // its answer reading four. Passes under X-3,
    // its record standing in decided shape, so the
    // counts in the record and the counts of
    // decided races agree at its answer. Passes
    // under X-4, its deciding race standing last
    // in its record. Passes under X-6, its answer
    // of five standing above its depth of two at
    // either cut. Passes under X-7, X-9, and X-10
    // by design, the pin seating at the entry.
    // Passes under X-8, the series holding a
    // winner.
    let dir = temp_dir("recovery-first-attainment");
    let championship = load(
        &dir,
        vec![(
            "attained.json",
            series_json(
                "the twice attained recovery saga",
                vec![ALPHA, BETA],
                4,
                vec![
                    clear_win("race one", ALPHA, BETA),
                    clear_win("race two", ALPHA, BETA),
                    clear_win("race three", BETA, ALPHA),
                    clear_win("race four", ALPHA, BETA),
                    clear_win("race five", BETA, ALPHA),
                    clear_win("race six", BETA, ALPHA),
                    clear_win("race seven", BETA, ALPHA),
                ],
            ),
        )],
        vec!["attained.json"],
    );
    assert_eq!(
        recovery_class(&championship.series[0]),
        Some((RecoveryClass::Delayed, 5, 2))
    );
}

#[test]
fn two_series_part_at_the_least_answer() {
    // RC-20. Two series parting at the least
    // answer, both at a depth of one, asserted at
    // the entry as a pair. The immediate series of
    // RC-4 stands at an answer of two, the least
    // answer a depth of one allows, and the
    // delayed series of RC-4 stands at an answer
    // of four, the least answer above the floor,
    // its loser having won one decided race inside
    // the answer. Fails under X-1, the two series
    // exchanging classes. Fails under X-2 at the
    // delayed series, its depth standing attained
    // at its first and third decided races and its
    // answer reading two from the last attainment,
    // the series classing immediate. Fails under
    // X-5, the answers reading one and three, the
    // immediate series classing delayed below its
    // floor. Fails under X-6 at the immediate
    // series, its answer of two standing above its
    // depth of one at the depth cut. Passes under
    // X-3, no race of either series holding no
    // winner. Passes under X-4, neither series
    // recording past its decision. Passes under
    // X-7, X-9, and X-10 by design, the pin seating
    // at the entries. Passes under X-8, both series
    // holding winners.
    let dir = temp_dir("recovery-least-answer");
    let championship = load(
        &dir,
        vec![
            ("immediate.json", immediate_saga()),
            ("delayed.json", delayed_saga()),
        ],
        vec!["immediate.json", "delayed.json"],
    );
    assert_eq!(
        recovery_class(&championship.series[0]),
        Some((RecoveryClass::Immediate, 2, 1))
    );
    assert_eq!(
        recovery_class(&championship.series[1]),
        Some((RecoveryClass::Delayed, 4, 1))
    );
}
