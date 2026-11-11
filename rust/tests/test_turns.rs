//! Pins for DH-40 and DH-41 in the Round 19 section of
//! CONTRACT.md.
//!
//! The discrimination record of this file stands pin by
//! pin, and no blanket claim stands over it. The turns
//! reading, its library function, and its command do not
//! stand on the pre round tree, so every pin in this file
//! that names the turns reading is absent there at
//! compile time. That absence is recorded once here and
//! is not a discrimination claim. The discrimination
//! each pin carries is against the stated mutation of
//! DH-40 beside it. Eight mutations are named for the
//! whole file. M-A, the walk does not stop, the count of
//! changes read over every recorded race of the series.
//! M-B, the counted adjacency, the count read over the
//! counted races, two counted races standing beside
//! each other only when adjacent among the counted
//! races. M-C, the streak statistic, DH-32's statistic
//! rendered in this reading's form. M-D, the closing
//! statistic, DH-36's statistic rendered in this
//! reading's form. M-E, the boundary moved, a series
//! standing turned at three or more changes of hand.
//! M-F, the runs figure, the figure of each section read
//! as the count of the series' runs. The classing
//! mutation, classing only a series holding a decided
//! race before its deciding race. The champion drop
//! mutation, classing a series holding no winner by the
//! decided record of the model holding the most decided
//! wins. The eight constructed sibling pins K-4 to K-11
//! pass on the pre round tree by design, as their own
//! comments state, and pass under M-A to M-F by design,
//! the mutations touching the turns module alone.

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
use deadheat::turns::turns_report;
use shared::*;

const ALPHA: &str = "alpha-1";
const BETA: &str = "beta-1";
const GAMMA: &str = "gamma-1";
const DELTA: &str = "delta-1";

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

