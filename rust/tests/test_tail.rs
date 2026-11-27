//! Pins for DH-74 and DH-75 in the Round 38
//! section of CONTRACT.md. The discrimination
//! record of this file stands pin by pin, and
//! no blanket claim stands over it. The tail
//! reading, its library function, and its
//! command do not stand on the pre round
//! tree, so every pin in this file that names
//! the tail reading is absent there at
//! compile time. That absence is recorded
//! once here and is not a discrimination
//! claim. The discrimination each pin carries
//! is against the stated mutation of DH-74
//! beside it. Eight mutations are named for
//! the whole file. FF-1, the longest spell of
//! DH-50 standing as the second figure in the
//! last spell's place. FF-2, the longest
//! reign of DH-52 standing as the first figure
//! in the last reign's place. FF-3, the first
//! spell standing as the second figure. FF-4,
//! the first reign standing as the first
//! figure. FF-5, the classes exchanged, a
//! series whose last reign stands above its
//! last spell standing spell longer and a
//! series whose last spell stands above its
//! last reign standing reign longer, matched
//! standing unchanged. FF-6, the report
//! carrying its sections for the matched
//! series instead of the spell longer, the
//! closing standing as before. FF-7, the
//! closing run of DH-36 standing as the first
//! figure in the last reign's place. FF-8,
//! the last stand standing as the second
//! figure in the last spell's place.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{load_championship_file, Championship};
use deadheat::json::JsonValue;
use deadheat::races::decide_winner;
use deadheat::series::Series;
use deadheat::tail::{tail_class, tail_report, TailClass};
use num_bigint::BigInt;
use shared::*;

const ALPHA: &str = "alpha-1";
const BETA: &str = "beta-1";

