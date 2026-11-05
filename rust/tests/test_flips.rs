//! Pins for DH-15 and DH-16, the flip reading, in the Round 7
//! section of CONTRACT.md.
//!
//! Each pin is built so it fails under the negation of the
//! rule it names. The scratch winner of a race must come
//! through the one winner rule with every adjustment
//! cleared, so the reading stands in pairs, the same race
//! with and without the figure that moves it, and a reading
//! that compared raw finishes alone fails the band pins, the
//! panel pin, and the tie pins. The adjusted winner of every
//! race is pinned against the series report itself, which
//! names the race winner independently of this reading.

mod shared;

use std::path::PathBuf;

use deadheat::flips::{flip_report, scratch_winner};
use deadheat::json::{dumps, JsonValue};
use deadheat::races::{decide_winner, is_close_call, Race};
use deadheat::series::{load_series_file, load_series_text, series_report, Series};
use shared::*;

const PRIME: &str = "prime-1";
const SHADOW: &str = "shadow-1";

fn attempt(
    model: &str,
    result: &str,
    finish: Option<JsonValue>,
    adjustment: Option<JsonValue>,
) -> JsonValue {
    let mut pairs = vec![
        ("model", s(model)),
        ("environment", s("lane")),
        ("result", s(result)),
        ("tokens", i(1000)),
        ("patch", s("a diff")),
    ];
    if let Some(finish) = finish {
        pairs.push(("finished_at", finish));
    }
    if let Some(adjustment) = adjustment {
        pairs.push(("adjustment", adjustment));
    }
    obj(pairs)
}

fn green(model: &str, finish: JsonValue, adjustment: Option<JsonValue>) -> JsonValue {
    attempt(model, "green", Some(finish), adjustment)
}

fn entry(chosen: &str) -> JsonValue {
    obj(vec![
        ("adjudicator", s("judge-1")),
        ("chosen", s(chosen)),
        ("confidence", f(0.9)),
        ("reason", s("the steadier diff")),
    ])
}

fn race(task: &str, first: JsonValue, second: JsonValue) -> JsonValue {
    obj(vec![
        ("task", s(task)),
        ("attempts", arr(vec![first, second])),
    ])
}

fn series(races: Vec<JsonValue>, target: i64) -> JsonValue {
    obj(vec![
        ("task", s("the import saga")),
        ("models", arr(vec![s(PRIME), s(SHADOW)])),
        ("wins_target", i(target)),
        ("races", arr(races)),
    ])
}

fn loaded(data: &JsonValue) -> Series {
    let (series, defects) = load_series_text(&dumps(data, None, true));
    assert_eq!(defects, Vec::<String>::new());
    series.expect("a clean series")
}

fn winner_model(race: &Race, winner: Option<usize>) -> Option<String> {
    winner.map(|index| race.attempts[index].model.display())
}

fn race_line(series: &Series, index: usize) -> String {
    flip_report(series)[index + 2].clone()
}

fn closing_line(series: &Series) -> String {
    flip_report(series).last().expect("a closing line").clone()
}

#[test]
fn the_handicap_series_reading_is_hand_checked() {
    let (series, defects) = load_series_file(&examples().join("handicap-series.json"));
    assert_eq!(defects, Vec::<String>::new());
    let series = series.expect("the handicap series");
    let winners: Vec<(Option<String>, Option<String>)> = series
        .races
        .iter()
        .map(|race| {
            (
                winner_model(race, scratch_winner(race)),
                winner_model(race, decide_winner(race).0),
            )
        })
        .collect();
    assert_eq!(
        winners,
        vec![
            (Some(PRIME.to_string()), Some(PRIME.to_string())),
            (Some(PRIME.to_string()), Some(SHADOW.to_string())),
            (Some(SHADOW.to_string()), Some(SHADOW.to_string())),
        ]
    );
    assert_eq!(
        flip_report(&series),
        vec![
            "series the ledger import season",
            "prime-1 against shadow-1, first to 2 wins",
            "race 1, ledger import one, scratch prime-1, adjusted prime-1, held",
            "race 2, balance the ledger import, scratch prime-1, adjusted shadow-1, flipped",
            "race 3, ledger import three, scratch shadow-1, adjusted shadow-1, held",
            "flips, 3 races, 1 flipped, 2 held, 0 no results",
        ]
    );
}

