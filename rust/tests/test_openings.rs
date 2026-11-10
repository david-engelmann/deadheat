//! Pins for DH-38 and DH-39 in the Round 17 section of
//! CONTRACT.md.
//!
//! The discrimination record of this file stands pin by
//! pin, and no blanket claim stands over it. The opening
//! reading, its library function, and its command do not
//! stand on the pre round tree, so every pin in this file
//! that names the opening reading is absent there at
//! compile time. That absence is recorded once here and
//! is not a discrimination claim. The discrimination
//! each pin carries is against the stated mutation of
//! DH-38 beside it. Five mutations are named for the
//! whole file. M-A, the walk's early stop removed for
//! this reading, the opening run read over the head of
//! the full recorded record, the races beyond the
//! decision included where the head of the record
//! reaches them. M-B, a race holding no result breaking
//! the opening run, the run read over the counted races
//! in place of the decided record. M-C, the winner's
//! longest run, the streak statistic of DH-32, read in
//! place of the opening run, in the class and in the
//! section figure alike. M-D, the winner's closing run,
//! the closing statistic of DH-36, read in place of the
//! opening run. M-E, the class boundary moved, a series
//! standing in a run only where its opening run stands
//! at three or more, the figure line unchanged. Seven
//! pins, the constructed file under the streak, cluster,
//! rally, deadlock, distance, comeback, and closing
//! commands, pass against the pre round surface by
//! design, as their own comments state. K-1 states no
//! M-A posture, for the reason its comment states.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{championship_report, load_championship_file, Championship};
use deadheat::closings::closing_report;
use deadheat::clusters::cluster_report;
use deadheat::comebacks::comeback_report;
use deadheat::deadlocks::deadlock_report;
use deadheat::distances::distance_report;
use deadheat::json::JsonValue;
use deadheat::openings::opening_report;
use deadheat::rallies::rally_report;
use deadheat::streaks::streak_report;
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

