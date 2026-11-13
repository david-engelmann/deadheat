//! The depths reading over a recorded championship,
//! DH-48 and DH-49 in CONTRACT.md.
//!
//! The reading classifies each decided series of a
//! championship by the depth of its winner over the
//! decided record, the largest deficit the winner
//! stood under, read against the margin the series
//! was won by. A decided series stands deep when its
//! depth stands above its margin, matched when its
//! depth stands equal to its margin, and shallow
//! when its depth stands below its margin. A series
//! holding no winner stands in none of the three
//! classes. One section stands for each series
//! standing deep, naming its decided races in race
//! order, the deepest deficit of the series, and its
//! winner with its race split. The counted races are
//! the races of the counting walk DH-13 states, read
//! series by series in record order and stopping at
//! each decision, and the decided races of a series
//! are read from the record that walk leaves, so no
//! second winner rule and no second counting walk
//! stands in this module.

use crate::championship::{count_series, tally_standings_of, Championship};

/// Return the depths report lines for one championship.
pub fn depths_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["depths".to_string()];
    let mut deep: u64 = 0;
    let mut matched: u64 = 0;
    let mut shallow: u64 = 0;
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
        // The depth of the series over the decided
        // record the walk leaves, read by the one
        // helper of the championship module, the
        // largest count by which the loser's tally of
        // decided wins stood above the winner's tally
        // after a decided race of the series, standing
        // at none where the winner never stood behind.
        // The winning margin is the count of the
        // winner's decided wins less the count of the
        // loser's decided wins, the figure DH-42
        // states and the walk leaves at the decision.
        let standings = tally_standings_of(&count.decided);
        let depth = standings[1 - champion];
        let margin = count.wins[champion] - count.wins[1 - champion];
        if depth < margin {
            shallow += 1;
            continue;
        }
        if depth == margin {
            matched += 1;
            continue;
        }
        deep += 1;
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
        lines.push(format!("deepest deficit {} over the decided record", depth));
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
        "depths, {} series, {} deep, {} matched, {} shallow, {} no winner",
        championship.series.len(),
        deep,
        matched,
        shallow,
        no_winner
    ));
    lines
}
