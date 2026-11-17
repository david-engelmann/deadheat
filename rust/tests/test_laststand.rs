//! Pins for DH-60 and DH-61 in the Round 31
//! section of CONTRACT.md. The discrimination
//! record of this file stands pin by pin, and
//! no blanket claim stands over it. The last
//! stand reading, its library function, and
//! its command do not stand on the pre round
//! tree, so every pin in this file that names
//! the last stand reading is absent there at
//! compile time. That absence is recorded
//! once here and is not a discrimination
//! claim. The discrimination each pin carries
//! is against the stated mutation of DH-60
//! beside it. Eight mutations are named for
//! the whole file. Y-1, the last stand read as
//! the longest run of the loser over the
//! decided record instead of the last run.
//! Y-2, the last stand read as the first run
//! of the loser over the decided record
//! instead of the last run. Y-3, the closing
//! run of the winner over the decided record
//! returned where the last stand stands, the
//! class taken from the closing run as the
//! last stand's class stands taken from the
//! last stand. Y-4, the held class standing at
//! a last stand of three or more, a last stand
//! of two classed single, the rest of the
//! reading unchanged. Y-5, the held and
//! single classes exchanged, a last stand of
//! two or more classed single and a last
//! stand of one classed held, a series
//! without reply standing without reply as
//! before. Y-6, a series whose loser won no
//! decided race classed single instead of
//! without reply, its last stand standing at
//! zero. Y-7, the report carrying its sections
//! for the single series instead of the held
//! series, the closing standing as before.
//! Y-8, the last stand standing one above its
//! count wherever a run of the loser stands,
//! the class taken from the raised figure, a
//! series without reply standing at a last
//! stand of zero and without reply as before.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::championship::{load_championship_file, Championship};
use deadheat::json::JsonValue;
use deadheat::laststand::{laststand_class, laststand_report, LaststandClass};
use deadheat::races::decide_winner;
use deadheat::series::Series;
use num_bigint::BigInt;
use shared::*;

const ALPHA: &str = "alpha-1";
const BETA: &str = "beta-1";

