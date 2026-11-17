//! Pins for DH-62 and DH-63 in the Round 32
//! section of CONTRACT.md. The discrimination
//! record of this file stands pin by pin, and
//! no blanket claim stands over it. The first
//! stand reading, its library function, and
//! its command do not stand on the pre round
//! tree, so every pin in this file that names
//! the first stand reading is absent there at
//! compile time. That absence is recorded
//! once here and is not a discrimination
//! claim. The discrimination each pin carries
//! is against the stated mutation of DH-62
//! beside it. Eight mutations are named for
//! the whole file. Z-1, the first stand read
//! as the longest run of the loser over the
//! decided record instead of the first run.
//! Z-2, the first stand read as the last run
//! of the loser over the decided record
//! instead of the first run. Z-3, the first
//! stand read as the opening run of the
//! winner over the decided record instead of
//! the first run of the loser. Z-4, the held
//! class standing at a first stand of three
//! or more, a first stand of two classed
//! single, the rest of the reading unchanged.
//! Z-5, the held and single classes
//! exchanged, a first stand of two or more
//! classed single and a first stand of one
//! classed held, a series without reply
//! standing without reply as before. Z-6, a
//! series whose loser won no decided race
//! classed single instead of without reply,
//! its first stand standing at zero. Z-7, the
//! report carrying its sections for the
//! single series instead of the held series,
//! the closing standing as before. Z-8, the
//! first stand standing one above its count
//! wherever a run of the loser stands, the
//! class taken from the raised figure, a
//! series without reply standing at a first
//! stand of zero and without reply as before.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{load_championship_file, Championship};
use deadheat::firststand::{firststand_class, firststand_report, FirststandClass};
use deadheat::json::JsonValue;
use deadheat::races::decide_winner;
use deadheat::series::Series;
use num_bigint::BigInt;
use shared::*;

const ALPHA: &str = "alpha-1";
const BETA: &str = "beta-1";
const ASH: &str = "firststand-ash";
const BIRCH: &str = "firststand-birch";

const ENROLLED: [&str; 23] = [
    "championship.json",
    "constructed-closing.json",
    "constructed-cluster.json",
    "constructed-comeback.json",
    "constructed-crossings.json",
    "constructed-deadlock.json",
    "constructed-depths.json",
    "constructed-distance.json",
    "constructed-firststand.json",
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

fn loser_of(series: &Series) -> usize {
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
        1
    } else {
        0
    }
}

fn enrolled() -> Vec<Championship> {
    ENROLLED.iter().map(|name| load_example(name)).collect()
}

#[test]
fn the_classification_reads_the_first_run_of_the_loser() {
    // FS-1. The firststand characterization pin.
    // A hand fixture of nine decided races at a
    // target of five, its blocks standing birch
    // two, ash two, birch one, ash two, and
    // birch two, birch championing and ash
    // losing with its first stand at two, its
    // last run at two, its longest run at two,
    // and the winner's opening run at two, every
    // neighbour figure standing at the first
    // stand's own count. It asserts the first
    // stand at two and the held class over it at
    // the module's entry, the section in full
    // over the fixture, and the closing line of
    // its one series championship. Fails under
    // Z-4, the raised cut classing a first stand
    // of two single. Fails under Z-5, the
    // exchanged classes classing a first stand
    // of two single. Fails under Z-7, the
    // sections standing for the single series
    // and this series losing its section. Fails
    // under Z-8, the first stand standing one
    // above its count at three. Passes under
    // Z-1, the loser's longest run standing at
    // two as its first run does. Passes under
    // Z-2, the loser's last run standing at two
    // on the same ground. Passes under Z-3, the
    // winner's opening run standing at two on
    // the same ground. Passes under Z-6, the
    // loser having won decided races.
    let dir = temp_dir("firststand-fs1");
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
        firststand_class(&championship.series[0]),
        Some((FirststandClass::Held, 2))
    );
    let expected: Vec<String> = vec![
        "firststand",
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
        "first stand 2 over the decided record",
        "series winner, birch-1, birch-1 5 race wins, ash-1 4 race wins, 0 no results",
        "firststand, 1 series, 1 held, 0 single, 0 without reply, 0 no winner",
    ]
    .iter()
    .map(|line| line.to_string())
    .collect();
    assert_eq!(firststand_report(&championship), expected);
}