const ENROLLED: [&str; 29] = [
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
    "constructed-outset.json",
    "constructed-peak.json",
    "constructed-place.json",
    "constructed-rally.json",
    "constructed-recovery.json",
    "constructed-reigns.json",
    "constructed-spells.json",
    "constructed-streak.json",
    "constructed-tail.json",
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

fn runs_of(flags: &[bool]) -> Vec<u64> {
    let mut runs = Vec::new();
    let mut current = 0u64;
    for flag in flags {
        if *flag {
            current += 1;
        } else if current > 0 {
            runs.push(current);
            current = 0;
        }
    }
    if current > 0 {
        runs.push(current);
    }
    runs
}

fn last_reign_of(series: &Series) -> u64 {
    let winner = winner_of(series).expect("a decided series");
    runs_of(&ahead_flags(&decided_record(series), winner))
        .into_iter()
        .last()
        .unwrap_or(0)
}

fn last_spell_of(series: &Series) -> u64 {
    let winner = winner_of(series).expect("a decided series");
    runs_of(&behind_flags(&decided_record(series), winner))
        .into_iter()
        .last()
        .unwrap_or(0)
}

fn longest_spell_of(series: &Series) -> u64 {
    let winner = winner_of(series).expect("a decided series");
    runs_of(&behind_flags(&decided_record(series), winner))
        .into_iter()
        .max()
        .unwrap_or(0)
}

fn front_count_of(series: &Series) -> u64 {
    let winner = winner_of(series).expect("a decided series");
    ahead_flags(&decided_record(series), winner)
        .iter()
        .filter(|flag| **flag)
        .count() as u64
}

fn behind_count_of(series: &Series) -> u64 {
    let winner = winner_of(series).expect("a decided series");
    behind_flags(&decided_record(series), winner)
        .iter()
        .filter(|flag| **flag)
        .count() as u64
}

#[test]
fn the_classification_reads_the_last_reign_against_the_last_spell() {
    // TA-1. The tail characterization pin
    // over the constructed pair. The first
    // drill seats birch as champion at a wins
    // target of five, winning five races to
    // four, its standings running behind,
    // level, behind, behind, behind, level,
    // behind, level, ahead, its last reign at
    // one and its last spell at one, matched.
    // The second drill seats ash as champion
    // at the same target and split, its
    // standings running behind, level, behind,
    // level, behind, behind, behind, level,
    // ahead, its last reign at one and its
    // last spell at three, spell longer. The
    // pair shares the wins split, the margin,
    // the front count at one, the behind count
    // at five, the level standings at three,
    // the first reign at one, the longest
    // reign at one, the last reign at one, the
    // first spell at one, the longest spell at
    // three, and the changes of hand at one,
    // and parts at the last spell alone within
    // that family. It asserts both drills'
    // classes and figures at the module's
    // entry and the neighbour figures beside
    // them. Fails under FF-1, the longest spell
    // of three standing as the first drill's
    // second figure and classing it spell
    // longer. Fails under FF-3, the first spell
    // of one standing as the second drill's
    // second figure and classing it matched.
    // Fails under FF-5, the exchange classing
    // the second drill reign longer. Fails
    // under FF-7, the closing run of two
    // standing as the first drill's first
    // figure and classing it reign longer.
    // Fails under FF-8, the last stand of two
    // standing as the second drill's second
    // figure, its entry figure parting while
    // its class stands. Passes under FF-2 and
    // FF-4, the longest and first reigns
    // standing at one over both drills as the
    // last reign does. Passes under FF-6, the
    // entry standing outside the report's
    // sections.
    let championship = load_example("constructed-tail.json");
    let first = &championship.series[0];
    let second = &championship.series[1];
    assert_eq!(tail_class(first), Some((TailClass::Matched, 1, 1)));
    assert_eq!(tail_class(second), Some((TailClass::SpellLonger, 1, 3)));
    assert_eq!(last_reign_of(first), 1);
    assert_eq!(last_spell_of(first), 1);
    assert_eq!(longest_spell_of(first), 3);
    assert_eq!(last_reign_of(second), 1);
    assert_eq!(last_spell_of(second), 3);
    assert_eq!(longest_spell_of(second), 3);
    assert_eq!(front_count_of(first), 1);
    assert_eq!(behind_count_of(first), 5);
    assert_eq!(front_count_of(second), 1);
    assert_eq!(behind_count_of(second), 5);
}

#[test]
fn the_survey_championship_closes_at_four_reign_longer_and_two_matched() {
    // TA-2. The survey championship's closing
    // line, asserting four reign longer, no
    // spell longer, and two matched over its
    // six series. Fails under FF-2, the longest
    // reigns moving the closing. Fails under
    // FF-5, the classes standing exchanged over
    // the field. Fails under FF-7, the closing
    // runs standing as the first figures and
    // moving the closing. Fails under FF-8, the
    // last stands standing as the second
    // figures and moving the closing. Passes
    // under FF-1, the last spells standing at
    // the longest spells at every survey series
    // and the closing standing as before.
    // Passes under FF-3, the first spells
    // classing the survey series as the last
    // spells do. Passes under FF-4, the first
    // reigns standing at the last reigns at the
    // survey seats the closing reads. Passes
    // under FF-6, the closing standing as
    // before while its sections move.
    let championship = load_example("survey-championship.json");
    let report = tail_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("tail, 6 series, 4 reign longer, 0 spell longer, 2 matched, 0 no winner")
    );
    assert_eq!(
        closing_counts(report.last().expect("a closing")),
        vec![6, 4, 0, 2, 0]
    );
}

#[test]
fn the_league_championship_closes_at_eight_reign_longer_and_two_matched() {
    // TA-3. The league championship's closing
    // line, asserting eight reign longer, no
    // spell longer, and two matched over its
    // ten series. Fails under FF-1, the longest
    // spells moving the closing. Fails under
    // FF-3, the first spells moving it. Fails
    // under FF-4, the first reigns moving it.
    // Fails under FF-5, the classes standing
    // exchanged over the field. Fails under
    // FF-8, the last stands moving it. Passes
    // under FF-2, the longest reigns standing
    // at the last reigns at the league seats
    // the closing reads. Passes under FF-6, the
    // closing standing as before while its
    // sections move. Passes under FF-7, the
    // closing runs classing the league series
    // as the last reigns do.
    let championship = load_example("league-championship.json");
    let report = tail_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("tail, 10 series, 8 reign longer, 0 spell longer, 2 matched, 0 no winner")
    );
    assert_eq!(
        closing_counts(report.last().expect("a closing")),
        vec![10, 8, 0, 2, 0]
    );
}

