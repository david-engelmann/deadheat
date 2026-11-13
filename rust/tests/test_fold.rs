//! Pins for FIX-DH-28 in the Round 24 section of
//! CONTRACT.md, the neutrality pins of the tally fold.
//!
//! The fold of DH-42, DH-44, DH-46, DH-48, DH-25, and
//! DH-29 into the one helper of the championship module
//! changes no behaviour, so the pins of this file assert
//! full report renderings over hand-built championships
//! whose figures stress the exact seat each module's
//! tally body stood in, and their discrimination stands
//! against the fold mutations, each a single restatement
//! inside the helper alone. F-1, the level count read
//! without its exclusion, the standing before the first
//! race counted as a level standing. F-2, the two
//! largest leads exchanged, side zero reading side one's
//! lead and side one reading side zero's. F-3, the front
//! counts read at level or in front, each side's front
//! count standing summed with the crossings count. F-4,
//! the largest leads and the front counts read as the
//! final tallies. F-5, the level count read as the count
//! of the decided races after which the tallies stood
//! apart. The postures stated in each pin's comment are
//! the executed postures, read on this tree in the
//! build's warm-up before this file was committed.

mod shared;

use std::path::{Path, PathBuf};

use num_bigint::BigInt;

use deadheat::championship::{load_championship_file, Championship};
use deadheat::crossings::crossings_report;
use deadheat::deadlocks::deadlock_report;
use deadheat::depths::depths_report;
use deadheat::json::JsonValue;
use deadheat::leads::lead_report;
use deadheat::pynum::Num;
use deadheat::pytext::PText;
use deadheat::races::{Attempt, Race, SuiteResult};
use deadheat::rallies::rally_report;
use deadheat::series::Series;
use deadheat::tenure::tenure_report;
use shared::*;

const ALPHA: &str = "alpha-1";
const BETA: &str = "beta-1";
const FORGED: &str = "FORGED";

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

/// A series over the two standing models whose decided
/// races stand won in the order the winners list states,
/// every race a clear win at a finish of 100 against 130,
/// outside the close call band of DH-5 at every race.
fn series_from_winners(task: &str, wins_target: i64, winners: &[&str]) -> JsonValue {
    let races = winners
        .iter()
        .enumerate()
        .map(|(index, winner)| {
            let loser = if *winner == ALPHA { BETA } else { ALPHA };
            clear_win(&format!("{} race {}", task, index + 1), winner, loser)
        })
        .collect::<Vec<_>>();
    series_json(task, vec![ALPHA, BETA], wins_target, races)
}

fn text(value: &str) -> PText {
    PText::from_text(value)
}

fn bad(value: &str) -> PText {
    PText::from_text(&format!("{value}\n{FORGED}"))
}

fn held_attempt(model: PText, environment: &str, result: SuiteResult, finish: f64) -> Attempt {
    Attempt {
        model,
        environment: text(environment),
        result,
        tokens: Some(BigInt::from(10)),
        patch: text("a diff"),
        finished_at: Some(Num::from_f64(finish)),
        adjustment: None,
    }
}

fn held_race(task: PText, winner: PText, loser: PText) -> Race {
    Race {
        task,
        attempts: vec![
            held_attempt(winner, "lane one", SuiteResult::Green, 100.0),
            held_attempt(loser, "lane two", SuiteResult::Red, 130.0),
        ],
        adjudication: None,
        panel: None,
    }
}

