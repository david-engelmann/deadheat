//! The spells reading over a recorded championship,
//! DH-50 and DH-51 in CONTRACT.md.
//!
//! The reading classifies each decided series of a
//! championship by the longest spell its winner
//! stood behind over the decided record, a spell
//! being a run of consecutive decided races after
//! each of which the winner's tally of race wins
//! stood below the loser's, read against the behind
//! count of the series, the count of decided races
//! after each of which the winner so stood. A
//! decided series stands never behind when its
//! winner never stood behind, unbroken when its
//! behind count stands above nought and its longest
//! spell stands equal to it, so its behind standings
//! stand in one run, and broken when its longest
//! spell stands below its behind count, so it stood
//! behind in two runs or more. A series holding no
//! winner stands in none of the three classes. One
//! section stands for each series standing broken,
//! naming its decided races in race order, the
//! longest spell and behind count of the series, and
//! its winner with its race split. The counted
//! races are the races of the counting walk DH-13
//! states, read series by series in record order
//! and stopping at each decision, and the decided
//! races of a series are read from the record that
//! walk leaves, so no second winner rule and no
//! second counting walk stands in this module. The
//! standings the reading classes by stand read in
//! this module's own body over that record, in the
//! posture of the crossings module's changes of
//! hand pass, and no tally helper is consumed here.

use crate::championship::{count_series, Championship, SeriesCount};
use crate::series::Series;

/// The class of a decided series under DH-50, its
/// longest spell read against its behind count.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpellClass {
    Unbroken,
    Broken,
    NeverBehind,
}

/// The classification entry of the reading, the
/// class of one series with its longest spell and
/// its behind count stated beside the class, in
/// that order. It stands as None where the series
/// holds no winner, a series holding no winner
/// standing in no class under DH-50. The two
/// tallies and the running count of the spell
/// standing open stand kept in this body over the
/// decided record the one walk leaves, one
/// standing added to the behind count and to the
/// open spell at each decided race after which the
/// winner's tally stood below the loser's, and the
/// open spell closing at each decided race after
/// which the tallies stood level or the winner's
/// stood above.
pub fn spell_class(series: &Series) -> Option<(SpellClass, u64, u64)> {
    let count = count_series(series, &series.models[0]);
    class_of(&count)
}

fn class_of(count: &SeriesCount) -> Option<(SpellClass, u64, u64)> {
    let champion = count.champion?;
    let mut tallies = [0u64; 2];
    let mut longest: u64 = 0;
    let mut behind: u64 = 0;
    let mut open: u64 = 0;
    for (_, side) in &count.decided {
        tallies[*side] += 1;
        if tallies[champion] < tallies[1 - champion] {
            behind += 1;
            open += 1;
            if open > longest {
                longest = open;
            }
        } else {
            open = 0;
        }
    }
    if behind == 0 {
        return Some((SpellClass::NeverBehind, longest, behind));
    }
    if longest == behind {
        return Some((SpellClass::Unbroken, longest, behind));
    }
    Some((SpellClass::Broken, longest, behind))
}

/// Return the spells report lines for one championship.
pub fn spells_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["spells".to_string()];
    let mut unbroken: u64 = 0;
    let mut broken: u64 = 0;
    let mut never_behind: u64 = 0;
    let mut no_winner: u64 = 0;
    for (offset, series) in championship.series.iter().enumerate() {
        let count = count_series(series, &series.models[0]);
        let (class, longest, behind) = match class_of(&count) {
            Some(classed) => classed,
            None => {
                no_winner += 1;
                continue;
            }
        };
        match class {
            SpellClass::NeverBehind => {
                never_behind += 1;
                continue;
            }
            SpellClass::Unbroken => {
                unbroken += 1;
                continue;
            }
            SpellClass::Broken => {
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
            "longest spell {}, behind count {} over the decided record",
            longest, behind
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
        "spells, {} series, {} unbroken, {} broken, {} never behind, {} no winner",
        championship.series.len(),
        unbroken,
        broken,
        never_behind,
        no_winner
    ));
    lines
}
