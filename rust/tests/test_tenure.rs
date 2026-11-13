//! Pins for DH-46 and DH-47 in the Round 22 section of
//! CONTRACT.md.
//!
//! The discrimination record of this file stands pin by
//! pin, and no blanket claim stands over it. The tenure
//! reading, its library function, and its command do
//! not stand on the pre round tree, so every pin in
//! this file that names the tenure reading is absent
//! there at compile time. That absence is recorded
//! once here and is not a discrimination claim. The
//! discrimination each pin carries is against the
//! stated mutation of DH-46 beside it. Eight mutations
//! are named for the whole file. M-A, the walk does
//! not stop, the decided record read whole to the end
//! of the series file and the front counts read at its
//! end. M-B, the final tally figure, the front count
//! of each model read as the count of that model's
//! decided wins over the decided record, the figure
//! the championship table already prints in another
//! seat. M-C, the counts swapped, the front count of
//! the winner read as the front count of the loser and
//! the front count of the loser read as the front
//! count of the winner, the printed figure standing
//! as the loser's count. M-D, one standing added, the
//! front count of the winner read with one standing
//! added to every decided series, its printed figure
//! standing one above its count. M-E, the boundary
//! moved, the class read as ahead where the front
//! count of the winner stands at or above the front
//! count of the loser, the level class standing empty.
//! M-F, the level or in front figure, the front count
//! read as the count of the decided races after which
//! the winner's tally stood level with or above the
//! loser's, the winner's front count and its crossings
//! count standing summed. The classing mutation, a
//! series holding no decided race before its deciding
//! race standing unclassed, its counts leaving the
//! closing line. The champion drop mutation, a series
//! holding no winner classed by the decided record of
//! the model holding the most decided wins of the
//! series. The eight constructed sibling pins M-5 to
//! M-12 pass on the pre round tree by design, as their
//! own comments state, and pass under M-A to M-F and
//! the classing and champion drop mutations by design,
//! the mutations touching the tenure module alone.

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
use deadheat::tenure::tenure_report;
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