#[test]
fn the_leads_pair_stands_wide_and_narrow_over_the_fold() {
    // P-1. The leads pin. A hand-built championship of
    // two series. The first at a target of three won in
    // the order loser, loser, winner, winner, winner, its
    // height standing at two supplied by its loser
    // against a margin of one, standing wide with its
    // section at the figure 2. The second at a target of
    // three won in the order winner, loser, winner,
    // winner, its height standing at two supplied by its
    // winner against a margin of two, standing narrow
    // with no section. The closing reads lead, 2 series,
    // 1 wide, 1 narrow, 0 no winner. Fails under F-4,
    // the first series' height reading three on its
    // winner's final tally, its figure line moving, and
    // the second series' height reading three against
    // its margin of two, the series standing wide and
    // entering a section. Passes under F-2 as executed,
    // the height standing as the larger of the two leads
    // on either exchange, the design's sentence pricing
    // the exchange at the champion's seat standing
    // corrected here. Passes under F-1 and F-5, the
    // leads reading consuming no level count, and under
    // F-3, the front counts standing unconsumed by this
    // reading.
    let dir = temp_dir("fold_leads");
    let championship = load(
        &dir,
        vec![
            (
                "pair-one.json",
                series_from_winners("fold pair one", 3, &[BETA, BETA, ALPHA, ALPHA, ALPHA]),
            ),
            (
                "pair-two.json",
                series_from_winners("fold pair two", 3, &[ALPHA, BETA, ALPHA, ALPHA]),
            ),
        ],
        vec!["pair-one.json", "pair-two.json"],
    );
    assert_eq!(
        lead_report(&championship),
        vec![
            "lead",
            "series 1, fold pair one",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, fold pair one race 1, beta-1 wins",
            "race 2, fold pair one race 2, beta-1 wins",
            "race 3, fold pair one race 3, alpha-1 wins",
            "race 4, fold pair one race 4, alpha-1 wins",
            "race 5, fold pair one race 5, alpha-1 wins",
            "largest lead 2 over the decided record",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 2 race wins, 0 no results",
            "lead, 2 series, 1 wide, 1 narrow, 0 no winner",
        ]
    );
}

#[test]
fn the_crossings_and_deadlock_pair_stands_on_one_level_count() {
    // P-2. The crossings and deadlocks pin, the pair
    // standing in one pin because the identity of the
    // level count and the crossings count is itself
    // under test. A hand-built championship of two
    // series. The first at a target of three won in the
    // order winner, loser, loser, winner, winner, its
    // tallies standing level after its second and fourth
    // decided races, its crossings standing at two
    // against two changes of hand, standing through with
    // its section at the figure 2, and its deadlock
    // figure standing at two with its section at the
    // same figure. The second at a target of four won in
    // the order loser, winner, winner, winner, loser,
    // winner, the tallies standing level after its
    // second decided race alone, its crossings standing
    // at one against three changes of hand, standing
    // short, and its deadlock figure standing at one
    // with its section at the figure 1. The pin asserts
    // both reports whole over the one championship.
    // Fails under F-1, the standing before the first
    // race counted, the first series' crossings and
    // deadlock figures standing at three and the second
    // series' deadlock figure at two. Fails under F-5,
    // the first series' two counts standing at three
    // apart standings and the second's at five, its
    // crossings entering a section and its deadlock
    // figure moving. Passes under F-2 and F-4, neither
    // reading consuming a lead or a front count, and
    // under F-3 on the same ground.
    let dir = temp_dir("fold_pair");
    let championship = load(
        &dir,
        vec![
            (
                "pair-three.json",
                series_from_winners("fold pair three", 3, &[ALPHA, BETA, BETA, ALPHA, ALPHA]),
            ),
            (
                "pair-four.json",
                series_from_winners(
                    "fold pair four",
                    4,
                    &[BETA, ALPHA, ALPHA, ALPHA, BETA, ALPHA],
                ),
            ),
        ],
        vec!["pair-three.json", "pair-four.json"],
    );
    assert_eq!(
        crossings_report(&championship),
        vec![
            "crossings",
            "series 1, fold pair three",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, fold pair three race 1, alpha-1 wins",
            "race 2, fold pair three race 2, beta-1 wins",
            "race 3, fold pair three race 3, beta-1 wins",
            "race 4, fold pair three race 4, alpha-1 wins",
            "race 5, fold pair three race 5, alpha-1 wins",
            "crossings 2 over the decided record",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 2 race wins, 0 no results",
            "crossings, 2 series, 1 through, 1 short, 0 no winner",
        ]
    );
    assert_eq!(
        deadlock_report(&championship),
        vec![
            "deadlock",
            "series 1, fold pair three",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, fold pair three race 1, alpha-1 wins",
            "race 2, fold pair three race 2, beta-1 wins",
            "race 3, fold pair three race 3, beta-1 wins",
            "race 4, fold pair three race 4, alpha-1 wins",
            "race 5, fold pair three race 5, alpha-1 wins",
            "stood level after 2 of its decided races",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 2 race wins, 0 no results",
            "series 2, fold pair four",
            "alpha-1 against beta-1, first to 4 wins",
            "race 1, fold pair four race 1, beta-1 wins",
            "race 2, fold pair four race 2, alpha-1 wins",
            "race 3, fold pair four race 3, alpha-1 wins",
            "race 4, fold pair four race 4, alpha-1 wins",
            "race 5, fold pair four race 5, beta-1 wins",
            "race 6, fold pair four race 6, alpha-1 wins",
            "stood level after 1 of its decided races",
            "series winner, alpha-1, alpha-1 4 race wins, beta-1 2 race wins, 0 no results",
            "deadlock, 2 series, 2 deadlocked, 0 clear, 0 no winner",
        ]
    );
}

