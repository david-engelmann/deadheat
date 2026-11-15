//! The reigns reading over a recorded championship,
//! DH-52 and DH-53 in CONTRACT.md.
//!
//! The reading classifies each decided series of a
//! championship by the longest reign its winner
//! stood ahead over the decided record, a reign
//! being a run of consecutive decided races after
//! each of which the winner's tally of race wins
//! stood above the loser's, read against the front
//! count of the series, the count of decided races
//! after each of which the winner so stood. A
//! decided series stands unbroken when its longest
//! reign stands equal to its front count, so its
//! winner's ahead standings stand in one run, and
//! broken when its longest reign stands below its
//! front count, so it stood ahead in two runs or
//! more. The reading names two classes and no
//! third, the deciding race always leaving the
//! winner ahead, so every decided series stands
//! ahead at least once and no never ahead class can
//! stand beside the two. A series holding no winner
//! stands in none of the two classes. One section
//! stands for each series standing broken, naming
//! its decided races in race order, the longest
//! reign and front count of the series, and its
//! winner with its race split. The counted races
//! are the races of the counting walk DH-13 states,
//! read series by series in record order and
//! stopping at each decision, and the decided races
//! of a series are read from the record that walk
//! leaves, so no second winner rule and no second
//! counting walk stands in this module. The
//! standings the reading classes by stand read in
//! this module's own body over that record, in the
//! posture of the spells module, and no tally
//! helper is consumed here.

use crate::championship::{count_series, Championship, SeriesCount};
use crate::series::Series;

/// The class of a decided series under DH-52, its
/// longest reign read against its front count.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReignClass {
    Unbroken,
    Broken,
}

/// The classification entry of the reading, the
/// class of one series with its longest reign and
/// its front count stated beside the class, in
/// that order. It stands as None where the series
/// holds no winner, a series holding no winner
/// standing in no class under DH-52. The two
/// tallies and the running count of the reign
/// standing open stand kept in this body over the
/// decided record the one walk leaves, one
/// standing added to the front count and to the
/// open reign at each decided race after which the
/// winner's tally stood above the loser's, and the
/// open reign closing at each decided race after
/// which the tallies stood level or the winner's
/// stood below.
pub fn reign_class(series: &Series) -> Option<(ReignClass, u64, u64)> {
    let count = count_series(series, &series.models[0]);
    class_of(&count)
}

fn class_of(count: &SeriesCount) -> Option<(ReignClass, u64, u64)> {
    let champion = count.champion?;
    let mut tallies = [0u64; 2];
    let mut longest: u64 = 0;
    let mut front: u64 = 0;
    let mut open: u64 = 0;
    for (_, side) in &count.decided {
        tallies[*side] += 1;
        if tallies[champion] > tallies[1 - champion] {
            front += 1;
            open += 1;
            if open > longest {
                longest = open;
            }
        } else {
            open = 0;
        }
    }
    if longest == front {
        return Some((ReignClass::Unbroken, longest, front));
    }
    Some((ReignClass::Broken, longest, front))
}

/// Return the reigns report lines for one championship.
pub fn reigns_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["reigns".to_string()];
    let mut unbroken: u64 = 0;
    let mut broken: u64 = 0;
    let mut no_winner: u64 = 0;
    for (offset, series) in championship.series.iter().enumerate() {
        let count = count_series(series, &series.models[0]);
        let (class, longest, front) = match class_of(&count) {
            Some(classed) => classed,
            None => {
                no_winner += 1;
                continue;
            }
        };
        match class {
            ReignClass::Unbroken => {
                unbroken += 1;
                continue;
            }
            ReignClass::Broken => {
                broken += 1;
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
            "longest reign {}, front count {} over the decided record",
            longest, front
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
        "reigns, {} series, {} unbroken, {} broken, {} no winner",
        championship.series.len(),
        unbroken,
        broken,
        no_winner
    ));
    lines
}