fn opening_for(dir: &Path, files: Vec<(&str, JsonValue)>, names: Vec<&str>) -> Vec<String> {
    opening_report(&load(dir, files, names))
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
fn the_standing_championship_carries_both_sections() {
    // K-1. The standing championship golden, both
    // sections in championship order, the pilot at a
    // figure of two and the panel at three. Fails under
    // M-B, by both series' counted records: the pilot's
    // counted races run win, no result, no result, no
    // result, win, no result, loss, no result, win, so
    // read over the counted races its opening run
    // breaks at the no result standing second and reads
    // one, the pilot stands alone, and its section
    // vanishes; and the panel's counted races run win,
    // win, no result, no result, win, so its opening run
    // reads two against the golden's three, its figure
    // moving while its class stands. Fails under M-D,
    // the pilot's closing run standing at one, so the
    // pilot stands alone under that reading and its
    // section vanishes, the closing reading 1 in a run
    // and 1 alone, the panel's closing run standing at
    // three and unmoved. Fails under M-E, the pilot
    // standing at exactly two and losing its section,
    // the closing reading 1 in a run and 1 alone. Passes
    // under M-C by design: the pilot's longest run stands
    // at two, the same figure as its opening run, and the
    // panel's at three, so the streak statistic returns
    // the same classes and the same figures over this
    // championship; the M-C carriers stand at K-2, K-3,
    // and K-14. M-A is not stated for this pin: the
    // standing series' races beyond the decision stand
    // under the panel and adjudication rules, whose full
    // records this plan does not re-derive; M-A is
    // carried at hand-built scale by K-16.
    let championship = load_example("championship.json");
    let lines = opening_report(&championship);
    assert_eq!(
        lines,
        vec![
            "opening",
            "series 1, the recorded close call pilot",
            "prime-1 against shadow-1, first to 3 wins",
            "race 1, pin the cache key, prime-1 wins",
            "race 5, name the worker pool, prime-1 wins",
            "race 7, shrink the prompt budget, shadow-1 wins",
            "race 9, sort the seed list, prime-1 wins",
            "opening run of 2 consecutive decided race wins",
            "series winner, prime-1, prime-1 3 race wins, shadow-1 1 race wins, 5 no results",
            "series 2, the recorded close call panel",
            "prime-1 against shadow-1, first to 3 wins",
            "race 1, pin the cache key, prime-1 wins",
            "race 2, trim the import list, prime-1 wins",
            "race 5, name the worker pool, prime-1 wins",
            "opening run of 3 consecutive decided race wins",
            "series winner, prime-1, prime-1 3 race wins, shadow-1 0 race wins, 2 no results",
            "opening, 2 series, 2 in a run, 0 alone, 0 from behind, 0 no winner",
        ]
    );
    assert_eq!(lines.join("\n") + "\n", golden("championship-opening.txt"));
}

#[test]
fn the_survey_championship_matches_its_opening_golden() {
    // K-2. The survey championship golden, one section,
    // the apex-crest series at a figure of three, pinned
    // byte for byte against the golden the real command
    // produced. Fails under M-C, the apex-bolt series
    // gaining a section at its longest run of two, the
    // apex-drift series at two, the bolt-crest series at
    // three, the bolt-drift series at four, and the
    // crest-drift series at three, the closing reading 6
    // in a run, 0 alone, 0 from behind. Fails under M-D,
    // the apex-bolt series gaining a section at its
    // closing run of two, the apex-crest series losing
    // its own at a closing run of one, the apex-drift
    // series gaining at two, the bolt-crest series at
    // three, and the crest-drift series at three, while
    // the bolt-drift series moves from behind to alone at
    // its closing run of one, the closing reading 4 in a
    // run, 2 alone, 0 from behind. Passes under M-B by
    // design: no no result stands inside any survey
    // series' opening span, every survey series' opening
    // figure over its counted races equalling its figure
    // over its decided record; the M-B carriers stand at
    // K-1, K-12, K-13, and, at league scale, K-14 and the
    // E-3 expectation. Passes under M-E by design: no
    // survey series stands at exactly two, its figures
    // standing at three, one, one, one, none, and none;
    // the M-E carriers stand at K-1, K-3, K-15, and K-16.
    let championship = load_example("survey-championship.json");
    let lines = opening_report(&championship);
    assert!(lines.len() > 2, "the survey golden stands in section form");
    assert_eq!(
        lines.join("\n") + "\n",
        golden("survey-championship-opening.txt")
    );
}

#[test]
fn the_constructed_example_parts_from_behind_from_in_a_run() {
    // K-3. The round's headline discrimination. The first
    // constructed series stands in a run with an opening
    // run of two, its decided record running alpha-1,
    // alpha-1, beta-1, alpha-1, while standing alone
    // under DH-36 at a closing run of one, strung,
    // scattered, from the front, wire to wire, clear,
    // and in between, and the second stands from behind
    // with an opening run of none, its decided record
    // running gamma-1, delta-1, gamma-1, delta-1,
    // delta-1, so the opening golden carries the first
    // series alone. Fails under M-C, the second series
    // gaining a section at its longest run of two, the
    // proof series of DH-38's stated relations rendered
    // as a section, the closing reading 2 in a run and 0
    // from behind. Fails under M-D, the first series
    // losing its section at its closing run of one and
    // the second gaining its own at a closing run of
    // two, the section standing for the other series and
    // the closing reading 1 in a run, 1 alone, 0 from
    // behind. Fails under M-E, the first series standing
    // at exactly two and losing its section, the report
    // standing in the empty form and the closing reading
    // 0 in a run, 1 alone, 1 from behind. Passes under
    // M-A by design: neither constructed winner won every
    // decided race of its series, the first series'
    // record holding a win of beta-1 and the second's two
    // wins of gamma-1, so no head of a full record can
    // extend either opening run past its decision; the
    // M-A carrier stands at K-16. Passes under M-B by
    // design: the constructed records carry no no results
    // at all, as DH-39's examples block states, so no
    // counted head can break; the M-B carriers stand at
    // K-1, K-12, and K-13.
    let championship = load_example("constructed-opening.json");
    let lines = opening_report(&championship);
    assert_eq!(
        lines,
        vec![
            "opening",
            "series 1, the constructed opening drill one",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, opening drill one race one, alpha-1 wins",
            "race 2, opening drill one race two, alpha-1 wins",
            "race 3, opening drill one race three, beta-1 wins",
            "race 4, opening drill one race four, alpha-1 wins",
            "opening run of 2 consecutive decided race wins",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 1 race wins, 0 no results",
            "opening, 2 series, 1 in a run, 0 alone, 1 from behind, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-opening-opening.txt")
    );
}

#[test]
fn the_constructed_example_stands_strung_under_streak() {
    // K-4. The same file under the streak command, both
    // series standing strung at a figure of two. Passes
    // on the pre round tree by design, in the posture of
    // DH-37's goldens over standing commands, the streak
    // reading and its command standing built in Round
    // 14. The pin holds that the round's new fixture does
    // not move a standing reading and that the two
    // readings part over the same file, read against the
    // pin above. Its failure mode is the disagreement
    // itself: a streak reading of this fixture that parts
    // from DH-39's stated figures fails here.
    let championship = load_example("constructed-opening.json");
    let lines = streak_report(&championship);
    assert_eq!(
        lines,
        vec![
            "streak",
            "series 1, the constructed opening drill one",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, opening drill one race one, alpha-1 wins",
            "race 2, opening drill one race two, alpha-1 wins",
            "race 3, opening drill one race three, beta-1 wins",
            "race 4, opening drill one race four, alpha-1 wins",
            "longest run of 2 consecutive decided race wins",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 1 race wins, 0 no results",
            "series 2, the constructed opening drill two",
            "gamma-1 against delta-1, first to 3 wins",
            "race 1, opening drill two race one, gamma-1 wins",
            "race 2, opening drill two race two, delta-1 wins",
            "race 3, opening drill two race three, gamma-1 wins",
            "race 4, opening drill two race four, delta-1 wins",
            "race 5, opening drill two race five, delta-1 wins",
            "longest run of 2 consecutive decided race wins",
            "series winner, delta-1, gamma-1 2 race wins, delta-1 3 race wins, 0 no results",
            "streak, 2 series, 2 strung, 0 single, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-opening-streak.txt")
    );
}

#[test]
fn the_constructed_example_stands_scattered_under_cluster() {
    // K-5. The same file under the cluster command, the
    // empty form, neither loser's wins standing beside
    // each other in either decided record. Passes on the
    // pre round tree by design, in the posture of the pin
    // above, the cluster reading and its command standing
    // built in Round 15. Its failure mode is the
    // disagreement itself, read against DH-39's stated
    // cluster closing over this fixture.
    let championship = load_example("constructed-opening.json");
    let lines = cluster_report(&championship);
    assert_eq!(
        lines,
        vec![
            "cluster",
            "cluster, 2 series, 0 clustered, 2 scattered, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-opening-cluster.txt")
    );
}

#[test]
fn the_constructed_example_stands_a_rally_under_rally() {
    // K-6. The same file under the rally command, the
    // second series standing a rally in its section
    // alone, its winner having stood behind after its
    // first and third decided races, the witness DH-38
    // states for the from behind class. Passes on the pre
    // round tree by design, in the posture of the pins
    // above, the rally reading standing built in Round
    // 12. Its failure mode is the disagreement itself.
    let championship = load_example("constructed-opening.json");
    let lines = rally_report(&championship);
    assert_eq!(
        lines,
        vec![
            "rally",
            "series 2, the constructed opening drill two",
            "gamma-1 against delta-1, first to 3 wins",
            "race 1, opening drill two race one, gamma-1 wins",
            "race 2, opening drill two race two, delta-1 wins",
            "race 3, opening drill two race three, gamma-1 wins",
            "race 4, opening drill two race four, delta-1 wins",
            "race 5, opening drill two race five, delta-1 wins",
            "stood behind by 1 race wins at the deepest",
            "series winner, delta-1, gamma-1 2 race wins, delta-1 3 race wins, 0 no results",
            "rally, 2 series, 1 rally, 1 wire to wire, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-opening-rally.txt")
    );
}

#[test]
fn the_constructed_example_stands_deadlocked_under_deadlock() {
    // K-7. The same file under the deadlock command, the
    // second series standing deadlocked in its section
    // alone, its tallies having stood level after its
    // second and fourth decided races, the witness DH-38
    // states for the deadlock posture of the from behind
    // class. Passes on the pre round tree by design, in
    // the posture of the pins above, the deadlock reading
    // standing built in Round 13. Its failure mode is the
    // disagreement itself.
    let championship = load_example("constructed-opening.json");
    let lines = deadlock_report(&championship);
    assert_eq!(
        lines,
        vec![
            "deadlock",
            "series 2, the constructed opening drill two",
            "gamma-1 against delta-1, first to 3 wins",
            "race 1, opening drill two race one, gamma-1 wins",
            "race 2, opening drill two race two, delta-1 wins",
            "race 3, opening drill two race three, gamma-1 wins",
            "race 4, opening drill two race four, delta-1 wins",
            "race 5, opening drill two race five, delta-1 wins",
            "stood level after 2 of its decided races",
            "series winner, delta-1, gamma-1 2 race wins, delta-1 3 race wins, 0 no results",
            "deadlock, 2 series, 1 deadlocked, 1 clear, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-opening-deadlock.txt")
    );
}

#[test]
fn the_constructed_example_stands_the_distance_under_distance() {
    // K-8. The same file under the distance command, the
    // second series standing the distance in its section
    // alone, its loser standing at the target less one
    // when the decision fell. Passes on the pre round
    // tree by design, in the posture of the pins above.
    // Its failure mode is the disagreement itself.
    let championship = load_example("constructed-opening.json");
    let lines = distance_report(&championship);
    assert_eq!(
        lines,
        vec![
            "distance",
            "series 2, the constructed opening drill two",
            "gamma-1 against delta-1, first to 3 wins",
            "race 1, opening drill two race one, gamma-1 wins",
            "race 2, opening drill two race two, delta-1 wins",
            "race 3, opening drill two race three, gamma-1 wins",
            "race 4, opening drill two race four, delta-1 wins",
            "race 5, opening drill two race five, delta-1 wins",
            "series winner, delta-1, gamma-1 2 race wins, delta-1 3 race wins, 0 no results",
            "distance, 2 series, 1 the distance, 0 without reply, 1 in between, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-opening-distance.txt")
    );
}

#[test]
fn the_constructed_example_stands_from_behind_under_comeback() {
    // K-9. The same file under the comeback command, the
    // second series standing from behind in its section
    // alone, its winner having lost the first decided
    // race. Passes on the pre round tree by design, in
    // the posture of the pins above. Its failure mode is
    // the disagreement itself.
    let championship = load_example("constructed-opening.json");
    let lines = comeback_report(&championship);
    assert_eq!(
        lines,
        vec![
            "comebacks",
            "series 2, the constructed opening drill two",
            "gamma-1 against delta-1, first to 3 wins",
            "race 1, opening drill two race one, gamma-1 wins",
            "race 2, opening drill two race two, delta-1 wins",
            "race 3, opening drill two race three, gamma-1 wins",
            "race 4, opening drill two race four, delta-1 wins",
            "race 5, opening drill two race five, delta-1 wins",
            "first decided race, race 1, delta-1 lost to gamma-1",
            "series winner, delta-1, gamma-1 2 race wins, delta-1 3 race wins, 0 no results",
            "comebacks, 2 series, 1 from behind, 1 from the front",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-opening-comebacks.txt")
    );
}

#[test]
fn the_constructed_example_stands_in_a_run_under_closing() {
    // K-10. The same file under the closing command, the
    // second series standing in a run in its section
    // alone at a closing run of two, the enrolment of the
    // constructed fixture under the closing command, a
    // sibling pin Round 16 could not carry because the
    // closing reading was that round's own subject and is
    // this round's standing sibling. Passes on the pre
    // round tree by design, in the posture of the pins
    // above, the closing reading standing built in Round
    // 16. Its failure mode is the disagreement itself,
    // read against DH-39's stated closing closing over
    // this fixture.
    let championship = load_example("constructed-opening.json");
    let lines = closing_report(&championship);
    assert_eq!(
        lines,
        vec![
            "closing",
            "series 2, the constructed opening drill two",
            "gamma-1 against delta-1, first to 3 wins",
            "race 1, opening drill two race one, gamma-1 wins",
            "race 2, opening drill two race two, delta-1 wins",
            "race 3, opening drill two race three, gamma-1 wins",
            "race 4, opening drill two race four, delta-1 wins",
            "race 5, opening drill two race five, delta-1 wins",
            "closing run of 2 consecutive decided race wins",
            "series winner, delta-1, gamma-1 2 race wins, delta-1 3 race wins, 0 no results",
            "closing, 2 series, 1 in a run, 1 alone, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-opening-closing.txt")
    );
}

#[test]
fn a_series_of_target_one_stands_alone() {
    // K-11. The only decided race of a decided series of
    // target one is its deciding race, won here by the
    // second model, so its opening run stands at one and
    // the series stands alone against DH-38's stated
    // consequence. Fails under the classing mutation, a
    // reading that classes only a series holding a
    // decided race before its deciding race, which drops
    // the lone decider series out of all three classes.
    // Passes under M-C, M-D, and M-E by design, its
    // longest run and its closing run both standing at
    // one with its opening run, so neither positional
    // rival changes its class, and the moved boundary
    // does not reach a figure of one; its teeth are the
    // classing mutation named, and DH-38's target one
    // sentence.
    let dir = temp_dir("openings-target-one");
    let series = series_json(
        "the single saga",
        vec![ALPHA, BETA],
        1,
        vec![clear_win("race one", BETA, ALPHA)],
    );
    let lines = opening_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "opening",
            "opening, 1 series, 0 in a run, 1 alone, 0 from behind, 0 no winner",
        ]
    );
}

