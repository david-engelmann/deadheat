//! Pins for DH-66 and DH-67 in the Round 34
//! section of CONTRACT.md. The discrimination
//! record of this file stands pin by pin, and
//! no blanket claim stands over it. The first
//! reign reading, its library function, and
//! its command do not stand on the pre round
//! tree, so every pin in this file that names
//! the first reign reading is absent there at
//! compile time. That absence is recorded
//! once here and is not a discrimination
//! claim. The discrimination each pin carries
//! is against the stated mutation of DH-66
//! beside it. Eight mutations are named for
//! the whole file. BB-1, the first reign read
//! as the longest reign of the winner under
//! DH-52 instead of its first reign. BB-2,
//! the first reign read as the front count of
//! the winner under DH-52. BB-3, the first
//! reign read as the closing reign of the
//! winner, the last run of ahead standings of
//! the record. BB-4, the held class standing
//! at a first reign of three or more, a first
//! reign of two classed single, the rest of
//! the reading unchanged. BB-5, the held and
//! single classes exchanged, a first reign of
//! two or more classed single and a first
//! reign of one classed held. BB-6, the first
//! reign read from the loser's seat, the
//! first run of behind standings of the
//! winner standing in the first reign's
//! place. BB-7, the report carrying its
//! sections for the single series instead of
//! the series standing held, the closing
//! standing as before. BB-8, the first reign
//! counted with the level standings standing
//! as ahead standings, a reign running
//! through a level standing unbroken.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{load_championship_file, Championship};
use deadheat::firstreign::{firstreign_class, firstreign_report, FirstreignClass};
use deadheat::json::JsonValue;
use deadheat::races::decide_winner;
use deadheat::series::Series;
use num_bigint::BigInt;
use shared::*;

const ALPHA: &str = "alpha-1";
const BETA: &str = "beta-1";

