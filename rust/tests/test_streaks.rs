//! Pins for DH-32 and DH-33 in the Round 14 section of
//! CONTRACT.md.
//!
//! The discrimination record of this file stands pin by
//! pin, and no blanket claim stands over it. The streak
//! reading, its library function, and its command do not
//! stand on the pre round tree, so every pin in this file
//! is absent there at compile time. That absence is
//! recorded once here and is not a discrimination claim.
//! The discrimination each pin carries is against the
//! stated mutation of DH-32 beside it. Five mutations are
//! named for the whole file. M-A, the early stop removed,
//! the runs and the sections read over every recorded
//! race, races beyond the decision among them. M-B, a no
//! result race breaking a run. M-C, the closing run read
//! in place of the longest run. M-D, either model's
//! longest run read in place of the winner's. M-E, the
//! deciding race set aside before the runs are read.
//! Four pins, the constructed file under the distance,
//! deadlock, rally, and comeback commands, pass against
//! the pre round surface by design, as their own comments
//! state.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{championship_report, load_championship_file, Championship};
use deadheat::comebacks::comeback_report;
use deadheat::deadlocks::deadlock_report;
use deadheat::distances::distance_report;
use deadheat::json::JsonValue;
use deadheat::rallies::rally_report;
use deadheat::streaks::streak_report;
use num_bigint::BigInt;
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