#[test]
fn an_interposed_no_result_leaves_an_opening_run_whole() {
    // K-12. The plan's M-B discriminator at hand built
    // scale. A series at target three whose counted races
    // run winner, no result, winner, loser, winner: its
    // decided record runs winner, winner, loser, winner,
    // its opening run stands at two, and it stands in a
    // run with a section at a figure of two. Fails under
    // M-B, the no result standing immediately after the
    // first counted race cutting the counted head to one,
    // so the series stands alone and its section
    // vanishes. Fails under M-D, its closing run standing
    // at one, so it stands alone under that reading.
    // Fails under M-E, its figure standing at exactly
    // two. Passes under M-C by design: its winner's
    // longest run is the opening pair itself, standing at
    // two, so the streak statistic returns the same class
    // and figure; the M-C carriers stand at K-2, K-3, and
    // K-14.
    let dir = temp_dir("openings-interposed");
    let series = series_json(
        "the interposed opening saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            no_result("race two", ALPHA, BETA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", BETA, ALPHA),
            clear_win("race five", ALPHA, BETA),
        ],
    );
    let lines = opening_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "opening",
            "series 1, the interposed opening saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, alpha-1 wins",
            "race 3, race three, alpha-1 wins",
            "race 4, race four, beta-1 wins",
            "race 5, race five, alpha-1 wins",
            "opening run of 2 consecutive decided race wins",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 1 race wins, 1 no results",
            "opening, 1 series, 1 in a run, 0 alone, 0 from behind, 0 no winner",
        ]
    );
}

