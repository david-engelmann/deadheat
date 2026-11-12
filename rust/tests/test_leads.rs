//! Pins for DH-42 and DH-43 in the Round 20 section of
//! CONTRACT.md.
//!
//! The discrimination record of this file stands pin by
//! pin, and no blanket claim stands over it. The lead
//! reading, its library function, and its command do
//! not stand on the pre round tree, so every pin in
//! this file that names the lead reading is absent
//! there at compile time. That absence is recorded
//! once here and is not a discrimination claim. The
//! discrimination each pin carries is against the
//! stated mutation of DH-42 beside it. Eight mutations
//! are named for the whole file. M-A, the walk does
//! not stop, the decided record read whole to the end
//! of the series file and the margin read at its end.
//! M-B, the winner's figure, the largest lead of the
//! series read as the winner's largest lead alone, the
//! loser's lead never consulted. M-C, the loser's
//! figure, the largest lead read as the loser's
//! largest lead alone. M-D, the margin printed, the
//! section figure printing the winning margin where
//! the largest lead stands. M-E, the boundary moved,
//! the class read as wide where the largest lead
//! stands at or above the margin. M-F, the runs
//! figure, the largest lead read as the count of the
//! maximal runs of the decided record, the statistic
//! DH-40 counts changes of hand over. The classing
//! mutation, a series holding no decided race before
//! its deciding race standing unclassed, its counts
//! leaving the closing line. The champion drop
//! mutation, a series holding no winner classed by
//! the decided record of the model holding the most
//! decided wins of the series. The eight constructed
//! sibling pins K-5 to K-12 pass on the pre round tree
//! by design, as their own comments state, and pass
//! under M-A to M-F by design, the mutations touching
//! the leads module alone.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{championship_report, load_championship_file, Championship};
use deadheat::closings::closing_report;
use deadheat::clusters::cluster_report;
use deadheat::comebacks::comeback_report;
use deadheat::deadlocks::deadlock_report;
use deadheat::distances::distance_report;
use deadheat::json::JsonValue;
use deadheat::leads::lead_report;
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

fn lead_for(dir: &Path, files: Vec<(&str, JsonValue)>, names: Vec<&str>) -> Vec<String> {
    lead_report(&load(dir, files, names))
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
fn the_standing_championship_stands_narrow_in_every_series() {
    // K-1. The standing championship golden, no section
    // standing, closing "lead, 2 series, 0 wide, 2
    // narrow, 0 no winner", the pilot standing narrow
    // with a largest lead of two standing equal to its
    // winning margin of two and the panel standing
    // narrow with a largest lead of three standing
    // equal to its winning margin of three. Passes
    // under M-A by design, the standing report standing
    // byte-identical under the full record. Passes
    // under M-B, each winner's largest lead standing at
    // its margin, two and three. Passes under M-C, each
    // loser's largest lead standing at none, below
    // either margin, so both series stand narrow as
    // they stand. Passes under M-D by design, no
    // section standing for its figure to move in.
    // Fails under M-E, both series standing at equality
    // and moving to wide. Fails under M-F, the pilot's
    // three runs standing above its margin of two and
    // gaining it a section.
    let championship = load_example("championship.json");
    let lines = lead_report(&championship);
    assert_eq!(
        lines,
        vec!["lead", "lead, 2 series, 0 wide, 2 narrow, 0 no winner"]
    );
    assert_eq!(lines.join("\n") + "\n", golden("championship-lead.txt"));
}

#[test]
fn the_survey_championship_matches_its_lead_golden() {
    // K-2. The survey championship golden, carrying the
    // apex-crest section at the figure 3 and the
    // apex-drift section at the figure 2 in that order
    // and closing "lead, 6 series, 2 wide, 4 narrow, 0
    // no winner". Fails under M-A, the full records of
    // the survey series moving heights and margins the
    // walk stops short of. Fails under M-C, the
    // apex-crest series' height reading as its loser's
    // none and its section vanishing. Fails under M-D,
    // the two section figures printing their margins of
    // two and one against their heights of three and
    // two. Fails under M-E, the four narrow series
    // standing at equality and moving to wide. Fails
    // under M-F, the apex-drift series printing its
    // five runs against its height of two. Passes under
    // M-B by the stated coincidence, the two wide
    // series' winner figures standing at their heights,
    // three and two, and no narrow series' winner
    // figure standing above its margin.
    let championship = load_example("survey-championship.json");
    let lines = lead_report(&championship);
    assert!(lines.len() > 2, "the survey golden stands in section form");
    assert_eq!(section_offsets(&lines), vec![2, 3]);
    assert_eq!(section_figures(&lines), vec![3, 2]);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("survey-championship-lead.txt")
    );
}

