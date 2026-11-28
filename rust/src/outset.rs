//! The outset reading of DH-72 and the
//! outset report of DH-73. The outset of a
//! decided series reads the first reign of
//! its winner against the first spell of its
//! winner, the first reign standing at the
//! count of the standings of the first run of
//! ahead standings of the winner over the
//! decided record and the first spell at the
//! count of the standings of the first run of
//! behind standings. A series is classed
//! reign longer where its first reign stands
//! above its first spell, spell longer where
//! its first spell stands above its first
//! reign, and matched where the two stand
//! equal. A series holding no winner carries
//! no first reign and no first spell and
//! stands in no class.

use crate::championship::{count_series, Championship, SeriesCount};
use crate::series::Series;

/// The outset class of a decided series under DH-72, reading the winner's
/// first reign against the winner's first spell behind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutsetClass {
    /// The winner's first reign stands above its first spell behind.
    ReignLonger,
    /// The winner's first spell behind stands above its first reign.
    SpellLonger,
    /// The winner's first reign and first spell behind stand equal.
    Matched,
}

fn class_of(count: &SeriesCount) -> Option<(OutsetClass, u64, u64)> {
    let champion = count.champion?;
    let mut tallies = [0u64; 2];
    let mut first_reign: u64 = 0;
    let mut first_spell: u64 = 0;
    let mut run_sign: i64 = 0;
    let mut run_length: u64 = 0;
    for (_, side) in &count.decided {
        tallies[*side] += 1;
        let diff = tallies[champion] as i64 - tallies[1 - champion] as i64;
        let sign = if diff > 0 {
            1
        } else if diff < 0 {
            -1
        } else {
            0
        };
        if sign != 0 && sign == run_sign {
            run_length += 1;
        } else {
            if run_sign > 0 && first_reign == 0 {
                first_reign = run_length;
            }
            if run_sign < 0 && first_spell == 0 {
                first_spell = run_length;
            }
            run_sign = sign;
            run_length = if sign != 0 { 1 } else { 0 };
        }
    }
    if run_sign > 0 && first_reign == 0 {
        first_reign = run_length;
    }
    if run_sign < 0 && first_spell == 0 {
        first_spell = run_length;
    }
    if first_reign > first_spell {
        return Some((OutsetClass::ReignLonger, first_reign, first_spell));
    }
    if first_spell > first_reign {
        return Some((OutsetClass::SpellLonger, first_reign, first_spell));
    }
    Some((OutsetClass::Matched, first_reign, first_spell))
}

/// The outset reading of one series, its class
/// and its first reign and first spell, or
/// None where the series holds no winner.
pub fn outset_class(series: &Series) -> Option<(OutsetClass, u64, u64)> {
    let count = count_series(series, &series.models[0]);
    class_of(&count)
}

/// The outset report of one championship, one
/// section for each series classed spell
/// longer, in championship order.
pub fn outset_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["outset".to_string()];
    let mut reign_longer: u64 = 0;
    let mut spell_longer: u64 = 0;
    let mut matched: u64 = 0;
    let mut no_winner: u64 = 0;
    for (offset, series) in championship.series.iter().enumerate() {
        let count = count_series(series, &series.models[0]);
        let (class, first_reign, first_spell) = match class_of(&count) {
            Some(triple) => triple,
            None => {
                no_winner += 1;
                continue;
            }
        };
        match class {
            OutsetClass::ReignLonger => {
                reign_longer += 1;
                continue;
            }
            OutsetClass::SpellLonger => {
                spell_longer += 1;
            }
            OutsetClass::Matched => {
                matched += 1;
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
            "first reign {}, first spell {} over the decided record",
            first_reign, first_spell
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
        "outset, {} series, {} reign longer, {} spell longer, {} matched, {} no winner",
        championship.series.len(),
        reign_longer,
        spell_longer,
        matched,
        no_winner
    ));
    lines
}
