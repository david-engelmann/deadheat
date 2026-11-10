//! Pins for DH-34 and DH-35 in the Round 15 section of
//! CONTRACT.md.
//!
//! The discrimination record of this file stands pin by
//! pin, and no blanket claim stands over it. The cluster
//! reading, its library function, and its command do not
//! stand on the pre round tree, so every pin in this file
//! is absent there at compile time. That absence is
//! recorded once here and is not a discrimination claim.
//! The discrimination each pin carries is against the
//! stated mutation of DH-34 beside it. Five mutations are
//! named for the whole file. M-A, the early stop removed,
//! the runs and the sections read over every recorded
//! race, races beyond the decision among them. M-B, a no
//! result race breaking a run. M-C, the winner's longest
//! run read in place of the loser's. M-D, either model's
//! longest run read in place of the loser's. M-E, the
//! boundary moved to three, a series standing clustered
//! only where its loser's longest run stands at three or
//! more. Five pins, the constructed file under the
//! deadlock, streak, distance, rally, and comeback
//! commands, pass against the pre round surface by
//! design, as their own comments state.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{championship_report, load_championship_file, Championship};
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

fn cluster_for(dir: &Path, files: Vec<(&str, JsonValue)>, names: Vec<&str>) -> Vec<String> {
    cluster_report(&load(dir, files, names))
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
fn the_standing_championship_renders_no_clustered_series() {
    // K-1. The standing championship in the empty form,
    // neither of its series standing clustered, the first
    // classification reading whose marked class covers
    // neither. The pilot's loser shadow-1 won one decided
    // race and the panel's loser won none, so both stand
    // scattered. Fails under M-C, under which the pilot's
    // winner prime-1 holds a longest run of 2 and the
    // panel's winner a longest run of 3 and both series
    // stand clustered with sections. Fails under M-D on
    // the same records, either side's longest run
    // reaching the boundary in both series. Passes under
    // M-B by design, no loser run standing open across a
    // no result in either series. Passes under M-E by
    // design, the moved boundary changing nothing where
    // no loser run reaches two.
    let championship = load_example("championship.json");
    let lines = cluster_report(&championship);
    assert_eq!(
        lines,
        vec![
            "cluster",
            "cluster, 2 series, 0 clustered, 2 scattered, 0 no winner",
        ]
    );
    assert_eq!(lines.join("\n") + "\n", golden("championship-cluster.txt"));
}

#[test]
fn the_survey_championship_matches_its_cluster_golden() {
    // K-2. The section form over the survey championship,
    // pinned byte for byte against the golden the real
    // command produced. Fails under M-E, under which the
    // two survey sections standing at figure 2 fall below
    // the moved boundary and the report loses them.
    // Fails under M-A, under which the records read past
    // their decisions and the sections the golden carries
    // move. The fuller mutation discrimination over this
    // fixture stands in the relations pin below, carried
    // jointly with it.
    let championship = load_example("survey-championship.json");
    let lines = cluster_report(&championship);
    assert!(lines.len() > 2, "the survey golden stands in section form");
    assert_eq!(
        lines.join("\n") + "\n",
        golden("survey-championship-cluster.txt")
    );
}

#[test]
fn the_constructed_example_parts_clustered_from_scattered() {
    // K-3. The round's headline discrimination. The first
    // constructed series stands clustered at figure 2,
    // its loser sorrel-1 having taken the fourth and
    // fifth decided races in a row, while standing won
    // from the front, in between, wire to wire, and
    // clear, the unmarked class of every sibling, and the
    // second stands scattered while standing the distance
    // and deadlocked, so the cluster golden carries the
    // first series alone. Fails under M-E, under which
    // the first series' figure of 2 falls below the moved
    // boundary and the report falls to the empty form.
    // Fails under M-A, under which the second series' two
    // recorded races beyond its decision, both won by its
    // loser vesper-1, extend its loser's run to 2 and the
    // report gains its section, the first series' figure
    // standing unmoved. Fails under M-C, under which the
    // first series' figure reads its winner's longest run
    // of 3 in place of its loser's 2.
    let championship = load_example("constructed-cluster.json");
    let lines = cluster_report(&championship);
    assert_eq!(
        lines,
        vec![
            "cluster",
            "series 1, the constructed cluster drill one",
            "sable-1 against sorrel-1, first to 5 wins",
            "race 1, cluster drill one race one, sable-1 wins",
            "race 2, cluster drill one race two, sable-1 wins",
            "race 3, cluster drill one race three, sable-1 wins",
            "race 4, cluster drill one race four, sorrel-1 wins",
            "race 5, cluster drill one race five, sorrel-1 wins",
            "race 6, cluster drill one race six, sable-1 wins",
            "race 7, cluster drill one race seven, sable-1 wins",
            "longest run against 2 consecutive decided race wins",
            "series winner, sable-1, sable-1 5 race wins, sorrel-1 2 race wins, 0 no results",
            "cluster, 2 series, 1 clustered, 1 scattered, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-cluster-cluster.txt")
    );
}

#[test]
fn the_constructed_example_stands_deadlocked_under_deadlock() {
    // K-4. The same file under the deadlock command, the
    // second series standing deadlocked in its section
    // alone, its tallies having stood level after its
    // second and fourth decided races. Passes on the pre
    // round tree by design, in the posture of DH-33's
    // goldens over standing commands, the deadlock reading
    // and its command standing built in Round 13. The pin
    // holds that the round's new fixture does not move a
    // standing reading and that the two readings part over
    // the same file, read against the pin above.
    let championship = load_example("constructed-cluster.json");
    let lines = deadlock_report(&championship);
    assert_eq!(
        lines,
        vec![
            "deadlock",
            "series 2, the constructed cluster drill two",
            "umbra-1 against vesper-1, first to 3 wins",
            "race 1, cluster drill two race one, umbra-1 wins",
            "race 2, cluster drill two race two, vesper-1 wins",
            "race 3, cluster drill two race three, umbra-1 wins",
            "race 4, cluster drill two race four, vesper-1 wins",
            "race 5, cluster drill two race five, umbra-1 wins",
            "stood level after 2 of its decided races",
            "series winner, umbra-1, umbra-1 3 race wins, vesper-1 2 race wins, 0 no results",
            "deadlock, 2 series, 1 deadlocked, 1 clear, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-cluster-deadlock.txt")
    );
}

#[test]
fn the_constructed_example_stands_strung_under_streak() {
    // K-5. The same file under the streak command, the
    // first series standing strung at figure 3 in its
    // section alone, its winner's first three decided
    // races standing in one run. Passes on the pre round
    // tree by design, in the posture of the pin above,
    // the streak reading and its command standing built
    // in Round 14.
    let championship = load_example("constructed-cluster.json");
    let lines = streak_report(&championship);
    assert_eq!(
        lines,
        vec![
            "streak",
            "series 1, the constructed cluster drill one",
            "sable-1 against sorrel-1, first to 5 wins",
            "race 1, cluster drill one race one, sable-1 wins",
            "race 2, cluster drill one race two, sable-1 wins",
            "race 3, cluster drill one race three, sable-1 wins",
            "race 4, cluster drill one race four, sorrel-1 wins",
            "race 5, cluster drill one race five, sorrel-1 wins",
            "race 6, cluster drill one race six, sable-1 wins",
            "race 7, cluster drill one race seven, sable-1 wins",
            "longest run of 3 consecutive decided race wins",
            "series winner, sable-1, sable-1 5 race wins, sorrel-1 2 race wins, 0 no results",
            "streak, 2 series, 1 strung, 1 single, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-cluster-streak.txt")
    );
}

#[test]
fn the_constructed_example_stands_the_distance_under_distance() {
    // K-6. The same file under the distance command, the
    // second series standing the distance in its section
    // alone. Passes on the pre round tree by design, in
    // the posture of the pins above, the distance reading
    // and its command standing built under DH-30.
    let championship = load_example("constructed-cluster.json");
    let lines = distance_report(&championship);
    assert_eq!(
        lines,
        vec![
            "distance",
            "series 2, the constructed cluster drill two",
            "umbra-1 against vesper-1, first to 3 wins",
            "race 1, cluster drill two race one, umbra-1 wins",
            "race 2, cluster drill two race two, vesper-1 wins",
            "race 3, cluster drill two race three, umbra-1 wins",
            "race 4, cluster drill two race four, vesper-1 wins",
            "race 5, cluster drill two race five, umbra-1 wins",
            "series winner, umbra-1, umbra-1 3 race wins, vesper-1 2 race wins, 0 no results",
            "distance, 2 series, 1 the distance, 0 without reply, 1 in between, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-cluster-distance.txt")
    );
}

#[test]
fn the_constructed_example_stands_wire_to_wire_under_rally() {
    // K-7. The same file under the rally command, the
    // empty form, neither winner ever standing behind.
    // Passes on the pre round tree by design, in the
    // posture of the pins above.
    let championship = load_example("constructed-cluster.json");
    let lines = rally_report(&championship);
    assert_eq!(
        lines,
        vec![
            "rally",
            "rally, 2 series, 0 rally, 2 wire to wire, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-cluster-rally.txt")
    );
}

#[test]
fn the_constructed_example_stands_from_the_front_under_comeback() {
    // K-8. The same file under the comeback command, the
    // empty form, each winner having won its first
    // decided race. Passes on the pre round tree by
    // design, in the posture of the pins above.
    let championship = load_example("constructed-cluster.json");
    let lines = comeback_report(&championship);
    assert_eq!(
        lines,
        vec![
            "comebacks",
            "comebacks, 2 series, 0 from behind, 2 from the front",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-cluster-comebacks.txt")
    );
}

#[test]
fn a_series_of_target_one_stands_scattered() {
    // K-9. The only decided race of a decided series of
    // target one is its deciding race, won here by the
    // second model, so its loser won no decided race and
    // the series stands scattered against DH-34's stated
    // consequence, its loser's longest run standing at
    // none. Fails under the classing mutation, a reading
    // that counts a decided series whose loser won no
    // race under no winner in place of scattered, under
    // which the closing moves the series between counts.
    // Passes under M-C by design, the winner's longest
    // run standing at 1, below the boundary on either
    // side's reading, so the closing stands as asserted.
    let dir = temp_dir("clusters-target-one");
    let series = series_json(
        "the single saga",
        vec![ALPHA, BETA],
        1,
        vec![clear_win("race one", BETA, ALPHA)],
    );
    let lines = cluster_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "cluster",
            "cluster, 1 series, 0 clustered, 1 scattered, 0 no winner",
        ]
    );
}