const ENROLLED: [&str; 25] = [
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

fn runs_of(decided: &[usize], side: usize) -> Vec<usize> {
    let mut runs: Vec<usize> = Vec::new();
    let mut current = 0usize;
    for entry in decided {
        if *entry == side {
            current += 1;
        } else {
            if current > 0 {
                runs.push(current);
            }
            current = 0;
        }
    }
    if current > 0 {
        runs.push(current);
    }
    runs
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

fn loser_of(series: &Series) -> usize {
    match winner_of(series) {
        Some(0) => 1,
        _ => 0,
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

fn closing_run(flags: &[bool]) -> u64 {
    let mut run = 0u64;
    for flag in flags.iter().rev() {
        if *flag {
            run += 1;
        } else {
            break;
        }
    }
    run
}

fn longest_reign_of(series: &Series) -> u64 {
    let winner = winner_of(series).expect("a decided series");
    longest_run(&ahead_flags(&decided_record(series), winner))
}

fn front_count_of(series: &Series) -> u64 {
    let winner = winner_of(series).expect("a decided series");
    ahead_flags(&decided_record(series), winner)
        .iter()
        .filter(|flag| **flag)
        .count() as u64
}

#[test]
fn the_classification_reads_the_first_run_of_ahead_standings() {
    // FR-1. The firstreign characterization pin.
    // A hand fixture at a target of five, its
    // blocks standing winner two, loser two,
    // winner one, loser one, and winner three,
    // its decided record closing at the fifth
    // win of its winner on the second race of
    // the last block, the winner championing
    // with its first reign at three, its
    // longest reign at three, its front count
    // at six, its closing reign at two, and its
    // first spell at none, the winner standing
    // ahead or level at every standing of the
    // record. It asserts the reign at three and
    // the held class over it at the module's
    // entry, the neighbour figures beside it,
    // the section in full over the fixture, and
    // the closing line of its one series
    // championship. Fails under BB-2, the front
    // count of six standing in the reign's
    // place. Fails under BB-3, the closing reign
    // of two standing in its place. Fails under
    // BB-5, the exchanged classes classing a
    // reign of three single and the section
    // falling away. Fails under BB-6, the first
    // run of behind standings standing at none
    // on a record whose winner never stood
    // behind. Fails under BB-7, the sections
    // standing for the single series and this
    // series losing its section. Fails under
    // BB-8, the level standings at the fourth
    // and sixth races standing as ahead
    // standings and the reign running through
    // them to the whole record. Passes under
    // BB-1, the winner's longest reign standing
    // at three as its first reign does. Passes
    // under BB-4, a reign of three standing
    // held at the raised cut as before.
    let dir = temp_dir("firstreign-fr1");
    let task = "the hand classification series";
    let series = series_json(
        task,
        vec!["birch-1", "ash-1"],
        5,
        vec![
            clear_win(task, "birch-1", "ash-1"),
            clear_win(task, "birch-1", "ash-1"),
            clear_win(task, "ash-1", "birch-1"),
            clear_win(task, "ash-1", "birch-1"),
            clear_win(task, "birch-1", "ash-1"),
            clear_win(task, "ash-1", "birch-1"),
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
    assert_eq!(firstreign_class(series), Some((FirstreignClass::Held, 3)));
    assert_eq!(longest_reign_of(series), 3);
    assert_eq!(front_count_of(series), 6);
    let flags = ahead_flags(
        &decided_record(series),
        winner_of(series).expect("a winner"),
    );
    assert_eq!(closing_run(&flags), 2);
    let behind = behind_flags(
        &decided_record(series),
        winner_of(series).expect("a winner"),
    );
    assert!(behind.iter().all(|flag| !flag));
    let expected: Vec<String> = vec![
        "firstreign",
        "series 1, the hand classification series",
        "birch-1 against ash-1, first to 5 wins",
        "race 1, the hand classification series, birch-1 wins",
        "race 2, the hand classification series, birch-1 wins",
        "race 3, the hand classification series, ash-1 wins",
        "race 4, the hand classification series, ash-1 wins",
        "race 5, the hand classification series, birch-1 wins",
        "race 6, the hand classification series, ash-1 wins",
        "race 7, the hand classification series, birch-1 wins",
        "race 8, the hand classification series, birch-1 wins",
        "first reign 3 over the decided record",
        "series winner, birch-1, birch-1 5 race wins, ash-1 3 race wins, 0 no results",
        "firstreign, 1 series, 1 held, 0 single, 0 no winner",
    ]
    .iter()
    .map(|line| line.to_string())
    .collect();
    assert_eq!(firstreign_report(&championship), expected);
}

#[test]
fn the_survey_championship_closes_at_three_held_and_three_single() {
    // FR-2. The survey championship's closing
    // line, asserting three held and three
    // single over its six series. Fails under
    // BB-1, the longest reigns moving the
    // closing. Fails under BB-2, the front
    // counts moving the closing. Fails under
    // BB-3, the closing reigns moving the
    // closing. Fails under BB-4, the survey
    // series standing at a first reign of two
    // classing single at the raised cut. Fails
    // under BB-6, the losers' seats moving the
    // closing. Fails under BB-8, the reigns
    // running through the level standings of
    // the survey fields. Passes under BB-5, the
    // exchange standing the closing at the same
    // three and three over this field. Passes
    // under BB-7, the closing standing as before
    // while its sections move.
    let championship = load_example("survey-championship.json");
    let report = firstreign_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("firstreign, 6 series, 3 held, 3 single, 0 no winner")
    );
    assert_eq!(
        closing_counts(report.last().expect("a closing")),
        vec![6, 3, 3, 0]
    );
}

#[test]
fn the_league_championship_closes_at_five_held_and_five_single() {
    // FR-3. The league championship's closing
    // line, asserting five held and five single
    // over its ten series. Fails under BB-1,
    // BB-2, BB-3, BB-4, BB-6, and BB-8, the
    // league fields moving under each. Passes
    // under BB-5, the exchange standing the
    // closing at the same five and five over
    // this field. Passes under BB-7, the closing
    // standing as before while its sections
    // move.
    let championship = load_example("league-championship.json");
    let report = firstreign_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("firstreign, 10 series, 5 held, 5 single, 0 no winner")
    );
    assert_eq!(
        closing_counts(report.last().expect("a closing")),
        vec![10, 5, 5, 0]
    );
}

#[test]
fn the_championship_example_closes_at_two_held_with_two_sections() {
    // FR-4. The championship example's closing
    // line, asserting two held and none single
    // over its two series, with the sections of
    // both series standing in championship
    // order. Fails under BB-5, the classes
    // standing exchanged and both sections
    // falling away. Fails under BB-6, the
    // losers' seats moving both series. Fails
    // under BB-7, the sections standing for the
    // single series and this championship
    // carrying none. Passes under BB-1, both
    // first reigns standing at their longest
    // reigns. Passes under BB-2, both series
    // standing unbroken with their first reigns
    // at their front counts. Passes under BB-3,
    // both closing reigns standing at their
    // first reigns on records that never stand
    // level. Passes under BB-4, both reigns
    // standing at three or more and held at the
    // raised cut. Passes under BB-8, neither
    // record carrying a level standing for the
    // count to run through.
    let championship = load_example("championship.json");
    let report = firstreign_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("firstreign, 2 series, 2 held, 0 single, 0 no winner")
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
    // FR-5. The constructed recovery
    // championship's closing line, asserting
    // none held and two single over its two
    // series, the seat whose DH-52 closing
    // stands at two unbroken and none broken
    // against this reading's split entire.
    // Fails under BB-5, the exchange classing
    // both series held and drawing their
    // sections. Fails under BB-6, the losers'
    // seats moving the closing. Fails under
    // BB-8, the reigns running through the
    // level standings of the pair. Passes under
    // BB-1, both first reigns standing at their
    // longest reigns of one. Passes under BB-2,
    // both front counts standing at one. Passes
    // under BB-3, both closing reigns standing
    // at one. Passes under BB-4, both reigns
    // standing at one and single at the raised
    // cut as before. Passes under BB-7, the
    // closing standing as before.
    let championship = load_example("constructed-recovery.json");
    let report = firstreign_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("firstreign, 2 series, 0 held, 2 single, 0 no winner")
    );
    assert_eq!(
        closing_counts(report.last().expect("a closing")),
        vec![2, 0, 2, 0]
    );
}

#[test]
fn the_constructed_firstreign_championship_closes_at_one_held_and_one_single() {
    // FR-6. The constructed firstreign
    // championship's closing line, asserting one
    // held and one single over its two series.
    // Fails under BB-1, the first drill's
    // longest reign of three classing it held.
    // Fails under BB-2, the first drill's front
    // count of five classing it held. Fails
    // under BB-3, the first drill's closing
    // reign of three classing it held. Fails
    // under BB-6, the losers' seats moving the
    // closing. Fails under BB-8, the first
    // drill's reign running through its level
    // standings to the whole record. Passes
    // under BB-4, the second drill's reign of
    // four standing held at the raised cut and
    // the first drill's standing single. Passes
    // under BB-5, the exchange standing this
    // pair at one and one still. Passes under
    // BB-7, the closing standing as before.
    let championship = load_example("constructed-firstreign.json");
    let report = firstreign_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("firstreign, 2 series, 1 held, 1 single, 0 no winner")
    );
    assert_eq!(
        closing_counts(report.last().expect("a closing")),
        vec![2, 1, 1, 0]
    );
}

#[test]
fn the_first_drill_classes_single_at_a_first_reign_of_one() {
    // FR-7. The first drill's entry assertion,
    // the series standing single at a first
    // reign of one, its longest reign at three,
    // its front count at five, and its closing
    // reign at three, the drill standing broken
    // under DH-52 in the pair's own record.
    // Fails under BB-1, the longest reign of
    // three classing it held. Fails under BB-2,
    // the front count of five classing it held.
    // Fails under BB-3, the closing reign of
    // three classing it held. Fails under BB-5,
    // the exchange classing the reign of one
    // held. Fails under BB-6, the winner never
    // standing behind over the record and the
    // loser's seat returning no reign. Fails
    // under BB-8, the reign running through the
    // level standings at the second and fourth
    // decided races to the whole of the record.
    // Passes under BB-4, the reign of one
    // standing single at the raised cut as
    // before. Passes under BB-7, the entry
    // standing outside the report's sections.
    let championship = load_example("constructed-firstreign.json");
    assert_eq!(
        firstreign_class(&championship.series[0]),
        Some((FirstreignClass::Single, 1))
    );
}

#[test]
fn the_second_drill_classes_held_at_a_first_reign_of_four() {
    // FR-8. The second drill's entry assertion,
    // the series standing held at a first reign
    // of four, the reign standing at an even
    // count as the closing run of the series,
    // its longest reign at four and its front
    // count at four, the drill standing
    // unbroken under DH-52 and from behind
    // under DH-38. Fails under BB-5, the
    // exchange classing the reign of four
    // single. Fails under BB-6, the first run of
    // behind standings of the winner standing
    // at one at the head of the record. Fails
    // under BB-8, the level standing at the
    // second decided race standing as an ahead
    // standing and the reign running from it at
    // five. Passes under BB-1, the longest reign
    // standing at four. Passes under BB-2, the
    // front count standing at four. Passes under
    // BB-3, the closing reign standing at four.
    // Passes under BB-4, the reign of four
    // standing held at the raised cut. Passes
    // under BB-7, the entry standing outside
    // the report's sections.
    let championship = load_example("constructed-firstreign.json");
    assert_eq!(
        firstreign_class(&championship.series[1]),
        Some((FirstreignClass::Held, 4))
    );
}

#[test]
fn the_panel_series_holds_at_a_first_reign_of_three_with_no_first_spell() {
    // FR-9. The panel series of the championship
    // example, its first reign standing at three
    // held and its first spell standing at none,
    // the winner holding every decided race of
    // the record and never standing behind.
    // Fails under BB-5, the exchange classing
    // the reign of three single. Fails under
    // BB-6, the winner never standing behind and
    // the loser's seat returning no reign.
    // Passes under BB-1, the longest reign
    // standing at three. Passes under BB-2, the
    // front count standing at three. Passes
    // under BB-3, the closing reign standing at
    // three. Passes under BB-4, the reign of
    // three standing held at the raised cut.
    // Passes under BB-7, the entry standing
    // outside the report's sections. Passes
    // under BB-8, the record carrying no level
    // standing for the count to run through.
    let championship = load_example("championship.json");
    let series = &championship.series[1];
    assert_eq!(firstreign_class(series), Some((FirstreignClass::Held, 3)));
    let winner = winner_of(series).expect("a decided series");
    let behind = behind_flags(&decided_record(series), winner);
    assert!(behind.iter().all(|flag| !flag));
}

#[test]
fn the_first_reign_never_stands_above_the_winners_longest_reign() {
    // FR-10. The theorem census of DH-66 over
    // the enrolled fifty nine series, the first
    // reign standing above the winner's longest
    // reign at no series, every series of the
    // enrolment holding a winner. Fails under
    // BB-2, the front counts standing above the
    // longest reigns at the broken series of
    // the enrolment. Fails under BB-6, the
    // losers' seats standing above the longest
    // reigns at series the enrolment carries.
    // Fails under BB-8, the reigns running
    // through level standings standing above
    // the longest reigns wherever a level
    // standing closes a reign, as at the first
    // drill of the constructed firstreign
    // championship. Passes under BB-1, the
    // figure standing at the longest reign
    // itself and never above it. Passes under
    // BB-3, the closing reign standing at or
    // below the longest by its form. Passes
    // under BB-4, BB-5, and BB-7, the census
    // reading the reign's count alone at the
    // entry, where the classes and sections
    // stand outside the count.
    let mut decided = 0;
    let mut violations = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, reign) = match firstreign_class(series) {
                Some(pair) => pair,
                None => continue,
            };
            decided += 1;
            if reign > longest_reign_of(series) {
                violations += 1;
            }
        }
    }
    assert_eq!(decided, 59);
    assert_eq!(violations, 0);
}

