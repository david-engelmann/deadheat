//! Pins for DH-68 and DH-69 in the Round 35
//! section of CONTRACT.md. The discrimination
//! record of this file stands pin by pin, and
//! no blanket claim stands over it. The last
//! reign reading, its library function, and
//! its command do not stand on the pre round
//! tree, so every pin in this file that names
//! the last reign reading is absent there at
//! compile time. That absence is recorded
//! once here and is not a discrimination
//! claim. The discrimination each pin carries
//! is against the stated mutation of DH-68
//! beside it. Eight mutations are named for
//! the whole file. CC-1, the last reign read
//! as the longest reign of the winner under
//! DH-52 instead of its last reign. CC-2,
//! the last reign read as the front count of
//! the winner under DH-52. CC-3, the last
//! reign read as the first reign of the
//! winner under DH-66. CC-4, the held class
//! standing at a last reign of three or
//! more, a last reign of two classed single,
//! the rest of the reading unchanged. CC-5,
//! the held and single classes exchanged, a
//! last reign of two or more classed single
//! and a last reign of one classed held.
//! CC-6, the last reign read from the
//! loser's seat, the last run of behind
//! standings of the winner standing in the
//! last reign's place. CC-7, the report
//! carrying its sections for the single
//! series instead of the series standing
//! held, the closing standing as before.
//! CC-8, the last reign counted with the
//! level standings standing as ahead
//! standings, a reign running through a
//! level standing unbroken.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{load_championship_file, Championship};
use deadheat::json::JsonValue;
use deadheat::lastreign::{lastreign_class, lastreign_report, LastreignClass};
use deadheat::races::decide_winner;
use deadheat::series::Series;
use num_bigint::BigInt;
use shared::*;

const ALPHA: &str = "alpha-1";
const BETA: &str = "beta-1";

const ENROLLED: [&str; 26] = [
    "championship.json",
    "constructed-closing.json",
    "constructed-cluster.json",
    "constructed-comeback.json",
    "constructed-crossings.json",
    "constructed-deadlock.json",
    "constructed-depths.json",
    "constructed-distance.json",
    "constructed-firstreign.json",
    "constructed-firststand.json",
    "constructed-firststrike.json",
    "constructed-lastreign.json",
    "constructed-laststand.json",
    "constructed-lead.json",
    "constructed-opening.json",
    "constructed-peak.json",
    "constructed-place.json",
    "constructed-rally.json",
    "constructed-recovery.json",
    "constructed-reigns.json",
    "constructed-spells.json",
    "constructed-streak.json",
    "constructed-tenure.json",
    "constructed-turns.json",
    "league-championship.json",
    "survey-championship.json",
];

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

fn write_json(dir: &Path, name: &str, value: &JsonValue) -> PathBuf {
    std::fs::create_dir_all(dir).expect("create fixture directory");
    let path = dir.join(name);
    std::fs::write(&path, deadheat::json::dumps(value, None, true)).expect("write fixture file");
    path
}

fn championship_json(series_files: Vec<&str>) -> JsonValue {
    obj(vec![(
        "series",
        arr(series_files.iter().map(|name| s(name)).collect()),
    )])
}

fn load(dir: &Path) -> Championship {
    let (championship, defects) = load_championship_file(&dir.join("championship.json"));
    assert_eq!(defects, Vec::<String>::new());
    championship.expect("load fixture championship")
}

fn load_example(name: &str) -> Championship {
    let (championship, defects) = load_championship_file(&examples().join(name));
    assert_eq!(defects, Vec::<String>::new());
    championship.expect("load example championship")
}

fn golden(name: &str) -> String {
    std::fs::read_to_string(examples().join(name)).expect("read golden file")
}

fn decided_record(series: &Series) -> Vec<usize> {
    let mut wins = [0u64; 2];
    let mut champion: Option<usize> = None;
    let mut decided: Vec<usize> = Vec::new();
    for race in &series.races {
        if champion.is_some() {
            break;
        }
        let (winner, _) = decide_winner(race);
        if let Some(seat) = winner {
            let side = if race.attempts[seat].model == series.models[0] {
                0
            } else {
                1
            };
            decided.push(side);
            wins[side] += 1;
            if BigInt::from(wins[side]) >= series.wins_target {
                champion = Some(side);
            }
        }
    }
    decided
}

