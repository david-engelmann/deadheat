//! The leadchanges reading of DH-70 and the
//! leadchanges report of DH-71. The changes
//! of hand of the lead of a decided series
//! stand at the count of the places over the
//! decided record where the lead passed from
//! one model to the other, a place standing
//! as a change where the winner's tally stood
//! on one side of the loser's after a decided
//! race and on the other side after the next
//! decided race after which the tallies stood
//! apart. A series is classed swung where its
//! changes stand at two or more, turned where
//! they stand at one, and never behind where
//! they stand at none. A series holding no
//! winner carries no changes of hand and
//! stands in no class.

use crate::championship::{count_series, Championship, SeriesCount};
use crate::series::Series;

/// The lead changes class of a decided series under DH-70, counting places
/// over the decided record where the lead passed from one model to the other.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeadchangesClass {
    /// The lead changed hands two or more times across the decided record.
    Swung,
    /// The lead changed hands once across the decided record.
    Turned,
    /// The lead never changed hands across the decided record.
    NeverBehind,
}

fn class_of(count: &SeriesCount) -> Option<(LeadchangesClass, u64)> {
    let champion = count.champion?;
    let mut tallies = [0u64; 2];
    let mut changes: u64 = 0;
    let mut apart: i64 = 0;
    for (_, side) in &count.decided {
        tallies[*side] += 1;
        let diff = tallies[champion] as i64 - tallies[1 - champion] as i64;
        if diff != 0 {
            let side_sign = if diff > 0 { 1 } else { -1 };
            if apart != 0 && side_sign != apart {
                changes += 1;
            }
            apart = side_sign;
        }
    }
    if changes >= 2 {
        return Some((LeadchangesClass::Swung, changes));
    }
    if changes == 1 {
        return Some((LeadchangesClass::Turned, changes));
    }
    Some((LeadchangesClass::NeverBehind, changes))
}

/// The leadchanges reading of one series, its
/// class and its changes of hand of the lead,
/// or None where the series holds no winner.
pub fn leadchanges_class(series: &Series) -> Option<(LeadchangesClass, u64)> {
    let count = count_series(series, &series.models[0]);
    class_of(&count)
}

/// The leadchanges report of one championship,
/// one section for each series classed swung,
/// in championship order.
pub fn leadchanges_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["leadchanges".to_string()];
    let mut swung: u64 = 0;
    let mut turned: u64 = 0;
    let mut never_behind: u64 = 0;
    let mut no_winner: u64 = 0;
    for (offset, series) in championship.series.iter().enumerate() {
        let count = count_series(series, &series.models[0]);
        let (class, changes) = match class_of(&count) {
            Some(pair) => pair,
            None => {
                no_winner += 1;
                continue;
            }
        };
        match class {
            LeadchangesClass::Swung => {
                swung += 1;
            }
            LeadchangesClass::Turned => {
                turned += 1;
                continue;
            }
            LeadchangesClass::NeverBehind => {
                never_behind += 1;
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
        lines.push(format!(
            "changes of hand of the lead {} over the decided record",
            changes
        ));
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
        "leadchanges, {} series, {} swung, {} turned, {} never behind, {} no winner",
        championship.series.len(),
        swung,
        turned,
        never_behind,
        no_winner
    ));
    lines
}
