//! Pins for DH-70 and DH-71 in the Round 36
//! section of CONTRACT.md. The discrimination
//! record of this file stands pin by pin, and
//! no blanket claim stands over it. The lead
//! changes reading, its library function, and
//! its command do not stand on the pre round
//! tree, so every pin in this file that names
//! the lead changes reading is absent there at
//! compile time. That absence is recorded
//! once here and is not a discrimination
//! claim. The discrimination each pin carries
//! is against the stated mutation of DH-70
//! beside it. Eight mutations are named for
//! the whole file. DD-1, the level count of
//! the series standing as its figure in the
//! changes' place. DD-2, the record changes
//! of DH-40 standing as the figure. DD-3, the
//! passes to the winner alone standing as the
//! figure, the changes counted only where the
//! lead passed from the loser to the winner.
//! DD-4, the swung class standing at three
//! changes or more, a series at two changes
//! classed turned, the rest of the reading
//! unchanged. DD-5, the swung and turned
//! classes exchanged, a series at two or more
//! changes classed turned and a series at one
//! classed swung. DD-6, the report carrying
//! its sections for the turned series instead
//! of the swung, the closing standing as
//! before. DD-7, the changes counted with the
//! level standings standing as ahead
//! standings, a change running through a
//! level standing counted on the coarsened
//! record. DD-8, the passes to the loser
//! alone standing as the figure, the changes
//! counted only where the lead passed from
//! the winner to the loser.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{load_championship_file, Championship};
use deadheat::json::JsonValue;
use deadheat::leadchanges::{leadchanges_class, leadchanges_report, LeadchangesClass};
use deadheat::races::decide_winner;
use deadheat::series::Series;
use num_bigint::BigInt;
use shared::*;

const ALPHA: &str = "alpha-1";
const BETA: &str = "beta-1";

const ENROLLED: [&str; 27] = [
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
    "constructed-leadchanges.json",
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

fn signs_of(series: &Series) -> Vec<i64> {
    let winner = winner_of(series).expect("a decided series");
    let mut tallies = [0i64; 2];
    let mut signs = Vec::new();
    for side in &decided_record(series) {
        tallies[*side] += 1;
        let diff = tallies[winner] - tallies[1 - winner];
        signs.push(if diff > 0 {
            1
        } else if diff < 0 {
            -1
        } else {
            0
        });
    }
    signs
}

fn level_count_of(series: &Series) -> u64 {
    signs_of(series).iter().filter(|sign| **sign == 0).count() as u64
}

fn record_changes_of(series: &Series) -> u64 {
    let decided = decided_record(series);
    let mut changes = 0u64;
    for pair in decided.windows(2) {
        if pair[0] != pair[1] {
            changes += 1;
        }
    }
    changes
}

#[test]
fn the_classification_reads_the_changes_of_hand_of_the_lead() {
    // LC-1. The leadchanges characterization
    // pin. A hand fixture at a target of six,
    // its blocks standing winner two, loser
    // two, winner one, loser one, and winner
    // three, its decided record closing at the
    // sixth win of its winner on the last race
    // of the record, the winner championing
    // never behind with its changes of hand at
    // none, its front count at seven, its first
    // reign at three, its level count at two,
    // its record changes at four, and its first
    // spell at none, the winner standing ahead
    // or level at every standing of the record.
    // It asserts the changes at none and the
    // never behind class over it at the module's
    // entry, the neighbour figures beside it,
    // and the report in its sectionless form
    // over the fixture, the closing line of its
    // one series championship standing below it.
    // Fails under DD-1, the level count of two
    // standing as the figure and classing the
    // series swung. Fails under DD-2, the record
    // changes of four standing as the figure.
    // Passes under DD-3, the passes to the
    // winner standing at none as the changes do.
    // Passes under DD-4, no change standing at
    // the raised cut. Passes under DD-5, the
    // exchange leaving a series at none in the
    // never behind class. Passes under DD-6, no
    // section standing for a series in no swung
    // or turned class under either form. Passes
    // under DD-7, the coarsened count standing
    // at none over a record the winner never
    // left. Passes under DD-8, the passes to the
    // loser standing at none as the changes do.
    let dir = temp_dir("leadchanges-lc1");
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
    assert_eq!(
        leadchanges_class(series),
        Some((LeadchangesClass::NeverBehind, 0))
    );
    assert_eq!(front_count_of(series), 7);
    assert_eq!(first_reign_of(series), 3);
    assert_eq!(level_count_of(series), 2);
    assert_eq!(record_changes_of(series), 4);
    let behind = behind_flags(
        &decided_record(series),
        winner_of(series).expect("a winner"),
    );
    assert!(behind.iter().all(|flag| !flag));
    assert_eq!(
        leadchanges_report(&championship),
        vec![
            "leadchanges".to_string(),
            "leadchanges, 1 series, 0 swung, 0 turned, 1 never behind, 0 no winner".to_string(),
        ]
    );
}

#[test]
fn the_survey_championship_closes_at_two_swung_two_turned_and_two_never_behind() {
    // LC-2. The survey championship's closing
    // line, asserting two swung, two turned,
    // and two never behind over its six series.
    // Fails under DD-1, the level counts moving
    // the closing. Fails under DD-2, the record
    // changes moving the closing. Fails under
    // DD-3, the passes to the winner moving the
    // closing. Fails under DD-4, the survey
    // series standing at two changes classing
    // turned at the raised cut. Fails under
    // DD-5, the classes standing exchanged over
    // the field. Fails under DD-8, the passes
    // to the loser moving the closing. Passes
    // under DD-6, the closing standing as
    // before while its sections move. Passes
    // under DD-7, the survey changes standing
    // at the coarsened count under the level as
    // ahead count at every survey series.
    let championship = load_example("survey-championship.json");
    let report = leadchanges_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("leadchanges, 6 series, 2 swung, 2 turned, 2 never behind, 0 no winner")
    );
    assert_eq!(
        closing_counts(report.last().expect("a closing")),
        vec![6, 2, 2, 2, 0]
    );
}