fn streak_for(dir: &Path, files: Vec<(&str, JsonValue)>, names: Vec<&str>) -> Vec<String> {
    streak_report(&load(dir, files, names))
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
fn the_standing_championship_renders_both_series_strung() {
    // S-1. The standing championship in section form, both
    // of its series standing strung, the first
    // classification reading whose marked class covers
    // both. The pilot figure stands at 2 on runs of two
    // and one over the decided winners prime-1, prime-1,
    // shadow-1, prime-1, and the panel figure stands at 3
    // on the decided record prime-1, prime-1, prime-1, the
    // two races holding no result neither extending nor
    // breaking the run. Fails under M-C, under which the
    // pilot's closing run stands at 1 and the series
    // stands single with no section. Fails under M-E,
    // under which the panel's deciding race stands set
    // aside and its figure reads 2. Fails under M-B,
    // under which the pilot's counted record, its winners
    // standing between three no results before its second
    // win and one no result before its last, breaks every
    // run at 1 and the series stands single.
    let championship = load_example("championship.json");
    let lines = streak_report(&championship);
    assert_eq!(
        lines,
        vec![
            "streak",
            "series 1, the recorded close call pilot",
            "prime-1 against shadow-1, first to 3 wins",
            "race 1, pin the cache key, prime-1 wins",
            "race 5, name the worker pool, prime-1 wins",
            "race 7, shrink the prompt budget, shadow-1 wins",
            "race 9, sort the seed list, prime-1 wins",
            "longest run of 2 consecutive decided race wins",
            "series winner, prime-1, prime-1 3 race wins, shadow-1 1 race wins, 5 no results",
            "series 2, the recorded close call panel",
            "prime-1 against shadow-1, first to 3 wins",
            "race 1, pin the cache key, prime-1 wins",
            "race 2, trim the import list, prime-1 wins",
            "race 5, name the worker pool, prime-1 wins",
            "longest run of 3 consecutive decided race wins",
            "series winner, prime-1, prime-1 3 race wins, shadow-1 0 race wins, 2 no results",
            "streak, 2 series, 2 strung, 0 single, 0 no winner",
        ]
    );
    assert_eq!(lines.join("\n") + "\n", golden("championship-streak.txt"));
}

#[test]
fn the_survey_championship_matches_its_streak_golden() {
    // S-2. The section form over the survey championship,
    // pinned byte for byte against the golden the real
    // command produced. Its stated failure mode is golden
    // drift, and the mutation discrimination over this
    // fixture stands in the relations pin below, carried
    // jointly with it.
    let championship = load_example("survey-championship.json");
    let lines = streak_report(&championship);
    assert!(lines.len() > 2, "the survey golden stands in section form");
    assert_eq!(
        lines.join("\n") + "\n",
        golden("survey-championship-streak.txt")
    );
}

#[test]
fn the_constructed_example_parts_strung_from_single() {
    // S-3. The round's headline discrimination. The first
    // constructed series stands strung at figure 2 while
    // standing won from the front, in between, wire to
    // wire, and clear, the unmarked class of every
    // sibling, and the second stands single while
    // standing the distance and deadlocked, so the streak
    // golden carries the first series alone. Fails under
    // M-C, under which the first series' closing run
    // stands at 1 and the report falls to the empty form.
    // Fails under M-A, under which the second series'
    // sixth recorded race strings its winner's runs to 2
    // and the report gains its section.
    let championship = load_example("constructed-streak.json");
    let lines = streak_report(&championship);
    assert_eq!(
        lines,
        vec![
            "streak",
            "series 1, the constructed streak drill one",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, streak drill one race one, alpha-1 wins",
            "race 2, streak drill one race two, alpha-1 wins",
            "race 3, streak drill one race three, beta-1 wins",
            "race 4, streak drill one race four, alpha-1 wins",
            "longest run of 2 consecutive decided race wins",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 1 race wins, 0 no results",
            "streak, 2 series, 1 strung, 1 single, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-streak-streak.txt")
    );
}

#[test]
fn the_constructed_example_stands_the_distance_under_distance() {
    // S-4. The same file under the distance command, the
    // second series standing the distance in its section
    // alone. Passes on the pre round tree by design, in
    // the posture of DH-30's goldens over standing
    // commands, the distance reading and its command
    // standing built there. The pin holds that the
    // round's new fixture does not move a standing
    // reading and that the two readings part over the
    // same file, read against the pin above.
    let championship = load_example("constructed-streak.json");
    let lines = distance_report(&championship);
    assert_eq!(
        lines,
        vec![
            "distance",
            "series 2, the constructed streak drill two",
            "gamma-1 against delta-1, first to 3 wins",
            "race 1, streak drill two race one, gamma-1 wins",
            "race 2, streak drill two race two, delta-1 wins",
            "race 3, streak drill two race three, gamma-1 wins",
            "race 4, streak drill two race four, delta-1 wins",
            "race 5, streak drill two race five, gamma-1 wins",
            "series winner, gamma-1, gamma-1 3 race wins, delta-1 2 race wins, 0 no results",
            "distance, 2 series, 1 the distance, 0 without reply, 1 in between, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-streak-distance.txt")
    );
}

#[test]
fn the_constructed_example_stands_deadlocked_under_deadlock() {
    // S-5. The same file under the deadlock command, the
    // second series standing deadlocked in its section
    // alone, its tallies having stood level after its
    // second and fourth decided races. Passes on the pre
    // round tree by design, in the posture of the pin
    // above, the deadlock reading and its command
    // standing built in Round 13.
    let championship = load_example("constructed-streak.json");
    let lines = deadlock_report(&championship);
    assert_eq!(
        lines,
        vec![
            "deadlock",
            "series 2, the constructed streak drill two",
            "gamma-1 against delta-1, first to 3 wins",
            "race 1, streak drill two race one, gamma-1 wins",
            "race 2, streak drill two race two, delta-1 wins",
            "race 3, streak drill two race three, gamma-1 wins",
            "race 4, streak drill two race four, delta-1 wins",
            "race 5, streak drill two race five, gamma-1 wins",
            "stood level after 2 of its decided races",
            "series winner, gamma-1, gamma-1 3 race wins, delta-1 2 race wins, 0 no results",
            "deadlock, 2 series, 1 deadlocked, 1 clear, 0 no winner",
        ]
    );
    assert_eq!(
        lines.join("\n") + "\n",
        golden("constructed-streak-deadlock.txt")
    );
}

#[test]
fn the_constructed_example_stands_wire_to_wire_under_rally() {
    // S-6. The same file under the rally command, the
    // empty form, neither winner ever standing behind.
    // Passes on the pre round tree by design, in the
    // posture of the pins above.
    let championship = load_example("constructed-streak.json");
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
        golden("constructed-streak-rally.txt")
    );
}