const ENROLLED: [&str; 22] = [
    "championship.json",
    "constructed-closing.json",
    "constructed-cluster.json",
    "constructed-comeback.json",
    "constructed-crossings.json",
    "constructed-deadlock.json",
    "constructed-depths.json",
    "constructed-distance.json",
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

fn attempt(winner: &str, loser: &str, base: i64) -> JsonValue {
    obj(vec![
        ("model", s(winner)),
        ("attempt_id", s(&format!("{}-a{}", winner, base))),
        ("status", s("accepted")),
        (
            "records",
            obj(vec![
                ("score", f(9.0)),
                ("penalties", i(0)),
                ("adjustments", arr(vec![])),
                ("margin", f(1.5)),
            ]),
        ),
        ("seed", s(&format!("{}-s{}", winner, base))),
        ("checksum", s(&format!("{}-c{}", winner, base))),
        ("note", s(&format!("{}-n{}", winner, base))),
        ("rival", s(loser)),
    ])
}

fn attempt_two(winner: &str, loser: &str, base: i64) -> JsonValue {
    obj(vec![
        ("model", s(winner)),
        ("attempt_id", s(&format!("{}-a{}", winner, base))),
        ("status", s("accepted")),
        (
            "records",
            obj(vec![
                ("score", f(4.0)),
                ("penalties", i(0)),
                ("adjustments", arr(vec![])),
                ("margin", f(0.5)),
            ]),
        ),
        ("seed", s(&format!("{}-s{}", winner, base))),
        ("checksum", s(&format!("{}-c{}", winner, base))),
        ("note", s(&format!("{}-n{}", winner, base))),
        ("rival", s(loser)),
    ])
}

fn race_json(number: i64, task: &str, attempts: Vec<JsonValue>) -> JsonValue {
    obj(vec![
        ("race_id", s(&format!("race-{}", number))),
        ("task", s(task)),
        ("attempts", arr(attempts)),
    ])
}

fn clear_win(number: i64, task: &str, winner: &str, loser: &str) -> JsonValue {
    let base = number * 10;
    race_json(
        number,
        task,
        vec![
            attempt(winner, loser, base + 1),
            attempt_two(loser, winner, base + 2),
        ],
    )
}

fn no_result(number: i64, task: &str, first: &str, second: &str) -> JsonValue {
    let base = number * 10;
    race_json(
        number,
        task,
        vec![
            obj(vec![
                ("model", s(first)),
                ("attempt_id", s(&format!("{}-a{}", first, base + 1))),
                ("status", s("error")),
                (
                    "records",
                    obj(vec![
                        ("score", f(0.0)),
                        ("penalties", i(0)),
                        ("adjustments", arr(vec![])),
                        ("margin", f(0.0)),
                    ]),
                ),
                ("seed", s(&format!("{}-s{}", first, base + 1))),
                ("checksum", s(&format!("{}-c{}", first, base + 1))),
                ("note", s(&format!("{}-n{}", first, base + 1))),
                ("rival", s(second)),
            ]),
            obj(vec![
                ("model", s(second)),
                ("attempt_id", s(&format!("{}-a{}", second, base + 2))),
                ("status", s("error")),
                (
                    "records",
                    obj(vec![
                        ("score", f(0.0)),
                        ("penalties", i(0)),
                        ("adjustments", arr(vec![])),
                        ("margin", f(0.0)),
                    ]),
                ),
                ("seed", s(&format!("{}-s{}", second, base + 2))),
                ("checksum", s(&format!("{}-c{}", second, base + 2))),
                ("note", s(&format!("{}-n{}", second, base + 2))),
                ("rival", s(first)),
            ]),
        ],
    )
}

fn series_json(models: Vec<&str>, target: i64, races: Vec<JsonValue>) -> JsonValue {
    obj(vec![
        ("models", arr(models.iter().map(|model| s(model)).collect())),
        ("wins_target", i(target)),
        ("races", arr(races)),
    ])
}

fn write_json(dir: &Path, name: &str, value: &JsonValue) {
    std::fs::create_dir_all(dir).expect("create fixture directory");
    std::fs::write(dir.join(name), value.to_pretty()).expect("write fixture file");
}

fn championship_json(series_files: Vec<&str>) -> JsonValue {
    obj(vec![(
        "series",
        arr(series_files.iter().map(|name| s(name)).collect()),
    )])
}

fn load(dir: &Path) -> Championship {
    load_championship_file(&dir.join("championship.json")).expect("load fixture championship")
}

fn laststand_for(championship: &Championship) -> Vec<String> {
    lines_of(&laststand_report(championship))
}

fn load_example(name: &str) -> Championship {
    load_championship_file(&examples().join(name)).expect("load example championship")
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
fn the_classification_reads_the_last_run_of_the_loser() {
    // LS-1. A hand fixture of five decided races
    // at a target of three, the winner taking
    // the first, the fourth, and the fifth, the
    // loser the second and the third in one run
    // at the record's end of its own standing,
    // asserting at the module's entry that the
    // series classes held with a last stand of
    // two. Fails under Y-4, the raised cut
    // classing a last stand of two single.
    // Fails under Y-5, the exchanged classes
    // classing a last stand of two single.
    // Fails under Y-8, the last stand standing
    // one above its count at three. Passes under
    // Y-1, the loser's longest run standing at
    // two as its last run does. Passes under
    // Y-2, the loser's first run standing at two
    // as its last run does, the loser holding
    // one run in all. Passes under Y-3, the
    // closing run of the winner standing at two
    // as the last stand does. Passes under Y-6,
    // the loser having won decided races.
    // Passes under Y-7, the entry standing
    // outside the report the mutation moves.
    let dir = temp_dir("laststand-ls1");
    let task = "the hand classification series";
    let series = series_json(
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win(1, task, ALPHA, BETA),
            clear_win(2, task, BETA, ALPHA),
            clear_win(3, task, BETA, ALPHA),
            clear_win(4, task, ALPHA, BETA),
            clear_win(5, task, ALPHA, BETA),
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
        laststand_class(&championship.series[0]),
        Some((LaststandClass::Held, 2))
    );
}

#[test]
fn the_survey_championship_closes_at_three_held_and_three_single() {
    // LS-2. The survey championship's closing
    // line, asserting three held and three
    // single over its six series. Fails under
    // Y-2, the apex and drift series' first run
    // of one classing it single and the closing
    // falling to two held. Fails under Y-3, the
    // apex and crest closing run of one
    // classing it single while the bolt and
    // crest and the crest and drift closing
    // runs of three class two single series
    // held, the closing standing at four held
    // and two single. Fails under Y-4, the apex
    // and bolt and apex and crest series' last
    // stands of two classing single at the
    // raised cut and the closing falling to one
    // held. Fails under Y-8, the three single
    // series' last stands of one standing
    // raised at two and classing held, the
    // closing standing at six held. Passes
    // under Y-1, every survey series' longest
    // run standing at its last run's count or
    // in its class, the closing standing as
    // before. Passes under Y-5, the exchanged
    // classes exchanging three held for three
    // single and three single for three held,
    // the closing line standing word for word.
    // Passes under Y-6, no survey series
    // standing without reply. Passes under Y-7,
    // the closing standing as before while the
    // sections move.
    let championship = load_example("survey-championship.json");
    let lines = laststand_for(&championship);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("laststand, 6 series, 3 held, 3 single, 0 without reply, 0 no winner")
    );
}

#[test]
fn the_league_championship_closes_at_two_held_six_single_and_two_without_reply() {
    // LS-3. The league championship's closing
    // line, asserting two held, six single, and
    // two without reply over its ten series.
    // Fails under Y-1, the bolt and ember
    // series' longest run of two classing it
    // held and the held count rising to three.
    // Fails under Y-2, the bolt and ember
    // series' first run of two classing it held
    // on the same rise. Fails under Y-3, the
    // closing runs classing six series held.
    // Fails under Y-4, the apex and crest
    // series' last stand of two classing single
    // and the held count falling to the bolt
    // and crest series alone. Fails under Y-5,
    // the exchanged classes standing the closing
    // at six held and two single. Fails under
    // Y-6, the bolt and drift and drift and
    // ember series classing single and the
    // without reply count falling to zero.
    // Fails under Y-8, the raised figures
    // classing eight series held. Passes under
    // Y-7, the closing standing as before while
    // the sections move.
    let championship = load_example("league-championship.json");
    let lines = laststand_for(&championship);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("laststand, 10 series, 2 held, 6 single, 2 without reply, 0 no winner")
    );
}