#[test]
fn the_championship_example_closes_at_two_reign_longer_with_no_section() {
    // TA-4. The championship example's closing
    // line, asserting two reign longer and no
    // spell longer or matched over its two
    // series, with no section standing for
    // either series. Fails under FF-5 alone,
    // the exchange classing both series spell
    // longer and drawing their sections. Passes
    // under FF-1 and FF-3, the longest and first
    // spells standing at none as the last
    // spells do over both series. Passes under
    // FF-2 and FF-4, the longest and first
    // reigns standing above the last spells of
    // none at both series. Passes under FF-6,
    // the sections standing for the matched
    // series and this championship carrying
    // none. Passes under FF-7, the closing runs
    // leaving both series reign longer. Passes
    // under FF-8, the last stands standing at
    // none as the last spells do at both
    // series, neither loser holding a run of
    // races over the record the closing reads.
    let championship = load_example("championship.json");
    let report = tail_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("tail, 2 series, 2 reign longer, 0 spell longer, 0 matched, 0 no winner")
    );
    let sections: Vec<&String> = report
        .iter()
        .filter(|line| line.starts_with("series ") && !line.starts_with("series winner"))
        .collect();
    assert_eq!(sections.len(), 0);
}

#[test]
fn the_constructed_recovery_championship_closes_at_two_spell_longer() {
    // TA-5. The constructed recovery
    // championship's closing line, asserting
    // two spell longer and no reign longer or
    // matched over its two series, each series
    // standing at a last reign of one and a
    // last spell of five over its one spell.
    // Fails under FF-5, the exchange classing
    // both series reign longer and their
    // sections falling away. Fails under FF-8,
    // the last stands standing as the second
    // figures and the closing moving. Passes
    // under FF-1, the longest spells standing
    // at five as the last spells do over both
    // series. Passes under FF-2, FF-3, FF-4,
    // and FF-7, the substituted figures
    // classing both series spell longer as the
    // last figures do. Passes under FF-6, the
    // closing standing as before.
    let championship = load_example("constructed-recovery.json");
    let report = tail_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("tail, 2 series, 0 reign longer, 2 spell longer, 0 matched, 0 no winner")
    );
    assert_eq!(
        closing_counts(report.last().expect("a closing")),
        vec![2, 0, 2, 0, 0]
    );
}

#[test]
fn the_constructed_tail_championship_closes_at_one_spell_longer_and_one_matched() {
    // TA-6. The constructed tail championship's
    // closing line, asserting one spell longer
    // and one matched over its two series, the
    // pair standing built for this reading.
    // Fails under FF-1, the longest spell of
    // three standing as the first drill's
    // second figure and classing it spell
    // longer. Fails under FF-3, the first spell
    // of one standing as the second drill's
    // second figure and classing it matched.
    // Fails under FF-5, the exchange standing
    // this pair at one reign longer and one
    // matched. Fails under FF-7, the closing
    // runs of two and three standing as the
    // first figures and the pair standing at
    // one reign longer and one spell longer.
    // Passes under FF-2 and FF-4, the longest
    // and first reigns standing at one over
    // both drills as the last reigns do.
    // Passes under FF-6, the closing standing
    // as before. Passes under FF-8, the last
    // stands standing at one and two, the
    // first drill standing matched and the
    // second spell longer as before.
    let championship = load_example("constructed-tail.json");
    let report = tail_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("tail, 2 series, 0 reign longer, 1 spell longer, 1 matched, 0 no winner")
    );
    assert_eq!(
        closing_counts(report.last().expect("a closing")),
        vec![2, 0, 1, 1, 0]
    );
}

