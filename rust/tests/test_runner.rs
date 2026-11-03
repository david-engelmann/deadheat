//! Pins for the local runner in the DH-1 runner text of
//! CONTRACT.md, translated from the Python corpus.
//!
//! Every command here is trivial and local, a shell exit, a
//! small file write, or a short sleep against a shorter
//! timeout. Nothing touches the network and nothing heavy
//! runs.
//!
//! One mechanical consequence of the port repair stands in
//! this file. run_attempt and run_race now return a Result,
//! because a result file nested past the depth the reading
//! accepts surfaces as an error rather than a fallback record,
//! so every standing call site unwraps its attempt with an
//! expect where it once took the record directly. No standing
//! assertion changed.

mod shared;

use deadheat::pytext::PText;
use deadheat::races::{load_race_text, race_to_json, SuiteResult};
use deadheat::runner::{run_attempt, run_race, AttemptSpec};
use shared::*;

fn text(value: &str) -> PText {
    PText::from_text(value)
}

#[test]
fn a_passing_command_records_green_with_a_finish() {
    let dir = temp_dir("runner-green");
    let attempt = run_attempt(
        &dir,
        "lane",
        &text("model-a"),
        &text("local shell"),
        "exit 0",
        None,
    )
    .expect("the attempt runs");
    assert_eq!(attempt.result, SuiteResult::Green);
    assert!(!attempt.finished_at.expect("a finish").is_negative());
    assert_eq!(attempt.tokens, None);
    assert_eq!(attempt.patch.display(), "");
    assert!(dir.join("lane").is_dir());
}

#[test]
fn a_failing_command_records_red_with_a_finish() {
    let dir = temp_dir("runner-red");
    let attempt = run_attempt(
        &dir,
        "lane",
        &text("model-a"),
        &text("local shell"),
        "exit 3",
        None,
    )
    .expect("the attempt runs");
    assert_eq!(attempt.result, SuiteResult::Red);
    assert!(attempt.finished_at.is_some());
}

#[test]
fn a_named_result_wins_over_the_exit_code() {
    let dir = temp_dir("runner-named");
    let attempt = run_attempt(
        &dir,
        "lane",
        &text("model-a"),
        &text("local shell"),
        "printf '%s' '{\"result\": \"red\"}' > result.json",
        None,
    )
    .expect("the attempt runs");
    assert_eq!(attempt.result, SuiteResult::Red);
}

#[test]
fn a_named_unfinished_result_carries_no_finish() {
    let dir = temp_dir("runner-named-unfinished");
    let attempt = run_attempt(
        &dir,
        "lane",
        &text("model-a"),
        &text("local shell"),
        "printf '%s' '{\"result\": \"unfinished\"}' > result.json; exit 0",
        None,
    )
    .expect("the attempt runs");
    assert_eq!(attempt.result, SuiteResult::Unfinished);
    assert!(attempt.finished_at.is_none());
}

#[test]
fn reported_tokens_are_recorded() {
    let dir = temp_dir("runner-tokens");
    let attempt = run_attempt(
        &dir,
        "lane",
        &text("model-a"),
        &text("local shell"),
        "printf '%s' '{\"tokens\": 321}' > result.json",
        None,
    )
    .expect("the attempt runs");
    assert_eq!(attempt.result, SuiteResult::Green);
    assert_eq!(attempt.tokens, Some(num_bigint::BigInt::from(321)));
}

#[test]
fn a_negative_token_count_in_the_result_file_is_not_recorded() {
    let dir = temp_dir("runner-negative-tokens");
    let attempt = run_attempt(
        &dir,
        "lane",
        &text("model-a"),
        &text("local shell"),
        "printf '%s' '{\"tokens\": -4}' > result.json",
        None,
    )
    .expect("the attempt runs");
    assert_eq!(attempt.tokens, None);
}