#[test]
fn the_constructed_example_stands_from_the_front_under_comeback() {
    // S-7. The same file under the comeback command, the
    // empty form, each winner having won its first
    // decided race. Passes on the pre round tree by
    // design, in the posture of the pins above.
    let championship = load_example("constructed-streak.json");
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
        golden("constructed-streak-comebacks.txt")
    );
}

#[test]
fn a_series_of_target_one_stands_single() {
    // S-8. The only decided race of a decided series of
    // target one is its deciding race, so its longest run
    // stands at 1 and the series stands single against
    // DH-32's stated consequence. Under M-E the pin
    // passes, on the executed record of three passes.
    // M-E sets the deciding race aside before the runs
    // are read and touches nothing else, so the
    // champion stands, the run reads zero over the
    // emptied record, and a decided series classes
    // single at any longest run below two, which
    // leaves the closing exactly as the pin asserts
    // it. The no winner outcome belongs to a
    // champion-dropping variant that is neither M-E
    // as this file states it nor DH-32, and the pin
    // fails under that variant, as it fails under a
    // reading that counts the deciding race into the
    // run and stands the series strung by it.
    let dir = temp_dir("streaks-target-one");
    let series = series_json(
        "the single saga",
        vec![ALPHA, BETA],
        1,
        vec![clear_win("race one", BETA, ALPHA)],
    );
    let lines = streak_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "streak",
            "streak, 1 series, 0 strung, 1 single, 0 no winner",
        ]
    );
}

#[test]
fn an_interposed_no_result_leaves_a_run_whole() {
    // S-9. The plan's M-B discriminator. A series at
    // target two whose counted races run winner, no
    // result, winner stands strung with figure 2, the two
    // winner wins consecutive in the decided record.
    // Fails under M-B, under which the no result breaks
    // the run, the runs stand at 1 and 1, and the series
    // stands single.
    let dir = temp_dir("streaks-interposed");
    let series = series_json(
        "the quiet middle saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", ALPHA, BETA),
            no_result("race two", ALPHA, BETA),
            clear_win("race three", ALPHA, BETA),
        ],
    );
    let lines = streak_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "streak",
            "series 1, the quiet middle saga",
            "alpha-1 against beta-1, first to 2 wins",
            "race 1, race one, alpha-1 wins",
            "race 3, race three, alpha-1 wins",
            "longest run of 2 consecutive decided race wins",
            "series winner, alpha-1, alpha-1 2 race wins, beta-1 0 race wins, 1 no results",
            "streak, 1 series, 1 strung, 0 single, 0 no winner",
        ]
    );
}

#[test]
fn an_interposed_no_result_leaves_the_section_naming_decided_races_only() {
    // S-10. A series at target three, decided winners
    // winner, loser, winner, winner, with a no result
    // standing between the second and third decided
    // races, stands strung with figure 2, its section
    // naming the four decided races and the closing split
    // counting its one no result. Fails under the
    // mutation that walks every counted race into the
    // section lines, which names five races. Under M-B
    // the pin passes by construction, the no result
    // standing between a loser win and the winner's next
    // win where the winner holds no open run for it to
    // break, and the file's M-B work stands in S-9.
    let dir = temp_dir("streaks-interposed-section");
    let series = series_json(
        "the interposed streak saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            no_result("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
        ],
    );
    let lines = streak_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "streak",
            "series 1, the interposed streak saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, beta-1 wins",
            "race 4, race four, alpha-1 wins",
            "race 5, race five, alpha-1 wins",
            "longest run of 2 consecutive decided race wins",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 1 race wins, 1 no results",
            "streak, 1 series, 1 strung, 0 single, 0 no winner",
        ]
    );
}