#[test]
fn the_survey_championship_closes_at_two_held_and_four_single() {
    // FS-2. The survey championship's closing
    // line, asserting two held and four single
    // over its six series. Fails under Z-1, the
    // longest runs moving the closing, the
    // survey's third series standing at a first
    // stand of one below its longest run of
    // three among the partings. Fails under
    // Z-2, the last runs moving the closing on
    // their own count. Fails under Z-3, the
    // opening runs of the winner moving the
    // closing. Fails under Z-4, the first stands
    // of two classing single at the raised cut
    // and the held count falling. Fails under
    // Z-5, the exchanged classes standing the
    // closing at four held and two single.
    // Fails under Z-8, the raised figures
    // classing first stands of one held and the
    // held count rising. Passes under Z-6, no
    // survey series standing without reply.
    // Passes under Z-7, the closing standing as
    // before while the sections move.
    let championship = load_example("survey-championship.json");
    let lines = firststand_report(&championship);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("firststand, 6 series, 2 held, 4 single, 0 without reply, 0 no winner")
    );
}

#[test]
fn the_league_championship_closes_at_three_held_five_single_and_two_without_reply() {
    // FS-3. The league championship's closing
    // line, asserting three held, five single,
    // and two without reply over its ten series.
    // Fails under Z-2, the last runs moving the
    // closing on their own count. Fails under
    // Z-3, the opening runs of the winner
    // moving the closing. Fails under Z-4, the
    // first stands of two classing single at
    // the raised cut and the held count
    // falling. Fails under Z-5, the exchanged
    // classes standing the closing at five held
    // and three single. Fails under Z-6, the
    // two series without reply classing single
    // and the without reply count falling to
    // zero. Fails under Z-8, the raised figures
    // classing first stands of one held. Passes
    // under Z-1, the league championship's
    // closings and sections standing unmoved
    // under the longest run, its figure standing
    // at the first stand's own count on each of
    // its held series. Passes under Z-7, the
    // closing standing as before.
    let championship = load_example("league-championship.json");
    let lines = firststand_report(&championship);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("firststand, 10 series, 3 held, 5 single, 2 without reply, 0 no winner")
    );
}

#[test]
fn the_standing_championship_closes_at_one_single_and_one_without_reply() {
    // FS-4. The standing championship example's
    // closing line, asserting one single and
    // one without reply over its two series.
    // Fails under Z-3, the panel series' winner
    // opening run of three classing it held and
    // a held count appearing in the closing.
    // Fails under Z-5, the pilot series classing
    // held under the exchange. Fails under Z-6,
    // the panel series classing single and the
    // without reply count falling to zero.
    // Fails under Z-8, the pilot series' first
    // stand of one standing raised at two and
    // classing held. Passes under Z-1, the pilot
    // series' longest run standing at one as its
    // first run does and the panel series'
    // loser holding no run. Passes under Z-2,
    // the pilot series' last run standing at
    // one on the same ground. Passes under Z-4,
    // a first stand of one classing single at
    // either cut and the panel series standing
    // without reply. Passes under Z-7, the
    // closing standing as before.
    let championship = load_example("championship.json");
    let lines = firststand_report(&championship);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("firststand, 2 series, 0 held, 1 single, 1 without reply, 0 no winner")
    );
}