fn tenure_for(dir: &Path, files: Vec<(&str, JsonValue)>, names: Vec<&str>) -> Vec<String> {
    tenure_report(&load(dir, files, names))
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
fn the_standing_championship_stands_ahead_in_both_of_its_series() {
    // M-1. The standing championship golden, its
    // sections standing over the pilot series at the
    // figure 4 and the panel series at the figure 3
    // in that order, closing "tenure, 2 series, 2
    // ahead, 0 level, 0 behind, 0 no winner". Fails
    // under M-A, the panel file holding races beyond
    // its decision under which its front count stands
    // at six against the shipped three, its figure
    // line moving. Fails under M-B, the pilot figure
    // moving from four to its win count of three, the
    // panel figure standing at three on both figures.
    // Fails under M-C, both series standing behind on
    // the swapped counts, their losers' front counts
    // standing at none, and both sections leaving the
    // report. Fails under M-D, every figure carrying
    // one standing too many. Passes under M-E, no
    // series of the standing championship standing
    // level. Passes under M-F as a stated coincidence
    // of its fixtures, neither series holding a level
    // standing, so its figures stand at four and three
    // on either reading.
    let championship = load_example("championship.json");
    let lines = tenure_report(&championship);
    assert_eq!(
        lines,
        vec![
            "tenure",
            "series 1, the recorded close call pilot",
            "prime-1 against shadow-1, first to 3 wins",
            "race 1, pin the cache key, prime-1 wins",
            "race 5, name the worker pool, prime-1 wins",
            "race 7, shrink the prompt budget, shadow-1 wins",
            "race 9, sort the seed list, prime-1 wins",
            "front count 4 over the decided record",
            "series winner, prime-1, prime-1 3 race wins, shadow-1 1 race wins, 5 no results",
            "series 2, the recorded close call panel",
            "prime-1 against shadow-1, first to 3 wins",
            "race 1, pin the cache key, prime-1 wins",
            "race 2, trim the import list, prime-1 wins",
            "race 5, name the worker pool, prime-1 wins",
            "front count 3 over the decided record",
            "series winner, prime-1, prime-1 3 race wins, shadow-1 0 race wins, 2 no results",
            "tenure, 2 series, 2 ahead, 0 level, 0 behind, 0 no winner",
        ]
    );
    assert_eq!(lines.join("\n") + "\n", golden("championship-tenure.txt"));
}

#[test]
fn the_survey_championship_matches_its_tenure_golden() {
    // M-2. The survey championship golden, carrying
    // its sections over all six series at the figures
    // 2, 6, 5, 4, 5, and 2 in championship order and
    // closing "tenure, 6 series, 6 ahead, 0 level, 0
    // behind, 0 no winner". Fails under M-A, the apex
    // and bolt series standing behind on its whole
    // file at three front standings against four, its
    // section leaving the report, and the apex and
    // crest figure moving from six to nine, the bolt
    // and crest figure from four to ten, and the crest
    // and drift figure from two to six. Fails under
    // M-B, M-C, and M-D, every decided series standing
    // ahead on the final tally figure, standing on the
    // swapped counts, or carrying one standing too
    // many. Fails under M-F, the apex and bolt series
    // standing at four against a shipped two on its
    // level or in front figure. Passes under M-E, no
    // series of the survey championship standing
    // level.
    let championship = load_example("survey-championship.json");
    let lines = tenure_report(&championship);
    assert!(lines.len() > 2, "the survey golden stands in section form");
    assert_eq!(section_offsets(&lines), vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(section_figures(&lines), vec![2, 6, 5, 4, 5, 2]);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("survey-championship-tenure.txt")
    );
}

#[test]
fn the_league_championship_matches_its_tenure_golden() {
    // M-3. The league championship golden, carrying
    // its sections over the eight ahead series at the
    // figures 2, 3, 5, 6, 3, 2, 4, and 3 in
    // championship order and closing "tenure, 10
    // series, 8 ahead, 1 level, 1 behind, 0 no
    // winner", the apex and ember series standing
    // level on a front count of one against one and
    // the bolt and ember series standing behind on a
    // front count of two against four. Fails under
    // M-A, figures and classes moving across the
    // league on the whole files, the apex and ember
    // series standing ahead and the bolt and ember
    // series standing ahead in its place. Fails under
    // M-B, M-C, and M-D on the shipped figures. Fails
    // under M-E, the level series moving to ahead and
    // standing in a section at the figure one where
    // the golden carries none, the level count of its
    // closing falling to none. Fails under M-F, the
    // apex and bolt series standing at four against a
    // shipped two on its level or in front figure.
    let championship = load_example("league-championship.json");
    let lines = tenure_report(&championship);
    assert!(lines.len() > 2, "the league golden stands in section form");
    assert_eq!(section_offsets(&lines), vec![1, 2, 3, 5, 6, 8, 9, 10]);
    assert_eq!(section_figures(&lines), vec![2, 3, 5, 6, 3, 2, 4, 3]);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("league-championship-tenure.txt")
    );
}

#[test]
fn the_constructed_example_carries_the_first_series_alone() {
    // M-4. The constructed tenure golden, the first
    // series in section form at the figure 5, closing
    // "tenure, 2 series, 1 ahead, 0 level, 1 behind, 0
    // no winner", the first series standing at five
    // front standings against none and the second at
    // one against five. Fails under M-B, the first
    // series' figure moving from five to its win count
    // of four and the second series entering a
    // section. Fails under M-C, the pair standing
    // exchanged, the first series leaving its section
    // and the second entering one at the figure five.
    // Fails under M-D, the first series carrying one
    // standing too many. Fails under M-F, the first
    // series standing at six against five on its level
    // or in front figure. Passes under M-A by design,
    // neither constructed series file holding a race
    // beyond its decision. Passes under M-E, neither
    // series standing level, so the widened boundary
    // moves no class.
    let championship = load_example("constructed-tenure.json");
    let lines = tenure_report(&championship);
    assert_eq!(
        lines,
        vec![
            "tenure",
            "series 1, the constructed tenure drill one",
            "merlin-1 against hobby-1, first to 4 wins",
            "race 1, tenure drill one race one, merlin-1 wins",
            "race 2, tenure drill one race two, hobby-1 wins",
            "race 3, tenure drill one race three, merlin-1 wins",
            "race 4, tenure drill one race four, merlin-1 wins",
            "race 5, tenure drill one race five, hobby-1 wins",
            "race 6, tenure drill one race six, merlin-1 wins",
            "front count 5 over the decided record",
            "series winner, merlin-1, merlin-1 4 race wins, hobby-1 2 race wins, 0 no results",
            "tenure, 2 series, 1 ahead, 0 level, 1 behind, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-tenure-tenure.txt")
    );
}

