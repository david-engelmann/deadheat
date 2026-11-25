//! Pins for DH-72 and DH-73 in the Round 37
//! section of CONTRACT.md. The discrimination
//! record of this file stands pin by pin, and
//! no blanket claim stands over it. The outset
//! reading, its library function, and its
//! command do not stand on the pre round
//! tree, so every pin in this file that names
//! the outset reading is absent there at
//! compile time. That absence is recorded
//! once here and is not a discrimination
//! claim. The discrimination each pin carries
//! is against the stated mutation of DH-72
//! beside it. Eight mutations are named for
//! the whole file. EE-1, the longest spell of
//! DH-50 standing as the second figure in the
//! first spell's place. EE-2, the longest
//! reign of DH-52 standing as the first figure
//! in the first reign's place. EE-3, the last
//! spell standing as the second figure. EE-4,
//! the last reign standing as the first
//! figure. EE-5, the classes exchanged, a
//! series whose first reign stands above its
//! first spell standing spell longer and a
//! series whose first spell stands above its
//! first reign standing reign longer, matched
//! standing unchanged. EE-6, the report
//! carrying its sections for the matched
//! series instead of the spell longer, the
//! closing standing as before. EE-7, the first
//! strike of DH-64 standing as the first
//! figure in the first reign's place. EE-8,
//! the first stand of DH-62 standing as the
//! second figure in the first spell's place.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{load_championship_file, Championship};
use deadheat::json::JsonValue;
use deadheat::outset::{outset_class, outset_report, OutsetClass};
use deadheat::races::decide_winner;
use deadheat::series::Series;
use num_bigint::BigInt;
use shared::*;

const ALPHA: &str = "alpha-1";
const BETA: &str = "beta-1";

const ENROLLED: [&str; 28] = [
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

fn first_reign_of(series: &Series) -> u64 {
    let winner = winner_of(series).expect("a decided series");
    first_run(&ahead_flags(&decided_record(series), winner))
}

fn first_spell_of(series: &Series) -> u64 {
    let winner = winner_of(series).expect("a decided series");
    first_run(&behind_flags(&decided_record(series), winner))
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
fn the_classification_reads_the_first_reign_against_the_first_spell() {
    // OS-1. The outset characterization pin
    // over the constructed pair. The first
    // drill seats birch as champion at a wins
    // target of five, winning five races to
    // four, its standings running behind,
    // level, behind, behind, behind, level,
    // behind, level, ahead, its first reign at
    // one and its first spell at one, matched.
    // The second drill seats ash as champion
    // at the same target and split, its
    // standings running behind, behind,
    // behind, level, behind, level, behind,
    // level, ahead, its first reign at one and
    // its first spell at three, spell longer.
    // The pair shares the wins split, the
    // margin, the front count at one, the
    // behind count at five, the level
    // standings at three, the first reign, the
    // longest reign at one, the last reign at
    // one, the longest spell at three, and the
    // last spell at one, and parts at the
    // first spell alone within that family.
    // It asserts both drills' classes and
    // figures at the module's entry and the
    // neighbour figures beside them. Fails
    // under EE-1, the longest spell of three
    // standing as the second drill's second
    // figure in the first drill's place and
    // classing the first drill spell longer.
    // Fails under EE-3, the last spell of one
    // standing as the second drill's second
    // figure and classing it matched. Fails
    // under EE-5, the exchange classing the
    // second drill reign longer. Fails under
    // EE-7, the first strike of two standing
    // as the second drill's first figure.
    // Fails under EE-8, the first stand of two
    // standing as the second drill's second
    // figure. Passes under EE-2, the longest
    // reign standing at one over both drills
    // as the first reign does. Passes under
    // EE-4, the last reign standing at one
    // over both drills. Passes under EE-6, the
    // entry standing outside the report's
    // sections.
    let championship = load_example("constructed-outset.json");
    let first = &championship.series[0];
    let second = &championship.series[1];
    assert_eq!(outset_class(first), Some((OutsetClass::Matched, 1, 1)));
    assert_eq!(outset_class(second), Some((OutsetClass::SpellLonger, 1, 3)));
    assert_eq!(first_reign_of(first), 1);
    assert_eq!(first_spell_of(first), 1);
    assert_eq!(longest_spell_of(first), 3);
    assert_eq!(first_reign_of(second), 1);
    assert_eq!(first_spell_of(second), 3);
    assert_eq!(longest_spell_of(second), 3);
    assert_eq!(front_count_of(first), 1);
    assert_eq!(behind_count_of(first), 5);
    assert_eq!(front_count_of(second), 1);
    assert_eq!(behind_count_of(second), 5);
}

#[test]
fn the_survey_championship_closes_at_four_reign_longer_and_two_matched() {
    // OS-2. The survey championship's closing
    // line, asserting four reign longer, no
    // spell longer, and two matched over its
    // six series. Fails under EE-2, the
    // longest reigns moving the closing.
    // Fails under EE-5, the classes standing
    // exchanged over the field. Fails under
    // EE-8, the first stands standing as the
    // second figures and moving the closing.
    // Passes under EE-1, the first spells
    // standing at the longest spells at every
    // survey series and the closing standing
    // as before. Passes under EE-3, the last
    // spells classing the survey series as
    // the first spells do. Passes under EE-4,
    // the last reigns standing at the first
    // reigns at the survey seats the closing
    // reads. Passes under EE-6, the closing
    // standing as before while its sections
    // move. Passes under EE-7, the first
    // strikes standing at the first reigns at
    // the survey seats the closing reads.
    let championship = load_example("survey-championship.json");
    let report = outset_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("outset, 6 series, 4 reign longer, 0 spell longer, 2 matched, 0 no winner")
    );
    assert_eq!(
        closing_counts(report.last().expect("a closing")),
        vec![6, 4, 0, 2, 0]
    );
}

#[test]
fn the_league_championship_closes_at_six_reign_longer_one_spell_longer_and_three_matched() {
    // OS-3. The league championship's closing
    // line, asserting six reign longer, one
    // spell longer, and three matched over
    // its ten series. Fails under EE-2, EE-3,
    // EE-4, EE-5, EE-7, and EE-8, the league
    // fields moving under each. Passes under
    // EE-1, the first spells standing at the
    // longest spells at every league series
    // and the closing standing as before.
    // Passes under EE-6, the closing standing
    // as before while its sections move.
    let championship = load_example("league-championship.json");
    let report = outset_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("outset, 10 series, 6 reign longer, 1 spell longer, 3 matched, 0 no winner")
    );
    assert_eq!(
        closing_counts(report.last().expect("a closing")),
        vec![10, 6, 1, 3, 0]
    );
}