#[test]
fn an_interposed_no_result_leaves_the_section_naming_decided_races_only() {
    // K-13. A series at target five whose counted races
    // run winner, winner, no result, winner, loser,
    // winner, winner: its decided record stands at six
    // races, its opening run at three, and its section
    // names its six decided races in race order, its
    // figure of three, and its split of six decided and
    // one no result. Fails under M-B on the figure, the
    // counted head breaking at the no result standing
    // third and reading two against the section's three,
    // its class unchanged, which is this pin's distinct
    // teeth against K-12's class teeth. Fails under the
    // mutation that walks every counted race into the
    // section's race lines, which names seven races
    // against the golden's six. Fails under M-D on the
    // figure, its closing run standing at two against
    // the section's three, its class unchanged. Passes
    // under M-C by design, its winner's longest run
    // standing at three as its opening run does. Passes
    // under M-E by design, its figure of three standing
    // on the marked side of either boundary.
    let dir = temp_dir("openings-interposed-section");
    let series = series_json(
        "the counted opening saga",
        vec![ALPHA, BETA],
        5,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", ALPHA, BETA),
            no_result("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", BETA, ALPHA),
            clear_win("race six", ALPHA, BETA),
            clear_win("race seven", ALPHA, BETA),
        ],
    );
    let lines = opening_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "opening",
            "series 1, the counted opening saga",
            "alpha-1 against beta-1, first to 5 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, alpha-1 wins",
            "race 4, race four, alpha-1 wins",
            "race 5, race five, beta-1 wins",
            "race 6, race six, alpha-1 wins",
            "race 7, race seven, alpha-1 wins",
            "opening run of 3 consecutive decided race wins",
            "series winner, alpha-1, alpha-1 5 race wins, beta-1 1 race wins, 1 no results",
            "opening, 1 series, 1 in a run, 0 alone, 0 from behind, 0 no winner",
        ]
    );
}