#[test]
fn the_recovery_championship_closes_at_two_held() {
    // FS-5. The recovery championship's closing
    // line, asserting two held and no single
    // over its two series, the first series
    // standing held at a first stand of two and
    // the second at a first stand of three.
    // Fails under Z-2, the first series' last
    // stand of one classing it single and the
    // held count falling to one. Fails under
    // Z-3, the opening runs of the winner
    // moving the closing. Fails under Z-4, the
    // first series' first stand of two classing
    // single at the raised cut on the same
    // fall. Fails under Z-5, the exchanged
    // classes standing the closing at no held
    // and two single. Passes under Z-1, each
    // series' longest run standing at its first
    // stand's own count, two and three. Passes
    // under Z-6, neither series standing without
    // reply. Passes under Z-7, the closing
    // standing as before. Passes under Z-8, the
    // raised figures standing at three and four
    // and both series holding at either figure.
    let championship = load_example("constructed-recovery.json");
    let lines = firststand_report(&championship);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("firststand, 2 series, 2 held, 0 single, 0 without reply, 0 no winner")
    );
}

#[test]
fn the_constructed_example_closes_at_one_held_and_one_single() {
    // FS-6. The constructed firststand
    // example's closing line alone, asserting
    // one held and one single over its two
    // series. Fails under Z-1, drill two's
    // longest run of two classing it held and
    // the closing standing at two held. Fails
    // under Z-3, drill one's winner opening run
    // standing at none over its loser's opening
    // and the closing losing its held series.
    // Fails under Z-4, drill one's first stand
    // of two classing single at the raised cut
    // and the closing standing at no held.
    // Fails under Z-8, drill one's figure
    // standing raised at three and drill two
    // classing held at two, the closing
    // standing at two held. Passes under Z-2,
    // the last runs exchanging the two series'
    // classes, drill one single at one and
    // drill two held at two, the closing
    // standing word for word. Passes under
    // Z-5, the exchanged classes exchanging one
    // held for one single on the same ground.
    // Passes under Z-6, neither drill standing
    // without reply. Passes under Z-7, the
    // closing standing as before.
    let championship = load_example("constructed-firststand.json");
    let lines = firststand_report(&championship);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("firststand, 2 series, 1 held, 1 single, 0 without reply, 0 no winner")
    );
}

#[test]
fn the_firststand_drill_one_stands_held_at_a_first_stand_of_two() {
    // FS-7. The constructed firststand
    // championship's first series at the
    // module's entry, its first stand standing
    // at two and its class held, the run of two
    // standing at the head of the decided
    // record with the race holding no winner
    // standing inside its span. Fails under
    // Z-2, its last stand standing at one and
    // its class single. Fails under Z-3, its
    // winner's opening run standing at none and
    // its class without reply. Fails under Z-4,
    // a first stand of two classing single at
    // the raised cut. Fails under Z-5, the
    // exchanged classes classing a first stand
    // of two single. Fails under Z-8, the first
    // stand standing one above its count at
    // three. Passes under Z-1, the loser's
    // longest run standing at two as its first
    // run does. Passes under Z-6, the loser
    // having won decided races. Passes under
    // Z-7, the entry standing outside the
    // report the mutation moves.
    let championship = load_example("constructed-firststand.json");
    assert_eq!(
        firststand_class(&championship.series[0]),
        Some((FirststandClass::Held, 2))
    );
}

#[test]
fn the_firststand_drill_two_stands_single_below_its_longest_run() {
    // FS-8. The constructed firststand
    // championship's second series at the
    // module's entry, its first stand standing
    // at one and its class single while the
    // longest run of its loser stands at two
    // standing after its first. Fails under
    // Z-1, the first stand reading the longest
    // run of two and the class standing held.
    // Fails under Z-2, the first stand reading
    // the last run of two on the same class.
    // Fails under Z-5, the exchanged classes
    // classing a first stand of one held.
    // Fails under Z-8, the first stand standing
    // raised at two and classing held. Passes
    // under Z-3, its winner's opening run
    // standing at one and its class single on
    // the same figure. Passes under Z-4, a
    // first stand of one classing single at
    // either cut. Passes under Z-6, the loser
    // having won decided races. Passes under
    // Z-7, the entry standing outside the
    // report.
    let championship = load_example("constructed-firststand.json");
    assert_eq!(
        firststand_class(&championship.series[1]),
        Some((FirststandClass::Single, 1))
    );
}