#[test]
fn the_constructed_example_stands_strung_under_streak() {
    // M-5. The same file under the streak command,
    // both series in section form, closing "streak, 2
    // series, 2 strung, 0 single, 0 no winner".
    // Passes on the pre round tree by design and
    // under M-A to M-F and the classing and champion
    // drop mutations by design, as the header of this
    // file states for the sibling pins.
    let championship = load_example("constructed-tenure.json");
    let lines = streak_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-tenure-streak.txt")
    );
}

#[test]
fn the_constructed_example_stands_clustered_under_cluster() {
    // M-6. The same file under the cluster command,
    // the second series in section form, closing
    // "cluster, 2 series, 1 clustered, 1 scattered, 0
    // no winner". Passes on the pre round tree by
    // design and under the named mutations by design.
    let championship = load_example("constructed-tenure.json");
    let lines = cluster_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-tenure-cluster.txt")
    );
}

#[test]
fn the_constructed_example_stands_a_rally_under_rally() {
    // M-7. The same file under the rally command, the
    // second series in section form, closing "rally,
    // 2 series, 1 rally, 1 wire to wire, 0 no
    // winner". Passes on the pre round tree by design
    // and under the named mutations by design.
    let championship = load_example("constructed-tenure.json");
    let lines = rally_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-tenure-rally.txt")
    );
}

#[test]
fn the_constructed_example_stands_deadlocked_under_deadlock() {
    // M-8. The same file under the deadlock command,
    // both series in section form, closing "deadlock,
    // 2 series, 2 deadlocked, 0 clear, 0 no winner".
    // Passes on the pre round tree by design and
    // under the named mutations by design.
    let championship = load_example("constructed-tenure.json");
    let lines = deadlock_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-tenure-deadlock.txt")
    );
}

#[test]
fn the_constructed_example_stands_the_distance_under_distance() {
    // M-9. The same file under the distance command,
    // the second series in section form, closing
    // "distance, 2 series, 1 the distance, 0 without
    // reply, 1 in between, 0 no winner". Passes on
    // the pre round tree by design and under the
    // named mutations by design.
    let championship = load_example("constructed-tenure.json");
    let lines = distance_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-tenure-distance.txt")
    );
}

#[test]
fn the_constructed_example_stands_from_behind_under_comeback() {
    // M-10. The same file under the comeback command,
    // the second series in section form, closing
    // "comebacks, 2 series, 1 from behind, 1 from the
    // front". Passes on the pre round tree by design
    // and under the named mutations by design.
    let championship = load_example("constructed-tenure.json");
    let lines = comeback_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-tenure-comebacks.txt")
    );
}

#[test]
fn the_constructed_example_stands_in_a_run_under_closing() {
    // M-11. The same file under the closing command,
    // the second series in section form, closing
    // "closing, 2 series, 1 in a run, 1 alone, 0 no
    // winner". Passes on the pre round tree by design
    // and under the named mutations by design.
    let championship = load_example("constructed-tenure.json");
    let lines = closing_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-tenure-closing.txt")
    );
}

#[test]
fn the_constructed_example_stands_alone_under_opening() {
    // M-12. The same file under the opening command,
    // no series in section form, the first series
    // standing alone at an opening run of one and the
    // second standing from behind, closing "opening,
    // 2 series, 0 in a run, 1 alone, 1 from behind, 0
    // no winner". Passes on the pre round tree by
    // design and under the named mutations by design.
    let championship = load_example("constructed-tenure.json");
    let lines = opening_report(&championship);
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-tenure-opening.txt")
    );
}

