//! Pins for DH-64 and DH-65 in the Round 33
//! section of CONTRACT.md. The discrimination
//! record of this file stands pin by pin, and
//! no blanket claim stands over it. The first
//! strike reading, its library function, and
//! its command do not stand on the pre round
//! tree, so every pin in this file that names
//! the first strike reading is absent there at
//! compile time. That absence is recorded
//! once here and is not a discrimination
//! claim. The discrimination each pin carries
//! is against the stated mutation of DH-64
//! beside it. Eight mutations are named for
//! the whole file. AA-1, the first strike read
//! as the longest run of the winner over the
//! decided record instead of its first run.
//! AA-2, the first strike read as the closing
//! run of the winner over the decided record
//! instead of its first run. AA-3, the first
//! strike read as the first run of the loser
//! over the decided record instead of the
//! first run of the winner. AA-4, the in a
//! run class standing at a first strike of
//! three or more, a first strike of two
//! classed alone, the rest of the reading
//! unchanged. AA-5, the in a run and alone
//! classes exchanged, a first strike of two
//! or more classed alone and a first strike
//! of one classed in a run. AA-6, the opening
//! run of the winner under DH-38 standing in
//! the strike's place, a series whose loser
//! won the first decided race classed alone
//! at a strike of none. AA-7, the report
//! carrying its sections for the alone series
//! instead of the series standing in a run,
//! the closing standing as before. AA-8, the
//! first strike standing one above its count
//! wherever a run of the winner stands, the
//! class taken from the raised figure.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{load_championship_file, Championship};
use deadheat::firststrike::{firststrike_class, firststrike_report, FirststrikeClass};
use deadheat::json::JsonValue;
use deadheat::races::decide_winner;
use deadheat::series::Series;
use num_bigint::BigInt;
use shared::*;

const ALPHA: &str = "alpha-1";
const BETA: &str = "beta-1";

const ENROLLED: [&str; 24] = [
    "championship.json",
    "constructed-closing.json",
    "constructed-cluster.json",
    "constructed-comeback.json",
    "constructed-crossings.json",
    "constructed-deadlock.json",
    "constructed-depths.json",
    "constructed-distance.json",
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

#[test]
fn the_classification_reads_the_first_run_of_the_winner() {
    // SK-1. The firststrike characterization pin.
    // A hand fixture of nine decided races at a
    // target of five, its blocks standing winner
    // two, loser two, winner one, loser two, and
    // winner two, the winner championing with
    // its first strike at two, its longest run
    // at two, its closing run at two, the
    // loser's first run at two, and the winner's
    // opening run at two, every neighbour figure
    // standing at the strike's own count. It
    // asserts the strike at two and the in a run
    // class over it at the module's entry, the
    // section in full over the fixture, and the
    // closing line of its one series
    // championship. Fails under AA-4, the raised
    // cut classing a strike of two alone. Fails
    // under AA-5, the exchanged classes classing
    // a strike of two alone. Fails under AA-7,
    // the sections standing for the alone series
    // and this series losing its section. Fails
    // under AA-8, the strike standing one above
    // its count at three. Passes under AA-1, the
    // winner's longest run standing at two as
    // its first run does. Passes under AA-2, the
    // winner's closing run standing at two on
    // the same ground. Passes under AA-3, the
    // loser's first run standing at two on the
    // same ground. Passes under AA-6, the
    // winner's opening run standing at two on
    // the same ground.
    let dir = temp_dir("firststrike-sk1");
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
    assert_eq!(
        firststrike_class(&championship.series[0]),
        Some((FirststrikeClass::InARun, 2))
    );
    let expected: Vec<String> = vec![
        "firststrike",
        "series 1, the hand classification series",
        "birch-1 against ash-1, first to 5 wins",
        "race 1, the hand classification series, birch-1 wins",
        "race 2, the hand classification series, birch-1 wins",
        "race 3, the hand classification series, ash-1 wins",
        "race 4, the hand classification series, ash-1 wins",
        "race 5, the hand classification series, birch-1 wins",
        "race 6, the hand classification series, ash-1 wins",
        "race 7, the hand classification series, ash-1 wins",
        "race 8, the hand classification series, birch-1 wins",
        "race 9, the hand classification series, birch-1 wins",
        "first strike 2 over the decided record",
        "series winner, birch-1, birch-1 5 race wins, ash-1 4 race wins, 0 no results",
        "firststrike, 1 series, 1 in a run, 0 alone, 0 no winner",
    ]
    .iter()
    .map(|line| line.to_string())
    .collect();
    assert_eq!(firststrike_report(&championship), expected);
}

#[test]
fn the_survey_championship_closes_at_three_in_a_run_and_three_alone() {
    // SK-2. The survey championship's closing
    // line, asserting three in a run and three
    // alone over its six series. Fails under
    // AA-1, the longest runs moving the closing.
    // Fails under AA-2, the closing runs moving
    // the closing on their own count. Fails
    // under AA-3, the losers' first runs moving
    // the closing. Fails under AA-6, the opening
    // runs and the from behind seats moving the
    // closing. Fails under AA-8, every strike
    // standing one above its count and the
    // closing moving. Passes under AA-4, no
    // survey strike standing at two for the
    // raised cut to move. Passes under AA-5, the
    // exchange standing the closing at the same
    // three and three over this field. Passes
    // under AA-7, the closing standing as before
    // while its sections move.
    let championship = load_example("survey-championship.json");
    let report = firststrike_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("firststrike, 6 series, 3 in a run, 3 alone, 0 no winner")
    );
    assert_eq!(
        closing_counts(report.last().expect("a closing")),
        vec![6, 3, 3, 0]
    );
}