fn turns_for(dir: &Path, files: Vec<(&str, JsonValue)>, names: Vec<&str>) -> Vec<String> {
    turns_report(&load(dir, files, names))
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
fn the_standing_championship_carries_the_pilot_section_alone() {
    // K-1. The standing championship golden, the pilot's
    // section alone at a figure of two, closing "turns,
    // 2 series, 1 turned, 1 steady, 0 no winner". Fails
    // under M-B as executed, the design note's narrative
    // for this pin standing superseded by execution. The
    // pilot's counted record stands at nine counted races
    // with five no results, its decided races standing at
    // races 1, 5, 7, and 9, no two decided races standing
    // adjacent among the counted races, so the counted
    // figure stands at none and the pilot's section
    // vanishes under M-B, the mutant closing "turns, 2
    // series, 0 turned, 2 steady, 0 no winner". M-B is
    // caught by K-1, K-2, K-13, K-15, E-1, E-2, and E-3.
    // Fails under M-C, the panel
    // gaining a section at its longest run of three.
    // Fails under M-D, the pilot's section vanishing at
    // its closing run of one and the panel gaining its
    // own at three. Fails under M-E, the pilot standing
    // at exactly two and losing its section. Fails under
    // M-F on the figure, the pilot's three runs printing
    // against its two changes. M-A is not stated for this
    // pin: the standing series' races beyond the decision
    // stand under the panel and adjudication rules, whose
    // full records this plan does not re-derive; M-A is
    // carried at hand-built scale by K-17 and at fixture
    // scale by the red team's charge in section 4.
    let championship = load_example("championship.json");
    let lines = turns_report(&championship);
    assert_eq!(
        lines,
        vec![
            "turns",
            "series 1, the recorded close call pilot",
            "prime-1 against shadow-1, first to 3 wins",
            "race 1, pin the cache key, prime-1 wins",
            "race 5, name the worker pool, prime-1 wins",
            "race 7, shrink the prompt budget, shadow-1 wins",
            "race 9, sort the seed list, prime-1 wins",
            "changes of hand 2 over the decided record",
            "series winner, prime-1, prime-1 3 race wins, shadow-1 1 race wins, 5 no results",
            "turns, 2 series, 1 turned, 1 steady, 0 no winner",
        ]
    );
    assert_eq!(lines.join("\n") + "\n", golden("championship-turns.txt"));
}

#[test]
fn the_survey_championship_matches_its_turns_golden() {
    // K-2. The survey championship golden, five sections
    // at figures of two, two, four, two, and three,
    // closing "turns, 6 series, 5 turned, 1 steady, 0 no
    // winner". Fails under M-B, the apex-bolt series'
    // counted figure standing at none and its section
    // vanishing. Fails under M-C, the figures of the
    // apex-crest, apex-drift, bolt-crest, and bolt-drift
    // sections moving to their longest runs of three,
    // two, three, and four, and the crest-drift series
    // gaining a section at three. Fails under M-D, the
    // apex-crest and bolt-drift sections vanishing at
    // closing runs of one, the apex-drift and bolt-crest
    // figures moving to two and three, and the
    // crest-drift series gaining a section at three.
    // Fails under M-E, three sections standing at exactly
    // two and vanishing. Fails under M-F on every figure,
    // each section printing its runs. M-A is not stated
    // for this pin, for the reason K-1 states in its own
    // form; its carriers stand as K-1 names.
    let championship = load_example("survey-championship.json");
    let lines = turns_report(&championship);
    assert!(lines.len() > 2, "the survey golden stands in section form");
    assert_eq!(
        lines.join("\n") + "\n",
        golden("survey-championship-turns.txt")
    );
}

#[test]
fn the_constructed_example_carries_the_first_series_alone() {
    // K-3. The constructed turns golden, the first
    // series' section alone at a figure of two, closing
    // "turns, 2 series, 1 turned, 1 steady, 0 no
    // winner". Fails under M-C, the second series gaining
    // a section at its longest run of three. Fails under
    // M-D, the second series gaining a section at its
    // closing run of three. Fails under M-E, the first
    // series standing at exactly two and losing its
    // section. Fails under M-F on the figure, the first
    // series' three runs printing against its two
    // changes. Passes under M-A by design, neither
    // constructed series holding a recorded race beyond
    // its decision, as the examples entry states their
    // records. Passes under M-B by design, the
    // constructed records carrying no no results at all;
    // the M-B carriers stand at K-1, K-2, K-13, and
    // K-15, with E-1, E-2, and E-3 in the examples
    // target, as K-1 names in full.
    let championship = load_example("constructed-turns.json");
    let lines = turns_report(&championship);
    assert_eq!(
        lines,
        vec![
            "turns",
            "series 1, the constructed turns drill one",
            "alpha-1 against beta-1, first to 4 wins",
            "race 1, turns drill one race one, alpha-1 wins",
            "race 2, turns drill one race two, alpha-1 wins",
            "race 3, turns drill one race three, beta-1 wins",
            "race 4, turns drill one race four, alpha-1 wins",
            "race 5, turns drill one race five, alpha-1 wins",
            "changes of hand 2 over the decided record",
            "series winner, alpha-1, alpha-1 4 race wins, beta-1 1 race wins, 0 no results",
            "turns, 2 series, 1 turned, 1 steady, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-turns-turns.txt")
    );
}

#[test]
fn the_constructed_example_stands_strung_under_streak() {
    // K-4. The same file under the streak command, both
    // series in section form at figures of two and
    // three, closing "streak, 2 series, 2 strung, 0
    // single, 0 no winner". Passes on the pre round tree
    // by design and under M-A to M-F by design, as the
    // header of this file states for the sibling pins.
    let championship = load_example("constructed-turns.json");
    let lines = streak_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-turns-streak.txt")
    );
}

#[test]
fn the_constructed_example_stands_clustered_under_cluster() {
    // K-5. The same file under the cluster command, the
    // second series' section at a figure of two against,
    // closing "cluster, 2 series, 1 clustered, 1
    // scattered, 0 no winner". Passes on the pre round
    // tree by design and under M-A to M-F by design.
    let championship = load_example("constructed-turns.json");
    let lines = cluster_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-turns-cluster.txt")
    );
}