#[test]
fn the_league_championship_closes_at_one_swung_four_turned_and_five_never_behind() {
    // LC-3. The league championship's closing
    // line, asserting one swung, four turned,
    // and five never behind over its ten
    // series. Fails under DD-1, DD-2, DD-3,
    // DD-4, DD-5, DD-7, and DD-8, the league
    // fields moving under each. Passes under
    // DD-6, the closing standing as before
    // while its sections move.
    let championship = load_example("league-championship.json");
    let report = leadchanges_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("leadchanges, 10 series, 1 swung, 4 turned, 5 never behind, 0 no winner")
    );
    assert_eq!(
        closing_counts(report.last().expect("a closing")),
        vec![10, 1, 4, 5, 0]
    );
}

#[test]
fn the_championship_example_closes_at_two_never_behind_with_no_section() {
    // LC-4. The championship example's closing
    // line, asserting two never behind and no
    // swung or turned over its two series, with
    // no section standing for either series.
    // Fails under DD-2 alone, the pilot series'
    // record changes standing at two there,
    // classing it swung and drawing its
    // section. Passes under DD-1, both level
    // counts standing at nought. Passes under
    // DD-3 and DD-8, no pass of either
    // direction standing in either record.
    // Passes under DD-4, no series standing at
    // the raised cut. Passes under DD-5, the
    // classes standing exchanged over two never
    // behind series and the closing standing
    // word for word. Passes under DD-6, the
    // sections standing for the turned series
    // and this championship carrying none.
    // Passes under DD-7, the coarsened counts
    // standing at the changes over two records
    // carrying no level standing.
    let championship = load_example("championship.json");
    let report = leadchanges_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("leadchanges, 2 series, 0 swung, 0 turned, 2 never behind, 0 no winner")
    );
    let sections: Vec<&String> = report
        .iter()
        .filter(|line| line.starts_with("series ") && !line.starts_with("series winner"))
        .collect();
    assert_eq!(sections.len(), 0);
}

#[test]
fn the_constructed_recovery_championship_closes_at_two_turned() {
    // LC-5. The constructed recovery
    // championship's closing line, asserting
    // two turned and none swung or never
    // behind over its two series. Fails under
    // DD-2, the record changes moving the
    // closing. Fails under DD-5, the exchange
    // classing both series swung and drawing
    // their sections. Fails under DD-8, the
    // passes to the loser moving the closing.
    // Passes under DD-1, the level counts
    // classing both series turned there as the
    // changes do. Passes under DD-3, the passes
    // to the winner standing at one over both
    // series as the changes do. Passes under
    // DD-4, both series standing at one and
    // turned at the raised cut as before.
    // Passes under DD-6, the closing standing
    // as before. Passes under DD-7, the
    // coarsened counts standing at the changes
    // over both records.
    let championship = load_example("constructed-recovery.json");
    let report = leadchanges_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("leadchanges, 2 series, 0 swung, 2 turned, 0 never behind, 0 no winner")
    );
    assert_eq!(
        closing_counts(report.last().expect("a closing")),
        vec![2, 0, 2, 0, 0]
    );
}