#[test]
fn the_standing_championship_closes_at_one_single_and_one_without_reply() {
    // LS-4. The standing championship example's
    // closing line, asserting one single and one
    // without reply over its two series. Fails
    // under Y-3, the panel series' closing run
    // of three classing it held and a held count
    // appearing in the closing. Fails under
    // Y-5, the pilot series classing held under
    // the exchange. Fails under Y-6, the panel
    // series classing single and the without
    // reply count falling to zero. Fails under
    // Y-8, the pilot series' last stand of one
    // standing raised at two and classing held.
    // Passes under Y-1, the pilot series'
    // longest run standing at one as its last
    // run does and the panel series' loser
    // holding no run. Passes under Y-2, the
    // pilot series' first run standing at one
    // on the same ground. Passes under Y-4, a
    // last stand of one classing single at
    // either cut and the panel series standing
    // without reply. Passes under Y-7, the
    // closing standing as before.
    let championship = load_example("championship.json");
    let lines = laststand_for(&championship);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("laststand, 2 series, 0 held, 1 single, 1 without reply, 0 no winner")
    );
}

#[test]
fn the_recovery_championship_carries_the_second_series_section_alone() {
    // LS-5. The recovery championship under the
    // last stand reading, its second series
    // standing held at a last stand of three and
    // carrying the report's one section, its
    // first series standing single and section
    // free, the closing standing at one held and
    // one single. Fails under Y-1, the first
    // series' longest run of two classing it
    // held and the section moving to it. Fails
    // under Y-2, the first series' first run of
    // two classing it held on the same move.
    // Fails under Y-3, the first series' closing
    // run of three classing it held on the same
    // move. Fails under Y-5, the first series
    // classing held under the exchange and the
    // section moving to it. Fails under Y-7, the
    // sections standing for the single series,
    // the first series gaining the section the
    // second loses. Fails under Y-8, the first
    // series' last stand of one standing raised
    // at two and classing held, the section
    // moving to it. Passes under Y-4, the second
    // series' last stand of three holding at
    // either cut and the first series' one
    // classing single at either cut. Passes
    // under Y-6, neither series standing without
    // reply.
    let championship = load_example("constructed-recovery.json");
    let lines = laststand_for(&championship);
    assert_eq!(lines.len(), 13);
    assert_eq!(lines[0], "laststand");
    assert_eq!(lines[1], "series 2, the constructed recovery drill two");
    assert!(lines
        .iter()
        .any(|line| line == "last stand 3 over the decided record"));
    assert!(!lines.iter().any(|line| line.starts_with("series 1,")));
    assert_eq!(
        lines.last().map(String::as_str),
        Some("laststand, 2 series, 1 held, 1 single, 0 without reply, 0 no winner")
    );
}