#[test]
fn the_first_drill_stands_matched_at_a_last_spell_of_one() {
    // TA-7. The first drill's entry assertion,
    // the roster's narrowest pin beside TA-19.
    // The drill stands matched at a last reign
    // of one and a last spell of one, its
    // longest spell standing at three over
    // spells of one, three, and one, and its
    // closing run standing at two. Fails under
    // FF-1, the longest spell of three standing
    // as the second figure and classing the
    // drill spell longer. Fails under FF-7, the
    // closing run of two standing as the first
    // figure and classing the drill reign
    // longer. Passes under FF-3, the first
    // spell standing at one as the last spell
    // does. Passes under FF-8, the last stand
    // standing at one as the last spell does,
    // the drill's last run of the loser's races
    // standing at one. Passes under FF-2 and
    // FF-4, the longest and first reigns
    // standing at one as the last reign does.
    // Passes under FF-5, matched standing
    // unchanged under the exchange. Passes
    // under FF-6, the entry standing outside
    // the report's sections.
    let championship = load_example("constructed-tail.json");
    let series = &championship.series[0];
    assert_eq!(tail_class(series), Some((TailClass::Matched, 1, 1)));
    assert_eq!(last_spell_of(series), 1);
    assert_eq!(longest_spell_of(series), 3);
}

#[test]
fn the_second_drill_stands_spell_longer_at_a_last_spell_of_three() {
    // TA-8. The second drill's entry assertion,
    // the series standing spell longer at a
    // last reign of one and a last spell of
    // three, its spells standing at one, one,
    // and three. Fails under FF-3, the first
    // spell of one standing as the second
    // figure and classing the drill matched.
    // Fails under FF-5, the exchange classing
    // the drill reign longer. Fails under FF-7,
    // the closing run of three standing as the
    // first figure and the drill standing
    // matched. Fails under FF-8, the last stand
    // of two standing as the second figure, its
    // entry figure parting while its class
    // stands. Passes under FF-1, the longest
    // spell standing at three as the last spell
    // does. Passes under FF-2 and FF-4, the
    // longest and first reigns standing at one
    // as the last reign does. Passes under
    // FF-6, the entry standing outside the
    // report's sections.
    let championship = load_example("constructed-tail.json");
    let series = &championship.series[1];
    assert_eq!(tail_class(series), Some((TailClass::SpellLonger, 1, 3)));
    assert_eq!(last_spell_of(series), 3);
}

#[test]
fn the_constructed_firststrike_drill_two_stands_spell_longer_at_one_and_three() {
    // TA-9. The constructed firststrike drill
    // two, the second series of the constructed
    // firststrike championship, standing spell
    // longer at a last reign of one and a last
    // spell of three over its one spell of
    // three, its changes of hand standing at
    // two under DH-70. Fails under FF-5, the
    // exchange classing the series reign
    // longer. Fails under FF-7, the closing run
    // of three standing as the first figure and
    // the series standing matched. Passes under
    // FF-1, the longest spell standing at three
    // as the last spell does over the one
    // spell. Passes under FF-3, the first spell
    // standing at three as the last spell does.
    // Passes under FF-2 and FF-4, the longest
    // and first reigns standing at one as the
    // last reign does. Passes under FF-8, the
    // last stand standing at three as the last
    // spell does. Passes under FF-6, the entry
    // standing outside the report's sections.
    let championship = load_example("constructed-firststrike.json");
    let series = &championship.series[1];
    assert_eq!(tail_class(series), Some((TailClass::SpellLonger, 1, 3)));
    assert_eq!(last_reign_of(series), 1);
    assert_eq!(last_spell_of(series), 3);
}

#[test]
fn the_last_spell_never_stands_above_the_longest_spell() {
    // TA-10. The theorem census of DH-74 over
    // the enrolled sixty seven series, the last
    // spell of a series standing above the
    // longest spell of the series at no series,
    // each last run standing among the runs the
    // longest is taken over, every series of
    // the enrolment holding a winner. Fails
    // under FF-8 alone, the last stand of the
    // loser standing above the longest spell at
    // enrolled seats where the last spell never
    // does. Passes under FF-1, the figure
    // standing at the longest spell itself and
    // never above it. Passes under FF-3, the
    // first spell standing at or below the
    // longest spell at every series. Passes
    // under FF-2, FF-4, FF-5, FF-6, and FF-7,
    // the census reading the second figure
    // alone at the entry, where the first
    // figures, the classes, and the sections
    // stand outside the count.
    let mut decided = 0;
    let mut violations = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, _, last_spell) = match tail_class(series) {
                Some(triple) => triple,
                None => continue,
            };
            decided += 1;
            if last_spell > longest_spell_of(series) {
                violations += 1;
            }
        }
    }
    assert_eq!(decided, 67);
    assert_eq!(violations, 0);
}

