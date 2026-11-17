//! The recovery reading over a recorded championship,
//! DH-58 and DH-59 in CONTRACT.md.
//!
//! The reading classifies each decided series of a
//! championship by the answer of its winner to the
//! deepest deficit of the series over the decided
//! record, the answer being the count of decided
//! races standing after the first attainment of
//! the depth up to and including the first decided
//! race after the attainment after which the tally
//! of the winner stood above the tally of the
//! loser, read against the depth plus one. A
//! decided series bearing a depth stands immediate
//! when its answer stands equal to its depth plus
//! one, so the winner won every decided race
//! standing after the attainment up to and
//! including the answer, and delayed when its
//! answer stands above its depth plus one, so its
//! loser won a decided race standing after the
//! attainment and before the answer. A decided
//! series bearing no depth, its winner never
//! having stood behind, stands never behind. A
//! series holding no winner stands in none of the
//! three classes. One section stands for each
//! series standing delayed, naming its decided
//! races in race order, the depth and answer of
//! the series, and its winner with its race
//! split. The counted races are the races of the
//! counting walk DH-13 states, read series by
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

/// The class of a decided series under DH-58, its
/// answer read against its depth plus one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecoveryClass {
    Immediate,
    Delayed,
    NeverBehind,
}

/// The classification entry of the reading, the
/// class of one series with its answer and its
/// depth stated beside the class, in that order.
/// It stands as None where the series holds no
/// winner, a series holding no winner standing in
/// no class under DH-58. The two tallies stand
/// kept in this body over the decided record the
/// one walk leaves, the deficit of the winner
/// standing recomputed at each decided race as
/// the tally of the loser less the tally of the
/// winner where that count stands above nought
/// and nought where it does not, the depth
/// standing at the largest deficit so read, the
/// attainment standing at the count of decided
/// races standing up to and including the first
/// decided race after which the deficit stood at
/// the depth, a later standing at the same depth
/// moving neither figure, and the answer standing
/// at the count of decided races standing after
/// the attainment up to and including the first
/// decided race after the attainment after which
/// the tally of the winner stood above the tally
/// of the loser, the search for the answer
/// standing begun again where a deeper deficit
/// stands attained.
pub fn recovery_class(series: &Series) -> Option<(RecoveryClass, u64, u64)> {
    let count = count_series(series, &series.models[0]);
    class_of(&count)
}

fn class_of(count: &SeriesCount) -> Option<(RecoveryClass, u64, u64)> {
    let champion = count.champion?;
    let mut tallies = [0u64; 2];
    let mut depth: u64 = 0;
    let mut attainment: u64 = 0;
    let mut answer: u64 = 0;
    for (index, (_, side)) in count.decided.iter().enumerate() {
        tallies[*side] += 1;
        let deficit = tallies[1 - champion].saturating_sub(tallies[champion]);
        if deficit > depth {
            depth = deficit;
            attainment = (index + 1) as u64;
            answer = 0;
        } else if answer == 0 && depth > 0 && tallies[champion] > tallies[1 - champion] {
            answer = (index + 1) as u64 - attainment;
        }
    }
    if depth == 0 {
        return Some((RecoveryClass::NeverBehind, answer, depth));
    }
    if answer == depth + 1 {
        return Some((RecoveryClass::Immediate, answer, depth));
    }
    Some((RecoveryClass::Delayed, answer, depth))
}

/// Return the recovery report lines for one championship.
pub fn recovery_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["recovery".to_string()];
    let mut immediate: u64 = 0;
    let mut delayed: u64 = 0;
    let mut never_behind: u64 = 0;
    let mut no_winner: u64 = 0;
    for (offset, series) in championship.series.iter().enumerate() {
        let count = count_series(series, &series.models[0]);
        let (class, answer, depth) = match class_of(&count) {
            Some(classed) => classed,
            None => {
                no_winner += 1;
                continue;
            }
        };
        match class {
            RecoveryClass::Immediate => {
                immediate += 1;
                continue;
            }
            RecoveryClass::NeverBehind => {
                never_behind += 1;
                continue;
            }
            RecoveryClass::Delayed => {
                delayed += 1;
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
            "deepest deficit {}, answer {} over the decided record",
            depth, answer
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
        "recovery, {} series, {} immediate, {} delayed, {} never behind, {} no winner",
        championship.series.len(),
        immediate,
        delayed,
        never_behind,
        no_winner
    ));
    lines
}
