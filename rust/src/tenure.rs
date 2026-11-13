//! The tenure reading over a recorded championship,
//! DH-46 and DH-47 in CONTRACT.md.
//!
//! The reading classifies each decided series of a
//! championship by the count of the decided races
//! after which its winner's tally stood above its
//! loser's, read against the same count for the
//! loser. A decided series stands ahead when its
//! winner's front count stands above its loser's,
//! level when the two counts stand equal, and behind
//! when its winner's front count stands below its
//! loser's. A series holding no winner stands in
//! none of the three classes. One section stands for
//! each series standing ahead, naming its decided
//! races in race order, the front count of the
//! series, and its winner with its race split. The
//! counted races are the races of the counting walk
//! DH-13 states, read series by series in record
//! order and stopping at each decision, and the
//! decided races of a series are read from the
//! record that walk leaves, so no second winner rule
//! and no second counting walk stands in this module.

use crate::championship::{count_series, tally_standings_of, Championship};

/// Return the tenure report lines for one championship.
pub fn tenure_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["tenure".to_string()];
    let mut ahead: u64 = 0;
    let mut level: u64 = 0;
    let mut behind: u64 = 0;
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
        // The front count of each model over the
        // decided record the walk leaves, read by
        // the one helper of the championship module,
        // one standing counted at each decided race
        // after which that model's tally stood above
        // the other model's tally. Every decided race
        // leaves the tallies apart for one model or
        // apart for the other or level, so the two
        // front counts and the count of the standings
        // at level sum to the count of the decided
        // races, and the deciding race always stands
        // counted for the winner, whose front count
        // therefore stands at one or more on every
        // decided series.
        let standings = tally_standings_of(&count.decided);
        let front: [u64; 2] = [standings[3], standings[4]];
        if front[champion] < front[1 - champion] {
            behind += 1;
            continue;
        }
        if front[champion] == front[1 - champion] {
            level += 1;
            continue;
        }
        ahead += 1;
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
            "front count {} over the decided record",
            front[champion]
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
        "tenure, {} series, {} ahead, {} level, {} behind, {} no winner",
        championship.series.len(),
        ahead,
        level,
        behind,
        no_winner
    ));
    lines
}