#[test]
fn the_constructed_leadchanges_championship_closes_at_one_swung_and_one_turned() {
    // LC-6. The constructed leadchanges
    // championship's closing line, asserting
    // one swung and one turned over its two
    // series, the pair standing built for this
    // reading. Fails under DD-1, the level
    // counts of the drills standing at two
    // each and both classing swung there.
    // Fails under DD-2, the record changes of
    // the drills moving the closing. Fails
    // under DD-3, the passes to the winner
    // standing at one over both drills and the
    // closing standing at none swung. Fails
    // under DD-4, the first drill's changes of
    // two classing turned at the raised cut.
    // Fails under DD-5, the exchange standing
    // this pair at one and one the other way.
    // Fails under DD-8, the passes to the loser
    // moving the closing. Passes under DD-6,
    // the closing standing as before. Passes
    // under DD-7, the coarsened counts standing
    // at the changes over both drills.
    let championship = load_example("constructed-leadchanges.json");
    let report = leadchanges_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("leadchanges, 2 series, 1 swung, 1 turned, 0 never behind, 0 no winner")
    );
    assert_eq!(
        closing_counts(report.last().expect("a closing")),
        vec![2, 1, 1, 0, 0]
    );
}

#[test]
fn the_first_drill_classes_swung_at_two_changes_of_hand() {
    // LC-7. The first drill's entry assertion,
    // the series standing swung at two changes
    // of hand, its standings running ahead,
    // level, behind, level, ahead over its
    // decided record, its first reign at one
    // and its last reign at one. Fails under
    // DD-3, the passes to the winner standing
    // at one and classing it turned. Fails
    // under DD-4, the changes of two classing
    // turned at the raised cut. Fails under
    // DD-5, the exchange classing the changes
    // of two turned. Fails under DD-8, the
    // passes to the loser standing at one.
    // Passes under DD-1, the level count
    // standing at two as the changes do.
    // Passes under DD-2, the record changes
    // standing at two as the changes do.
    // Passes under DD-6, the entry standing
    // outside the report's sections. Passes
    // under DD-7, the coarsened count standing
    // at two over the drill's record.
    let championship = load_example("constructed-leadchanges.json");
    assert_eq!(
        leadchanges_class(&championship.series[0]),
        Some((LeadchangesClass::Swung, 2))
    );
}

#[test]
fn the_second_drill_classes_turned_at_one_change_of_hand() {
    // LC-8. The second drill's entry
    // assertion, the series standing turned at
    // one change of hand, its standings running
    // behind, level, ahead, level, ahead over
    // its decided record, its first reign at
    // one and its last reign at one. Fails
    // under DD-1, the level count of two
    // standing as the figure and classing it
    // swung. Fails under DD-2, the record
    // changes of three standing as the figure.
    // Fails under DD-5, the exchange classing
    // the change of one swung. Fails under
    // DD-8, the passes to the loser standing
    // at none and classing it never behind.
    // Passes under DD-3, the passes to the
    // winner standing at one as the change
    // does. Passes under DD-4, the change of
    // one standing turned at the raised cut as
    // before. Passes under DD-6, the entry
    // standing outside the report's sections.
    // Passes under DD-7, the coarsened count
    // standing at one over the drill's record.
    let championship = load_example("constructed-leadchanges.json");
    assert_eq!(
        leadchanges_class(&championship.series[1]),
        Some((LeadchangesClass::Turned, 1))
    );
}

