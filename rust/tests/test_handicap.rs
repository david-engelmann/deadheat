//! Pins for DH-11 and DH-12, the handicap and the adjusted
//! finish, in the Round 5 section of CONTRACT.md.
//!
//! Each pin is built so it fails under the negation of the rule
//! it names. The winner and band pins stand in pairs, the same
//! race with and without its adjustments, so a reading that
//! ignored the adjustments or read the recorded figures where
//! the adjusted ones govern fails one side of the pair. The
//! closing sweep holds every standing golden byte for byte
//! over the races carrying no adjustment.

mod shared;

use std::path::{Path, PathBuf};

use deadheat::json::JsonValue;
use deadheat::panels::join_panel_records;
use deadheat::pynum::Num;
use deadheat::races::{
    adjusted_finish, close_call_report, decide_winner, empty_standings, format_seconds_fraction,
    funeral, is_close_call, load_race_data, load_race_file, load_race_text, race_report,
    race_to_json, standings_table, update_standings, Race,
};
use deadheat::seats::attempt_summary;
use deadheat::series::{load_series_file, load_series_text, series_report};
use shared::*;

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

fn race(first: JsonValue, second: JsonValue) -> JsonValue {
    obj(vec![
        ("task", s("a task")),
        ("attempts", arr(vec![first, second])),
    ])
}

fn load(value: &JsonValue) -> Race {
    let (race, defects) = load_race_data(value);
    assert_eq!(defects, Vec::<String>::new());
    race.expect("a race")
}

fn defects_of(value: &JsonValue) -> Vec<String> {
    let (race, defects) = load_race_data(value);
    assert!(race.is_none());
    defects
}

fn report_text(race: &Race) -> String {
    race_report(race).join("\n")
}

#[test]
fn an_adjustment_loads_in_each_of_its_shapes() {
    for (figure, rendered) in [
        (i(-30), "70"),
        (f(-2.5), "97.5"),
        (i(0), "100"),
        (f(12.25), "112.25"),
    ] {
        let value = race(
            green("prime-1", i(100), Some(figure)),
            green("shadow-1", i(200), None),
        );
        let loaded = load(&value);
        let adjusted = adjusted_finish(&loaded.attempts[0]).expect("an adjusted finish");
        assert_eq!(format_seconds_fraction(&adjusted), rendered);
    }
    let value = race(
        green("prime-1", i(100), None),
        green("shadow-1", i(200), None),
    );
    let loaded = load(&value);
    assert!(loaded.attempts[0].adjustment.is_none());
    let adjusted = adjusted_finish(&loaded.attempts[0]).expect("an adjusted finish");
    assert_eq!(format_seconds_fraction(&adjusted), "100");
    let unfinished = load(&race(
        attempt("prime-1", "unfinished", None, None),
        green("shadow-1", i(200), None),
    ));
    assert!(adjusted_finish(&unfinished.attempts[0]).is_none());
}

#[test]
fn an_adjustment_of_the_wrong_type_is_refused() {
    for bad in [s("five"), JsonValue::Null, JsonValue::Bool(true)] {
        let value = race(
            green("prime-1", i(100), Some(bad)),
            green("shadow-1", i(200), None),
        );
        assert_eq!(
            defects_of(&value),
            vec!["attempt 0 field adjustment has the wrong type".to_string()]
        );
    }
    let value = race(
        green("prime-1", i(100), None),
        green("shadow-1", i(200), Some(s("five"))),
    );
    assert_eq!(
        defects_of(&value),
        vec!["attempt 1 field adjustment has the wrong type".to_string()]
    );
}

#[test]
fn a_non_finite_adjustment_is_refused() {
    for bad in [f(f64::INFINITY), f(f64::NAN), bigint(&"1".repeat(400))] {
        let value = race(
            green("prime-1", i(100), Some(bad)),
            green("shadow-1", i(200), None),
        );
        assert_eq!(
            defects_of(&value),
            vec!["attempt 0 field adjustment is not a finite number".to_string()]
        );
    }
    for literal in ["1e400", &"9".repeat(400)] {
        let text = format!(
            "{{\"task\": \"a task\", \"attempts\": [{{\"model\": \"prime-1\", \"environment\": \"lane\", \"result\": \"green\", \"tokens\": 1, \"patch\": \"\", \"finished_at\": 100, \"adjustment\": {literal}}}, {{\"model\": \"shadow-1\", \"environment\": \"lane\", \"result\": \"green\", \"tokens\": 1, \"patch\": \"\", \"finished_at\": 200}}]}}"
        );
        let (race, defects) = load_race_text(&text);
        assert!(race.is_none());
        assert_eq!(
            defects,
            vec!["attempt 0 field adjustment is not a finite number".to_string()],
            "{literal}"
        );
    }
}

#[test]
fn an_adjustment_on_an_unfinished_attempt_is_refused() {
    let value = race(
        attempt("prime-1", "unfinished", None, Some(i(5))),
        green("shadow-1", i(200), None),
    );
    assert_eq!(
        defects_of(&value),
        vec!["attempt 0 field adjustment is present but the suite did not finish".to_string()]
    );
}