#[test]
fn a_series_of_target_one_stands_ahead() {
    // M-13. The target one edge. A hand-built series
    // of target one decided by its only decided race
    // stands ahead, its winner's front count standing
    // at one and its loser's at none, its report
    // carrying its section at the figure 1 and
    // closing "tenure, 1 series, 1 ahead, 0 level, 0
    // behind, 0 no winner". Fails under the classing
    // mutation alone among the named mutations besides
    // M-C and M-D, the lone-decider series standing
    // unclassed and its closing moving. Fails under
    // M-C, the series standing behind on the swapped
    // counts. Fails under M-D, its one standing added
    // moving its figure to two. Passes under M-A, M-B,
    // M-E, and M-F by design, its record holding one
    // race, so no figure moves, the series standing
    // ahead on both figures, at the widened boundary,
    // and at no level standing of its winner.
    let dir = temp_dir("tenure-target-one");
    let series = series_json(
        "the target one tenure saga",
        vec![ALPHA, BETA],
        1,
        vec![clear_win("race one", ALPHA, BETA)],
    );
    let lines = tenure_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "tenure",
            "series 1, the target one tenure saga",
            "alpha-1 against beta-1, first to 1 wins",
            "race 1, race one, alpha-1 wins",
            "front count 1 over the decided record",
            "series winner, alpha-1, alpha-1 1 race wins, beta-1 0 race wins, 0 no results",
            "tenure, 1 series, 1 ahead, 0 level, 0 behind, 0 no winner",
        ]
    );
}