#[test]
fn the_panel_series_stands_without_reply() {
    // FS-9. The standing championship example's
    // second series at the module's entry, the
    // panel series, its loser having won no
    // decided race, the series standing without
    // reply at a first stand of zero. Fails
    // under Z-3, the winner's opening run of
    // three classing the series held where it
    // stands without reply. Fails under Z-6,
    // the series classing single where it
    // stands without reply. Passes under Z-1,
    // the loser holding no run for the longest
    // reading to take. Passes under Z-2, the
    // loser holding no run for the last reading
    // to take. Passes under Z-4, a first stand
    // of zero standing without reply at either
    // cut. Passes under Z-5, the exchange
    // touching the held and single classes
    // alone. Passes under Z-7, the entry
    // standing outside the report. Passes under
    // Z-8, no run of the loser standing for the
    // figure to rise above, the first stand
    // standing at zero.
    let championship = load_example("championship.json");
    assert_eq!(
        firststand_class(&championship.series[1]),
        Some((FirststandClass::WithoutReply, 0))
    );
}

#[test]
fn the_first_stand_stands_above_the_longest_run_on_no_series() {
    // FS-10. The theorem census. Over the
    // enrolment of twenty three championships,
    // the first stand standing above the
    // longest run of the loser on no series,
    // counted from the module's entry against a
    // longest run the pin derives from the
    // decided record itself. Fails under Z-3,
    // the opening run standing above the
    // longest run wherever the loser holds no
    // run, the panel series' three standing
    // over none among them. Fails under Z-8,
    // the raised figure standing above the
    // longest run wherever the first stand
    // stands at it. Passes under Z-1, the
    // figure standing at the longest run on
    // every series and so above it on none.
    // Passes under Z-2, the last run standing
    // at or below the longest run on every
    // series. Passes under Z-4, Z-5, and Z-6,
    // the classes moving while the figure the
    // census reads stands as it stands. Passes
    // under Z-7, the report's sections moving
    // while every entry stands.
    let mut above = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let Some((_, first)) = firststand_class(series) else {
                continue;
            };
            let first = first as usize;
            let decided = decided_record(series);
            let loser = loser_of(series);
            let longest = runs_of(&decided, loser).into_iter().max().unwrap_or(0);
            if first > longest {
                above += 1;
            }
        }
    }
    assert_eq!(above, 0);
}

#[test]
fn the_first_stand_stands_at_the_whole_of_the_losers_wins_on_twenty_five_series() {
    // FS-11. The theorem census at the whole
    // count. The first stand standing at the
    // whole of the loser's wins on twenty five
    // series whose loser won at least one race
    // over the enrolment, counted from the
    // module's entry against a count of the
    // loser's wins the pin derives from the
    // decided record itself, exactly the series
    // whose loser won its races in one run
    // standing first. Fails under Z-3, the
    // opening run standing at the whole of the
    // loser's wins on fewer series, the panel
    // series' opening run of three standing
    // over no wins among the partings. Fails
    // under Z-8, the raised figure standing at
    // the whole of the wins on fewer series
    // still, every equality at a standing run
    // breaking. Passes under Z-1, the longest
    // run standing at the whole of the wins on
    // the same twenty five series the design
    // computes. Passes under Z-2, the last run
    // standing at the whole of the wins on the
    // same series again. Passes under Z-4,
    // Z-5, and Z-6, the classes moving while
    // the figure stands. Passes under Z-7, the
    // sections moving while every entry stands.
    let mut at_whole = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let Some((_, first)) = firststand_class(series) else {
                continue;
            };
            let first = first as usize;
            let decided = decided_record(series);
            let loser = loser_of(series);
            let wins = decided.iter().filter(|side| **side == loser).count();
            if wins >= 1 && first == wins {
                at_whole += 1;
            }
        }
    }
    assert_eq!(at_whole, 25);
}