#[test]
fn the_rally_pair_stands_on_the_losers_largest_lead() {
    // P-3. The rallies pin, carrying the sentence that
    // the pre-decision deficit and the whole-record
    // depth stand equal. A hand-built championship of
    // two series. The first at a target of four won in
    // the order loser, loser, winner, loser, winner,
    // winner, winner, its winner standing behind after
    // its first five decided races at a depth of two and
    // level after its sixth, its deepest deficit before
    // the deciding race standing at two, standing a
    // rally with its section at the figure 2. The second
    // at a target of three won in the order winner,
    // winner, loser, winner, its winner never standing
    // behind, standing wire to wire with no section.
    // The closing reads rally, 2 series, 1 rally, 1 wire
    // to wire, 0 no winner. Fails under F-2, the second
    // series' deficit seat reading its winner's largest
    // lead of two and the series entering a section at
    // the figure 2 where none stands, and the first
    // series' figure reading one on its winner's largest
    // lead. Fails under F-4, the first series' deficit
    // reading the loser's final tally of three in the
    // lead seat, its figure standing at 3 against the
    // true 2, and the second series entering a section
    // at the figure 1 on the same seat. Passes under
    // F-1, F-3, and F-5, the rallies reading consuming
    // the lead counts alone. On the confusion the
    // sentence guards against, the pre-decision figure
    // read over the whole record, this pin carries no
    // fixture that parts the two readings and says so
    // plainly, the deciding race never carrying a
    // deficit on any record, so no fixture can part
    // them, and the discrimination on that seat stands
    // on the shipped rally goldens inside the
    // re-execution the build's warm-up carries.
    let dir = temp_dir("fold_rally");
    let championship = load(
        &dir,
        vec![
            (
                "pair-five.json",
                series_from_winners(
                    "fold pair five",
                    4,
                    &[BETA, BETA, ALPHA, BETA, ALPHA, ALPHA, ALPHA],
                ),
            ),
            (
                "pair-six.json",
                series_from_winners("fold pair six", 3, &[ALPHA, ALPHA, BETA, ALPHA]),
            ),
        ],
        vec!["pair-five.json", "pair-six.json"],
    );
    assert_eq!(
        rally_report(&championship),
        vec![
            "rally",
            "series 1, fold pair five",
            "alpha-1 against beta-1, first to 4 wins",
            "race 1, fold pair five race 1, beta-1 wins",
            "race 2, fold pair five race 2, beta-1 wins",
            "race 3, fold pair five race 3, alpha-1 wins",
            "race 4, fold pair five race 4, beta-1 wins",
            "race 5, fold pair five race 5, alpha-1 wins",
            "race 6, fold pair five race 6, alpha-1 wins",
            "race 7, fold pair five race 7, alpha-1 wins",
            "stood behind by 2 race wins at the deepest",
            "series winner, alpha-1, alpha-1 4 race wins, beta-1 3 race wins, 0 no results",
            "rally, 2 series, 1 rally, 1 wire to wire, 0 no winner",
        ]
    );
}