#[test]
fn the_league_championship_closes_at_seven_in_a_run_and_three_alone() {
    // SK-3. The league championship's closing
    // line, asserting seven in a run and three
    // alone over its ten series. Fails under
    // AA-1, AA-2, AA-3, AA-4, AA-5, and AA-6, the
    // league fields moving under each, and
    // under AA-8, every strike standing raised.
    // Passes under AA-7, the closing standing as
    // before while its sections move.
    let championship = load_example("league-championship.json");
    let report = firststrike_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("firststrike, 10 series, 7 in a run, 3 alone, 0 no winner")
    );
    assert_eq!(
        closing_counts(report.last().expect("a closing")),
        vec![10, 7, 3, 0]
    );
}

#[test]
fn the_championship_example_closes_at_two_in_a_run_with_two_sections() {
    // SK-4. The championship example's closing
    // line, asserting two in a run and none
    // alone over its two series, with the
    // sections of both series standing in
    // championship order. Fails under AA-2, the
    // closing runs moving the pilot off its
    // strike. Fails under AA-3, the losers'
    // first runs moving both series. Fails under
    // AA-4, the pilot's strike of two classing
    // alone at the raised cut and its section
    // falling away. Fails under AA-5, the classes
    // standing exchanged and both sections
    // falling away. Fails under AA-7, the
    // sections standing for the alone series and
    // this championship carrying none. Passes
    // under AA-1, both strikes standing at their
    // longest runs. Passes under AA-6, both
    // winners opening with the runs this reading
    // counts. Passes under AA-8, the raised
    // strikes of three and four standing in a
    // run still and the closing unmoved.
    let championship = load_example("championship.json");
    let report = firststrike_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("firststrike, 2 series, 2 in a run, 0 alone, 0 no winner")
    );
    let sections: Vec<&String> = report
        .iter()
        .filter(|line| line.starts_with("series "))
        .collect();
    assert_eq!(sections.len(), 2);
    assert!(sections[0].starts_with("series 1,"));
    assert!(sections[1].starts_with("series 2,"));
}