#[test]
fn the_last_spell_stands_at_the_longest_spell_at_sixty_two_series() {
    // TA-11. The shadow census of DH-74 over
    // the enrolled sixty seven series, the last
    // spell of a series standing at the longest
    // spell of the series at exactly sixty two
    // series, the reading's last run standing
    // at the family's greatest run of behind
    // standings at five seats short of the
    // whole field. Fails under FF-1, the figure
    // standing at the longest spell at every
    // series and the count standing at sixty
    // seven. Fails under FF-3, the first spells
    // standing at the longest spells at a
    // smaller count. Fails under FF-8, the last
    // stands standing at the longest spells at
    // a different count. Passes under FF-2,
    // FF-4, FF-5, FF-6, and FF-7, the census
    // reading the second figure alone against
    // its own last figure's greatest run, where
    // the first figures, the classes, and the
    // sections stand outside the count.
    let mut decided = 0;
    let mut at_longest = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, _, last_spell) = match tail_class(series) {
                Some(triple) => triple,
                None => continue,
            };
            decided += 1;
            if last_spell == longest_spell_of(series) {
                at_longest += 1;
            }
        }
    }
    assert_eq!(decided, 67);
    assert_eq!(at_longest, 62);
}

#[test]
fn a_series_at_a_wins_target_of_one_stands_reign_longer() {
    // TA-12. A fixture series standing at a
    // wins target of one, the winner taking its
    // only race, the reading's class standing
    // reign longer with a last reign of one and
    // a last spell of none. Fails under FF-5
    // alone, the exchange classing the series
    // spell longer. Passes under FF-1 and FF-3,
    // the longest and first spells standing at
    // none as the last spell does. Passes under
    // FF-2 and FF-4, the longest and first
    // reigns standing at one as the last reign
    // does. Passes under FF-7, the closing run
    // standing at one as the last reign does.
    // Passes under FF-8, the last stand standing
    // at none as the last spell does. Passes
    // under FF-6, the entry standing outside the
    // report's sections.
    let dir = temp_dir("tail-ta12");
    let series = series_json(
        "the tail target one fixture",
        vec![ALPHA, BETA],
        1,
        vec![clear_win("the tail target one fixture", ALPHA, BETA)],
    );
    write_json(&dir, "fixture-series.json", &series);
    write_json(
        &dir,
        "championship.json",
        &championship_json(vec!["fixture-series.json"]),
    );
    let championship = load(&dir);
    assert_eq!(
        tail_class(&championship.series[0]),
        Some((TailClass::ReignLonger, 1, 0))
    );
}

#[test]
fn a_series_holding_no_winner_takes_no_class_and_no_section() {
    // TA-13. A fixture series standing at a
    // wins target of two holding no winner, the
    // winner taking its only race and the
    // series standing open, the reading
    // returning no class and no figures and the
    // report carrying no section for it, the
    // closing counting the series under no
    // winner alone. Falls under no variant. The
    // class is taken over the decided record
    // and an undecided series holds no decided
    // record to class, so every mutation of the
    // reading leaves its absence standing.
    let dir = temp_dir("tail-ta13");
    let series = series_json(
        "the tail open fixture",
        vec![ALPHA, BETA],
        2,
        vec![clear_win("the tail open fixture", ALPHA, BETA)],
    );
    write_json(&dir, "fixture-series.json", &series);
    write_json(
        &dir,
        "championship.json",
        &championship_json(vec!["fixture-series.json"]),
    );
    let championship = load(&dir);
    assert_eq!(tail_class(&championship.series[0]), None);
    assert_eq!(
        tail_report(&championship),
        vec![
            "tail".to_string(),
            "tail, 1 series, 0 reign longer, 0 spell longer, 0 matched, 1 no winner".to_string(),
        ]
    );
}