#[test]
fn a_series_of_target_one_stands_without_reply() {
    // FS-12. A series of target one, its one
    // race taken by the first model, the second
    // model standing as loser with no decided
    // race won, the series standing without
    // reply at a first stand of zero at the
    // module's entry. Fails under Z-3, the
    // winner's opening run standing at one and
    // classing the series single. Fails under
    // Z-6, the series classing single where it
    // stands without reply. Passes under Z-1,
    // the loser holding no run for the longest
    // reading to take. Passes under Z-2, the
    // loser holding no run for the first
    // reading to take. Passes under Z-4, a first
    // stand of zero standing without reply at
    // either cut. Passes under Z-5, the exchange
    // touching the held and single classes
    // alone. Passes under Z-7, the entry
    // standing outside the report. Passes under
    // Z-8, no run of the loser standing for the
    // figure to rise above.
    let dir = temp_dir("firststand-fs12");
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
        firststand_class(&championship.series[0]),
        Some((FirststandClass::WithoutReply, 0))
    );
}

#[test]
fn a_series_holding_no_winner_takes_no_class_and_no_section() {
    // FS-13. The roster's characterization pin
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
    // the no winner place of DH-62 and DH-63
    // stands pinned in words.
    let dir = temp_dir("firststand-fs13");
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
    assert_eq!(firststand_class(&championship.series[0]), None);
    assert_eq!(
        firststand_report(&championship),
        vec![
            "firststand".to_string(),
            "firststand, 1 series, 0 held, 0 single, 0 without reply, 1 no winner".to_string(),
        ]
    );
}

#[test]
fn a_championship_holding_no_series_prints_the_empty_form() {
    // FS-14. The empty report form pin. A
    // championship holding no series at all,
    // its report standing as the closing line
    // over zero series and no section. The pin
    // falls under no mutation of the eight, no
    // series standing in the championship for
    // any mutation to read, and it stands in
    // the roster so the empty form of DH-63
    // stands pinned in words.
    let dir = temp_dir("firststand-fs14");
    write_json(&dir, "championship.json", &championship_json(vec![]));
    let championship = load(&dir);
    assert_eq!(
        firststand_report(&championship),
        vec![
            "firststand".to_string(),
            "firststand, 0 series, 0 held, 0 single, 0 without reply, 0 no winner".to_string(),
        ]
    );
}

#[test]
fn the_drill_one_section_stands_in_the_family_form() {
    // FS-15. The firststand pair section pin.
    // The section form over the firststand
    // championship, drill one's section standing
    // in full with its series line, its models
    // and target line, its race lines over the
    // decided races numbered 1, 3, 4, 5, 6, 7,
    // and 8, its figure line at a first stand of
    // two, and its series winner line, drill two
    // standing in no section, the whole report
    // standing line for line as the golden
    // stands. Fails under Z-2, drill one's last
    // stand of one classing it single and its
    // section falling away while drill two
    // gains the section. Fails under Z-3, drill
    // one's winner opening run standing at none
    // and its section falling away. Fails under
    // Z-4, drill one's first stand of two
    // classing single at the raised cut and its
    // section falling away. Fails under Z-5,
    // the section moving to drill two under the
    // exchange. Fails under Z-7, the sections
    // standing for the single series on the
    // same move. Fails under Z-8, drill one's
    // figure standing raised at three and drill
    // two gaining a section. Passes under Z-1
    // at the section of drill one itself, its
    // longest run standing at two as its first
    // run does, the section standing word for
    // word, its second series assertion
    // standing charged beside it in this lane's
    // report. Passes under Z-6, neither drill
    // standing without reply.
    let championship = load_example("constructed-firststand.json");
    let expected: Vec<String> = vec![
        "firststand",
        "series 1, the constructed firststand drill one",
        "firststand-ash against firststand-birch, first to 4 wins",
        "race 1, firststand drill one race one, firststand-ash wins",
        "race 3, firststand drill one race three, firststand-ash wins",
        "race 4, firststand drill one race four, firststand-birch wins",
        "race 5, firststand drill one race five, firststand-ash wins",
        "race 6, firststand drill one race six, firststand-birch wins",
        "race 7, firststand drill one race seven, firststand-birch wins",
        "race 8, firststand drill one race eight, firststand-birch wins",
        "first stand 2 over the decided record",
        "series winner, firststand-birch, firststand-ash 3 race wins, firststand-birch 4 race wins, 1 no results",
        "firststand, 2 series, 1 held, 1 single, 0 without reply, 0 no winner",
    ]
    .iter()
    .map(|line| line.to_string())
    .collect();
    assert_eq!(firststand_report(&championship), expected);
    assert_eq!(
        firststand_report(&championship).join("\n") + "\n",
        golden("constructed-firststand-firststand.txt")
    );
}