#[test]
fn an_adjustment_taking_the_finish_below_zero_is_refused() {
    let value = race(
        green("prime-1", i(30), Some(i(-31))),
        green("shadow-1", i(200), None),
    );
    assert_eq!(
        defects_of(&value),
        vec!["attempt 0 field adjustment takes the finish below zero".to_string()]
    );
    let value = race(
        green("prime-1", f(30.5), Some(f(-30.75))),
        green("shadow-1", i(200), None),
    );
    assert_eq!(
        defects_of(&value),
        vec!["attempt 0 field adjustment takes the finish below zero".to_string()]
    );
}

#[test]
fn an_adjustment_taking_the_finish_exactly_to_zero_stands() {
    let value = race(
        green("prime-1", i(30), Some(i(-30))),
        attempt("shadow-1", "red", Some(i(50)), None),
    );
    let loaded = load(&value);
    let adjusted = adjusted_finish(&loaded.attempts[0]).expect("an adjusted finish");
    assert_eq!(format_seconds_fraction(&adjusted), "0");
    assert_eq!(decide_winner(&loaded), (Some(0), Some(1)));
    assert!(report_text(&loaded)
        .contains("green in 30 seconds, adjustment -30 seconds, adjusted finish 0 seconds"));
}

#[test]
fn the_adjustment_reverses_the_scratch_outcome() {
    let handicapped = load(&race(
        green("prime-1", i(100), None),
        green("shadow-1", i(120), Some(i(-30))),
    ));
    assert_eq!(decide_winner(&handicapped), (Some(1), Some(0)));
    assert!(report_text(&handicapped).contains("verdict, shadow-1 wins"));
    let scratch = load(&race(
        green("prime-1", i(100), None),
        green("shadow-1", i(120), None),
    ));
    assert_eq!(decide_winner(&scratch), (Some(0), Some(1)));
    assert!(report_text(&scratch).contains("verdict, prime-1 wins"));
}

#[test]
fn an_exact_adjusted_tie_goes_to_the_first_attempt_in_the_record() {
    let value = race(
        green("shadow-1", i(110), Some(i(-10))),
        green("prime-1", i(100), None),
    );
    let loaded = load(&value);
    assert!(!is_close_call(&loaded));
    assert_eq!(decide_winner(&loaded), (Some(0), Some(1)));
    let closecall = close_call_report(&loaded).join("\n");
    assert!(closecall.contains(
        "outside the band, the finish times are tied and the tie goes to the first attempt in the record"
    ));
    assert!(closecall.contains("verdict, shadow-1 wins"));
    let scratch = load(&race(
        green("shadow-1", i(110), None),
        green("prime-1", i(100), None),
    ));
    assert_eq!(decide_winner(&scratch), (Some(1), Some(0)));
}

#[test]
fn an_adjustment_moves_a_race_into_the_band() {
    let handicapped = load(&race(
        green("prime-1", i(100), None),
        green("shadow-1", i(120), Some(i(-16))),
    ));
    assert!(is_close_call(&handicapped));
    assert_eq!(decide_winner(&handicapped), (None, None));
    assert!(report_text(&handicapped).contains("close call, no adjudication on record"));
    let closecall = close_call_report(&handicapped).join("\n");
    assert!(closecall.contains(
        "in the band, both attempts went green and the gap of 4 seconds is within five percent of the slower finish of 104 seconds"
    ));
    let scratch = load(&race(
        green("prime-1", i(100), None),
        green("shadow-1", i(120), None),
    ));
    assert!(!is_close_call(&scratch));
    assert_eq!(decide_winner(&scratch), (Some(0), Some(1)));
}

#[test]
fn an_adjustment_moves_a_race_out_of_the_band() {
    let mut handicapped = race(
        green("prime-1", i(100), None),
        green("shadow-1", i(104), Some(i(10))),
    );
    handicapped = set(
        &handicapped,
        "adjudication",
        obj(vec![
            ("adjudicator", s("judge-2")),
            ("chosen", s("shadow-1")),
            ("confidence", f(0.9)),
            ("reason", s("the steadier diff")),
        ]),
    );
    let loaded = load(&handicapped);
    assert!(!is_close_call(&loaded));
    assert_eq!(decide_winner(&loaded), (Some(0), Some(1)));
    assert!(report_text(&loaded).contains(
        "adjudication by judge-2 not needed, the race stands outside the close call band"
    ));
    let scratch = load(&race(
        green("prime-1", i(100), None),
        green("shadow-1", i(104), None),
    ));
    assert!(is_close_call(&scratch));
    assert_eq!(decide_winner(&scratch), (None, None));
}

#[test]
fn the_band_edge_stands_on_adjusted_figures() {
    let at_edge = load(&race(
        green("prime-1", i(100), Some(i(-5))),
        green("shadow-1", i(100), None),
    ));
    assert!(is_close_call(&at_edge));
    let past_edge = load(&race(
        green("prime-1", i(100), Some(f(-5.25))),
        green("shadow-1", i(100), None),
    ));
    assert!(!is_close_call(&past_edge));
    assert_eq!(decide_winner(&past_edge), (Some(0), Some(1)));
}