/// The offsets of the series standing a section in one
/// report, read from the section opening lines. A line
/// opens a section when it reads series, an offset, and
/// a comma, the form the section openings of every
/// classification report take, and the series winner
/// lines, which open with the same word, are not
/// sections and are passed over.
fn section_offsets(lines: &[String]) -> Vec<u64> {
    lines
        .iter()
        .filter_map(|line| {
            let head = line.strip_prefix("series ")?;
            let token = head.split(',').next()?;
            token.parse::<u64>().ok()
        })
        .collect()
}

/// The figures of the sections of one opening report, in
/// section order.
fn section_figures(lines: &[String]) -> Vec<u64> {
    lines
        .iter()
        .filter(|line| line.starts_with("opening run of "))
        .map(|line| {
            line.split(' ')
                .nth(3)
                .expect("a figure")
                .parse::<u64>()
                .expect("a figure")
        })
        .collect()
}

/// The stated relations of DH-38 and DH-39 over one
/// championship, read from the printed readings alone.
/// The section figures and memberships stand stated by
/// the caller, and the series standing without reply
/// under DH-23 stand named by their championship
/// offsets, each standing in an opening section by
/// DH-38's stated relation at targets above one.
fn check_relations(
    championship: &Championship,
    series_count: u64,
    figures: Vec<u64>,
    sections_expected: Vec<u64>,
    without_reply: Vec<u64>,
) {
    let table = championship_report(championship);
    let decided = table
        .iter()
        .filter(|line| line.starts_with("series winner,"))
        .count() as u64;
    assert!(decided > 0, "the fixture holds decided series");
    let opening = opening_report(championship);
    let counts = closing_counts(opening.last().expect("a closing"));
    assert_eq!(counts[0], series_count);
    assert_eq!(
        counts[1] + counts[2] + counts[3],
        decided,
        "the three classes"
    );
    assert_eq!(counts[1] + counts[2] + counts[3] + counts[4], series_count);
    assert_eq!(
        section_figures(&opening),
        figures,
        "the section figures in order"
    );
    let sections = section_offsets(&opening);
    assert_eq!(sections, sections_expected, "the section memberships");
    assert_eq!(
        sections.len() as u64,
        counts[1],
        "one section per series in a run"
    );
    // The no winner count equals the figures of the
    // seven sibling closings DH-39 names, the comeback
    // leftover among them, because the eight readings
    // read one walk.
    let distance = distance_report(championship);
    let distance_counts = closing_counts(distance.last().expect("a closing"));
    assert_eq!(counts[4], distance_counts[4], "the no winner figure");
    let comebacks = comeback_report(championship);
    let comeback_counts = closing_counts(comebacks.last().expect("a closing"));
    assert_eq!(
        counts[4],
        series_count - (comeback_counts[1] + comeback_counts[2]),
        "the comeback leftover"
    );
    let rally = rally_report(championship);
    let rally_counts = closing_counts(rally.last().expect("a closing"));
    assert_eq!(counts[4], rally_counts[3], "the rally no winner figure");
    let deadlock = deadlock_report(championship);
    let deadlock_counts = closing_counts(deadlock.last().expect("a closing"));
    assert_eq!(
        counts[4], deadlock_counts[3],
        "the deadlock no winner figure"
    );
    let streak = streak_report(championship);
    let streak_counts = closing_counts(streak.last().expect("a closing"));
    assert_eq!(counts[4], streak_counts[3], "the streak no winner figure");
    let cluster = cluster_report(championship);
    let cluster_counts = closing_counts(cluster.last().expect("a closing"));
    assert_eq!(counts[4], cluster_counts[3], "the cluster no winner figure");
    let closing = closing_report(championship);
    let closing_counts_vec = closing_counts(closing.last().expect("a closing"));
    assert_eq!(
        counts[4], closing_counts_vec[3],
        "the closing no winner figure"
    );
    // The count of series standing from behind is the
    // count of series won from behind in the comeback
    // closing, as DH-38 states, and the counts of series
    // standing in a run and alone sum to its count of
    // series won from the front.
    assert_eq!(
        counts[3], comeback_counts[1],
        "the from behind count equals the comeback count"
    );
    assert_eq!(
        counts[1] + counts[2],
        comeback_counts[2],
        "the in a run and alone counts sum to the from the front count"
    );
    // Every series standing in a run stands strung under
    // DH-32, and every series standing single stands
    // alone or from behind, as DH-38 states, so the in a
    // run count stands at or below the strung count and
    // the alone and from behind counts together stand at
    // or above the single count.
    assert!(
        counts[1] <= streak_counts[1],
        "the in a run count stands at or below the strung count"
    );
    assert!(
        counts[2] + counts[3] >= streak_counts[2],
        "the alone and from behind counts stand at or above the single count"
    );
    // Every series standing in an opening section stands
    // in a streak section, its opening run being a run of
    // its winner at two or more.
    let streak_sections = section_offsets(&streak);
    for offset in &sections {
        assert!(
            streak_sections.contains(offset),
            "series {offset} stands in a streak section"
        );
    }
    // Every named series standing without reply stands
    // in an opening section, its winner's wins standing
    // in one unbroken run from the head of its record to
    // a target above one.
    for offset in without_reply {
        assert!(
            sections.contains(&offset),
            "series {offset} stands in an opening section"
        );
    }
}