#[test]
fn a_plain_adjustment_flip_stands_on_the_figures() {
    let flipped = loaded(&series(
        vec![race(
            "a task",
            green(PRIME, i(100), None),
            green(SHADOW, i(130), Some(i(-40))),
        )],
        1,
    ));
    assert_eq!(
        race_line(&flipped, 0),
        "race 1, a task, scratch prime-1, adjusted shadow-1, flipped"
    );
    assert_eq!(
        closing_line(&flipped),
        "flips, 1 races, 1 flipped, 0 held, 0 no results"
    );
    // The same adjustment on the leading finisher deepens the
    // lead instead of turning the race, and the race stands
    // held, so the reading follows the figures and not the
    // mere presence of an adjustment.
    let held = loaded(&series(
        vec![race(
            "a task",
            green(PRIME, i(100), Some(i(-10))),
            green(SHADOW, i(130), None),
        )],
        1,
    ));
    assert_eq!(
        race_line(&held, 0),
        "race 1, a task, scratch prime-1, adjusted prime-1, held"
    );
}

#[test]
fn an_adjustment_into_the_band_flips_the_race_through_its_entry() {
    let value = set(
        &race(
            "a task",
            green(PRIME, i(100), None),
            green(SHADOW, i(110), Some(i(-15))),
        ),
        "adjudication",
        entry(SHADOW),
    );
    let series = loaded(&series(vec![value], 1));
    let race = &series.races[0];
    // The recorded figures read 100 against 110, outside the
    // band, and the adjusted figures read 100 against 95,
    // inside it, so the flip is owed to the band and its
    // entry and not to the finishes alone.
    let mut unadjusted = race.clone();
    for attempt in &mut unadjusted.attempts {
        attempt.adjustment = None;
    }
    assert!(!is_close_call(&unadjusted));
    assert!(is_close_call(race));
    assert_eq!(
        race_line(&series, 0),
        "race 1, a task, scratch prime-1, adjusted shadow-1, flipped"
    );
}

#[test]
fn the_band_race_without_an_entry_reads_no_result() {
    let series = loaded(&series(
        vec![race(
            "a task",
            green(PRIME, i(100), None),
            green(SHADOW, i(110), Some(i(-15))),
        )],
        1,
    ));
    // The adjusted reading stands in the band with no entry
    // to decide it, so the race reads no result, and a
    // reading that fell back to the finishes would call it
    // held and fail here.
    assert_eq!(
        race_line(&series, 0),
        "race 1, a task, scratch prime-1, adjusted none, no result"
    );
    assert_eq!(
        closing_line(&series),
        "flips, 1 races, 0 flipped, 0 held, 1 no results"
    );
}

#[test]
fn an_entry_for_the_scratch_winner_holds_the_band_race() {
    let value = set(
        &race(
            "a task",
            green(PRIME, i(100), None),
            green(SHADOW, i(110), Some(i(-15))),
        ),
        "adjudication",
        entry(PRIME),
    );
    let series = loaded(&series(vec![value], 1));
    // The adjusted finishes crossed, 95 against 100, but the
    // entry decided for prime-1, so the race stands held. A
    // reading that compared finishes alone would call it
    // flipped.
    assert_eq!(
        race_line(&series, 0),
        "race 1, a task, scratch prime-1, adjusted prime-1, held"
    );
}

#[test]
fn an_exact_adjusted_tie_goes_to_the_first_attempt_in_the_record() {
    let held = loaded(&series(
        vec![race(
            "a task",
            green(PRIME, i(100), None),
            green(SHADOW, i(120), Some(i(-20))),
        )],
        1,
    ));
    // The adjusted figures tie at 100 and the tie rule of
    // DH-2 gives the race to prime-1, which stands first in
    // the record, so the race stands held and not no result.
    assert_eq!(
        race_line(&held, 0),
        "race 1, a task, scratch prime-1, adjusted prime-1, held"
    );
    let flipped = loaded(&series(
        vec![race(
            "a task",
            green(SHADOW, i(120), Some(i(-20))),
            green(PRIME, i(100), None),
        )],
        1,
    ));
    // The same tie with the record reversed gives the
    // adjusted race to shadow-1, which stands first now, and
    // the race flips.
    assert_eq!(
        race_line(&flipped, 0),
        "race 1, a task, scratch prime-1, adjusted shadow-1, flipped"
    );
}

#[test]
fn an_exact_scratch_tie_goes_to_the_first_attempt_in_the_record() {
    let series = loaded(&series(
        vec![race(
            "a task",
            green(PRIME, i(100), None),
            green(SHADOW, i(100), Some(i(-10))),
        )],
        1,
    ));
    // The recorded figures tie at 100 and the tie rule gives
    // the scratch race to prime-1, while the adjustment moves
    // shadow-1 to 90 and wins the adjusted race outright.
    assert_eq!(
        race_line(&series, 0),
        "race 1, a task, scratch prime-1, adjusted shadow-1, flipped"
    );
}