#[test]
fn a_zero_adjustment_renders_its_clause() {
    let loaded = load(&race(
        green("prime-1", i(100), Some(i(0))),
        attempt("shadow-1", "red", Some(i(130)), None),
    ));
    let lines = race_report(&loaded);
    assert!(lines.contains(
        &"prime-1 in lane, green in 100 seconds, adjustment 0 seconds, adjusted finish 100 seconds, 1000 tokens"
            .to_string()
    ));
    assert_eq!(decide_winner(&loaded), (Some(0), Some(1)));
    let funeral_lines = funeral(&loaded);
    assert!(
        funeral_lines.contains(&"margin 30 seconds between the two adjusted finishes".to_string())
    );
}

#[test]
fn negative_adjustments_render_with_a_minus_over_the_absolute_value() {
    let loaded = load(&race(
        green("prime-1", i(100), Some(f(-0.5))),
        attempt("shadow-1", "red", Some(i(130)), None),
    ));
    assert!(report_text(&loaded).contains("adjustment -0.5 seconds, adjusted finish 99.5 seconds"));
    let loaded = load(&race(
        green("prime-1", i(100), Some(f(-0.25))),
        attempt("shadow-1", "red", Some(i(130)), None),
    ));
    assert!(
        report_text(&loaded).contains("adjustment -0.25 seconds, adjusted finish 99.75 seconds")
    );
    let loaded = load(&race(
        green("prime-1", i(100), Some(f(-0.000000476837158203125))),
        attempt("shadow-1", "red", Some(i(130)), None),
    ));
    let text = report_text(&loaded);
    assert!(
        text.contains("adjustment -0.000000476837158203125 seconds"),
        "{text}"
    );
}

#[test]
fn the_margin_reads_adjusted_finishes_when_an_adjustment_stands() {
    let loaded = load(&race(
        attempt("prime-1", "red", Some(i(100)), Some(i(-10))),
        green("shadow-1", i(50), None),
    ));
    let lines = funeral(&loaded);
    assert!(
        lines.contains(&"margin 40 seconds between the two adjusted finishes".to_string()),
        "{lines:?}"
    );
    assert!(lines.contains(
        &"loser prime-1 in lane went red in 100 seconds, adjustment -10 seconds, adjusted finish 90 seconds"
            .to_string()
    ));
    let scratch = load(&race(
        attempt("prime-1", "red", Some(i(100)), None),
        green("shadow-1", i(50), None),
    ));
    let lines = funeral(&scratch);
    assert!(
        lines.contains(&"margin 50 seconds between the two finishes".to_string()),
        "{lines:?}"
    );
}

#[test]
fn the_standings_count_a_handicapped_race_by_its_adjusted_winner() {
    let loaded = load(&race(
        green("prime-1", i(100), None),
        green("shadow-1", i(120), Some(i(-30))),
    ));
    let standings = update_standings(&empty_standings(), &loaded);
    let table = standings_table(&standings).join("\n");
    assert!(
        table.contains("shadow-1, 1 wins, 0 losses, 0 no results, 1000 tokens, 0 unreported"),
        "{table}"
    );
    assert!(
        table.contains("prime-1, 0 wins, 1 losses, 0 no results, 1000 tokens, 0 unreported"),
        "{table}"
    );
}

#[test]
fn a_panel_decides_a_race_the_adjustments_put_in_the_band() {
    let mut value = race(
        green("prime-1", i(100), None),
        green("shadow-1", i(130), Some(i(-27))),
    );
    value = set(
        &value,
        "panel",
        arr(vec![obj(vec![
            ("adjudicator", s("judge-2")),
            ("chosen", s("shadow-1")),
            ("confidence", f(0.9)),
            ("reason", s("the steadier diff")),
        ])]),
    );
    let loaded = load(&value);
    assert!(is_close_call(&loaded));
    assert_eq!(decide_winner(&loaded), (Some(1), Some(0)));
    assert!(report_text(&loaded).contains(
        "close call, panel, adjudication by judge-2 chose shadow-1 at confidence 0.9 and the other judge is absent, the panel follows judge-2"
    ));
    let scratch = load(&race(
        green("prime-1", i(100), None),
        green("shadow-1", i(130), None),
    ));
    assert!(!is_close_call(&scratch));
}

#[test]
fn the_join_carries_adjustments_and_refuses_a_mismatch() {
    let entry = |judge: &str| {
        obj(vec![
            ("adjudicator", s(judge)),
            ("chosen", s("shadow-1")),
            ("confidence", f(0.9)),
            ("reason", s("the steadier diff")),
        ])
    };
    let base = race(
        green("prime-1", i(100), None),
        green("shadow-1", i(120), Some(i(-30))),
    );
    let first = load(&set(&base, "adjudication", entry("judge-2")));
    let second = load(&set(&base, "adjudication", entry("judge-3")));
    let (joined, defects) = join_panel_records(&first, &second);
    assert_eq!(defects, Vec::<String>::new());
    let joined = joined.expect("a joined record");
    assert_eq!(joined.attempts[1].adjustment, Some(Num::from_i64(-30)));
    let rendered = race_to_json(&joined);
    assert_eq!(
        get_path(&rendered, &["attempts", "1", "adjustment"]),
        Some(&JsonValue::Num(Num::from_i64(-30)))
    );
    let (reloaded, defects) = load_race_data(&rendered);
    assert_eq!(defects, Vec::<String>::new());
    assert_eq!(reloaded.expect("a reloaded race"), joined);
    let second_with_entry = load(&set(
        &race(
            green("prime-1", i(100), None),
            green("shadow-1", i(120), Some(i(-25))),
        ),
        "adjudication",
        entry("judge-3"),
    ));
    let (joined, defects) = join_panel_records(&first, &second_with_entry);
    assert!(joined.is_none());
    assert_eq!(
        defects,
        vec!["the two records hold different attempts".to_string()]
    );
}