#[test]
fn an_interposed_no_result_leaves_a_loser_run_whole() {
    // K-10. The plan's M-B discriminator. A series at
    // target three whose counted races run winner, loser,
    // no result, loser, winner, winner stands clustered
    // with figure 2, the two loser wins consecutive in
    // the decided record. Fails under M-B, under which
    // the no result breaks the run, the loser's runs
    // stand at 1 and 1, and the series stands scattered.
    let dir = temp_dir("clusters-interposed");
    let series = series_json(
        "the interposed cluster saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            no_result("race three", ALPHA, BETA),
            clear_win("race four", BETA, ALPHA),
            clear_win("race five", ALPHA, BETA),
            clear_win("race six", ALPHA, BETA),
        ],
    );
    let lines = cluster_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "cluster",
            "series 1, the interposed cluster saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, beta-1 wins",
            "race 4, race four, beta-1 wins",
            "race 5, race five, alpha-1 wins",
            "race 6, race six, alpha-1 wins",
            "longest run against 2 consecutive decided race wins",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 2 race wins, 1 no results",
            "cluster, 1 series, 1 clustered, 0 scattered, 0 no winner",
        ]
    );
}

#[test]
fn an_interposed_no_result_leaves_the_section_naming_decided_races_only() {
    // K-11. A series at target four, its decided winners
    // running winner, loser, loser, winner, loser,
    // winner, winner over the decided record, with a no
    // result standing between the first and second loser
    // wins in counted order, stands clustered with figure
    // 2, its section naming the seven decided races and
    // the closing split counting its one no result.
    // Fails under the mutation that walks every counted
    // race into the section lines, which names eight
    // races. Fails under M-B, under which the no result
    // breaks the loser's run, its runs standing at 1 and
    // 1 and 1, and the series stands scattered.
    let dir = temp_dir("clusters-interposed-section");
    let series = series_json(
        "the counted cluster saga",
        vec![ALPHA, BETA],
        4,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            no_result("race three", ALPHA, BETA),
            clear_win("race four", BETA, ALPHA),
            clear_win("race five", ALPHA, BETA),
            clear_win("race six", BETA, ALPHA),
            clear_win("race seven", ALPHA, BETA),
            clear_win("race eight", ALPHA, BETA),
        ],
    );
    let lines = cluster_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "cluster",
            "series 1, the counted cluster saga",
            "alpha-1 against beta-1, first to 4 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, beta-1 wins",
            "race 4, race four, beta-1 wins",
            "race 5, race five, alpha-1 wins",
            "race 6, race six, beta-1 wins",
            "race 7, race seven, alpha-1 wins",
            "race 8, race eight, alpha-1 wins",
            "longest run against 2 consecutive decided race wins",
            "series winner, alpha-1, alpha-1 4 race wins, beta-1 3 race wins, 1 no results",
            "cluster, 1 series, 1 clustered, 0 scattered, 0 no winner",
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

