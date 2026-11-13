//! Pins for DH-44 and DH-45 in the Round 21 section of
//! CONTRACT.md.
//!
//! The discrimination record of this file stands pin by
//! pin, and no blanket claim stands over it. The
//! crossings reading, its library function, and its
//! command do not stand on the pre round tree, so
//! every pin in this file that names the crossings
//! reading is absent there at compile time. That
//! absence is recorded once here and is not a
//! discrimination claim. The discrimination each pin
//! carries is against the stated mutation of DH-44
//! beside it. Eight mutations are named for the whole
//! file. M-A, the walk does not stop, the decided
//! record read whole to the end of the series file
//! and the crossings and changes of hand read at its
//! end. M-B, the turns figure, the crossings count
//! read as the count of the changes of hand of the
//! decided record, the statistic DH-40 counts. M-C,
//! the loser's side, the crossings counted only where
//! the tally stood apart for the loser before the
//! crossing, the crossings that erase the loser's
//! lead alone standing counted. M-D, the opening
//! departure, the crossings count read with the
//! departure after the first decided race counted as
//! a crossing, one standing added to every decided
//! series. M-E, the boundary moved, the class read as
//! through where the crossings count stands at the
//! count of changes of hand or one below it. M-F, the
//! level or behind figure, the crossings count read
//! as the count of the decided races after which the
//! winner's tally stood level with or below the
//! loser's. The classing mutation, a series holding
//! no decided race before its deciding race standing
//! unclassed, its counts leaving the closing line.
//! The champion drop mutation, a series holding no
//! winner classed by the decided record of the model
//! holding the most decided wins of the series. The
//! eight constructed sibling pins L-5 to L-12 pass on
//! the pre round tree by design, as their own
//! comments state, and pass under M-A to M-F and the
//! classing and champion drop mutations by design,
//! the mutations touching the crossings module alone.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{championship_report, load_championship_file, Championship};
use deadheat::closings::closing_report;
use deadheat::clusters::cluster_report;
use deadheat::comebacks::comeback_report;
use deadheat::crossings::crossings_report;
use deadheat::deadlocks::deadlock_report;
use deadheat::distances::distance_report;
use deadheat::json::JsonValue;
use deadheat::leads::lead_report;
use deadheat::openings::opening_report;
use deadheat::rallies::rally_report;
use deadheat::series::series_report;
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