#[test]
fn the_constructed_example_carries_the_held_series_alone() {
    // LS-6. The constructed laststand example,
    // drill one standing held at a last stand of
    // two and carrying the report's one section,
    // drill two standing single at a last stand
    // of one and section free, the closing
    // standing at one held and one single.
    // Fails under Y-1, drill two's longest run
    // of two classing it held and the closing
    // standing at two held. Fails under Y-2,
    // drill two's first run of two classing it
    // held on the same count. Fails under Y-3,
    // drill one's figure line reading the
    // closing run of three where the last stand
    // of two stands. Fails under Y-4, drill
    // one's last stand of two classing single
    // at the raised cut and the section falling
    // away. Fails under Y-5, the classes
    // exchanging and the section moving to
    // drill two. Fails under Y-7, the sections
    // standing for the single series on the
    // same move. Fails under Y-8, drill one's
    // figure standing raised at three and drill
    // two classing held at two. Passes under
    // Y-6, neither drill standing without
    // reply, each loser having won decided
    // races.
    let championship = load_example("constructed-laststand.json");
    let lines = laststand_for(&championship);
    assert_eq!(lines.len(), 15);
    assert_eq!(lines[1], "series 1, the constructed laststand drill one");
    assert!(lines
        .iter()
        .any(|line| line == "last stand 2 over the decided record"));
    assert!(!lines.iter().any(|line| line.starts_with("series 2,")));
    assert_eq!(
        lines.last().map(String::as_str),
        Some("laststand, 2 series, 1 held, 1 single, 0 without reply, 0 no winner")
    );
    assert_eq!(
        laststand_report(&championship),
        golden("constructed-laststand-laststand.txt")
    );
}

#[test]
fn the_recovery_drill_two_stands_held_at_a_last_stand_of_three() {
    // LS-7. The recovery championship's second
    // series at the module's entry, its last
    // stand standing at three and its class
    // held, the loser's three wins standing in
    // one run at the head of the decided
    // record. Fails under Y-3, the closing run
    // of the winner standing at four where the
    // last stand of three stands. Fails under
    // Y-5, the exchanged classes classing a
    // last stand of three single. Fails under
    // Y-8, the last stand standing one above
    // its count at four. Passes under Y-1, the
    // loser's longest run standing at three as
    // its last run does, the loser holding one
    // run in all. Passes under Y-2, the loser's
    // first run standing at three on the same
    // ground. Passes under Y-4, a last stand of
    // three holding at either cut. Passes under
    // Y-6, the loser having won decided races.
    // Passes under Y-7, the entry standing
    // outside the report the mutation moves.
    let championship = load_example("constructed-recovery.json");
    assert_eq!(
        laststand_class(&championship.series[1]),
        Some((LaststandClass::Held, 3))
    );
}

#[test]
fn the_recovery_drill_one_stands_single_below_its_longest_run() {
    // LS-8. The recovery championship's first
    // series at the module's entry, the parting
    // witness of the whole roster, its last
    // stand standing at one and its class
    // single while the longest run of its loser
    // stands at two, the loser's first run, the
    // record of seven decided races reading the
    // loser twice, the winner once, the loser
    // once, and the winner three times to the
    // decision. Fails under Y-1, the last stand
    // reading the longest run of two and the
    // class standing held. Fails under Y-2, the
    // last stand reading the first run of two
    // on the same class. Fails under Y-3, the
    // closing run of the winner standing at
    // three where the last stand of one stands.
    // Fails under Y-5, the exchanged classes
    // classing a last stand of one held. Fails
    // under Y-8, the last stand standing raised
    // at two and classing held. Passes under
    // Y-4, a last stand of one classing single
    // at either cut. Passes under Y-6, the
    // loser having won decided races. Passes
    // under Y-7, the entry standing outside the
    // report the mutation moves.
    let championship = load_example("constructed-recovery.json");
    assert_eq!(
        laststand_class(&championship.series[0]),
        Some((LaststandClass::Single, 1))
    );
}