#[test]
fn the_championship_example_closes_at_two_reign_longer_with_no_section() {
    // OS-4. The championship example's
    // closing line, asserting two reign
    // longer and no spell longer or matched
    // over its two series, with no section
    // standing for either series. Fails under
    // EE-5 alone, the exchange classing both
    // series spell longer and drawing their
    // sections. Passes under EE-1, the first
    // spells standing at none as the longest
    // spells do over both series. Passes
    // under EE-2 and EE-4, the longest and
    // last reigns standing above the first
    // spells of none at both series. Passes
    // under EE-3, the last spells standing at
    // none at both series. Passes under EE-6,
    // the sections standing for the matched
    // series and this championship carrying
    // none. Passes under EE-7 and EE-8, the
    // first strikes and first stands leaving
    // both series reign longer.
    let championship = load_example("championship.json");
    let report = outset_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("outset, 2 series, 2 reign longer, 0 spell longer, 0 matched, 0 no winner")
    );
    let sections: Vec<&String> = report
        .iter()
        .filter(|line| line.starts_with("series ") && !line.starts_with("series winner"))
        .collect();
    assert_eq!(sections.len(), 0);
}

#[test]
fn the_constructed_recovery_championship_closes_at_two_spell_longer() {
    // OS-5. The constructed recovery
    // championship's closing line, asserting
    // two spell longer and no reign longer or
    // matched over its two series. Fails
    // under EE-5 alone, the exchange classing
    // both series reign longer and their
    // sections falling away. Passes under
    // EE-1, the first spells standing at the
    // longest spells over both series, the
    // first series' one spell of five
    // standing as both. Passes under EE-2,
    // EE-3, EE-4, EE-7, and EE-8, the
    // substituted figures classing both
    // series spell longer as the first
    // figures do. Passes under EE-6, the
    // closing standing as before.
    let championship = load_example("constructed-recovery.json");
    let report = outset_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("outset, 2 series, 0 reign longer, 2 spell longer, 0 matched, 0 no winner")
    );
    assert_eq!(
        closing_counts(report.last().expect("a closing")),
        vec![2, 0, 2, 0, 0]
    );
}