fn crossings_for(dir: &Path, files: Vec<(&str, JsonValue)>, names: Vec<&str>) -> Vec<String> {
    crossings_report(&load(dir, files, names))
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
fn the_standing_championship_stands_through_in_the_panel_series() {
    // L-1. The standing championship golden, its
    // section standing over the panel series at the
    // figure 0, closing "crossings, 2 series, 1
    // through, 1 short, 0 no winner", the pilot
    // standing short at two changes of hand and no
    // crossing. Fails under M-A, the panel file
    // holding races beyond its decision under which
    // its changes of hand stand at two against no
    // crossing, its section leaving the report.
    // Fails under M-B, the pilot standing through on
    // the turns figure at two crossings against its
    // two changes of hand and gaining its section,
    // and the panel standing at no change of hand,
    // its section standing where it stands. Fails under M-D, the panel carrying
    // one crossing too many against no change of hand
    // and leaving its section. Passes under M-C, each
    // series of the standing championship standing at
    // no crossing on either side of the record.
    // Passes under M-E, the panel standing at
    // equality and staying through on the widened
    // boundary and the pilot standing two below
    // equality and staying short. Passes under M-F,
    // no winner of the standing championship ever
    // standing behind.
    let championship = load_example("championship.json");
    let lines = crossings_report(&championship);
    assert_eq!(
        lines,
        vec![
            "crossings",
            "series 2, the recorded close call panel",
            "prime-1 against shadow-1, first to 3 wins",
            "race 1, pin the cache key, prime-1 wins",
            "race 2, trim the import list, prime-1 wins",
            "race 5, name the worker pool, prime-1 wins",
            "crossings 0 over the decided record",
            "series winner, prime-1, prime-1 3 race wins, shadow-1 0 race wins, 2 no results",
            "crossings, 2 series, 1 through, 1 short, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("championship-crossings.txt")
    );
}

#[test]
fn the_survey_championship_matches_its_crossings_golden() {
    // L-2. The survey championship golden, carrying
    // the apex and bolt section at the figure 2 and
    // the crest and drift section at the figure 1 in
    // that order and closing "crossings, 6 series, 2
    // through, 4 short, 0 no winner". Fails under
    // M-A, the apex and bolt series standing short on
    // its whole file at four crossings against five
    // changes of hand and its section leaving the
    // report. Fails under M-B, every decided series
    // standing through on the turns figure, its
    // sections and closings moving. Fails under M-C,
    // one of the apex and bolt series' two crossings
    // erasing its winner's lead, its counted
    // crossings falling to one against two changes of
    // hand and its section leaving the report. Fails
    // under M-D, every decided series carrying one
    // crossing too many. Fails under M-E, the apex
    // and drift series standing one below equality
    // and moving to through with a section the golden
    // does not carry. Fails under M-F, the apex and
    // bolt series' winner standing behind on its
    // record, its level or behind count of three
    // standing against its two changes of hand and
    // its section leaving the report.
    let championship = load_example("survey-championship.json");
    let lines = crossings_report(&championship);
    assert!(lines.len() > 2, "the survey golden stands in section form");
    assert_eq!(section_offsets(&lines), vec![1, 6]);
    assert_eq!(section_figures(&lines), vec![2, 1]);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("survey-championship-crossings.txt")
    );
}

#[test]
fn the_league_championship_matches_its_crossings_golden() {
    // L-3. The league championship golden, carrying
    // the apex and crest section at the figure 2, the
    // apex and ember section at the figure 1, the
    // bolt and drift section at the figure 0, and the
    // drift and ember section at the figure 0 in that
    // order and closing "crossings, 10 series, 4
    // through, 6 short, 0 no winner". Fails under
    // M-A, five of its sections and closings moving
    // across the league on the whole files. Fails
    // under M-B, every decided series standing
    // through on the turns figure. Fails under M-C,
    // one of the apex and crest series' crossings
    // erasing its winner's lead, its counted
    // crossings falling against its changes of hand
    // and its section leaving the report. Fails under
    // M-D, every decided series carrying one crossing
    // too many. Fails under M-E, the five league
    // series standing one below equality moving to
    // through with sections the golden does not
    // carry. Fails under M-F, the apex and crest
    // series' winner standing behind on its record,
    // its level or behind count standing above its
    // crossings and its section leaving the report.
    let championship = load_example("league-championship.json");
    let lines = crossings_report(&championship);
    assert!(lines.len() > 2, "the league golden stands in section form");
    assert_eq!(section_offsets(&lines), vec![2, 4, 6, 10]);
    assert_eq!(section_figures(&lines), vec![2, 1, 0, 0]);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("league-championship-crossings.txt")
    );
}