#[test]
fn the_panel_series_stands_never_behind_with_no_level_standing() {
    // LC-9. The panel series of the
    // championship example, its changes of
    // hand standing at none in the never
    // behind class and its level count
    // standing at none, the winner holding
    // every decided race of the record and
    // never standing behind, its last reign
    // standing at three held under DH-68 and
    // its record changes standing at none.
    // The pin is characterization under the
    // variant family, the series standing at
    // the meeting point of the level count,
    // the record changes, both pass counts,
    // and the coarsened count at none, so it
    // falls under no variant of DD-1 to DD-8.
    let championship = load_example("championship.json");
    let series = &championship.series[1];
    assert_eq!(
        leadchanges_class(series),
        Some((LeadchangesClass::NeverBehind, 0))
    );
    assert_eq!(level_count_of(series), 0);
    assert_eq!(record_changes_of(series), 0);
    let winner = winner_of(series).expect("a decided series");
    let behind = behind_flags(&decided_record(series), winner);
    assert!(behind.iter().all(|flag| !flag));
}

#[test]
fn the_changes_of_hand_never_stand_above_the_level_count() {
    // LC-10. The theorem census of DH-70 over
    // the enrolled sixty three series, the
    // changes of hand standing above the count
    // of the level standings of a series at no
    // series, every series of the enrolment
    // holding a winner. Fails under DD-2, the
    // record changes standing above the level
    // counts at series of the enrolment.
    // Fails under DD-7, the coarsened counts
    // standing above the level counts wherever
    // a level standing runs into a pass.
    // Passes under DD-1, the figure standing at
    // the level count itself and never above
    // it. Passes under DD-3 and DD-8, each pass
    // count standing at or below the changes,
    // which stand at or below the level count.
    // Passes under DD-4, DD-5, and DD-6, the
    // census reading the changes' count alone
    // at the entry, where the classes and
    // sections stand outside the count.
    let mut decided = 0;
    let mut violations = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, changes) = match leadchanges_class(series) {
                Some(pair) => pair,
                None => continue,
            };
            decided += 1;
            if changes > level_count_of(series) {
                violations += 1;
            }
        }
    }
    assert_eq!(decided, 63);
    assert_eq!(violations, 0);
}

#[test]
fn the_changes_stand_at_the_whole_of_the_level_count_at_thirty_eight_series() {
    // LC-11. The level census of DH-70 over
    // the enrolled sixty three series, the
    // changes of hand standing at the whole of
    // the level count at exactly thirty eight
    // series, every apart standing of those
    // series running through a level standing.
    // Fails under DD-1, the figure standing at
    // the level count itself at every series
    // and the count rising to the whole
    // enrolment. Fails under DD-2, the record
    // changes standing at the level counts at
    // other series. Fails under DD-3, the
    // passes to the winner standing at the
    // level counts at fewer series. Fails
    // under DD-8, the passes to the loser
    // standing at the level counts at their
    // own seats. Passes under DD-4, DD-5, and
    // DD-6, the census reading the changes'
    // count alone. Passes under DD-7, the
    // coarsened count standing at the level
    // count exactly at the series whose
    // changes stand there.
    let mut whole = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, changes) = match leadchanges_class(series) {
                Some(pair) => pair,
                None => continue,
            };
            if changes == level_count_of(series) {
                whole += 1;
            }
        }
    }
    assert_eq!(whole, 38);
}

#[test]
fn a_series_at_a_wins_target_of_one_stands_never_behind() {
    // LC-12. The wins target one fixture, a
    // series of one decided race at a target
    // of one, its winner winning the one race
    // the record holds before the decision and
    // its changes of hand standing at none in
    // the never behind class, no standing of
    // the record standing level or behind. The
    // pin is characterization under the
    // variant family, every figure of the
    // family standing at none over the one
    // standing, so it falls under no variant
    // of DD-1 to DD-8.
    let dir = temp_dir("leadchanges-lc12");
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
        leadchanges_class(&championship.series[0]),
        Some((LeadchangesClass::NeverBehind, 0))
    );
}

#[test]
fn a_series_holding_no_winner_takes_no_class_and_no_section() {
    // LC-13. The roster's characterization pin
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
    // DH-70 and DH-71 stands pinned in words.
    let dir = temp_dir("leadchanges-lc13");
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
    assert_eq!(leadchanges_class(&championship.series[0]), None);
    assert_eq!(
        leadchanges_report(&championship),
        vec![
            "leadchanges".to_string(),
            "leadchanges, 1 series, 0 swung, 0 turned, 0 never behind, 1 no winner".to_string(),
        ]
    );
}