#[test]
fn a_malformed_result_file_falls_back_to_the_exit_code() {
    let dir = temp_dir("runner-malformed");
    let attempt = run_attempt(
        &dir,
        "lane",
        &text("model-a"),
        &text("local shell"),
        "printf '%s' 'not json at all' > result.json; exit 3",
        None,
    )
    .expect("the attempt runs");
    assert_eq!(attempt.result, SuiteResult::Red);
    assert_eq!(attempt.tokens, None);
}

#[test]
fn a_result_file_nested_past_the_depth_cap_is_an_error_not_a_fallback() {
    // The repair pin for the runner depth finding. The writer
    // leaves a result file whose ignored field nests arrays
    // 10000 deep inside the root object, past the depth the
    // reading accepts. The Python runner lets its recursion
    // failure escape on this file, so the port surfaces an
    // error where the pre repair code quietly fell back to
    // the exit code and recorded a green attempt.
    let dir = temp_dir("runner-deep");
    let writer = dir.join("write_result.py");
    std::fs::write(
        &writer,
        "import sys\ndepth = int(sys.argv[1])\ntext = '{\"result\": \"red\", \"tokens\": 5, \"x\": ' + '[' * depth + ']' * depth + '}'\nopen('result.json', 'w').write(text)\n",
    )
    .expect("the writer");
    let command = format!("python3 {} 10000", writer.display());
    let outcome = run_attempt(
        &dir,
        "lane",
        &text("model-a"),
        &text("local shell"),
        &command,
        None,
    );
    assert!(outcome.is_err(), "a deep result file is an error");
}

#[test]
fn a_result_file_nested_inside_the_depth_cap_still_names_its_result() {
    // The other side of the depth boundary. At 2000 nested
    // arrays the Python runner reads the file and honors the
    // named result, and the port agrees.
    let dir = temp_dir("runner-deep-inside");
    let writer = dir.join("write_result.py");
    std::fs::write(
        &writer,
        "import sys\ndepth = int(sys.argv[1])\ntext = '{\"result\": \"red\", \"tokens\": 5, \"x\": ' + '[' * depth + ']' * depth + '}'\nopen('result.json', 'w').write(text)\n",
    )
    .expect("the writer");
    let command = format!("python3 {} 2000", writer.display());
    let attempt = run_attempt(
        &dir,
        "lane",
        &text("model-a"),
        &text("local shell"),
        &command,
        None,
    )
    .expect("the attempt runs");
    assert_eq!(attempt.result, SuiteResult::Red);
    assert_eq!(attempt.tokens, Some(num_bigint::BigInt::from(5)));
}

#[test]
fn a_patch_file_the_command_wrote_is_kept_verbatim() {
    let dir = temp_dir("runner-patch");
    let attempt = run_attempt(
        &dir,
        "lane",
        &text("model-a"),
        &text("local shell"),
        "printf '%s' 'fake diff body' > patch.diff",
        None,
    )
    .expect("the attempt runs");
    assert_eq!(attempt.patch.display(), "fake diff body");
}

#[test]
fn an_unreadable_result_word_falls_back_but_its_tokens_still_count() {
    let dir = temp_dir("runner-unreadable-word");
    let attempt = run_attempt(
        &dir,
        "lane",
        &text("model-a"),
        &text("local shell"),
        "printf '%s' '{\"result\": \"blue\", \"tokens\": 321}' > result.json",
        None,
    )
    .expect("the attempt runs");
    assert_eq!(attempt.result, SuiteResult::Green);
    assert_eq!(attempt.tokens, Some(num_bigint::BigInt::from(321)));
}

