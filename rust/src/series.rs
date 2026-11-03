//! Series over recorded races, DH-6 in CONTRACT.md.
//!
//! A series file holds recorded races between the same two
//! models and a wins target. The loader names every defect it
//! finds and never fails on malformed content, and the series
//! report is a pure function of the loaded series. The Python
//! module of the same name is the behavioral spec.

use std::path::Path;

use num_bigint::BigInt;
use num_traits::Zero;

use crate::json::{self, JsonValue};
use crate::pytext::PText;
use crate::races::{check_text_field_shared, decide_winner, load_race_data, read_text_file, Race};

/// One loaded series in its clean shape.
#[derive(Clone, Debug, PartialEq)]
pub struct Series {
    pub task: PText,
    pub models: Vec<PText>,
    pub wins_target: BigInt,
    pub races: Vec<Race>,
}

fn load_models(data: &JsonValue, defects: &mut Vec<String>) -> Option<Vec<PText>> {
    let raw = match data.get("models") {
        None => {
            defects.push("missing field models".to_string());
            return None;
        }
        Some(value) => value,
    };
    let entries = match raw.as_array() {
        None => {
            defects.push("field models has the wrong type".to_string());
            return None;
        }
        Some(entries) => entries,
    };
    let before = defects.len();
    if entries.len() != 2 {
        defects.push("field models must hold exactly two models".to_string());
    }
    let mut clean = Vec::new();
    for (index, entry) in entries.iter().enumerate() {
        match entry.as_text() {
            None => defects.push(format!("models entry {} has the wrong type", index)),
            Some(text) => {
                if text.is_empty() {
                    defects.push(format!("models entry {} is empty", index));
                } else if text.holds_unpaired_surrogate() {
                    defects.push(format!(
                        "models entry {} holds an unpaired surrogate",
                        index
                    ));
                } else if text.holds_line_break() {
                    defects.push(format!("models entry {} holds a newline", index));
                } else {
                    clean.push(text.clone());
                }
            }
        }
    }
    if clean.len() == 2 && clean[0] == clean[1] {
        defects.push("field models names the same model twice".to_string());
    }
    if defects.len() != before {
        return None;
    }
    Some(clean)
}

fn load_wins_target(data: &JsonValue, defects: &mut Vec<String>) -> Option<BigInt> {
    let value = match data.get("wins_target") {
        None => {
            defects.push("missing field wins_target".to_string());
            return None;
        }
        Some(value) => value,
    };
    match crate::races::as_int(value) {
        None => {
            defects.push("field wins_target has the wrong type".to_string());
            None
        }
        Some(digits) => {
            if *digits < BigInt::from(1) {
                defects.push("field wins_target is not positive".to_string());
                None
            } else {
                Some(digits.clone())
            }
        }
    }
}

fn load_series_races(
    data: &JsonValue,
    models: Option<&Vec<PText>>,
    defects: &mut Vec<String>,
) -> Option<Vec<Race>> {
    let raw = match data.get("races") {
        None => {
            defects.push("missing field races".to_string());
            return None;
        }
        Some(value) => value,
    };
    let entries = match raw.as_array() {
        None => {
            defects.push("field races has the wrong type".to_string());
            return None;
        }
        Some(entries) => entries,
    };
    let before = defects.len();
    let mut clean = Vec::new();
    for (index, entry) in entries.iter().enumerate() {
        if entry.as_object().is_none() {
            defects.push(format!("race {} is not an object", index));
            continue;
        }
        let (race, race_defects) = load_race_data(entry);
        for defect in race_defects {
            defects.push(format!("race {} {}", index, defect));
        }
        let race = match race {
            None => continue,
            Some(race) => race,
        };
        if let Some(models) = models {
            let named: Vec<&PText> = race.attempts.iter().map(|attempt| &attempt.model).collect();
            if named.iter().any(|model| !models.contains(model)) {
                defects.push(format!("race {} names a model outside the series", index));
                continue;
            }
            if named[0] == named[1] {
                defects.push(format!("race {} names the same model twice", index));
                continue;
            }
        }
        clean.push(race);
    }
    if defects.len() != before {
        return None;
    }
    Some(clean)
}