#[test]
fn the_constructed_recovery_championship_closes_at_one_in_a_run_and_one_alone() {
    // SK-5. The constructed recovery
    // championship's closing line, asserting one
    // in a run and one alone over its two
    // series, the seat whose DH-62 closing
    // stands at two held and none single against
    // this reading's split. Fails under AA-1,
    // the longest runs moving the closing.
    // Fails under AA-2, the closing runs moving
    // the closing. Fails under AA-3, the losers'
    // first runs moving the closing. Fails under
    // AA-6, the opening runs moving the closing.
    // Fails under AA-8, the strikes standing
    // raised and the closing moving. Passes
    // under AA-4, neither strike standing at two
    // for the raised cut to move. Passes under
    // AA-5, the exchange standing the closing at
    // the same one and one over this pair.
    // Passes under AA-7, the closing standing as
    // before while its sections move.
    let championship = load_example("constructed-recovery.json");
    let report = firststrike_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("firststrike, 2 series, 1 in a run, 1 alone, 0 no winner")
    );
    assert_eq!(
        closing_counts(report.last().expect("a closing")),
        vec![2, 1, 1, 0]
    );
}

#[test]
fn the_constructed_firststrike_championship_closes_at_one_in_a_run_and_one_alone() {
    // SK-6. The constructed firststrike
    // championship's closing line, asserting one
    // in a run and one alone over its two
    // series. Fails under AA-1, the second
    // drill's longest run of three classing it
    // in a run. Fails under AA-2, the second
    // drill's closing run of three classing it
    // in a run on the same move. Fails under
    // AA-4, the first drill's strike of two
    // classing alone at the raised cut. Fails
    // under AA-6, the first drill standing from
    // behind and classing alone at none. Fails
    // under AA-8, the second drill's strike
    // standing raised at two and classing in a
    // run. Passes under AA-3, the losers' first
    // runs standing at one over the first drill
    // and three over the second, classing the
    // first alone and the second in a run, the
    // closing standing unmoved at one and one.
    // Passes under AA-5 in its closing alone,
    // the exchange standing this pair at one and
    // one still, its sections moving against
    // SK-15. Passes under AA-7, the closing
    // standing as before.
    let championship = load_example("constructed-firststrike.json");
    let report = firststrike_report(&championship);
    assert_eq!(
        report.last().map(|line| line.as_str()),
        Some("firststrike, 2 series, 1 in a run, 1 alone, 0 no winner")
    );
    assert_eq!(
        closing_counts(report.last().expect("a closing")),
        vec![2, 1, 1, 0]
    );
}

#[test]
fn the_first_drill_classes_in_a_run_at_a_strike_of_two() {
    // SK-7. The first drill's entry assertion,
    // the series standing in a run at a strike
    // of two, its winner standing from behind
    // under DH-38, its closing run at two, its
    // loser's first stand at one, and its
    // loser's last stand at two. Fails under
    // AA-3, the loser's first stand of one
    // classing the series alone. Fails under
    // AA-4, the strike of two classing alone at
    // the raised cut. Fails under AA-5, the
    // exchange classing the strike of two alone.
    // Fails under AA-6, the series standing from
    // behind and taking no class at the strike's
    // seat. Fails under AA-8, the strike standing
    // raised at three. Passes under AA-1, the
    // winner's longest run standing at two.
    // Passes under AA-2, the winner's closing
    // run standing at two. Passes under AA-7,
    // the entry standing outside the report's
    // sections.
    let championship = load_example("constructed-firststrike.json");
    assert_eq!(
        firststrike_class(&championship.series[0]),
        Some((FirststrikeClass::InARun, 2))
    );
}

#[test]
fn the_second_drill_classes_alone_at_a_strike_of_one() {
    // SK-8. The second drill's entry assertion,
    // the series standing alone at a strike of
    // one, its winner's longest run at three,
    // its closing run at three, and its loser's
    // first stand at three. Fails under AA-1,
    // the longest run of three classing it in a
    // run. Fails under AA-2, the closing run of
    // three classing it in a run on the same
    // move. Fails under AA-3, the loser's first
    // stand of three classing it in a run.
    // Fails under AA-5, the exchange classing
    // the strike of one in a run. Fails under
    // AA-8, the strike standing raised at two
    // and classing in a run. Passes under AA-4,
    // the strike of one standing alone at the
    // raised cut as before. Passes under AA-6,
    // the winner's opening run standing at one.
    // Passes under AA-7, the entry standing
    // outside the report's sections.
    let championship = load_example("constructed-firststrike.json");
    assert_eq!(
        firststrike_class(&championship.series[1]),
        Some((FirststrikeClass::Alone, 1))
    );
}