#[test]
fn the_panel_series_stands_without_reply() {
    // LS-9. The standing championship example's
    // second series at the module's entry, the
    // panel series, its loser having won no
    // decided race, the series standing without
    // reply at a last stand of zero. Fails
    // under Y-3, the closing run of the winner
    // standing at three and classing the series
    // held where it stands without reply.
    // Fails under Y-6, the series classing
    // single where it stands without reply.
    // Passes under Y-1, the loser holding no run
    // for the longest reading to take. Passes
    // under Y-2, the loser holding no run for
    // the first reading to take. Passes under
    // Y-4, a last stand of zero standing without
    // reply at either cut. Passes under Y-5,
    // the exchange touching the held and single
    // classes alone and a series without reply
    // standing as it stands. Passes under Y-7,
    // the entry standing outside the report.
    // Passes under Y-8, no run of the loser
    // standing for the figure to rise above,
    // the last stand standing at zero.
    let championship = load_example("championship.json");
    assert_eq!(
        laststand_class(&championship.series[1]),
        Some((LaststandClass::WithoutReply, 0))
    );
}

#[test]
fn the_last_stand_stands_above_the_longest_run_on_no_series() {
    // LS-10. The theorem census. Over the
    // enrolment of twenty two championships,
    // the last stand standing above the longest
    // run of the loser on no series, counted
    // from the module's entry against a longest
    // run the pin derives from the decided
    // record itself. Fails under Y-3, the
    // closing run standing above the longest
    // run on the panel series and its like,
    // the winner's closing run of three
    // standing over a loser holding no run.
    // Fails under Y-8, the raised figure
    // standing above the longest run wherever
    // the last stand stands at it, the lead
    // drill one's last stand of three among
    // them. Passes under Y-1, the figure
    // standing at the longest run on every
    // series and so above it on none. Passes
    // under Y-2, the first run standing at or
    // below the longest run on every series.
    // Passes under Y-4, Y-5, and Y-6, the
    // classes moving while the figure the
    // census reads stands as it stands. Passes
    // under Y-7, the report's sections moving
    // while every entry stands.
    let mut above = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let Some((_, last)) = laststand_class(series) else {
                continue;
            };
            let decided = decided_record(series);
            let loser = loser_of(series);
            let longest = runs_of(&decided, loser).into_iter().max().unwrap_or(0);
            if last > longest {
                above += 1;
            }
        }
    }
    assert_eq!(above, 0);
}

#[test]
fn the_last_stand_stands_at_the_whole_of_the_losers_wins_on_thirty_series() {
    // LS-11. The theorem census at the whole
    // count. The last stand standing at the
    // whole of the loser's wins on thirty
    // series over the enrolment, counted from
    // the module's entry against a count of the
    // loser's wins the pin derives from the
    // decided record itself, the laststand
    // drill one standing among the thirty at a
    // last stand of two over two wins in one
    // run. Fails under Y-3, the closing run
    // standing at the whole of the loser's
    // wins on fewer series, the panel series'
    // closing run of three standing over no
    // wins among the partings. Fails under
    // Y-8, the raised figure standing at the
    // whole of the wins on fewer series still,
    // every equality at a standing run
    // breaking. Passes under Y-1, the longest
    // run standing at the whole of the wins
    // exactly where the wins stand in one run,
    // the same series the last stand names.
    // Passes under Y-2, the first run standing
    // at the whole of the wins exactly where
    // the first run stands alone, the same
    // series again. Passes under Y-4, Y-5, and
    // Y-6, the classes moving while the figure
    // stands. Passes under Y-7, the sections
    // moving while every entry stands.
    let mut at_whole = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let Some((_, last)) = laststand_class(series) else {
                continue;
            };
            let decided = decided_record(series);
            let loser = loser_of(series);
            let wins = decided.iter().filter(|side| **side == loser).count();
            if last == wins {
                at_whole += 1;
            }
        }
    }
    assert_eq!(at_whole, 30);
}

#[test]
fn a_series_of_target_one_stands_without_reply() {
    // LS-12. A series of target one, its one
    // race taken by the first model, the second
    // model standing as loser with no decided
    // race won, the series standing without
    // reply at a last stand of zero at the
    // module's entry. Fails under Y-3, the
    // closing run of the winner standing at one
    // and classing the series single. Fails
    // under Y-6, the series classing single
    // where it stands without reply. Passes
    // under Y-1, the loser holding no run for
    // the longest reading to take. Passes under
    // Y-2, the loser holding no run for the
    // first reading to take. Passes under Y-4,
    // a last stand of zero standing without
    // reply at either cut. Passes under Y-5,
    // the exchange touching the held and single
    // classes alone. Passes under Y-7, the
    // entry standing outside the report.
    // Passes under Y-8, no run of the loser
    // standing for the figure to rise above.
    let dir = temp_dir("laststand-ls12");
    let task = "the hand target one series";
    let series = series_json(vec![ALPHA, BETA], 1, vec![clear_win(1, task, ALPHA, BETA)]);
    write_json(&dir, "series.json", &series);
    write_json(
        &dir,
        "championship.json",
        &championship_json(vec!["series.json"]),
    );
    let championship = load(&dir);
    assert_eq!(
        laststand_class(&championship.series[0]),
        Some((LaststandClass::WithoutReply, 0))
    );
}

