//! Pins for the recorded example races and their golden
//! renderings, translated from the Python corpus.
//!
//! Each golden file holds the exact bytes the race command
//! prints for its recorded race, so a rendering drift anywhere
//! in the report or the funeral fails here. The Round 2
//! examples pin the race and closecall commands for the close
//! call races and the series command for the series files the
//! same way.

mod shared;

use deadheat::races::{close_call_report, funeral, load_race_file, race_report};
use deadheat::series::{load_series_file, series_report};
use shared::*;

const ROUND_2_RACES: [&str; 4] = [
    "close-call-decided",
    "close-call-abstained",
    "close-call-unadjudicated",
    "outside-band-adjudicated",
];

fn read_text(path: &std::path::Path) -> String {
    std::fs::read_to_string(path).expect("a text file")
}

fn run_ok(args: &[&str]) -> String {
    let (out, _, code) = run_cli(args);
    assert_eq!(code, 0, "{args:?}");
    out
}

#[test]
fn the_shadow_wins_example_matches_its_golden() {
    let source = examples().join("shadow-wins.json");
    let out = run_ok(&["race", source.to_str().expect("utf8")]);
    let golden = read_text(&examples().join("shadow-wins.txt"));
    assert_eq!(out, golden);
    assert!(out.contains("verdict, shadow-1 wins"));
}

#[test]
fn the_neither_green_example_matches_its_golden() {
    let source = examples().join("neither-green.json");
    let out = run_ok(&["race", source.to_str().expect("utf8")]);
    let golden = read_text(&examples().join("neither-green.txt"));
    assert_eq!(out, golden);
    assert!(out.contains("verdict, no winner"));
}

#[test]
fn the_goldens_equal_the_module_renderings() {
    for name in ["shadow-wins", "neither-green"] {
        let (race, defects) = load_race_file(&examples().join(format!("{name}.json")));
        assert_eq!(defects, Vec::<String>::new());
        let race = race.expect("a race");
        let mut lines = race_report(&race);
        lines.extend(funeral(&race));
        let rendered = lines.join("\n") + "\n";
        let golden = read_text(&examples().join(format!("{name}.txt")));
        assert_eq!(rendered, golden);
    }
}

#[test]
fn the_round_2_race_examples_match_their_goldens() {
    for name in ROUND_2_RACES {
        let source = examples().join(format!("{name}.json"));
        let out = run_ok(&["race", source.to_str().expect("utf8")]);
        let golden = read_text(&examples().join(format!("{name}.txt")));
        assert_eq!(out, golden, "{name}");
    }
}

#[test]
fn the_round_2_closecall_examples_match_their_goldens() {
    for name in ROUND_2_RACES {
        let source = examples().join(format!("{name}.json"));
        let out = run_ok(&["closecall", source.to_str().expect("utf8")]);
        let golden = read_text(&examples().join(format!("{name}-closecall.txt")));
        assert_eq!(out, golden, "{name}");
    }
}

#[test]
fn the_round_2_goldens_equal_the_module_renderings() {
    for name in ROUND_2_RACES {
        let (race, defects) = load_race_file(&examples().join(format!("{name}.json")));
        assert_eq!(defects, Vec::<String>::new());
        let race = race.expect("a race");
        let mut lines = race_report(&race);
        lines.extend(funeral(&race));
        let rendered = lines.join("\n") + "\n";
        let golden = read_text(&examples().join(format!("{name}.txt")));
        assert_eq!(rendered, golden, "{name}");
        let rendered = close_call_report(&race).join("\n") + "\n";
        let golden = read_text(&examples().join(format!("{name}-closecall.txt")));
        assert_eq!(rendered, golden, "{name}");
    }
}

#[test]
fn the_series_examples_match_their_goldens() {
    for name in ["series-decided", "series-unfinished"] {
        let source = examples().join(format!("{name}.json"));
        let out = run_ok(&["series", source.to_str().expect("utf8")]);
        let golden = read_text(&examples().join(format!("{name}.txt")));
        assert_eq!(out, golden, "{name}");
        let (series, defects) = load_series_file(&source);
        assert_eq!(defects, Vec::<String>::new());
        let rendered = series_report(&series.expect("a series")).join("\n") + "\n";
        assert_eq!(rendered, golden, "{name}");
    }
}

fn pilot_races() -> Vec<String> {
    (1..=10).map(|index| format!("race-{index:02}")).collect()
}

