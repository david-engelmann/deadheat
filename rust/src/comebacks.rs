//! The comeback reading over a recorded championship,
//! DH-21 and DH-22 in CONTRACT.md.
//!
//! The reading classifies each series of a championship by
//! its first decided race. A series stands won from behind
//! when its winner lost the first decided race of the
//! series, and won from the front when its winner won that
//! race, and a series holding no winner stands in neither
//! class. One section stands for each series won from
//! behind, naming its decided races in race order, the
//! loss of its winner in the first decided race, and its
//! winner with its race split. The counted races are the
//! races of the counting walk DH-13 states, read series by
//! series in record order and stopping at each decision,
//! and the decided races of a series are read from the
//! record that walk leaves, so no second winner rule and
//! no second counting walk stands in this module.

use crate::championship::{count_series, Championship};

/// Return the comeback report lines for one championship.
pub fn comeback_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["comebacks".to_string()];
    let mut from_behind: u64 = 0;
    let mut from_front: u64 = 0;
    for (offset, series) in championship.series.iter().enumerate() {
        let count = count_series(series, &series.models[0]);
        let champion = match count.champion {
            Some(champion) => champion,
            None => continue,
        };
        // A champion stands only where a race was won, so a
        // decided series always holds a first decided race.
        let (first_race, first_side) = count.decided[0];
        if first_side == champion {
            from_front += 1;
            continue;
        }
        from_behind += 1;
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
            "first decided race, race {}, {} lost to {}",
            first_race,
            series.models[champion].display_line(),
            series.models[1 - champion].display_line()
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
        "comebacks, {} series, {} from behind, {} from the front",
        championship.series.len(),
        from_behind,
        from_front
    ));
    lines
}