#[test]
fn the_constructed_example_stands_a_rally_under_rally() {
    // K-6. The same file under the rally command, the
    // second series' section, closing "rally, 2 series,
    // 1 rally, 1 wire to wire, 0 no winner". Passes on
    // the pre round tree by design and under M-A to M-F
    // by design.
    let championship = load_example("constructed-turns.json");
    let lines = rally_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-turns-rally.txt")
    );
}

#[test]
fn the_constructed_example_stands_deadlocked_under_deadlock() {
    // K-7. The same file under the deadlock command, the
    // second series' section, closing "deadlock, 2
    // series, 1 deadlocked, 1 clear, 0 no winner".
    // Passes on the pre round tree by design and under
    // M-A to M-F by design.
    let championship = load_example("constructed-turns.json");
    let lines = deadlock_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-turns-deadlock.txt")
    );
}

#[test]
fn the_constructed_example_stands_the_distance_under_distance() {
    // K-8. The same file under the distance command, the
    // second series' section, closing "distance, 2
    // series, 1 the distance, 0 without reply, 1 in
    // between, 0 no winner". Passes on the pre round tree
    // by design and under M-A to M-F by design.
    let championship = load_example("constructed-turns.json");
    let lines = distance_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-turns-distance.txt")
    );
}

#[test]
fn the_constructed_example_stands_from_behind_under_comeback() {
    // K-9. The same file under the comeback command, the
    // second series' section, closing "comebacks, 2
    // series, 1 from behind, 1 from the front". Passes on
    // the pre round tree by design and under M-A to M-F
    // by design.
    let championship = load_example("constructed-turns.json");
    let lines = comeback_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-turns-comebacks.txt")
    );
}

#[test]
fn the_constructed_example_stands_in_a_run_under_closing() {
    // K-10. The same file under the closing command,
    // both series in section form at figures of two and
    // three, closing "closing, 2 series, 2 in a run, 0
    // alone, 0 no winner". Passes on the pre round tree
    // by design and under M-A to M-F by design.
    let championship = load_example("constructed-turns.json");
    let lines = closing_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-turns-closing.txt")
    );
}

#[test]
fn the_constructed_example_stands_in_a_run_under_opening() {
    // K-11. The same file under the opening command, the
    // first series' section at a figure of two, closing
    // "opening, 2 series, 1 in a run, 0 alone, 1 from
    // behind, 0 no winner". Passes on the pre round tree
    // by design and under M-A to M-F by design.
    let championship = load_example("constructed-turns.json");
    let lines = opening_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-turns-opening.txt")
    );
}

#[test]
fn a_series_of_target_one_stands_steady() {
    // K-12. The target one pin. A hand-built championship
    // of one series at a wins target of one, decided by
    // its only decided race, stands steady: no section,
    // and the closing counts it steady. Fails under the
    // classing mutation, which drops the lone-decider
    // series out of both classes. Passes under M-A to
    // M-F by design, its record holding one race, so no
    // count moves, its longest run and its closing run
    // standing at one with its changes at none, and the
    // moved boundary not reaching a figure of none; its
    // teeth are the classing mutation named and DH-40's
    // target one sentence.
    let dir = temp_dir("turns-target-one");
    let series = series_json(
        "the target one turns saga",
        vec![ALPHA, BETA],
        1,
        vec![clear_win("race one", ALPHA, BETA)],
    );
    let lines = turns_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec!["turns", "turns, 1 series, 0 turned, 1 steady, 0 no winner"]
    );
}