#[test]
fn a_championship_holding_no_series_stands_refused_at_the_load() {
    // TA-14. The empty report form pin. A
    // championship holding no series at all
    // stands refused at the load, the empty
    // series list drawing the one defect
    // field series holds no series files, so
    // no championship stands for a report to
    // stand over. The pin falls under no
    // mutation of the eight, no series
    // standing in the championship for any
    // mutation to read, and it stands in the
    // roster so the load boundary of DH-75
    // stands pinned in words.
    let dir = temp_dir("tail-ta14");
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
    // TA-15. The constructed pair's section pin.
    // The second drill stands alone under the
    // spell longer sections, its race lines
    // standing at the record places entire, its
    // figure line standing at a last reign of
    // one and a last spell of three, and the
    // first drill drawing no section, the whole
    // report standing byte equal to the golden
    // the family keeps. Fails under FF-1, the
    // first drill standing spell longer under
    // the longest spell of three and drawing a
    // section beside the second's. Fails under
    // FF-3, the first spell of one classing the
    // second drill matched and its section
    // falling away. Fails under FF-5, the
    // exchange classing the second drill reign
    // longer and its section falling away.
    // Fails under FF-6, the sections standing
    // for the matched first drill instead.
    // Fails under FF-7, the closing runs of two
    // and three classing the first drill reign
    // longer and the second matched, the
    // sections falling away whole. Fails under
    // FF-8, the second drill's figure line
    // standing at a last spell of two, the
    // report parting from the golden while its
    // section stands. Passes under FF-2 and
    // FF-4, the longest and first reigns
    // standing at one over both drills as the
    // last reigns do and the sections standing
    // as before.
    let championship = load_example("constructed-tail.json");
    let report = tail_report(&championship);
    let expected: Vec<String> = vec![
        "tail",
        "series 2, the constructed tail drill two",
        "tail-ash against tail-birch, first to 5 wins",
        "race 1, tail drill two race one, tail-birch wins",
        "race 2, tail drill two race two, tail-ash wins",
        "race 3, tail drill two race three, tail-birch wins",
        "race 4, tail drill two race four, tail-ash wins",
        "race 5, tail drill two race five, tail-birch wins",
        "race 6, tail drill two race six, tail-birch wins",
        "race 7, tail drill two race seven, tail-ash wins",
        "race 8, tail drill two race eight, tail-ash wins",
        "race 9, tail drill two race nine, tail-ash wins",
        "last reign 1, last spell 3 over the decided record",
        "series winner, tail-ash, tail-ash 5 race wins, tail-birch 4 race wins, 0 no results",
        "tail, 2 series, 0 reign longer, 1 spell longer, 1 matched, 0 no winner",
    ]
    .into_iter()
    .map(|line| line.to_string())
    .collect();
    assert_eq!(report, expected);
    assert_eq!(
        report.join("\n") + "\n",
        golden("constructed-tail-tail.txt")
    );
}

#[test]
fn the_last_reign_stands_at_one_at_thirty_nine_series() {
    // TA-16. The last reign census of DH-74 over
    // the enrolled sixty seven series, the last
    // reign standing at one at exactly thirty
    // nine series. Fails under FF-2, the longest
    // reigns standing at one at a different
    // count. Fails under FF-4, the first reigns
    // standing at one at a different count.
    // Fails under FF-7, the closing runs standing
    // at one at a different count. Passes under
    // FF-1, FF-3, FF-5, FF-6, and FF-8, the
    // census reading the first figure alone,
    // where the second figures, the classes, and
    // the sections stand outside the count.
    let mut decided = 0;
    let mut at_one = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, last_reign, _) = match tail_class(series) {
                Some(triple) => triple,
                None => continue,
            };
            decided += 1;
            if last_reign == 1 {
                at_one += 1;
            }
        }
    }
    assert_eq!(decided, 67);
    assert_eq!(at_one, 39);
}

#[test]
fn the_enrolled_championships_close_at_thirty_three_reign_longer_sixteen_spell_longer_and_eighteen_matched(
) {
    // TA-17. The aggregate closing pin of DH-75
    // over the enrolled twenty nine
    // championships, the closings summed at
    // sixty seven series, thirty three reign
    // longer, sixteen spell longer, eighteen
    // matched, and no series holding no winner.
    // Fails under FF-1, the longest spells moving
    // the sums. Fails under FF-2, the longest
    // reigns moving them. Fails under FF-3, the
    // first spells moving them. Fails under FF-4,
    // the first reigns moving them. Fails under
    // FF-5, the classes standing exchanged over
    // the field. Fails under FF-7, the closing
    // runs moving the sums. Fails under FF-8, the
    // last stands moving them. Passes under FF-6
    // alone among the variants, the closing
    // standing as before at every championship
    // while its sections move.
    let mut totals = vec![0; 5];
    for championship in enrolled() {
        let report = tail_report(&championship);
        let counts = closing_counts(report.last().expect("a closing"));
        for (index, count) in counts.iter().enumerate() {
            totals[index] += count;
        }
    }
    assert_eq!(totals, vec![67, 33, 16, 18, 0]);
}