#[test]
fn the_opening_counts_stand_in_their_stated_relations() {
    // K-14. The relations of DH-38 and DH-39 over the
    // survey championship and the league championship of
    // DH-31, test by test, the opening printed by the
    // opening, streak, cluster, closing, championship,
    // comeback, distance, rally, and deadlock readings
    // over each fixture cross-read through the shared
    // test module's closing counts helper, and the
    // section figures of the opening report cross-read
    // against the streak and closing sections of the
    // same series. Fails under M-C, naming its parting
    // series: the survey apex-bolt (longest run two,
    // opening run one), apex-drift (two, one), bolt-crest
    // (three, one), bolt-drift (four, none), and
    // crest-drift (three, none), and the league apex-bolt
    // (two, none), apex-crest (three, one), apex-drift
    // (four, none), apex-ember (two, none), bolt-ember
    // (three, none), and crest-ember (three, one), each
    // standing in a run under the streak statistic where
    // this reading classes it alone or from behind, so
    // the in a run count's standing at or below the
    // strung count breaks in the cross-read. Fails under
    // M-D, naming: the survey apex-crest (opening run
    // three, closing run one), apex-bolt (one, two), and
    // crest-drift (none, three), and the league
    // bolt-crest (three, one), apex-ember (none, two),
    // bolt-ember (none, three), and crest-ember (one,
    // three). Fails under M-B, naming the league
    // crest-drift series: its counted record opens on a
    // no result, so its opening run over the counted
    // races reads none against its decided figure of one,
    // the series moving from alone to from behind and the
    // league closing line moving with it (3 in a run, 3
    // alone, 4 from behind reading 3 in a run, 2 alone, 5
    // from behind over the league's opening figures).
    // The pin also carries DH-38's comeback equality at
    // both fixtures (the from behind counts, survey two
    // and league four, equalling the comebacks closings'
    // from behind counts over the same championships) and
    // its without reply relation at the league's two
    // without reply series, bolt-drift and drift-ember,
    // both standing in a run at targets above one.
    // Passes under M-E by design at fixture scale: no
    // survey or league series stands at exactly two, the
    // survey figures standing at three, one, one, one,
    // none, and none and the league figures at none, one,
    // none, none, three, three, none, one, one, and three
    // in championship order; the M-E carriers stand at
    // K-1, K-3, K-15, and K-16. M-A is not stated at
    // fixture scale, for the reason K-1 states in its own
    // form: the fixtures' full records stand under the
    // winner rule applied to races beyond the decision,
    // which this plan does not re-derive; M-A is carried
    // at hand-built scale by K-16.
    check_relations(
        &load_example("survey-championship.json"),
        6,
        vec![3],
        vec![2],
        vec![],
    );
    check_relations(
        &load_example("league-championship.json"),
        10,
        vec![3, 3, 3],
        vec![5, 6, 10],
        vec![6, 10],
    );
    // The second arm, hand-built: one series at a wins
    // target of four whose decided winners run winner,
    // winner, loser, loser, loser, winner, winner,
    // standing in a run with a section at a figure of
    // two, standing a rally (its winner behind by one
    // after the fifth decided race) and deadlocked (its
    // tallies level after the fourth), so DH-38's rally
    // direction for the in a run class stands witnessed
    // at hand-built scale beside the fixtures' wire to
    // wire and deadlocked witnesses. The second arm
    // passes under M-C and M-D by design, its longest run
    // and its closing run both standing at two as its
    // opening run does; its teeth are the rally and
    // deadlock cross-read itself.
    let dir = temp_dir("openings-relations-arm");
    let series = series_json(
        "the rally deadlock opening saga",
        vec![ALPHA, BETA],
        4,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", BETA, ALPHA),
            clear_win("race five", BETA, ALPHA),
            clear_win("race six", ALPHA, BETA),
            clear_win("race seven", ALPHA, BETA),
        ],
    );
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    let lines = opening_report(&championship);
    assert_eq!(
        lines,
        vec![
            "opening",
            "series 1, the rally deadlock opening saga",
            "alpha-1 against beta-1, first to 4 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, alpha-1 wins",
            "race 3, race three, beta-1 wins",
            "race 4, race four, beta-1 wins",
            "race 5, race five, beta-1 wins",
            "race 6, race six, alpha-1 wins",
            "race 7, race seven, alpha-1 wins",
            "opening run of 2 consecutive decided race wins",
            "series winner, alpha-1, alpha-1 4 race wins, beta-1 3 race wins, 0 no results",
            "opening, 1 series, 1 in a run, 0 alone, 0 from behind, 0 no winner",
        ]
    );
    let rally = rally_report(&championship);
    let rally_counts = closing_counts(rally.last().expect("a closing"));
    assert_eq!(rally_counts[1], 1, "the arm stands a rally");
    let deadlock = deadlock_report(&championship);
    let deadlock_counts = closing_counts(deadlock.last().expect("a closing"));
    assert_eq!(deadlock_counts[1], 1, "the arm stands deadlocked");
}

