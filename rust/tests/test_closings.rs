//! Pins for DH-36 and DH-37 in the Round 16 section of
//! CONTRACT.md.
//!
//! The discrimination record of this file stands pin by
//! pin, and no blanket claim stands over it. The closing
//! reading, its library function, and its command do not
//! stand on the pre round tree, so every pin in this file
//! is absent there at compile time. That absence is
//! recorded once here and is not a discrimination claim.
//! The discrimination each pin carries is against the
//! stated mutation of DH-36 beside it. Five mutations are
//! named for the whole file. M-A, the walk's early stop
//! removed for this reading, the closing run read over
//! the suffix of the full recorded record, races beyond
//! the decision among them. M-B, a race holding no result
//! breaking the closing run, the run read over the
//! counted races in place of the decided record. M-C, the
//! winner's longest run, the streak statistic of DH-32,
//! read in place of the closing run, in the class and in
//! the section figure alike. M-D, the winner's opening
//! run, the run standing first in the decided record,
//! read in place of the closing run. M-E, the class
//! boundary moved, a series standing in a run only where
//! its closing run stands at three or more, the figure
//! line unchanged. Six pins, the constructed file under
//! the streak, cluster, rally, deadlock, distance, and
//! comeback commands, pass against the pre round surface
//! by design, as their own comments state. K-1 states no
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