#[test]
fn the_second_drills_section_spans_the_record_places_one_to_nine() {
    // TA-18. The span pin over the constructed
    // pair's report, the second drill's race
    // lines standing at the record places one to
    // nine entire, the drill holding five race
    // wins over nine decided races and its
    // figure line standing in the report. Fails
    // under FF-3, the first spell of one classing
    // the drill matched and its section falling
    // away. Fails under FF-5, the exchange
    // classing the drill reign longer and its
    // section falling away. Fails under FF-6, the
    // sections standing for the matched first
    // drill instead. Fails under FF-7, the closing
    // run of three classing the drill matched and
    // its section falling away. Fails under FF-8,
    // the figure line standing at a last spell of
    // two. Passes under FF-1, the second drill's
    // section standing with its figure line at
    // three, the longest spell standing at three
    // as the last spell does. Passes under FF-2
    // and FF-4, the longest and first reigns
    // standing at one over both drills as the
    // last reigns do and the section standing as
    // before.
    let championship = load_example("constructed-tail.json");
    let report = tail_report(&championship);
    let joined = report.join("\n");
    for place in 1..=9 {
        assert!(
            joined.contains(&format!("race {place},")),
            "the section carries race place {place}"
        );
    }
    assert!(joined.contains("last reign 1, last spell 3 over the decided record"));
    assert!(joined.contains("tail-ash 5 race wins, tail-birch 4 race wins"));
}

#[test]
fn the_last_spell_stands_at_none_at_twenty_six_series() {
    // TA-19. The nought census of DH-74 over the
    // enrolled sixty seven series, the last spell
    // standing at none at exactly twenty six
    // series, the roster's narrowest pin beside
    // TA-7. Fails under FF-8 alone, the last
    // stands standing at none at a different
    // count, the loser holding a run of races at
    // seats the last spell never reaches. Passes
    // under FF-1, the longest spells standing at
    // none at the same twenty six seats the last
    // spells do. Passes under FF-3, the first
    // spells standing at none at the same seats.
    // Passes under FF-2, FF-4, FF-5, FF-6, and
    // FF-7, the census reading the second figure
    // alone at the entry, where the first figures,
    // the classes, and the sections stand outside
    // the count.
    let mut decided = 0;
    let mut at_none = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, _, last_spell) = match tail_class(series) {
                Some(triple) => triple,
                None => continue,
            };
            decided += 1;
            if last_spell == 0 {
                at_none += 1;
            }
        }
    }
    assert_eq!(decided, 67);
    assert_eq!(at_none, 26);
}

#[test]
fn the_last_reign_stands_at_the_last_spell_at_eighteen_series() {
    // TA-20. The matched census of DH-74 over the
    // enrolled sixty seven series, the reading's
    // class standing matched at exactly eighteen
    // series. Fails under FF-1, the longest spells
    // moving seats out of and into the matched
    // class. Fails under FF-3, the first spells
    // moving them. Fails under FF-4, the first
    // reigns moving them. Fails under FF-7, the
    // closing runs moving them. Fails under FF-8,
    // the last stands moving them. Passes under
    // FF-2, the longest reigns standing at the
    // last spells at the same eighteen seats the
    // last reigns do. Passes under FF-5, matched
    // standing unchanged under the exchange at
    // every seat, the count standing as before.
    // Passes under FF-6, the census reading the
    // entry alone, where the sections stand
    // outside the count.
    let mut decided = 0;
    let mut matched = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (class, _, _) = match tail_class(series) {
                Some(triple) => triple,
                None => continue,
            };
            decided += 1;
            if class == TailClass::Matched {
                matched += 1;
            }
        }
    }
    assert_eq!(decided, 67);
    assert_eq!(matched, 18);
}