#[test]
fn the_constructed_example_carries_the_second_series_alone() {
    // L-4. The constructed crossings golden, the
    // second series in section form at the figure 3,
    // closing "crossings, 2 series, 1 through, 1
    // short, 0 no winner", the first series standing
    // short at two crossings against four changes of
    // hand and the second standing through at three
    // against three. Fails under M-B, both series
    // standing through on the turns figure, the first
    // gaining a section and the second's figure
    // printing its changes of hand. Fails under M-C,
    // one of the second series' three crossings
    // erasing its winner's lead, its counted
    // crossings falling to two against three changes
    // of hand and its section leaving the report.
    // Fails under M-D, the second series carrying one
    // crossing too many against its changes of hand
    // and leaving its section. Fails under M-F, the
    // second series' winner standing behind on its
    // record, its level or behind count of four
    // standing against its three crossings. Passes
    // under M-A by design, neither constructed series
    // holding a recorded race beyond its decision.
    // Passes under M-E, the first series standing two
    // below equality and the second standing at
    // equality and staying through on the widened
    // boundary.
    let championship = load_example("constructed-crossings.json");
    let lines = crossings_report(&championship);
    assert_eq!(
        lines,
        vec![
            "crossings",
            "series 2, the constructed crossings drill two",
            "petrel-1 against puffin-1, first to 4 wins",
            "race 1, crossings drill two race one, petrel-1 wins",
            "race 2, crossings drill two race two, puffin-1 wins",
            "race 3, crossings drill two race three, puffin-1 wins",
            "race 4, crossings drill two race four, petrel-1 wins",
            "race 5, crossings drill two race five, petrel-1 wins",
            "race 6, crossings drill two race six, puffin-1 wins",
            "race 7, crossings drill two race seven, puffin-1 wins",
            "crossings 3 over the decided record",
            "series winner, puffin-1, petrel-1 3 race wins, puffin-1 4 race wins, 0 no results",
            "crossings, 2 series, 1 through, 1 short, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-crossings-crossings.txt")
    );
}

#[test]
fn the_constructed_example_stands_strung_under_streak() {
    // L-5. The same file under the streak command,
    // both series in section form, closing "streak, 2
    // series, 2 strung, 0 single, 0 no winner".
    // Passes on the pre round tree by design and
    // under M-A to M-F and the classing and champion
    // drop mutations by design, as the header of this
    // file states for the sibling pins.
    let championship = load_example("constructed-crossings.json");
    let lines = streak_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-crossings-streak.txt")
    );
}

#[test]
fn the_constructed_example_stands_clustered_under_cluster() {
    // L-6. The same file under the cluster command,
    // both series in section form, closing "cluster,
    // 2 series, 2 clustered, 0 scattered, 0 no
    // winner". Passes on the pre round tree by design
    // and under the named mutations by design.
    let championship = load_example("constructed-crossings.json");
    let lines = cluster_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-crossings-cluster.txt")
    );
}

#[test]
fn the_constructed_example_stands_a_rally_under_rally() {
    // L-7. The same file under the rally command, the
    // second series in section form, closing "rally,
    // 2 series, 1 rally, 1 wire to wire, 0 no
    // winner". Passes on the pre round tree by design
    // and under the named mutations by design.
    let championship = load_example("constructed-crossings.json");
    let lines = rally_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-crossings-rally.txt")
    );
}

#[test]
fn the_constructed_example_stands_deadlocked_under_deadlock() {
    // L-8. The same file under the deadlock command,
    // both series in section form, closing
    // "deadlock, 2 series, 2 deadlocked, 0 clear, 0
    // no winner". Passes on the pre round tree by
    // design and under the named mutations by design.
    let championship = load_example("constructed-crossings.json");
    let lines = deadlock_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-crossings-deadlock.txt")
    );
}

#[test]
fn the_constructed_example_stands_the_distance_under_distance() {
    // L-9. The same file under the distance command,
    // both series in section form, closing
    // "distance, 2 series, 2 the distance, 0 without
    // reply, 0 in between, 0 no winner". Passes on
    // the pre round tree by design and under the
    // named mutations by design.
    let championship = load_example("constructed-crossings.json");
    let lines = distance_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-crossings-distance.txt")
    );
}

#[test]
fn the_constructed_example_stands_from_behind_under_comeback() {
    // L-10. The same file under the comeback command,
    // the second series in section form, closing
    // "comebacks, 2 series, 1 from behind, 1 from the
    // front". Passes on the pre round tree by design
    // and under the named mutations by design.
    let championship = load_example("constructed-crossings.json");
    let lines = comeback_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-crossings-comebacks.txt")
    );
}

#[test]
fn the_constructed_example_stands_in_a_run_under_closing() {
    // L-11. The same file under the closing command,
    // the second series in section form, closing
    // "closing, 2 series, 1 in a run, 1 alone, 0 no
    // winner". Passes on the pre round tree by design
    // and under the named mutations by design.
    let championship = load_example("constructed-crossings.json");
    let lines = closing_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-crossings-closing.txt")
    );
}