fn closing_for(dir: &Path, files: Vec<(&str, JsonValue)>, names: Vec<&str>) -> Vec<String> {
    closing_report(&load(dir, files, names))
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
fn the_standing_championship_carries_the_panel_section_alone() {
    // K-1. The standing championship golden, the panel
    // section alone at a figure of three, the pilot
    // standing alone with a closing run of one. Fails
    // under M-B, by the panel's counted record as the
    // panel series golden prints it (win, win, no result,
    // no result, win): read over the counted races, the
    // run ending at the decider breaks at the no result
    // standing immediately before it, the panel's closing
    // run reads one, its section vanishes, and the closing
    // reads 0 in a run, 2 alone. Fails under M-C, the
    // pilot gaining a section at its longest run of two
    // and the closing reading 2 in a run, 0 alone. Fails
    // under M-D, the pilot gaining a section at its
    // opening run of two. Passes under M-E by design: the
    // panel's figure of three stands on the marked side
    // of either boundary, and the pilot's one stands
    // alone under either, so no series of the standing
    // championship sits at the moved boundary; the
    // boundary carriers stand at K-2 and K-14. M-A is not
    // stated for this pin: the standing series' races
    // beyond the decision stand under the panel and
    // adjudication rules, whose full records this plan
    // does not re-derive; M-A is carried at fixture scale
    // by K-13 and at hand built scale by K-17.
    let championship = load_example("championship.json");
    let lines = closing_report(&championship);
    assert_eq!(
        lines,
        vec![
            "closing",
            "series 2, the recorded close call panel",
            "prime-1 against shadow-1, first to 3 wins",
            "race 1, pin the cache key, prime-1 wins",
            "race 2, trim the import list, prime-1 wins",
            "race 5, name the worker pool, prime-1 wins",
            "closing run of 3 consecutive decided race wins",
            "series winner, prime-1, prime-1 3 race wins, shadow-1 0 race wins, 2 no results",
            "closing, 2 series, 1 in a run, 1 alone, 0 no winner",
        ]
    );
    assert_eq!(lines.join("\n") + "\n", golden("championship-closing.txt"));
}

#[test]
fn the_survey_championship_matches_its_closing_golden() {
    // K-2. The survey championship golden, four sections
    // in championship order at figures 2, 2, 3, 3, pinned
    // byte for byte against the golden the real command
    // produced. Fails under M-C, the apex-crest series
    // gaining a section at its longest run of three and
    // the bolt-drift series gaining one at four, the
    // closing reading 6 in a run, 0 alone. Fails under
    // M-A, the apex-bolt series losing its section, its
    // full record ending on two wins of its loser, and
    // the crest-drift figure reading seven against the
    // golden's three. Fails under M-B, the crest-drift
    // figure reading two against three, its counted
    // record running no result, a win of crest-1, no
    // result, a win of drift-1, no result, and two
    // further wins of drift-1. Fails under M-D, the
    // apex-crest series gaining a section at its opening
    // run of three and the bolt-crest series losing its
    // own, its opening run standing at one. Fails under
    // M-E, the apex-bolt and apex-drift sections standing
    // at exactly two and vanishing, the closing reading
    // 2 in a run, 4 alone.
    let championship = load_example("survey-championship.json");
    let lines = closing_report(&championship);
    assert!(lines.len() > 2, "the survey golden stands in section form");
    assert_eq!(
        lines.join("\n") + "\n",
        golden("survey-championship-closing.txt")
    );
}

#[test]
fn the_constructed_example_parts_in_a_run_from_alone() {
    // K-3. The round's headline discrimination. The first
    // constructed series stands alone with a closing run
    // of one, its decided record running alpha-1,
    // alpha-1, beta-1, alpha-1 with a fifth recorded race
    // won by beta-1 beyond the decision, while standing
    // strung at a longest run of two, clear, wire to
    // wire, from the front, in between, and scattered,
    // and the second stands in a run with a closing run
    // of two, its decided record running delta-1,
    // gamma-1, delta-1, gamma-1, gamma-1, so the closing
    // golden carries the second series alone. Fails under
    // M-C, the first series gaining a section at its
    // longest run of two, the proof series of DH-36's
    // stated relations rendered as a section. Fails under
    // M-D, the first series gaining a section at its
    // opening run of two. Fails under M-E, the second
    // series standing at exactly two and losing its
    // section. Passes under M-A by design: the first
    // series' one race beyond the decision was won by its
    // loser, so over the full record its closing run
    // reads none and it stands alone under either
    // reading, and the second series holds no race beyond
    // its decision; the M-A carriers stand at K-13 and
    // K-17. Passes under M-B by design: the constructed
    // records carry no no results at all, as DH-37's
    // examples block states, so no counted suffix can
    // break; the M-B carriers stand at K-1, K-11, and
    // K-12.
    let championship = load_example("constructed-closing.json");
    let lines = closing_report(&championship);
    assert_eq!(
        lines,
        vec![
            "closing",
            "series 2, the constructed closing drill two",
            "gamma-1 against delta-1, first to 3 wins",
            "race 1, closing drill two race one, delta-1 wins",
            "race 2, closing drill two race two, gamma-1 wins",
            "race 3, closing drill two race three, delta-1 wins",
            "race 4, closing drill two race four, gamma-1 wins",
            "race 5, closing drill two race five, gamma-1 wins",
            "closing run of 2 consecutive decided race wins",
            "series winner, gamma-1, gamma-1 3 race wins, delta-1 2 race wins, 0 no results",
            "closing, 2 series, 1 in a run, 1 alone, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-closing-closing.txt")
    );
}

#[test]
fn the_constructed_example_stands_strung_under_streak() {
    // K-4. The same file under the streak command, both
    // series standing strung at a figure of two, the
    // first series standing as the proof that a series
    // stands strung and alone at once. Passes on the pre
    // round tree by design, in the posture of DH-35's
    // goldens over standing commands, the streak reading
    // and its command standing built in Round 14. The pin
    // holds that the round's new fixture does not move a
    // standing reading and that the two readings part
    // over the same file, read against the pin above.
    // Its failure mode is the disagreement itself: a
    // streak reading of this fixture that parts from
    // DH-37's stated figures fails here.
    let championship = load_example("constructed-closing.json");
    let lines = streak_report(&championship);
    assert_eq!(
        lines,
        vec![
            "streak",
            "series 1, the constructed closing drill one",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, closing drill one race one, alpha-1 wins",
            "race 2, closing drill one race two, alpha-1 wins",
            "race 3, closing drill one race three, beta-1 wins",
            "race 4, closing drill one race four, alpha-1 wins",
            "longest run of 2 consecutive decided race wins",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 1 race wins, 0 no results",
            "series 2, the constructed closing drill two",
            "gamma-1 against delta-1, first to 3 wins",
            "race 1, closing drill two race one, delta-1 wins",
            "race 2, closing drill two race two, gamma-1 wins",
            "race 3, closing drill two race three, delta-1 wins",
            "race 4, closing drill two race four, gamma-1 wins",
            "race 5, closing drill two race five, gamma-1 wins",
            "longest run of 2 consecutive decided race wins",
            "series winner, gamma-1, gamma-1 3 race wins, delta-1 2 race wins, 0 no results",
            "streak, 2 series, 2 strung, 0 single, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-closing-streak.txt")
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
    // disagreement itself, read against DH-37's stated
    // cluster closing over this fixture.
    let championship = load_example("constructed-closing.json");
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
        golden("constructed-closing-cluster.txt")
    );
}

#[test]
fn the_constructed_example_stands_a_rally_under_rally() {
    // K-6. The same file under the rally command, the
    // second series standing a rally in its section
    // alone, its winner having stood behind after its
    // first and third decided races, the witness DH-36
    // states for the rally direction. Passes on the pre
    // round tree by design, in the posture of the pins
    // above, the rally reading standing built in Round
    // 12. Its failure mode is the disagreement itself.
    let championship = load_example("constructed-closing.json");
    let lines = rally_report(&championship);
    assert_eq!(
        lines,
        vec![
            "rally",
            "series 2, the constructed closing drill two",
            "gamma-1 against delta-1, first to 3 wins",
            "race 1, closing drill two race one, delta-1 wins",
            "race 2, closing drill two race two, gamma-1 wins",
            "race 3, closing drill two race three, delta-1 wins",
            "race 4, closing drill two race four, gamma-1 wins",
            "race 5, closing drill two race five, gamma-1 wins",
            "stood behind by 1 race wins at the deepest",
            "series winner, gamma-1, gamma-1 3 race wins, delta-1 2 race wins, 0 no results",
            "rally, 2 series, 1 rally, 1 wire to wire, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-closing-rally.txt")
    );
}

#[test]
fn the_constructed_example_stands_deadlocked_under_deadlock() {
    // K-7. The same file under the deadlock command, the
    // second series standing deadlocked in its section
    // alone, its tallies having stood level after its
    // second and fourth decided races, the witness DH-36
    // states for the deadlock direction. Passes on the
    // pre round tree by design, in the posture of the
    // pins above, the deadlock reading standing built in
    // Round 13. Its failure mode is the disagreement
    // itself.
    let championship = load_example("constructed-closing.json");
    let lines = deadlock_report(&championship);
    assert_eq!(
        lines,
        vec![
            "deadlock",
            "series 2, the constructed closing drill two",
            "gamma-1 against delta-1, first to 3 wins",
            "race 1, closing drill two race one, delta-1 wins",
            "race 2, closing drill two race two, gamma-1 wins",
            "race 3, closing drill two race three, delta-1 wins",
            "race 4, closing drill two race four, gamma-1 wins",
            "race 5, closing drill two race five, gamma-1 wins",
            "stood level after 2 of its decided races",
            "series winner, gamma-1, gamma-1 3 race wins, delta-1 2 race wins, 0 no results",
            "deadlock, 2 series, 1 deadlocked, 1 clear, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-closing-deadlock.txt")
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
    let championship = load_example("constructed-closing.json");
    let lines = distance_report(&championship);
    assert_eq!(
        lines,
        vec![
            "distance",
            "series 2, the constructed closing drill two",
            "gamma-1 against delta-1, first to 3 wins",
            "race 1, closing drill two race one, delta-1 wins",
            "race 2, closing drill two race two, gamma-1 wins",
            "race 3, closing drill two race three, delta-1 wins",
            "race 4, closing drill two race four, gamma-1 wins",
            "race 5, closing drill two race five, gamma-1 wins",
            "series winner, gamma-1, gamma-1 3 race wins, delta-1 2 race wins, 0 no results",
            "distance, 2 series, 1 the distance, 0 without reply, 1 in between, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-closing-distance.txt")
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
    let championship = load_example("constructed-closing.json");
    let lines = comeback_report(&championship);
    assert_eq!(
        lines,
        vec![
            "comebacks",
            "series 2, the constructed closing drill two",
            "gamma-1 against delta-1, first to 3 wins",
            "race 1, closing drill two race one, delta-1 wins",
            "race 2, closing drill two race two, gamma-1 wins",
            "race 3, closing drill two race three, delta-1 wins",
            "race 4, closing drill two race four, gamma-1 wins",
            "race 5, closing drill two race five, gamma-1 wins",
            "first decided race, race 1, gamma-1 lost to delta-1",
            "series winner, gamma-1, gamma-1 3 race wins, delta-1 2 race wins, 0 no results",
            "comebacks, 2 series, 1 from behind, 1 from the front",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-closing-comebacks.txt")
    );
}

#[test]
fn a_series_of_target_one_stands_alone() {
    // K-10. The only decided race of a decided series of
    // target one is its deciding race, won here by the
    // second model, so its closing run stands at one and
    // the series stands alone against DH-36's stated
    // consequence. Fails under the classing mutation, a
    // reading that classes only a series holding a
    // decided race before its deciding race, which drops
    // the lone decider series out of both classes.
    // Passes under M-D and M-E by design, its opening run
    // and its closing run both standing at one, so
    // neither the positional rival nor the moved boundary
    // changes its class; its teeth are the classing
    // mutation named, and DH-36's target one sentence.
    let dir = temp_dir("closings-target-one");
    let series = series_json(
        "the single saga",
        vec![ALPHA, BETA],
        1,
        vec![clear_win("race one", BETA, ALPHA)],
    );
    let lines = closing_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "closing",
            "closing, 1 series, 0 in a run, 1 alone, 0 no winner",
        ]
    );
}

#[test]
fn an_interposed_no_result_leaves_a_closing_run_whole() {
    // K-11. The plan's M-B discriminator at hand built
    // scale. A series at target three whose counted races
    // run winner, loser, winner, no result, winner: its
    // decided record runs winner, loser, winner, winner,
    // its closing run stands at two, and it stands in a
    // run with a section at a figure of two. Fails under
    // M-B, the no result standing immediately before the
    // deciding race cutting the counted suffix to one, so
    // the series stands alone and its section vanishes.
    // Fails under M-E, its figure standing at exactly
    // two. Passes under M-C by design: its winner's
    // longest run is the closing pair itself, standing at
    // two, so the streak statistic returns the same class
    // and figure; the M-C carriers stand at K-1, K-2, K-3,
    // and K-13.
    let dir = temp_dir("closings-interposed");
    let series = series_json(
        "the interposed closing saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            no_result("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
        ],
    );
    let lines = closing_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "closing",
            "series 1, the interposed closing saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, alpha-1 wins",
            "race 5, race five, alpha-1 wins",
            "closing run of 2 consecutive decided race wins",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 1 race wins, 1 no results",
            "closing, 1 series, 1 in a run, 0 alone, 0 no winner",
        ]
    );
}