#[test]
fn a_second_attempt_in_the_same_lane_reads_only_its_own_files() {
    let dir = temp_dir("runner-same-lane");
    let first = run_attempt(
        &dir,
        "lane",
        &text("model-a"),
        &text("local shell"),
        "printf '%s' '{\"result\": \"green\", \"tokens\": 99}' > result.json; printf '%s' 'first patch' > patch.diff",
        None,
    )
    .expect("the attempt runs");
    assert_eq!(first.result, SuiteResult::Green);
    assert_eq!(first.tokens, Some(num_bigint::BigInt::from(99)));
    assert_eq!(first.patch.display(), "first patch");
    let second = run_attempt(
        &dir,
        "lane",
        &text("model-a"),
        &text("local shell"),
        "exit 1",
        None,
    )
    .expect("the attempt runs");
    assert_eq!(second.result, SuiteResult::Red);
    assert!(second.finished_at.is_some());
    assert_eq!(second.tokens, None);
    assert_eq!(second.patch.display(), "");
    assert!(!dir.join("lane").join("result.json").exists());
    assert!(!dir.join("lane").join("patch.diff").exists());
}

fn spec(model: &str, command: &str) -> AttemptSpec {
    AttemptSpec {
        model: text(model),
        environment: text("local shell"),
        command: command.to_string(),
        timeout_seconds: None,
    }
}

#[test]
fn a_second_race_on_one_workspace_reads_only_its_own_files() {
    let dir = temp_dir("runner-second-race");
    let first_specs = vec![
        spec(
            "model-a",
            "printf '%s' '{\"result\": \"green\", \"tokens\": 5}' > result.json; printf '%s' 'first patch' > patch.diff",
        ),
        spec("model-b", "exit 0"),
    ];
    let first_race = run_race(&dir, &text("fix the parser"), &first_specs).expect("the race runs");
    assert_eq!(
        first_race.attempts[0].tokens,
        Some(num_bigint::BigInt::from(5))
    );
    assert_eq!(first_race.attempts[0].patch.display(), "first patch");
    let second_specs = vec![spec("model-a", "exit 1"), spec("model-b", "exit 0")];
    let second_race =
        run_race(&dir, &text("fix the parser"), &second_specs).expect("the race runs");
    assert_eq!(second_race.attempts[0].result, SuiteResult::Red);
    assert_eq!(second_race.attempts[0].tokens, None);
    assert_eq!(second_race.attempts[0].patch.display(), "");
}

#[test]
fn a_command_past_its_timeout_is_unfinished() {
    let dir = temp_dir("runner-timeout");
    let attempt = run_attempt(
        &dir,
        "lane",
        &text("model-a"),
        &text("local shell"),
        "sleep 2",
        Some(0.2),
    )
    .expect("the attempt runs");
    assert_eq!(attempt.result, SuiteResult::Unfinished);
    assert!(attempt.finished_at.is_none());
    assert_eq!(attempt.tokens, None);
}

#[test]
fn each_attempt_runs_in_its_own_directory() {
    let dir = temp_dir("runner-directories");
    let specs = vec![
        spec("model-a", "printf a > mark.txt"),
        spec("model-b", "printf b > mark.txt; exit 1"),
    ];
    let race = run_race(&dir, &text("fix the parser"), &specs).expect("the race runs");
    assert_eq!(
        std::fs::read_to_string(dir.join("attempt-1").join("mark.txt")).expect("a mark"),
        "a"
    );
    assert_eq!(
        std::fs::read_to_string(dir.join("attempt-2").join("mark.txt")).expect("a mark"),
        "b"
    );
    assert_eq!(race.attempts[0].result, SuiteResult::Green);
    assert_eq!(race.attempts[1].result, SuiteResult::Red);
}

#[test]
fn a_run_race_record_loads_clean() {
    let dir = temp_dir("runner-loads");
    let specs = vec![spec("model-a", "exit 0"), spec("model-b", "exit 1")];
    let race = run_race(&dir, &text("fix the parser"), &specs).expect("the race runs");
    let (loaded, defects) =
        load_race_text(&deadheat::json::dumps(&race_to_json(&race), None, true));
    assert_eq!(defects, Vec::<String>::new());
    let loaded = loaded.expect("a race");
    assert_eq!(loaded.task.display(), "fix the parser");
    assert_eq!(loaded.attempts[0].result, SuiteResult::Green);
    assert_eq!(loaded.attempts[1].result, SuiteResult::Red);
}