#[test]
fn the_pilot_records_match_their_closecall_goldens() {
    let pilot = examples().join("pilot");
    for name in pilot_races() {
        for route in ["jev", "openai"] {
            let source = pilot.join("records").join(format!("{name}-{route}.json"));
            let out = run_ok(&["closecall", source.to_str().expect("utf8")]);
            let golden = read_text(
                &pilot
                    .join("records")
                    .join(format!("{name}-{route}-closecall.txt")),
            );
            assert_eq!(out, golden, "{name} {route}");
            let (race, defects) = load_race_file(&source);
            assert_eq!(defects, Vec::<String>::new());
            let rendered = close_call_report(&race.expect("a race")).join("\n") + "\n";
            assert_eq!(rendered, golden, "{name} {route}");
        }
    }
}

#[test]
fn the_pilot_series_matches_its_golden() {
    let source = examples().join("pilot-series.json");
    let out = run_ok(&["series", source.to_str().expect("utf8")]);
    let golden = read_text(&examples().join("pilot-series.txt"));
    assert_eq!(out, golden);
    assert!(out.contains("series winner, prime-1"));
}

#[test]
fn the_panel_records_match_their_closecall_goldens() {
    let panel = examples().join("panel");
    for name in pilot_races() {
        let source = panel.join("records").join(format!("{name}.json"));
        let out = run_ok(&["closecall", source.to_str().expect("utf8")]);
        let golden = read_text(&panel.join("records").join(format!("{name}-closecall.txt")));
        assert_eq!(out, golden, "{name}");
        let (race, defects) = load_race_file(&source);
        assert_eq!(defects, Vec::<String>::new());
        let rendered = close_call_report(&race.expect("a race")).join("\n") + "\n";
        assert_eq!(rendered, golden, "{name}");
        assert!(golden.contains("panel,"), "{name}");
    }
}

#[test]
fn the_handicapped_example_matches_its_goldens() {
    let source = examples().join("handicapped.json");
    let out = run_ok(&["race", source.to_str().expect("utf8")]);
    let golden = read_text(&examples().join("handicapped.txt"));
    assert_eq!(out, golden);
    assert!(out.contains("verdict, shadow-1 wins"));
    assert!(out.contains("adjustment -30 seconds, adjusted finish 90 seconds"));
    let out = run_ok(&["closecall", source.to_str().expect("utf8")]);
    let closecall_golden = read_text(&examples().join("handicapped-closecall.txt"));
    assert_eq!(out, closecall_golden);
    let (race, defects) = load_race_file(&source);
    assert_eq!(defects, Vec::<String>::new());
    let race = race.expect("a race");
    let mut lines = race_report(&race);
    lines.extend(funeral(&race));
    assert_eq!(lines.join("\n") + "\n", golden);
    assert_eq!(close_call_report(&race).join("\n") + "\n", closecall_golden);
}

#[test]
fn the_handicap_series_matches_its_golden() {
    let source = examples().join("handicap-series.json");
    let out = run_ok(&["series", source.to_str().expect("utf8")]);
    let golden = read_text(&examples().join("handicap-series.txt"));
    assert_eq!(out, golden);
    assert!(out.contains("race 2, balance the ledger import, shadow-1 wins"));
    assert!(out.trim_end().ends_with("series winner, shadow-1"));
    let (series, defects) = load_series_file(&source);
    assert_eq!(defects, Vec::<String>::new());
    let rendered = series_report(&series.expect("a series")).join("\n") + "\n";
    assert_eq!(rendered, golden);
}

#[test]
fn the_flip_example_matches_its_golden() {
    let source = examples().join("handicap-series.json");
    let out = run_ok(&["flip", source.to_str().expect("utf8")]);
    let golden = read_text(&examples().join("handicap-series-flip.txt"));
    assert_eq!(out, golden);
    assert!(out.contains(
        "race 2, balance the ledger import, scratch prime-1, adjusted shadow-1, flipped"
    ));
    assert!(out
        .trim_end()
        .ends_with("flips, 3 races, 1 flipped, 2 held, 0 no results"));
    let (series, defects) = load_series_file(&source);
    assert_eq!(defects, Vec::<String>::new());
    let rendered = deadheat::flips::flip_report(&series.expect("a series")).join("\n") + "\n";
    assert_eq!(rendered, golden);
}

#[test]
fn the_championship_example_matches_its_golden() {
    let source = examples().join("championship.json");
    let out = run_ok(&["championship", source.to_str().expect("utf8")]);
    let golden = read_text(&examples().join("championship.txt"));
    assert_eq!(out, golden);
    assert!(out.contains("aggregate margin 45.25 seconds"));
    let (championship, defects) =
        deadheat::championship::load_championship_file(&examples().join("championship.json"));
    assert_eq!(defects, Vec::<String>::new());
    let rendered =
        deadheat::championship::championship_report(&championship.expect("a championship"))
            .join("\n")
            + "\n";
    assert_eq!(rendered, golden);
}