#[test]
fn an_interposed_no_result_leaves_the_section_naming_decided_races_only() {
    // K-12. A series at target four whose counted races
    // run winner, no result, loser, winner, no result,
    // winner, winner: its decided record stands at five
    // races, its closing run at three, and its section
    // names its five decided races in race order, its
    // figure of three, and its split of five decided and
    // two no results. Fails under M-B on the figure, the
    // counted suffix breaking at the second no result and
    // reading two against the section's three, its class
    // unchanged, which is this pin's distinct teeth
    // against K-11's class teeth. Fails under the
    // mutation that walks every counted race into the
    // section's race lines, which names seven races
    // against the golden's five.
    let dir = temp_dir("closings-interposed-section");
    let series = series_json(
        "the counted closing saga",
        vec![ALPHA, BETA],
        4,
        vec![
            clear_win("race one", ALPHA, BETA),
            no_result("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
            no_result("race five", ALPHA, BETA),
            clear_win("race six", ALPHA, BETA),
            clear_win("race seven", ALPHA, BETA),
        ],
    );
    let lines = closing_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "closing",
            "series 1, the counted closing saga",
            "alpha-1 against beta-1, first to 4 wins",
            "race 1, race one, alpha-1 wins",
            "race 3, race three, beta-1 wins",
            "race 4, race four, alpha-1 wins",
            "race 6, race six, alpha-1 wins",
            "race 7, race seven, alpha-1 wins",
            "closing run of 3 consecutive decided race wins",
            "series winner, alpha-1, alpha-1 4 race wins, beta-1 1 race wins, 2 no results",
            "closing, 1 series, 1 in a run, 0 alone, 0 no winner",
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

/// The figures of the sections of one closing report, in
/// section order.
fn section_figures(lines: &[String]) -> Vec<u64> {
    lines
        .iter()
        .filter(|line| line.starts_with("closing run of "))
        .map(|line| {
            line.split(' ')
                .nth(3)
                .expect("a figure")
                .parse::<u64>()
                .expect("a figure")
        })
        .collect()
}

/// The stated relations of DH-36 and DH-37 over one
/// championship, read from the printed readings alone.
/// The section figures and memberships stand stated by
/// the caller, the alone series that stand strung under
/// DH-32 stand named by their championship offsets, and
/// the series standing without reply under DH-23 stand
/// named likewise, each standing in a closing section by
/// DH-36's stated relation at targets above one.
fn check_relations(
    championship: &Championship,
    series_count: u64,
    figures: Vec<u64>,
    sections_expected: Vec<u64>,
    alone_strung: Vec<u64>,
    without_reply: Vec<u64>,
) {
    let table = championship_report(championship);
    let decided = table
        .iter()
        .filter(|line| line.starts_with("series winner,"))
        .count() as u64;
    assert!(decided > 0, "the fixture holds decided series");
    let closing = closing_report(championship);
    let counts = closing_counts(closing.last().expect("a closing"));
    assert_eq!(counts[0], series_count);
    assert_eq!(counts[1] + counts[2], decided, "in a run and alone");
    assert_eq!(counts[1] + counts[2] + counts[3], series_count);
    assert_eq!(
        section_figures(&closing),
        figures,
        "the section figures in order"
    );
    let sections = section_offsets(&closing);
    assert_eq!(sections, sections_expected, "the section memberships");
    assert_eq!(
        sections.len() as u64,
        counts[1],
        "one section per series in a run"
    );
    // The no winner count equals the figures of the six
    // sibling closings DH-37 names, the comeback leftover
    // among them, because the seven readings read one
    // walk.
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
    let rally = rally_report(championship);
    let rally_counts = closing_counts(rally.last().expect("a closing"));
    assert_eq!(counts[3], rally_counts[3], "the rally no winner figure");
    let deadlock = deadlock_report(championship);
    let deadlock_counts = closing_counts(deadlock.last().expect("a closing"));
    assert_eq!(
        counts[3], deadlock_counts[3],
        "the deadlock no winner figure"
    );
    let streak = streak_report(championship);
    let streak_counts = closing_counts(streak.last().expect("a closing"));
    assert_eq!(counts[3], streak_counts[3], "the streak no winner figure");
    let cluster = cluster_report(championship);
    let cluster_counts = closing_counts(cluster.last().expect("a closing"));
    assert_eq!(counts[3], cluster_counts[3], "the cluster no winner figure");
    // Every series standing in a run stands strung under
    // DH-32, and every series standing single stands
    // alone, as DH-36 states, so the in a run count
    // stands at or below the strung count and the alone
    // count at or above the single count.
    assert!(
        counts[1] <= streak_counts[1],
        "the in a run count stands at or below the strung count"
    );
    assert!(
        counts[2] >= streak_counts[2],
        "the alone count stands at or above the single count"
    );
    // Every named alone series that stands strung stands
    // in a streak section and in no closing section, the
    // parting DH-36 states between the two readings.
    let streak_sections = section_offsets(&streak);
    for offset in alone_strung {
        assert!(
            !sections.contains(&offset),
            "series {offset} stands in no closing section"
        );
        assert!(
            streak_sections.contains(&offset),
            "series {offset} stands in a streak section"
        );
    }
    // Every named series standing without reply stands
    // in a closing section, its winner's wins standing in
    // one unbroken run to a target above one.
    for offset in without_reply {
        assert!(
            sections.contains(&offset),
            "series {offset} stands in a closing section"
        );
    }
}

#[test]
fn the_closing_counts_stand_in_their_stated_relations() {
    // K-13. The relations of DH-36 and DH-37 over the
    // survey championship and the league championship of
    // DH-31, test by test, the closing printed by the
    // closing, streak, cluster, championship, comeback,
    // distance, rally, and deadlock readings over each
    // fixture cross-read through the shared test module's
    // closing counts helper, and the section figures of
    // the closing report cross-read against the streak
    // sections of the same series. Fails under M-C,
    // naming its parting series: the survey apex-crest
    // (longest run three, closing run one) and bolt-drift
    // (four, one), and the league bolt-crest (three, one)
    // and apex-bolt (two, one), each standing alone here
    // and strung there, so the alone count's standing at
    // or above the single count breaks in the cross-read
    // and the memberships move. Fails under M-A, naming:
    // the survey apex-bolt (decided closing run two, full
    // record none) and crest-drift (three against seven),
    // and the league apex-crest (three against none),
    // apex-ember (two against none), bolt-drift (three
    // against none), and drift-ember (three against
    // none). Fails under M-D, naming: the survey
    // apex-crest (opening run three, gaining a section),
    // the survey bolt-crest (opening run one, losing its
    // section), and the league crest-ember (opening run
    // one, losing its section). Fails under M-E, naming
    // the sections standing at exactly two: the survey
    // apex-bolt and apex-drift, and the league apex-ember.
    // The league call also carries DH-36's without reply
    // relation at its two without reply series, bolt-drift
    // and drift-ember, both standing in a run.
    check_relations(
        &load_example("survey-championship.json"),
        6,
        vec![2, 2, 3, 3],
        vec![1, 3, 4, 6],
        vec![2, 5],
        vec![],
    );
    check_relations(
        &load_example("league-championship.json"),
        10,
        vec![3, 2, 3, 3, 3, 3],
        vec![2, 4, 6, 7, 9, 10],
        vec![1, 3, 5],
        vec![6, 10],
    );
}

#[test]
fn the_section_figure_reads_the_closing_run() {
    // K-14. The plan's M-D discriminator at hand built
    // scale. A series at a wins target of five whose
    // decided winners run winner, loser, winner, winner,
    // loser, winner, winner: its winner's runs stand at
    // one, two, and two, its opening run at one, its
    // longest at two, and its closing run at two, so it
    // stands in a run with a section figure of two. Fails
    // under M-D, the opening run of one classing it alone
    // and its section vanishing. Fails under M-E, its
    // figure standing at exactly two. Passes under M-C by
    // design, its longest run standing at two as its
    // closing run does, the section and figure unchanged;
    // the M-C separation stands at K-13's scale, stated
    // here so no blanket claim covers this pin.
    let dir = temp_dir("closings-opening-run");
    let series = series_json(
        "the opening run saga",
        vec![ALPHA, BETA],
        5,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", BETA, ALPHA),
            clear_win("race six", ALPHA, BETA),
            clear_win("race seven", ALPHA, BETA),
        ],
    );
    let lines = closing_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "closing",
            "series 1, the opening run saga",
            "alpha-1 against beta-1, first to 5 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, alpha-1 wins",
            "race 4, race four, alpha-1 wins",
            "race 5, race five, beta-1 wins",
            "race 6, race six, alpha-1 wins",
            "race 7, race seven, alpha-1 wins",
            "closing run of 2 consecutive decided race wins",
            "series winner, alpha-1, alpha-1 5 race wins, beta-1 2 race wins, 0 no results",
            "closing, 1 series, 1 in a run, 0 alone, 0 no winner",
        ]
    );
}