#[test]
fn the_first_stand_and_the_opening_run_together_stand_above_the_decided_count_on_no_series() {
    // FS-16. The adjacency census. Over the
    // enrolment, the first stand and the
    // opening run of the winner together
    // standing above the count of the decided
    // races on no series, the two runs standing
    // as the first two blocks of the record or
    // as one block where the loser opens the
    // record, counted from the module's entry
    // against figures the pin derives from the
    // decided record itself. Fails under Z-3
    // alone, the opening run counted twice
    // standing above the decided count at four
    // series. Passes under Z-1, the longest run
    // standing in the first run's place and the
    // sum standing where the shipped sum stands
    // or below it on the design's computation.
    // Passes under Z-2, the last run standing
    // in the first run's place on the same
    // ground. Passes under Z-4, Z-5, and Z-6,
    // the classes moving while the figures
    // stand. Passes under Z-7, the sections
    // moving while every entry stands. Passes
    // under Z-8, the raised figure standing in
    // the sum on the design's computation,
    // which prices the sum above the count on
    // no series under the raised figure either.
    let mut above = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let Some((_, first)) = firststand_class(series) else {
                continue;
            };
            let first = first as usize;
            let decided = decided_record(series);
            let loser = loser_of(series);
            let winner = 1 - loser;
            let opening = if decided.first() == Some(&winner) {
                runs_of(&decided, winner).first().copied().unwrap_or(0)
            } else {
                0
            };
            if first + opening > decided.len() {
                above += 1;
            }
        }
    }
    assert_eq!(above, 0);
}

#[test]
fn the_enrolled_championships_close_at_twenty_two_held_over_fifty_five_series() {
    // FS-17. The aggregate census over the
    // enrolment of fifty five series, the
    // closings of the twenty three
    // championships summed to twenty two held,
    // twenty nine single, four without reply,
    // and no series without a winner. Fails
    // under Z-1, the longest runs moving the
    // sums. Fails under Z-2, the last runs
    // moving the sums. Fails under Z-3, the
    // opening runs standing in the first
    // stand's place over the whole enrolment.
    // Fails under Z-4, the held sum falling as
    // first stands of two class single. Fails
    // under Z-5, the held and single sums
    // exchanging. Fails under Z-6, the without
    // reply sum falling to zero as its four
    // series class single. Fails under Z-8, the
    // single sum falling as first stands of one
    // stand raised and held. Passes under Z-7,
    // the sections moving while every closing
    // stands.
    let mut totals = [0u64; 5];
    for championship in enrolled() {
        let lines = firststand_report(&championship);
        let counts = closing_counts(lines.last().expect("closing line"));
        for (index, count) in counts.iter().enumerate() {
            totals[index] += count;
        }
    }
    assert_eq!(totals, [55, 22, 29, 4, 0]);
}