#[test]
fn the_league_championship_matches_its_lead_golden() {
    // K-3. The league championship golden, carrying the
    // bolt-crest section alone at the figure 3 and
    // closing "lead, 10 series, 1 wide, 9 narrow, 0 no
    // winner". Fails under M-A, the full records of the
    // league series moving heights and margins the walk
    // stops short of. Fails under M-C, the bolt-crest
    // series' height reading as its loser's none and
    // its section vanishing. Fails under M-D, the
    // section figure printing its margin of one against
    // its height of three. Fails under M-E, the nine
    // narrow series standing at equality and moving to
    // wide. Fails under M-F, narrow series gaining
    // sections at their runs where their runs stand
    // above their margins. Passes under M-B by the
    // stated coincidence, the bolt-crest winner's
    // figure standing at its height of three and no
    // narrow series' winner figure standing above its
    // margin.
    let championship = load_example("league-championship.json");
    let lines = lead_report(&championship);
    assert!(lines.len() > 2, "the league golden stands in section form");
    assert_eq!(section_offsets(&lines), vec![5]);
    assert_eq!(section_figures(&lines), vec![3]);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("league-championship-lead.txt")
    );
}

#[test]
fn the_constructed_example_carries_the_first_series_alone() {
    // K-4. The constructed lead golden, the first
    // series in section form at the figure 3, closing
    // "lead, 2 series, 1 wide, 1 narrow, 0 no winner",
    // the first series standing wide on its loser's
    // height of three against its margin of one and the
    // second standing narrow at two equal to two.
    // Fails under M-B, the first series' winner figure
    // standing at one, at its margin, and its section
    // vanishing. Fails under M-D, the section figure
    // printing its margin of one against its height of
    // three. Fails under M-E, the second series
    // standing at equality and moving to wide. Fails
    // under M-F on the figure, the first series' two
    // runs printing against its height of three, and
    // the second series gaining a section at its three
    // runs. Passes under M-A by design, neither
    // constructed series holding a recorded race beyond
    // its decision. Passes under M-C by the stated
    // coincidence, the first series' height standing on
    // its loser's figure of three and the second
    // series' loser standing at none.
    let championship = load_example("constructed-lead.json");
    let lines = lead_report(&championship);
    assert_eq!(
        lines,
        vec![
            "lead",
            "series 1, the constructed lead drill one",
            "falcon-1 against heron-1, first to 4 wins",
            "race 1, lead drill one race one, heron-1 wins",
            "race 2, lead drill one race two, heron-1 wins",
            "race 3, lead drill one race three, heron-1 wins",
            "race 4, lead drill one race four, falcon-1 wins",
            "race 5, lead drill one race five, falcon-1 wins",
            "race 6, lead drill one race six, falcon-1 wins",
            "race 7, lead drill one race seven, falcon-1 wins",
            "largest lead 3 over the decided record",
            "series winner, falcon-1, falcon-1 4 race wins, heron-1 3 race wins, 0 no results",
            "lead, 2 series, 1 wide, 1 narrow, 0 no winner",
        ]
    );
    assert_eq!(lines.join("\n") + "\n", golden("constructed-lead-lead.txt"));
}

#[test]
fn the_constructed_example_stands_strung_under_streak() {
    // K-5. The same file under the streak command, both
    // series in section form, closing "streak, 2
    // series, 2 strung, 0 single, 0 no winner". Passes
    // on the pre round tree by design and under M-A to
    // M-F by design, as the header of this file states
    // for the sibling pins.
    let championship = load_example("constructed-lead.json");
    let lines = streak_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-lead-streak.txt")
    );
}

#[test]
fn the_constructed_example_stands_clustered_under_cluster() {
    // K-6. The same file under the cluster command, the
    // first series in section form, closing "cluster, 2
    // series, 1 clustered, 1 scattered, 0 no winner".
    // Passes on the pre round tree by design and under
    // M-A to M-F by design.
    let championship = load_example("constructed-lead.json");
    let lines = cluster_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-lead-cluster.txt")
    );
}

#[test]
fn the_constructed_example_stands_a_rally_under_rally() {
    // K-7. The same file under the rally command, the
    // first series in section form, closing "rally, 2
    // series, 1 rally, 1 wire to wire, 0 no winner".
    // Passes on the pre round tree by design and under
    // M-A to M-F by design.
    let championship = load_example("constructed-lead.json");
    let lines = rally_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-lead-rally.txt")
    );
}

