//! The rally reading over a recorded championship, DH-25
//! and DH-26 in CONTRACT.md.
//!
//! The reading classifies each decided series of a
//! championship by the tally of its winner over the decided
//! races. A decided series stands a rally when its winner's
//! tally stood below the other model's tally after some
//! decided race that is not the deciding race, and wire to
//! wire when that never stood, and a series holding no
//! winner stands in neither class. One section stands for
//! each rally series, naming its decided races in race
//! order, the deepest the winner stood behind, and its
//! winner with its race split. The counted races are the
//! races of the counting walk DH-13 states, read series by
//! series in record order and stopping at each decision,
//! and the decided races of a series are read from the
//! record that walk leaves, so no second winner rule and
//! no second counting walk stands in this module.

use crate::championship::{count_series, Championship};

/// Return the rally report lines for one championship.
pub fn rally_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["rally".to_string()];
    let mut rally: u64 = 0;
    let mut wire_to_wire: u64 = 0;
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
        // The deepest the winner stood behind, read over the
        // decided record the walk leaves with the deciding
        // race set aside, because a tally read after the
        // deciding race is the closing tally and never a
        // deficit. A level tally stands as not behind, so a
        // winner who only ever stood level stands wire to
        // wire, and a series of target one holds only the
        // deciding race in its record and so always stands
        // wire to wire.
        let mut tallies = [0u64; 2];
        let mut deepest = 0u64;
        let decided_count = count.decided.len();
        for (position, (_, side)) in count.decided.iter().enumerate() {
            tallies[*side] += 1;
            if position + 1 == decided_count {
                break;
            }
            let deficit = tallies[1 - champion].saturating_sub(tallies[champion]);
            if deficit > deepest {
                deepest = deficit;
            }
        }
        if deepest == 0 {
            wire_to_wire += 1;
            continue;
        }
        rally += 1;
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
            "stood behind by {} race wins at the deepest",
            deepest
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
        "rally, {} series, {} rally, {} wire to wire, {} no winner",
        championship.series.len(),
        rally,
        wire_to_wire,
        no_winner
    ));
    lines
}
