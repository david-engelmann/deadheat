//! Championships over recorded series, DH-13 and DH-14 in
//! CONTRACT.md.
//!
//! A championship file names a stated collection of series
//! files. The loader reads each named series whole or nothing
//! and never fails on malformed content, and the championship
//! report is a pure function of the loaded collection. The per
//! series outcomes and the race winners are read through the
//! standing series and winner machinery, and the aggregate
//! margin is carried in the exact arithmetic of DH-11.

use std::path::{Path, PathBuf};

use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{Signed, Zero};

use crate::json::{self, JsonValue};
use crate::pytext::PText;
use crate::races::{adjusted_finish, decide_winner, format_seconds_fraction, read_text_file};
use crate::series::{load_series_file, series_report, Series};

/// One loaded championship in its clean shape, the named
/// series in collection order.
#[derive(Clone, Debug, PartialEq)]
pub struct Championship {
    pub series: Vec<Series>,
}

/// The counted figures of one model over one championship.
struct Standing {
    model: PText,
    series_won: u64,
    stood_in: u64,
    race_wins: u64,
    no_results: u64,
    margin: Ratio<BigInt>,
}

impl Standing {
    fn blank(model: PText) -> Standing {
        Standing {
            model,
            series_won: 0,
            stood_in: 0,
            race_wins: 0,
            no_results: 0,
            margin: Ratio::zero(),
        }
    }
}

fn load_entries(data: &JsonValue, defects: &mut Vec<String>) -> Option<Vec<PText>> {
    let raw = match data.get("series") {
        None => {
            defects.push("missing field series".to_string());
            return None;
        }
        Some(value) => value,
    };
    let entries = match raw.as_array() {
        None => {
            defects.push("field series has the wrong type".to_string());
            return None;
        }
        Some(entries) => entries,
    };
    if entries.is_empty() {
        defects.push("field series holds no series files".to_string());
        return None;
    }
    let before = defects.len();
    let mut clean = Vec::new();
    for (index, entry) in entries.iter().enumerate() {
        match entry.as_text() {
            None => defects.push(format!("series entry {} has the wrong type", index)),
            Some(text) => {
                if text.is_empty() {
                    defects.push(format!("series entry {} is empty", index));
                } else if text.holds_unpaired_surrogate() {
                    defects.push(format!(
                        "series entry {} holds an unpaired surrogate",
                        index
                    ));
                } else if text.holds_line_break() {
                    defects.push(format!("series entry {} holds a newline", index));
                } else {
                    clean.push(text.clone());
                }
            }
        }
    }
    if defects.len() != before {
        return None;
    }
    Some(clean)
}

/// Parse championship text into a championship, reading each
/// named series against the given folder and naming every
/// defect found. A named series that fails to load refuses the
/// championship whole, with the defects of that series standing
/// under its series prefix.
pub fn load_championship_text(text: &str, base_dir: &Path) -> (Option<Championship>, Vec<String>) {
    let data = match json::parse(text) {
        Ok(data) => data,
        Err(_) => return (None, vec!["input is not valid JSON".to_string()]),
    };
    if data.as_object().is_none() {
        return (None, vec!["input is not a JSON object".to_string()]);
    }
    let mut defects = Vec::new();
    let entries = load_entries(&data, &mut defects);
    let mut loaded = Vec::new();
    if let Some(entries) = &entries {
        for (index, entry) in entries.iter().enumerate() {
            let path: PathBuf = base_dir.join(entry.display());
            let (series, series_defects) = load_series_file(&path);
            for defect in series_defects {
                defects.push(format!("series {} {}", index, defect));
            }
            if let Some(series) = series {
                loaded.push(series);
            }
        }
    }
    if !defects.is_empty() {
        return (None, defects);
    }
    (Some(Championship { series: loaded }), Vec::new())
}

/// Read a championship from disk, with the same contract as
/// text. Relative series names are read against the folder
/// holding the championship file.
pub fn load_championship_file(path: &Path) -> (Option<Championship>, Vec<String>) {
    let text = match read_text_file(path) {
        None => return (None, vec!["file cannot be read".to_string()]),
        Some(text) => text,
    };
    let base_dir: &Path = path.parent().unwrap_or_else(|| Path::new(""));
    load_championship_text(&text, base_dir)
}

/// Count one series under the counting walk of DH-13, folding
/// its counted figures into the standings. The walk reads the
/// races in record order and stops at the decision, so the race
/// reaching the wins target counts and every race after it
/// counts for nothing.
fn count_series(standings: &mut Vec<Standing>, series: &Series) {
    let mut positions: Vec<usize> = Vec::new();
    for model in &series.models {
        let position = match standings.iter().position(|held| held.model == *model) {
            Some(position) => position,
            None => {
                standings.push(Standing::blank(model.clone()));
                standings.len() - 1
            }
        };
        standings[position].stood_in += 1;
        positions.push(position);
    }
    let mut wins: Vec<u64> = vec![0, 0];
    let mut no_results: u64 = 0;
    let mut champion: Option<usize> = None;
    for race in &series.races {
        if champion.is_some() {
            break;
        }
        let (winner, _) = decide_winner(race);
        match winner {
            None => no_results += 1,
            Some(winner) => {
                let model = &race.attempts[winner].model;
                let side = if *model == series.models[0] { 0 } else { 1 };
                wins[side] += 1;
                let position = positions[side];
                standings[position].race_wins += 1;
                let first = adjusted_finish(&race.attempts[0]);
                let second = adjusted_finish(&race.attempts[1]);
                if let (Some(first), Some(second)) = (first, second) {
                    standings[position].margin += (first - second).abs();
                }
                if BigInt::from(wins[side]) >= series.wins_target {
                    champion = Some(side);
                }
            }
        }
    }
    for position in &positions {
        standings[*position].no_results += no_results;
    }
    if let Some(side) = champion {
        standings[positions[side]].series_won += 1;
    }
}

/// Return the championship report lines for one championship.
pub fn championship_report(championship: &Championship) -> Vec<String> {
    let mut lines = vec!["championship".to_string()];
    let mut standings: Vec<Standing> = Vec::new();
    for (offset, series) in championship.series.iter().enumerate() {
        let report = series_report(series);
        lines.push(format!("series {}, {}", offset + 1, series.task.display()));
        lines.push(report[1].clone());
        lines.push(report[report.len() - 1].clone());
        count_series(&mut standings, series);
    }
    lines.push("championship table".to_string());
    standings.sort_by(|a, b| {
        b.series_won
            .cmp(&a.series_won)
            .then_with(|| b.race_wins.cmp(&a.race_wins))
            .then_with(|| b.margin.cmp(&a.margin))
            .then_with(|| a.model.cmp_code_points(&b.model))
    });
    for held in &standings {
        lines.push(format!(
            "{}, {} series won, {} series stood in, {} race wins, {} no results, aggregate margin {} seconds",
            held.model.display(),
            held.series_won,
            held.stood_in,
            held.race_wins,
            held.no_results,
            format_seconds_fraction(&held.margin)
        ));
    }
    lines
}
