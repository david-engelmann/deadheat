//! The closing reading over a recorded championship,
//! DH-36 and DH-37 in CONTRACT.md.
//!
//! The reading classifies each decided series of a
//! championship by the closing run of consecutive decided
//! races its winner won, the run that ends its decided
//! record. A decided series stands in a run when its
//! closing run stands at two or more, and alone when it
//! stands at one, and a series holding no winner stands
//! in neither class. One section stands for each series
//! standing in a run, naming its decided races in race
//! order, the closing run of its winner, and its winner
//! with its race split. The counted races are the races
//! of the counting walk DH-13 states, read series by
//! series in record order and stopping at each decision,
//! and the decided races of a series are read from the
//! record that walk leaves, so no second winner rule and
//! no second counting walk stands in this module.

use crate::championship::{count_series, Championship};

/// Return the closing report lines for one championship.
pub fn closing_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["closing".to_string()];
    let mut in_a_run: u64 = 0;
    let mut alone: u64 = 0;
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
        // The closing run of consecutive decided races won
        // by the series winner, read over the decided
        // record the walk leaves from its end. A race
        // holding no winner stands in no decided record,
        // so it neither extends nor breaks a run, and the
        // runs stand read over the decided record alone.
        let mut closing_run = 0u64;
        for (_, side) in count.decided.iter().rev() {
            if *side == champion {
                closing_run += 1;
            } else {
                break;
            }
        }
        if closing_run < 2 {
            alone += 1;
            continue;
        }
        in_a_run += 1;
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
            "closing run of {} consecutive decided race wins",
            closing_run
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
        "closing, {} series, {} in a run, {} alone, {} no winner",
        championship.series.len(),
        in_a_run,
        alone,
        no_winner
    ));
    lines
}