#[test]
fn the_constructed_example_stands_in_a_run_under_opening() {
    // L-12. The same file under the opening command,
    // the first series in section form, closing
    // "opening, 2 series, 1 in a run, 0 alone, 1 from
    // behind, 0 no winner". Passes on the pre round
    // tree by design and under the named mutations by
    // design.
    let championship = load_example("constructed-crossings.json");
    let lines = opening_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-crossings-opening.txt")
    );
}

#[test]
fn a_series_of_target_one_stands_through() {
    // L-13. The target one edge. A hand-built series
    // of target one decided by its only decided race
    // stands through, its crossings and its changes
    // of hand both standing at none, its report
    // carrying its section at the figure 0 and
    // closing "crossings, 1 series, 1 through, 0
    // short, 0 no winner". Fails under the classing
    // mutation alone among the named mutations
    // besides M-D, the lone-decider series standing
    // unclassed and its closing moving. Fails under
    // M-D, its one standing added crossing standing
    // against no change of hand and the series
    // falling short out of its section. Passes under
    // M-A, M-B, M-C, M-E, and M-F by design, its
    // record holding one race, so no figure moves,
    // the series standing through on both figures,
    // at equality on the widened boundary, and at no
    // level or behind standing of its winner.
    let dir = temp_dir("crossings-target-one");
    let series = series_json(
        "the target one crossings saga",
        vec![ALPHA, BETA],
        1,
        vec![clear_win("race one", ALPHA, BETA)],
    );
    let lines = crossings_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "crossings",
            "series 1, the target one crossings saga",
            "alpha-1 against beta-1, first to 1 wins",
            "race 1, race one, alpha-1 wins",
            "crossings 0 over the decided record",
            "series winner, alpha-1, alpha-1 1 race wins, beta-1 0 race wins, 0 no results",
            "crossings, 1 series, 1 through, 0 short, 0 no winner",
        ]
    );
}

