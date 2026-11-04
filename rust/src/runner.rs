//! The local runner for deadheat.
//!
//! The runner runs one shell command per attempt, each in its
//! own directory under one race workspace, and builds the
//! attempt record the loader in races accepts. It is offline
//! and local only. The obligations this module carries are in
//! the DH-1 runner text in CONTRACT.md, in the full form
//! section. The Python module of the same name is the
//! behavioral spec.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use num_bigint::BigInt;

use crate::json::{self, JsonValue};
use crate::pytext::PText;
use crate::races::{Attempt, Race, SuiteResult};

/// Create the lane directory and remove everything it holds.
fn empty_directory(directory: &Path) {
    let _ = std::fs::create_dir_all(directory);
    if let Ok(entries) = std::fs::read_dir(directory) {
        for entry in entries.flatten() {
            let path = entry.path();
            let metadata = std::fs::symlink_metadata(&path);
            let is_plain_dir = match metadata {
                Ok(metadata) => metadata.is_dir() && !metadata.file_type().is_symlink(),
                Err(_) => false,
            };
            if is_plain_dir {
                let _ = std::fs::remove_dir_all(&path);
            } else {
                let _ = std::fs::remove_file(&path);
            }
        }
    }
}

/// How reading the result file the command wrote ends. A
/// file that reads as a JSON object hands its value back, and
/// a file that cannot be read as that shape is ignored and an
/// empty object stands in its place. A file nested past the
/// depth the reading accepts is the one failure that does not
/// fall back, because the Python reader lets its matching
/// failure escape instead of catching it, so it returns here
/// as its own outcome and run_attempt surfaces it as an error.
enum ResultRead {
    Value(JsonValue),
    TooDeep,
}

fn read_result_file(directory: &Path) -> ResultRead {
    let empty = JsonValue::Obj(Vec::new());
    let text = match crate::races::read_text_file(&directory.join("result.json")) {
        None => return ResultRead::Value(empty),
        Some(text) => text,
    };
    match json::parse(&text) {
        Ok(value) if value.as_object().is_some() => ResultRead::Value(value),
        Ok(_) => ResultRead::Value(empty),
        Err(json::ParseError::TooDeep) => ResultRead::TooDeep,
        Err(json::ParseError::Malformed) => ResultRead::Value(empty),
    }
}

/// The attempt record of a run that produced no result, an
/// unfinished attempt holding the patch the lane holds. A
/// command that cannot be started and a command killed at its
/// timeout both land here, through this one construction.
fn unfinished_attempt(model: &PText, environment: &PText, directory: &Path) -> Attempt {
    Attempt {
        model: model.clone(),
        environment: environment.clone(),
        result: SuiteResult::Unfinished,
        tokens: None,
        patch: read_patch(directory),
        finished_at: None,
        adjustment: None,
    }
}

/// Return the patch file text the command wrote, or empty text.
fn read_patch(directory: &Path) -> PText {
    match crate::races::read_text_file(&directory.join("patch.diff")) {
        None => PText::from_text(""),
        Some(text) => PText::from_text(&text),
    }
}

/// One attempt specification for run_race, the model, the
/// environment, the shell command, and an optional timeout in
/// seconds.
pub struct AttemptSpec {
    pub model: PText,
    pub environment: PText,
    pub command: String,
    pub timeout_seconds: Option<f64>,
}

/// Run one attempt command and return its attempt record.
///
/// The command runs in the directory the lane names under the
/// workspace. The lane is emptied before the command starts, so
/// the record can only be built from files this command writes.
/// The record follows the DH-1 runner text in CONTRACT.md. A
/// result file nested past the depth the reading accepts is
/// the one failure of the run, returned as an error, mirroring
/// the failure the Python runner lets escape from its own
/// run_attempt.
pub fn run_attempt(
    workspace: &Path,
    lane: &str,
    model: &PText,
    environment: &PText,
    command: &str,
    timeout_seconds: Option<f64>,
) -> Result<Attempt, String> {
    let directory: PathBuf = workspace.join(lane);
    empty_directory(&directory);
    let started = Instant::now();
    let mut child = match Command::new("/bin/sh")
        .arg("-c")
        .arg(command)
        .current_dir(&directory)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(child) => child,
        Err(_) => {
            // A command that cannot be started at all stands as
            // an unfinished attempt with the patch the lane
            // holds, the closest reading of a failed launch.
            return Ok(unfinished_attempt(model, environment, &directory));
        }
    };
    let status = match timeout_seconds {
        None => child.wait().ok(),
        Some(timeout) => {
            let limit = Duration::from_secs_f64(timeout.max(0.0));
            loop {
                match child.try_wait() {
                    Ok(Some(status)) => break Some(status),
                    Ok(None) => {
                        if started.elapsed() >= limit {
                            let _ = child.kill();
                            let _ = child.wait();
                            return Ok(unfinished_attempt(model, environment, &directory));
                        }
                        std::thread::sleep(Duration::from_millis(2));
                    }
                    Err(_) => break None,
                }
            }
        }
    };
    let elapsed = started.elapsed().as_secs_f64();
    let named = match read_result_file(&directory) {
        ResultRead::Value(value) => value,
        ResultRead::TooDeep => {
            return Err("the result file nests past the depth the reading accepts".to_string())
        }
    };
    let named_result = named
        .get("result")
        .and_then(|value| value.as_text().cloned());
    let result = match named_result {
        Some(text) if text == PText::from_text("green") => SuiteResult::Green,
        Some(text) if text == PText::from_text("red") => SuiteResult::Red,
        Some(text) if text == PText::from_text("unfinished") => SuiteResult::Unfinished,
        _ => {
            if status.map(|status| status.success()).unwrap_or(false) {
                SuiteResult::Green
            } else {
                SuiteResult::Red
            }
        }
    };
    let tokens = match named.get("tokens") {
        Some(value) => match crate::races::as_int(value) {
            Some(digits) if *digits >= BigInt::from(0) => Some(digits.clone()),
            _ => None,
        },
        None => None,
    };
    Ok(Attempt {
        model: model.clone(),
        environment: environment.clone(),
        result,
        tokens,
        patch: read_patch(&directory),
        finished_at: if result != SuiteResult::Unfinished {
            Some(crate::pynum::Num::from_f64(elapsed))
        } else {
            None
        },
        adjustment: None,
    })
}

/// Run the two attempts of one race and return the race record.
/// The lanes are named attempt-1 and attempt-2 under the
/// workspace, in record order. An error from either attempt is
/// the error of the run, as the Python exception escapes its
/// run through both.
pub fn run_race(workspace: &Path, task: &PText, specs: &[AttemptSpec]) -> Result<Race, String> {
    let mut attempts = Vec::new();
    for (index, spec) in specs.iter().enumerate() {
        attempts.push(run_attempt(
            workspace,
            &format!("attempt-{}", index + 1),
            &spec.model,
            &spec.environment,
            &spec.command,
            spec.timeout_seconds,
        )?);
    }
    Ok(Race {
        task: task.clone(),
        attempts,
        adjudication: None,
        panel: None,
    })
}