#[test]
fn a_series_holding_no_winner_takes_no_class_and_no_section() {
    // LS-13. The roster's one characterization
    // pin. A series holding no winner, its
    // three races standing at two wins against
    // one at a target of three, taking no class
    // at the module's entry and no section in
    // the report, the report over its
    // championship standing in the empty form
    // with the series counted in the no winner
    // place alone. The pin falls under no
    // mutation of the eight, each mutation
    // reading a decided record the series does
    // not hold, and it stands in the roster so
    // the no winner place of DH-60 and DH-61
    // stands pinned in words.
    let dir = temp_dir("laststand-ls13");
    let task = "the hand undecided series";
    let series = series_json(
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win(1, task, ALPHA, BETA),
            clear_win(2, task, BETA, ALPHA),
            clear_win(3, task, ALPHA, BETA),
        ],
    );
    write_json(&dir, "series.json", &series);
    write_json(
        &dir,
        "championship.json",
        &championship_json(vec!["series.json"]),
    );
    let championship = load(&dir);
    assert_eq!(laststand_class(&championship.series[0]), None);
    assert_eq!(
        laststand_for(&championship),
        vec![
            "laststand".to_string(),
            "laststand, 1 series, 0 held, 0 single, 0 without reply, 1 no winner".to_string(),
        ]
    );
}

#[test]
fn the_depths_championship_prints_the_empty_form() {
    // LS-14. The depths championship under the
    // last stand reading, no series standing
    // held, the report standing in the empty
    // form with its closing at two single.
    // Fails under Y-1, drill one's longest run
    // of two classing it held and a section
    // appearing. Fails under Y-2, drill one's
    // first run of two classing it held on the
    // same appearance. Fails under Y-3, drill
    // one's closing run of three classing it
    // held on the same appearance. Fails under
    // Y-5, both series classing held under the
    // exchange and two sections appearing.
    // Fails under Y-7, the sections standing
    // for the single series and both series
    // gaining sections. Fails under Y-8, both
    // series' last stands of one standing
    // raised at two and classing held. Passes
    // under Y-4, a last stand of one classing
    // single at either cut and no section
    // standing at either cut. Passes under
    // Y-6, neither series standing without
    // reply.
    let championship = load_example("constructed-depths.json");
    assert_eq!(
        laststand_for(&championship),
        vec![
            "laststand".to_string(),
            "laststand, 2 series, 0 held, 2 single, 0 without reply, 0 no winner".to_string(),
        ]
    );
    assert_eq!(
        laststand_report(&championship),
        golden("constructed-depths-laststand.txt")
    );
}

#[test]
fn the_drill_one_section_stands_in_the_family_form() {
    // LS-15. The section form over the
    // laststand championship, drill one's
    // section standing with its series line,
    // its models and target line, its race
    // lines numbered by the record count over
    // the nine decided races with the race
    // holding no winner's number skipped, its
    // figure line, and the series winner line
    // of DH-30, the whole report standing line
    // for line as the golden stands. Fails
    // under Y-1, drill two's longest run of two
    // classing it held and a second section
    // appearing. Fails under Y-2, drill two's
    // first run of two classing it held on the
    // same appearance. Fails under Y-3, drill
    // one's figure line reading the closing
    // run of three. Fails under Y-4, drill
    // one's last stand of two classing single
    // and its section falling away. Fails
    // under Y-5, the section moving to drill
    // two under the exchange. Fails under Y-7,
    // the sections standing for the single
    // series on the same move. Fails under
    // Y-8, drill one's figure standing raised
    // at three and drill two gaining a
    // section. Passes under Y-6, neither drill
    // standing without reply.
    let championship = load_example("constructed-laststand.json");
    let expected: Vec<String> = vec![
        "laststand",
        "series 1, the constructed laststand drill one",
        "whimbrel-1 against redshank-1, first to 5 wins",
        "race 1, laststand drill one race one, whimbrel-1 wins",
        "race 2, laststand drill one race two, whimbrel-1 wins",
        "race 3, laststand drill one race three, redshank-1 wins",
        "race 4, laststand drill one race four, redshank-1 wins",
        "race 6, laststand drill one race six, whimbrel-1 wins",
        "race 7, laststand drill one race seven, whimbrel-1 wins",
        "race 8, laststand drill one race eight, redshank-1 wins",
        "race 9, laststand drill one race nine, redshank-1 wins",
        "race 10, laststand drill one race ten, redshank-1 wins",
        "last stand 2 over the decided record",
        "series winner, redshank-1, whimbrel-1 4 race wins, redshank-1 5 race wins, 1 no results",
        "laststand, 2 series, 1 held, 1 single, 0 without reply, 0 no winner",
    ]
    .iter()
    .map(|line| line.to_string())
    .collect();
    assert_eq!(laststand_for(&championship), expected);
    assert_eq!(
        laststand_report(&championship),
        golden("constructed-laststand-laststand.txt")
    );
}