#[test]
fn the_constructed_example_stands_deadlocked_under_deadlock() {
    // K-8. The same file under the deadlock command,
    // the first series in section form, closing
    // "deadlock, 2 series, 1 deadlocked, 1 clear, 0 no
    // winner". Passes on the pre round tree by design
    // and under M-A to M-F by design.
    let championship = load_example("constructed-lead.json");
    let lines = deadlock_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-lead-deadlock.txt")
    );
}

#[test]
fn the_constructed_example_stands_the_distance_under_distance() {
    // K-9. The same file under the distance command,
    // the first series in section form, closing
    // "distance, 2 series, 1 the distance, 0 without
    // reply, 1 in between, 0 no winner". Passes on the
    // pre round tree by design and under M-A to M-F by
    // design.
    let championship = load_example("constructed-lead.json");
    let lines = distance_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-lead-distance.txt")
    );
}

#[test]
fn the_constructed_example_stands_from_behind_under_comeback() {
    // K-10. The same file under the comeback command,
    // the first series in section form, closing
    // "comebacks, 2 series, 1 from behind, 1 from the
    // front". Passes on the pre round tree by design
    // and under M-A to M-F by design.
    let championship = load_example("constructed-lead.json");
    let lines = comeback_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-lead-comebacks.txt")
    );
}

#[test]
fn the_constructed_example_stands_in_a_run_under_closing() {
    // K-11. The same file under the closing command,
    // the first series in section form, closing
    // "closing, 2 series, 1 in a run, 1 alone, 0 no
    // winner". Passes on the pre round tree by design
    // and under M-A to M-F by design.
    let championship = load_example("constructed-lead.json");
    let lines = closing_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-lead-closing.txt")
    );
}

#[test]
fn the_constructed_example_stands_in_a_run_under_opening() {
    // K-12. The same file under the opening command,
    // the second series in section form, closing
    // "opening, 2 series, 1 in a run, 0 alone, 1 from
    // behind, 0 no winner". Passes on the pre round
    // tree by design and under M-A to M-F by design.
    let championship = load_example("constructed-lead.json");
    let lines = opening_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-lead-opening.txt")
    );
}

#[test]
fn a_series_of_target_one_stands_narrow() {
    // K-13. The target one edge. A hand-built series of
    // target one decided by its only decided race
    // stands narrow, its winner's largest lead standing
    // at one, its loser's at none, and its margin at
    // one, its report carrying no section and closing
    // "lead, 1 series, 0 wide, 1 narrow, 0 no winner".
    // Fails under the classing mutation alone among the
    // named mutations besides M-E, the lone-decider
    // series leaving both classes and the closing
    // counting it nowhere. Fails under M-E, its height
    // of one standing at its margin of one and moving
    // to wide. Passes under M-A to M-D and M-F by
    // design, its record holding one race, so no figure
    // moves, its winner's figure standing at one as its
    // loser's stands at none and its runs at one.
    let dir = temp_dir("lead-target-one");
    let series = series_json(
        "the target one lead saga",
        vec![ALPHA, BETA],
        1,
        vec![clear_win("race one", ALPHA, BETA)],
    );
    let lines = lead_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec!["lead", "lead, 1 series, 0 wide, 1 narrow, 0 no winner"]
    );
}