#[test]
fn the_constructed_outset_championship_closes_at_one_spell_longer_and_one_matched() {
    // OS-6. The constructed outset
    // championship's closing line, asserting
    // one spell longer and one matched over
    // its two series, the pair standing built
    // for this reading. Fails under EE-1, the
    // longest spell of three standing as the
    // first drill's second figure and
    // classing it spell longer. Fails under
    // EE-3, the last spell of one standing as
    // the second drill's second figure and
    // classing it matched. Fails under EE-5,
    // the exchange standing this pair at one
    // reign longer and one matched. Passes
    // under EE-2 and EE-4, the longest and
    // last reigns standing at one over both
    // drills as the first reigns do. Passes
    // under EE-6, the closing standing as
    // before. Passes under EE-7, the first
    // strikes standing at one and two and the
    // closing standing as before, the second
    // drill standing spell longer at a first
    // figure of two against three. Passes
    // under EE-8, the first stands standing
    // at one and two and the closing standing
    // as before, the second drill standing
    // spell longer at a second figure of two
    // against one.
    let championship = load_example("constructed-outset.json");
    let report = outset_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("outset, 2 series, 0 reign longer, 1 spell longer, 1 matched, 0 no winner")
    );
    assert_eq!(
        closing_counts(report.last().expect("a closing")),
        vec![2, 0, 1, 1, 0]
    );
}

#[test]
fn the_first_drill_stands_matched_at_a_first_spell_of_one() {
    // OS-7. The first drill's entry
    // assertion, the roster's narrowest pin.
    // The drill stands matched at a first
    // reign of one and a first spell of one,
    // its longest spell standing at three
    // over spells of one, three, and one, the
    // one series of the enrolment whose first
    // spell parts from its longest spell.
    // Fails under EE-1 alone, the longest
    // spell of three standing as the second
    // figure and classing the drill spell
    // longer. Passes under EE-3, the last
    // spell standing at one as the first
    // spell does. Passes under EE-8, the
    // first stand standing at one as the
    // first spell does. Passes under EE-2
    // and EE-4, the longest and last reigns
    // standing at one as the first reign
    // does. Passes under EE-5, matched
    // standing unchanged under the exchange.
    // Passes under EE-6, the entry standing
    // outside the report's sections. Passes
    // under EE-7, the first strike standing
    // at one as the first reign does.
    let championship = load_example("constructed-outset.json");
    let series = &championship.series[0];
    assert_eq!(outset_class(series), Some((OutsetClass::Matched, 1, 1)));
    assert_eq!(first_spell_of(series), 1);
    assert_eq!(longest_spell_of(series), 3);
}

#[test]
fn the_second_drill_stands_spell_longer_at_a_first_spell_of_three() {
    // OS-8. The second drill's entry
    // assertion, the series standing spell
    // longer at a first reign of one and a
    // first spell of three, its spells
    // standing at three, one, and one. Fails
    // under EE-3, the last spell of one
    // standing as the second figure and
    // classing the drill matched. Fails under
    // EE-5, the exchange classing the drill
    // reign longer. Fails under EE-7, the
    // first strike of two standing as the
    // first figure. Fails under EE-8, the
    // first stand of two standing as the
    // second figure. Passes under EE-1, the
    // longest spell standing at three as the
    // first spell does. Passes under EE-2
    // and EE-4, the longest and last reigns
    // standing at one as the first reign
    // does. Passes under EE-6, the entry
    // standing outside the report's sections.
    let championship = load_example("constructed-outset.json");
    let series = &championship.series[1];
    assert_eq!(outset_class(series), Some((OutsetClass::SpellLonger, 1, 3)));
    assert_eq!(first_spell_of(series), 3);
}

#[test]
fn the_bolt_ember_series_stands_spell_longer_at_two_and_three() {
    // OS-9. The bolt ember series of the
    // league championship, the seventh series
    // of that championship, standing spell
    // longer at a first reign of two and a
    // first spell of three over spells of
    // three and one, broken under DH-50.
    // Fails under EE-3, the last spell of one
    // standing as the second figure and
    // classing the series reign longer.
    // Fails under EE-5, the exchange classing
    // the series reign longer. Fails under
    // EE-8, the first stand standing as the
    // second figure in the first spell's
    // place. Passes under EE-1, the longest
    // spell standing at three as the first
    // spell does. Passes under EE-2 and
    // EE-4, the longest and last reigns
    // standing at the first reign's two at
    // this seat. Passes under EE-6, the entry
    // standing outside the report's sections.
    // Passes under EE-7, the first strike
    // standing at the first reign's two at
    // this seat.
    let championship = load_example("league-championship.json");
    let series = &championship.series[6];
    assert_eq!(outset_class(series), Some((OutsetClass::SpellLonger, 2, 3)));
    assert_eq!(first_reign_of(series), 2);
    assert_eq!(first_spell_of(series), 3);
}