/// Parse series text into a series, naming every defect found.
pub fn load_series_text(text: &str) -> (Option<Series>, Vec<String>) {
    let data = match json::parse(text) {
        Ok(data) => data,
        Err(_) => return (None, vec!["input is not valid JSON".to_string()]),
    };
    if data.as_object().is_none() {
        return (None, vec!["input is not a JSON object".to_string()]);
    }
    let mut defects = Vec::new();
    let task = check_text_field_shared(&data, "task", &mut defects);
    let models = load_models(&data, &mut defects);
    let wins_target = load_wins_target(&data, &mut defects);
    let races = load_series_races(&data, models.as_ref(), &mut defects);
    if !defects.is_empty() {
        return (None, defects);
    }
    (
        Some(Series {
            task: task.expect("a clean task"),
            models: models.expect("clean models"),
            wins_target: wins_target.expect("a clean target"),
            races: races.expect("clean races"),
        }),
        Vec::new(),
    )
}

/// Read a series from disk, with the same contract as text.
pub fn load_series_file(path: &Path) -> (Option<Series>, Vec<String>) {
    let text = match read_text_file(path) {
        None => return (None, vec!["file cannot be read".to_string()]),
        Some(text) => text,
    };
    load_series_text(&text)
}

/// Return the series report lines for one series.
pub fn series_report(series: &Series) -> Vec<String> {
    let first_model = &series.models[0];
    let second_model = &series.models[1];
    let target = &series.wins_target;
    let mut lines = vec![
        format!("series {}", series.task.display()),
        format!(
            "{} against {}, first to {} wins",
            first_model.display(),
            second_model.display(),
            target
        ),
    ];
    let mut first_wins = BigInt::zero();
    let mut second_wins = BigInt::zero();
    let mut no_results = 0usize;
    let mut champion: Option<&PText> = None;
    for (offset, race) in series.races.iter().enumerate() {
        let index = offset + 1;
        if champion.is_some() {
            lines.push(format!(
                "race {}, {}, beyond the decision",
                index,
                race.task.display()
            ));
            continue;
        }
        let (winner, _) = decide_winner(race);
        let winner_model = winner.map(|winner| &race.attempts[winner].model);
        match winner_model {
            None => {
                no_results += 1;
                lines.push(format!(
                    "race {}, {}, no result",
                    index,
                    race.task.display()
                ));
            }
            Some(model) => {
                if *model == *first_model {
                    first_wins += 1;
                } else {
                    second_wins += 1;
                }
                lines.push(format!(
                    "race {}, {}, {} wins",
                    index,
                    race.task.display(),
                    model.display()
                ));
            }
        }
        lines.push(format!(
            "tally, {} {} wins, {} {} wins, {} no results",
            first_model.display(),
            first_wins,
            second_model.display(),
            second_wins,
            no_results
        ));
        if let Some(model) = winner_model {
            let wins = if *model == *first_model {
                &first_wins
            } else {
                &second_wins
            };
            if wins >= target {
                champion = Some(model);
            }
        }
    }
    match champion {
        Some(model) => lines.push(format!("series winner, {}", model.display())),
        None => lines.push(format!(
            "series unfinished, the target of {} wins is not reached",
            target
        )),
    }
    lines
}

/// Render a series as the JSON value the makers write, in the
/// shape the loader reads back.
pub fn series_to_json(series: &Series) -> JsonValue {
    JsonValue::object(vec![
        (
            PText::from_text("task"),
            JsonValue::Str(series.task.clone()),
        ),
        (
            PText::from_text("models"),
            JsonValue::Arr(
                series
                    .models
                    .iter()
                    .map(|model| JsonValue::Str(model.clone()))
                    .collect(),
            ),
        ),
        (
            PText::from_text("wins_target"),
            JsonValue::Num(crate::pynum::Num::Int(series.wins_target.clone())),
        ),
        (
            PText::from_text("races"),
            JsonValue::Arr(
                series
                    .races
                    .iter()
                    .map(crate::races::race_to_json)
                    .collect(),
            ),
        ),
    ])
}