#[test]
fn the_tenure_pair_stands_behind_and_ahead_over_the_fold() {
    // P-4. The tenure pin. A hand-built championship of
    // two series. The first at a target of four won in
    // the order loser, loser, winner, loser, winner,
    // winner, winner, its winner's front count standing
    // at one against its loser's five, standing behind
    // with no section. The second at a target of three
    // won in the order winner, loser, winner, winner,
    // its winner's front count standing at three against
    // its loser's none, standing ahead with its section
    // at the figure 3. The closing reads tenure, 2
    // series, 1 ahead, 0 level, 1 behind, 0 no winner.
    // Fails under F-3, the second series' figure
    // standing at four on its front count summed with
    // its one level standing, the first series standing
    // behind on either sum. Fails under F-4, the front
    // seats reading the final tallies, the first series
    // standing ahead at four wins against three and
    // entering a section at the figure 4. Passes under
    // F-1, F-2, and F-5, the tenure reading consuming
    // the front counts alone, its level standings
    // entering only through F-3's sum.
    let dir = temp_dir("fold_tenure");
    let championship = load(
        &dir,
        vec![
            (
                "pair-seven.json",
                series_from_winners(
                    "fold pair seven",
                    4,
                    &[BETA, BETA, ALPHA, BETA, ALPHA, ALPHA, ALPHA],
                ),
            ),
            (
                "pair-eight.json",
                series_from_winners("fold pair eight", 3, &[ALPHA, BETA, ALPHA, ALPHA]),
            ),
        ],
        vec!["pair-seven.json", "pair-eight.json"],
    );
    assert_eq!(
        tenure_report(&championship),
        vec![
            "tenure",
            "series 2, fold pair eight",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, fold pair eight race 1, alpha-1 wins",
            "race 2, fold pair eight race 2, beta-1 wins",
            "race 3, fold pair eight race 3, alpha-1 wins",
            "race 4, fold pair eight race 4, alpha-1 wins",
            "front count 3 over the decided record",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 1 race wins, 0 no results",
            "tenure, 2 series, 1 ahead, 0 level, 1 behind, 0 no winner",
        ]
    );
}