#[test]
fn the_first_reign_stands_at_the_whole_of_the_front_count_at_thirty_five_series() {
    // FR-11. The front count census of DH-66
    // over the enrolled fifty nine series, the
    // first reign standing at the whole of its
    // winner's front count at exactly thirty
    // five series, the second drill of the
    // constructed firstreign championship
    // standing among them beside the thirty
    // four unbroken series of the head's
    // enrolment. Fails under BB-2, the figure
    // standing at the front count itself at
    // every series and the count rising to the
    // whole enrolment. Fails under BB-6, the
    // losers' seats standing at the front counts
    // at other series and falling short at
    // these. Fails under BB-8, the reigns
    // running through level standings standing
    // at the front counts at fewer series.
    // Passes under BB-1, the longest reign
    // standing at the whole of the front count
    // exactly at the unbroken series. Passes
    // under BB-3, BB-4, BB-5, and BB-7, the
    // census reading the reign's count alone.
    let mut whole = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, reign) = match firstreign_class(series) {
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
    // FR-12. The wins target one fixture, a
    // series of one decided race at a target of
    // one, its winner winning the one race the
    // record holds before the decision and its
    // first reign standing at one single. Fails
    // under BB-5, the exchange classing the
    // reign of one held. Fails under BB-6, the
    // winner never standing behind over the one
    // race and the loser's seat returning no
    // reign. Passes under BB-1, the winner's
    // longest reign standing at one. Passes
    // under BB-2, the winner's front count
    // standing at one. Passes under BB-3, the
    // winner's closing reign standing at one.
    // Passes under BB-4, the reign of one
    // standing single at the raised cut as
    // before. Passes under BB-7, the pin reading
    // the entry alone and the report's sections
    // standing outside it. Passes under BB-8,
    // the one standing of the record standing
    // ahead and no level standing standing in
    // it.
    let dir = temp_dir("firstreign-fr12");
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
        firstreign_class(&championship.series[0]),
        Some((FirstreignClass::Single, 1))
    );
}