#[test]
fn the_panel_series_strikes_at_three_with_its_first_stand_at_none() {
    // SK-9. The panel series of the championship
    // example, its strike standing at three in a
    // run and its loser's first stand standing
    // at none, the loser holding no decided race
    // of the record. Fails under AA-3, the
    // loser's first stand of none classing the
    // series at no strike the entry returns.
    // Fails under AA-5, the exchange classing
    // the strike of three alone. Fails under
    // AA-8, the strike standing raised at four.
    // Passes under AA-1, the winner's longest
    // run standing at three. Passes under AA-2,
    // the winner's closing run standing at
    // three. Passes under AA-4, the strike of
    // three standing in a run at the raised cut.
    // Passes under AA-6, the winner's opening run
    // standing at three at the head of the
    // record. Passes under AA-7, the entry
    // standing outside the report's sections.
    let championship = load_example("championship.json");
    let series = &championship.series[1];
    assert_eq!(
        firststrike_class(series),
        Some((FirststrikeClass::InARun, 3))
    );
    let decided = decided_record(series);
    assert!(runs_of(&decided, loser_of(series)).is_empty());
}

#[test]
fn the_first_strike_never_stands_above_the_winners_longest_run() {
    // SK-10. The theorem census of DH-64 over
    // the enrolled fifty seven series, the first
    // strike standing above the winner's longest
    // run at no series, every series of the
    // enrolment holding a winner. Fails under
    // AA-3, the losers' first runs standing above
    // the winners' longest runs at series the
    // enrolment carries. Fails under AA-8, the
    // raised strike standing above the longest
    // run wherever the strike stands at it, as
    // at the panel series of the championship
    // example. Passes under AA-1, the figure
    // standing at the longest run itself and
    // never above it. Passes under AA-2, the
    // closing run standing at or below the
    // longest by its form. Passes under AA-4,
    // AA-5, AA-6, and AA-7, the census reading
    // the strike's count alone at the entry,
    // where the opening run stands at or below
    // the longest by its form and the classes
    // and sections stand outside the count.
    let mut decided = 0;
    let mut violations = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, strike) = match firststrike_class(series) {
                Some(pair) => pair,
                None => continue,
            };
            decided += 1;
            let record = decided_record(series);
            let winner = winner_of(series).expect("a decided series");
            let longest = runs_of(&record, winner).into_iter().max().unwrap_or(0) as u64;
            if strike > longest {
                violations += 1;
            }
        }
    }
    assert_eq!(decided, 57);
    assert_eq!(violations, 0);
}

#[test]
fn the_first_strike_stands_at_the_whole_of_the_win_count_at_twelve_series() {
    // SK-11. The whole of the win count census
    // of DH-64 over the enrolled fifty seven
    // series, the first strike standing at the
    // whole of its winner's win count at exactly
    // twelve series, the panel series of the
    // championship example standing among them,
    // its winner winning three races in one run
    // at the head of the record. Fails under
    // AA-3, the losers' first runs standing at
    // the whole of the winners' counts at other
    // series and falling short at these. Fails
    // under AA-6, the opening runs standing at
    // the whole of the count at fewer series,
    // the runs standing wherever they stand
    // under this reading and at the head alone
    // under DH-38. Passes under AA-1 and AA-2 by
    // the one run identity, a series whose
    // winner won in one run standing with its
    // longest and closing runs at the same whole
    // count. Passes under AA-8 by the enrolment's
    // own distribution, no raised strike landing
    // on a win count it did not already stand
    // at. Passes under AA-4, AA-5, and AA-7, the
    // census reading the strike's count alone.
    let mut whole = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, strike) = match firststrike_class(series) {
                Some(pair) => pair,
                None => continue,
            };
            let record = decided_record(series);
            let winner = winner_of(series).expect("a decided series");
            let wins = record.iter().filter(|side| **side == winner).count() as u64;
            if strike == wins {
                whole += 1;
            }
        }
    }
    assert_eq!(whole, 12);
}