fn winner_of(series: &Series) -> Option<usize> {
    let decided = decided_record(series);
    let target = series
        .wins_target
        .to_string()
        .parse::<usize>()
        .expect("target parses");
    let mut wins = [0usize; 2];
    for side in &decided {
        wins[*side] += 1;
    }
    if wins[0] >= target {
        Some(0)
    } else if wins[1] >= target {
        Some(1)
    } else {
        None
    }
}

fn enrolled() -> Vec<Championship> {
    ENROLLED.iter().map(|name| load_example(name)).collect()
}

fn ahead_flags(decided: &[usize], winner: usize) -> Vec<bool> {
    let mut tallies = [0usize; 2];
    let mut flags = Vec::new();
    for side in decided {
        tallies[*side] += 1;
        flags.push(tallies[winner] > tallies[1 - winner]);
    }
    flags
}

fn behind_flags(decided: &[usize], winner: usize) -> Vec<bool> {
    let mut tallies = [0usize; 2];
    let mut flags = Vec::new();
    for side in decided {
        tallies[*side] += 1;
        flags.push(tallies[winner] < tallies[1 - winner]);
    }
    flags
}

fn longest_run(flags: &[bool]) -> u64 {
    let mut longest = 0u64;
    let mut open = 0u64;
    for flag in flags {
        if *flag {
            open += 1;
            if open > longest {
                longest = open;
            }
        } else {
            open = 0;
        }
    }
    longest
}

fn first_run(flags: &[bool]) -> u64 {
    let mut run = 0u64;
    let mut started = false;
    for flag in flags {
        if *flag {
            started = true;
            run += 1;
        } else if started {
            break;
        }
    }
    run
}

fn longest_reign_of(series: &Series) -> u64 {
    let winner = winner_of(series).expect("a decided series");
    longest_run(&ahead_flags(&decided_record(series), winner))
}

fn first_reign_of(series: &Series) -> u64 {
    let winner = winner_of(series).expect("a decided series");
    first_run(&ahead_flags(&decided_record(series), winner))
}

fn front_count_of(series: &Series) -> u64 {
    let winner = winner_of(series).expect("a decided series");
    ahead_flags(&decided_record(series), winner)
        .iter()
        .filter(|flag| **flag)
        .count() as u64
}

fn margin_of(series: &Series) -> u64 {
    let winner = winner_of(series).expect("a decided series");
    let decided = decided_record(series);
    let wins = decided.iter().filter(|side| **side == winner).count() as u64;
    let losses = decided.len() as u64 - wins;
    wins - losses
}

