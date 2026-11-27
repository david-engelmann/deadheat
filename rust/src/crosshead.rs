//! The crosshead reading of DH-76 and the
//! crosshead report of DH-77. The crosshead
//! of a decided series reads the first reign
//! of its winner against the last spell of
//! its winner, the first reign standing at
//! the count of the standings of the first
//! run of ahead standings of the winner over
//! the decided record and the last spell at
//! the count of the standings of the last run
//! of behind standings. A series is classed
//! reign longer where its first reign stands
//! above its last spell, spell longer where
//! its last spell stands above its first
//! reign, and matched where the two stand
//! equal. A series holding no winner carries
//! no first reign and no last spell and
//! stands in no class.

use crate::championship::{count_series, Championship, SeriesCount};
use crate::series::Series;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrossheadClass {
    ReignLonger,
    SpellLonger,
    Matched,
}

fn class_of(count: &SeriesCount) -> Option<(CrossheadClass, u64, u64)> {
    let champion = count.champion?;
    let mut tallies = [0u64; 2];
    let mut reigns: Vec<u64> = Vec::new();
    let mut spells: Vec<u64> = Vec::new();
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
            if run_sign > 0 {
                reigns.push(run_length);
            }
            if run_sign < 0 {
                spells.push(run_length);
            }
            run_sign = sign;
            run_length = if sign != 0 { 1 } else { 0 };
        }
    }
    if run_sign > 0 {
        reigns.push(run_length);
    }
    if run_sign < 0 {
        spells.push(run_length);
    }
    let first_reign = reigns.first().copied().unwrap_or(0);
    let last_spell = spells.last().copied().unwrap_or(0);
    if first_reign > last_spell {
        return Some((CrossheadClass::ReignLonger, first_reign, last_spell));
    }
    if last_spell > first_reign {
        return Some((CrossheadClass::SpellLonger, first_reign, last_spell));
    }
    Some((CrossheadClass::Matched, first_reign, last_spell))
}

/// The crosshead reading of one series, its
/// class and its first reign and last spell,
/// or None where the series holds no winner.
pub fn crosshead_class(series: &Series) -> Option<(CrossheadClass, u64, u64)> {
    let count = count_series(series, &series.models[0]);
    class_of(&count)
}

/// The crosshead report of one championship,
/// one section for each series classed spell
/// longer, in championship order.
pub fn crosshead_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["crosshead".to_string()];
    let mut reign_longer: u64 = 0;
    let mut spell_longer: u64 = 0;
    let mut matched: u64 = 0;
    let mut no_winner: u64 = 0;
    for (offset, series) in championship.series.iter().enumerate() {
        let count = count_series(series, &series.models[0]);
        let (class, first_reign, last_spell) = match class_of(&count) {
            Some(triple) => triple,
            None => {
                no_winner += 1;
                continue;
            }
        };
        match class {
            CrossheadClass::ReignLonger => {
                reign_longer += 1;
                continue;
            }
            CrossheadClass::SpellLonger => {
                spell_longer += 1;
            }
            CrossheadClass::Matched => {
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
            "first reign {}, last spell {} over the decided record",
            first_reign, last_spell
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
        "crosshead, {} series, {} reign longer, {} spell longer, {} matched, {} no winner",
        championship.series.len(),
        reign_longer,
        spell_longer,
        matched,
        no_winner
    ));
    lines
}