#[test]
fn an_interposed_no_result_leaves_the_tenure_whole() {
    // M-14. The decided record read whole. A
    // hand-built series at a target of three whose
    // record interposes two races holding no winner
    // among four counted decided races won by alpha,
    // beta, alpha, alpha, with a fifth decided race
    // won by beta standing beyond the decision, stands
    // ahead in section form at the figure 3, closing
    // "tenure, 1 series, 1 ahead, 0 level, 0 behind, 0
    // no winner", and the turns report over the same
    // championship closes turned at the figure two,
    // proving the interposed races left the decided
    // record the tenure reading reads whole and in
    // order. Fails under M-A, the fifth decided race
    // standing beyond the decision raising its front
    // count from three to four. Fails under M-C and
    // M-D on the swapped and added counts. Fails under
    // M-F, its level or in front figure standing at
    // four against its front count of three. Passes
    // under M-B as a stated coincidence, its front
    // count of three standing equal to its win count
    // of three. Passes under M-E, the series standing
    // ahead on either boundary.
    let dir = temp_dir("tenure-interposed");
    let series = series_json(
        "the interposed tenure saga",
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
    let lines = tenure_report(&championship);
    assert_eq!(
        lines,
        vec![
            "tenure",
            "series 1, the interposed tenure saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, alpha-1 wins",
            "race 3, race three, beta-1 wins",
            "race 5, race five, alpha-1 wins",
            "race 6, race six, alpha-1 wins",
            "front count 3 over the decided record",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 1 race wins, 2 no results",
            "tenure, 1 series, 1 ahead, 0 level, 0 behind, 0 no winner",
        ]
    );
    let turns = turns_report(&championship);
    assert_eq!(
        turns,
        vec![
            "turns",
            "series 1, the interposed tenure saga",
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

/// The figures of the sections of one tenure report,
/// in section order.
fn section_figures(lines: &[String]) -> Vec<u64> {
    lines
        .iter()
        .filter(|line| line.starts_with("front count "))
        .map(|line| {
            line.split(' ')
                .nth(2)
                .expect("a figure")
                .parse::<u64>()
                .expect("a figure")
        })
        .collect()
}

/// One side of a tally line of a series report, its
/// model name and its count of race wins, read from
/// the field that names the model, its count, and the
/// words wins.
fn tally_split(field: &str) -> (String, u64) {
    let body = field.strip_suffix(" wins").expect("a wins field");
    let (model, count) = body.rsplit_once(' ').expect("a model and its count");
    (model.to_string(), count.parse::<u64>().expect("a count"))
}

/// The front counts of the winner and the loser of one
/// series, read from the tally lines of its series
/// report alone. Each tally line whose two counts move
/// from the standing before it records one decided
/// race, and a standing counts for the model whose
/// tally stands above the other's after it.
fn front_counts(series: &deadheat::series::Series) -> (u64, u64) {
    let report = series_report(series);
    let champion = report
        .last()
        .expect("a winner line")
        .strip_prefix("series winner, ")
        .expect("a series winner")
        .to_string();
    let mut previous: Option<(u64, u64)> = None;
    let mut front: [u64; 2] = [0, 0];
    let mut first_model = String::new();
    for line in &report {
        if let Some(tally) = line.strip_prefix("tally, ") {
            let fields: Vec<&str> = tally.split(", ").collect();
            let (model, first_wins) = tally_split(fields[0]);
            let (_, second_wins) = tally_split(fields[1]);
            first_model = model;
            let standing = (first_wins, second_wins);
            if previous == Some(standing) {
                continue;
            }
            previous = Some(standing);
            if first_wins > second_wins {
                front[0] += 1;
            } else if second_wins > first_wins {
                front[1] += 1;
            }
        }
    }
    if champion == first_model {
        (front[0], front[1])
    } else {
        (front[1], front[0])
    }
}

/// The stated relations of DH-46 and DH-47 over one
/// championship, read from the printed readings alone.
/// Both championships this helper reads hold a winner
/// in every series.
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
    let tenure = tenure_report(championship);
    let counts = closing_counts(tenure.last().expect("a closing"));
    assert_eq!(counts[0], series_count);
    assert_eq!(
        counts[1] + counts[2] + counts[3],
        decided,
        "the three classes"
    );
    assert_eq!(counts[1] + counts[2] + counts[3] + counts[4], series_count);
    let sections = section_offsets(&tenure);
    assert_eq!(
        sections.len() as u64,
        counts[1],
        "one section per ahead series"
    );
    // The no winner count equals the figures of the
    // eleven sibling closings DH-47 names, the comeback
    // leftover among them, because the twelve readings
    // read one walk.
    let turns = turns_report(championship);
    let turns_counts = closing_counts(turns.last().expect("a closing"));
    assert_eq!(counts[4], turns_counts[3], "the turns no winner figure");
    let lead = lead_report(championship);
    let lead_counts = closing_counts(lead.last().expect("a closing"));
    assert_eq!(counts[4], lead_counts[3], "the lead no winner figure");
    let crossings = crossings_report(championship);
    let crossings_counts = closing_counts(crossings.last().expect("a closing"));
    assert_eq!(
        counts[4], crossings_counts[3],
        "the crossings no winner figure"
    );
    let distance = distance_report(championship);
    let distance_counts = closing_counts(distance.last().expect("a closing"));
    assert_eq!(
        counts[4], distance_counts[4],
        "the distance no winner figure"
    );
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
    let opening = opening_report(championship);
    let opening_counts = closing_counts(opening.last().expect("a closing"));
    assert_eq!(counts[4], opening_counts[4], "the opening no winner figure");
    // The classes read from the tally lines of the
    // series reports stand in the sections and counts
    // the tenure report prints, every series standing
    // behind standing in a rally section over the same
    // championship, and every series standing level or
    // behind standing in a deadlock section, the
    // consequences DH-46 states.
    let rally_sections = section_offsets(&rally);
    let deadlock_sections = section_offsets(&deadlock);
    for (index, series) in championship.series.iter().enumerate() {
        let offset = index as u64 + 1;
        let (winner_front, loser_front) = front_counts(series);
        if winner_front > loser_front {
            assert!(
                sections.contains(&offset),
                "series {offset} stands in a tenure section"
            );
        } else if winner_front == loser_front {
            assert!(
                deadlock_sections.contains(&offset),
                "series {offset} stands deadlocked"
            );
        } else {
            assert!(
                rally_sections.contains(&offset),
                "series {offset} stands a rally"
            );
            assert!(
                deadlock_sections.contains(&offset),
                "series {offset} stands deadlocked"
            );
        }
    }
    // The count of behind series stands at or below
    // the count of rally series in the rally closing,
    // and the count of level and behind series together
    // stands at or below the count of deadlocked series
    // in the deadlock closing, over the same
    // championship.
    assert!(
        counts[3] <= rally_counts[1],
        "the behind count stands at or below the rally count"
    );
    assert!(
        counts[2] + counts[3] <= deadlock_counts[1],
        "the level and behind counts stand at or below the deadlocked count"
    );
}

#[test]
fn the_tenure_counts_stand_in_their_stated_relations() {
    // M-15. The relations pin, asserting over the
    // survey and league championships test by test
    // the relations DH-46 and DH-47 state, the ahead
    // and level and behind counts and the no winner
    // count summing to the championship report's
    // series count over the same files, the no winner
    // count standing equal across the sibling closings
    // at the grain those closings print it, every
    // series standing behind in the tenure closing
    // standing in a rally section over the same
    // championship, the count of level and behind
    // series together standing at or below the count
    // of deadlocked series in the deadlock closing
    // over the same championship, and the count of
    // behind series standing at or below the count of
    // rally series in the rally closing over the same
    // championship. Fails under M-C, the league's
    // behind count standing at eight against a rally
    // count of five and its inclusion breaking.
    // Passes under M-A as a stated coincidence, the
    // relations surviving the mutation on the shipped
    // championships, the sums and the no winner
    // equality standing untouched and the behind count
    // standing at one against a rally count of four in
    // the survey and at none in the league, the golden
    // pins carrying the mutation. Passes under M-B as
    // a stated coincidence, every decided series
    // standing ahead under the mutation, so its
    // inclusions hold and its sums stand. Passes under
    // M-D as a stated coincidence, its relations
    // surviving with the apex and ember series
    // standing ahead in the league and the bolt and
    // ember series standing behind still. Passes under
    // M-E as a stated coincidence, its inclusions
    // surviving with the level class standing empty.
    // Passes under M-F as a stated coincidence, the
    // apex and ember series standing ahead and the
    // bolt and ember series standing level under the
    // mutation and its inclusions holding.
    check_relations(&load_example("survey-championship.json"), 6);
    check_relations(&load_example("league-championship.json"), 10);
}

#[test]
fn a_series_level_on_front_counts_stands_level() {
    // M-16. The level case at hand scale. A hand-built
    // series at a target of four won by alpha, beta,
    // beta, alpha, beta, alpha, alpha, its winner's
    // front count standing at two against its loser's
    // two with three level standings, stands level
    // with no section, closing "tenure, 1 series, 0
    // ahead, 1 level, 0 behind, 0 no winner", the
    // turns report over the same championship carrying
    // its section at the figure four and the deadlock
    // report counting it deadlocked, proving a level
    // standing of the front counts stands apart from
    // the turns figure and inside the deadlock class.
    // Fails under M-B, the series standing ahead on
    // the final tally figure with a section where the
    // pin carries none. Fails under M-D, the series
    // standing ahead at the figure three. Fails under
    // M-E, the level series moving to ahead. Fails
    // under M-F, the series standing ahead with a
    // section at the figure five. Passes under M-A by
    // design, no race standing beyond its decision.
    // Passes under M-C as a stated coincidence, the
    // level class standing fixed under the swap and
    // its printed figure standing at two on either
    // count.
    let dir = temp_dir("tenure-level");
    let series = series_json(
        "the level tenure saga",
        vec![ALPHA, BETA],
        4,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", BETA, ALPHA),
            clear_win("race six", ALPHA, BETA),
            clear_win("race seven", ALPHA, BETA),
        ],
    );
    let championship = load(&dir, vec![("one.json", series)], vec!["one.json"]);
    let lines = tenure_report(&championship);
    assert_eq!(
        lines,
        vec![
            "tenure",
            "tenure, 1 series, 0 ahead, 1 level, 0 behind, 0 no winner"
        ]
    );
    let turns = turns_report(&championship);
    assert_eq!(
        turns,
        vec![
            "turns",
            "series 1, the level tenure saga",
            "alpha-1 against beta-1, first to 4 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, beta-1 wins",
            "race 4, race four, alpha-1 wins",
            "race 5, race five, beta-1 wins",
            "race 6, race six, alpha-1 wins",
            "race 7, race seven, alpha-1 wins",
            "changes of hand 4 over the decided record",
            "series winner, alpha-1, alpha-1 4 race wins, beta-1 3 race wins, 0 no results",
            "turns, 1 series, 1 turned, 0 steady, 0 no winner",
        ]
    );
    let deadlock = deadlock_report(&championship);
    assert_eq!(
        deadlock,
        vec![
            "deadlock",
            "series 1, the level tenure saga",
            "alpha-1 against beta-1, first to 4 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, beta-1 wins",
            "race 4, race four, alpha-1 wins",
            "race 5, race five, beta-1 wins",
            "race 6, race six, alpha-1 wins",
            "race 7, race seven, alpha-1 wins",
            "stood level after 3 of its decided races",
            "series winner, alpha-1, alpha-1 4 race wins, beta-1 3 race wins, 0 no results",
            "deadlock, 1 series, 1 deadlocked, 0 clear, 0 no winner",
        ]
    );
}

#[test]
fn a_series_behind_on_front_counts_stands_behind() {
    // M-17. The behind case at hand scale. A
    // hand-built series at a target of four won by
    // beta, beta, alpha, beta, alpha, alpha, alpha,
    // its winner's front count standing at one against
    // its loser's five, stands behind with no section,
    // closing "tenure, 1 series, 0 ahead, 0 level, 1
    // behind, 0 no winner", the rally report over the
    // same championship counting it a rally and the
    // crossings report counting it short, proving the
    // behind class is neither the rally class nor the
    // short class restated. Fails under M-B, the
    // series standing ahead with a section where the
    // pin carries none. Fails under M-C, the series
    // standing ahead with a section at the figure
    // five. Passes under M-A by design, no race
    // standing beyond its decision. Passes under M-D,
    // the series standing behind on two against five
    // as on one against five and its closing standing
    // unchanged. Passes under M-E, the widened boundary
    // reaching no series whose counts stand apart.
    // Passes under M-F, the series standing behind on
    // two against five as on one against five, its one
    // level standing raising its figure without moving
    // its class or its closing.
    let dir = temp_dir("tenure-behind");
    let series = series_json(
        "the behind tenure saga",
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
    let lines = tenure_report(&championship);
    assert_eq!(
        lines,
        vec![
            "tenure",
            "tenure, 1 series, 0 ahead, 0 level, 1 behind, 0 no winner"
        ]
    );
    let rally = rally_report(&championship);
    assert_eq!(
        rally,
        vec![
            "rally",
            "series 1, the behind tenure saga",
            "alpha-1 against beta-1, first to 4 wins",
            "race 1, race one, beta-1 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, alpha-1 wins",
            "race 4, race four, beta-1 wins",
            "race 5, race five, alpha-1 wins",
            "race 6, race six, alpha-1 wins",
            "race 7, race seven, alpha-1 wins",
            "stood behind by 2 race wins at the deepest",
            "series winner, alpha-1, alpha-1 4 race wins, beta-1 3 race wins, 0 no results",
            "rally, 1 series, 1 rally, 0 wire to wire, 0 no winner",
        ]
    );
    let crossings = crossings_report(&championship);
    assert_eq!(
        crossings,
        vec![
            "crossings",
            "crossings, 1 series, 0 through, 1 short, 0 no winner"
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

/// The handicap twin of the tenure pins. On scratch
/// finishes the decided races run beta, alpha, alpha,
/// beta, beta at a target of three, beta's series
/// standing ahead at a front count of two against
/// one. Under an adjustment on alpha's attempt in the
/// third race the same finishes return that race to
/// beta, whose record runs beta, alpha, beta, beta
/// and decides at its fourth decided race, standing
/// ahead at a front count of three against none.
fn handicap_series(adjusted: bool) -> JsonValue {
    series_json(
        "the handicap tenure saga",
        vec![ALPHA, BETA],
        3,
        vec![
            beta_first("race one"),
            alpha_first("race two", false),
            alpha_first("race three", adjusted),
            beta_first("race four"),
            beta_first("race five"),
        ],
    )
}

#[test]
fn a_handicap_adjustment_moves_the_front_count() {
    // M-18. The handicap restatement. The twin pair
    // over identical recorded finishes at a target of
    // three, the scratch arm won by beta, alpha,
    // alpha, beta, beta standing ahead at a front
    // count of two against one with its section at the
    // figure 2, and the adjusted arm, the same
    // finishes under an adjustment returning the third
    // race to beta, its walk deciding at its fourth
    // decided race standing ahead at a front count of
    // three against none with its section at the
    // figure 3, the pair proving the classification
    // reads the adjusted record the walk holds and not
    // the recorded one. Fails under M-A on its
    // adjusted arm, whose races beyond its adjusted
    // decision raise its front count from three to
    // four. Fails under M-B, its scratch figure moving
    // from two to its win count of three. Fails under
    // M-C and M-D on the swapped and added counts.
    // Fails under M-F, its scratch arm standing at
    // four against two on its level or in front
    // figure. Passes under M-E, both arms standing
    // ahead on either boundary.
    let dir = temp_dir("tenure-handicap");
    let plain = tenure_for(
        &dir,
        vec![("one.json", handicap_series(false))],
        vec!["one.json"],
    );
    assert_eq!(
        plain,
        vec![
            "tenure",
            "series 1, the handicap tenure saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, beta-1 wins",
            "race 2, race two, alpha-1 wins",
            "race 3, race three, alpha-1 wins",
            "race 4, race four, beta-1 wins",
            "race 5, race five, beta-1 wins",
            "front count 2 over the decided record",
            "series winner, beta-1, alpha-1 2 race wins, beta-1 3 race wins, 0 no results",
            "tenure, 1 series, 1 ahead, 0 level, 0 behind, 0 no winner",
        ]
    );
    let adjusted = tenure_for(
        &dir,
        vec![("one.json", handicap_series(true))],
        vec!["one.json"],
    );
    assert_eq!(
        adjusted,
        vec![
            "tenure",
            "series 1, the handicap tenure saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, beta-1 wins",
            "race 2, race two, alpha-1 wins",
            "race 3, race three, beta-1 wins",
            "race 4, race four, beta-1 wins",
            "front count 3 over the decided record",
            "series winner, beta-1, alpha-1 1 race wins, beta-1 3 race wins, 0 no results",
            "tenure, 1 series, 1 ahead, 0 level, 0 behind, 0 no winner",
        ]
    );
}

#[test]
fn a_championship_holding_no_winner_stands_in_no_class() {
    // M-19. The no winner pin. A hand-built
    // championship of two series holding no winner,
    // the first holding decided races won by alpha,
    // beta, alpha, beta at a target of three with its
    // tallies standing level after its second and
    // fourth decided races and its target unreached,
    // the second holding no decided race, carries no
    // section and closes "tenure, 2 series, 0 ahead, 0
    // level, 0 behind, 2 no winner", the first series
    // standing in none of the three classes even where
    // its two front counts over its counted races stand
    // apart or equal. Fails under the champion drop
    // mutation alone, its first series leaving the no
    // winner count, classed ahead on the record of
    // alpha, the model holding the most decided wins
    // standing tied and read first. Passes under M-A
    // to M-F and the classing mutation by design,
    // neither series holding a winner for any of them
    // to class, and neither holding a deciding race
    // for the classing mutation to reach.
    let dir = temp_dir("tenure-open");
    let first = series_json(
        "the open tenure saga",
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
        "the silent tenure saga",
        vec![GAMMA, DELTA],
        3,
        vec![no_result("race one", GAMMA, DELTA)],
    );
    let lines = tenure_for(
        &dir,
        vec![("one.json", first), ("two.json", second)],
        vec!["one.json", "two.json"],
    );
    assert_eq!(
        lines,
        vec![
            "tenure",
            "tenure, 2 series, 0 ahead, 0 level, 0 behind, 2 no winner",
        ]
    );
}

#[test]
fn a_collection_holding_no_series_renders_the_empty_form_at_the_library_surface() {
    // M-20. The empty form, in the sibling form of the
    // lead reading's K-20 and the crossings reading's
    // L-20. No championship file carries an empty
    // collection, because the loader of DH-13 refuses
    // a series list holding no names, so this form
    // stands at the library surface alone, over a
    // collection built by hand.
    let championship = Championship { series: vec![] };
    let lines = tenure_report(&championship);
    assert_eq!(
        lines,
        vec![
            "tenure",
            "tenure, 0 series, 0 ahead, 0 level, 0 behind, 0 no winner",
        ]
    );
}