#[test]
fn the_alternating_record_stands_single_and_its_twin_strung() {
    // S-11. The characterization pair of DH-32. The
    // alternating series winner, loser, winner, loser,
    // winner at target three stands single and holds no
    // section. Its near twin winner, loser, winner,
    // winner at target three, decided at the fourth
    // decided race, stands strung with figure 2. The pair
    // fails under any reading that classifies from the
    // final tallies, which classes the two alike, each
    // ending three against two at target three, the
    // distance shape in both, and so cannot part them.
    // Under M-C both halves pass by design, the
    // alternating closing run and longest run both
    // standing at 1 and the twin's both standing at 2,
    // and the pair's M-C posture is carried by S-1 and
    // S-3, as the plan states.
    let dir = temp_dir("streaks-alternating");
    let alternating = series_json(
        "the alternating saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", BETA, ALPHA),
            clear_win("race five", ALPHA, BETA),
        ],
    );
    let lines = streak_for(&dir, vec![("one.json", alternating)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "streak",
            "streak, 1 series, 0 strung, 1 single, 0 no winner",
        ]
    );
    let twin = series_json(
        "the near twin saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
        ],
    );
    let lines = streak_for(&dir, vec![("one.json", twin)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "streak",
            "series 1, the near twin saga",
            "alpha-1 against beta-1, first to 3 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, alpha-1 wins",
            "race 4, race four, alpha-1 wins",
            "longest run of 2 consecutive decided race wins",
            "series winner, alpha-1, alpha-1 3 race wins, beta-1 1 race wins, 0 no results",
            "streak, 1 series, 1 strung, 0 single, 0 no winner",
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

/// The figures of the sections of one streak report, in
/// section order.
fn section_figures(lines: &[String]) -> Vec<u64> {
    lines
        .iter()
        .filter(|line| line.starts_with("longest run of "))
        .map(|line| {
            line.split(' ')
                .nth(3)
                .expect("a figure")
                .parse::<u64>()
                .expect("a figure")
        })
        .collect()
}

/// The stated relations of DH-33 over one championship,
/// read from the printed readings alone, in the form the
/// deadlock relations pin states them. The singles stand
/// named by their championship offsets, each standing in
/// no streak section and, its target standing above one,
/// standing the distance in a distance section.
fn check_relations(
    championship: &Championship,
    series_count: u64,
    figures: Vec<u64>,
    singles: Vec<u64>,
) {
    let table = championship_report(championship);
    let decided = table
        .iter()
        .filter(|line| line.starts_with("series winner,"))
        .count() as u64;
    assert!(decided > 0, "the fixture holds decided series");
    let streak = streak_report(championship);
    let counts = closing_counts(streak.last().expect("a closing"));
    assert_eq!(counts[0], series_count);
    assert_eq!(counts[1] + counts[2], decided, "strung and single");
    assert_eq!(counts[1] + counts[2] + counts[3], series_count);
    assert_eq!(
        section_figures(&streak),
        figures,
        "the section figures in order"
    );
    let sections = section_offsets(&streak);
    assert_eq!(
        sections.len() as u64,
        counts[1],
        "one section per strung series"
    );
    // The no winner count equals the distance closing's
    // figure outright, the count neither comeback class
    // holds, and the no winner figures of the rally and
    // deadlock closings.
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
    // Every rally series stands strung, its winner having
    // stood behind, which no alternating record does, so
    // the rally count stands at or below the strung
    // count.
    assert!(
        rally_counts[1] <= counts[1],
        "the rally count stands at or below the strung count"
    );
    // Every single series whose target stands above one
    // stands the distance, its loser standing one win
    // short of the target.
    let distance_sections = section_offsets(&distance);
    for offset in singles {
        assert!(
            !sections.contains(&offset),
            "series {offset} stands in no streak section"
        );
        assert!(
            championship.series[(offset - 1) as usize].wins_target > BigInt::from(1),
            "series {offset} stands above target one"
        );
        assert!(
            distance_sections.contains(&offset),
            "series {offset} stands the distance"
        );
    }
}

#[test]
fn the_closing_counts_stand_in_their_stated_relations() {
    // S-12. The relations of DH-33 over the survey
    // championship and the league championship of DH-31,
    // computed over the same readings. Fails under M-C,
    // naming the series where the longest and closing
    // runs part, the survey's apex-1 and crest-1 series,
    // longest 3 against closing 1, and the league's
    // apex-1 and bolt-1, apex-1 and drift-1, and bolt-1
    // and crest-1 series, each standing single or
    // carrying a moved figure under M-C, so the section
    // figures and memberships the pin cross-reads move.
    // Fails under M-A, under which the league's crest-1
    // and drift-1 series, the league's one single series,
    // reads a longest run of 4 over its full recorded
    // races and stands strung, and the survey's crest-1
    // and drift-1 figure moves from 3 to 7. Fails under
    // M-E, under which the survey's apex-1 and bolt-1
    // figure moves from 2 to 1 and the series stands
    // single.
    check_relations(
        &load_example("survey-championship.json"),
        6,
        vec![2, 3, 2, 3, 4, 3],
        vec![],
    );
    check_relations(
        &load_example("league-championship.json"),
        10,
        vec![2, 3, 4, 2, 3, 3, 3, 3, 3],
        vec![8],
    );
}

#[test]
fn the_section_figure_reads_the_winners_longest_run() {
    // S-13. The plan's M-D discriminator. A series at
    // target six whose decided winners run loser, loser,
    // loser, winner, winner, loser, winner, winner,
    // loser, winner, winner. The winner's runs stand at
    // 2, 2, and 2, the longest at 2, and the loser's
    // stand at 3, 1, and 1, the longest at 3. The series
    // stands strung and its section figure reads 2.
    // Fails under M-D, which prints 3. Neither tally
    // reaches the target before the final race, so the
    // walk's stop stands at the last decided race.
    let dir = temp_dir("streaks-winners-run");
    let series = series_json(
        "the long haul saga",
        vec![ALPHA, BETA],
        6,
        vec![
            clear_win("race one", BETA, ALPHA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", BETA, ALPHA),
            clear_win("race four", ALPHA, BETA),
            clear_win("race five", ALPHA, BETA),
            clear_win("race six", BETA, ALPHA),
            clear_win("race seven", ALPHA, BETA),
            clear_win("race eight", ALPHA, BETA),
            clear_win("race nine", BETA, ALPHA),
            clear_win("race ten", ALPHA, BETA),
            clear_win("race eleven", ALPHA, BETA),
        ],
    );
    let lines = streak_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "streak",
            "series 1, the long haul saga",
            "alpha-1 against beta-1, first to 6 wins",
            "race 1, race one, beta-1 wins",
            "race 2, race two, beta-1 wins",
            "race 3, race three, beta-1 wins",
            "race 4, race four, alpha-1 wins",
            "race 5, race five, alpha-1 wins",
            "race 6, race six, beta-1 wins",
            "race 7, race seven, alpha-1 wins",
            "race 8, race eight, alpha-1 wins",
            "race 9, race nine, beta-1 wins",
            "race 10, race ten, alpha-1 wins",
            "race 11, race eleven, alpha-1 wins",
            "longest run of 2 consecutive decided race wins",
            "series winner, alpha-1, alpha-1 6 race wins, beta-1 5 race wins, 0 no results",
            "streak, 1 series, 1 strung, 0 single, 0 no winner",
        ]
    );
}

/// The handicap twin of the streak pins. On scratch
/// finishes the series runs winner, loser, winner at
/// target two and stands single. The second race's
/// first model attempt carries an adjustment that moves
/// its finish ahead of the second model's, so the
/// adjusted record runs winner, winner, the series
/// stands decided at the second race, and its winner's
/// wins stand in one run of two.
fn handicap_series(adjusted: bool) -> JsonValue {
    let mut second_first = attempt(ALPHA, "green", 130.0, Some(10));
    if adjusted {
        second_first = set(&second_first, "adjustment", i(-30));
    }
    series_json(
        "the handicap saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", ALPHA, BETA),
            race_json(
                "race two",
                vec![second_first, attempt_two(BETA, "green", 110.0, Some(20))],
            ),
            clear_win("race three", ALPHA, BETA),
        ],
    )
}

