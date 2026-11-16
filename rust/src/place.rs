//! The place reading over a recorded championship,
//! DH-54 and DH-55 in CONTRACT.md.
//!
//! The reading classifies each decided series of a
//! championship by the place of the deepest deficit
//! its winner stood at over the decided record, the
//! place being the count of decided races standing
//! up to and including the first decided race after
//! which the winner's deficit stood at the depth of
//! the series, the depth being the largest deficit
//! of the winner as DH-48 states it, read against
//! the depth itself. A decided series bearing a
//! depth stands early when its place stands equal
//! to its depth, so the depth stands attained
//! before its winner won a decided race, and late
//! when its place stands above its depth, so its
//! winner had won a decided race before the depth
//! stands attained. A decided series bearing no
//! depth, its winner never having stood behind,
//! stands never behind. A series holding no winner
//! stands in none of the three classes. One section
//! stands for each series standing late, naming its
//! decided races in race order, the depth and place
//! of the series, and its winner with its race
//! split. The counted races are the races of the
//! counting walk DH-13 states, read series by
//! series in record order and stopping at each
//! decision, and the decided races of a series are
//! read from the record that walk leaves, so no
//! second winner rule and no second counting walk
//! stands in this module. The standings the reading
//! classes by stand read in this module's own body
//! over that record, in the posture of the depths
//! module, and no tally helper is consumed here.

use crate::championship::{count_series, Championship, SeriesCount};
use crate::series::Series;

/// The class of a decided series under DH-54, its
/// place read against its depth.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaceClass {
    Early,
    Late,
    NeverBehind,
}

/// The classification entry of the reading, the
/// class of one series with its place and its depth
/// stated beside the class, in that order. It
/// stands as None where the series holds no winner,
/// a series holding no winner standing in no class
/// under DH-54. The two tallies stand kept in this
/// body over the decided record the one walk
/// leaves, the deficit of the winner standing
/// recomputed at each decided race as the tally of
/// the loser less the tally of the winner where
/// that count stands above nought and nought where
/// it does not, the depth standing at the largest
/// deficit so read, and the place standing at the
/// count of decided races standing up to and
/// including the first decided race after which
/// the deficit stood at the depth, a later standing
/// at the same depth moving neither figure.
pub fn place_class(series: &Series) -> Option<(PlaceClass, u64, u64)> {
    let count = count_series(series, &series.models[0]);
    class_of(&count)
}

fn class_of(count: &SeriesCount) -> Option<(PlaceClass, u64, u64)> {
    let champion = count.champion?;
    let mut tallies = [0u64; 2];
    let mut depth: u64 = 0;
    let mut place: u64 = 0;
    for (index, (_, side)) in count.decided.iter().enumerate() {
        tallies[*side] += 1;
        let deficit = tallies[1 - champion].saturating_sub(tallies[champion]);
        if deficit > depth {
            depth = deficit;
            place = (index + 1) as u64;
        }
    }
    if depth == 0 {
        return Some((PlaceClass::NeverBehind, place, depth));
    }
    if place == depth {
        return Some((PlaceClass::Early, place, depth));
    }
    Some((PlaceClass::Late, place, depth))
}

/// Return the place report lines for one championship.
pub fn place_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["place".to_string()];
    let mut early: u64 = 0;
    let mut late: u64 = 0;
    let mut never_behind: u64 = 0;
    let mut no_winner: u64 = 0;
    for (offset, series) in championship.series.iter().enumerate() {
        let count = count_series(series, &series.models[0]);
        let (class, place, depth) = match class_of(&count) {
            Some(classed) => classed,
            None => {
                no_winner += 1;
                continue;
            }
        };
        match class {
            PlaceClass::Early => {
                early += 1;
                continue;
            }
            PlaceClass::NeverBehind => {
                never_behind += 1;
                continue;
            }
            PlaceClass::Late => {
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
            "deepest deficit {}, place {} over the decided record",
            depth, place
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
        "place, {} series, {} early, {} late, {} never behind, {} no winner",
        championship.series.len(),
        early,
        late,
        never_behind,
        no_winner
    ));
    lines
}
