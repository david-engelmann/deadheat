//! Pins for the race specification and the run command of
//! DH-27 in CONTRACT.md.
//!
//! The duel pins run the real binary over the shipped
//! specification in a scratch working directory and check what
//! can be checked of a measured run. No pin fixes a measured
//! figure, because a finish the runner measures is not a
//! figure a pin can state in advance. The defect pins name
//! every phrase of the specification loader's vocabulary on
//! its own malformed specification, and each one fails
//! against a loader that leaves the field unread.

mod shared;

use std::path::Path;
use std::process::Command;

use deadheat::json::{self, JsonValue};
use deadheat::races::{load_race_file, SuiteResult};
use deadheat::specs::{load_spec_file, load_spec_text};
use shared::{arr, bigint, f, i, obj, s, set, temp_dir, without};

fn attempt_value(model: &str) -> JsonValue {
    obj(vec![
        ("model", s(model)),
        ("environment", s("local shell")),
        ("command", s("exit 0")),
    ])
}

fn spec_value() -> JsonValue {
    obj(vec![
        ("task", s("the example duel")),
        ("workspace", s("duel-workspace")),
        ("record", s("duel-record.json")),
        (
            "attempts",
            arr(vec![attempt_value("duel-1"), attempt_value("duel-2")]),
        ),
    ])
}

fn defects_of(value: &JsonValue) -> Vec<String> {
    let (spec, defects) = load_spec_text(&json::dumps(value, None, true));
    assert!(spec.is_none(), "a malformed specification loads no spec");
    defects
}

fn phrases(list: &[&str]) -> Vec<String> {
    list.iter().map(|line| line.to_string()).collect()
}

