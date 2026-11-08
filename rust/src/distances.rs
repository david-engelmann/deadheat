//! The distance reading over a recorded championship,
//! DH-23 and DH-24 in CONTRACT.md.
//!
//! The reading classifies each series of a championship by
//! the wins of its loser when the decision fell. A decided
//! series stands without reply when its loser won no
//! decided race, it stands the distance when its loser
//! stood one win short of the wins target, and it stands
//! in between in every other case, and a series holding
//! no winner stands in no class. One section stands for
//! each series gone the distance, naming its decided
//! races in race order and its winner with its race
//! split. The counted races are the races of the counting
//! walk DH-13 states, read series by series in record
//! order and stopping at each decision, and the decided
//! races of a series are read from the record that walk
//! leaves, so no second winner rule and no second
//! counting walk stands in this module.

use num_bigint::BigInt;

use crate::championship::{count_series, Championship};

/// Return the distance report lines for one championship.
pub fn distance_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["distance".to_string()];
    let mut the_distance: u64 = 0;
    let mut without_reply: u64 = 0;
    let mut in_between: u64 = 0;
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
        // The wins of the loser stand as the walk left them
        // when the deciding race fell, over the counted
        // races alone, so a win recorded after the decision
        // moves no series between classes. A loser standing
        // at none stands without reply, and a loser standing
        // at the target less one stands the distance, and
        // that figure stands above none wherever it stands,
        // because a series of target one leaves its loser at
        // none and so stands without reply alone.
        let loser_wins = count.wins[1 - champion];
        if loser_wins == 0 {
            without_reply += 1;
            continue;
        }
        if BigInt::from(loser_wins) != &series.wins_target - BigInt::from(1) {
            in_between += 1;
            continue;
        }
        the_distance += 1;
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
        "distance, {} series, {} the distance, {} without reply, {} in between, {} no winner",
        championship.series.len(),
        the_distance,
        without_reply,
        in_between,
        no_winner
    ));
    lines
}