/// The handicap twin of the closing pins. On scratch
/// finishes the series runs winner, winner, loser,
/// winner at target three, decided at the fourth race
/// with a closing run of one, and stands alone. The
/// third race's first model attempt carries an
/// adjustment that moves its finish ahead of the second
/// model's, so the adjusted record runs winner, winner,
/// winner, the series stands decided at the third race,
/// and its closing run stands at three.
fn handicap_series(adjusted: bool) -> JsonValue {
    let mut third_first = attempt(ALPHA, "green", 130.0, Some(10));
    if adjusted {
        third_first = set(&third_first, "adjustment", i(-30));
    }
    series_json(
        "the handicap saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", ALPHA, BETA),
            race_json(
                "race three",
                vec![third_first, attempt_two(BETA, "green", 110.0, Some(20))],
            ),
            clear_win("race four", ALPHA, BETA),
        ],
    )
}

#[test]
fn a_handicap_adjustment_moves_a_series_between_alone_and_in_a_run() {
    // K-15. The twin stands alone on its scratch record,
    // the report standing in the empty form, and in a run
    // with a section at a figure of three on its adjusted
    // record. Fails under the mutation that reads the
    // scratch winners in place of the adjusted record,
    // the posture of the Round 15 K-14 pin and the
    // FIX-DH-11 genus. The race figures stand inside the
    // DH-11 and DH-12 forms, an adjustment inside the
    // adjustment band moving a decided race across the
    // close call band, as the standing handicap pins
    // author theirs.
    let dir = temp_dir("closings-handicap");
    let plain = closing_for(
        &dir,
        vec![("one.json", handicap_series(false))],
        vec!["one.json"],
    );
    assert_eq!(
        plain,
        vec![
            "closing",
            "closing, 1 series, 0 in a run, 1 alone, 0 no winner",
        ]
    );
    let adjusted = closing_for(
        &dir,
        vec![("one.json", handicap_series(true))],
        vec!["one.json"],
    );
    assert_eq!(
        adjusted,
        vec![
            "closing",
            "series 1, the handicap saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, alpha-1 wins",
            "race 3, race three, alpha-1 wins",
            "closing run of 3 consecutive decided race wins",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 0 race wins, 0 no results",
            "closing, 1 series, 1 in a run, 0 alone, 0 no winner",
        ]
    );
}