#[test]
fn the_first_spell_never_stands_above_the_longest_spell() {
    // OS-10. The theorem census of DH-72
    // over the enrolled sixty five series,
    // the first spell of a series standing
    // above the longest spell of the series
    // at no series, each first run standing
    // among the runs the longest is taken
    // over, every series of the enrolment
    // holding a winner. Fails under EE-8
    // alone, the first stand of the loser
    // standing above the longest spell at
    // enrolled seats where the first spell
    // never does. Passes under EE-1, the
    // figure standing at the longest spell
    // itself and never above it. Passes
    // under EE-3, the last spell standing at
    // or below the longest spell at every
    // series. Passes under EE-2, EE-4, EE-5,
    // EE-6, and EE-7, the census reading the
    // second figure alone at the entry,
    // where the first figures, the classes,
    // and the sections stand outside the
    // count.
    let mut decided = 0;
    let mut violations = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, _, first_spell) = match outset_class(series) {
                Some(triple) => triple,
                None => continue,
            };
            decided += 1;
            if first_spell > longest_spell_of(series) {
                violations += 1;
            }
        }
    }
    assert_eq!(decided, 65);
    assert_eq!(violations, 0);
}

#[test]
fn the_first_spell_stands_at_the_longest_spell_at_sixty_four_series() {
    // OS-11. The shadow census of DH-72 over
    // the enrolled sixty five series, the
    // first spell standing at the longest
    // spell at exactly sixty four series, the
    // standing sixty three entire and the
    // second drill beside them, the first
    // drill standing as the enrolment's one
    // separation of the two figures. Fails
    // under EE-1, the figure standing at the
    // longest spell at every series and the
    // count rising to the whole enrolment.
    // Fails under EE-3, the last spells
    // standing at the longest spells at
    // fewer series. Fails under EE-8, the
    // first stands standing at the longest
    // spells at their own seats. Passes
    // under EE-2, EE-4, EE-5, EE-6, and
    // EE-7, the census reading the second
    // figure alone at the entry.
    let mut whole = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, _, first_spell) = match outset_class(series) {
                Some(triple) => triple,
                None => continue,
            };
            if first_spell == longest_spell_of(series) {
                whole += 1;
            }
        }
    }
    assert_eq!(whole, 64);
}

#[test]
fn a_series_at_a_wins_target_of_one_stands_reign_longer() {
    // OS-12. The wins target one fixture, a
    // series of one decided race at a target
    // of one, its winner winning the one race
    // the record holds before the decision,
    // standing reign longer at a first reign
    // of one and a first spell of none, no
    // standing of the record standing level
    // or behind. Fails under EE-5 alone, the
    // exchange classing the series spell
    // longer. Passes under EE-1 and EE-3, the
    // longest and last spells standing at
    // none as the first spell does. Passes
    // under EE-2 and EE-4, the longest and
    // last reigns standing at one as the
    // first reign does. Passes under EE-6,
    // no section standing for a series in no
    // spell longer or matched class under
    // either form. Passes under EE-7, the
    // first strike standing at one as the
    // first reign does. Passes under EE-8,
    // the first stand standing at none, the
    // loser holding no race of the record.
    let dir = temp_dir("outset-os12");
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
        outset_class(&championship.series[0]),
        Some((OutsetClass::ReignLonger, 1, 0))
    );
}

#[test]
fn a_series_holding_no_winner_takes_no_class_and_no_section() {
    // OS-13. The roster's characterization
    // pin of no posture. A series holding no
    // winner, its three races standing at two
    // wins against one at a target of three,
    // taking no class at the module's entry
    // and no section in the report, the
    // report over its championship standing
    // in the empty form with the series
    // counted in the no winner place alone.
    // The pin falls under no mutation of the
    // eight, each mutation reading a decided
    // record the series does not hold, and it
    // stands in the roster so the no winner
    // place of DH-72 and DH-73 stands pinned
    // in words.
    let dir = temp_dir("outset-os13");
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
    assert_eq!(outset_class(&championship.series[0]), None);
    assert_eq!(
        outset_report(&championship),
        vec![
            "outset".to_string(),
            "outset, 1 series, 0 reign longer, 0 spell longer, 0 matched, 1 no winner".to_string(),
        ]
    );
}