#[test]
fn a_series_holding_no_winner_takes_no_class_and_no_section() {
    // FR-13. The roster's characterization pin
    // of no posture. A series holding no
    // winner, its three races standing at two
    // wins against one at a target of three,
    // taking no class at the module's entry and
    // no section in the report, the report over
    // its championship standing in the empty
    // form with the series counted in the no
    // winner place alone. The pin falls under
    // no mutation of the eight, each mutation
    // reading a decided record the series does
    // not hold, and it stands in the roster so
    // the no winner place of DH-66 and DH-67
    // stands pinned in words.
    let dir = temp_dir("firstreign-fr13");
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
    assert_eq!(firstreign_class(&championship.series[0]), None);
    assert_eq!(
        firstreign_report(&championship),
        vec![
            "firstreign".to_string(),
            "firstreign, 1 series, 0 held, 0 single, 1 no winner".to_string(),
        ]
    );
}

#[test]
fn a_championship_holding_no_series_stands_refused_at_the_load() {
    // FR-14. The empty report form pin. A
    // championship holding no series at all
    // stands refused at the load, the empty
    // series list drawing the one defect
    // field series holds no series files, so
    // no championship stands for a report to
    // stand over. The pin falls under no
    // mutation of the eight, no series
    // standing in the championship for any
    // mutation to read, and it stands in the
    // roster so the load boundary of DH-67
    // stands pinned in words.
    let dir = temp_dir("firstreign-fr14");
    write_json(&dir, "championship.json", &championship_json(vec![]));
    let (championship, defects) = load_championship_file(&dir.join("championship.json"));
    assert_eq!(championship, None);
    assert_eq!(
        defects,
        vec!["field series holds no series files".to_string()]
    );
}