/// The figures of the sections of one cluster report, in
/// section order.
fn section_figures(lines: &[String]) -> Vec<u64> {
    lines
        .iter()
        .filter(|line| line.starts_with("longest run against "))
        .map(|line| {
            line.split(' ')
                .nth(3)
                .expect("a figure")
                .parse::<u64>()
                .expect("a figure")
        })
        .collect()
}

/// The stated relations of DH-35 over one championship,
/// read from the printed readings alone, in the form the
/// streak relations pin states them. The singles and the
/// series standing without reply stand named by their
/// championship offsets, each standing in no cluster
/// section, a single series standing scattered by DH-34's
/// stated relation and a series standing without reply
/// standing scattered by its loser holding no win at all.
fn check_relations(
    championship: &Championship,
    series_count: u64,
    figures: Vec<u64>,
    singles: Vec<u64>,
    without_reply: Vec<u64>,
) {
    let table = championship_report(championship);
    let decided = table
        .iter()
        .filter(|line| line.starts_with("series winner,"))
        .count() as u64;
    assert!(decided > 0, "the fixture holds decided series");
    let cluster = cluster_report(championship);
    let counts = closing_counts(cluster.last().expect("a closing"));
    assert_eq!(counts[0], series_count);
    assert_eq!(counts[1] + counts[2], decided, "clustered and scattered");
    assert_eq!(counts[1] + counts[2] + counts[3], series_count);
    assert_eq!(
        section_figures(&cluster),
        figures,
        "the section figures in order"
    );
    let sections = section_offsets(&cluster);
    assert_eq!(
        sections.len() as u64,
        counts[1],
        "one section per clustered series"
    );
    // The no winner count equals the distance closing's
    // figure outright, the count neither comeback class
    // holds, and the no winner figures of the rally,
    // deadlock, and streak closings.
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
    // Every single series stands scattered by DH-34's
    // stated relation between the two readings, so every
    // clustered series stands strung, and the clustered
    // count stands at or below the strung count.
    assert!(
        counts[1] <= streak_counts[1],
        "the clustered count stands at or below the strung count"
    );
    // Every named single series and every named series
    // standing without reply stands in no cluster
    // section, each standing scattered.
    for offset in singles {
        assert!(
            !sections.contains(&offset),
            "series {offset} stands in no cluster section"
        );
    }
    for offset in without_reply {
        assert!(
            !sections.contains(&offset),
            "series {offset} stands in no cluster section"
        );
    }
}