#[test]
fn an_interposed_no_result_leaves_the_changes_whole() {
    // K-13. The interposed no result classification pin,
    // the plan's M-B discriminator at hand-built scale. A
    // hand-built series at a wins target of two whose
    // counted races run winner, no result, loser, winner:
    // its decided record runs winner, loser, winner, its
    // record changes hands twice, and it stands turned
    // with a section at a figure of two. Fails under M-B,
    // the no result standing between its first two
    // decided races removing their change, its counted
    // figure standing at one and its section vanishing.
    // Fails under M-C, its winner's longest run standing
    // at one, so the streak form classes it steady and
    // its section vanishes. Fails under M-D, its closing
    // run standing at one, so the closing form classes it
    // steady. Fails under M-E, its figure standing at
    // exactly two. Fails under M-F on the figure, its
    // three runs printing against its two changes. Passes
    // under M-A by design, its fixture holding no recorded
    // race beyond its decision; the M-A carrier stands at
    // K-17.
    let dir = temp_dir("turns-interposed-class");
    let series = series_json(
        "the interposed turns saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", ALPHA, BETA),
            no_result("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
        ],
    );
    let lines = turns_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "turns",
            "series 1, the interposed turns saga",
            "alpha-1 against beta-1, first to 2 wins",
            "race 1, race one, alpha-1 wins",
            "race 3, race three, beta-1 wins",
            "race 4, race four, alpha-1 wins",
            "changes of hand 2 over the decided record",
            "series winner, alpha-1, alpha-1 2 race wins, beta-1 1 race wins, 1 no results",
            "turns, 1 series, 1 turned, 0 steady, 0 no winner",
        ]
    );
}