#[test]
fn the_depths_report_fences_its_fields_over_the_fold() {
    // P-5. The depths pin, doubling the fencing seat.
    // A hand-built championship of one series whose
    // task, race tasks, and both model labels hold line
    // breaks in the family's fencing form, built by hand
    // at the library surface as the fencing pins build
    // theirs, at a target of four won in the order
    // loser, loser, loser, winner, winner, winner,
    // winner, standing deep at a depth of three against
    // a margin of one, its section standing with every
    // printed field fenced to the empty field through
    // the one helper of FIX-DH-16, as the tenure fencing
    // pin of test_fencing.rs asserts its own. Its
    // closing reads depths, 1 series, 1 deep, 0 matched,
    // 0 shallow, 0 no winner, and its figure line reads
    // deepest deficit 3 over the decided record. Fails
    // under F-2, the depth seat reading the winner's own
    // largest lead of one on the exchanged seat, the
    // depth standing at one against its margin of one
    // and the series standing matched, its section gone
    // from the report. Passes under F-4,
    // the depth seat reading the loser's largest lead
    // standing at three on this fixture under either
    // form, the final tallies parting only on the
    // winner's side, whose lead the depth does not read.
    // Passes under F-1, F-3, and F-5, the depths reading
    // consuming the lead counts alone.
    let first = bad("model");
    let second = bad("beta");
    let mut races = Vec::new();
    for (winner, loser) in [
        (second.clone(), first.clone()),
        (second.clone(), first.clone()),
        (second.clone(), first.clone()),
        (first.clone(), second.clone()),
        (first.clone(), second.clone()),
        (first.clone(), second.clone()),
        (first.clone(), second.clone()),
    ] {
        races.push(held_race(bad("race task"), winner, loser));
    }
    let championship = Championship {
        series: vec![Series {
            task: bad("series task"),
            models: vec![first, second],
            wins_target: BigInt::from(4),
            races,
        }],
    };
    let lines = depths_report(&championship);
    for line in &lines {
        assert!(!line.contains(FORGED), "{line}");
    }
    assert_eq!(
        lines,
        vec![
            "depths",
            "series 1, ",
            " against , first to 4 wins",
            "race 1, ,  wins",
            "race 2, ,  wins",
            "race 3, ,  wins",
            "race 4, ,  wins",
            "race 5, ,  wins",
            "race 6, ,  wins",
            "race 7, ,  wins",
            "deepest deficit 3 over the decided record",
            "series winner, ,  4 race wins,  3 race wins, 0 no results",
            "depths, 1 series, 1 deep, 0 matched, 0 shallow, 0 no winner",
        ]
    );
}

#[test]
fn no_series_stands_classed_without_a_winner_under_any_reading() {
    // P-6. The no winner and empty pin. A hand-built
    // championship of two series, one carrying no
    // decided race and one whose target stands
    // unreached, asserted in no class under all six
    // readings with the six closings standing at their
    // no winner counts, and the empty championship
    // asserted at its whole rendering under the leads
    // reading at the library surface, the two lines its
    // report states. The pin fails under no fold
    // mutation, the helper standing uncalled on a record
    // with no champion in the six modules as the tree
    // stands, the champion match standing before the
    // call in each, and its seat in the set is the
    // closing counts' guard, a fold that classed a no
    // winner series under any reading failing it through
    // the closings it asserts.
    let dir = temp_dir("fold_open");
    let idle = series_json(
        "fold pair nine",
        vec![ALPHA, BETA],
        3,
        vec![no_result("fold pair nine race 1", ALPHA, BETA)],
    );
    let open = series_from_winners("fold pair ten", 3, &[ALPHA, BETA]);
    let championship = load(
        &dir,
        vec![("pair-nine.json", idle), ("pair-ten.json", open)],
        vec!["pair-nine.json", "pair-ten.json"],
    );
    assert_eq!(
        lead_report(&championship),
        vec!["lead", "lead, 2 series, 0 wide, 0 narrow, 2 no winner"]
    );
    assert_eq!(
        crossings_report(&championship),
        vec![
            "crossings",
            "crossings, 2 series, 0 through, 0 short, 2 no winner"
        ]
    );
    assert_eq!(
        tenure_report(&championship),
        vec![
            "tenure",
            "tenure, 2 series, 0 ahead, 0 level, 0 behind, 2 no winner"
        ]
    );
    assert_eq!(
        rally_report(&championship),
        vec![
            "rally",
            "rally, 2 series, 0 rally, 0 wire to wire, 2 no winner"
        ]
    );
    assert_eq!(
        deadlock_report(&championship),
        vec![
            "deadlock",
            "deadlock, 2 series, 0 deadlocked, 0 clear, 2 no winner"
        ]
    );
    assert_eq!(
        depths_report(&championship),
        vec![
            "depths",
            "depths, 2 series, 0 deep, 0 matched, 0 shallow, 2 no winner"
        ]
    );
    let empty = Championship { series: vec![] };
    assert_eq!(
        lead_report(&empty),
        vec!["lead", "lead, 0 series, 0 wide, 0 narrow, 0 no winner"]
    );
}