#[test]
fn the_classification_reads_the_last_run_of_ahead_standings() {
    // LR-1. The lastreign characterization pin.
    // A hand fixture at a target of six, its
    // blocks standing winner two, loser two,
    // winner one, loser one, and winner three,
    // its decided record closing at the sixth
    // win of its winner on the last race of the
    // record, the winner championing with its
    // last reign at three, its longest reign at
    // three, its first reign at three, its
    // front count at seven, and its first spell
    // at none, the winner standing ahead or
    // level at every standing of the record.
    // It asserts the reign at three and the
    // held class over it at the module's entry,
    // the neighbour figures beside it, the
    // section in full over the fixture, and the
    // closing line of its one series
    // championship. Fails under CC-2, the front
    // count of seven standing in the reign's
    // place. Fails under CC-5, the exchanged
    // classes classing a reign of three single
    // and the section falling away. Fails under
    // CC-6, the last run of behind standings
    // standing at none on a record whose winner
    // never stood behind. Fails under CC-7, the
    // sections standing for the single series
    // and this series losing its section. Fails
    // under CC-8, the level standings at the
    // fourth and sixth races standing as ahead
    // standings and the reign running through
    // them to the whole record. Passes under
    // CC-1, the winner's longest reign standing
    // at three as its last reign does. Passes
    // under CC-3, the winner's first reign
    // standing at three as its last reign does.
    // Passes under CC-4, a reign of three
    // standing held at the raised cut as
    // before.
    let dir = temp_dir("lastreign-lr1");
    let task = "the hand classification series";
    let series = series_json(
        task,
        vec!["birch-1", "ash-1"],
        6,
        vec![
            clear_win(task, "birch-1", "ash-1"),
            clear_win(task, "birch-1", "ash-1"),
            clear_win(task, "ash-1", "birch-1"),
            clear_win(task, "ash-1", "birch-1"),
            clear_win(task, "birch-1", "ash-1"),
            clear_win(task, "ash-1", "birch-1"),
            clear_win(task, "birch-1", "ash-1"),
            clear_win(task, "birch-1", "ash-1"),
            clear_win(task, "birch-1", "ash-1"),
        ],
    );
    write_json(&dir, "series.json", &series);
    write_json(
        &dir,
        "championship.json",
        &championship_json(vec!["series.json"]),
    );
    let championship = load(&dir);
    let series = &championship.series[0];
    assert_eq!(lastreign_class(series), Some((LastreignClass::Held, 3)));
    assert_eq!(longest_reign_of(series), 3);
    assert_eq!(first_reign_of(series), 3);
    assert_eq!(front_count_of(series), 7);
    let behind = behind_flags(
        &decided_record(series),
        winner_of(series).expect("a winner"),
    );
    assert!(behind.iter().all(|flag| !flag));
    let expected: Vec<String> = vec![
        "lastreign",
        "series 1, the hand classification series",
        "birch-1 against ash-1, first to 6 wins",
        "race 1, the hand classification series, birch-1 wins",
        "race 2, the hand classification series, birch-1 wins",
        "race 3, the hand classification series, ash-1 wins",
        "race 4, the hand classification series, ash-1 wins",
        "race 5, the hand classification series, birch-1 wins",
        "race 6, the hand classification series, ash-1 wins",
        "race 7, the hand classification series, birch-1 wins",
        "race 8, the hand classification series, birch-1 wins",
        "race 9, the hand classification series, birch-1 wins",
        "last reign 3 over the decided record",
        "series winner, birch-1, birch-1 6 race wins, ash-1 3 race wins, 0 no results",
        "lastreign, 1 series, 1 held, 0 single, 0 no winner",
    ]
    .iter()
    .map(|line| line.to_string())
    .collect();
    assert_eq!(lastreign_report(&championship), expected);
}

#[test]
fn the_survey_championship_closes_at_four_held_and_two_single() {
    // LR-2. The survey championship's closing
    // line, asserting four held and two single
    // over its six series. Fails under CC-1,
    // the longest reigns moving the closing.
    // Fails under CC-2, the front counts moving
    // the closing. Fails under CC-3, the first
    // reigns moving the closing. Fails under
    // CC-4, the survey series standing at a
    // last reign of two classing single at the
    // raised cut. Fails under CC-5, the classes
    // standing exchanged over the field. Fails
    // under CC-6, the losers' seats moving the
    // closing. Fails under CC-8, the reigns
    // running through the level standings of
    // the survey fields. Passes under CC-7, the
    // closing standing as before while its
    // sections move.
    let championship = load_example("survey-championship.json");
    let report = lastreign_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("lastreign, 6 series, 4 held, 2 single, 0 no winner")
    );
    assert_eq!(
        closing_counts(report.last().expect("a closing")),
        vec![6, 4, 2, 0]
    );
}

#[test]
fn the_league_championship_closes_at_six_held_and_four_single() {
    // LR-3. The league championship's closing
    // line, asserting six held and four single
    // over its ten series. Fails under CC-1,
    // CC-2, CC-3, CC-4, CC-5, CC-6, and CC-8,
    // the league fields moving under each.
    // Passes under CC-7, the closing standing
    // as before while its sections move.
    let championship = load_example("league-championship.json");
    let report = lastreign_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("lastreign, 10 series, 6 held, 4 single, 0 no winner")
    );
    assert_eq!(
        closing_counts(report.last().expect("a closing")),
        vec![10, 6, 4, 0]
    );
}

