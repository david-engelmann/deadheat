//! The crossings reading over a recorded championship,
//! DH-44 and DH-45 in CONTRACT.md.
//!
//! The reading classifies each decided series of a
//! championship by the count of the crossings over
//! its decided record, the decided races after which
//! the two tallies stood equal having stood apart
//! after the decided race before it, read against
//! the count of its changes of hand. A decided
//! series stands through when its crossings count
//! stands equal to its count of changes of hand, and
//! short when its crossings count stands below it,
//! and it cannot stand above, because the tally
//! difference moves by one at each decided race, so
//! no change of hand passes between two decided
//! races without a standing at level between them.
//! A series holding no winner stands in neither
//! class. One section stands for each series
//! standing through, naming its decided races in
//! race order, the crossings of the series, and its
//! winner with its race split. The counted races are
//! the races of the counting walk DH-13 states, read
//! series by series in record order and stopping at
//! each decision, and the decided races of a series
//! are read from the record that walk leaves, so no
//! second winner rule and no second counting walk
//! stands in this module.

use crate::championship::{count_series, tally_standings_of, Championship};

/// Return the crossings report lines for one championship.
pub fn crossings_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["crossings".to_string()];
    let mut through: u64 = 0;
    let mut short: u64 = 0;
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
        // The count of the crossings over the decided
        // record the walk leaves, read by the one
        // helper of the championship module, one
        // crossing at each decided race after the
        // first where the two tallies stood equal
        // after it, the tallies standing apart after
        // the decided race before it by the one win
        // that race recorded, and the count of the
        // changes of hand over the same record, one
        // change at each place where one decided race
        // was won by one model and the next decided
        // race was won by the other. The crossings
        // stand at or below the changes of hand, a run
        // of the record after the first holding at
        // most one crossing.
        let standings = tally_standings_of(&count.decided);
        let crossings = standings[2];
        let mut changes: u64 = 0;
        for pair in count.decided.windows(2) {
            if pair[0].1 != pair[1].1 {
                changes += 1;
            }
        }
        if crossings < changes {
            short += 1;
            continue;
        }
        through += 1;
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
        lines.push(format!("crossings {} over the decided record", crossings));
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
        "crossings, {} series, {} through, {} short, {} no winner",
        championship.series.len(),
        through,
        short,
        no_winner
    ));
    lines
}