fn run_binary(dir: &Path, args: &[&str]) -> (String, i32) {
    let output = Command::new(env!("CARGO_BIN_EXE_deadheat"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("the binary runs");
    (
        String::from_utf8_lossy(&output.stdout).to_string(),
        output.status.code().unwrap_or(-1),
    )
}

#[test]
fn the_shipped_duel_specification_runs_and_its_record_reads_back() {
    let scratch = temp_dir("runs-duel");
    std::fs::create_dir_all(&scratch).unwrap();
    let spec_path = shared::examples().join("duel-spec.json");
    let (stdout, code) = run_binary(&scratch, &["run", spec_path.to_str().expect("a path")]);
    assert_eq!(code, 0, "the duel runs clean: {stdout}");
    let record_path = scratch.join("duel-record.json");
    assert!(record_path.is_file(), "the record stands where named");
    assert!(scratch.join("duel-workspace").is_dir());
    let (race, defects) = load_race_file(&record_path);
    assert_eq!(defects, Vec::<String>::new());
    let race = race.expect("a clean record");
    assert_eq!(race.attempts[0].result, SuiteResult::Green);
    assert_eq!(race.attempts[1].result, SuiteResult::Red);
    for attempt in &race.attempts {
        let finish = attempt.finished_at.as_ref().expect("a finish");
        assert!(finish.is_finite_number(), "each finish is finite");
        assert!(!finish.is_negative(), "each finish stands at or above zero");
        assert_eq!(attempt.tokens, None, "no result file, no token count");
    }
    assert_eq!(race.attempts[0].patch.display(), "attempt one patch line\n");
    assert_eq!(race.attempts[1].patch.display(), "attempt two patch line\n");
    let (race_stdout, race_code) =
        run_binary(&scratch, &["race", record_path.to_str().expect("a path")]);
    assert_eq!(race_code, 0);
    assert_eq!(
        race_stdout, stdout,
        "the race command over the written record prints the run output byte for byte"
    );
}

#[test]
fn a_clean_specification_loads_with_its_timeout() {
    let attempt = set(&attempt_value("duel-1"), "timeout_seconds", f(2.5));
    let value = set(
        &spec_value(),
        "attempts",
        arr(vec![attempt, attempt_value("duel-2")]),
    );
    let (spec, defects) = load_spec_text(&json::dumps(&value, None, true));
    assert_eq!(defects, Vec::<String>::new());
    let spec = spec.expect("a clean specification");
    assert_eq!(spec.attempts.len(), 2);
    assert_eq!(spec.attempts[0].timeout_seconds, Some(2.5));
    assert_eq!(spec.attempts[1].timeout_seconds, None);
}

#[test]
fn a_specification_at_version_2_draws_the_unsupported_version_defect() {
    assert_eq!(
        defects_of(&set(&spec_value(), "version", i(2))),
        phrases(&["field version names an unsupported version"])
    );
}

#[test]
fn a_specification_with_a_text_version_draws_the_wrong_type_defect() {
    assert_eq!(
        defects_of(&set(&spec_value(), "version", s("1"))),
        phrases(&["field version has the wrong type"])
    );
}

#[test]
fn a_missing_task_draws_its_defect() {
    assert_eq!(
        defects_of(&without(&spec_value(), &["task"])),
        phrases(&["missing field task"])
    );
}

#[test]
fn a_task_of_the_wrong_type_draws_its_defect() {
    assert_eq!(
        defects_of(&set(&spec_value(), "task", i(3))),
        phrases(&["field task has the wrong type"])
    );
}

#[test]
fn an_empty_task_draws_its_defect() {
    assert_eq!(
        defects_of(&set(&spec_value(), "task", s(""))),
        phrases(&["field task is empty"])
    );
}

#[test]
fn a_task_holding_a_newline_draws_its_defect() {
    assert_eq!(
        defects_of(&set(&spec_value(), "task", s("two\nlines"))),
        phrases(&["field task holds a newline"])
    );
}

#[test]
fn a_task_holding_an_unpaired_surrogate_draws_its_defect() {
    let text = "{\"task\": \"\\ud800\", \"workspace\": \"w\", \"record\": \"r.json\", \"attempts\": [{\"model\": \"a\", \"environment\": \"e\", \"command\": \"exit 0\"}, {\"model\": \"b\", \"environment\": \"e\", \"command\": \"exit 0\"}]}";
    let (spec, defects) = load_spec_text(text);
    assert!(spec.is_none());
    assert_eq!(
        defects,
        phrases(&["field task holds an unpaired surrogate"])
    );
}

#[test]
fn a_missing_workspace_draws_its_defect() {
    assert_eq!(
        defects_of(&without(&spec_value(), &["workspace"])),
        phrases(&["missing field workspace"])
    );
}

#[test]
fn a_workspace_of_the_wrong_type_draws_its_defect() {
    assert_eq!(
        defects_of(&set(&spec_value(), "workspace", i(3))),
        phrases(&["field workspace has the wrong type"])
    );
}

#[test]
fn an_empty_workspace_draws_its_defect() {
    assert_eq!(
        defects_of(&set(&spec_value(), "workspace", s(""))),
        phrases(&["field workspace is empty"])
    );
}

#[test]
fn a_missing_record_draws_its_defect() {
    assert_eq!(
        defects_of(&without(&spec_value(), &["record"])),
        phrases(&["missing field record"])
    );
}

#[test]
fn an_empty_record_draws_its_defect() {
    assert_eq!(
        defects_of(&set(&spec_value(), "record", s(""))),
        phrases(&["field record is empty"])
    );
}

#[test]
fn a_missing_attempts_field_draws_its_defect() {
    assert_eq!(
        defects_of(&without(&spec_value(), &["attempts"])),
        phrases(&["missing field attempts"])
    );
}

#[test]
fn an_attempts_field_of_the_wrong_type_draws_its_defect() {
    assert_eq!(
        defects_of(&set(&spec_value(), "attempts", s("two"))),
        phrases(&["field attempts has the wrong type"])
    );
}

#[test]
fn one_attempt_draws_the_exactly_two_defect() {
    let value = set(
        &spec_value(),
        "attempts",
        arr(vec![attempt_value("duel-1")]),
    );
    assert_eq!(
        defects_of(&value),
        phrases(&["field attempts must hold exactly two attempts"])
    );
}

#[test]
fn three_attempts_draw_the_exactly_two_defect() {
    let value = set(
        &spec_value(),
        "attempts",
        arr(vec![
            attempt_value("duel-1"),
            attempt_value("duel-2"),
            attempt_value("duel-3"),
        ]),
    );
    assert_eq!(
        defects_of(&value),
        phrases(&["field attempts must hold exactly two attempts"])
    );
}

#[test]
fn an_attempt_that_is_not_an_object_draws_its_defect() {
    let value = set(
        &spec_value(),
        "attempts",
        arr(vec![i(1), attempt_value("duel-2")]),
    );
    assert_eq!(defects_of(&value), phrases(&["attempt 0 is not an object"]));
}

#[test]
fn an_attempt_missing_its_model_draws_its_defect() {
    let attempt = without(&attempt_value("duel-1"), &["model"]);
    let value = set(
        &spec_value(),
        "attempts",
        arr(vec![attempt, attempt_value("duel-2")]),
    );
    assert_eq!(
        defects_of(&value),
        phrases(&["attempt 0 is missing field model"])
    );
}

#[test]
fn an_attempt_model_of_the_wrong_type_draws_its_defect() {
    let attempt = set(&attempt_value("duel-1"), "model", i(3));
    let value = set(
        &spec_value(),
        "attempts",
        arr(vec![attempt, attempt_value("duel-2")]),
    );
    assert_eq!(
        defects_of(&value),
        phrases(&["attempt 0 field model has the wrong type"])
    );
}

#[test]
fn an_empty_attempt_model_draws_its_defect() {
    let attempt = set(&attempt_value("duel-1"), "model", s(""));
    let value = set(
        &spec_value(),
        "attempts",
        arr(vec![attempt, attempt_value("duel-2")]),
    );
    assert_eq!(
        defects_of(&value),
        phrases(&["attempt 0 field model is empty"])
    );
}

#[test]
fn an_attempt_model_holding_a_newline_draws_its_defect() {
    let attempt = set(&attempt_value("duel-1"), "model", s("two\nlines"));
    let value = set(
        &spec_value(),
        "attempts",
        arr(vec![attempt, attempt_value("duel-2")]),
    );
    assert_eq!(
        defects_of(&value),
        phrases(&["attempt 0 field model holds a newline"])
    );
}

#[test]
fn an_attempt_environment_holding_an_unpaired_surrogate_draws_its_defect() {
    let text = "{\"task\": \"t\", \"workspace\": \"w\", \"record\": \"r.json\", \"attempts\": [{\"model\": \"a\", \"environment\": \"\\ud800\", \"command\": \"exit 0\"}, {\"model\": \"b\", \"environment\": \"e\", \"command\": \"exit 0\"}]}";
    let (spec, defects) = load_spec_text(text);
    assert!(spec.is_none());
    assert_eq!(
        defects,
        phrases(&["attempt 0 field environment holds an unpaired surrogate"])
    );
}

// The seven pins of FIX-DH-20 in the Round 13 section of
// CONTRACT.md. Every one of the seven passes on the pre
// round tree by design, stated in so many words, the
// loader's behaviour already standing as the pins hold it
// through the shared helpers, and the Round 12 finding
// they close was a coverage finding, not a behaviour
// finding. Their discrimination is against the stated
// mutation of the routing each pins, named in each pin's
// own comment, and no claim stands here or in the header
// above that any of them fails pre round.

#[test]
fn a_specification_text_that_is_not_json_draws_the_preamble_defect() {
    // Fails under the mutation renaming or dropping the
    // loader preamble's not valid JSON defect.
    let (spec, defects) = load_spec_text("this is not json");
    assert!(spec.is_none());
    assert_eq!(defects, phrases(&["input is not valid JSON"]));
}

#[test]
fn a_specification_holding_a_list_draws_the_not_an_object_defect() {
    // Fails under the mutation reading a non object
    // specification as an empty object, or misphrasing
    // the defect.
    let (spec, defects) = load_spec_text("[1, 2]");
    assert!(spec.is_none());
    assert_eq!(defects, phrases(&["input is not a JSON object"]));
}

#[test]
fn a_specification_file_that_cannot_be_read_draws_its_defect() {
    // Fails under the mutation reading an unreadable file
    // as an empty specification.
    let dir = temp_dir("runs-unreadable");
    let (spec, defects) = load_spec_file(&dir.join("no-such-spec.json"));
    assert!(spec.is_none());
    assert_eq!(defects, phrases(&["file cannot be read"]));
}

#[test]
fn a_record_of_the_wrong_type_draws_its_defect() {
    // Fails under the mutation reading the record field
    // without the shared wrong type check, a numeric
    // record standing accepted.
    assert_eq!(
        defects_of(&set(&spec_value(), "record", i(3))),
        phrases(&["field record has the wrong type"])
    );
}

#[test]
fn an_attempt_environment_of_the_wrong_type_draws_its_defect() {
    // Fails under the mutation applying the shared attempt
    // text check to the model field alone among the
    // attempt's label fields.
    let attempt = set(&attempt_value("duel-1"), "environment", i(3));
    let value = set(
        &spec_value(),
        "attempts",
        arr(vec![attempt, attempt_value("duel-2")]),
    );
    assert_eq!(
        defects_of(&value),
        phrases(&["attempt 0 field environment has the wrong type"])
    );
}

#[test]
fn an_empty_attempt_environment_draws_its_defect() {
    // Fails under the same routing mutation in its empty
    // form, the check applied to model and not to
    // environment.
    let attempt = set(&attempt_value("duel-1"), "environment", s(""));
    let value = set(
        &spec_value(),
        "attempts",
        arr(vec![attempt, attempt_value("duel-2")]),
    );
    assert_eq!(
        defects_of(&value),
        phrases(&["attempt 0 field environment is empty"])
    );
}

#[test]
fn an_attempt_environment_holding_a_newline_draws_its_defect() {
    // Fails under the same routing mutation in its newline
    // form.
    let attempt = set(&attempt_value("duel-1"), "environment", s("two\nlines"));
    let value = set(
        &spec_value(),
        "attempts",
        arr(vec![attempt, attempt_value("duel-2")]),
    );
    assert_eq!(
        defects_of(&value),
        phrases(&["attempt 0 field environment holds a newline"])
    );
}

#[test]
fn an_attempt_missing_its_command_draws_its_defect() {
    let attempt = without(&attempt_value("duel-2"), &["command"]);
    let value = set(
        &spec_value(),
        "attempts",
        arr(vec![attempt_value("duel-1"), attempt]),
    );
    assert_eq!(
        defects_of(&value),
        phrases(&["attempt 1 is missing field command"])
    );
}

#[test]
fn an_attempt_command_of_the_wrong_type_draws_its_defect() {
    let attempt = set(&attempt_value("duel-2"), "command", i(3));
    let value = set(
        &spec_value(),
        "attempts",
        arr(vec![attempt_value("duel-1"), attempt]),
    );
    assert_eq!(
        defects_of(&value),
        phrases(&["attempt 1 field command has the wrong type"])
    );
}

#[test]
fn an_empty_attempt_command_draws_its_defect() {
    let attempt = set(&attempt_value("duel-2"), "command", s(""));
    let value = set(
        &spec_value(),
        "attempts",
        arr(vec![attempt_value("duel-1"), attempt]),
    );
    assert_eq!(
        defects_of(&value),
        phrases(&["attempt 1 field command is empty"])
    );
}

#[test]
fn a_timeout_of_the_wrong_type_draws_its_defect() {
    let attempt = set(&attempt_value("duel-1"), "timeout_seconds", s("5"));
    let value = set(
        &spec_value(),
        "attempts",
        arr(vec![attempt, attempt_value("duel-2")]),
    );
    assert_eq!(
        defects_of(&value),
        phrases(&["attempt 0 field timeout_seconds has the wrong type"])
    );
}

#[test]
fn a_timeout_that_is_not_a_finite_number_draws_its_defect() {
    let attempt = set(
        &attempt_value("duel-1"),
        "timeout_seconds",
        bigint(&"9".repeat(400)),
    );
    let value = set(
        &spec_value(),
        "attempts",
        arr(vec![attempt, attempt_value("duel-2")]),
    );
    assert_eq!(
        defects_of(&value),
        phrases(&["attempt 0 field timeout_seconds is not a finite number"])
    );
}

#[test]
fn a_timeout_below_zero_draws_its_defect() {
    let attempt = set(&attempt_value("duel-1"), "timeout_seconds", f(-0.5));
    let value = set(
        &spec_value(),
        "attempts",
        arr(vec![attempt, attempt_value("duel-2")]),
    );
    assert_eq!(
        defects_of(&value),
        phrases(&["attempt 0 field timeout_seconds is below zero"])
    );
}

#[test]
fn one_pass_names_defects_in_the_task_and_the_attempts_together() {
    let attempt = without(&attempt_value("duel-2"), &["command"]);
    let mut value = set(&spec_value(), "task", s(""));
    value = set(
        &value,
        "attempts",
        arr(vec![attempt_value("duel-1"), attempt]),
    );
    assert_eq!(
        defects_of(&value),
        phrases(&["field task is empty", "attempt 1 is missing field command"])
    );
}

#[test]
fn a_malformed_specification_runs_nothing_and_writes_nothing() {
    let scratch = temp_dir("runs-defects");
    std::fs::create_dir_all(&scratch).unwrap();
    let mut value = set(&spec_value(), "workspace", s("never-created"));
    value = set(&value, "attempts", arr(vec![attempt_value("duel-1")]));
    let spec_path = scratch.join("bad-spec.json");
    std::fs::write(&spec_path, json::dumps(&value, None, true)).unwrap();
    let (stdout, code) = run_binary(&scratch, &["run", spec_path.to_str().expect("a path")]);
    assert_eq!(code, 1);
    assert_eq!(
        stdout,
        "invalid race specification file\ndefect field attempts must hold exactly two attempts\n"
    );
    assert!(
        !scratch.join("never-created").exists(),
        "the refused run creates no workspace"
    );
    assert!(
        !scratch.join("duel-record.json").exists(),
        "the refused run writes no record"
    );
}

#[test]
fn a_result_file_nested_past_the_depth_cap_escapes_as_the_only_line() {
    let scratch = temp_dir("runs-deep");
    std::fs::create_dir_all(&scratch).unwrap();
    let writer = scratch.join("write_result.py");
    std::fs::write(
        &writer,
        "import sys\ndepth = int(sys.argv[1])\ntext = '{\"result\": \"red\", \"tokens\": 5, \"x\": ' + '[' * depth + ']' * depth + '}'\nopen('result.json', 'w').write(text)\n",
    )
    .expect("the writer");
    let deep = set(
        &attempt_value("duel-1"),
        "command",
        s(&format!("python3 {} 10000", writer.display())),
    );
    let mut value = set(&spec_value(), "workspace", s("deep-workspace"));
    value = set(&value, "attempts", arr(vec![deep, attempt_value("duel-2")]));
    let spec_path = scratch.join("deep-spec.json");
    std::fs::write(&spec_path, json::dumps(&value, None, true)).unwrap();
    let (stdout, code) = run_binary(&scratch, &["run", spec_path.to_str().expect("a path")]);
    assert_eq!(code, 1);
    assert_eq!(
        stdout,
        "the result file nests past the depth the reading accepts\n"
    );
    assert!(
        !scratch.join("duel-record.json").exists(),
        "the escaped run writes no record"
    );
}

#[test]
fn a_record_that_cannot_be_written_prints_nothing_and_exits_one() {
    let scratch = temp_dir("runs-unwritable");
    std::fs::create_dir_all(&scratch).unwrap();
    // A directory standing where the record file must land
    // makes the write fail after both attempts have run.
    std::fs::create_dir_all(scratch.join("duel-record.json")).unwrap();
    let spec_path = scratch.join("spec.json");
    std::fs::write(&spec_path, json::dumps(&spec_value(), None, true)).unwrap();
    let (stdout, code) = run_binary(&scratch, &["run", spec_path.to_str().expect("a path")]);
    assert_eq!(code, 1);
    assert_eq!(stdout, "", "the failed write prints nothing");
}

#[test]
fn an_attempt_outliving_its_timeout_stands_unfinished_in_the_record() {
    let scratch = temp_dir("runs-timeout");
    std::fs::create_dir_all(&scratch).unwrap();
    let slow = obj(vec![
        ("model", s("duel-1")),
        ("environment", s("local shell")),
        ("command", s("sleep 2")),
        ("timeout_seconds", f(0.05)),
    ]);
    let value = set(
        &spec_value(),
        "attempts",
        arr(vec![slow, attempt_value("duel-2")]),
    );
    let spec_path = scratch.join("spec.json");
    std::fs::write(&spec_path, json::dumps(&value, None, true)).unwrap();
    let (stdout, code) = run_binary(&scratch, &["run", spec_path.to_str().expect("a path")]);
    assert_eq!(code, 0, "the run completes: {stdout}");
    let (race, defects) = load_race_file(&scratch.join("duel-record.json"));
    assert_eq!(defects, Vec::<String>::new());
    let race = race.expect("a clean record");
    assert_eq!(race.attempts[0].result, SuiteResult::Unfinished);
    assert!(race.attempts[0].finished_at.is_none());
    assert_eq!(race.attempts[1].result, SuiteResult::Green);
}
