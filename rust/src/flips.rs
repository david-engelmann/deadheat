//! The flip reading over a recorded series, DH-15 and DH-16
//! in CONTRACT.md.
//!
//! The reading sets the scratch winner of each race beside
//! its adjusted winner. Both winners come from the one
//! winner rule, read over the race as recorded and over the
//! race with every adjustment cleared, so no second winner
//! rule stands in this module.

use crate::races::{decide_winner, Race};
use crate::series::{series_report, Series};

/// Return the scratch winner of one race, the winner the
/// standing winner rule decides for the race with every
/// adjustment cleared. With no adjustment standing, the
/// adjusted finish of DH-11 is the recorded finish for every
/// attempt, the close call band reads the recorded finishes,
/// and the adjudication and panel steps stand ahead of the
/// finish comparison exactly as they stand, so the one
/// decide_winner reads both winners of the reading.
pub fn scratch_winner(race: &Race) -> Option<usize> {
    let mut unadjusted = race.clone();
    for attempt in &mut unadjusted.attempts {
        attempt.adjustment = None;
    }
    let (winner, _) = decide_winner(&unadjusted);
    winner
}

/// Return the flip report lines for one series.
pub fn flip_report(series: &Series) -> Vec<String> {
    let report = series_report(series);
    let mut lines = vec![report[0].clone(), report[1].clone()];
    let mut flipped = 0usize;
    let mut held = 0usize;
    let mut no_results = 0usize;
    for (offset, race) in series.races.iter().enumerate() {
        let scratch = scratch_winner(race);
        let (adjusted, _) = decide_winner(race);
        let name = |winner: Option<usize>| match winner {
            Some(index) => race.attempts[index].model.display_line(),
            None => "none".to_string(),
        };
        let standing = match (scratch, adjusted) {
            (Some(first), Some(second)) if first == second => {
                held += 1;
                "held"
            }
            (Some(_), Some(_)) => {
                flipped += 1;
                "flipped"
            }
            _ => {
                no_results += 1;
                "no result"
            }
        };
        lines.push(format!(
            "race {}, {}, scratch {}, adjusted {}, {}",
            offset + 1,
            race.task.display_line(),
            name(scratch),
            name(adjusted),
            standing
        ));
    }
    lines.push(format!(
        "flips, {} races, {} flipped, {} held, {} no results",
        series.races.len(),
        flipped,
        held,
        no_results
    ));
    lines
}