#[test]
fn the_championship_example_closes_at_two_held_with_two_sections() {
    // LR-4. The championship example's closing
    // line, asserting two held and none single
    // over its two series, with the sections of
    // both series standing in championship
    // order. Fails under CC-5, the classes
    // standing exchanged and both sections
    // falling away. Fails under CC-6, the
    // losers' seats moving both series. Fails
    // under CC-7, the sections standing for the
    // single series and this championship
    // carrying none. Passes under CC-1, both
    // last reigns standing at their longest
    // reigns. Passes under CC-2, both series
    // standing with their last reigns at their
    // front counts. Passes under CC-3, both
    // last reigns standing at their first
    // reigns. Passes under CC-4, both reigns
    // standing at three or more and held at
    // the raised cut. Passes under CC-8,
    // neither record's last reign running
    // through a level standing.
    let championship = load_example("championship.json");
    let report = lastreign_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("lastreign, 2 series, 2 held, 0 single, 0 no winner")
    );
    let sections: Vec<&String> = report
        .iter()
        .filter(|line| line.starts_with("series ") && !line.starts_with("series winner"))
        .collect();
    assert_eq!(sections.len(), 2);
    assert!(sections[0].starts_with("series 1,"));
    assert!(sections[1].starts_with("series 2,"));
}

#[test]
fn the_constructed_recovery_championship_closes_at_none_held_and_two_single() {
    // LR-5. The constructed recovery
    // championship's closing line, asserting
    // none held and two single over its two
    // series. Fails under CC-5, the exchange
    // classing both series held and drawing
    // their sections. Fails under CC-6, the
    // losers' seats moving the closing. Fails
    // under CC-8, the reigns running through
    // the level standings of the pair. Passes
    // under CC-1, both last reigns standing at
    // their longest reigns of one. Passes under
    // CC-2, both front counts standing at one.
    // Passes under CC-3, both first reigns
    // standing at one. Passes under CC-4, both
    // reigns standing at one and single at the
    // raised cut as before. Passes under CC-7,
    // the closing standing as before.
    let championship = load_example("constructed-recovery.json");
    let report = lastreign_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("lastreign, 2 series, 0 held, 2 single, 0 no winner")
    );
    assert_eq!(
        closing_counts(report.last().expect("a closing")),
        vec![2, 0, 2, 0]
    );
}

#[test]
fn the_constructed_lastreign_championship_closes_at_one_held_and_one_single() {
    // LR-6. The constructed lastreign
    // championship's closing line, asserting
    // one held and one single over its two
    // series, the pair standing built for this
    // reading. Fails under CC-1, the second
    // drill's longest reign of three classing
    // it held. Fails under CC-2, the front
    // counts of the drills classing both held.
    // Fails under CC-3, the first reigns
    // standing at one over both drills and the
    // closing standing at none held. Fails
    // under CC-6, the losers' seats moving the
    // closing. Fails under CC-8, the reigns
    // running through the level standings of
    // the drills. Passes under CC-4, the first
    // drill's reign of three standing held at
    // the raised cut and the second drill's
    // standing single. Passes under CC-5, the
    // exchange standing this pair at one and
    // one still. Passes under CC-7, the closing
    // standing as before.
    let championship = load_example("constructed-lastreign.json");
    let report = lastreign_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("lastreign, 2 series, 1 held, 1 single, 0 no winner")
    );
    assert_eq!(
        closing_counts(report.last().expect("a closing")),
        vec![2, 1, 1, 0]
    );
}

#[test]
fn the_first_drill_classes_held_at_a_last_reign_of_three() {
    // LR-7. The first drill's entry assertion,
    // the series standing held at a last reign
    // of three, its first reign at one, its
    // longest reign at three, and its front
    // count at five, the drill standing single
    // under DH-66 in the pair's own record.
    // Fails under CC-2, the front count of five
    // standing in the reign's place. Fails
    // under CC-3, the first reign of one
    // classing it single. Fails under CC-5, the
    // exchange classing the reign of three
    // single. Fails under CC-6, the winner
    // never standing behind over the record and
    // the loser's seat returning no reign.
    // Fails under CC-8, the reign running
    // through the level standings at the second
    // and fourth decided races and standing at
    // the whole span of its closing run.
    // Passes under CC-1, the longest reign
    // standing at three. Passes under CC-4, the
    // reign of three standing held at the
    // raised cut. Passes under CC-7, the entry
    // standing outside the report's sections.
    let championship = load_example("constructed-lastreign.json");
    assert_eq!(
        lastreign_class(&championship.series[0]),
        Some((LastreignClass::Held, 3))
    );
}

