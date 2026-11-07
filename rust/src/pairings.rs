//! The pairing reading over a recorded championship, DH-19
//! and DH-20 in CONTRACT.md.
//!
//! The reading groups the series of a championship by the
//! pair of models each series names and tallies, over the
//! series of each pair, the series won, the race wins, the
//! no results, and the aggregate margin of each model. The
//! counted races are the races of the counting walk DH-13
//! states, read series by series in record order and
//! stopping at each decision, and the winner of each race
//! comes from the one winner decision, so no second winner
//! rule and no second counting walk stands in this module.
//! Membership in a pair follows the models field of a
//! series, as DH-19 states, so a series holding no races
//! still stands its pair in the reading with zero counts.

use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::Zero;

use crate::championship::{count_series, Championship};
use crate::pytext::PText;
use crate::races::format_seconds_fraction;
use crate::series::Series;

/// The counted figures of one model over the series of one
/// pair.
struct PairStanding {
    series_won: u64,
    race_wins: u64,
    no_results: u64,
    margin: Ratio<BigInt>,
}

impl PairStanding {
    fn blank() -> PairStanding {
        PairStanding {
            series_won: 0,
            race_wins: 0,
            no_results: 0,
            margin: Ratio::zero(),
        }
    }
}

/// One pair of the reading, its two models in the order the
/// first series standing in the pair names them, and the
/// figures counted over the series standing in it.
struct Pair {
    models: [PText; 2],
    series_read: u64,
    races_read: u64,
    standings: [PairStanding; 2],
}

impl Pair {
    fn blank(first: &PText, second: &PText) -> Pair {
        Pair {
            models: [first.clone(), second.clone()],
            series_read: 0,
            races_read: 0,
            standings: [PairStanding::blank(), PairStanding::blank()],
        }
    }

    /// Return whether a series names this pair, in either
    /// order, since a pair is the two models and its order
    /// stands from the first series that named them.
    fn names(&self, series: &Series) -> bool {
        (series.models[0] == self.models[0] && series.models[1] == self.models[1])
            || (series.models[0] == self.models[1] && series.models[1] == self.models[0])
    }
}

/// Count one series into its pair through the one
/// counting walk of DH-13, which stands in the
/// championship module. The walk states its figures per
/// side of the order this pair holds its models in, so
/// the count folds straight into the pair standings, and
/// the races the walk read join the pair's races read.
fn count_into_pair(pair: &mut Pair, series: &Series) {
    let count = count_series(series, &pair.models[0]);
    pair.series_read += 1;
    pair.races_read += count.races_walked;
    for side in 0..2 {
        let held = &mut pair.standings[side];
        held.race_wins += count.wins[side];
        held.no_results += count.no_results;
        held.margin += count.margins[side].clone();
    }
    if let Some(side) = count.champion {
        pair.standings[side].series_won += 1;
    }
}

/// Return the pairing report lines for one championship.
pub fn pairing_report(championship: &Championship) -> Vec<String> {
    let mut pairs: Vec<Pair> = Vec::new();
    for series in &championship.series {
        let position = match pairs.iter().position(|held| held.names(series)) {
            Some(position) => position,
            None => {
                pairs.push(Pair::blank(&series.models[0], &series.models[1]));
                pairs.len() - 1
            }
        };
        count_into_pair(&mut pairs[position], series);
    }
    let mut lines = vec!["pairings".to_string()];
    let mut races_read: u64 = 0;
    for pair in &pairs {
        races_read += pair.races_read;
        lines.push(format!(
            "pairing, {}, against, {}",
            pair.models[0].display_line(),
            pair.models[1].display_line()
        ));
        lines.push(format!("series read, {}", pair.series_read));
        lines.push(format!("races read, {}", pair.races_read));
        for (offset, model) in pair.models.iter().enumerate() {
            let held = &pair.standings[offset];
            lines.push(format!(
                "{}, {} series won, {} race wins, {} no results, aggregate margin {} seconds",
                model.display_line(),
                held.series_won,
                held.race_wins,
                held.no_results,
                format_seconds_fraction(&held.margin)
            ));
        }
    }
    lines.push(format!(
        "pairings, {} pairings, {} series, {} races",
        pairs.len(),
        championship.series.len(),
        races_read
    ));
    lines
}