#[test]
fn the_seat_summary_keeps_the_dh7_form_over_a_handicapped_race() {
    let loaded = load(&race(
        green("prime-1", i(100), None),
        green("shadow-1", i(130), Some(i(-27))),
    ));
    let summary = attempt_summary(&loaded);
    assert!(
        summary.contains("attempt, model shadow-1, environment lane, result green, finished in 130 seconds, 1000 tokens"),
        "{summary}"
    );
    assert!(!summary.contains("adjustment"), "{summary}");
}

#[test]
fn a_series_counts_the_handicapped_race_by_its_adjusted_winner() {
    let text = "{\"task\": \"a season\", \"models\": [\"prime-1\", \"shadow-1\"], \"wins_target\": 1, \"races\": [{\"task\": \"the handicapped race\", \"attempts\": [{\"model\": \"prime-1\", \"environment\": \"lane\", \"result\": \"green\", \"tokens\": 10, \"patch\": \"\", \"finished_at\": 100}, {\"model\": \"shadow-1\", \"environment\": \"lane\", \"result\": \"green\", \"tokens\": 10, \"patch\": \"\", \"finished_at\": 120, \"adjustment\": -30}]}]}";
    let (series, defects) = load_series_text(text);
    assert_eq!(defects, Vec::<String>::new());
    let rendered = series_report(&series.expect("a series")).join("\n");
    assert!(
        rendered.contains("race 1, the handicapped race, shadow-1 wins"),
        "{rendered}"
    );
    assert!(rendered.contains("series winner, shadow-1"), "{rendered}");
}

#[test]
fn the_cli_refuses_an_adjustment_on_an_unfinished_attempt() {
    let dir = temp_dir("handicap-cli");
    let path = dir.join("race.json");
    let text = "{\"task\": \"a task\", \"attempts\": [{\"model\": \"prime-1\", \"environment\": \"lane\", \"result\": \"unfinished\", \"tokens\": 1, \"patch\": \"\", \"adjustment\": 5}, {\"model\": \"shadow-1\", \"environment\": \"lane\", \"result\": \"green\", \"tokens\": 1, \"patch\": \"\", \"finished_at\": 200}]}";
    std::fs::write(&path, text).expect("a written race");
    let (out, _, code) = run_cli(&["race", path.to_str().expect("utf8")]);
    assert_eq!(code, 1);
    assert_eq!(
        out,
        "invalid recorded race\ndefect attempt 0 field adjustment is present but the suite did not finish\n"
    );
}

fn collect_goldens(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
        .expect("a readable directory")
        .map(|entry| entry.expect("an entry").path())
        .collect();
    entries.sort();
    for entry in entries {
        if entry.is_dir() {
            collect_goldens(&entry, out);
        } else if entry.extension().is_some_and(|ext| ext == "txt") {
            out.push(entry);
        }
    }
}

fn race_is_unadjusted(race: &Race) -> bool {
    race.attempts
        .iter()
        .all(|attempt| attempt.adjustment.is_none())
}