#[test]
fn the_second_drill_classes_single_at_a_last_reign_of_one() {
    // LR-8. The second drill's entry
    // assertion, the series standing single at
    // a last reign of one, its longest reign at
    // three, its front count at five, and its
    // first reign at one, the drill's reigns
    // standing at one, three, and one over its
    // record. Fails under CC-1, the longest
    // reign of three classing it held. Fails
    // under CC-2, the front count of five
    // classing it held. Fails under CC-5, the
    // exchange classing the reign of one held.
    // Fails under CC-6, the winner never
    // standing behind over the record and the
    // loser's seat returning no reign. Fails
    // under CC-8, the reign running through
    // the level standings of the record to the
    // whole of its closing span. Passes under
    // CC-3, the first reign standing at one.
    // Passes under CC-4, the reign of one
    // standing single at the raised cut as
    // before. Passes under CC-7, the entry
    // standing outside the report's sections.
    let championship = load_example("constructed-lastreign.json");
    assert_eq!(
        lastreign_class(&championship.series[1]),
        Some((LastreignClass::Single, 1))
    );
}

#[test]
fn the_panel_series_holds_at_a_last_reign_of_three_with_no_first_spell() {
    // LR-9. The panel series of the
    // championship example, its last reign
    // standing at three held and its first
    // spell standing at none, the winner
    // holding every decided race of the record
    // and never standing behind. Fails under
    // CC-5, the exchange classing the reign of
    // three single. Fails under CC-6, the
    // winner never standing behind and the
    // loser's seat returning no reign. Passes
    // under CC-1, the longest reign standing at
    // three. Passes under CC-2, the front count
    // standing at three. Passes under CC-3, the
    // first reign standing at three. Passes
    // under CC-4, the reign of three standing
    // held at the raised cut. Passes under
    // CC-7, the entry standing outside the
    // report's sections. Passes under CC-8, the
    // record carrying no level standing for
    // the count to run through.
    let championship = load_example("championship.json");
    let series = &championship.series[1];
    assert_eq!(lastreign_class(series), Some((LastreignClass::Held, 3)));
    let winner = winner_of(series).expect("a decided series");
    let behind = behind_flags(&decided_record(series), winner);
    assert!(behind.iter().all(|flag| !flag));
}

#[test]
fn the_last_reign_never_stands_above_the_winners_longest_reign() {
    // LR-10. The theorem census of DH-68 over
    // the enrolled sixty one series, the last
    // reign standing above the winner's
    // longest reign at no series, every series
    // of the enrolment holding a winner. Fails
    // under CC-2, the front counts standing
    // above the longest reigns at the broken
    // series of the enrolment. Fails under
    // CC-6, the losers' seats standing above
    // the longest reigns at series the
    // enrolment carries. Fails under CC-8, the
    // reigns running through level standings
    // standing above the longest reigns
    // wherever a level standing stands inside
    // a closing span. Passes under CC-1, the
    // figure standing at the longest reign
    // itself and never above it. Passes under
    // CC-3, the first reign standing at or
    // below the longest by its form. Passes
    // under CC-4, CC-5, and CC-7, the census
    // reading the reign's count alone at the
    // entry, where the classes and sections
    // stand outside the count.
    let mut decided = 0;
    let mut violations = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, reign) = match lastreign_class(series) {
                Some(pair) => pair,
                None => continue,
            };
            decided += 1;
            if reign > longest_reign_of(series) {
                violations += 1;
            }
        }
    }
    assert_eq!(decided, 61);
    assert_eq!(violations, 0);
}

#[test]
fn the_last_reign_stands_at_the_whole_of_the_front_count_at_thirty_five_series() {
    // LR-11. The front count census of DH-68
    // over the enrolled sixty one series, the
    // last reign standing at the whole of its
    // winner's front count at exactly thirty
    // five series, the series standing
    // unbroken under DH-52, whose ahead
    // standings stand in one run entire, that
    // run being its last. Fails under CC-2, the
    // figure standing at the front count
    // itself at every series and the count
    // rising to the whole enrolment. Fails
    // under CC-6, the losers' seats standing at
    // the front counts at other series and
    // falling short at these. Fails under
    // CC-8, the reigns running through level
    // standings standing at the front counts
    // at fewer series. Passes under CC-1, the
    // longest reign standing at the whole of
    // the front count exactly at the unbroken
    // series. Passes under CC-3, the first
    // reign standing at the whole of the front
    // count at those same series. Passes under
    // CC-4, CC-5, and CC-7, the census reading
    // the reign's count alone.
    let mut whole = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, reign) = match lastreign_class(series) {
                Some(pair) => pair,
                None => continue,
            };
            if reign == front_count_of(series) {
                whole += 1;
            }
        }
    }
    assert_eq!(whole, 35);
}