#[test]
fn a_championship_holding_no_series_stands_refused_at_the_load() {
    // OS-14. The empty report form pin. A
    // championship holding no series at all
    // stands refused at the load, the empty
    // series list drawing the one defect
    // field series holds no series files, so
    // no championship stands for a report to
    // stand over. The pin falls under no
    // mutation of the eight, no series
    // standing in the championship for any
    // mutation to read, and it stands in the
    // roster so the load boundary of DH-73
    // stands pinned in words.
    let dir = temp_dir("outset-os14");
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
    // OS-15. The outset pair section pin,
    // the roster's strongest pin beside
    // OS-18. The section form over the outset
    // championship, the second drill's
    // section standing in full with its
    // series line, its models and target
    // line, its race lines over the decided
    // races at the record places one to nine
    // entire, its figure line at a first
    // reign of one and a first spell of
    // three, and its series winner line, the
    // first drill standing in no section, the
    // whole report standing line for line as
    // the golden stands. Fails under EE-1,
    // the longest spell of three standing as
    // the first drill's second figure,
    // classing it spell longer and drawing
    // its section beside the second drill's.
    // Fails under EE-3, the last spell of one
    // standing as the second drill's second
    // figure, classing it matched and its
    // section falling away. Fails under EE-5,
    // the sections moving between the drills
    // under the exchange. Fails under EE-6,
    // the sections standing for the matched
    // series, the first drill drawing the
    // section and the second losing it.
    // Fails under EE-7, the first strike of
    // two standing in the second drill's
    // figure line. Fails under EE-8, the
    // first stand of two standing in the
    // second drill's figure line. Passes
    // under EE-2 and EE-4, the longest and
    // last reigns standing at one over both
    // drills and the report standing word
    // for word.
    let championship = load_example("constructed-outset.json");
    let expected: Vec<String> = [
        "outset",
        "series 2, the constructed outset drill two",
        "outset-ash against outset-birch, first to 5 wins",
        "race 1, outset drill two race one, outset-birch wins",
        "race 2, outset drill two race two, outset-birch wins",
        "race 3, outset drill two race three, outset-ash wins",
        "race 4, outset drill two race four, outset-ash wins",
        "race 5, outset drill two race five, outset-birch wins",
        "race 6, outset drill two race six, outset-ash wins",
        "race 7, outset drill two race seven, outset-birch wins",
        "race 8, outset drill two race eight, outset-ash wins",
        "race 9, outset drill two race nine, outset-ash wins",
        "first reign 1, first spell 3 over the decided record",
        "series winner, outset-ash, outset-ash 5 race wins, outset-birch 4 race wins, 0 no results",
        "outset, 2 series, 0 reign longer, 1 spell longer, 1 matched, 0 no winner",
    ]
    .iter()
    .map(|line| line.to_string())
    .collect();
    assert_eq!(outset_report(&championship), expected);
    assert_eq!(
        outset_report(&championship).join("\n") + "\n",
        golden("constructed-outset-outset.txt")
    );
}

#[test]
fn the_first_reign_stands_at_one_at_forty_three_series() {
    // OS-16. The first reign census of DH-72
    // over the enrolled sixty five series,
    // the first reign standing at one at
    // exactly forty three series, the winner
    // first standing ahead by a single
    // standing at those series before any
    // longer run or any level or behind
    // standing parts the run. Fails under
    // EE-2, the longest reigns standing at
    // one at fewer series. Fails under EE-4,
    // the last reigns standing at one at
    // their own seats. Fails under EE-7, the
    // first strikes standing at one at their
    // own seats. Passes under EE-1, EE-3,
    // and EE-8, the census reading the first
    // figure alone at the entry, where the
    // second figures stand outside the
    // count. Passes under EE-5 and EE-6, the
    // classes and the sections standing
    // outside the count of the first figure.
    let mut at_one = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, first_reign, _) = match outset_class(series) {
                Some(triple) => triple,
                None => continue,
            };
            if first_reign == 1 {
                at_one += 1;
            }
        }
    }
    assert_eq!(at_one, 43);
}