#[test]
fn an_interposed_no_result_leaves_the_lead_whole() {
    // K-14. The decided record read whole. A hand-built
    // series at a target of three whose record
    // interposes two races holding no winner among four
    // counted decided races won by alpha, beta, alpha,
    // alpha, with a fifth decided race won by beta
    // standing beyond the decision, stands narrow with
    // no section, its winner's largest lead standing at
    // two, its loser's at none, its height at two, and
    // its margin at two, and the turns report over the
    // same championship closes turned at the figure
    // two, proving the interposed races left the
    // decided record the lead reading reads whole and
    // in order. Fails under M-A, the full record
    // tallying three against two past the decision,
    // its margin falling to one and its height of two
    // standing wide. Fails under M-E, its height
    // standing at its margin and moving to wide. Fails
    // under M-F, its three runs standing above its
    // margin of two and gaining it a section. Passes
    // under M-B, its winner's figure standing at two as
    // its height does. Passes under M-C, its loser's
    // figure standing at none. Passes under M-D by
    // design, no section standing for its figure to
    // move in.
    let dir = temp_dir("lead-interposed");
    let series = series_json(
        "the interposed lead saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            no_result("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            no_result("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
            clear_win("race six", ALPHA, BETA),
            clear_win("race seven", BETA, ALPHA),
        ],
    );
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    let lines = lead_report(&championship);
    assert_eq!(
        lines,
        vec!["lead", "lead, 1 series, 0 wide, 1 narrow, 0 no winner"]
    );
    let turns = turns_report(&championship);
    assert_eq!(
        turns,
        vec![
            "turns",
            "series 1, the interposed lead saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, alpha-1 wins",
            "race 3, race three, beta-1 wins",
            "race 5, race five, alpha-1 wins",
            "race 6, race six, alpha-1 wins",
            "changes of hand 2 over the decided record",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 1 race wins, 2 no results",
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

/// The figures of the sections of one lead report, in
/// section order.
fn section_figures(lines: &[String]) -> Vec<u64> {
    lines
        .iter()
        .filter(|line| line.starts_with("largest lead "))
        .map(|line| {
            line.split(' ')
                .nth(2)
                .expect("a figure")
                .parse::<u64>()
                .expect("a figure")
        })
        .collect()
}

/// The stated relations of DH-42 and DH-43 over one
/// championship, read from the printed readings alone.
/// The section figures and memberships stand stated by
/// the caller, and the series standing without reply
/// under DH-23 stand named by their championship
/// offsets, each standing narrow under this reading by
/// DH-42's stated consequence.
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
    let lead = lead_report(championship);
    let counts = closing_counts(lead.last().expect("a closing"));
    assert_eq!(counts[0], series_count);
    assert_eq!(counts[1] + counts[2], decided, "wide and narrow");
    assert_eq!(counts[1] + counts[2] + counts[3], series_count);
    assert_eq!(
        section_figures(&lead),
        figures,
        "the section figures in order"
    );
    let sections = section_offsets(&lead);
    assert_eq!(sections, sections_expected, "the section memberships");
    assert_eq!(
        sections.len() as u64,
        counts[1],
        "one section per wide series"
    );
    // The no winner count equals the figures of the
    // nine sibling closings DH-43 names, the comeback
    // leftover among them, because the ten readings
    // read one walk.
    let turns = turns_report(championship);
    let turns_counts = closing_counts(turns.last().expect("a closing"));
    assert_eq!(counts[3], turns_counts[3], "the turns no winner figure");
    let distance = distance_report(championship);
    let distance_counts = closing_counts(distance.last().expect("a closing"));
    assert_eq!(
        counts[3], distance_counts[4],
        "the distance no winner figure"
    );
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
    // Every series standing in a lead section stands
    // in a cluster section over the same championship,
    // every wide series standing clustered under
    // DH-34.
    let cluster_sections = section_offsets(&cluster);
    for offset in &sections {
        assert!(
            cluster_sections.contains(offset),
            "series {offset} stands clustered"
        );
    }
    // Every named series standing without reply
    // stands narrow, its loser holding no decided win,
    // so it stands in no lead section, and the distance
    // closing counts them as the caller names them.
    assert_eq!(
        distance_counts[2],
        without_reply.len() as u64,
        "the without reply count"
    );
    for offset in without_reply {
        assert!(
            !sections.contains(&offset),
            "series {offset} stands in no lead section"
        );
    }
    // The count of narrow series stands at or above
    // the count of single series in the streak
    // closing, every single series standing narrow.
    assert!(
        counts[2] >= streak_counts[2],
        "the narrow count stands at or above the single count"
    );
}

#[test]
fn the_lead_counts_stand_in_their_stated_relations() {
    // K-15. The relations pin, asserting over the
    // survey and league championships test by test the
    // relations DH-42 and DH-43 state, the wide and
    // narrow counts and the no winner count summing to
    // the championship report's series count over the
    // same files, the no winner count standing equal
    // across the sibling closings at the grain those
    // closings print it, every series standing in a
    // lead section standing in a cluster section over
    // the same championship, the series standing
    // without reply standing in no lead section, named
    // by their championship offsets, and the count of
    // narrow series standing at or above the count of
    // single series in the streak closing. Fails under
    // M-A, the full records moving the closing counts
    // the pin sums. Fails under M-C, the wide series of
    // both fixtures standing on heights their losers do
    // not hold, their sections vanishing from the
    // memberships the pin reads. Fails under M-D, the
    // section figures printing their margins against
    // the figures the pin states. Fails under M-E, the
    // narrow series of both fixtures standing at
    // equality and moving to wide, the closing counts
    // and memberships moving with them. Fails under
    // M-F, the apex-drift section printing its five
    // runs against its stated figure of two. Passes
    // under M-B by the stated coincidence K-2 and K-3
    // name, the wide series' winner figures standing at
    // their heights over both fixtures.
    check_relations(
        &load_example("survey-championship.json"),
        6,
        vec![3, 2],
        vec![2, 3],
        vec![],
    );
    check_relations(
        &load_example("league-championship.json"),
        10,
        vec![3],
        vec![5],
        vec![6, 10],
    );
}

#[test]
fn a_winners_fading_lead_still_stands_wide() {
    // K-16. The winner's fade at hand scale. A
    // hand-built series at a target of four won by
    // alpha, alpha, alpha, beta, beta, alpha, its
    // winner's lead rising to three and falling to one
    // before the decision, stands wide in section form
    // at the figure three, its margin standing at two,
    // closing "lead, 1 series, 1 wide, 0 narrow, 0 no
    // winner". Fails under M-C, its loser's figure
    // standing at none and its section vanishing.
    // Fails under M-D, its figure printing its margin
    // of two against its height of three. Passes under
    // M-B by design, its height standing on its
    // winner's figure of three. Passes under M-E by
    // design, its height standing above its margin on
    // either side of the boundary. Passes under M-F by
    // the stated coincidence, its three runs standing
    // at its height of three. Passes under M-A by
    // design, no race standing beyond its decision.
    let dir = temp_dir("lead-fade");
    let series = series_json(
        "the fading lead saga",
        vec![ALPHA, BETA],
        4,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", BETA, ALPHA),
            clear_win("race five", BETA, ALPHA),
            clear_win("race six", ALPHA, BETA),
        ],
    );
    let lines = lead_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "lead",
            "series 1, the fading lead saga",
            "alpha-1 against beta-1, first to 4 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, alpha-1 wins",
            "race 3, race three, alpha-1 wins",
            "race 4, race four, beta-1 wins",
            "race 5, race five, beta-1 wins",
            "race 6, race six, alpha-1 wins",
            "largest lead 3 over the decided record",
            "series winner, alpha-1, alpha-1 4 race wins, beta-1 2 race wins, 0 no results",
            "lead, 1 series, 1 wide, 0 narrow, 0 no winner",
        ]
    );
}