#[test]
fn an_interposed_no_result_leaves_the_crossings_whole() {
    // L-14. The decided record read whole. A
    // hand-built series at a target of three whose
    // record interposes two races holding no winner
    // among four counted decided races won by alpha,
    // beta, alpha, alpha, with a fifth decided race
    // won by beta standing beyond the decision,
    // stands short with no section, its crossings
    // standing at one against its two changes of
    // hand, closing "crossings, 1 series, 0 through,
    // 1 short, 0 no winner", and the turns report
    // over the same championship closes turned at the
    // figure two, proving the interposed races left
    // the decided record the crossings reading reads
    // whole and in order. Fails under M-A at its
    // turns cross read, the whole file record turning
    // at three. Fails under M-B, its crossings
    // reading as its two changes of hand and the
    // series standing through in its section. Fails
    // under M-D, its one standing added crossing
    // standing it through at two against two. Fails
    // under M-E, its crossings standing one below its
    // changes of hand and the series moving to
    // through. Passes under M-C, its one crossing
    // erasing its winner's lead and its counted
    // crossings falling to none, the series standing
    // short as it stands. Passes under M-F, its
    // winner's level or behind count standing at its
    // one crossing and the series standing short as
    // it stands.
    let dir = temp_dir("crossings-interposed");
    let series = series_json(
        "the interposed crossings saga",
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
    let lines = crossings_report(&championship);
    assert_eq!(
        lines,
        vec![
            "crossings",
            "crossings, 1 series, 0 through, 1 short, 0 no winner"
        ]
    );
    let turns = turns_report(&championship);
    assert_eq!(
        turns,
        vec![
            "turns",
            "series 1, the interposed crossings saga",
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

/// The figures of the sections of one crossings report,
/// in section order.
fn section_figures(lines: &[String]) -> Vec<u64> {
    lines
        .iter()
        .filter(|line| line.starts_with("crossings "))
        .map(|line| {
            line.split(' ')
                .nth(1)
                .expect("a figure")
                .parse::<u64>()
                .expect("a figure")
        })
        .collect()
}

/// One side of a series winner line of the championship
/// report, its model name and its count of race wins,
/// read from the field that names the model, its
/// count, and the words race wins.
fn tally_split(field: &str) -> (String, u64) {
    let body = field.strip_suffix(" wins").expect("a wins field");
    let (model, count) = body.rsplit_once(' ').expect("a model and its count");
    (model.to_string(), count.parse::<u64>().expect("a count"))
}

/// The stated relations of DH-44 and DH-45 over one
/// championship, read from the printed readings alone.
/// Both championships this helper reads hold a winner
/// in every series, so the series winner lines of the
/// championship report stand in championship order,
/// and the final tally of a series' own series report
/// states its split, naming the series' loser by the
/// side the champion is not.
fn check_relations(championship: &Championship, series_count: u64) {
    let table = championship_report(championship);
    let winner_lines: Vec<&String> = table
        .iter()
        .filter(|line| line.starts_with("series winner,"))
        .collect();
    let decided = winner_lines.len() as u64;
    assert!(decided > 0, "the fixture holds decided series");
    assert_eq!(decided, series_count, "every series holds a winner");
    assert_eq!(
        section_offsets(&table).len() as u64,
        series_count,
        "the championship series count"
    );
    let crossings = crossings_report(championship);
    let counts = closing_counts(crossings.last().expect("a closing"));
    assert_eq!(counts[0], series_count);
    assert_eq!(counts[1] + counts[2], decided, "through and short");
    assert_eq!(counts[1] + counts[2] + counts[3], series_count);
    let sections = section_offsets(&crossings);
    assert_eq!(
        sections.len() as u64,
        counts[1],
        "one section per through series"
    );
    // The no winner count equals the figures of the
    // ten sibling closings DH-45 names, the comeback
    // leftover among them, because the eleven
    // readings read one walk.
    let turns = turns_report(championship);
    let turns_counts = closing_counts(turns.last().expect("a closing"));
    assert_eq!(counts[3], turns_counts[3], "the turns no winner figure");
    let lead = lead_report(championship);
    let lead_counts = closing_counts(lead.last().expect("a closing"));
    assert_eq!(counts[3], lead_counts[3], "the lead no winner figure");
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
    // Every series standing in a crossings section at
    // a figure of one or more stands in a deadlock
    // section over the same championship, a series
    // carrying a crossing standing deadlocked under
    // DH-29.
    let figures = section_figures(&crossings);
    let deadlock_sections = section_offsets(&deadlock);
    for (offset, figure) in sections.iter().zip(figures.iter()) {
        if *figure >= 1 {
            assert!(
                deadlock_sections.contains(offset),
                "series {offset} stands deadlocked"
            );
        }
    }
    // Every series whose loser stands at no race wins
    // in its championship block stands in a crossings
    // section, the series standing without reply
    // under DH-23 and through under DH-44.
    for (index, series) in championship.series.iter().enumerate() {
        let report = series_report(series);
        let champion = report
            .last()
            .expect("a winner line")
            .strip_prefix("series winner, ")
            .expect("a series winner");
        let tally = report
            .iter()
            .rev()
            .find(|line| line.starts_with("tally, "))
            .expect("a final tally");
        let fields: Vec<&str> = tally.split(", ").collect();
        let (first_model, first_wins) = tally_split(fields[1]);
        let (_, second_wins) = tally_split(fields[2]);
        let loser_wins = if champion == first_model {
            second_wins
        } else {
            first_wins
        };
        if loser_wins == 0 {
            let offset = index as u64 + 1;
            assert!(
                sections.contains(&offset),
                "series {offset} stands in a crossings section"
            );
        }
    }
    // The count of short series stands at or above
    // the count of single series in the streak
    // closing, as the league figures of DH-44 state
    // it.
    assert!(
        counts[2] >= streak_counts[2],
        "the short count stands at or above the single count"
    );
}

#[test]
fn the_crossings_counts_stand_in_their_stated_relations() {
    // L-15. The relations pin, asserting over the
    // survey and league championships test by test
    // the relations DH-44 and DH-45 state, the
    // through and short counts and the no winner
    // count summing to the championship report's
    // series count over the same files, the no winner
    // count standing equal across the ten sibling
    // closings at the grain those closings print it,
    // every series standing in a crossings section at
    // a figure of one or more standing in a deadlock
    // section over the same championship, every
    // series whose loser stands at no race wins in
    // its championship block standing in a crossings
    // section, and the count of short series in the
    // league crossings closing standing at or above
    // the count of single series in the league streak
    // closing. Fails under M-A, the full records
    // moving the closing counts the pin sums. Fails
    // under M-B, the apex and crest series of the
    // survey standing through on the turns figure
    // while standing clear, its deadlock inclusion
    // breaking. Fails under M-D, the series whose
    // losers stand at no race wins carrying one
    // crossing too many against no change of hand and
    // leaving the crossings sections the pin reads.
    // Passes under M-C by the stated coincidence, the
    // relations surviving the mutation on the shipped
    // championships, the golden pins carrying it.
    // Passes under M-E by the stated coincidence,
    // every series the mutation moves standing
    // deadlocked, so its inclusions hold. Passes
    // under M-F, its assertions reading counts,
    // inclusions, and equalities alone, and the
    // series the mutation moves standing out of the
    // sections the pin reads its inclusions from.
    check_relations(&load_example("survey-championship.json"), 6);
    check_relations(&load_example("league-championship.json"), 10);
}

#[test]
fn a_winners_lead_erased_to_level_stands_short() {
    // L-16. The erasure at hand scale. A hand-built
    // series at a target of three won by alpha,
    // alpha, beta, beta, alpha, its winner's lead
    // erased to level at its fourth decided race
    // without the winner ever standing behind, stands
    // short with no section, its crossings standing
    // at one against its two changes of hand, closing
    // "crossings, 1 series, 0 through, 1 short, 0 no
    // winner", the deadlock report over the same
    // championship counting it deadlocked and the
    // rally report counting it wire to wire, proving
    // a crossing stands without a rally. Fails under
    // M-B, its crossings reading as its two changes
    // of hand and the series standing through in its
    // section. Fails under M-D, its one standing added
    // crossing standing it through at two against
    // two. Fails under M-E, its crossings standing one
    // below its changes of hand and the series moving
    // to through. Passes under M-A by design, no race
    // standing beyond its decision. Passes under M-C,
    // its one crossing erasing its winner's lead and
    // its counted crossings falling to none, the
    // series standing short as it stands. Passes
    // under M-F, as the plan states for this pin and
    // its touch sibling, its crossings assertions
    // reading the class and the closing, which the
    // mutation leaves standing on this fixture.
    let dir = temp_dir("crossings-erasure");
    let series = series_json(
        "the erasure crossings saga",
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
    let lines = crossings_report(&championship);
    assert_eq!(
        lines,
        vec![
            "crossings",
            "crossings, 1 series, 0 through, 1 short, 0 no winner"
        ]
    );
    let deadlock = deadlock_report(&championship);
    assert_eq!(
        deadlock,
        vec![
            "deadlock",
            "series 1, the erasure crossings saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, alpha-1 wins",
            "race 3, race three, beta-1 wins",
            "race 4, race four, beta-1 wins",
            "race 5, race five, alpha-1 wins",
            "stood level after 1 of its decided races",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 2 race wins, 0 no results",
            "deadlock, 1 series, 1 deadlocked, 0 clear, 0 no winner",
        ]
    );
    let rally = rally_report(&championship);
    assert_eq!(
        rally,
        vec![
            "rally",
            "rally, 1 series, 0 rally, 1 wire to wire, 0 no winner"
        ]
    );
}

#[test]
fn a_crossing_retaken_by_the_same_side_stands_short() {
    // L-17. The touch at hand scale. A hand-built
    // series at a target of three won by alpha, beta,
    // alpha, beta, beta, its tallies standing level
    // after its second and fourth decided races and
    // departing after the second crossing to the
    // model that had held the lead before it, stands
    // short at two crossings against three changes of
    // hand with no section, closing "crossings, 1
    // series, 0 through, 1 short, 0 no winner", and
    // the turns report over the same championship
    // carrying its section at the figure three,
    // proving a crossing the same side retakes leaves
    // its run standing at level at its head and the
    // series short. Fails under M-B, its crossings
    // reading as its three changes of hand and the
    // series standing through in its section. Fails
    // under M-D, its one standing added crossing
    // standing it through at three against three.
    // Fails under M-E, its crossings standing one
    // below its changes of hand and the series moving
    // to through. Passes under M-A by design, no race
    // standing beyond its decision. Passes under M-C,
    // both of its crossings erasing its loser's lead,
    // its counted crossings standing at two against
    // three changes of hand and the series standing
    // short as it stands.
    // Passes under M-F, its crossings assertions
    // reading the class and the closing, which the
    // mutation leaves standing on this fixture.
    let dir = temp_dir("crossings-touch");
    let series = series_json(
        "the touch crossings saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", BETA, ALPHA),
            clear_win("race five", BETA, ALPHA),
        ],
    );
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    let lines = crossings_report(&championship);
    assert_eq!(
        lines,
        vec![
            "crossings",
            "crossings, 1 series, 0 through, 1 short, 0 no winner"
        ]
    );
    let turns = turns_report(&championship);
    assert_eq!(
        turns,
        vec![
            "turns",
            "series 1, the touch crossings saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, alpha-1 wins",
            "race 4, race four, beta-1 wins",
            "race 5, race five, beta-1 wins",
            "changes of hand 3 over the decided record",
            "series winner, beta-1, alpha-1 2 race wins, beta-1 3 race wins, 0 no results",
            "turns, 1 series, 1 turned, 0 steady, 0 no winner",
        ]
    );
}

/// One race of the handicap twin on identical
/// recorded finishes, beta standing ahead of alpha on
/// scratch, its finish of 110 standing against
/// alpha's 130.
fn beta_first(task: &str) -> JsonValue {
    race_json(
        task,
        vec![
            attempt(ALPHA, "green", 130.0, Some(10)),
            attempt_two(BETA, "green", 110.0, Some(20)),
        ],
    )
}

/// The mirror race of the handicap twin, alpha
/// standing ahead of beta on scratch, its finish of
/// 110 standing against beta's 130, and an adjustment
/// of 40 standing on alpha's attempt in the adjusted
/// arm, moving its finish behind beta's and
/// returning the race to beta.
fn alpha_first(task: &str, adjusted: bool) -> JsonValue {
    let mut first = attempt(ALPHA, "green", 110.0, Some(10));
    if adjusted {
        first = set(&first, "adjustment", i(40));
    }
    race_json(
        task,
        vec![first, attempt_two(BETA, "green", 130.0, Some(20))],
    )
}

/// The handicap twin of the crossings pins. On
/// scratch finishes the decided races run beta,
/// alpha, beta, alpha, beta at a target of three,
/// beta's series standing short at two crossings
/// against four changes of hand. Under an adjustment
/// on alpha's attempt in the second race the same
/// finishes return that race to beta, whose record
/// runs three straight and decides at its third
/// decided race, standing through at no crossing and
/// no change of hand.
fn handicap_series(adjusted: bool) -> JsonValue {
    series_json(
        "the handicap crossings saga",
        vec![ALPHA, BETA],
        3,
        vec![
            beta_first("race one"),
            alpha_first("race two", adjusted),
            beta_first("race three"),
            alpha_first("race four", false),
            beta_first("race five"),
        ],
    )
}

#[test]
fn a_handicap_adjustment_moves_a_series_between_short_and_through() {
    // L-18. The handicap restatement. The twin pair
    // over identical recorded finishes stands short
    // with no section on its scratch arm, its decided
    // races running beta, alpha, beta, alpha, beta at
    // a target of three, and through in section form
    // at the figure 0 on its adjusted arm, the same
    // finishes under an adjustment on the second race
    // returning it to beta, whose walk decides at its
    // third decided race, the pair proving the
    // classification reads the adjusted record the
    // walk holds and not the recorded one. Fails
    // under M-A on its adjusted arm, whose races
    // beyond its adjusted decision leave it short on
    // the whole file. Fails under M-B, the scratch
    // arm standing through on the turns figure at
    // four crossings against four changes of hand.
    // Fails under M-D, the adjusted arm carrying one
    // crossing too many against no change of hand and
    // leaving its section. Passes under M-C, both of
    // the scratch arm's crossings erasing its
    // winner's lead and its counted crossings falling
    // to none, the series standing short as it
    // stands, and the adjusted arm standing at no
    // crossing on either side. Passes under M-E, its
    // scratch arm standing two below equality and its
    // adjusted arm standing at equality and staying
    // through on the widened boundary. Passes under
    // M-F, the scratch arm's level or behind count
    // standing at its two crossings and the adjusted
    // arm's at none.
    let dir = temp_dir("crossings-handicap");
    let plain = crossings_for(
        &dir,
        vec![("one.json", handicap_series(false))],
        vec!["one.json"],
    );
    assert_eq!(
        plain,
        vec![
            "crossings",
            "crossings, 1 series, 0 through, 1 short, 0 no winner"
        ]
    );
    let adjusted = crossings_for(
        &dir,
        vec![("one.json", handicap_series(true))],
        vec!["one.json"],
    );
    assert_eq!(
        adjusted,
        vec![
            "crossings",
            "series 1, the handicap crossings saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, beta-1 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, beta-1 wins",
            "crossings 0 over the decided record",
            "series winner, beta-1, alpha-1 0 race wins, beta-1 3 race wins, 0 no results",
            "crossings, 1 series, 1 through, 0 short, 0 no winner",
        ]
    );
}

#[test]
fn a_championship_holding_no_winner_stands_in_no_class() {
    // L-19. The no winner pin. A hand-built
    // championship of two series holding no winner,
    // the first holding decided races won by alpha,
    // beta, alpha, beta at a target of three with its
    // tallies standing level after its second and
    // fourth decided races and its target unreached,
    // the second holding no decided race, carries no
    // section and closes "crossings, 2 series, 0
    // through, 0 short, 2 no winner", the first
    // series standing in neither class even where its
    // tallies stood level over its counted races.
    // Fails under the champion drop mutation alone,
    // which classes the first series by its plurality
    // leader's record and takes it out of the no
    // winner count. Passes under M-A to M-F and the
    // classing mutation by design, neither series
    // holding a winner for any of them to class, and
    // neither holding a deciding race for the
    // classing mutation to reach.
    let dir = temp_dir("crossings-open");
    let first = series_json(
        "the open crossings saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", BETA, ALPHA),
        ],
    );
    let second = series_json(
        "the silent crossings saga",
        vec![GAMMA, DELTA],
        3,
        vec![no_result("race one", GAMMA, DELTA)],
    );
    let lines = crossings_for(
        &dir,
        vec![("one.json", first), ("two.json", second)],
        vec!["one.json", "two.json"],
    );
    assert_eq!(
        lines,
        vec![
            "crossings",
            "crossings, 2 series, 0 through, 0 short, 2 no winner",
        ]
    );
}

#[test]
fn a_collection_holding_no_series_renders_the_empty_form_at_the_library_surface() {
    // No championship file carries an empty collection,
    // because the loader of DH-13 refuses a series list
    // holding no names, so this form stands at the library
    // surface alone, over a collection built by hand.
    let championship = Championship { series: vec![] };
    let lines = crossings_report(&championship);
    assert_eq!(
        lines,
        vec![
            "crossings",
            "crossings, 0 series, 0 through, 0 short, 0 no winner",
        ]
    );
}