#[test]
fn a_series_at_a_wins_target_of_one_stands_single() {
    // LR-12. The wins target one fixture, a
    // series of one decided race at a target
    // of one, its winner winning the one race
    // the record holds before the decision and
    // its last reign standing at one single.
    // Fails under CC-5, the exchange classing
    // the reign of one held. Fails under CC-6,
    // the winner never standing behind over the
    // one race and the loser's seat returning
    // no reign. Passes under CC-1, the winner's
    // longest reign standing at one. Passes
    // under CC-2, the winner's front count
    // standing at one. Passes under CC-3, the
    // winner's first reign standing at one.
    // Passes under CC-4, the reign of one
    // standing single at the raised cut as
    // before. Passes under CC-7, the pin
    // reading the entry alone and the report's
    // sections standing outside it. Passes
    // under CC-8, the one standing of the
    // record standing ahead and no level
    // standing standing in it.
    let dir = temp_dir("lastreign-lr12");
    let task = "the hand target one series";
    let series = series_json(
        task,
        vec![ALPHA, BETA],
        1,
        vec![clear_win(task, ALPHA, BETA)],
    );
    write_json(&dir, "series.json", &series);
    write_json(
        &dir,
        "championship.json",
        &championship_json(vec!["series.json"]),
    );
    let championship = load(&dir);
    assert_eq!(
        lastreign_class(&championship.series[0]),
        Some((LastreignClass::Single, 1))
    );
}

#[test]
fn a_series_holding_no_winner_takes_no_class_and_no_section() {
    // LR-13. The roster's characterization pin
    // of no posture. A series holding no
    // winner, its three races standing at two
    // wins against one at a target of three,
    // taking no class at the module's entry
    // and no section in the report, the report
    // over its championship standing in the
    // empty form with the series counted in
    // the no winner place alone. The pin falls
    // under no mutation of the eight, each
    // mutation reading a decided record the
    // series does not hold, and it stands in
    // the roster so the no winner place of
    // DH-68 and DH-69 stands pinned in words.
    let dir = temp_dir("lastreign-lr13");
    let task = "the hand undecided series";
    let series = series_json(
        task,
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win(task, ALPHA, BETA),
            clear_win(task, BETA, ALPHA),
            clear_win(task, ALPHA, BETA),
        ],
    );
    write_json(&dir, "series.json", &series);
    write_json(
        &dir,
        "championship.json",
        &championship_json(vec!["series.json"]),
    );
    let championship = load(&dir);
    assert_eq!(lastreign_class(&championship.series[0]), None);
    assert_eq!(
        lastreign_report(&championship),
        vec![
            "lastreign".to_string(),
            "lastreign, 1 series, 0 held, 0 single, 1 no winner".to_string(),
        ]
    );
}

#[test]
fn a_championship_holding_no_series_stands_refused_at_the_load() {
    // LR-14. The empty report form pin. A
    // championship holding no series at all
    // stands refused at the load, the empty
    // series list drawing the one defect field
    // series holds no series files, so no
    // championship stands for a report to
    // stand over. The pin falls under no
    // mutation of the eight, no series
    // standing in the championship for any
    // mutation to read, and it stands in the
    // roster so the load boundary of DH-69
    // stands pinned in words.
    let dir = temp_dir("lastreign-lr14");
    write_json(&dir, "championship.json", &championship_json(vec![]));
    let (championship, defects) = load_championship_file(&dir.join("championship.json"));
    assert_eq!(championship, None);
    assert_eq!(
        defects,
        vec!["field series holds no series files".to_string()]
    );
}