#[test]
fn a_series_holding_no_winner_stands_in_neither_class() {
    // K-16. A series at a wins target of four whose
    // decided winners run winner, loser, winner, winner,
    // the target unreached, holds no winner and stands in
    // neither class, with no section, the closing
    // counting it as no winner, although the suffix run
    // of its first model over the decided record stands
    // at two. Fails under the champion drop mutation that
    // reads the first model as the winner of a series
    // holding no winner, in the Round 15 K-15 form, under
    // which the series stands in a run with a section.
    let dir = temp_dir("closings-open");
    let series = series_json(
        "the open saga",
        vec![ALPHA, BETA],
        4,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
        ],
    );
    let lines = closing_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "closing",
            "closing, 1 series, 0 in a run, 0 alone, 1 no winner",
        ]
    );
}

#[test]
fn a_series_decided_early_keeps_its_class_against_its_later_races() {
    // K-17. The plan's M-A discriminator at hand built
    // scale. A series at a wins target of two whose
    // decided winners run loser, winner, winner, decided
    // at the third race with a closing run of two,
    // standing in a run with a section at a figure of
    // two, and holding a fourth and fifth recorded race,
    // both won by the loser, beyond the decision. Fails
    // under M-A, the full record's suffix ending on the
    // loser's wins, so the winner's closing run over the
    // full record reads none, the series stands alone,
    // and its section vanishes. Fails under M-E, its
    // figure standing at exactly two.
    let dir = temp_dir("closings-beyond");
    let series = series_json(
        "the early saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", BETA, ALPHA),
            clear_win("race five", BETA, ALPHA),
        ],
    );
    let lines = closing_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "closing",
            "series 1, the early saga",
            "alpha-1 against beta-1, first to 2 wins",
            "race 1, race one, beta-1 wins",
            "race 2, race two, alpha-1 wins",
            "race 3, race three, alpha-1 wins",
            "closing run of 2 consecutive decided race wins",
            "series winner, alpha-1, alpha-1 2 race wins, beta-1 1 race wins, 0 no results",
            "closing, 1 series, 1 in a run, 0 alone, 0 no winner",
        ]
    );
}