#[test]
fn a_championship_holding_no_series_stands_refused_at_the_load() {
    // LC-14. The empty report form pin. A
    // championship holding no series at all
    // stands refused at the load, the empty
    // series list drawing the one defect field
    // series holds no series files, so no
    // championship stands for a report to
    // stand over. The pin falls under no
    // mutation of the eight, no series
    // standing in the championship for any
    // mutation to read, and it stands in the
    // roster so the load boundary of DH-71
    // stands pinned in words.
    let dir = temp_dir("leadchanges-lc14");
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
    // LC-15. The leadchanges pair section pin,
    // the roster's strongest pin. The section
    // form over the leadchanges championship,
    // drill one's section standing in full with
    // its series line, its models and target
    // line, its race lines over the decided
    // races at the record places 1, 2, 3, 4,
    // and 6, its figure line at two changes of
    // hand, and its series winner line, drill
    // two standing in no section, the whole
    // report standing line for line as the
    // golden stands. Fails under every variant
    // of the eight but DD-7. Under DD-1, the
    // level counts of the drills classing both
    // swung and drill two drawing its section.
    // Under DD-2, the record changes moving
    // both drills' classes and sections. Under
    // DD-3, drill one's passes to the winner
    // standing at one, classing it turned and
    // losing its section. Under DD-4, drill
    // one's changes of two classing turned at
    // the raised cut and its section falling
    // away. Under DD-5, the sections moving
    // between the drills under the exchange.
    // Under DD-6, the sections standing for
    // the turned series, drill two drawing the
    // section and drill one losing it. Under
    // DD-8, drill one's passes to the loser
    // standing at one and its section falling
    // away. Passes under DD-7, the coarsened
    // counts standing at the changes over both
    // drills and the report standing word for
    // word.
    let championship = load_example("constructed-leadchanges.json");
    let expected: Vec<String> = [
        "leadchanges",
        "series 1, the constructed leadchanges drill one",
        "leadchanges-ash against leadchanges-birch, first to 3 wins",
        "race 1, leadchanges drill one race one, leadchanges-birch wins",
        "race 2, leadchanges drill one race two, leadchanges-ash wins",
        "race 3, leadchanges drill one race three, leadchanges-ash wins",
        "race 4, leadchanges drill one race four, leadchanges-birch wins",
        "race 6, leadchanges drill one race six, leadchanges-birch wins",
        "changes of hand of the lead 2 over the decided record",
        "series winner, leadchanges-birch, leadchanges-ash 2 race wins, leadchanges-birch 3 race wins, 1 no results",
        "leadchanges, 2 series, 1 swung, 1 turned, 0 never behind, 0 no winner",
    ]
    .iter()
    .map(|line| line.to_string())
    .collect();
    assert_eq!(leadchanges_report(&championship), expected);
    assert_eq!(
        leadchanges_report(&championship).join("\n") + "\n",
        golden("constructed-leadchanges-leadchanges.txt")
    );
}

#[test]
fn the_changes_stand_below_the_level_count_at_twenty_five_series() {
    // LC-16. The below census of DH-70 over
    // the enrolled sixty three series, the
    // changes of hand standing below the level
    // count at exactly twenty five series, a
    // level standing of those series standing
    // between two standings of the same side.
    // Fails under DD-1, the figure standing at
    // the level count itself at every series
    // and the count falling to none. Fails
    // under DD-2, the record changes standing
    // below the level counts at their own
    // seats. Fails under DD-3, the passes to
    // the winner standing below the level
    // counts at fewer series. Fails under
    // DD-7, the coarsened counts standing
    // below the level counts at other series.
    // Fails under DD-8, the passes to the
    // loser standing below the level counts at
    // their own seats. Passes under DD-4,
    // DD-5, and DD-6, the census reading the
    // changes' count alone at the entry.
    let mut below = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, changes) = match leadchanges_class(series) {
                Some(pair) => pair,
                None => continue,
            };
            if changes < level_count_of(series) {
                below += 1;
            }
        }
    }
    assert_eq!(below, 25);
}

#[test]
fn the_enrolled_championships_close_at_eleven_swung_twenty_six_turned_and_twenty_six_never_behind()
{
    // LC-17. The aggregate closing over the
    // enrolled sixty three series of the
    // twenty seven championships, asserting
    // eleven swung, twenty six turned, twenty
    // six never behind, and none holding no
    // winner, summed over the closing line of
    // each championship's report. Fails under
    // DD-1, DD-2, DD-3, DD-4, DD-5, DD-7, and
    // DD-8, the fields of the enrolment moving
    // under each. Passes under DD-6, the
    // closings standing as before while their
    // sections move.
    let mut totals = vec![0u64; 5];
    for championship in enrolled() {
        let report = leadchanges_report(&championship);
        let counts = closing_counts(report.last().expect("a closing"));
        for (index, count) in counts.iter().enumerate() {
            totals[index] += count;
        }
    }
    assert_eq!(totals, vec![63, 11, 26, 26, 0]);
}