#[test]
fn an_interposed_no_result_leaves_the_section_naming_decided_races_only() {
    // K-14. The interposed no result section pin. A
    // hand-built series at a wins target of five whose
    // counted races run winner, winner, no result,
    // winner, loser, winner, winner: its decided record
    // stands at six races changing hands twice, and its
    // section names its six decided races in race order,
    // its figure of two, and its split of five race wins,
    // one race win, and one no result. Passes under M-B
    // by design, its one interposed no result standing
    // between two decided races its winner won, so the
    // counted adjacencies that carry winners on both
    // sides are its last two pairs, both changes, and its
    // counted figure stands at two as its decided figure
    // does; this joint posture with K-13 is stated here
    // at design time, the two pins' gaps standing between
    // different winners in K-13 and between one winner's
    // wins in K-14, and the M-B carriers stand at
    // K-1, K-2, K-13, and K-15, with E-1, E-2, and E-3
    // in the examples target, as K-1 names in full.
    // Fails under M-C, its winner's
    // longest run standing at three against its figure of
    // two. Passes under M-D by design, its closing run
    // standing at two as its count of changes does, the
    // coincidence stated here at design time; the M-D
    // carriers stand at K-1, K-2, K-3, K-13, K-15,
    // K-16, and K-18, as executed. Fails
    // under M-E, its figure standing at exactly two.
    // Fails under M-F on the figure, its three runs
    // printing against its two changes. Passes under M-A
    // by design, no recorded race standing beyond its
    // decision.
    let dir = temp_dir("turns-interposed-section");
    let series = series_json(
        "the long interposed turns saga",
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
    let lines = turns_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "turns",
            "series 1, the long interposed turns saga",
            "alpha-1 against beta-1, first to 5 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, alpha-1 wins",
            "race 4, race four, alpha-1 wins",
            "race 5, race five, beta-1 wins",
            "race 6, race six, alpha-1 wins",
            "race 7, race seven, alpha-1 wins",
            "changes of hand 2 over the decided record",
            "series winner, alpha-1, alpha-1 5 race wins, beta-1 1 race wins, 1 no results",
            "turns, 1 series, 1 turned, 0 steady, 0 no winner",
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

/// The figures of the sections of one turns report, in
/// section order.
fn section_figures(lines: &[String]) -> Vec<u64> {
    lines
        .iter()
        .filter(|line| line.starts_with("changes of hand "))
        .map(|line| {
            line.split(' ')
                .nth(3)
                .expect("a figure")
                .parse::<u64>()
                .expect("a figure")
        })
        .collect()
}

/// The figure of the section standing at one offset in
/// one report, read from the figure line whose head the
/// caller names, or none where the series stands no
/// section in that report.
fn figure_at(lines: &[String], offset: u64, prefix: &str) -> Option<u64> {
    let mut current: Option<u64> = None;
    for line in lines {
        if let Some(head) = line.strip_prefix("series ") {
            if let Ok(value) = head.split(',').next().expect("a token").parse::<u64>() {
                current = Some(value);
            }
        }
        if line.starts_with(prefix) && current == Some(offset) {
            return line
                .split(' ')
                .nth(3)
                .and_then(|token| token.parse::<u64>().ok());
        }
    }
    None
}

/// The stated relations of DH-40 and DH-41 over one
/// championship, read from the printed readings alone.
/// The section figures and memberships stand stated by
/// the caller, and the series standing without reply
/// under DH-23 stand named by their championship
/// offsets, each standing steady under this reading by
/// DH-40's stated relation.
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
    assert_eq!(
        section_offsets(&table).len() as u64,
        series_count,
        "the championship series count"
    );
    let turns = turns_report(championship);
    let counts = closing_counts(turns.last().expect("a closing"));
    assert_eq!(counts[0], series_count);
    assert_eq!(counts[1] + counts[2], decided, "turned and steady");
    assert_eq!(counts[1] + counts[2] + counts[3], series_count);
    assert_eq!(
        section_figures(&turns),
        figures,
        "the section figures in order"
    );
    let sections = section_offsets(&turns);
    assert_eq!(sections, sections_expected, "the section memberships");
    assert_eq!(
        sections.len() as u64,
        counts[1],
        "one section per turned series"
    );
    // The no winner count equals the figures of the
    // eight sibling closings DH-41 names, the comeback
    // leftover among them, because the nine readings
    // read one walk.
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
    let closing = closing_report(championship);
    let closing_counts_vec = closing_counts(closing.last().expect("a closing"));
    assert_eq!(
        counts[3], closing_counts_vec[3],
        "the closing no winner figure"
    );
    let opening = opening_report(championship);
    let opening_counts = closing_counts(opening.last().expect("a closing"));
    assert_eq!(counts[3], opening_counts[4], "the opening no winner figure");
    // Every named series standing without reply stands
    // steady, its record changing hands not at all, so
    // it stands in no turns section, and the distance
    // closing counts them as the caller names them.
    assert_eq!(
        distance_counts[2],
        without_reply.len() as u64,
        "the without reply count"
    );
    for offset in without_reply {
        assert!(
            !sections.contains(&offset),
            "series {offset} stands in no turns section"
        );
    }
}

#[test]
fn the_turns_counts_stand_in_their_stated_relations() {
    // K-15. The relations pin, over the survey and
    // league championships test by test, in the form of
    // the Round 16 K-13 and Round 17 K-14: the closings
    // printed by the turns, streak, cluster, closing,
    // opening, championship, comeback, distance, rally,
    // and deadlock readings over each fixture are
    // cross-read through the shared test module's
    // closing counts helper, and the relations of DH-40
    // and DH-41 are named line by line, the section
    // figures of the turns report cross-read against
    // the streak and closing sections of the same
    // series. Fails under M-B, naming the league
    // crest-drift series: its counted record interposes
    // no results before its first decided race and
    // between its first and second, so its counted
    // figure stands at one against its decided figure
    // of two, the series moving from turned to steady
    // and the single relation of DH-40 breaking in the
    // cross-read, its record standing in strict
    // alternation at a target of two. Fails under M-C
    // on the figure cross-read, naming the survey
    // apex-crest series, whose turns figure stands at
    // two against its streak figure of three, and the
    // league crest-ember series, at two against three.
    // Fails under M-D on the figure cross-read, naming
    // the survey apex-drift series, whose turns figure
    // stands at four against its closing figure of
    // two. Fails under M-E, the series standing at
    // exactly two over both fixtures, the survey
    // apex-bolt, apex-crest, and bolt-crest and the
    // league apex-crest, bolt-crest, crest-drift, and
    // crest-ember, moving out of the turned class and
    // the turned counts the pin cross-reads falling
    // with them. Fails under M-F on the single arm's
    // figure, the league crest-drift series printing
    // three runs against the twice the target less two
    // figure of two DH-40 states for it. The pin also
    // carries DH-40's without reply relation at the
    // league's two without reply series, bolt-drift
    // and drift-ember, both standing steady at a
    // figure of none, its same-run relation at
    // hand-built scale in the pin's second arm, and
    // the no winner count's equality across all nine
    // closings over both fixtures. M-A is not stated
    // at fixture scale, for the reason K-1 states; the
    // red team's charge carries it.
    check_relations(
        &load_example("survey-championship.json"),
        6,
        vec![2, 2, 4, 2, 3],
        vec![1, 2, 3, 4, 5],
        vec![],
    );
    check_relations(
        &load_example("league-championship.json"),
        10,
        vec![3, 2, 3, 2, 3, 2, 2],
        vec![1, 2, 3, 5, 7, 8, 9],
        vec![6, 10],
    );
    // The figure cross-reads the mutation split names,
    // taken series by series over the two fixtures.
    let survey = load_example("survey-championship.json");
    let survey_turns = turns_report(&survey);
    assert_eq!(
        figure_at(&survey_turns, 2, "changes of hand "),
        Some(2),
        "the survey apex-crest turns figure"
    );
    assert_eq!(
        figure_at(&streak_report(&survey), 2, "longest run of "),
        Some(3),
        "the survey apex-crest streak figure"
    );
    assert_eq!(
        figure_at(&survey_turns, 3, "changes of hand "),
        Some(4),
        "the survey apex-drift turns figure"
    );
    assert_eq!(
        figure_at(&closing_report(&survey), 3, "closing run of "),
        Some(2),
        "the survey apex-drift closing figure"
    );
    let league = load_example("league-championship.json");
    let league_turns = turns_report(&league);
    assert_eq!(
        figure_at(&league_turns, 9, "changes of hand "),
        Some(2),
        "the league crest-ember turns figure"
    );
    assert_eq!(
        figure_at(&streak_report(&league), 9, "longest run of "),
        Some(3),
        "the league crest-ember streak figure"
    );
    // The single arm. The league crest-drift series
    // stands single under DH-32, the one single series
    // of the fixture, at a wins target of two, so
    // DH-40's second consequence stands read on it:
    // its record changes hands twice the target less
    // two times, a figure of two, and it stands
    // turned.
    let league_streak = streak_report(&league);
    assert!(
        !section_offsets(&league_streak).contains(&8),
        "the crest-drift series stands in no streak section"
    );
    let league_streak_counts = closing_counts(league_streak.last().expect("a closing"));
    assert_eq!(league_streak_counts[2], 1, "the one single series");
    let crest_head = league_turns
        .iter()
        .position(|line| line.starts_with("series 8,"))
        .expect("the crest-drift section");
    assert_eq!(
        league_turns[crest_head + 1],
        "crest-1 against drift-1, first to 2 wins"
    );
    assert_eq!(
        figure_at(&league_turns, 8, "changes of hand "),
        Some(2),
        "twice the target less two"
    );
    // The second arm, hand-built: one series at a wins
    // target of three whose decided winners run loser,
    // winner, winner, winner. Its winner's one run of
    // three stands as both the first and the last run
    // of its winner in DH-40's same-run sense, and the
    // series stands steady with one change of hand.
    // Under the owner's ruling on the design note, the
    // arm asserts the true edge postures: it stands
    // from behind under DH-38, its opening run
    // standing at none, and in a run only under DH-36,
    // its closing section standing at a figure of
    // three. Its M-F posture stands settled by
    // execution in the build record, not by this
    // comment, under the same ruling.
    let dir = temp_dir("turns-relations-arm");
    let series = series_json(
        "the same run turns saga",
        vec![GAMMA, DELTA],
        3,
        vec![
            clear_win("race one", GAMMA, DELTA),
            clear_win("race two", DELTA, GAMMA),
            clear_win("race three", DELTA, GAMMA),
            clear_win("race four", DELTA, GAMMA),
        ],
    );
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    let lines = turns_report(&championship);
    assert_eq!(
        lines,
        vec!["turns", "turns, 1 series, 0 turned, 1 steady, 0 no winner"]
    );
    let closing = closing_report(&championship);
    assert_eq!(
        figure_at(&closing, 1, "closing run of "),
        Some(3),
        "the closing section at figure three"
    );
    let closing_counts_arm = closing_counts(closing.last().expect("a closing"));
    assert_eq!(
        closing_counts_arm[1], 1,
        "the arm stands in a run at its closing"
    );
    let opening = opening_report(&championship);
    let opening_counts_arm = closing_counts(opening.last().expect("a closing"));
    assert_eq!(
        opening_counts_arm[3], 1,
        "the opening closing line counts it from behind"
    );
}

#[test]
fn the_section_figure_reads_the_changes_of_hand() {
    // K-16. The edge figure pin, the plan's M-D
    // discriminator at hand-built scale beside its M-C
    // teeth. A hand-built series at a wins target of
    // four whose decided winners run winner, winner,
    // loser, winner, loser, winner: its record changes
    // hands four times, its opening run stands at two,
    // its longest at two, and its closing run at one,
    // so it stands turned with a section at a figure
    // of four. Fails under M-D, its closing run of one
    // classing it steady in the closing form and its
    // section vanishing. Fails under M-C on the
    // figure, its longest run of two printing against
    // its four changes. Fails under M-F on the figure,
    // its five runs printing against its four changes.
    // Passes under M-E by design, its figure of four
    // standing on the marked side of either boundary.
    // Passes under M-B and M-A by design, its record
    // carrying no no results and no race beyond its
    // decision.
    let dir = temp_dir("turns-edge-figure");
    let series = series_json(
        "the edge figure turns saga",
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
    let lines = turns_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "turns",
            "series 1, the edge figure turns saga",
            "alpha-1 against beta-1, first to 4 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, alpha-1 wins",
            "race 3, race three, beta-1 wins",
            "race 4, race four, alpha-1 wins",
            "race 5, race five, beta-1 wins",
            "race 6, race six, alpha-1 wins",
            "changes of hand 4 over the decided record",
            "series winner, alpha-1, alpha-1 4 race wins, beta-1 2 race wins, 0 no results",
            "turns, 1 series, 1 turned, 0 steady, 0 no winner",
        ]
    );
}