#[test]
fn a_handicap_adjustment_moves_a_series_between_single_and_strung() {
    // S-14. The twin stands single on its scratch record
    // and strung with figure 2 on its adjusted record.
    // Fails under the mutation reading the scratch
    // winners in place of the adjusted winners the walk
    // records, the standing discipline every reading of
    // this repo states.
    let dir = temp_dir("streaks-handicap");
    let plain = streak_for(
        &dir,
        vec![("one.json", handicap_series(false))],
        vec!["one.json"],
    );
    assert_eq!(
        plain,
        vec![
            "streak",
            "streak, 1 series, 0 strung, 1 single, 0 no winner",
        ]
    );
    let adjusted = streak_for(
        &dir,
        vec![("one.json", handicap_series(true))],
        vec!["one.json"],
    );
    assert_eq!(
        adjusted,
        vec![
            "streak",
            "series 1, the handicap saga",
            "alpha-1 against beta-1, first to 2 wins",
            "race 1, race one, alpha-1 wins",
            "race 2, race two, alpha-1 wins",
            "longest run of 2 consecutive decided race wins",
            "series winner, alpha-1, alpha-1 2 race wins, beta-1 0 race wins, 0 no results",
            "streak, 1 series, 1 strung, 0 single, 0 no winner",
        ]
    );
}