#[test]
fn the_section_figure_reads_the_opening_run() {
    // K-15. The plan's M-D discriminator at hand built
    // scale. A series at a wins target of four whose
    // decided winners run winner, winner, loser, winner,
    // loser, winner: its winner's runs stand at two, one,
    // and one, its opening run at two, its longest at
    // two, and its closing run at one, so it stands in a
    // run with a section figure of two. Fails under M-D,
    // the closing run of one classing it alone and its
    // section vanishing. Fails under M-E, its figure
    // standing at exactly two. Passes under M-C by
    // design, its longest run standing at two as its
    // opening run does, the section and figure unchanged;
    // the M-C separation stands at K-14's scale, stated
    // here so no blanket claim covers this pin. Passes
    // under M-B by design: its record carries no no
    // results, so no counted head can break.
    let dir = temp_dir("openings-closing-run");
    let series = series_json(
        "the closing run saga",
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
    let lines = opening_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "opening",
            "series 1, the closing run saga",
            "alpha-1 against beta-1, first to 4 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, alpha-1 wins",
            "race 3, race three, beta-1 wins",
            "race 4, race four, alpha-1 wins",
            "race 5, race five, beta-1 wins",
            "race 6, race six, alpha-1 wins",
            "opening run of 2 consecutive decided race wins",
            "series winner, alpha-1, alpha-1 4 race wins, beta-1 2 race wins, 0 no results",
            "opening, 1 series, 1 in a run, 0 alone, 0 from behind, 0 no winner",
        ]
    );
}

