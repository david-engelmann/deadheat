//! The streak reading over a recorded championship,
//! DH-32 and DH-33 in CONTRACT.md.
//!
//! The reading classifies each decided series of a
//! championship by the longest run of consecutive decided
//! races its winner won. A decided series stands strung
//! when that longest run stands at two or more, and
//! single when it stands at one, and a series holding no
//! winner stands in neither class. One section stands
//! for each strung series, naming its decided races in
//! race order, the longest run of its winner, and its
//! winner with its race split. The counted races are the
//! races of the counting walk DH-13 states, read series
//! by series in record order and stopping at each
//! decision, and the decided races of a series are read
//! from the record that walk leaves, so no second winner
//! rule and no second counting walk stands in this
//! module.

use crate::championship::{count_series, Championship};

/// Return the streak report lines for one championship.
pub fn streak_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["streak".to_string()];
    let mut strung: u64 = 0;
    let mut single: u64 = 0;
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
        // The longest run of consecutive decided races won
        // by the series winner, read over the decided
        // record the walk leaves. A race holding no winner
        // stands in no decided record, so it neither
        // extends nor breaks a run, and the runs stand
        // read over the decided record alone.
        let mut longest = 0u64;
        let mut run = 0u64;
        for (_, side) in &count.decided {
            if *side == champion {
                run += 1;
                if run > longest {
                    longest = run;
                }
            } else {
                run = 0;
            }
        }
        if longest < 2 {
            single += 1;
            continue;
        }
        strung += 1;
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
            "longest run of {} consecutive decided race wins",
            longest
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
        "streak, {} series, {} strung, {} single, {} no winner",
        championship.series.len(),
        strung,
        single,
        no_winner
    ));
    lines
}
