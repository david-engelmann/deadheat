//! The totals reading over a recorded championship, DH-17
//! and DH-18 in CONTRACT.md.
//!
//! The reading tallies the counted races of a championship
//! per model, the races entered, won, lost, and left without
//! a result, and the token spend the attempts reported. The
//! counted races are the races of the counting walk DH-13
//! states, read series by series in record order and
//! stopping at each decision. The winner of each race comes
//! from the one winner decision, and the spend is carried
//! as an exact integer sum, so no second winner rule and no
//! rounded figure stands in this module.

use num_bigint::BigInt;
use num_traits::Zero;

use crate::championship::Championship;
use crate::pytext::PText;
use crate::races::decide_winner;
use crate::series::Series;

/// The tallied figures of one model over the counted races
/// of one championship.
struct Tally {
    model: PText,
    entered: u64,
    won: u64,
    lost: u64,
    no_results: u64,
    tokens: BigInt,
    reported: u64,
}

impl Tally {
    fn blank(model: PText) -> Tally {
        Tally {
            model,
            entered: 0,
            won: 0,
            lost: 0,
            no_results: 0,
            tokens: BigInt::zero(),
            reported: 0,
        }
    }
}

/// Count one series under the counting walk of DH-17 into
/// the tallies, registering each model at its first standing
/// as an attempt's model. The walk reads the races in record
/// order and stops at the decision, so the race reaching the
/// wins target counts and every race after it counts for
/// nothing. The count of races the walk counted is returned.
fn count_series(tallies: &mut Vec<Tally>, series: &Series) -> u64 {
    let mut counted: u64 = 0;
    let mut wins: Vec<u64> = vec![0, 0];
    let mut decided = false;
    for race in &series.races {
        if decided {
            break;
        }
        counted += 1;
        let mut positions: Vec<usize> = Vec::new();
        for attempt in &race.attempts {
            let position = match tallies.iter().position(|held| held.model == attempt.model) {
                Some(position) => position,
                None => {
                    tallies.push(Tally::blank(attempt.model.clone()));
                    tallies.len() - 1
                }
            };
            tallies[position].entered += 1;
            if let Some(tokens) = &attempt.tokens {
                tallies[position].tokens += tokens.clone();
                tallies[position].reported += 1;
            }
            positions.push(position);
        }
        let (winner, _) = decide_winner(race);
        match winner {
            None => {
                tallies[positions[0]].no_results += 1;
                tallies[positions[1]].no_results += 1;
            }
            Some(winner) => {
                let loser = 1 - winner;
                tallies[positions[winner]].won += 1;
                tallies[positions[loser]].lost += 1;
                let model = &race.attempts[winner].model;
                let side = if *model == series.models[0] { 0 } else { 1 };
                wins[side] += 1;
                if BigInt::from(wins[side]) >= series.wins_target {
                    decided = true;
                }
            }
        }
    }
    counted
}

/// Return the totals report lines for one championship.
pub fn totals_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["totals".to_string()];
    let mut tallies: Vec<Tally> = Vec::new();
    let mut races_read: u64 = 0;
    for series in &championship.series {
        races_read += count_series(&mut tallies, series);
    }
    for tally in &tallies {
        let spend = if tally.reported == 0 {
            "tokens not reported".to_string()
        } else {
            format!("{} tokens", tally.tokens)
        };
        lines.push(format!(
            "{}, {} races, {} wins, {} losses, {} no results, {}",
            tally.model.display(),
            tally.entered,
            tally.won,
            tally.lost,
            tally.no_results,
            spend
        ));
    }
    lines.push(format!(
        "totals, {} series, {} races, {} models",
        championship.series.len(),
        races_read,
        tallies.len()
    ));
    lines
}