#[test]
fn a_series_decided_early_keeps_its_class_against_its_later_races() {
    // K-16. The plan's M-A discriminator at hand built
    // scale. A series at a wins target of two whose
    // decided winners run winner, winner, decided at the
    // second race with an opening run of two, standing in
    // a run with a section at a figure of two, and
    // holding a third and fourth recorded race, both won
    // by its winner, and a fifth, won by its loser,
    // beyond the decision. Fails under M-A, the full
    // record's head running winner four times before its
    // loser first wins, so the winner's opening run over
    // the full record reads four against the section's
    // two. Fails under M-E, its figure standing at
    // exactly two. Passes under M-C and M-D by design, its
    // longest run and its closing run both standing at
    // two as its opening run does.
    let dir = temp_dir("openings-beyond");
    let series = series_json(
        "the early opening saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", BETA, ALPHA),
        ],
    );
    let lines = opening_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "opening",
            "series 1, the early opening saga",
            "alpha-1 against beta-1, first to 2 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, alpha-1 wins",
            "opening run of 2 consecutive decided race wins",
            "series winner, alpha-1, alpha-1 2 race wins, beta-1 0 race wins, 0 no results",
            "opening, 1 series, 1 in a run, 0 alone, 0 from behind, 0 no winner",
        ]
    );
}

/// The handicap twin of the opening pins. On scratch
/// finishes the series runs loser, winner, winner,
/// winner at target three, decided at the fourth race
/// with an opening run of none, and stands from behind.
/// The first race's second model attempt carries an
/// adjustment that moves its finish behind the first
/// model's, so the adjusted record runs winner, winner,
/// winner, the series stands decided at the third race,
/// and its opening run stands at three.
fn handicap_series(adjusted: bool) -> JsonValue {
    let mut first_second = attempt_two(BETA, "green", 110.0, Some(20));
    if adjusted {
        first_second = set(&first_second, "adjustment", i(30));
    }
    series_json(
        "the handicap opening saga",
        vec![ALPHA, BETA],
        3,
        vec![
            race_json(
                "race one",
                vec![attempt(ALPHA, "green", 130.0, Some(10)), first_second],
            ),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
        ],
    )
}

#[test]
fn a_handicap_adjustment_moves_a_series_between_from_behind_and_in_a_run() {
    // K-17. The twin stands from behind on its scratch
    // record, the report standing in the empty form, and
    // in a run with a section at a figure of three on its
    // adjusted record. Fails under the mutation that
    // reads the scratch winners in place of the adjusted
    // record, the posture of the Round 16 K-15 pin and
    // the FIX-DH-11 genus. The race figures stand inside
    // the DH-11 and DH-12 forms, an adjustment inside the
    // adjustment band moving a decided race across the
    // close call band, as the standing handicap pins
    // author theirs.
    let dir = temp_dir("openings-handicap");
    let plain = opening_for(
        &dir,
        vec![("one.json", handicap_series(false))],
        vec!["one.json"],
    );
    assert_eq!(
        plain,
        vec![
            "opening",
            "opening, 1 series, 0 in a run, 0 alone, 1 from behind, 0 no winner",
        ]
    );
    let adjusted = opening_for(
        &dir,
        vec![("one.json", handicap_series(true))],
        vec!["one.json"],
    );
    assert_eq!(
        adjusted,
        vec![
            "opening",
            "series 1, the handicap opening saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, alpha-1 wins",
            "race 3, race three, alpha-1 wins",
            "opening run of 3 consecutive decided race wins",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 0 race wins, 0 no results",
            "opening, 1 series, 1 in a run, 0 alone, 0 from behind, 0 no winner",
        ]
    );
}

#[test]
fn a_series_holding_no_winner_stands_in_no_class() {
    // K-18. A series at a wins target of four whose
    // decided winners run winner, winner, loser, winner,
    // the target unreached, holds no winner and stands in
    // none of the three classes, with no section, the
    // closing counting it as no winner, although the
    // opening run of its first model over the decided
    // record stands at two. Fails under the champion drop
    // mutation that reads the first model as the winner
    // of a series holding no winner, in the Round 16 K-16
    // form, under which the series would stand in a run
    // with a section at a figure of two.
    let dir = temp_dir("openings-open");
    let series = series_json(
        "the open opening saga",
        vec![ALPHA, BETA],
        4,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
        ],
    );
    let lines = opening_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "opening",
            "opening, 1 series, 0 in a run, 0 alone, 0 from behind, 1 no winner",
        ]
    );
}
