//! The first strike reading of DH-64 and the
//! firststrike report of DH-65. The first
//! strike of a decided series stands at the
//! count of races in the first run of the
//! series winner over the decided races, a
//! run standing as a maximal block of
//! consecutive decided races the winner won,
//! the first run standing wherever it stands
//! in the record. A series is classed in a
//! run where its first strike stands at two
//! or more and alone where it stands at one.
//! A series holding no winner carries no
//! strike and stands in no class.

use crate::championship::{count_series, Championship, SeriesCount};
use crate::series::Series;

/// The first strike class of a decided series under DH-64, standing in a run
/// where the winner's first run of wins holds two or more decided races and
/// alone where it holds one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FirststrikeClass {
    /// The winner's first run of decided race wins holds two or more races.
    InARun,
    /// The winner's first run of decided race wins holds a single race.
    Alone,
}

fn class_of(count: &SeriesCount) -> Option<(FirststrikeClass, u64)> {
    let champion = count.champion?;
    let mut strike: u64 = 0;
    let mut started = false;
    for (_, side) in &count.decided {
        if *side == champion {
            started = true;
            strike += 1;
        } else if started {
            break;
        }
    }
    if strike >= 2 {
        return Some((FirststrikeClass::InARun, strike));
    }
    Some((FirststrikeClass::Alone, strike))
}

/// The first strike reading of one series, its
/// class and its strike, or None where the
/// series holds no winner.
pub fn firststrike_class(series: &Series) -> Option<(FirststrikeClass, u64)> {
    let count = count_series(series, &series.models[0]);
    class_of(&count)
}

/// The firststrike report of one championship,
/// one section for each series classed in a
/// run, in championship order.
pub fn firststrike_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["firststrike".to_string()];
    let mut in_a_run: u64 = 0;
    let mut alone: u64 = 0;
    let mut no_winner: u64 = 0;
    for (offset, series) in championship.series.iter().enumerate() {
        let count = count_series(series, &series.models[0]);
        let (class, strike) = match class_of(&count) {
            Some(pair) => pair,
            None => {
                no_winner += 1;
                continue;
            }
        };
        match class {
            FirststrikeClass::InARun => {
                in_a_run += 1;
            }
            FirststrikeClass::Alone => {
                alone += 1;
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
        lines.push(format!("first strike {} over the decided record", strike));
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
        "firststrike, {} series, {} in a run, {} alone, {} no winner",
        championship.series.len(),
        in_a_run,
        alone,
        no_winner
    ));
    lines
}
