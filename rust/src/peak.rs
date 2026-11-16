//! The peak reading over a recorded championship,
//! DH-56 and DH-57 in CONTRACT.md.
//!
//! The reading classifies each decided series of a
//! championship by the place of the largest lead
//! its winner stood at over the decided record, the
//! place being the count of decided races standing
//! up to and including the first decided race after
//! which the winner's lead stood at the largest
//! lead of the series, the largest lead being the
//! largest count by which the winner's tally stood
//! above the loser's tally after a decided race as
//! DH-42 states it, read against the lead itself. A
//! decided series stands early when its place
//! stands equal to its lead, so the lead stands
//! first attained before its loser won a decided
//! race, and late when its place stands above its
//! lead, so its loser had won a decided race before
//! the lead stands first attained. A series holding
//! no winner stands in neither class. One section
//! stands for each series standing late, naming its
//! decided races in race order, the largest lead
//! and place of the series, and its winner with its
//! race split. The counted races are the races of
//! the counting walk DH-13 states, read series by
//! series in record order and stopping at each
//! decision, and the decided races of a series are
//! read from the record that walk leaves, so no
//! second winner rule and no second counting walk
//! stands in this module. The standings the reading
//! classes by stand read in this module's own body
//! over that record, in the posture of the reigns
//! module, and no tally helper is consumed here.

use crate::championship::{count_series, Championship, SeriesCount};
use crate::series::Series;

/// The class of a decided series under DH-56, its
/// place read against its largest lead.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PeakClass {
    Early,
    Late,
}

/// The classification entry of the reading, the
/// class of one series with its place and its
/// largest lead stated beside the class, in that
/// order. It stands as None where the series holds
/// no winner, a series holding no winner standing
/// in no class under DH-56. The two tallies stand
/// kept in this body over the decided record the
/// one walk leaves, the lead of the winner standing
/// recomputed at each decided race as the tally of
/// the winner less the tally of the loser where
/// that count stands above nought and nought where
/// it does not, the largest lead standing at the
/// largest lead so read, and the place standing at
/// the count of decided races standing up to and
/// including the first decided race after which
/// the lead stood at the largest lead, a later
/// standing at the same lead moving neither figure.
pub fn peak_class(series: &Series) -> Option<(PeakClass, u64, u64)> {
    let count = count_series(series, &series.models[0]);
    class_of(&count)
}

fn class_of(count: &SeriesCount) -> Option<(PeakClass, u64, u64)> {
    let champion = count.champion?;
    let mut tallies = [0u64; 2];
    let mut lead: u64 = 0;
    let mut place: u64 = 0;
    for (index, (_, side)) in count.decided.iter().enumerate() {
        tallies[*side] += 1;
        let margin = tallies[champion].saturating_sub(tallies[1 - champion]);
        if margin > lead {
            lead = margin;
            place = (index + 1) as u64;
        }
    }
    if place == lead {
        return Some((PeakClass::Early, place, lead));
    }
    Some((PeakClass::Late, place, lead))
}

/// Return the peak report lines for one championship.
pub fn peak_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["peak".to_string()];
    let mut early: u64 = 0;
    let mut late: u64 = 0;
    let mut no_winner: u64 = 0;
    for (offset, series) in championship.series.iter().enumerate() {
        let count = count_series(series, &series.models[0]);
        let (class, place, lead) = match class_of(&count) {
            Some(classed) => classed,
            None => {
                no_winner += 1;
                continue;
            }
        };
        match class {
            PeakClass::Early => {
                early += 1;
                continue;
            }
            PeakClass::Late => {
                late += 1;
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
            "largest lead {}, place {} over the decided record",
            lead, place
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
        "peak, {} series, {} early, {} late, {} no winner",
        championship.series.len(),
        early,
        late,
        no_winner
    ));
    lines
}