#[test]
fn a_series_decided_early_keeps_its_figure_against_its_later_races() {
    // K-17. The beyond the decision pin, the plan's
    // M-A carrier at hand-built scale. A hand-built
    // series at a wins target of three whose recorded
    // races run winner, loser, winner, winner, and
    // then loser and loser standing beyond the
    // decision: its decided record changes hands
    // twice and it stands turned with a section at a
    // figure of two naming its four decided races.
    // Fails under M-A, the full record changing hands
    // three times and its figure moving to three.
    // Fails under M-E, its figure standing at exactly
    // two. Fails under M-F on the figure, its three
    // runs printing against its two changes. Passes
    // under M-C by design, its winner's longest run
    // standing at two as its count of changes does,
    // the coincidence stated here at design time; the
    // M-C carriers stand at K-1, K-2, K-3, K-13, K-14,
    // K-15, K-16, and K-18. Passes under M-D by
    // design, its closing run standing at two as its
    // count of changes does, the same coincidence at
    // the other edge; the M-D carriers stand as K-14
    // names. Passes under M-B by design, its record
    // carrying no no results.
    let dir = temp_dir("turns-beyond");
    let series = series_json(
        "the early turns saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", BETA, ALPHA),
            clear_win("race six", BETA, ALPHA),
        ],
    );
    let lines = turns_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "turns",
            "series 1, the early turns saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, alpha-1 wins",
            "race 4, race four, alpha-1 wins",
            "changes of hand 2 over the decided record",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 1 race wins, 0 no results",
            "turns, 1 series, 1 turned, 0 steady, 0 no winner",
        ]
    );
}

