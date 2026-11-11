//! The turns reading over a recorded championship,
//! DH-40 and DH-41 in CONTRACT.md.
//!
//! The reading classifies each decided series of a
//! championship by the count of the changes of hand
//! over its decided record, the places where one
//! decided race was won by one model and the next
//! decided race was won by the other. A decided series
//! stands turned when its record changes hands two or
//! more times, and steady when it changes hands once
//! or not at all, and a series holding no winner stands
//! in neither class. One section stands for each series
//! standing turned, naming its decided races in race
//! order, the changes of hand of the series, and its
//! winner with its race split. The counted races are
//! the races of the counting walk DH-13 states, read
//! series by series in record order and stopping at
//! each decision, and the decided races of a series are
//! read from the record that walk leaves, so no second
//! winner rule and no second counting walk stands in
//! this module.

use crate::championship::{count_series, Championship};

/// Return the turns report lines for one championship.
pub fn turns_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["turns".to_string()];
    let mut turned: u64 = 0;
    let mut steady: u64 = 0;
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
        // The count of the changes of hand over the
        // decided record the walk leaves, one change at
        // each place where one decided race was won by
        // one model and the next decided race was won by
        // the other, standing one below the count of the
        // record's runs. A race holding no winner stands
        // in no decided record, so it neither stands in
        // the count nor breaks it, and the count stands
        // read over the decided record alone.
        let mut changes: u64 = 0;
        for pair in count.decided.windows(2) {
            if pair[0].1 != pair[1].1 {
                changes += 1;
            }
        }
        if changes < 2 {
            steady += 1;
            continue;
        }
        turned += 1;
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
            "changes of hand {} over the decided record",
            changes
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
        "turns, {} series, {} turned, {} steady, {} no winner",
        championship.series.len(),
        turned,
        steady,
        no_winner
    ));
    lines
}