#[test]
fn the_last_stand_and_the_closing_run_together_stand_above_the_decided_count_on_no_series() {
    // LS-16. The adjacency census. Over the
    // enrolment, the last stand and the closing
    // run of the winner together standing above
    // the decided count on no series, the two
    // runs standing back to back over the
    // decided record's tail and so never
    // passing it together, counted from the
    // module's entry against figures the pin
    // derives from the decided record itself.
    // Fails under Y-3, the closing run counted
    // twice standing above the decided count
    // wherever it passes half of it, the panel
    // series' three standing over its three
    // among them. Fails under Y-8, the raised
    // figure carrying the sum above the count
    // where the two runs stand at it together,
    // the lead drill one and the spells drill
    // two among them. Passes under Y-1, the
    // longest run standing no higher than the
    // last, the sum standing where the shipped
    // sum stands or below it. Passes under
    // Y-2, the first run standing in the last
    // run's place on the same ground. Passes
    // under Y-4, Y-5, and Y-6, the classes
    // moving while the figures stand. Passes
    // under Y-7, the sections moving while
    // every entry stands.
    let mut above = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let Some((_, last)) = laststand_class(series) else {
                continue;
            };
            let decided = decided_record(series);
            let loser = loser_of(series);
            let closing = runs_of(&decided, 1 - loser).into_iter().last().unwrap_or(0);
            if last + closing > decided.len() {
                above += 1;
            }
        }
    }
    assert_eq!(above, 0);
}

#[test]
fn the_enrolled_championships_close_at_fifteen_held_over_fifty_three_series() {
    // LS-17. The aggregate census over the
    // enrolment of fifty three series, the
    // closings of the twenty two championships
    // summed to fifteen held, thirty four
    // single, four without reply, and no series
    // without a winner. Fails under Y-1, the
    // bolt and ember series and their like
    // classing held at their longest runs and
    // the held sum rising. Fails under Y-2, the
    // first runs moving the sums on the same
    // series and their like. Fails under Y-3,
    // the closing runs standing in the last
    // stand's place over the whole enrolment.
    // Fails under Y-4, the held sum falling as
    // last stands of two class single. Fails
    // under Y-5, the held and single sums
    // exchanging. Fails under Y-6, the without
    // reply sum falling to zero as its four
    // series class single. Fails under Y-8, the
    // single sum falling as last stands of one
    // stand raised and held. Passes under Y-7,
    // the sections moving while every closing
    // stands.
    let mut totals = [0u64; 5];
    for championship in enrolled() {
        let lines = laststand_for(&championship);
        let counts = closing_counts(lines.last().expect("closing line"));
        for (index, count) in counts.iter().enumerate() {
            totals[index] += count;
        }
    }
    assert_eq!(totals, [53, 15, 34, 4, 0]);
}