#[test]
fn a_series_decided_early_keeps_its_class_against_its_later_races() {
    // K-17. Beyond the decision at hand scale. A
    // hand-built series at a target of three won by
    // alpha, alpha, beta, alpha, with two further
    // decided races won by beta standing beyond the
    // decision in the same series file, stands narrow
    // with no section and closing "lead, 1 series, 0
    // wide, 1 narrow, 0 no winner", the beyond races
    // moving neither the height nor the margin the
    // reading reads. Fails under M-A, the full record
    // tallying three against three, its margin falling
    // to none and its height of two standing wide.
    // Fails under M-E, its height standing at its
    // margin and moving to wide. Fails under M-F, its
    // three runs standing above its margin of two and
    // gaining it a section at the figure three. Passes
    // under M-B, its winner's figure standing at two as
    // its height does. Passes under M-C, its loser's
    // figure standing at none. Passes under M-D by
    // design, no section standing for its figure to
    // move in.
    let dir = temp_dir("lead-beyond");
    let series = series_json(
        "the early lead saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", BETA, ALPHA),
            clear_win("race six", BETA, ALPHA),
        ],
    );
    let lines = lead_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec!["lead", "lead, 1 series, 0 wide, 1 narrow, 0 no winner"]
    );
}

/// One race of the handicap twin on identical
/// recorded finishes, beta standing ahead of alpha on
/// scratch, its finish of 110 standing against
/// alpha's 130, and an adjustment of 40 standing on
/// beta's attempt in the adjusted arm, moving its
/// finish behind alpha's and returning the race to
/// alpha.
fn beta_first(task: &str, adjusted: bool) -> JsonValue {
    let mut second = attempt_two(BETA, "green", 110.0, Some(20));
    if adjusted {
        second = set(&second, "adjustment", i(40));
    }
    race_json(task, vec![attempt(ALPHA, "green", 130.0, Some(10)), second])
}