#[test]
fn the_second_drills_section_stands_in_the_family_form() {
    // FR-15. The firstreign pair section pin.
    // The section form over the firstreign
    // championship, drill two's section standing
    // in full with its series line, its models
    // and target line, its race lines over the
    // decided races numbered 1, 2, 3, 5, 6, and
    // 7, its figure line at a first reign of
    // four, and its series winner line, drill
    // one standing in no section, the whole
    // report standing line for line as the
    // golden stands. Fails under every variant
    // of the eight but BB-4. Under BB-1, drill
    // one's longest reign of three classing it
    // held and drawing its section. Under BB-2,
    // drill one's front count of five drawing
    // its section on the same move. Under BB-3,
    // drill one's closing reign of three drawing
    // its section. Under BB-5, the sections
    // moving between the drills under the
    // exchange. Under BB-6, drill two's reign
    // reading one from the loser's seat and its
    // section falling away while drill one gains
    // no section at none. Under BB-7, the
    // sections standing for the single series,
    // drill one drawing the section and drill
    // two losing it. Under BB-8, drill one's
    // reign running through its level standings
    // and drawing its section while drill two's
    // figure stands at five. Passes under BB-4,
    // drill two's reign of four standing held at
    // the raised cut and drill one's standing
    // single, the report standing word for word.
    let championship = load_example("constructed-firstreign.json");
    let expected: Vec<String> = [
        "firstreign",
        "series 2, the constructed firstreign drill two",
        "firstreign-ash against firstreign-birch, first to 5 wins",
        "race 1, firstreign drill two race one, firstreign-birch wins",
        "race 2, firstreign drill two race two, firstreign-ash wins",
        "race 3, firstreign drill two race three, firstreign-ash wins",
        "race 5, firstreign drill two race five, firstreign-ash wins",
        "race 6, firstreign drill two race six, firstreign-ash wins",
        "race 7, firstreign drill two race seven, firstreign-ash wins",
        "first reign 4 over the decided record",
        "series winner, firstreign-ash, firstreign-ash 5 race wins, firstreign-birch 1 race wins, 1 no results",
        "firstreign, 2 series, 1 held, 1 single, 0 no winner",
    ]
    .iter()
    .map(|line| line.to_string())
    .collect();
    assert_eq!(firstreign_report(&championship), expected);
    assert_eq!(
        firstreign_report(&championship).join("\n") + "\n",
        golden("constructed-firstreign-firstreign.txt")
    );
}

