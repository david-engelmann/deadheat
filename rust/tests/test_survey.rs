//! Pins for the survey championship of DH-28 in the
//! Round 12 section of CONTRACT.md.
//!
//! The fixture is pinned by its shape and by its closings,
//! not by its bytes alone, the goldens standing in
//! test_examples.rs. The shape pins fail when the fixture
//! shrinks below the shape DH-28 states, six series over
//! four models at the stated targets and volumes with the
//! finish values distinct at the stated rate and the no
//! result share inside its stated band. The relation pins
//! verify the stated relations of DH-13, DH-18, DH-20,
//! DH-22, DH-24, and DH-26 over the survey championship
//! from the printed readings alone, the same figures a
//! reader of the reports holds.

mod shared;

use std::collections::BTreeSet;

use deadheat::championship::{championship_report, load_championship_file, Championship};
use deadheat::comebacks::comeback_report;
use deadheat::distances::distance_report;
use deadheat::pairings::pairing_report;
use deadheat::races::decide_winner;
use deadheat::rallies::rally_report;
use deadheat::totals::totals_report;
use shared::{closing_counts, examples};

fn survey() -> Championship {
    let (championship, defects) =
        load_championship_file(&examples().join("survey-championship.json"));
    assert_eq!(defects, Vec::<String>::new());
    championship.expect("a championship")
}

#[test]
fn the_survey_championship_holds_its_stated_shape() {
    let championship = survey();
    assert_eq!(championship.series.len(), 6, "six series");
    let mut pairs = BTreeSet::new();
    let mut targets = BTreeSet::new();
    let mut models = BTreeSet::new();
    let mut total_races = 0usize;
    let mut no_result_races = 0usize;
    let mut finishes: Vec<String> = Vec::new();
    for series in &championship.series {
        assert_eq!(series.models.len(), 2);
        let pair = (series.models[0].display(), series.models[1].display());
        assert!(pairs.insert(pair.clone()), "each pair stands once");
        models.insert(pair.0.clone());
        models.insert(pair.1.clone());
        targets.insert(series.wins_target.to_string());
        assert!(
            series.races.len() >= 8,
            "every series holds at least 8 recorded races"
        );
        assert!(
            series.task.display().contains("constructed"),
            "the series task text is labelled as constructed"
        );
        for race in &series.races {
            total_races += 1;
            if decide_winner(race).0.is_none() {
                no_result_races += 1;
            }
            for attempt in &race.attempts {
                if let Some(finish) = &attempt.finished_at {
                    finishes.push(finish.plain_text());
                }
            }
        }
    }
    assert_eq!(models.len(), 4, "four models");
    assert_eq!(
        targets,
        BTreeSet::from(["3".to_string(), "4".to_string(), "5".to_string()]),
        "the wins targets stand spread over 3, 4, and 5"
    );
    let distinct: BTreeSet<&String> = finishes.iter().collect();
    assert!(
        distinct.len() * 10 >= finishes.len() * 9,
        "the distinct finish values stand at nine in ten of the finishes or better, {} of {}",
        distinct.len(),
        finishes.len()
    );
    assert!(
        no_result_races * 12 >= total_races,
        "the no result share stands at or above one in twelve"
    );
    assert!(
        no_result_races * 5 <= total_races,
        "the no result share stands at or below one in five"
    );
}

#[test]
fn the_survey_closings_stand_in_their_stated_relations() {
    let championship = survey();
    // The decided series of the fixture, read from the
    // championship report's own winner lines.
    let table = championship_report(&championship);
    let decided = table
        .iter()
        .filter(|line| line.starts_with("series winner,"))
        .count() as u64;
    assert!(decided > 0, "the fixture holds decided series");
    let no_winner = 6 - decided;
    // DH-13 and DH-18, the totals closing counts the
    // counted races of the walk over the six series and
    // the four models, and the pairing closing of DH-20
    // counts the same races.
    let totals = totals_report(&championship);
    let totals_counts = closing_counts(totals.last().expect("a closing"));
    assert_eq!(totals_counts[0], 6);
    assert_eq!(totals_counts[2], 4);
    let counted_races = totals_counts[1];
    let pairing = pairing_report(&championship);
    assert_eq!(
        closing_counts(pairing.last().expect("a closing")),
        vec![6, 6, counted_races]
    );
    // DH-22, the comeback classes sum to the decided
    // series, and the count the classes leave over is the
    // no winner count.
    let comebacks = comeback_report(&championship);
    let comeback_counts = closing_counts(comebacks.last().expect("a closing"));
    assert_eq!(comeback_counts[0], 6);
    assert_eq!(comeback_counts[1] + comeback_counts[2], decided);
    // DH-26, the rally and wire to wire counts sum to the
    // decided series, the no winner count agrees with the
    // comeback reading's leftover, and every series won
    // from behind stands a rally.
    let rally = rally_report(&championship);
    let rally_counts = closing_counts(rally.last().expect("a closing"));
    assert_eq!(rally_counts[0], 6);
    assert_eq!(rally_counts[1] + rally_counts[2], decided);
    assert_eq!(rally_counts[3], no_winner);
    assert!(
        rally_counts[1] >= comeback_counts[1],
        "the rally count stands at or above the from behind count"
    );
    // DH-24, the distance classes sum to the decided
    // series and its no winner figure agrees.
    let distance = distance_report(&championship);
    let distance_counts = closing_counts(distance.last().expect("a closing"));
    assert_eq!(distance_counts[0], 6);
    assert_eq!(
        distance_counts[1] + distance_counts[2] + distance_counts[3],
        decided
    );
    assert_eq!(distance_counts[4], no_winner);
}
