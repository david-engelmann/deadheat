//! The race specification of DH-27, the input of the run
//! command.
//!
//! A race specification is one JSON object holding the task
//! label under task, the race workspace under workspace, the
//! race record file the run writes under record, and the two
//! attempts under attempts. The loader returns the
//! specification and a list of defects, names every malformed
//! field it finds in one pass, and never raises on malformed
//! content, the discipline every loader of this repo stands
//! in. The run command runs nothing over a specification
//! carrying any defect.

use std::path::Path;

use crate::json::{self, JsonValue};
use crate::pytext::PText;
use crate::races::{check_text_field_shared, check_version, read_text_file};
use crate::runner::AttemptSpec;

/// One loaded race specification, ready for the runner.
pub struct RaceSpec {
    pub task: PText,
    pub workspace: PText,
    pub record: PText,
    pub attempts: Vec<AttemptSpec>,
}

fn check_path_field(data: &JsonValue, name: &str, defects: &mut Vec<String>) -> Option<PText> {
    match data.get(name) {
        None => {
            defects.push(format!("missing field {}", name));
            None
        }
        Some(value) => match value.as_text() {
            None => {
                defects.push(format!("field {} has the wrong type", name));
                None
            }
            Some(text) => {
                if text.is_empty() {
                    defects.push(format!("field {} is empty", name));
                    None
                } else {
                    Some(text.clone())
                }
            }
        },
    }
}

fn check_attempt_label(
    entry: &JsonValue,
    index: usize,
    name: &str,
    defects: &mut Vec<String>,
) -> Option<PText> {
    let value = match entry.get(name) {
        None => {
            defects.push(format!("attempt {} is missing field {}", index, name));
            return None;
        }
        Some(value) => value,
    };
    let Some(text) = value.as_text() else {
        defects.push(format!(
            "attempt {} field {} has the wrong type",
            index, name
        ));
        return None;
    };
    if text.is_empty() {
        defects.push(format!("attempt {} field {} is empty", index, name));
        return None;
    }
    if text.holds_unpaired_surrogate() {
        defects.push(format!(
            "attempt {} field {} holds an unpaired surrogate",
            index, name
        ));
        return None;
    }
    if text.holds_line_break() {
        defects.push(format!("attempt {} field {} holds a newline", index, name));
        return None;
    }
    Some(text.clone())
}

fn check_attempt_command(
    entry: &JsonValue,
    index: usize,
    defects: &mut Vec<String>,
) -> Option<String> {
    let value = match entry.get("command") {
        None => {
            defects.push(format!("attempt {} is missing field command", index));
            return None;
        }
        Some(value) => value,
    };
    let Some(text) = value.as_text() else {
        defects.push(format!(
            "attempt {} field command has the wrong type",
            index
        ));
        return None;
    };
    if text.is_empty() {
        defects.push(format!("attempt {} field command is empty", index));
        return None;
    }
    Some(text.display())
}

fn check_attempt_timeout(
    entry: &JsonValue,
    index: usize,
    defects: &mut Vec<String>,
) -> Option<Option<f64>> {
    let value = match entry.get("timeout_seconds") {
        None => return Some(None),
        Some(value) => value,
    };
    let Some(num) = value.as_num() else {
        defects.push(format!(
            "attempt {} field timeout_seconds has the wrong type",
            index
        ));
        return None;
    };
    if !num.is_finite_number() {
        defects.push(format!(
            "attempt {} field timeout_seconds is not a finite number",
            index
        ));
        return None;
    }
    if num.is_negative() {
        defects.push(format!(
            "attempt {} field timeout_seconds is below zero",
            index
        ));
        return None;
    }
    let seconds = match num {
        crate::pynum::Num::Int(digits) => {
            use num_traits::ToPrimitive;
            digits.to_f64().unwrap_or(0.0)
        }
        crate::pynum::Num::Float(value) => *value,
    };
    Some(Some(seconds))
}

fn load_attempt(entry: &JsonValue, index: usize, defects: &mut Vec<String>) -> Option<AttemptSpec> {
    if entry.as_object().is_none() {
        defects.push(format!("attempt {} is not an object", index));
        return None;
    }
    let before = defects.len();
    let model = check_attempt_label(entry, index, "model", defects);
    let environment = check_attempt_label(entry, index, "environment", defects);
    let command = check_attempt_command(entry, index, defects);
    let timeout_seconds = check_attempt_timeout(entry, index, defects);
    if defects.len() != before {
        return None;
    }
    Some(AttemptSpec {
        model: model.expect("a clean model"),
        environment: environment.expect("a clean environment"),
        command: command.expect("a clean command"),
        timeout_seconds: timeout_seconds.expect("a clean timeout"),
    })
}

fn load_attempts(data: &JsonValue, defects: &mut Vec<String>) -> Option<Vec<AttemptSpec>> {
    let value = match data.get("attempts") {
        None => {
            defects.push("missing field attempts".to_string());
            return None;
        }
        Some(value) => value,
    };
    let Some(entries) = value.as_array() else {
        defects.push("field attempts has the wrong type".to_string());
        return None;
    };
    if entries.len() != 2 {
        defects.push("field attempts must hold exactly two attempts".to_string());
        return None;
    }
    let mut attempts = Vec::new();
    let mut clean = true;
    for (index, entry) in entries.iter().enumerate() {
        match load_attempt(entry, index, defects) {
            Some(attempt) => attempts.push(attempt),
            None => clean = false,
        }
    }
    if clean {
        Some(attempts)
    } else {
        None
    }
}

/// Read a race specification from its parsed JSON object,
/// returning the specification beside the defect list. A
/// malformed object returns no specification.
pub fn load_spec_data(data: &JsonValue) -> (Option<RaceSpec>, Vec<String>) {
    if data.as_object().is_none() {
        return (None, vec!["input is not a JSON object".to_string()]);
    }
    let mut defects = Vec::new();
    check_version(
        data,
        "field version has the wrong type",
        "field version names an unsupported version",
        &mut defects,
    );
    let task = check_text_field_shared(data, "task", &mut defects);
    let workspace = check_path_field(data, "workspace", &mut defects);
    let record = check_path_field(data, "record", &mut defects);
    let attempts = load_attempts(data, &mut defects);
    if !defects.is_empty() {
        return (None, defects);
    }
    (
        Some(RaceSpec {
            task: task.expect("a clean task"),
            workspace: workspace.expect("a clean workspace"),
            record: record.expect("a clean record"),
            attempts: attempts.expect("clean attempts"),
        }),
        defects,
    )
}

/// Read a race specification from JSON text.
pub fn load_spec_text(text: &str) -> (Option<RaceSpec>, Vec<String>) {
    let data = match json::parse(text) {
        Ok(data) => data,
        Err(_) => return (None, vec!["input is not valid JSON".to_string()]),
    };
    load_spec_data(&data)
}

/// Read a race specification from a file. A file that cannot
/// be read draws the standing defect for it.
pub fn load_spec_file(path: &Path) -> (Option<RaceSpec>, Vec<String>) {
    match read_text_file(path) {
        Some(text) => load_spec_text(&text),
        None => (None, vec!["file cannot be read".to_string()]),
    }
}