#[test]
fn a_series_at_a_wins_target_of_one_stands_alone() {
    // SK-12. The wins target one fixture, a
    // series of one decided race at a target of
    // one, its winner winning the one race the
    // record holds before the decision and its
    // strike standing at one alone. Fails under
    // AA-3, the loser holding no decided race
    // and the entry returning no class at a
    // strike of none. Fails under AA-5, the
    // exchange classing the strike of one in a
    // run. Fails under AA-8, the strike standing
    // raised at two and classing in a run.
    // Passes under AA-1, the winner's longest
    // run standing at one. Passes under AA-2,
    // the winner's closing run standing at one.
    // Passes under AA-4, the strike of one
    // standing alone at the raised cut as
    // before. Passes under AA-6, the winner's
    // opening run standing at one. Passes under
    // AA-7, the report carrying no section for
    // the series under either form.
    let dir = temp_dir("firststrike-sk12");
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
        firststrike_class(&championship.series[0]),
        Some((FirststrikeClass::Alone, 1))
    );
    assert_eq!(
        firststrike_report(&championship),
        vec![
            "firststrike".to_string(),
            "firststrike, 1 series, 0 in a run, 1 alone, 0 no winner".to_string(),
        ]
    );
}

#[test]
fn a_series_holding_no_winner_takes_no_class_and_no_section() {
    // SK-13. The roster's characterization pin
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
    // the no winner place of DH-64 and DH-65
    // stands pinned in words.
    let dir = temp_dir("firststrike-sk13");
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
    assert_eq!(firststrike_class(&championship.series[0]), None);
    assert_eq!(
        firststrike_report(&championship),
        vec![
            "firststrike".to_string(),
            "firststrike, 1 series, 0 in a run, 0 alone, 1 no winner".to_string(),
        ]
    );
}

#[test]
fn a_championship_holding_no_series_stands_refused_at_the_load() {
    // SK-14. The empty report form pin. A
    // championship holding no series at all
    // stands refused at the load, the empty
    // series list drawing the one defect
    // field series holds no series files, so
    // no championship stands for a report to
    // stand over. The pin falls under no
    // mutation of the eight, no series
    // standing in the championship for any
    // mutation to read, and it stands in the
    // roster so the load boundary of DH-65
    // stands pinned in words.
    let dir = temp_dir("firststrike-sk14");
    write_json(&dir, "championship.json", &championship_json(vec![]));
    let (championship, defects) = load_championship_file(&dir.join("championship.json"));
    assert_eq!(championship, None);
    assert_eq!(
        defects,
        vec!["field series holds no series files".to_string()]
    );
}