#[test]
fn the_first_drills_section_stands_in_the_family_form() {
    // LR-15. The lastreign pair section pin,
    // the roster's strongest pin. The section
    // form over the lastreign championship,
    // drill one's section standing in full with
    // its series line, its models and target
    // line, its race lines over the decided
    // races at the record places 1, 2, 3, 4, 5,
    // 6, and 8, its figure line at a last reign
    // of three, and its series winner line,
    // drill two standing in no section, the
    // whole report standing line for line as
    // the golden stands. Fails under every
    // variant of the eight but CC-4. Under
    // CC-1, drill two's longest reign of three
    // classing it held and drawing its section.
    // Under CC-2, drill two's front count of
    // five drawing its section on the same
    // move. Under CC-3, drill one's first reign
    // of one losing its section while drill
    // two gains none. Under CC-5, the sections
    // moving between the drills under the
    // exchange. Under CC-6, drill one's reign
    // reading none from the loser's seat and
    // its section falling away. Under CC-7, the
    // sections standing for the single series,
    // drill two drawing the section and drill
    // one losing it. Under CC-8, drill one's
    // figure standing at seven with the level
    // standings of its record counted in the
    // reign, and drill two gaining a section
    // at the same move. Passes under CC-4,
    // drill one's reign of three standing held
    // at the raised cut and drill two's
    // standing single, the report standing
    // word for word.
    let championship = load_example("constructed-lastreign.json");
    let expected: Vec<String> = [
        "lastreign",
        "series 1, the constructed lastreign drill one",
        "lastreign-ash against lastreign-birch, first to 5 wins",
        "race 1, lastreign drill one race one, lastreign-birch wins",
        "race 2, lastreign drill one race two, lastreign-ash wins",
        "race 3, lastreign drill one race three, lastreign-birch wins",
        "race 4, lastreign drill one race four, lastreign-ash wins",
        "race 5, lastreign drill one race five, lastreign-birch wins",
        "race 6, lastreign drill one race six, lastreign-birch wins",
        "race 8, lastreign drill one race eight, lastreign-birch wins",
        "last reign 3 over the decided record",
        "series winner, lastreign-birch, lastreign-ash 2 race wins, lastreign-birch 5 race wins, 1 no results",
        "lastreign, 2 series, 1 held, 1 single, 0 no winner",
    ]
    .iter()
    .map(|line| line.to_string())
    .collect();
    assert_eq!(lastreign_report(&championship), expected);
    assert_eq!(
        lastreign_report(&championship).join("\n") + "\n",
        golden("constructed-lastreign-lastreign.txt")
    );
}

#[test]
fn the_last_reign_stands_above_the_final_margin_at_twelve_series() {
    // LR-16. The margin census of DH-68 over
    // the enrolled sixty one series, the last
    // reign standing above the count of its
    // winner's wins less the count of its
    // loser's wins at exactly twelve series,
    // the first series of the championship
    // example standing among them at a last
    // reign of four against a count of two,
    // its last reign spanning a win of its
    // loser. Fails under CC-1, the longest
    // reigns standing above the margin at
    // their own seats. Fails under CC-2, the
    // front counts standing above the margin
    // at the broken series. Fails under CC-3,
    // the first reigns standing above the
    // margin at fewer seats. Fails under CC-6,
    // the losers' seats standing above the
    // margin at their own seats. Fails under
    // CC-8, the reigns running through level
    // standings standing above the margin at
    // more seats. Passes under CC-4, CC-5, and
    // CC-7, the census reading the reign's
    // count alone at the entry.
    let mut above = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, reign) = match lastreign_class(series) {
                Some(pair) => pair,
                None => continue,
            };
            if reign > margin_of(series) {
                above += 1;
            }
        }
    }
    assert_eq!(above, 12);
}

#[test]
fn the_enrolled_championships_close_at_twenty_eight_held_and_thirty_three_single() {
    // LR-17. The aggregate closing over the
    // enrolled sixty one series of the twenty
    // six championships, asserting twenty
    // eight held, thirty three single, and
    // none holding no winner, summed over the
    // closing line of each championship's
    // report. Fails under CC-1, CC-2, CC-3,
    // CC-4, CC-5, and CC-6, the fields of the
    // enrolment moving under each, and under
    // CC-8, every reign running through the
    // level standings of the enrolment. Passes
    // under CC-7, the closings standing as
    // before while their sections move.
    let mut totals = vec![0u64; 4];
    for championship in enrolled() {
        let report = lastreign_report(&championship);
        let counts = closing_counts(report.last().expect("a closing"));
        for (index, count) in counts.iter().enumerate() {
            totals[index] += count;
        }
    }
    assert_eq!(totals, vec![61, 28, 33, 0]);
}

