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
fn the_distance_example_matches_its_golden() {
    // The standing championship holds no series standing
    // the distance, the pilot series standing in between
    // and the panel series standing without reply, so the
    // golden is the empty form of DH-24, the head line and
    // the closing counts and no section.
    let source = examples().join("championship.json");
    let out = run_ok(&["distance", source.to_str().expect("utf8")]);
    let golden = read_text(&examples().join("championship-distance.txt"));
    assert_eq!(out, golden);
    assert_eq!(
        out,
        "distance\ndistance, 2 series, 0 the distance, 1 without reply, 1 in between, 0 no winner\n"
    );
    let (championship, defects) =
        deadheat::championship::load_championship_file(&examples().join("championship.json"));
    assert_eq!(defects, Vec::<String>::new());
    let rendered = deadheat::distances::distance_report(&championship.expect("a championship"))
        .join("\n")
        + "\n";
    assert_eq!(rendered, golden);
}

#[test]
fn the_constructed_distance_example_matches_its_golden() {
    // The constructed championship of DH-24, labelled as
    // constructed in its file names and its task text and
    // never presented as recorded. Its first series stands
    // the distance, so the golden carries the section form,
    // its second series stands without reply, and the sixth
    // race of the first series and the fourth race of the
    // second stand beyond their decisions and are named
    // nowhere in the reading.
    let source = examples().join("constructed-distance.json");
    let out = run_ok(&["distance", source.to_str().expect("utf8")]);
    let golden = read_text(&examples().join("constructed-distance-distance.txt"));
    assert_eq!(out, golden);
    assert!(out.contains("series 1, the constructed distance drill"));
    assert!(out.contains("series winner, alpha-1, alpha-1 3 race wins, beta-1 2 race wins"));
    assert!(!out.contains("distance drill race six"));
    assert!(!out.contains("reply drill race four"));
    assert!(out.trim_end().ends_with(
        "distance, 2 series, 1 the distance, 1 without reply, 0 in between, 0 no winner"
    ));
    let (championship, defects) = deadheat::championship::load_championship_file(
        &examples().join("constructed-distance.json"),
    );
    assert_eq!(defects, Vec::<String>::new());
    let rendered = deadheat::distances::distance_report(&championship.expect("a championship"))
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

#[test]
fn the_rally_example_matches_its_golden() {
    // The standing championship holds no rally series, both
    // of its series standing wire to wire under DH-25, so
    // the golden is the empty form of DH-26, the head line
    // and the closing counts and no section.
    let source = examples().join("championship.json");
    let out = run_ok(&["rally", source.to_str().expect("utf8")]);
    let golden = read_text(&examples().join("championship-rally.txt"));
    assert_eq!(out, golden);
    assert_eq!(
        out,
        "rally\nrally, 2 series, 0 rally, 2 wire to wire, 0 no winner\n"
    );
    let (championship, defects) =
        deadheat::championship::load_championship_file(&examples().join("championship.json"));
    assert_eq!(defects, Vec::<String>::new());
    let rendered =
        deadheat::rallies::rally_report(&championship.expect("a championship")).join("\n") + "\n";
    assert_eq!(rendered, golden);
}

#[test]
fn the_constructed_rally_example_matches_its_golden() {
    // The constructed championship of DH-26, labelled as
    // constructed in its file names and its task text and
    // never presented as recorded. Its one series stands a
    // rally, its winner standing behind by two race wins at
    // the deepest, so the golden carries the section form,
    // and its ninth race stands beyond the decision and is
    // named nowhere in the reading.
    let source = examples().join("constructed-rally.json");
    let out = run_ok(&["rally", source.to_str().expect("utf8")]);
    let golden = read_text(&examples().join("constructed-rally-rally.txt"));
    assert_eq!(out, golden);
    assert!(out.contains("series 1, the constructed rally drill"));
    assert!(out.contains("stood behind by 2 race wins at the deepest"));
    assert!(!out.contains("rally drill race nine"));
    assert!(out
        .trim_end()
        .ends_with("rally, 1 series, 1 rally, 0 wire to wire, 0 no winner"));
    let (championship, defects) =
        deadheat::championship::load_championship_file(&examples().join("constructed-rally.json"));
    assert_eq!(defects, Vec::<String>::new());
    let rendered =
        deadheat::rallies::rally_report(&championship.expect("a championship")).join("\n") + "\n";
    assert_eq!(rendered, golden);
}

#[test]
fn the_deadlock_example_matches_its_golden() {
    // E-1 of the Round 13 enrolment. The standing
    // championship holds no deadlocked series, both of its
    // series standing clear under DH-29, so the golden is
    // the empty form of DH-30, the head line and the
    // closing counts and no section.
    let source = examples().join("championship.json");
    let out = run_ok(&["deadlock", source.to_str().expect("utf8")]);
    let golden = read_text(&examples().join("championship-deadlock.txt"));
    assert_eq!(out, golden);
    assert_eq!(
        out,
        "deadlock\ndeadlock, 2 series, 0 deadlocked, 2 clear, 0 no winner\n"
    );
    let (championship, defects) =
        deadheat::championship::load_championship_file(&examples().join("championship.json"));
    assert_eq!(defects, Vec::<String>::new());
    let rendered = deadheat::deadlocks::deadlock_report(&championship.expect("a championship"))
        .join("\n")
        + "\n";
    assert_eq!(rendered, golden);
}

#[test]
fn the_survey_championship_deadlock_matches_its_golden() {
    // E-2. The survey championship under the deadlock
    // command, in the section form it falls out in, pinned
    // at the CLI and at the library surface both.
    let source = examples().join("survey-championship.json");
    let out = run_ok(&["deadlock", source.to_str().expect("utf8")]);
    let golden = read_text(&examples().join("survey-championship-deadlock.txt"));
    assert_eq!(out, golden);
    let (championship, defects) = deadheat::championship::load_championship_file(
        &examples().join("survey-championship.json"),
    );
    assert_eq!(defects, Vec::<String>::new());
    let rendered = deadheat::deadlocks::deadlock_report(&championship.expect("a championship"))
        .join("\n")
        + "\n";
    assert_eq!(rendered, golden);
}

#[test]
fn the_constructed_deadlock_example_matches_its_golden() {
    // E-3. The constructed championship of DH-30, labelled
    // as constructed in its file names and its task text
    // and never presented as recorded. Its one series
    // stands deadlocked, its winner never standing behind,
    // so the golden carries the section form, and its
    // fifth race stands beyond the decision and is named
    // nowhere in the reading.
    let source = examples().join("constructed-deadlock.json");
    let out = run_ok(&["deadlock", source.to_str().expect("utf8")]);
    let golden = read_text(&examples().join("constructed-deadlock-deadlock.txt"));
    assert_eq!(out, golden);
    assert!(out.contains("series 1, the constructed deadlock drill"));
    assert!(out.contains("stood level after 1 of its decided races"));
    assert!(!out.contains("deadlock drill race five"));
    assert!(out
        .trim_end()
        .ends_with("deadlock, 1 series, 1 deadlocked, 0 clear, 0 no winner"));
    let (championship, defects) = deadheat::championship::load_championship_file(
        &examples().join("constructed-deadlock.json"),
    );
    assert_eq!(defects, Vec::<String>::new());
    let rendered = deadheat::deadlocks::deadlock_report(&championship.expect("a championship"))
        .join("\n")
        + "\n";
    assert_eq!(rendered, golden);
}

#[test]
fn the_constructed_deadlock_example_under_rally_matches_its_golden() {
    // E-4. The same constructed file under the rally
    // command, the empty form, so the distinction between
    // the two readings stands pinned over shipped data
    // byte for byte against the pin above.
    let source = examples().join("constructed-deadlock.json");
    let out = run_ok(&["rally", source.to_str().expect("utf8")]);
    let golden = read_text(&examples().join("constructed-deadlock-rally.txt"));
    assert_eq!(out, golden);
    assert_eq!(
        out,
        "rally\nrally, 1 series, 0 rally, 1 wire to wire, 0 no winner\n"
    );
    let (championship, defects) = deadheat::championship::load_championship_file(
        &examples().join("constructed-deadlock.json"),
    );
    assert_eq!(defects, Vec::<String>::new());
    let rendered =
        deadheat::rallies::rally_report(&championship.expect("a championship")).join("\n") + "\n";
    assert_eq!(rendered, golden);
}

#[test]
fn the_league_championship_matches_its_goldens() {
    // E-5. The league championship of DH-31, drawn at
    // realistic shapes and labelled as constructed in its
    // file names and its task texts. Every championship
    // reading over it is pinned byte for byte against the
    // golden the real command produced, at the CLI and at
    // the library surface both.
    let source = examples().join("league-championship.json");
    let source_arg = source.to_str().expect("utf8");
    let (championship, defects) = deadheat::championship::load_championship_file(&source);
    assert_eq!(defects, Vec::<String>::new());
    let championship = championship.expect("a championship");
    let readings: Vec<(&str, String, &str)> = vec![
        (
            "championship",
            deadheat::championship::championship_report(&championship).join("\n") + "\n",
            "league-championship.txt",
        ),
        (
            "totals",
            deadheat::totals::totals_report(&championship).join("\n") + "\n",
            "league-championship-totals.txt",
        ),
        (
            "pairing",
            deadheat::pairings::pairing_report(&championship).join("\n") + "\n",
            "league-championship-pairings.txt",
        ),
        (
            "comeback",
            deadheat::comebacks::comeback_report(&championship).join("\n") + "\n",
            "league-championship-comebacks.txt",
        ),
        (
            "distance",
            deadheat::distances::distance_report(&championship).join("\n") + "\n",
            "league-championship-distance.txt",
        ),
        (
            "rally",
            deadheat::rallies::rally_report(&championship).join("\n") + "\n",
            "league-championship-rally.txt",
        ),
        (
            "deadlock",
            deadheat::deadlocks::deadlock_report(&championship).join("\n") + "\n",
            "league-championship-deadlock.txt",
        ),
    ];
    for (command, rendered, golden_name) in readings {
        let golden = read_text(&examples().join(golden_name));
        assert_eq!(run_ok(&[command, source_arg]), golden, "{command} cli");
        assert_eq!(rendered, golden, "{command} library");
    }
}

#[test]
fn the_league_series_match_their_goldens() {
    // E-5 continued. One series rendering per league
    // series, pinned byte for byte the same way, the ten
    // goldens completing the seventeen of DH-31 with the
    // seven championship renderings above.
    for name in [
        "league-series-apex-bolt",
        "league-series-apex-crest",
        "league-series-apex-drift",
        "league-series-apex-ember",
        "league-series-bolt-crest",
        "league-series-bolt-drift",
        "league-series-bolt-ember",
        "league-series-crest-drift",
        "league-series-crest-ember",
        "league-series-drift-ember",
    ] {
        let source = examples().join(format!("{name}.json"));
        let out = run_ok(&["series", source.to_str().expect("utf8")]);
        let golden = read_text(&examples().join(format!("{name}.txt")));
        assert_eq!(out, golden, "{name}");
    }
}

#[test]
fn the_survey_championship_matches_its_goldens() {
    // The survey championship of DH-28, drawn at realistic
    // shapes and labelled as constructed in its file names
    // and its task texts. Every championship reading over
    // it is pinned byte for byte against the golden the
    // real command produced, at the CLI and at the library
    // surface both.
    let source = examples().join("survey-championship.json");
    let source_arg = source.to_str().expect("utf8");
    let (championship, defects) = deadheat::championship::load_championship_file(&source);
    assert_eq!(defects, Vec::<String>::new());
    let championship = championship.expect("a championship");
    let readings: Vec<(&str, String, &str)> = vec![
        (
            "championship",
            deadheat::championship::championship_report(&championship).join("\n") + "\n",
            "survey-championship.txt",
        ),
        (
            "totals",
            deadheat::totals::totals_report(&championship).join("\n") + "\n",
            "survey-championship-totals.txt",
        ),
        (
            "pairing",
            deadheat::pairings::pairing_report(&championship).join("\n") + "\n",
            "survey-championship-pairings.txt",
        ),
        (
            "comeback",
            deadheat::comebacks::comeback_report(&championship).join("\n") + "\n",
            "survey-championship-comebacks.txt",
        ),
        (
            "distance",
            deadheat::distances::distance_report(&championship).join("\n") + "\n",
            "survey-championship-distance.txt",
        ),
        (
            "rally",
            deadheat::rallies::rally_report(&championship).join("\n") + "\n",
            "survey-championship-rally.txt",
        ),
    ];
    for (command, rendered, golden_name) in readings {
        let golden = read_text(&examples().join(golden_name));
        assert_eq!(run_ok(&[command, source_arg]), golden, "{command} cli");
        assert_eq!(rendered, golden, "{command} library");
    }
}

#[test]
fn the_survey_series_match_their_goldens() {
    // One series rendering per survey series, pinned byte
    // for byte the same way.
    for name in [
        "survey-series-apex-bolt",
        "survey-series-apex-crest",
        "survey-series-apex-drift",
        "survey-series-bolt-crest",
        "survey-series-bolt-drift",
        "survey-series-crest-drift",
    ] {
        let source = examples().join(format!("{name}.json"));
        let out = run_ok(&["series", source.to_str().expect("utf8")]);
        let golden = read_text(&examples().join(format!("{name}.txt")));
        assert_eq!(out, golden, "{name} cli");
        let (series, defects) = load_series_file(&source);
        assert_eq!(defects, Vec::<String>::new());
        let rendered = series_report(&series.expect("a series")).join("\n") + "\n";
        assert_eq!(rendered, golden, "{name} library");
    }
}

#[test]
fn the_streak_example_matches_its_golden() {
    // E-1 of the Round 14 enrolment. The standing
    // championship holds both of its series strung under
    // DH-32, so the golden is the section form of DH-33,
    // the first classification golden over this fixture
    // carrying a section for each of its series.
    let source = examples().join("championship.json");
    let out = run_ok(&["streak", source.to_str().expect("utf8")]);
    let golden = read_text(&examples().join("championship-streak.txt"));
    assert_eq!(out, golden);
    assert!(out.contains("longest run of 2 consecutive decided race wins"));
    assert!(out.contains("longest run of 3 consecutive decided race wins"));
    assert!(out
        .trim_end()
        .ends_with("streak, 2 series, 2 strung, 0 single, 0 no winner"));
    let (championship, defects) =
        deadheat::championship::load_championship_file(&examples().join("championship.json"));
    assert_eq!(defects, Vec::<String>::new());
    let rendered =
        deadheat::streaks::streak_report(&championship.expect("a championship")).join("\n") + "\n";
    assert_eq!(rendered, golden);
}

#[test]
fn the_survey_championship_streak_matches_its_golden() {
    // E-2. The survey championship under the streak
    // command, in the section form it falls out in, pinned
    // at the CLI and at the library surface both.
    let source = examples().join("survey-championship.json");
    let out = run_ok(&["streak", source.to_str().expect("utf8")]);
    let golden = read_text(&examples().join("survey-championship-streak.txt"));
    assert_eq!(out, golden);
    let (championship, defects) = deadheat::championship::load_championship_file(
        &examples().join("survey-championship.json"),
    );
    assert_eq!(defects, Vec::<String>::new());
    let rendered =
        deadheat::streaks::streak_report(&championship.expect("a championship")).join("\n") + "\n";
    assert_eq!(rendered, golden);
}

#[test]
fn the_league_championship_streak_matches_its_golden() {
    // E-3. The league championship under the streak
    // command, nine of its series standing strung and one
    // standing single, pinned at the CLI and at the
    // library surface both.
    let source = examples().join("league-championship.json");
    let out = run_ok(&["streak", source.to_str().expect("utf8")]);
    let golden = read_text(&examples().join("league-championship-streak.txt"));
    assert_eq!(out, golden);
    let (championship, defects) = deadheat::championship::load_championship_file(
        &examples().join("league-championship.json"),
    );
    assert_eq!(defects, Vec::<String>::new());
    let rendered =
        deadheat::streaks::streak_report(&championship.expect("a championship")).join("\n") + "\n";
    assert_eq!(rendered, golden);
}

#[test]
fn the_constructed_streak_example_matches_its_golden() {
    // E-4. The constructed championship of DH-33, labelled
    // as constructed in its file names and its task texts
    // and never presented as recorded. Its first series
    // stands strung while standing in the unmarked class
    // of every sibling reading, so the golden carries its
    // section alone, and its fifth race stands beyond the
    // decision and is named nowhere in the reading.
    let source = examples().join("constructed-streak.json");
    let out = run_ok(&["streak", source.to_str().expect("utf8")]);
    let golden = read_text(&examples().join("constructed-streak-streak.txt"));
    assert_eq!(out, golden);
    assert!(out.contains("series 1, the constructed streak drill one"));
    assert!(out.contains("longest run of 2 consecutive decided race wins"));
    assert!(!out.contains("series 2,"));
    assert!(!out.contains("streak drill one race five"));
    assert!(out
        .trim_end()
        .ends_with("streak, 2 series, 1 strung, 1 single, 0 no winner"));
    let (championship, defects) =
        deadheat::championship::load_championship_file(&examples().join("constructed-streak.json"));
    assert_eq!(defects, Vec::<String>::new());
    let rendered =
        deadheat::streaks::streak_report(&championship.expect("a championship")).join("\n") + "\n";
    assert_eq!(rendered, golden);
}

#[test]
fn the_constructed_streak_example_under_distance_and_deadlock_matches_its_goldens() {
    // E-5. The same constructed file under the distance
    // and deadlock commands, the second series standing
    // the distance and deadlocked in its section alone
    // under each, so the distinction between the readings
    // stands pinned over shipped data byte for byte
    // against the pin above.
    let source = examples().join("constructed-streak.json");
    let source_arg = source.to_str().expect("utf8");
    let out = run_ok(&["distance", source_arg]);
    let golden = read_text(&examples().join("constructed-streak-distance.txt"));
    assert_eq!(out, golden);
    let out = run_ok(&["deadlock", source_arg]);
    let golden = read_text(&examples().join("constructed-streak-deadlock.txt"));
    assert_eq!(out, golden);
    let (championship, defects) =
        deadheat::championship::load_championship_file(&examples().join("constructed-streak.json"));
    assert_eq!(defects, Vec::<String>::new());
    let championship = championship.expect("a championship");
    let rendered = deadheat::distances::distance_report(&championship).join("\n") + "\n";
    assert_eq!(
        rendered,
        read_text(&examples().join("constructed-streak-distance.txt"))
    );
    let rendered = deadheat::deadlocks::deadlock_report(&championship).join("\n") + "\n";
    assert_eq!(
        rendered,
        read_text(&examples().join("constructed-streak-deadlock.txt"))
    );
}

#[test]
fn the_constructed_streak_example_under_rally_and_comeback_matches_its_goldens() {
    // E-6. The same constructed file under the rally and
    // comeback commands, the empty form under each,
    // neither winner ever standing behind and each winner
    // having won its first decided race.
    let source = examples().join("constructed-streak.json");
    let source_arg = source.to_str().expect("utf8");
    let out = run_ok(&["rally", source_arg]);
    let golden = read_text(&examples().join("constructed-streak-rally.txt"));
    assert_eq!(out, golden);
    assert_eq!(
        out,
        "rally\nrally, 2 series, 0 rally, 2 wire to wire, 0 no winner\n"
    );
    let out = run_ok(&["comeback", source_arg]);
    let golden = read_text(&examples().join("constructed-streak-comebacks.txt"));
    assert_eq!(out, golden);
    assert_eq!(
        out,
        "comebacks\ncomebacks, 2 series, 0 from behind, 2 from the front\n"
    );
    let (championship, defects) =
        deadheat::championship::load_championship_file(&examples().join("constructed-streak.json"));
    assert_eq!(defects, Vec::<String>::new());
    let championship = championship.expect("a championship");
    let rendered = deadheat::rallies::rally_report(&championship).join("\n") + "\n";
    assert_eq!(
        rendered,
        read_text(&examples().join("constructed-streak-rally.txt"))
    );
    let rendered = deadheat::comebacks::comeback_report(&championship).join("\n") + "\n";
    assert_eq!(
        rendered,
        read_text(&examples().join("constructed-streak-comebacks.txt"))
    );
}
