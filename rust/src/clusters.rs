//! The cluster reading over a recorded championship,
//! DH-34 and DH-35 in CONTRACT.md.
//!
//! The reading classifies each decided series of a
//! championship by the longest run of consecutive decided
//! races its loser won. A decided series stands clustered
//! when that longest run stands at two or more, and
//! scattered when it stands at one or at none, and a
//! series holding no winner stands in neither class. One
//! section stands for each clustered series, naming its
//! decided races in race order, the longest run of its
//! loser, and its winner with its race split. The counted
//! races are the races of the counting walk DH-13 states,
//! read series by series in record order and stopping at
//! each decision, and the decided races of a series are
//! read from the record that walk leaves, so no second
//! winner rule and no second counting walk stands in this
//! module.

use crate::championship::{count_series, longest_run_of, Championship};

/// Return the cluster report lines for one championship.
pub fn cluster_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["cluster".to_string()];
    let mut clustered: u64 = 0;
    let mut scattered: u64 = 0;
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
        let loser = 1 - champion;
        // The longest run of consecutive decided races won
        // by the loser of the series, read by the one helper
        // of the championship module over the decided
        // record the walk leaves.
        let longest = longest_run_of(&count.decided, loser);
        if longest < 2 {
            scattered += 1;
            continue;
        }
        clustered += 1;
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
            "longest run against {} consecutive decided race wins",
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
        "cluster, {} series, {} clustered, {} scattered, {} no winner",
        championship.series.len(),
        clustered,
        scattered,
        no_winner
    ));
    lines
}