#[test]
fn the_drill_one_section_stands_in_the_family_form() {
    // SK-15. The firststrike pair section pin.
    // The section form over the firststrike
    // championship, drill one's section standing
    // in full with its series line, its models
    // and target line, its race lines over the
    // decided races numbered 1, 2, 4, 5, 6, 7,
    // and 8, its figure line at a first strike
    // of two, and its series winner line, drill
    // two standing in no section, the whole
    // report standing line for line as the
    // golden stands. Fails under every variant
    // of the eight. Under AA-1, drill two's
    // longest run of three classing it in a run
    // and drawing its section. Under AA-2, drill
    // two's closing run of three drawing its
    // section on the same move. Under AA-3,
    // drill one's strike reading one and its
    // section falling away while drill two gains
    // the section at three. Under AA-4, drill
    // one's strike of two classing alone and its
    // section falling away. Under AA-5, the
    // sections moving between the drills under
    // the exchange. Under AA-6, drill one
    // standing from behind and losing its
    // section. Under AA-7, the sections standing
    // for the alone series, drill two drawing
    // the section and drill one losing it.
    // Under AA-8, drill one's figure standing
    // raised at three and drill two gaining a
    // section at two.
    let championship = load_example("constructed-firststrike.json");
    let expected: Vec<String> = vec![
        "firststrike",
        "series 1, the constructed firststrike drill one",
        "firststrike-ash against firststrike-birch, first to 4 wins",
        "race 1, firststrike drill one race one, firststrike-ash wins",
        "race 2, firststrike drill one race two, firststrike-birch wins",
        "race 4, firststrike drill one race four, firststrike-birch wins",
        "race 5, firststrike drill one race five, firststrike-ash wins",
        "race 6, firststrike drill one race six, firststrike-ash wins",
        "race 7, firststrike drill one race seven, firststrike-birch wins",
        "race 8, firststrike drill one race eight, firststrike-birch wins",
        "first strike 2 over the decided record",
        "series winner, firststrike-birch, firststrike-ash 3 race wins, firststrike-birch 4 race wins, 1 no results",
        "firststrike, 2 series, 1 in a run, 1 alone, 0 no winner",
    ]
    .iter()
    .map(|line| line.to_string())
    .collect();
    assert_eq!(firststrike_report(&championship), expected);
    assert_eq!(
        firststrike_report(&championship).join("\n") + "\n",
        golden("constructed-firststrike-firststrike.txt")
    );
}

#[test]
fn the_first_strike_and_the_first_stand_never_stand_above_the_decided_count() {
    // SK-16. The adjacency census of DH-64 over
    // the enrolled fifty seven series, the first
    // strike and the loser's first stand
    // together standing above the count of the
    // decided races at no series, the two runs
    // standing as the first two blocks of the
    // record in the one order or the other, the
    // sum standing at the whole of the decided
    // count at the second series of the
    // constructed turns championship among the
    // seats where it stands at the whole. Fails
    // under AA-8 alone, the raised strike
    // carrying the sum above the count wherever
    // the two blocks stand at the whole of the
    // record, as at the panel series, its raised
    // strike of four and its first stand of none
    // standing above its three decided races.
    // Passes under every other variant, the
    // disjointness of the two blocks carrying
    // the bound against each, the figures the
    // variants read standing as blocks of the
    // same record or at counts the bound already
    // holds.
    let mut violations = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, strike) = match firststrike_class(series) {
                Some(pair) => pair,
                None => continue,
            };
            let record = decided_record(series);
            let loser = loser_of(series);
            let first_stand = runs_of(&record, loser).first().copied().unwrap_or(0) as u64;
            if strike + first_stand > record.len() as u64 {
                violations += 1;
            }
        }
    }
    assert_eq!(violations, 0);
}

#[test]
fn the_enrolled_championships_close_at_thirty_three_in_a_run_and_twenty_four_alone() {
    // SK-17. The aggregate closing over the
    // enrolled fifty seven series of the twenty
    // four championships, asserting thirty three
    // in a run, twenty four alone, and none
    // holding no winner, summed over the closing
    // line of each championship's report. Fails
    // under AA-1, AA-2, AA-3, AA-4, AA-5, and
    // AA-6, the fields of the enrolment moving
    // under each, and under AA-8, every strike
    // standing raised over the enrolment.
    // Passes under AA-7, the closings standing
    // as before while their sections move.
    let mut totals = vec![0u64; 4];
    for championship in enrolled() {
        let report = firststrike_report(&championship);
        let counts = closing_counts(report.last().expect("a closing"));
        for (index, count) in counts.iter().enumerate() {
            totals[index] += count;
        }
    }
    assert_eq!(totals, vec![57, 33, 24, 0]);
}