#[test]
fn the_enrolled_championships_close_at_thirty_one_reign_longer_seventeen_spell_longer_and_seventeen_matched(
) {
    // OS-17. The aggregate closing over the
    // enrolled sixty five series of the
    // twenty eight championships, asserting
    // thirty one reign longer, seventeen
    // spell longer, seventeen matched, and
    // none holding no winner, summed over
    // the closing line of each championship's
    // report. Fails under EE-1, EE-2, EE-3,
    // EE-4, EE-5, EE-7, and EE-8, the fields
    // of the enrolment moving under each.
    // Passes under EE-6, the closings
    // standing as before while their
    // sections move.
    let mut totals = vec![0u64; 5];
    for championship in enrolled() {
        let report = outset_report(&championship);
        let counts = closing_counts(report.last().expect("a closing"));
        for (index, count) in counts.iter().enumerate() {
            totals[index] += count;
        }
    }
    assert_eq!(totals, vec![65, 31, 17, 17, 0]);
}

#[test]
fn the_second_drills_section_spans_the_record_places_one_to_nine() {
    // OS-18. The span pin on the second
    // drill, the roster's strongest pin
    // beside OS-15. Its section's race lines
    // stand numbered by the counting walk's
    // place among the decided races at the
    // record places one to nine entire,
    // every race of its record standing
    // decided and no place standing skipped
    // inside the span, and its figure line
    // stands at a first reign of one and a
    // first spell of three. Passes under
    // EE-1, the first drill classing spell
    // longer and drawing its own section,
    // the substitution moving nothing this
    // pin reads, its race places and its
    // figure line standing as before.
    // Fails under EE-3, the drill
    // classing matched at its last spell of
    // one and its section falling away.
    // Fails under EE-5, the drill classing
    // reign longer under the exchange and
    // its section falling away. Fails under
    // EE-6, the sections standing for the
    // matched series and the second drill
    // losing its section to the first.
    // Fails under EE-7, the figure line
    // standing at a first strike of two.
    // Fails under EE-8, the figure line
    // standing at a first stand of two.
    // Passes under EE-2 and EE-4, the
    // section standing as before at reigns
    // of one under either substitution.
    let championship = load_example("constructed-outset.json");
    let report = outset_report(&championship);
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
    assert_eq!(places, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert!(report.contains(&"first reign 1, first spell 3 over the decided record".to_string()));
}

#[test]
fn the_first_spell_stands_at_none_at_twenty_six_series() {
    // OS-19. The nought census of DH-72 over
    // the enrolled sixty five series, the
    // first spell standing at none at
    // exactly twenty six series, the never
    // behind seats of the enrolment, the
    // winner standing ahead or level at
    // every standing of those series. Fails
    // under EE-8 alone, the first stands
    // standing at none at fewer series, a
    // series whose loser first won a race
    // before the winner's first standing
    // ahead standing among the parted seats.
    // Passes under EE-1, the longest spells
    // standing at none exactly at the series
    // whose first spells stand at none.
    // Passes under EE-3, the last spells
    // standing at none at the same twenty
    // six series over this enrolment.
    // Passes under EE-2, EE-4, EE-5, EE-6,
    // and EE-7, the census reading the
    // second figure alone at the entry.
    let mut none = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, _, first_spell) = match outset_class(series) {
                Some(triple) => triple,
                None => continue,
            };
            if first_spell == 0 {
                none += 1;
            }
        }
    }
    assert_eq!(none, 26);
}

#[test]
fn the_first_reign_stands_at_the_first_spell_at_seventeen_series() {
    // OS-20. The matched census of DH-72
    // over the enrolled sixty five series,
    // the first reign standing at the first
    // spell at exactly seventeen series, the
    // matched class of the reading itself.
    // Fails under EE-1, the longest spells
    // standing at the first reigns at other
    // series. Fails under EE-2, the longest
    // reigns standing at the first spells at
    // their own seats. Fails under EE-3, the
    // last spells standing at the first
    // reigns at other series. Fails under
    // EE-4, the last reigns standing at the
    // first spells at their own seats.
    // Fails under EE-7, the first strikes
    // standing at the first spells at their
    // own seats. Fails under EE-8, the first
    // stands standing at the first reigns at
    // other series. Passes under EE-5,
    // matched standing unchanged under the
    // exchange. Passes under EE-6, the
    // sections standing outside the census
    // of the classes.
    let mut matched = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (class, _, _) = match outset_class(series) {
                Some(triple) => triple,
                None => continue,
            };
            if class == OutsetClass::Matched {
                matched += 1;
            }
        }
    }
    assert_eq!(matched, 17);
}