#[test]
fn the_pair_drill_one_span_stands_over_the_race_holding_no_winner() {
    // FS-18. The firststand span pin. The first
    // series of the constructed pair alone, its
    // first stand standing at two over the
    // decided record with the race holding no
    // winner standing inside the span between
    // the walked places 1 and 3, and the race
    // lines of its section standing numbered 1,
    // 3, 4, 5, 6, 7, and 8. Fails under Z-2, its
    // last stand of one classing it single and
    // its section falling away. Fails under
    // Z-3, its winner's opening run standing at
    // none on the same loss. Fails under Z-4, a
    // first stand of two classing single at the
    // raised cut. Fails under Z-5, the exchanged
    // classes classing a first stand of two
    // single. Fails under Z-7, the sections
    // standing for the single series and this
    // section falling away. Fails under Z-8, its
    // figure line standing raised at three.
    // Passes under Z-1, its longest run standing
    // at two as its first run does, its section
    // and its race lines standing as they
    // stand. Passes under Z-6, its loser having
    // won decided races.
    let championship = load_example("constructed-firststand.json");
    assert_eq!(
        firststand_class(&championship.series[0]),
        Some((FirststandClass::Held, 2))
    );
    let lines = firststand_report(&championship);
    let start = lines
        .iter()
        .position(|line| line.starts_with("series 1,"))
        .expect("the first series section");
    let mut numbers: Vec<u64> = Vec::new();
    for line in &lines[start + 1..] {
        if line.starts_with("first stand") {
            break;
        }
        if let Some(rest) = line.strip_prefix("race ") {
            let number: u64 = rest
                .split(',')
                .next()
                .expect("a race number")
                .parse()
                .expect("the race number parses");
            numbers.push(number);
        }
    }
    assert_eq!(numbers, vec![1, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn the_first_stand_stands_below_the_longest_run_on_three_series() {
    // FS-19. The parting census. The first
    // stand standing below the longest run of
    // the loser on exactly three series over
    // the enrolment, the second series of the
    // constructed crossings championship and
    // the third series of the survey
    // championship of the standing enrolment
    // and the second series of the constructed
    // firststand championship beside them,
    // counted from the module's entry against a
    // longest run the pin derives from the
    // decided record itself. Fails under Z-1,
    // the figure standing at the longest run on
    // every series and the parting count
    // standing at zero. Fails under Z-2, the
    // last run standing below the longest run
    // on a count of its own. Fails under Z-3,
    // the opening run standing below the
    // longest run on a count of its own. Fails
    // under Z-8, the raised figure standing at
    // the longest run on the partings one apart
    // and the count falling. Passes under Z-4,
    // Z-5, and Z-6, the classes moving while
    // the figures the census reads stand.
    // Passes under Z-7, the sections moving
    // while every entry stands.
    let mut below = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let Some((_, first)) = firststand_class(series) else {
                continue;
            };
            let first = first as usize;
            let decided = decided_record(series);
            let loser = loser_of(series);
            let longest = runs_of(&decided, loser).into_iter().max().unwrap_or(0);
            if first < longest {
                below += 1;
            }
        }
    }
    assert_eq!(below, 3);
}

#[test]
fn the_recovery_sections_stand_in_the_championships_order() {
    // FS-20. The firststand section order pin.
    // The recovery championship's sections
    // standing in the championship's own series
    // order, the series A section before the
    // series B section, both series standing
    // held under this reading at first stands of
    // two and three, the sections' figure lines
    // standing in the same order. Fails under
    // Z-2, the first series' last stand of one
    // classing it single and its section
    // falling out of the order. Fails under
    // Z-3, the opening runs moving the
    // sections. Fails under Z-4, the first
    // series' first stand of two classing
    // single at the raised cut and its section
    // falling out. Fails under Z-5, the sections
    // standing for the two single series under
    // the exchange and this order standing
    // empty. Fails under Z-7, the sections
    // standing for the single series on the
    // same loss. Fails under Z-8, the figure
    // lines standing raised at three and four.
    // Passes under Z-1, the two series' longest
    // runs standing at two and three as their
    // first stands do, the sections and their
    // figures standing in their places. Passes
    // under Z-6, neither series standing without
    // reply.
    let championship = load_example("constructed-recovery.json");
    let lines = firststand_report(&championship);
    let mut seats: Vec<(&str, &str)> = Vec::new();
    let mut pending: Option<&str> = None;
    for line in &lines {
        if line.starts_with("series ") && !line.starts_with("series winner") {
            pending = Some(line.as_str());
        } else if line.starts_with("first stand") {
            seats.push((pending.expect("a section figure line"), line.as_str()));
        }
    }
    assert_eq!(
        seats,
        vec![
            (
                "series 1, the constructed recovery drill one",
                "first stand 2 over the decided record"
            ),
            (
                "series 2, the constructed recovery drill two",
                "first stand 3 over the decided record"
            ),
        ]
    );
}