#[test]
fn the_closing_counts_stand_in_their_stated_relations() {
    // K-12. The relations of DH-35 over the survey
    // championship and the league championship of DH-31,
    // computed over the same readings. Fails under M-C,
    // under which the section figures read the winner's
    // longest run in place of the loser's and the figures
    // and memberships the pin cross-reads move wherever
    // the two sides' longest runs part. Fails under M-A,
    // under which the decided records read past their
    // decisions and loser runs the walk never counted
    // move the closings and the figures. Fails under M-E,
    // under which the sections standing at figure 2, two
    // in the survey and two in the league, fall below the
    // moved boundary and stand scattered.
    check_relations(
        &load_example("survey-championship.json"),
        6,
        vec![2, 2, 3],
        vec![],
        vec![],
    );
    check_relations(
        &load_example("league-championship.json"),
        10,
        vec![2, 3, 2],
        vec![8],
        vec![6, 10],
    );
}

#[test]
fn the_section_figure_reads_the_losers_longest_run() {
    // K-13. The plan's M-D discriminator, and an M-C
    // discriminator beside it. A series at target four
    // whose decided winners run loser, winner, winner,
    // winner, loser, loser, winner. The loser's runs
    // stand at 1 and 2, the longest at 2, and the
    // winner's stand at 3 and 1, the longest at 3. The
    // series stands clustered and its section figure
    // reads 2. Fails under M-D, which prints 3, the
    // longer of the two sides' runs. Fails under M-C,
    // which prints 3, the winner's run read in place of
    // the loser's.
    let dir = temp_dir("clusters-losers-run");
    let series = series_json(
        "the split run saga",
        vec![ALPHA, BETA],
        4,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", ALPHA, BETA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", BETA, ALPHA),
            clear_win("race six", BETA, ALPHA),
            clear_win("race seven", ALPHA, BETA),
        ],
    );
    let lines = cluster_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "cluster",
            "series 1, the split run saga",
            "alpha-1 against beta-1, first to 4 wins",
            "race 1, race one, beta-1 wins",
            "race 2, race two, alpha-1 wins",
            "race 3, race three, alpha-1 wins",
            "race 4, race four, alpha-1 wins",
            "race 5, race five, beta-1 wins",
            "race 6, race six, beta-1 wins",
            "race 7, race seven, alpha-1 wins",
            "longest run against 2 consecutive decided race wins",
            "series winner, alpha-1, alpha-1 4 race wins, beta-1 3 race wins, 0 no results",
            "cluster, 1 series, 1 clustered, 0 scattered, 0 no winner",
        ]
    );
}

