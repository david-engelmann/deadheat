//! The last reign reading of DH-68 and the
//! lastreign report of DH-69. The last
//! reign of a decided series stands at the
//! count of decided races in the last run
//! of ahead standings of the series winner
//! over the decided record, a reign standing
//! as a run of consecutive decided races
//! after each of which the winner's tally of
//! race wins stood above the loser's, the
//! last reign standing as the last such
//! run of the record. A series is classed
//! held where its last reign stands at two
//! or more and single where it stands at
//! one. A series holding no winner carries
//! no last reign and stands in no class.

use crate::championship::{count_series, Championship, SeriesCount};
use crate::series::Series;

/// The last reign class of a decided series under DH-68, standing held
/// where the winner's closing run of ahead standings holds two or more
/// decided races and single where it holds one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LastreignClass {
    /// The winner's last run of ahead standings holds two or more races.
    Held,
    /// The winner's last run of ahead standings holds one race.
    Single,
}

fn class_of(count: &SeriesCount) -> Option<(LastreignClass, u64)> {
    let champion = count.champion?;
    let mut tallies = [0u64; 2];
    let mut reign: u64 = 0;
    for (_, side) in &count.decided {
        tallies[*side] += 1;
        if tallies[champion] > tallies[1 - champion] {
            reign += 1;
        } else {
            reign = 0;
        }
    }
    if reign >= 2 {
        return Some((LastreignClass::Held, reign));
    }
    Some((LastreignClass::Single, reign))
}

/// The last reign reading of one series, its
/// class and its last reign, or None where
/// the series holds no winner.
pub fn lastreign_class(series: &Series) -> Option<(LastreignClass, u64)> {
    let count = count_series(series, &series.models[0]);
    class_of(&count)
}

/// The lastreign report of one championship,
/// one section for each series classed held,
/// in championship order.
pub fn lastreign_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["lastreign".to_string()];
    let mut held: u64 = 0;
    let mut single: u64 = 0;
    let mut no_winner: u64 = 0;
    for (offset, series) in championship.series.iter().enumerate() {
        let count = count_series(series, &series.models[0]);
        let (class, reign) = match class_of(&count) {
            Some(pair) => pair,
            None => {
                no_winner += 1;
                continue;
            }
        };
        match class {
            LastreignClass::Held => {
                held += 1;
            }
            LastreignClass::Single => {
                single += 1;
                continue;
            }
        }
        let champion = count.champion.expect("a classed series holds a winner");
        lines.push(format!(
            "series {}, {}",
            offset + 1,
            series.task.display_line()
        ));
        lines.push(format!(
            "{} against {}, first to {} wins",
            series.models[0].display_line(),
            series.models[1].display_line(),
            series.wins_target
        ));
        for (race_count, side) in &count.decided {
            let race = &series.races[(race_count - 1) as usize];
            lines.push(format!(
                "race {}, {}, {} wins",
                race_count,
                race.task.display_line(),
                series.models[*side].display_line()
            ));
        }
        lines.push(format!("last reign {} over the decided record", reign));
        lines.push(format!(
            "series winner, {}, {} {} race wins, {} {} race wins, {} no results",
            series.models[champion].display_line(),
            series.models[0].display_line(),
            count.wins[0],
            series.models[1].display_line(),
            count.wins[1],
            count.no_results
        ));
    }
    lines.push(format!(
        "lastreign, {} series, {} held, {} single, {} no winner",
        championship.series.len(),
        held,
        single,
        no_winner
    ));
    lines
}