#[test]
fn a_race_holding_no_winner_neither_extends_nor_breaks_the_run() {
    // LS-18. A hand fixture of six races at a
    // target of three, the third race holding
    // no winner, the decided record reading the
    // winner, the loser twice, and the winner
    // twice to the decision, the race holding
    // no winner neither extending nor breaking
    // the loser's run, the series standing held
    // at a last stand of two at the module's
    // entry and its championship closing at one
    // held. Fails under Y-4, the raised cut
    // classing a last stand of two single.
    // Fails under Y-5, the exchanged classes
    // classing a last stand of two single.
    // Fails under Y-8, the last stand standing
    // raised at three. Passes under Y-1, the
    // loser's longest run standing at two as
    // its last run does. Passes under Y-2, the
    // loser's first run standing at two on the
    // same ground. Passes under Y-3, the
    // closing run of the winner standing at
    // two as the last stand does. Passes under
    // Y-6, the loser having won decided races.
    // Passes under Y-7, the entry and the
    // closing standing as they stand while the
    // sections move.
    let dir = temp_dir("laststand-ls18");
    let task = "the hand broken record series";
    let series = series_json(
        vec![ALPHA, BETA],
        3,
        vec![
            clear_win(1, task, ALPHA, BETA),
            clear_win(2, task, BETA, ALPHA),
            no_result(3, task, ALPHA, BETA),
            clear_win(4, task, BETA, ALPHA),
            clear_win(5, task, ALPHA, BETA),
            clear_win(6, task, ALPHA, BETA),
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
        laststand_class(&championship.series[0]),
        Some((LaststandClass::Held, 2))
    );
    let lines = laststand_for(&championship);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("laststand, 1 series, 1 held, 0 single, 0 without reply, 0 no winner")
    );
}

#[test]
fn the_last_stand_stands_below_the_longest_run_on_eight_series() {
    // LS-19. The parting census. The last stand
    // standing below the longest run of the
    // loser on eight series over the enrolment,
    // the recovery drill one among them, its
    // last stand of one standing below its
    // longest run of two, and the laststand
    // drill two beside it on the same parting,
    // counted from the module's entry against
    // a longest run the pin derives from the
    // decided record itself. Fails under Y-1,
    // the figure standing at the longest run on
    // every series and the parting count
    // standing at zero. Fails under Y-2, the
    // first run standing below the longest on a
    // count of its own, the crossings drill
    // two's first run of one below its longest
    // of two among the partings the shipped
    // reading does not count. Fails under Y-3,
    // the closing run standing below the
    // longest run on a count of its own. Fails
    // under Y-8, the raised figure standing at
    // the longest run on the partings one
    // apart, the crossings drill one and its
    // like, and the count falling. Passes
    // under Y-4, Y-5, and Y-6, the classes
    // moving while the figures the census
    // reads stand. Passes under Y-7, the
    // sections moving while every entry
    // stands.
    let mut below = 0;
    for championship in enrolled() {
        for series in &championship.series {
            let Some((_, last)) = laststand_class(series) else {
                continue;
            };
            let decided = decided_record(series);
            let loser = loser_of(series);
            let longest = runs_of(&decided, loser).into_iter().max().unwrap_or(0);
            if last < longest {
                below += 1;
            }
        }
    }
    assert_eq!(below, 8);
}

#[test]
fn the_survey_sections_stand_in_the_championships_order() {
    // LS-20. The survey championship's sections
    // standing in the championship's own order,
    // the apex and bolt series first, the apex
    // and crest series second, and the apex and
    // drift series third, each the held series
    // of its place in the file. Fails under
    // Y-2, the apex and drift series' first run
    // of one classing it single and its section
    // falling out of the order. Fails under
    // Y-3, the apex and crest series' closing
    // run of one classing it single on the same
    // loss. Fails under Y-4, the first two
    // sections' series classing single at the
    // raised cut and the order standing at the
    // apex and drift section alone. Fails under
    // Y-5, the sections standing for the three
    // single series under the exchange. Fails
    // under Y-7, the sections standing for the
    // single series on the same three. Fails
    // under Y-8, the bolt and crest series'
    // last stand of one standing raised at two
    // and a fourth section joining the order.
    // Passes under Y-1, the three series'
    // longest runs standing held at two, two,
    // and four, the sections standing in their
    // places while their figures move. Passes
    // under Y-6, no survey series standing
    // without reply.
    let championship = load_example("survey-championship.json");
    let lines = laststand_for(&championship);
    let sections: Vec<&str> = lines
        .iter()
        .filter(|line| line.starts_with("series "))
        .map(String::as_str)
        .collect();
    assert_eq!(
        sections,
        vec![
            "series 1, the constructed survey series, apex-1 against bolt-1",
            "series 2, the constructed survey series, apex-1 against crest-1",
            "series 3, the constructed survey series, apex-1 against drift-1",
        ]
    );
}