#[test]
fn a_race_with_one_green_finisher_stands_held() {
    let series = loaded(&series(
        vec![race(
            "a task",
            green(PRIME, i(100), None),
            attempt(SHADOW, "unfinished", None, None),
        )],
        1,
    ));
    assert_eq!(
        race_line(&series, 0),
        "race 1, a task, scratch prime-1, adjusted prime-1, held"
    );
}

#[test]
fn a_race_with_no_green_attempt_reads_no_result() {
    let series = loaded(&series(
        vec![
            race(
                "a task",
                attempt(PRIME, "red", Some(i(100)), None),
                attempt(SHADOW, "red", Some(i(140)), None),
            ),
            race(
                "another task",
                attempt(PRIME, "unfinished", None, None),
                attempt(SHADOW, "red", Some(i(130)), None),
            ),
        ],
        1,
    ));
    assert_eq!(
        race_line(&series, 0),
        "race 1, a task, scratch none, adjusted none, no result"
    );
    assert_eq!(
        race_line(&series, 1),
        "race 2, another task, scratch none, adjusted none, no result"
    );
    assert_eq!(
        closing_line(&series),
        "flips, 2 races, 0 flipped, 0 held, 2 no results"
    );
}

#[test]
fn a_divided_panel_stands_in_the_scratch_reading() {
    let panel = arr(vec![
        obj(vec![
            ("adjudicator", s("judge-1")),
            ("chosen", s(PRIME)),
            ("confidence", f(0.9)),
            ("reason", s("the steadier diff")),
        ]),
        obj(vec![
            ("adjudicator", s("judge-2")),
            ("chosen", s(SHADOW)),
            ("confidence", f(0.9)),
            ("reason", s("the quicker recovery")),
        ]),
    ]);
    let value = set(
        &race(
            "a task",
            green(PRIME, i(100), None),
            green(SHADOW, i(104), Some(i(20))),
        ),
        "panel",
        panel,
    );
    let series = loaded(&series(vec![value], 1));
    // The recorded figures read 100 against 104, inside the
    // band, and the panel divided, so no scratch winner
    // stands. The adjusted figures read 100 against 124,
    // outside the band, and prime-1 wins outright. A scratch
    // reading that compared finishes alone would name
    // prime-1 the scratch winner and call the race held.
    assert_eq!(
        race_line(&series, 0),
        "race 1, a task, scratch none, adjusted prime-1, no result"
    );
}

#[test]
fn a_race_beyond_the_decision_stands_in_the_flip_report() {
    let series = loaded(&series(
        vec![
            race(
                "the decider",
                green(PRIME, i(100), None),
                attempt(SHADOW, "red", Some(i(130)), None),
            ),
            race(
                "the late race",
                green(PRIME, i(100), None),
                green(SHADOW, i(130), Some(i(-40))),
            ),
        ],
        1,
    ));
    let report = series_report(&series);
    assert!(report
        .iter()
        .any(|line| line.contains("race 2, the late race, beyond the decision")));
    assert_eq!(
        race_line(&series, 1),
        "race 2, the late race, scratch prime-1, adjusted shadow-1, flipped"
    );
    assert_eq!(
        closing_line(&series),
        "flips, 2 races, 1 flipped, 1 held, 0 no results"
    );
}

#[test]
fn an_empty_series_renders_its_opening_and_zero_counts() {
    let series = loaded(&series(vec![], 2));
    assert_eq!(
        flip_report(&series),
        vec![
            "series the import saga",
            "prime-1 against shadow-1, first to 2 wins",
            "flips, 0 races, 0 flipped, 0 held, 0 no results",
        ]
    );
}