#[test]
fn the_first_drills_section_spans_the_record_places_of_its_changes() {
    // LC-18. The span pin on the first drill.
    // Its section's race lines stand numbered
    // by the counting walk's place among the
    // decided races at 1, 2, 3, 4, and 6, the
    // race holding no winner at the fifth
    // place of the record standing in no race
    // line, and its figure line stands at two
    // changes of hand, the skipped place
    // standing inside the span of the changes
    // themselves, the span witness of the
    // roster. Fails under DD-1, the level
    // count of the drill standing at two and
    // its figure standing there while drill
    // two gains a section beside it. Fails
    // under DD-2, the record changes moving
    // the drill's class and section. Fails
    // under DD-3, the drill classing turned at
    // its passes to the winner of one and its
    // section falling away. Fails under DD-4,
    // the drill classing turned at the raised
    // cut and its section falling away. Fails
    // under DD-5, the drill classing turned
    // under the exchange and its section
    // falling away. Fails under DD-6, the
    // sections standing for the turned series
    // and the first drill losing its section
    // to the second. Fails under DD-8, the
    // drill classing turned at its passes to
    // the loser of one and its section falling
    // away. Passes under DD-7, the coarsened
    // count standing at two and the section
    // standing as before.
    let championship = load_example("constructed-leadchanges.json");
    let report = leadchanges_report(&championship);
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
    assert_eq!(places, vec![1, 2, 3, 4, 6]);
    assert!(report.contains(&"changes of hand of the lead 2 over the decided record".to_string()));
}

#[test]
fn the_changes_stand_at_none_at_twenty_six_series() {
    // LC-19. The nought census of DH-70 over
    // the enrolled sixty three series, the
    // changes of hand standing at none at
    // exactly twenty six series, the
    // never-behind class of DH-70 itself,
    // standing set for set as DH-50's class
    // over the enrolment. Fails under DD-1,
    // the level counts standing at none at
    // fewer series. Fails under DD-2, the
    // record changes standing at none at
    // fewer series, every series that ever
    // changed its winner holding a record
    // change. Fails under DD-8, the passes to
    // the loser standing at none at more
    // series, the series that took the lead
    // from behind and kept it standing among
    // them. Passes under DD-3, the passes to
    // the winner standing at none exactly at
    // the series whose changes stand at none
    // over this enrolment. Passes under DD-4,
    // DD-5, and DD-6, the census reading the
    // changes' count alone. Passes under DD-7,
    // the coarsened count standing at none
    // exactly where the changes stand at none.
    let mut none = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, changes) = match leadchanges_class(series) {
                Some(pair) => pair,
                None => continue,
            };
            if changes == 0 {
                none += 1;
            }
        }
    }
    assert_eq!(none, 26);
}

#[test]
fn the_changes_stand_odd_at_twenty_eight_series() {
    // LC-20. The parity census of DH-70 over
    // the enrolled sixty three series, the
    // changes of hand standing odd at exactly
    // twenty eight series, the series standing
    // odd exactly on DH-38's from behind set
    // over the enrolment, a series standing
    // odd where its winner first stood behind.
    // Fails under DD-3, the passes to the
    // winner standing odd at their own seats.
    // Fails under DD-8, the passes to the
    // loser standing odd at their own seats.
    // Passes under DD-1, the level counts
    // standing odd at the same twenty eight
    // series over this enrolment. Passes under
    // DD-2, the record changes standing odd
    // exactly where the changes stand odd.
    // Passes under DD-4, DD-5, and DD-6, the
    // census reading the changes' count alone.
    // Passes under DD-7, the coarsened counts
    // standing odd at the same series.
    let mut odd = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, changes) = match leadchanges_class(series) {
                Some(pair) => pair,
                None => continue,
            };
            if changes % 2 == 1 {
                odd += 1;
            }
        }
    }
    assert_eq!(odd, 28);
}
