//! The first stand reading over a recorded championship,
//! DH-62 and DH-63 in CONTRACT.md.
//!
//! The reading classifies each decided series of a
//! championship by the first stand of the series over
//! the decided record, the run of its loser standing
//! first among the runs of the loser. A decided series
//! stands held when its first stand stands at two or
//! more, single when it stands at one, and without
//! reply where its loser won no decided race, and a
//! series holding no winner stands in none of the
//! three classes. One section stands for each series
//! standing held, naming its decided races in race
//! order, the first stand of the series, and its winner
//! with its race split. The counted races are the
//! races of the counting walk DH-13 states, read
//! series by series in record order and stopping at
//! each decision, and the decided races of a series
//! are read from the record that walk leaves, so no
//! second winner rule and no second counting walk
//! stands in this module. The runs the reading classes
//! by stand read in this module's own body over that
//! record, in one pass in which the runs of the record
//! stand counted in order and the run of the loser
//! standing first stands kept.

use crate::championship::{count_series, Championship, SeriesCount};
use crate::series::Series;

/// The class of a decided series under DH-62, its
/// first stand read at the threshold of two.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FirststandClass {
    Held,
    Single,
    WithoutReply,
}

/// The classification entry of the reading, the
/// class of one series with its first stand stated
/// beside the class, in that order. It stands as
/// None where the series holds no winner, a series
/// holding no winner standing in no class under
/// DH-62. The runs of the decided record stand
/// counted in this body in one pass in their order,
/// the run of the loser standing kept at its length
/// as each of its decided races stands counted
/// until the first run of the loser closes, so
/// the length standing when that run closes is
/// the run of the loser standing first, and the
/// first stand stands at nought where no run of
/// the loser stands in the record.
pub fn firststand_class(series: &Series) -> Option<(FirststandClass, u64)> {
    let count = count_series(series, &series.models[0]);
    class_of(&count)
}

fn class_of(count: &SeriesCount) -> Option<(FirststandClass, u64)> {
    let champion = count.champion?;
    let loser = 1 - champion;
    let mut run: u64 = 0;
    let mut first_stand: u64 = 0;
    let mut first_closed = false;
    for (_, side) in &count.decided {
        if *side == loser {
            run += 1;
            if !first_closed {
                first_stand = run;
            }
        } else {
            if run > 0 {
                first_closed = true;
            }
            run = 0;
        }
    }
    if first_stand >= 2 {
        return Some((FirststandClass::Held, first_stand));
    }
    if first_stand == 1 {
        return Some((FirststandClass::Single, first_stand));
    }
    Some((FirststandClass::WithoutReply, first_stand))
}

/// Return the first stand report lines for one championship.
pub fn firststand_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["firststand".to_string()];
    let mut held: u64 = 0;
    let mut single: u64 = 0;
    let mut without_reply: u64 = 0;
    let mut no_winner: u64 = 0;
    for (offset, series) in championship.series.iter().enumerate() {
        let count = count_series(series, &series.models[0]);
        let (class, first_stand) = match class_of(&count) {
            Some(classed) => classed,
            None => {
                no_winner += 1;
                continue;
            }
        };
        match class {
            FirststandClass::Held => {
                held += 1;
            }
            FirststandClass::Single => {
                single += 1;
                continue;
            }
            FirststandClass::WithoutReply => {
                without_reply += 1;
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
            "first stand {} over the decided record",
            first_stand
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
        "firststand, {} series, {} held, {} single, {} without reply, {} no winner",
        championship.series.len(),
        held,
        single,
        without_reply,
        no_winner
    ));
    lines
}