/// The handicap twin of the cluster pins. On scratch
/// finishes the series runs winner, loser, loser,
/// winner, winner at target three and stands clustered,
/// its loser's two wins standing in one run. The second
/// race's first model attempt carries an adjustment that
/// moves its finish ahead of the second model's, so the
/// adjusted record runs winner, winner, loser, winner,
/// the series stands decided at the fourth race, and its
/// loser holds one win standing alone.
fn handicap_series(adjusted: bool) -> JsonValue {
    let mut second_first = attempt(ALPHA, "green", 130.0, Some(10));
    if adjusted {
        second_first = set(&second_first, "adjustment", i(-30));
    }
    series_json(
        "the handicap saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            race_json(
                "race two",
                vec![second_first, attempt_two(BETA, "green", 110.0, Some(20))],
            ),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
        ],
    )
}

#[test]
fn a_handicap_adjustment_moves_a_series_between_clustered_and_scattered() {
    // K-14. The twin stands clustered with figure 2 on
    // its scratch record, its section naming all five
    // races, and scattered on its adjusted record, the
    // report standing in the empty form. Fails under the
    // mutation reading the scratch winners in place of
    // the adjusted winners the walk records, the standing
    // discipline every reading of this repo states.
    let dir = temp_dir("clusters-handicap");
    let plain = cluster_for(
        &dir,
        vec![("one.json", handicap_series(false))],
        vec!["one.json"],
    );
    assert_eq!(
        plain,
        vec![
            "cluster",
            "series 1, the handicap saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, beta-1 wins",
            "race 4, race four, alpha-1 wins",
            "race 5, race five, alpha-1 wins",
            "longest run against 2 consecutive decided race wins",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 2 race wins, 0 no results",
            "cluster, 1 series, 1 clustered, 0 scattered, 0 no winner",
        ]
    );
    let adjusted = cluster_for(
        &dir,
        vec![("one.json", handicap_series(true))],
        vec!["one.json"],
    );
    assert_eq!(
        adjusted,
        vec![
            "cluster",
            "cluster, 1 series, 0 clustered, 1 scattered, 0 no winner",
        ]
    );
}

#[test]
fn a_series_holding_no_winner_stands_in_neither_class() {
    // K-15. The decided winners run winner, loser, loser,
    // winner at target three, the target unreached and
    // the series holding no winner. No section stands for
    // it and the closing counts it under no winner alone.
    // Fails under the champion drop mutation, a reading
    // that classes an undecided series by the longest run
    // of a side in place of the champion the walk names,
    // under which the series leaves the no winner count
    // and stands classed. This pin carries the no winner
    // form of DH-21 and DH-34.
    let dir = temp_dir("clusters-open");
    let series = series_json(
        "the open saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
        ],
    );
    let lines = cluster_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "cluster",
            "cluster, 1 series, 0 clustered, 0 scattered, 1 no winner",
        ]
    );
}

#[test]
fn a_series_decided_early_keeps_its_class_against_its_later_races() {
    // K-16. The plan's small fixture M-A discriminator.
    // The series is decided at a target of two by the
    // decided winners loser, winner, winner at the third
    // race, with a fourth and a fifth recorded race won
    // by the loser beyond the decision, and stands
    // scattered, because the walk counts nothing beyond
    // the decision and its loser's longest counted run
    // stands at 1. Fails under M-A, under which the
    // loser's two races beyond the decision stand in one
    // run of 2 over the full record and the series stands
    // clustered.
    let dir = temp_dir("clusters-beyond");
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
    let lines = cluster_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "cluster",
            "cluster, 1 series, 0 clustered, 1 scattered, 0 no winner",
        ]
    );
}
