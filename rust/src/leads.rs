//! The lead reading over a recorded championship,
//! DH-42 and DH-43 in CONTRACT.md.
//!
//! The reading classifies each decided series of a
//! championship by the largest lead either model held
//! over its decided record, read against the margin
//! the series was won by. A decided series stands
//! wide when its largest lead stands above its
//! winning margin, and narrow when its largest lead
//! stands equal to its winning margin, and it cannot
//! stand below, because after the deciding race the
//! winner's tally stands above the loser's by the
//! margin. A series holding no winner stands in
//! neither class. One section stands for each series
//! standing wide, naming its decided races in race
//! order, the largest lead of the series, and its
//! winner with its race split. The counted races are
//! the races of the counting walk DH-13 states, read
//! series by series in record order and stopping at
//! each decision, and the decided races of a series
//! are read from the record that walk leaves, so no
//! second winner rule and no second counting walk
//! stands in this module.

use crate::championship::{count_series, tally_standings_of, Championship};

/// Return the lead report lines for one championship.
pub fn lead_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["lead".to_string()];
    let mut wide: u64 = 0;
    let mut narrow: u64 = 0;
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
        // The largest lead of each model over the
        // decided record the walk leaves, read by the
        // one helper of the championship module, the
        // largest count by which its tally of decided
        // wins stood above the other model's tally
        // after a decided race of the series, standing
        // at none where its tally never stood above.
        // The largest lead of the series is the larger
        // of the two models' largest leads, and the
        // winning margin is the count of the winner's
        // decided wins less the count of the loser's
        // decided wins, the figure the walk leaves at
        // the decision.
        let standings = tally_standings_of(&count.decided);
        let height = standings[0].max(standings[1]);
        let margin = count.wins[champion] - count.wins[1 - champion];
        if height <= margin {
            narrow += 1;
            continue;
        }
        wide += 1;
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
        lines.push(format!("largest lead {} over the decided record", height));
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
        "lead, {} series, {} wide, {} narrow, {} no winner",
        championship.series.len(),
        wide,
        narrow,
        no_winner
    ));
    lines
}