#[test]
fn the_adjusted_winners_reconcile_with_the_series_report() {
    let band_held = set(
        &race(
            "the band race",
            green(PRIME, i(100), None),
            green(SHADOW, i(110), Some(i(-15))),
        ),
        "adjudication",
        entry(PRIME),
    );
    let series = loaded(&series(
        vec![
            race(
                "the turned race",
                green(PRIME, i(100), None),
                green(SHADOW, i(130), Some(i(-40))),
            ),
            band_held,
            race(
                "the tied race",
                green(PRIME, i(100), None),
                green(SHADOW, i(120), Some(i(-20))),
            ),
            race(
                "the silent race",
                attempt(PRIME, "red", Some(i(100)), None),
                attempt(SHADOW, "red", Some(i(140)), None),
            ),
            race(
                "the single race",
                green(PRIME, i(100), None),
                attempt(SHADOW, "unfinished", None, None),
            ),
        ],
        5,
    ));
    let (series_file, defects) = load_series_file(&examples().join("handicap-series.json"));
    assert_eq!(defects, Vec::<String>::new());
    let example = series_file.expect("the handicap series");
    for candidate in [&series, &example] {
        let report = series_report(candidate);
        let flips = flip_report(candidate);
        for (index, race) in candidate.races.iter().enumerate() {
            // The series report renders a tally line after each
            // race line, so the race lines stand at every second
            // index from the third line. Neither candidate holds
            // a race beyond its decision, so every race line
            // names its winner or its no result.
            let line = &report[index * 2 + 2];
            assert!(!line.contains("beyond the decision"), "{line}");
            let expected = if line.contains(", no result") {
                None
            } else {
                Some(
                    [PRIME, SHADOW]
                        .iter()
                        .find(|model| line.contains(&format!(", {model} wins")))
                        .unwrap_or_else(|| panic!("an unnamed winner in {line}"))
                        .to_string(),
                )
            };
            assert_eq!(
                winner_model(race, decide_winner(race).0),
                expected,
                "{line}"
            );
            let flip_line = &flips[index + 2];
            let adjusted_field = flip_line
                .split(", adjusted ")
                .nth(1)
                .expect("an adjusted field")
                .split(',')
                .next()
                .expect("an adjusted winner");
            assert_eq!(
                adjusted_field,
                expected.as_deref().unwrap_or("none"),
                "{flip_line}"
            );
        }
    }
}

#[test]
fn the_flip_command_prints_the_report_for_a_series_file() {
    let directory = temp_dir("flip-command");
    let file = directory.join("series.json");
    let data = series(
        vec![race(
            "a task",
            green(PRIME, i(100), None),
            green(SHADOW, i(130), Some(i(-40))),
        )],
        1,
    );
    std::fs::write(&file, dumps(&data, None, true)).expect("the series file writes");
    let (stdout, stderr, code) = run_cli(&["flip", file.to_str().expect("a path")]);
    assert_eq!(code, 0);
    assert_eq!(stderr, "");
    assert_eq!(
        stdout,
        "series the import saga\n\
         prime-1 against shadow-1, first to 1 wins\n\
         race 1, a task, scratch prime-1, adjusted shadow-1, flipped\n\
         flips, 1 races, 1 flipped, 0 held, 0 no results\n"
    );
}

#[test]
fn the_flip_command_refuses_a_malformed_series_whole() {
    let directory = temp_dir("flip-malformed");
    let file = directory.join("series.json");
    let data = series(
        vec![race(
            "a task",
            green(PRIME, i(100), None),
            green("rogue-1", i(130), None),
        )],
        1,
    );
    std::fs::write(&file, dumps(&data, None, true)).expect("the series file writes");
    let (stdout, stderr, code) = run_cli(&["flip", file.to_str().expect("a path")]);
    assert_eq!(code, 1);
    assert_eq!(stderr, "");
    assert_eq!(
        stdout,
        "invalid series file\ndefect race 0 names a model outside the series\n"
    );
    assert!(!stdout.contains("flips,"));
}

#[test]
fn the_flip_command_names_a_missing_file() {
    let missing: PathBuf = temp_dir("flip-missing").join("absent.json");
    let (stdout, stderr, code) = run_cli(&["flip", missing.to_str().expect("a path")]);
    assert_eq!(code, 1);
    assert_eq!(stderr, "");
    assert_eq!(stdout, "invalid series file\ndefect file cannot be read\n");
}

#[test]
fn the_flip_reading_writes_nothing() {
    let directory = temp_dir("flip-writes-nothing");
    let file = directory.join("series.json");
    let data = series(
        vec![race(
            "a task",
            green(PRIME, i(100), None),
            green(SHADOW, i(130), Some(i(-40))),
        )],
        1,
    );
    std::fs::write(&file, dumps(&data, None, true)).expect("the series file writes");
    let before: Vec<(String, Vec<u8>)> = std::fs::read_dir(&directory)
        .expect("the directory reads")
        .map(|entry| {
            let entry = entry.expect("an entry");
            (
                entry.file_name().to_string_lossy().into_owned(),
                std::fs::read(entry.path()).expect("the file reads"),
            )
        })
        .collect();
    let (_, _, code) = run_cli(&["flip", file.to_str().expect("a path")]);
    assert_eq!(code, 0);
    let after: Vec<(String, Vec<u8>)> = std::fs::read_dir(&directory)
        .expect("the directory reads")
        .map(|entry| {
            let entry = entry.expect("an entry");
            (
                entry.file_name().to_string_lossy().into_owned(),
                std::fs::read(entry.path()).expect("the file reads"),
            )
        })
        .collect();
    assert_eq!(before, after);
}