#[test]
fn an_even_first_reign_never_stands_below_the_front_count() {
    // FR-16. The parity census of DH-66 over
    // the enrolled fifty nine series, a first
    // reign standing at an even count below its
    // winner's front count at no series, every
    // run of ahead standings that a level
    // standing closes standing at an odd count,
    // so an even first reign stands as the
    // closing run of its series with the reign
    // at the whole of the front count. Fails
    // under BB-1, the longest reigns standing at
    // even counts below the front counts at the
    // broken series whose longest reign closes
    // on a level standing. Fails under BB-3, the
    // closing reigns standing at even counts
    // below the front counts where a series
    // stands ahead again after its close. Fails
    // under BB-6, the losers' seats standing at
    // even counts below the front counts at
    // series the enrolment carries. Fails under
    // BB-8, the reigns running through level
    // standings standing at even counts below
    // the front counts wherever a reign so
    // extended closes short of the record's
    // end. Passes under BB-2, the figure
    // standing at the front count itself and
    // never below it. Passes under BB-4, BB-5,
    // and BB-7, the census reading the reign's
    // count alone at the entry.
    let mut violations = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, reign) = match firstreign_class(series) {
                Some(pair) => pair,
                None => continue,
            };
            if reign % 2 == 0 && reign < front_count_of(series) {
                violations += 1;
            }
        }
    }
    assert_eq!(violations, 0);
}

#[test]
fn the_enrolled_championships_close_at_twenty_two_held_and_thirty_seven_single() {
    // FR-17. The aggregate closing over the
    // enrolled fifty nine series of the twenty
    // five championships, asserting twenty two
    // held, thirty seven single, and none
    // holding no winner, summed over the closing
    // line of each championship's report. Fails
    // under BB-1, BB-2, BB-3, BB-4, BB-5, and
    // BB-6, the fields of the enrolment moving
    // under each, and under BB-8, every reign
    // running through the level standings of
    // the enrolment. Passes under BB-7, the
    // closings standing as before while their
    // sections move.
    let mut totals = vec![0u64; 4];
    for championship in enrolled() {
        let report = firstreign_report(&championship);
        let counts = closing_counts(report.last().expect("a closing"));
        for (index, count) in counts.iter().enumerate() {
            totals[index] += count;
        }
    }
    assert_eq!(totals, vec![59, 22, 37, 0]);
}