#[test]
fn the_first_drills_section_spans_the_record_places_of_its_reign() {
    // LR-18. The span pin on the first drill.
    // Its section's race lines stand numbered
    // by the counting walk's place among the
    // decided races at 1, 2, 3, 4, 5, 6, and 8,
    // the race holding no winner at the seventh
    // place of the record standing in no race
    // line, and its figure line stands at a
    // last reign of three, the skipped place
    // standing inside the span of the reign
    // itself, the span witness of the roster.
    // Fails under CC-2, the front count of five
    // standing in the figure's place. Fails
    // under CC-3, the first drill classing
    // single at its first reign of one and its
    // section falling away. Fails under CC-5,
    // the first drill classing single under
    // the exchange and its section falling
    // away. Fails under CC-6, the first drill's
    // reign reading none from the loser's seat
    // and its section falling away. Fails under
    // CC-7, the sections standing for the
    // single series and the first drill losing
    // its section to the second. Fails under
    // CC-8, the figure standing at seven with
    // the level standings of the record
    // counted in the reign. Passes under CC-1,
    // the first drill's longest reign standing
    // at three and its section standing as
    // before. Passes under CC-4, the reign of
    // three standing held at the raised cut
    // and the section standing.
    let championship = load_example("constructed-lastreign.json");
    let report = lastreign_report(&championship);
    let start = report
        .iter()
        .position(|line| line.starts_with("series 1,"))
        .expect("the first drill's section");
    let places: Vec<u64> = report[start..]
        .iter()
        .skip_while(|line| !line.starts_with("race "))
        .take_while(|line| line.starts_with("race "))
        .map(|line| {
            line.trim_start_matches("race ")
                .split(',')
                .next()
                .expect("a place")
                .parse::<u64>()
                .expect("a place number")
        })
        .collect();
    assert_eq!(places, vec![1, 2, 3, 4, 5, 6, 8]);
    assert!(report.contains(&"last reign 3 over the decided record".to_string()));
}

#[test]
fn the_last_reign_stands_below_the_longest_reign_at_five_series() {
    // LR-19. The parting census of DH-68 over
    // the enrolled sixty one series, the last
    // reign standing below the winner's
    // longest reign at exactly five series,
    // the first series of the constructed
    // crossings championship standing among
    // them at one against three and the
    // constructed pair's second drill adding
    // the fifth to the four of the head's
    // enrolment. Fails under CC-1, the figure
    // standing at the longest reign itself at
    // every series and the count falling to
    // none. Fails under CC-2, the front counts
    // standing at or above the longest reigns
    // at every series and the count falling to
    // none. Fails under CC-3, the first reigns
    // standing below the longest at a count of
    // their own. Fails under CC-6, the losers'
    // seats standing below the longest reigns
    // at a count of their own. Fails under
    // CC-8, the reigns running through level
    // standings standing below the longest at
    // fewer series. Passes under CC-4, CC-5,
    // and CC-7, the census reading the reign's
    // count alone at the entry.
    let mut below = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, reign) = match lastreign_class(series) {
                Some(pair) => pair,
                None => continue,
            };
            if reign < longest_reign_of(series) {
                below += 1;
            }
        }
    }
    assert_eq!(below, 5);
}

#[test]
fn the_last_reign_stands_at_the_first_reign_at_forty_nine_series() {
    // LR-20. The first reign census of DH-68
    // over the enrolled sixty one series, the
    // last reign standing at the winner's
    // first reign at exactly forty nine
    // series, the count standing at forty
    // eight over the head's enrolment with the
    // last reign standing above the first at
    // eight series and below it at three, the
    // constructed pair's first drill standing
    // above it at three against one and its
    // second drill standing at it at one
    // against one. Fails under CC-1, the
    // longest reigns
    // standing at the first reigns at a count
    // of their own. Fails under CC-2, the front
    // counts standing at the first reigns at
    // fewer series. Fails under CC-3, the
    // figure standing at the first reign
    // itself at every series and the count
    // rising to the whole enrolment. Fails
    // under CC-6, the losers' seats standing
    // at the first reigns at their own seats.
    // Fails under CC-8, the reigns running
    // through level standings standing at the
    // first reigns at fewer series. Passes
    // under CC-4, CC-5, and CC-7, the census
    // reading the reign's count alone at the
    // entry.
    let mut at = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, reign) = match lastreign_class(series) {
                Some(pair) => pair,
                None => continue,
            };
            if reign == first_reign_of(series) {
                at += 1;
            }
        }
    }
    assert_eq!(at, 49);
}
