//! Helpers shared by the translated test files, building the
//! JSON values the Python test dicts build and driving the
//! command line the way the Python main is driven.

#![allow(dead_code)]

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use deadheat::json::JsonValue;
use deadheat::pynum::Num;
use deadheat::pytext::PText;

pub fn s(text: &str) -> JsonValue {
    JsonValue::Str(PText::from_text(text))
}

pub fn i(value: i64) -> JsonValue {
    JsonValue::Num(Num::from_i64(value))
}

pub fn f(value: f64) -> JsonValue {
    JsonValue::Num(Num::from_f64(value))
}

pub fn bigint(value: &str) -> JsonValue {
    JsonValue::Num(Num::Int(
        num_bigint::BigInt::parse_bytes(value.as_bytes(), 10).expect("digits"),
    ))
}

pub fn obj(pairs: Vec<(&str, JsonValue)>) -> JsonValue {
    JsonValue::object(
        pairs
            .into_iter()
            .map(|(key, value)| (PText::from_text(key), value))
            .collect(),
    )
}

pub fn arr(items: Vec<JsonValue>) -> JsonValue {
    JsonValue::Arr(items)
}

/// Return a copy of an object value with one key set, holding
/// the position the key already holds when it stands there.
pub fn set(value: &JsonValue, key: &str, replacement: JsonValue) -> JsonValue {
    let pairs = value.as_object().expect("an object").clone();
    let mut out: Vec<(PText, JsonValue)> = Vec::new();
    let mut placed = false;
    for (held, item) in pairs {
        if held == PText::from_text(key) {
            out.push((held, replacement.clone()));
            placed = true;
        } else {
            out.push((held, item));
        }
    }
    if !placed {
        out.push((PText::from_text(key), replacement));
    }
    JsonValue::Obj(out)
}

/// Return a copy of an object value with the named keys removed.
pub fn without(value: &JsonValue, keys: &[&str]) -> JsonValue {
    let pairs = value.as_object().expect("an object").clone();
    JsonValue::Obj(
        pairs
            .into_iter()
            .filter(|(held, _)| !keys.iter().any(|key| *held == PText::from_text(key)))
            .collect(),
    )
}

/// Read one path step, an object key or an array index held
/// as its decimal text.
fn get_step<'a>(value: &'a JsonValue, step: &str) -> Option<&'a JsonValue> {
    match value {
        JsonValue::Arr(items) => step
            .parse::<usize>()
            .ok()
            .and_then(|index| items.get(index)),
        _ => value.get(step),
    }
}

/// Write one path step, an object key or an array index held
/// as its decimal text, returning the rebuilt value.
fn set_step(value: &JsonValue, step: &str, replacement: JsonValue) -> JsonValue {
    match value {
        JsonValue::Arr(items) => {
            let index: usize = step.parse().expect("an array index");
            let mut out = items.clone();
            out[index] = replacement;
            JsonValue::Arr(out)
        }
        _ => set(value, step, replacement),
    }
}

/// Set a value at a path of object keys and array indices,
/// the way the Python tests reach into a nested envelope.
pub fn set_path(value: &JsonValue, path: &[&str], replacement: JsonValue) -> JsonValue {
    if path.len() == 1 {
        return set_step(value, path[0], replacement);
    }
    let inner = get_step(value, path[0]).expect("a path step").clone();
    set_step(value, path[0], set_path(&inner, &path[1..], replacement))
}

/// Remove a key at a path of object keys and array indices.
pub fn remove_path(value: &JsonValue, path: &[&str]) -> JsonValue {
    if path.len() == 1 {
        return without(value, &[path[0]]);
    }
    let inner = get_step(value, path[0]).expect("a path step").clone();
    set_step(value, path[0], remove_path(&inner, &path[1..]))
}

pub fn get_path<'a>(value: &'a JsonValue, path: &[&str]) -> Option<&'a JsonValue> {
    let mut current = value;
    for step in path {
        current = get_step(current, step)?;
    }
    Some(current)
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// A fresh empty directory for one test, in the shape of the
/// Python tmp_path fixture.
pub fn temp_dir(name: &str) -> PathBuf {
    let serial = COUNTER.fetch_add(1, Ordering::SeqCst);
    let directory = std::env::temp_dir().join(format!(
        "deadheat-rust-{}-{}-{}",
        name,
        std::process::id(),
        serial
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("a temp directory");
    directory
}

pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn examples() -> PathBuf {
    repo_root().join("examples")
}

/// Run the command line library function at eighty columns and
/// return its stdout text, stderr text, and exit status.
pub fn run_cli(args: &[&str]) -> (String, String, i32) {
    run_cli_columns(args, 80)
}

pub fn run_cli_columns(args: &[&str], columns: usize) -> (String, String, i32) {
    let owned: Vec<PText> = args.iter().map(|arg| PText::from_text(arg)).collect();
    let outcome = deadheat::cli::run(&owned, columns);
    (
        String::from_utf8_lossy(&outcome.stdout).into_owned(),
        String::from_utf8_lossy(&outcome.stderr).into_owned(),
        outcome.code,
    )
}

pub fn lines_of(text: &str) -> Vec<String> {
    text.lines().map(|line| line.to_string()).collect()
}

/// The counts of one closing line, the integer at the head
/// of each comma separated field after the first.
pub fn closing_counts(line: &str) -> Vec<u64> {
    line.split(", ")
        .skip(1)
        .map(|field| {
            field
                .split(' ')
                .next()
                .expect("a field")
                .parse::<u64>()
                .expect("a count")
        })
        .collect()
}
