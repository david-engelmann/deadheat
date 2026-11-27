//! The first reign reading of DH-66 and the
//! firstreign report of DH-67. The first
//! reign of a decided series stands at the
//! count of decided races in the first run
//! of ahead standings of the series winner
//! over the decided record, a reign standing
//! as a run of consecutive decided races
//! after each of which the winner's tally of
//! race wins stood above the loser's, the
//! first reign standing as the first such
//! run of the record, wherever in the record
//! it stands. A series is classed held where
//! its first reign stands at two or more and
//! single where it stands at one. A series
//! holding no winner carries no first reign
//! and stands in no class.

use crate::championship::{count_series, Championship, SeriesCount};
use crate::series::Series;

/// The first reign class of a decided series under DH-66, standing held
/// where the winner's first run of ahead standings holds two or more
/// decided races and single where it holds one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FirstreignClass {
    /// The winner's first run of ahead standings holds two or more races.
    Held,
    /// The winner's first run of ahead standings holds one race.
    Single,
}

fn class_of(count: &SeriesCount) -> Option<(FirstreignClass, u64)> {
    let champion = count.champion?;
    let mut tallies = [0u64; 2];
    let mut reign: u64 = 0;
    let mut started = false;
    for (_, side) in &count.decided {
        tallies[*side] += 1;
        if tallies[champion] > tallies[1 - champion] {
            started = true;
            reign += 1;
        } else if started {
            break;
        }
    }
    if reign >= 2 {
        return Some((FirstreignClass::Held, reign));
    }
    Some((FirstreignClass::Single, reign))
}

/// The first reign reading of one series, its
/// class and its first reign, or None where
/// the series holds no winner.
pub fn firstreign_class(series: &Series) -> Option<(FirstreignClass, u64)> {
    let count = count_series(series, &series.models[0]);
    class_of(&count)
}

/// The firstreign report of one championship,
/// one section for each series classed held,
/// in championship order.
pub fn firstreign_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["firstreign".to_string()];
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
            FirstreignClass::Held => {
                held += 1;
            }
            FirstreignClass::Single => {
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
        lines.push(format!("first reign {} over the decided record", reign));
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
        "firstreign, {} series, {} held, {} single, {} no winner",
        championship.series.len(),
        held,
        single,
        no_winner
    ));
    lines
}