/// The handicap twin of the lead pins. On scratch
/// finishes beta takes the first two decided races and
/// alpha the last three at a target of three, alpha's
/// winner standing wide on beta's height of two
/// against its margin of one. Under adjustments on
/// beta's first two attempts the same finishes return
/// those races to alpha, whose record runs three
/// straight, standing narrow at a height of three
/// equal to its margin of three.
fn handicap_series(adjusted: bool) -> JsonValue {
    series_json(
        "the handicap lead saga",
        vec![ALPHA, BETA],
        3,
        vec![
            beta_first("race one", adjusted),
            beta_first("race two", adjusted),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
        ],
    )
}

#[test]
fn a_handicap_adjustment_moves_a_series_between_wide_and_narrow() {
    // K-18. The handicap restatement. The twin pair
    // over identical recorded finishes stands wide in
    // section form at the figure two on its scratch
    // arm, its winner alpha having lost the first two
    // decided races to beta at a target of three, and
    // narrow with no section on its adjusted arm, the
    // same finishes under adjustments returning the
    // first two races to alpha, the pair proving the
    // classification reads the adjusted record the walk
    // holds and not the recorded one. Fails under M-B,
    // the scratch arm's winner figure standing at one,
    // at its margin, and its section vanishing. Fails
    // under M-D, the scratch section printing its
    // margin of one against its height of two. Fails
    // under M-E, the adjusted arm's height of three
    // standing at its margin of three and moving to
    // wide. Passes under M-C by the stated coincidence,
    // the scratch height standing on the loser's figure
    // of two and the adjusted loser's figure standing
    // at none. Passes under M-F by the stated
    // coincidence, the scratch record's two runs
    // standing at its height of two and the adjusted
    // record's one run standing below its margin.
    // Passes under M-A by design, no race standing
    // beyond either arm's decision, the adjusted arm
    // standing narrow under the full record as well,
    // its height and its margin rising together to
    // five.
    let dir = temp_dir("lead-handicap");
    let plain = lead_for(
        &dir,
        vec![("one.json", handicap_series(false))],
        vec!["one.json"],
    );
    assert_eq!(
        plain,
        vec![
            "lead",
            "series 1, the handicap lead saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, beta-1 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, alpha-1 wins",
            "race 4, race four, alpha-1 wins",
            "race 5, race five, alpha-1 wins",
            "largest lead 2 over the decided record",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 2 race wins, 0 no results",
            "lead, 1 series, 1 wide, 0 narrow, 0 no winner",
        ]
    );
    let adjusted = lead_for(
        &dir,
        vec![("one.json", handicap_series(true))],
        vec!["one.json"],
    );
    assert_eq!(
        adjusted,
        vec!["lead", "lead, 1 series, 0 wide, 1 narrow, 0 no winner"]
    );
}

#[test]
fn a_championship_holding_no_winner_stands_in_no_class() {
    // K-19. The no winner pin. A hand-built
    // championship of two series holding no winner, the
    // first holding decided races at a target of four
    // standing two to one with its target unreached,
    // the second holding no decided race, carries no
    // section and closes "lead, 2 series, 0 wide, 0
    // narrow, 2 no winner". Fails under the champion
    // drop mutation alone, which classes the first
    // series by its plurality leader's record, that
    // record standing wide at a height of two against
    // a margin of one and gaining it a section. Passes
    // under M-A to M-F and the classing mutation by
    // design, neither series holding a winner for any
    // of them to class, and neither holding a deciding
    // race for the classing mutation to reach.
    let dir = temp_dir("lead-open");
    let first = series_json(
        "the open lead saga",
        vec![ALPHA, BETA],
        4,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", BETA, ALPHA),
        ],
    );
    let second = series_json(
        "the silent lead saga",
        vec![GAMMA, DELTA],
        3,
        vec![no_result("race one", GAMMA, DELTA)],
    );
    let lines = lead_for(
        &dir,
        vec![("one.json", first), ("two.json", second)],
        vec!["one.json", "two.json"],
    );
    assert_eq!(
        lines,
        vec!["lead", "lead, 2 series, 0 wide, 0 narrow, 2 no winner"]
    );
}

#[test]
fn a_collection_holding_no_series_renders_the_empty_form_at_the_library_surface() {
    // No championship file carries an empty collection,
    // because the loader of DH-13 refuses a series list
    // holding no names, so this form stands at the library
    // surface alone, over a collection built by hand.
    let championship = Championship { series: vec![] };
    let lines = lead_report(&championship);
    assert_eq!(
        lines,
        vec!["lead", "lead, 0 series, 0 wide, 0 narrow, 0 no winner"]
    );
}