#[test]
fn the_totals_example_matches_its_golden() {
    let source = examples().join("championship.json");
    let out = run_ok(&["totals", source.to_str().expect("utf8")]);
    let golden = read_text(&examples().join("championship-totals.txt"));
    assert_eq!(out, golden);
    assert!(out.contains("prime-1, 14 races, 6 wins, 1 losses, 7 no results, 1017000 tokens"));
    assert!(out
        .trim_end()
        .ends_with("totals, 2 series, 14 races, 2 models"));
    let (championship, defects) =
        deadheat::championship::load_championship_file(&examples().join("championship.json"));
    assert_eq!(defects, Vec::<String>::new());
    let rendered =
        deadheat::totals::totals_report(&championship.expect("a championship")).join("\n") + "\n";
    assert_eq!(rendered, golden);
}

#[test]
fn the_pairing_example_matches_its_golden() {
    let source = examples().join("championship.json");
    let out = run_ok(&["pairing", source.to_str().expect("utf8")]);
    let golden = read_text(&examples().join("championship-pairings.txt"));
    assert_eq!(out, golden);
    assert!(out.contains("pairing, prime-1, against, shadow-1"));
    assert!(out
        .trim_end()
        .ends_with("pairings, 1 pairings, 2 series, 14 races"));
    let (championship, defects) =
        deadheat::championship::load_championship_file(&examples().join("championship.json"));
    assert_eq!(defects, Vec::<String>::new());
    let rendered = deadheat::pairings::pairing_report(&championship.expect("a championship"))
        .join("\n")
        + "\n";
    assert_eq!(rendered, golden);
}

#[test]
fn the_comeback_example_matches_its_golden() {
    // The standing championship holds no series won from
    // behind, both of its series standing won from the
    // front, so the golden is the empty form of DH-22, the
    // head line and the closing counts and no section.
    let source = examples().join("championship.json");
    let out = run_ok(&["comeback", source.to_str().expect("utf8")]);
    let golden = read_text(&examples().join("championship-comebacks.txt"));
    assert_eq!(out, golden);
    assert_eq!(
        out,
        "comebacks\ncomebacks, 2 series, 0 from behind, 2 from the front\n"
    );
    let (championship, defects) =
        deadheat::championship::load_championship_file(&examples().join("championship.json"));
    assert_eq!(defects, Vec::<String>::new());
    let rendered = deadheat::comebacks::comeback_report(&championship.expect("a championship"))
        .join("\n")
        + "\n";
    assert_eq!(rendered, golden);
}

#[test]
fn the_constructed_comeback_example_matches_its_golden() {
    // The constructed championship of DH-22, labelled as
    // constructed in its file names and its task text and
    // never presented as recorded. Its one series stands
    // won from behind, so the golden carries the section
    // form, and its fourth race stands beyond the decision
    // and is named nowhere in the reading.
    let source = examples().join("constructed-comeback.json");
    let out = run_ok(&["comeback", source.to_str().expect("utf8")]);
    let golden = read_text(&examples().join("constructed-comeback-comebacks.txt"));
    assert_eq!(out, golden);
    assert!(out.contains("series 1, the constructed comeback drill"));
    assert!(out.contains("first decided race, race 1, alpha-1 lost to beta-1"));
    assert!(!out.contains("race four"));
    assert!(out
        .trim_end()
        .ends_with("comebacks, 1 series, 1 from behind, 0 from the front"));
    let (championship, defects) = deadheat::championship::load_championship_file(
        &examples().join("constructed-comeback.json"),
    );
    assert_eq!(defects, Vec::<String>::new());
    let rendered = deadheat::comebacks::comeback_report(&championship.expect("a championship"))
        .join("\n")
        + "\n";
    assert_eq!(rendered, golden);
}

#[test]
fn the_panel_series_matches_its_golden() {
    let source = examples().join("panel-series.json");
    let out = run_ok(&["series", source.to_str().expect("utf8")]);
    let golden = read_text(&examples().join("panel-series.txt"));
    assert_eq!(out, golden);
    assert!(out.contains("race 5, name the worker pool, prime-1 wins"));
    assert!(out.contains("race 6, draft the migration note, beyond the decision"));
    assert!(out.trim_end().ends_with("series winner, prime-1"));
    let (series, defects) = load_series_file(&source);
    assert_eq!(defects, Vec::<String>::new());
    let rendered = series_report(&series.expect("a series")).join("\n") + "\n";
    assert_eq!(rendered, golden);
}