#[test]
fn the_first_drills_section_spans_the_record_places_of_its_strike() {
    // SK-18. The span pin on the first drill.
    // Its section's race lines stand numbered by
    // the counting walk's place among the
    // decided races at 1, 2, 4, 5, 6, 7, and 8,
    // the race holding no winner at the third
    // place of the record standing in no race
    // line, and its figure line stands at a
    // first strike of two. Fails under AA-3, the
    // strike reading the loser's first stand of
    // one and the section falling away. Fails
    // under AA-4, the strike of two classing
    // alone at the raised cut and the section
    // falling away. Fails under AA-5, the section
    // moving to the second drill under the
    // exchange. Fails under AA-6, the first drill
    // standing from behind and losing its
    // section. Fails under AA-7, the sections
    // standing for the alone series. Fails under
    // AA-8, the figure standing raised at three.
    // Passes under AA-1, the winner's longest run
    // standing at two and the section standing
    // word for word. Passes under AA-2, the
    // winner's closing run standing at two on
    // the same ground.
    let championship = load_example("constructed-firststrike.json");
    let report = firststrike_report(&championship);
    let places: Vec<u64> = report
        .iter()
        .filter(|line| line.starts_with("race "))
        .map(|line| {
            line.trim_start_matches("race ")
                .split(',')
                .next()
                .expect("a place")
                .parse::<u64>()
                .expect("a place number")
        })
        .collect();
    assert_eq!(places, vec![1, 2, 4, 5, 6, 7, 8]);
    assert!(report.contains(&"first strike 2 over the decided record".to_string()));
}

#[test]
fn the_first_strike_stands_below_the_longest_run_at_twenty_three_series() {
    // SK-19. The parting census of DH-64 over
    // the enrolled fifty seven series, the first
    // strike standing below the winner's longest
    // run at exactly twenty three series, the
    // second series of the constructed laststand
    // championship and the series of the
    // constructed rally championship standing
    // among them, and the second drill of the
    // constructed firststrike championship
    // adding the twenty third. Fails under AA-1,
    // the figure standing at the longest run
    // itself at every series and the count
    // falling to none. Fails under AA-2, the
    // closing runs standing below the longest at
    // other series and at the longest at these.
    // Fails under AA-3, the losers' first runs
    // standing below the winners' longest runs
    // at a count of their own. Fails under AA-6,
    // the opening runs standing below the
    // longest at a count of their own. Fails
    // under AA-8, the raised strikes standing
    // below the longest at fewer series. Passes
    // under AA-4, AA-5, and AA-7, the census
    // reading the strike's count alone at the
    // entry.
    let mut below = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let (_, strike) = match firststrike_class(series) {
                Some(pair) => pair,
                None => continue,
            };
            let record = decided_record(series);
            let winner = winner_of(series).expect("a decided series");
            let longest = runs_of(&record, winner).into_iter().max().unwrap_or(0) as u64;
            if strike < longest {
                below += 1;
            }
        }
    }
    assert_eq!(below, 23);
}

#[test]
fn the_steady_series_stand_at_eight_with_a_loser_win_and_four_without_reply() {
    // SK-20. The steady census, the guard pin of
    // FIX-DH-31. Over the enrolled fifty seven
    // series, the series standing steady under
    // DH-40, their decided record changing hands
    // once or not at all, stand at exactly eight
    // with a win of their loser beside them and
    // exactly four standing without reply, the
    // second series of the constructed turns
    // championship standing among the eight, its
    // loser winning the record's first two
    // decided races before the winner's one run
    // of three closes the record. The pin
    // falls under no variant of this family, its
    // office standing at the amendment's count
    // and not at this reading's figure, and it
    // stands in the roster so the count
    // FIX-DH-31 states stands pinned in words
    // beside the reading it corrects.
    let mut with_loser_win = 0;
    let mut without_reply = 0;
    for championship in enrolled() {
        for series in &championship.series {
            if winner_of(series).is_none() {
                continue;
            }
            let record = decided_record(series);
            let changes = record.windows(2).filter(|pair| pair[0] != pair[1]).count();
            if changes > 1 {
                continue;
            }
            let loser = loser_of(series);
            let loser_wins = record.iter().filter(|side| **side == loser).count();
            if loser_wins >= 1 {
                with_loser_win += 1;
            }
            if runs_of(&record, loser).is_empty() {
                without_reply += 1;
            }
        }
    }
    assert_eq!(with_loser_win, 8);
    assert_eq!(without_reply, 4);
}