#[test]
fn a_series_holding_no_winner_stands_in_neither_class() {
    // S-15. The decided winners run winner, loser,
    // winner, loser at target three, the target unreached
    // and the series holding no winner. No section stands
    // for it and the closing counts it under no winner
    // alone. Fails under the champion drop mutation, a
    // reading that classes an undecided series by the
    // longest run of a side in place of the champion the
    // walk names, under which the series leaves the no
    // winner count and stands classed. This pin carries
    // the no winner form of DH-21 and DH-32.
    let dir = temp_dir("streaks-open");
    let series = series_json(
        "the open saga",
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", BETA, ALPHA),
        ],
    );
    let lines = streak_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "streak",
            "streak, 1 series, 0 strung, 0 single, 1 no winner",
        ]
    );
}

#[test]
fn a_series_decided_early_keeps_its_class_against_its_later_races() {
    // S-16. The plan's small fixture M-A discriminator.
    // The series is decided at a target of two by the
    // decided winners winner, loser, winner at the third
    // race, with a fourth recorded race won by the winner
    // beyond the decision, and stands single, because
    // the walk counts nothing beyond the decision. Fails
    // under M-A, under which the winner's closing races
    // string to a run of 2 over the full record and the
    // series stands strung.
    let dir = temp_dir("streaks-beyond");
    let series = series_json(
        "the early saga",
        vec![ALPHA, BETA],
        2,
        vec![
            clear_win("race one", ALPHA, BETA),
            clear_win("race two", BETA, ALPHA),
            clear_win("race three", ALPHA, BETA),
            clear_win("race four", ALPHA, BETA),
        ],
    );
    let lines = streak_for(&dir, vec![("one.json", series)], vec!["one.json"]);
    assert_eq!(
        lines,
        vec![
            "streak",
            "streak, 1 series, 0 strung, 1 single, 0 no winner",
        ]
    );
}
