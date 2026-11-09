//! The deadlock reading over a recorded championship,
//! DH-29 and DH-30 in CONTRACT.md.
//!
//! The reading classifies each decided series of a
//! championship by whether its two tallies ever stood
//! level over the decided races. A decided series stands
//! deadlocked when its two tallies stood equal after some
//! decided race that is not the deciding race, and clear
//! when that never stood, and a series holding no winner
//! stands in neither class. One section stands for each
//! deadlocked series, naming its decided races in race
//! order, how many of its decided races left the tallies
//! level, and its winner with its race split. The counted
//! races are the races of the counting walk DH-13 states,
//! read series by series in record order and stopping at
//! each decision, and the decided races of a series are
//! read from the record that walk leaves, so no second
//! winner rule and no second counting walk stands in this
//! module.

use crate::championship::{count_series, Championship};

/// Return the deadlock report lines for one championship.
pub fn deadlock_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["deadlock".to_string()];
    let mut deadlocked: u64 = 0;
    let mut clear: u64 = 0;
    let mut no_winner: u64 = 0;
    for (offset, series) in championship.series.iter().enumerate() {
        let count = count_series(series, &series.models[0]);
        let champion = match count.champion {
            Some(champion) => champion,
            None => {
                no_winner += 1;
                continue;
            }
        };
        // The count of decided races that left the tallies
        // level, read over the decided record the walk
        // leaves with the deciding race set aside, because
        // the tallies cannot stand equal after the deciding
        // race and a level standing read there would be the
        // closing tally. A series holding no winner never
        // reaches this count, because the classes of this
        // reading are classes of decided series alone, and a
        // series of target one holds only the deciding race
        // in its record and so always stands clear.
        let mut tallies = [0u64; 2];
        let mut level = 0u64;
        let decided_count = count.decided.len();
        for (position, (_, side)) in count.decided.iter().enumerate() {
            tallies[*side] += 1;
            if position + 1 == decided_count {
                break;
            }
            if tallies[0] == tallies[1] {
                level += 1;
            }
        }
        if level == 0 {
            clear += 1;
            continue;
        }
        deadlocked += 1;
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
        lines.push(format!("stood level after {} of its decided races", level));
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
        "deadlock, {} series, {} deadlocked, {} clear, {} no winner",
        championship.series.len(),
        deadlocked,
        clear,
        no_winner
    ));
    lines
}