#[test]
fn the_second_drills_section_spans_the_record_places_of_its_reign() {
    // FR-18. The span pin on the second drill.
    // Its section's race lines stand numbered by
    // the counting walk's place among the
    // decided races at 1, 2, 3, 5, 6, and 7, the
    // race holding no winner at the fourth place
    // of the record standing in no race line,
    // and its figure line stands at a first
    // reign of four, the skipped place standing
    // inside the span of the reign itself. Fails
    // under BB-5, the second drill classing
    // single under the exchange and its section
    // falling away. Fails under BB-6, the second
    // drill's reign reading one from the loser's
    // seat and its section falling away. Fails
    // under BB-7, the sections standing for the
    // single series and the second drill losing
    // its section to the first. Fails under
    // BB-8, the figure standing at five with the
    // level standing of the second decided race
    // counted in the reign. Passes under BB-1,
    // the second drill's longest reign standing
    // at four and its section standing as
    // before. Passes under BB-2, its front count
    // standing at four on the same ground.
    // Passes under BB-3, its closing reign
    // standing at four on the same ground.
    // Passes under BB-4, the reign of four
    // standing held at the raised cut and the
    // section standing.
    let championship = load_example("constructed-firstreign.json");
    let report = firstreign_report(&championship);
    let start = report
        .iter()
        .position(|line| line.starts_with("series 2,"))
        .expect("the second drill's section");
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
    assert_eq!(places, vec![1, 2, 3, 5, 6, 7]);
    assert!(report.contains(&"first reign 4 over the decided record".to_string()));
}

#[test]
fn the_first_reign_stands_below_the_longest_reign_at_nine_series() {
    // FR-19. The parting census of DH-66 over
    // the enrolled fifty nine series, the first
    // reign standing below the winner's longest
    // reign at exactly nine series, the first
    // drill of the constructed firstreign
    // championship adding the ninth to the eight
    // of the head's enrolment. Fails under BB-1,
    // the figure standing at the longest reign
    // itself at every series and the count
    // falling to none. Fails under BB-2, the
    // front counts standing at or above the
    // longest reigns at every series and the
    // count falling to none. Fails under BB-3,
    // the closing reigns standing below the
    // longest at a count of their own. Fails
    // under BB-6, the losers' seats standing
    // below the longest reigns at a count of
    // their own. Fails under BB-8, the reigns
    // running through level standings standing
    // below the longest at fewer series. Passes
    // under BB-4, BB-5, and BB-7, the census
    // reading the reign's count alone at the
    // entry.
    let mut below = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, reign) = match firstreign_class(series) {
                Some(pair) => pair,
                None => continue,
            };
            if reign < longest_reign_of(series) {
                below += 1;
            }
        }
    }
    assert_eq!(below, 9);
}

#[test]
fn the_first_reign_stands_at_twice_the_first_strike_less_one_at_its_seats() {
    // FR-20. The twice the strike less one
    // census of DH-66 over the enrolled fifty
    // nine series. At every series whose winner
    // won the first decided race and whose
    // loser's first stand stands at or above
    // its first strike, the first reign stands
    // at twice the first strike less one, the
    // reply of the loser levelling the tallies
    // at that count and closing the reign there,
    // and the seats stand at twenty over the
    // enrolment, the second series of the
    // constructed peak championship standing
    // among the nineteen of the head's enrolment
    // and the constructed pair's first drill
    // standing beside them. Fails under BB-1,
    // the longest reigns standing above twice
    // the strike less one at seats whose winner
    // stood ahead again in a longer run. Fails
    // under BB-2, the front counts standing
    // above that count at the broken seats.
    // Fails under BB-3, the closing reigns
    // standing off that count at seats whose
    // record closes in a later run. Fails under
    // BB-6, the losers' seats standing off that
    // count at their own seats. Fails under
    // BB-8, the reigns running through the level
    // standing that closes them standing at
    // twice the strike at these seats. Passes
    // under BB-4, BB-5, and BB-7, the census
    // reading the reign's count alone at the
    // entry, the seats and the strike and stand
    // figures standing outside the reading.
    let mut seats = 0;
    let mut violations = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, reign) = match firstreign_class(series) {
                Some(pair) => pair,
                None => continue,
            };
            let record = decided_record(series);
            let winner = winner_of(series).expect("a decided series");
            if record.first() != Some(&winner) {
                continue;
            }
            let strike = runs_of(&record, winner).first().copied().unwrap_or(0) as u64;
            let stand = runs_of(&record, loser_of(series))
                .first()
                .copied()
                .unwrap_or(0) as u64;
            if stand < strike {
                continue;
            }
            seats += 1;
            if reign != 2 * strike - 1 {
                violations += 1;
            }
        }
    }
    assert_eq!(seats, 20);
    assert_eq!(violations, 0);
}