#[test]
fn every_standing_golden_still_matches_byte_for_byte() {
    let mut goldens = Vec::new();
    collect_goldens(&examples(), &mut goldens);
    let mut unadjusted = 0usize;
    for golden_path in &goldens {
        let golden = std::fs::read_to_string(golden_path).expect("a golden");
        let name = golden_path
            .file_name()
            .expect("a file name")
            .to_string_lossy()
            .into_owned();
        if let Some(stem) = name.strip_suffix("-closecall.txt") {
            let source = golden_path.with_file_name(format!("{stem}.json"));
            let (race, defects) = load_race_file(&source);
            assert_eq!(defects, Vec::<String>::new(), "{name}");
            let race = race.expect("a race");
            if race_is_unadjusted(&race) {
                unadjusted += 1;
            }
            let rendered = close_call_report(&race).join("\n") + "\n";
            assert_eq!(rendered, golden, "{name}");
            continue;
        }
        // Altered at the Round 7 slice for its named mechanical
        // consequence. The flip example of DH-16 adds one golden
        // whose stem names a series rather than a race, so the
        // sweep reads it through the series loader and the flip
        // report before the generic race reading can misread its
        // source, and the golden count below rises by one. The
        // unadjusted count does not rise, because the handicap
        // series the flip golden reads carries an adjustment in
        // its second race.
        if let Some(stem) = name.strip_suffix("-flip.txt") {
            let source = golden_path.with_file_name(format!("{stem}.json"));
            let (series, defects) = load_series_file(&source);
            assert_eq!(defects, Vec::<String>::new(), "{name}");
            let series = series.expect("a series");
            if series.races.iter().all(race_is_unadjusted) {
                unadjusted += 1;
            }
            let rendered = deadheat::flips::flip_report(&series).join("\n") + "\n";
            assert_eq!(rendered, golden, "{name}");
            continue;
        }
        // Altered at the Round 8 slice for its named mechanical
        // consequence. The totals example of DH-18 adds one golden
        // whose stem names a championship rather than a race, so
        // the sweep reads it through the championship loader and
        // the totals report before the generic race reading can
        // misread its source, and the two counts below rise by one
        // each, the golden count for the new golden and the
        // unadjusted count because every race under the two
        // recorded series stands unadjusted.
        if let Some(stem) = name.strip_suffix("-totals.txt") {
            let source = golden_path.with_file_name(format!("{stem}.json"));
            let (championship, defects) = deadheat::championship::load_championship_file(&source);
            assert_eq!(defects, Vec::<String>::new(), "{name}");
            let championship = championship.expect("a championship");
            if championship
                .series
                .iter()
                .flat_map(|series| series.races.iter())
                .all(race_is_unadjusted)
            {
                unadjusted += 1;
            }
            let rendered = deadheat::totals::totals_report(&championship).join("\n") + "\n";
            assert_eq!(rendered, golden, "{name}");
            continue;
        }
        // Altered at the Round 9 slice for its named mechanical
        // consequence. The pairing example of DH-20 adds one
        // golden whose stem names a championship rather than a
        // race, so the sweep reads it through the championship
        // loader and the pairing report before the generic race
        // reading can misread its source, and the two counts
        // below rise by one each, the golden count for the new
        // golden and the unadjusted count because every race
        // under the two recorded series stands unadjusted.
        if let Some(stem) = name.strip_suffix("-pairings.txt") {
            let source = golden_path.with_file_name(format!("{stem}.json"));
            let (championship, defects) = deadheat::championship::load_championship_file(&source);
            assert_eq!(defects, Vec::<String>::new(), "{name}");
            let championship = championship.expect("a championship");
            if championship
                .series
                .iter()
                .flat_map(|series| series.races.iter())
                .all(race_is_unadjusted)
            {
                unadjusted += 1;
            }
            let rendered = deadheat::pairings::pairing_report(&championship).join("\n") + "\n";
            assert_eq!(rendered, golden, "{name}");
            continue;
        }
        // Altered at the Round 10 slice for its named mechanical
        // consequence. The comeback examples of DH-22 add two
        // goldens whose stems name championships rather than a
        // race, so the sweep reads them through the championship
        // loader and the comeback report before the generic race
        // reading can misread their sources, and the two counts
        // below rise by two each, the golden count for the two
        // new goldens and the unadjusted count because every
        // race under the two recorded series and every race of
        // the constructed series stands unadjusted.
        if let Some(stem) = name.strip_suffix("-comebacks.txt") {
            let source = golden_path.with_file_name(format!("{stem}.json"));
            let (championship, defects) = deadheat::championship::load_championship_file(&source);
            assert_eq!(defects, Vec::<String>::new(), "{name}");
            let championship = championship.expect("a championship");
            if championship
                .series
                .iter()
                .flat_map(|series| series.races.iter())
                .all(race_is_unadjusted)
            {
                unadjusted += 1;
            }
            let rendered = deadheat::comebacks::comeback_report(&championship).join("\n") + "\n";
            assert_eq!(rendered, golden, "{name}");
            continue;
        }
        // Altered at the Round 11 slice for its named mechanical
        // consequence. The distance examples of DH-24 add two
        // goldens whose stems name championships rather than a
        // race, so the sweep reads them through the championship
        // loader and the distance report before the generic race
        // reading can misread their sources, and the two counts
        // below rise by two each, the golden count for the two
        // new goldens and the unadjusted count because every
        // race under the two recorded series and every race of
        // the two constructed series stands unadjusted.
        if let Some(stem) = name.strip_suffix("-distance.txt") {
            let source = golden_path.with_file_name(format!("{stem}.json"));
            let (championship, defects) = deadheat::championship::load_championship_file(&source);
            assert_eq!(defects, Vec::<String>::new(), "{name}");
            let championship = championship.expect("a championship");
            if championship
                .series
                .iter()
                .flat_map(|series| series.races.iter())
                .all(race_is_unadjusted)
            {
                unadjusted += 1;
            }
            let rendered = deadheat::distances::distance_report(&championship).join("\n") + "\n";
            assert_eq!(rendered, golden, "{name}");
            continue;
        }
        // Altered at the Round 12 slice for its named mechanical
        // consequence. The rally examples of DH-26 add two goldens
        // whose stems name championships rather than a race, and
        // the survey championship of DH-28 adds a third over its
        // own stem, so the sweep reads them through the championship
        // loader and the rally report before the generic race
        // reading can misread their sources. The two counts below
        // rise by fourteen each this round, the three rally goldens
        // here, the five survey championship goldens the standing
        // arms and the generic championship reading already read,
        // and the six survey series goldens the generic series
        // reading already reads, the unadjusted count rising with
        // them because every race of the three championships and
        // the six series stands unadjusted.
        if let Some(stem) = name.strip_suffix("-rally.txt") {
            let source = golden_path.with_file_name(format!("{stem}.json"));
            let (championship, defects) = deadheat::championship::load_championship_file(&source);
            assert_eq!(defects, Vec::<String>::new(), "{name}");
            let championship = championship.expect("a championship");
            if championship
                .series
                .iter()
                .flat_map(|series| series.races.iter())
                .all(race_is_unadjusted)
            {
                unadjusted += 1;
            }
            let rendered = deadheat::rallies::rally_report(&championship).join("\n") + "\n";
            assert_eq!(rendered, golden, "{name}");
            continue;
        }
        // Altered at the Round 13 slice for its named mechanical
        // consequence. The deadlock examples of DH-30 add three
        // goldens whose stems name championships rather than a
        // race, and the league championship of DH-31 adds a
        // fourth over its own stem, so the sweep reads them
        // through the championship loader and the deadlock
        // report before the generic race reading can misread
        // their sources. The two counts below rise by twenty
        // one each this round, the four deadlock goldens here,
        // the constructed rally golden the rally arm above
        // reads, the six league championship goldens the
        // standing arms and the generic championship reading
        // already read, and the ten league series goldens the
        // generic series reading already reads, the unadjusted
        // count rising with them because every race of the
        // four championships and the ten series stands
        // unadjusted.
        if let Some(stem) = name.strip_suffix("-deadlock.txt") {
            let source = golden_path.with_file_name(format!("{stem}.json"));
            let (championship, defects) = deadheat::championship::load_championship_file(&source);
            assert_eq!(defects, Vec::<String>::new(), "{name}");
            let championship = championship.expect("a championship");
            if championship
                .series
                .iter()
                .flat_map(|series| series.races.iter())
                .all(race_is_unadjusted)
            {
                unadjusted += 1;
            }
            let rendered = deadheat::deadlocks::deadlock_report(&championship).join("\n") + "\n";
            assert_eq!(rendered, golden, "{name}");
            continue;
        }
        // Altered at the Round 14 slice for its named mechanical
        // consequence. The streak examples of DH-33 add four
        // goldens whose stems name championships rather than a
        // race, so the sweep reads them through the championship
        // loader and the streak report before the generic race
        // reading can misread their sources. The two counts
        // below rise by eight each this round, the four streak
        // goldens here and the four constructed sibling goldens
        // the standing distance, deadlock, rally, and comebacks
        // arms already read over the constructed streak stem,
        // the unadjusted count rising with them because every
        // race of the four championships stands unadjusted.
        if let Some(stem) = name.strip_suffix("-streak.txt") {
            let source = golden_path.with_file_name(format!("{stem}.json"));
            let (championship, defects) = deadheat::championship::load_championship_file(&source);
            assert_eq!(defects, Vec::<String>::new(), "{name}");
            let championship = championship.expect("a championship");
            if championship
                .series
                .iter()
                .flat_map(|series| series.races.iter())
                .all(race_is_unadjusted)
            {
                unadjusted += 1;
            }
            let rendered = deadheat::streaks::streak_report(&championship).join("\n") + "\n";
            assert_eq!(rendered, golden, "{name}");
            continue;
        }
        // Altered at the Round 15 slice for its named mechanical
        // consequence. The cluster examples of DH-35 add four
        // goldens whose stems name championships rather than a
        // race, so the sweep reads them through the championship
        // loader and the cluster report before the generic race
        // reading can misread their sources. The two counts
        // below rise by nine each this round, the four cluster
        // goldens here and the five constructed sibling goldens
        // the standing deadlock, streak, distance, rally, and
        // comebacks arms already read over the constructed
        // cluster stem, the unadjusted count rising with them
        // because every race of the four championships stands
        // unadjusted.
        if let Some(stem) = name.strip_suffix("-cluster.txt") {
            let source = golden_path.with_file_name(format!("{stem}.json"));
            let (championship, defects) = deadheat::championship::load_championship_file(&source);
            assert_eq!(defects, Vec::<String>::new(), "{name}");
            let championship = championship.expect("a championship");
            if championship
                .series
                .iter()
                .flat_map(|series| series.races.iter())
                .all(race_is_unadjusted)
            {
                unadjusted += 1;
            }
            let rendered = deadheat::clusters::cluster_report(&championship).join("\n") + "\n";
            assert_eq!(rendered, golden, "{name}");
            continue;
        }
        // Altered at the Round 16 slice for its named mechanical
        // consequence. The closing examples of DH-37 add four
        // goldens whose stems name championships rather than a
        // race, so the sweep reads them through the championship
        // loader and the closing report before the generic race
        // reading can misread their sources. The two counts
        // below rise by ten each this round, the four closing
        // goldens here and the six constructed sibling goldens
        // the standing streak, cluster, rally, deadlock,
        // distance, and comebacks arms already read over the
        // constructed closing stem, the unadjusted count rising
        // with them because every race of the four championships
        // stands unadjusted.
        if let Some(stem) = name.strip_suffix("-closing.txt") {
            let source = golden_path.with_file_name(format!("{stem}.json"));
            let (championship, defects) = deadheat::championship::load_championship_file(&source);
            assert_eq!(defects, Vec::<String>::new(), "{name}");
            let championship = championship.expect("a championship");
            if championship
                .series
                .iter()
                .flat_map(|series| series.races.iter())
                .all(race_is_unadjusted)
            {
                unadjusted += 1;
            }
            let rendered = deadheat::closings::closing_report(&championship).join("\n") + "\n";
            assert_eq!(rendered, golden, "{name}");
            continue;
        }
        // Altered at the Round 17 slice for its named mechanical
        // consequence. The opening examples of DH-39 add four
        // goldens whose stems name championships rather than a
        // race, so the sweep reads them through the championship
        // loader and the opening report before the generic race
        // reading can misread their sources. The two counts
        // below rise by eleven each this round, the four opening
        // goldens here and the seven constructed sibling goldens
        // the standing streak, cluster, rally, deadlock,
        // distance, comebacks, and closing arms already read over
        // the constructed opening stem, the unadjusted count
        // rising with them because every race of the four
        // championships stands unadjusted.
        if let Some(stem) = name.strip_suffix("-opening.txt") {
            let source = golden_path.with_file_name(format!("{stem}.json"));
            let (championship, defects) = deadheat::championship::load_championship_file(&source);
            assert_eq!(defects, Vec::<String>::new(), "{name}");
            let championship = championship.expect("a championship");
            if championship
                .series
                .iter()
                .flat_map(|series| series.races.iter())
                .all(race_is_unadjusted)
            {
                unadjusted += 1;
            }
            let rendered = deadheat::openings::opening_report(&championship).join("\n") + "\n";
            assert_eq!(rendered, golden, "{name}");
            continue;
        }
        // Altered at the Round 19 slice for its named mechanical
        // consequence. The turns examples of DH-41 add four
        // goldens whose stems name championships rather than a
        // race, so the sweep reads them through the championship
        // loader and the turns report before the generic race
        // reading can misread their sources. The two counts
        // below rise by twelve each this round, the four turns
        // goldens here and the eight constructed sibling goldens
        // the standing streak, cluster, rally, deadlock,
        // distance, comebacks, closing, and opening arms already
        // read over the constructed turns stem, the unadjusted
        // count rising with them because every race of the four
        // championships stands unadjusted.
        if let Some(stem) = name.strip_suffix("-turns.txt") {
            let source = golden_path.with_file_name(format!("{stem}.json"));
            let (championship, defects) = deadheat::championship::load_championship_file(&source);
            assert_eq!(defects, Vec::<String>::new(), "{name}");
            let championship = championship.expect("a championship");
            if championship
                .series
                .iter()
                .flat_map(|series| series.races.iter())
                .all(race_is_unadjusted)
            {
                unadjusted += 1;
            }
            let rendered = deadheat::turns::turns_report(&championship).join("\n") + "\n";
            assert_eq!(rendered, golden, "{name}");
            continue;
        }
        // Altered at the Round 20 slice for its named mechanical
        // consequence. The lead examples of DH-43 add four
        // goldens whose stems name championships rather than a
        // race, so the sweep reads them through the championship
        // loader and the lead report before the generic race
        // reading can misread their sources. The two counts
        // below rise by twelve each this round, the four lead
        // goldens here and the eight constructed sibling goldens
        // the standing streak, cluster, rally, deadlock,
        // distance, comebacks, closing, and opening arms already
        // read over the constructed lead stem, the unadjusted
        // count rising with them because every race of the four
        // championships stands unadjusted.
        if let Some(stem) = name.strip_suffix("-lead.txt") {
            let source = golden_path.with_file_name(format!("{stem}.json"));
            let (championship, defects) = deadheat::championship::load_championship_file(&source);
            assert_eq!(defects, Vec::<String>::new(), "{name}");
            let championship = championship.expect("a championship");
            if championship
                .series
                .iter()
                .flat_map(|series| series.races.iter())
                .all(race_is_unadjusted)
            {
                unadjusted += 1;
            }
            let rendered = deadheat::leads::lead_report(&championship).join("\n") + "\n";
            assert_eq!(rendered, golden, "{name}");
            continue;
        }

        // Altered at the Round 21 slice for its named mechanical
        // consequence. The crossings examples of DH-45 add four
        // goldens whose stems name championships rather than a
        // race, so the sweep reads them through the championship
        // loader and the crossings report before the generic race
        // reading can misread their sources. The two counts
        // below rise by twelve each this round, the four crossings
        // goldens here and the eight constructed sibling goldens
        // the standing streak, cluster, rally, deadlock,
        // distance, comebacks, closing, and opening arms already
        // read over the constructed crossings stem, the unadjusted
        // count rising with them because every race of the four
        // championships stands unadjusted.
        if let Some(stem) = name.strip_suffix("-crossings.txt") {
            let source = golden_path.with_file_name(format!("{stem}.json"));
            let (championship, defects) = deadheat::championship::load_championship_file(&source);
            assert_eq!(defects, Vec::<String>::new(), "{name}");
            let championship = championship.expect("a championship");
            if championship
                .series
                .iter()
                .flat_map(|series| series.races.iter())
                .all(race_is_unadjusted)
            {
                unadjusted += 1;
            }
            let rendered = deadheat::crossings::crossings_report(&championship).join("\n") + "\n";
            assert_eq!(rendered, golden, "{name}");
            continue;
        }
        // Altered at the Round 22 slice for its named mechanical
        // consequence. The tenure examples of DH-47 add four
        // goldens whose stems name championships rather than a
        // race, so the sweep reads them through the championship
        // loader and the tenure report before the generic race
        // reading can misread their sources. The two counts
        // below rise by twelve each this round, the four tenure
        // goldens here and the eight constructed sibling goldens
        // the standing streak, cluster, rally, deadlock,
        // distance, comebacks, closing, and opening arms already
        // read over the constructed tenure stem, the unadjusted
        // count rising with them because every race of the four
        // championships stands unadjusted.
        if let Some(stem) = name.strip_suffix("-tenure.txt") {
            let source = golden_path.with_file_name(format!("{stem}.json"));
            let (championship, defects) = deadheat::championship::load_championship_file(&source);
            assert_eq!(defects, Vec::<String>::new(), "{name}");
            let championship = championship.expect("a championship");
            if championship
                .series
                .iter()
                .flat_map(|series| series.races.iter())
                .all(race_is_unadjusted)
            {
                unadjusted += 1;
            }
            let rendered = deadheat::tenure::tenure_report(&championship).join("\n") + "\n";
            assert_eq!(rendered, golden, "{name}");
            continue;
        }
        // Altered at the Round 23 slice for its named mechanical
        // consequence. The depths examples of DH-49 add four
        // goldens whose stems name championships rather than a
        // race, so the sweep reads them through the championship
        // loader and the depths report before the generic race
        // reading can misread their sources. The two counts
        // below rise by fourteen each this round, the four depths
        // goldens here and the ten constructed sibling goldens
        // the standing streak, cluster, rally, deadlock,
        // distance, comebacks, closing, opening, crossings, and
        // tenure arms already read over the constructed depths
        // stem, the unadjusted count rising with them because
        // every race of the four championships stands
        // unadjusted.
        if let Some(stem) = name.strip_suffix("-depths.txt") {
            let source = golden_path.with_file_name(format!("{stem}.json"));
            let (championship, defects) = deadheat::championship::load_championship_file(&source);
            assert_eq!(defects, Vec::<String>::new(), "{name}");
            let championship = championship.expect("a championship");
            if championship
                .series
                .iter()
                .flat_map(|series| series.races.iter())
                .all(race_is_unadjusted)
            {
                unadjusted += 1;
            }
            let rendered = deadheat::depths::depths_report(&championship).join("\n") + "\n";
            assert_eq!(rendered, golden, "{name}");
            continue;
        }
        let source = golden_path.with_extension("json");
        let (race, _) = load_race_file(&source);
        if let Some(race) = race {
            if race_is_unadjusted(&race) {
                unadjusted += 1;
            }
            let mut lines = race_report(&race);
            lines.extend(funeral(&race));
            let rendered = lines.join("\n") + "\n";
            assert_eq!(rendered, golden, "{name}");
            continue;
        }
        let (series, _) = load_series_file(&source);
        if let Some(series) = series {
            if series.races.iter().all(race_is_unadjusted) {
                unadjusted += 1;
            }
            let rendered = series_report(&series).join("\n") + "\n";
            assert_eq!(rendered, golden, "{name}");
            continue;
        }
        // Altered at the Round 6 slice for its named mechanical
        // consequence. The championship example of DH-14 adds
        // one golden whose source loads as neither a race nor a
        // series, so the sweep reads it through the championship
        // loader before it may name a defect, and the two counts
        // below rise by one each, the golden count for the new
        // golden and the unadjusted count because every race
        // under the two recorded series stands unadjusted.
        let (championship, defects) = deadheat::championship::load_championship_file(&source);
        assert_eq!(defects, Vec::<String>::new(), "{name}");
        let championship = championship.expect("a championship");
        if championship
            .series
            .iter()
            .flat_map(|series| series.races.iter())
            .all(race_is_unadjusted)
        {
            unadjusted += 1;
        }
        let rendered = deadheat::championship::championship_report(&championship).join("\n") + "\n";
        assert_eq!(rendered, golden, "{name}");
    }
    // The counts stand at one hundred ninety
    // goldens and one hundred eighty six unadjusted
    // after the Round 23 enrolment counted at the
    // depths arm above, fourteen of each over the one
    // hundred seventy six and one hundred seventy two
    // the Round 22 enrolment left standing. The
    // Round 24 fold moved no figure the sweep reads,
    // the tally computation of the six tally readings
    // standing folded into the one helper of the
    // championship module under FIX-DH-28 with every
    // golden standing byte for byte as it stood, so no
    // golden joins at that round, no arm joins the
    // chain, and the counts stand unmoved.
    assert_eq!(goldens.len(), 190);
    assert_eq!(unadjusted, 186);
}