/// The handicap twin of the turns pins. On scratch
/// finishes the series runs winner, loser, winner at
/// a wins target of two, its record changing hands
/// twice. The second race's scratch winner carries an
/// adjustment of 40 that moves its finish behind the
/// first model's, so the adjusted record stands the
/// second race to the winner and the series stands
/// steady.
fn handicap_series(adjusted: bool) -> JsonValue {
    let mut second_second = attempt_two(BETA, "green", 110.0, Some(20));
    if adjusted {
        second_second = set(&second_second, "adjustment", i(40));
    }
    series_json(
        "the handicap turns saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", ALPHA, BETA),
            race_json(
                "race two",
                vec![attempt(ALPHA, "green", 130.0, Some(10)), second_second],
            ),
            clear_win("race three", ALPHA, BETA),
        ],
    )
}

#[test]
fn a_handicap_adjustment_moves_a_series_between_turned_and_steady() {
    // K-18. The handicap pin. The twin stands turned
    // with a section at a figure of two on its scratch
    // record, and steady with no section on its
    // adjusted record, the classification reading the
    // adjusted record the walk holds. Fails under
    // M-C, the adjusted record's longest run of three
    // standing it a section in the streak form. Fails
    // under M-D, its adjusted closing run of three
    // standing it a section in the closing form.
    // Fails under the mutation that reads the stated
    // finishes and not the adjusted, its stated record
    // changing hands twice and standing it turned.
    // Fails under M-E and M-F as executed through
    // its scratch arm, the scratch record standing at
    // exactly two and losing its section to the moved
    // boundary under M-E, and its figure of two
    // printing as three runs under M-F. Its adjusted
    // arm passes under both by design, verified by
    // direct mutant-command execution, standing steady
    // with no section under each, its adjusted record
    // carrying no no results, its class standing steady
    // on either side of the moved boundary, and no
    // section standing for its figure to move in.
    // Passes under M-B by design, and its classing and
    // champion-drop postures stand as designed, as its
    // M-C, M-D, and stated-finishes postures above do.
    let dir = temp_dir("turns-handicap");
    let plain = turns_for(
        &dir,
        vec![("one.json", handicap_series(false))],
        vec!["one.json"],
    );
    assert_eq!(
        plain,
        vec![
            "turns",
            "series 1, the handicap turns saga",
            "alpha-1 against beta-1, first to 2 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, alpha-1 wins",
            "changes of hand 2 over the decided record",
            "series winner, alpha-1, alpha-1 2 race wins, beta-1 1 race wins, 0 no results",
            "turns, 1 series, 1 turned, 0 steady, 0 no winner",
        ]
    );
    let adjusted = turns_for(
        &dir,
        vec![("one.json", handicap_series(true))],
        vec!["one.json"],
    );
    assert_eq!(
        adjusted,
        vec!["turns", "turns, 1 series, 0 turned, 1 steady, 0 no winner"]
    );
}

#[test]
fn a_series_holding_no_winner_stands_in_no_class() {
    // K-19. The no winner pin, in the posture of
    // Round 17's K-18. A hand-built series at a wins
    // target of four whose decided winners run
    // winner, winner, loser, winner, its target
    // unreached, stands in neither class: no section,
    // and the closing counts it among the series
    // holding no winner. Fails under the
    // champion-drop mutation, which classes it by its
    // plurality leader's record, that record changing
    // hands twice and standing it a section at a
    // figure of two against the pin's expected
    // report. Passes under the six named mutations by
    // design as the pin's own statement records, its
    // class standing in neither class under each for
    // the reason its statement gives; its teeth are
    // the champion-drop mutation named and DH-40's
    // no winner sentence.
    let dir = temp_dir("turns-open");
    let series = series_json(
        "the open turns saga",
        vec![ALPHA, BETA],
        4,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
        ],
    );
    let lines = turns_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec!["turns", "turns, 1 series, 0 turned, 0 steady, 1 no winner"]
    );
}
