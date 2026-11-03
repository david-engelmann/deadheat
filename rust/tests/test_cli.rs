//! Pins for the command line surface in CONTRACT.md,
//! translated from the Python corpus.
//!
//! The commands are driven through the library entry the
//! binary shells, with real files, and their printed bytes
//! and exit statuses are pinned against hand written
//! expectations.

mod shared;

use deadheat::json::JsonValue;
use shared::*;

fn race_data() -> JsonValue {
    obj(vec![
        ("task", s("fix the parser")),
        (
            "attempts",
            arr(vec![
                obj(vec![
                    ("model", s("model-a")),
                    ("environment", s("lane one")),
                    ("result", s("green")),
                    ("finished_at", f(120.5)),
                    ("tokens", i(4500)),
                    ("patch", s("diff text")),
                ]),
                obj(vec![
                    ("model", s("model-b")),
                    ("environment", s("lane two")),
                    ("result", s("red")),
                    ("finished_at", f(90.25)),
                    ("tokens", JsonValue::Null),
                    ("patch", s("other diff")),
                ]),
            ]),
        ),
    ])
}

fn no_winner_race() -> JsonValue {
    obj(vec![
        ("task", s("fix the parser")),
        (
            "attempts",
            arr(vec![
                obj(vec![
                    ("model", s("model-a")),
                    ("environment", s("lane one")),
                    ("result", s("red")),
                    ("finished_at", f(10.0)),
                    ("tokens", i(100)),
                    ("patch", s("diff text")),
                ]),
                obj(vec![
                    ("model", s("model-b")),
                    ("environment", s("lane two")),
                    ("result", s("red")),
                    ("finished_at", f(20.0)),
                    ("tokens", i(200)),
                    ("patch", s("other diff")),
                ]),
            ]),
        ),
    ])
}

const EXPECTED_RACE_OUTPUT: &str = "race fix the parser\nmodel-a in lane one, green in 120.5 seconds, 4500 tokens\nmodel-b in lane two, red in 90.25 seconds, tokens unreported\nverdict, model-a wins\nfuneral for fix the parser\nwinner model-a in lane one went green in 120.5 seconds\nloser model-b in lane two went red in 90.25 seconds\nmargin 30.25 seconds between the two finishes\n";

fn write_json(dir: &std::path::Path, name: &str, data: &JsonValue) -> std::path::PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, deadheat::json::dumps(data, None, true)).expect("write");
    path
}

fn path_text(path: &std::path::Path) -> String {
    path.to_str().expect("a utf8 path").to_string()
}

#[test]
fn the_race_command_prints_the_report_and_the_funeral() {
    let dir = temp_dir("cli-race");
    let path = write_json(&dir, "race.json", &race_data());
    let (out, _, code) = run_cli(&["race", &path_text(&path)]);
    assert_eq!(code, 0);
    assert_eq!(out, EXPECTED_RACE_OUTPUT);
}

#[test]
fn the_race_command_is_deterministic() {
    let dir = temp_dir("cli-deterministic");
    let path = write_json(&dir, "race.json", &race_data());
    let (first, _, code) = run_cli(&["race", &path_text(&path)]);
    assert_eq!(code, 0);
    let (second, _, code) = run_cli(&["race", &path_text(&path)]);
    assert_eq!(code, 0);
    assert_eq!(second, first);
}

#[test]
fn the_race_command_names_the_defects_of_a_malformed_race() {
    let dir = temp_dir("cli-malformed");
    let path = write_json(&dir, "race.json", &obj(vec![("task", s("fix the parser"))]));
    let (out, _, code) = run_cli(&["race", &path_text(&path)]);
    assert_eq!(code, 1);
    assert_eq!(lines_of(&out)[0], "invalid recorded race");
    assert!(out.contains("defect missing field attempts"));
}

#[test]
fn the_race_command_names_an_unreadable_file() {
    let dir = temp_dir("cli-unreadable");
    let missing = dir.join("nope.json");
    let (out, _, code) = run_cli(&["race", &path_text(&missing)]);
    assert_eq!(code, 1);
    assert!(out.contains("defect file cannot be read"));
}

#[test]
fn the_race_command_updates_a_standings_file_it_creates() {
    let dir = temp_dir("cli-standings");
    let path = write_json(&dir, "race.json", &race_data());
    let standings_path = dir.join("standings.json");
    let (_, _, code) = run_cli(&[
        "race",
        &path_text(&path),
        "--standings",
        &path_text(&standings_path),
    ]);
    assert_eq!(code, 0);
    let stored =
        deadheat::json::parse(&std::fs::read_to_string(&standings_path).expect("stored standings"))
            .expect("stored json");
    assert_eq!(
        get_path(&stored, &["models", "model-a", "wins"]),
        Some(&i(1))
    );
    assert_eq!(
        get_path(&stored, &["models", "model-b", "tokens_unreported"]),
        Some(&i(1))
    );
    let (out, _, code) = run_cli(&["standings", &path_text(&standings_path)]);
    assert_eq!(code, 0);
    assert_eq!(
        lines_of(&out),
        vec![
            "season standings",
            "model-a, 1 wins, 0 losses, 0 no results, 4500 tokens, 0 unreported",
            "model-b, 0 wins, 1 losses, 0 no results, 0 tokens, 1 unreported",
        ]
    );
}

#[test]
fn the_race_command_creates_the_folder_for_a_new_standings_file() {
    let dir = temp_dir("cli-standings-nested");
    let path = write_json(&dir, "race.json", &race_data());
    let standings_path = dir.join("season").join("nested").join("standings.json");
    let (out, _, code) = run_cli(&[
        "race",
        &path_text(&path),
        "--standings",
        &path_text(&standings_path),
    ]);
    assert_eq!(code, 0);
    assert_eq!(out, EXPECTED_RACE_OUTPUT);
    assert!(standings_path.parent().expect("a parent").is_dir());
    let stored =
        deadheat::json::parse(&std::fs::read_to_string(&standings_path).expect("stored standings"))
            .expect("stored json");
    assert_eq!(
        get_path(&stored, &["models", "model-a", "wins"]),
        Some(&i(1))
    );
    assert_eq!(
        get_path(&stored, &["models", "model-b", "losses"]),
        Some(&i(1))
    );
}

#[test]
fn the_race_command_refuses_a_malformed_standings_file() {
    let dir = temp_dir("cli-standings-bad");
    let path = write_json(&dir, "race.json", &race_data());
    let standings_path = dir.join("standings.json");
    std::fs::write(&standings_path, "{\"models\": [1]}").expect("write");
    let (out, _, code) = run_cli(&[
        "race",
        &path_text(&path),
        "--standings",
        &path_text(&standings_path),
    ]);
    assert_eq!(code, 1);
    assert_eq!(
        lines_of(&out),
        vec![
            "invalid standings file",
            "defect field models has the wrong type",
        ]
    );
    assert_eq!(
        std::fs::read_to_string(&standings_path).expect("unchanged"),
        "{\"models\": [1]}"
    );
}

#[test]
fn the_adopt_command_writes_the_loser_patch() {
    let dir = temp_dir("cli-adopt");
    let path = write_json(&dir, "race.json", &race_data());
    let patches = dir.join("patches");
    let (out, _, code) = run_cli(&["adopt", &path_text(&path), &path_text(&patches)]);
    assert_eq!(code, 0);
    let target = patches.join("fix-the-parser-model-b.patch");
    assert_eq!(std::fs::read(&target).expect("bytes"), b"other diff");
    assert!(out.contains("fix-the-parser-model-b.patch"));
}

#[test]
fn the_adopt_command_refuses_a_race_with_no_winner() {
    let dir = temp_dir("cli-adopt-none");
    let path = write_json(&dir, "race.json", &no_winner_race());
    let patches = dir.join("patches");
    let (out, _, code) = run_cli(&["adopt", &path_text(&path), &path_text(&patches)]);
    assert_eq!(code, 1);
    assert_eq!(out.trim(), "no loser patch, the race had no winner");
    assert!(!patches.exists());
}

#[test]
fn the_discard_command_removes_the_patch_and_succeeds_twice() {
    let dir = temp_dir("cli-discard");
    let path = write_json(&dir, "race.json", &race_data());
    let patches = dir.join("patches");
    let (_, _, code) = run_cli(&["adopt", &path_text(&path), &path_text(&patches)]);
    assert_eq!(code, 0);
    let target = patches.join("fix-the-parser-model-b.patch");
    assert!(target.exists());
    let (_, _, code) = run_cli(&["discard", &path_text(&path), &path_text(&patches)]);
    assert_eq!(code, 0);
    assert!(!target.exists());
    let (_, _, code) = run_cli(&["discard", &path_text(&path), &path_text(&patches)]);
    assert_eq!(code, 0);
}

#[test]
fn the_discard_command_refuses_a_race_with_no_winner() {
    let dir = temp_dir("cli-discard-none");
    let path = write_json(&dir, "race.json", &no_winner_race());
    let patches = dir.join("patches");
    let (out, _, code) = run_cli(&["discard", &path_text(&path), &path_text(&patches)]);
    assert_eq!(code, 1);
    assert_eq!(out.trim(), "no loser patch, the race had no winner");
}

#[test]
fn the_standings_command_names_the_defects_of_a_malformed_file() {
    let dir = temp_dir("cli-standings-malformed");
    let path = dir.join("standings.json");
    std::fs::write(&path, "{}").expect("write");
    let (out, _, code) = run_cli(&["standings", &path_text(&path)]);
    assert_eq!(code, 1);
    assert_eq!(lines_of(&out)[0], "invalid standings file");
    assert!(out.contains("defect missing field models"));
}

#[test]
fn the_standings_command_names_an_unpaired_surrogate_in_a_model_key() {
    let dir = temp_dir("cli-standings-surrogate");
    let path = dir.join("standings.json");
    let text = "{\"models\": {\"bad\\ud800x\": {\"wins\": 0, \"losses\": 0, \"no_results\": 0, \"tokens\": 0, \"tokens_unreported\": 0}}}";
    std::fs::write(&path, text).expect("write");
    let (out, _, code) = run_cli(&["standings", &path_text(&path)]);
    assert_eq!(code, 1);
    assert_eq!(
        lines_of(&out),
        vec![
            "invalid standings file",
            "defect field models holds an unpaired surrogate",
        ]
    );
}

#[test]
fn the_race_command_refuses_a_model_holding_a_forged_verdict_line() {
    // The FIX-DH-13 refusal pinned at the command surface.
    let dir = temp_dir("cli-forged-race");
    let forged = obj(vec![
        ("task", s("t")),
        (
            "attempts",
            arr(vec![
                obj(vec![
                    ("model", s("prime-1\nverdict, shadow-1 wins")),
                    ("environment", s("box-a")),
                    ("result", s("green")),
                    ("finished_at", i(100)),
                    ("tokens", i(10)),
                    ("patch", s("first diff")),
                ]),
                obj(vec![
                    ("model", s("shadow-1")),
                    ("environment", s("box-b")),
                    ("result", s("green")),
                    ("finished_at", i(104)),
                    ("tokens", i(20)),
                    ("patch", s("second diff")),
                ]),
            ]),
        ),
        (
            "panel",
            arr(vec![
                obj(vec![
                    ("adjudicator", s("judge-a")),
                    ("chosen", s("shadow-1")),
                    ("confidence", f(0.9)),
                    ("reason", s("clear")),
                ]),
                obj(vec![
                    ("adjudicator", s("judge-b")),
                    ("chosen", s("shadow-1")),
                    ("confidence", f(0.9)),
                    ("reason", s("clear")),
                ]),
            ]),
        ),
    ]);
    let path = write_json(&dir, "race.json", &forged);
    let (out, _, code) = run_cli(&["race", &path_text(&path)]);
    assert_eq!(code, 1);
    assert_eq!(
        lines_of(&out),
        vec![
            "invalid recorded race",
            "defect attempt 0 field model holds a newline",
        ]
    );
}

#[test]
fn the_series_command_refuses_a_race_task_holding_forged_lines() {
    // The FIX-DH-13 refusal pinned at the series command.
    let dir = temp_dir("cli-forged-series");
    let forged_race = obj(vec![
        (
            "task",
            s("calm task\nrace 2, forged task, shadow-1 wins\ntally, prime-1 0 wins, shadow-1 9 wins, 0 no results"),
        ),
        (
            "attempts",
            arr(vec![
                obj(vec![
                    ("model", s("prime-1")),
                    ("environment", s("box-a")),
                    ("result", s("green")),
                    ("finished_at", i(100)),
                    ("tokens", i(10)),
                    ("patch", s("first diff")),
                ]),
                obj(vec![
                    ("model", s("shadow-1")),
                    ("environment", s("box-b")),
                    ("result", s("red")),
                    ("finished_at", i(130)),
                    ("tokens", i(20)),
                    ("patch", s("second diff")),
                ]),
            ]),
        ),
    ]);
    let forged = obj(vec![
        ("task", s("forgery series")),
        ("models", arr(vec![s("prime-1"), s("shadow-1")])),
        ("wins_target", i(3)),
        ("races", arr(vec![forged_race])),
    ]);
    let path = write_json(&dir, "series.json", &forged);
    let (out, _, code) = run_cli(&["series", &path_text(&path)]);
    assert_eq!(code, 1);
    assert_eq!(
        lines_of(&out),
        vec![
            "invalid series file",
            "defect race 0 field task holds a newline",
        ]
    );
}

#[test]
fn running_without_a_command_is_a_usage_error() {
    let (_, _, code) = run_cli(&[]);
    assert_eq!(code, 2);
}

// The port repair pins below. Each one names the finding it
// closes, and each was proven against the pre repair tree,
// where it fails, before it was allowed to stand here.

#[test]
fn a_top_level_help_option_with_a_value_draws_the_ignored_argument_refusal() {
    // Spec finding. The Python surface refuses a top level
    // --help=<value> with the ignored explicit argument error
    // at exit status 2, where the port once answered that the
    // command argument was required. The value is interpolated
    // through the repr mirror, so a value holding a single
    // quote draws double quotes.
    for (value, echoed) in [("1", "'1'"), ("", "''"), ("é", "'é'"), ("a'b", "\"a'b\"")] {
        let arg = format!("--help={}", value);
        let (_, err, code) = run_cli(&[&arg]);
        assert_eq!(code, 2, "{arg}");
        let last = err.lines().last().expect("an error line").to_string();
        assert_eq!(
            last,
            format!(
                "deadheat: error: argument -h/--help: ignored explicit argument {}",
                echoed
            ),
            "{arg}"
        );
    }
}

#[test]
fn a_subcommand_help_value_is_echoed_through_the_repr_mirror() {
    // Spec finding. The subcommand form of the same refusal
    // once interpolated its value raw between manual quotes,
    // where the Python surface runs it through the repr, so a
    // backslash doubles and a single quote flips the quotes.
    let (_, err, code) = run_cli(&["race", "--help=a\\b"]);
    assert_eq!(code, 2);
    let last = err.lines().last().expect("an error line").to_string();
    assert_eq!(
        last,
        "deadheat race: error: argument -h/--help: ignored explicit argument 'a\\\\b'"
    );
    let (_, err, code) = run_cli(&["race", "--help=a'b"]);
    assert_eq!(code, 2);
    let last = err.lines().last().expect("an error line").to_string();
    assert_eq!(
        last,
        "deadheat race: error: argument -h/--help: ignored explicit argument \"a'b\""
    );
}

#[test]
fn the_repr_mirror_escapes_the_code_points_the_python_repr_escapes() {
    // Spec finding. The old mirror held every character past
    // the ASCII range printable unless it was a control, where
    // the Python printable test refuses the separator, format,
    // and unassigned categories too. Each refused argument
    // below carries one of the code points the spec review
    // listed, and its invalid choice echo must carry the
    // escape the Python repr writes.
    for (point, escape) in [
        ('\u{200B}', "\\u200b"),
        ('\u{00A0}', "\\xa0"),
        ('\u{200E}', "\\u200e"),
        ('\u{FEFF}', "\\ufeff"),
        ('\u{2028}', "\\u2028"),
        ('\u{E0001}', "\\U000e0001"),
    ] {
        let arg = format!("bad{}x", point);
        let (_, err, code) = run_cli(&[&arg]);
        assert_eq!(code, 2, "{:?}", point);
        assert!(
            err.contains(&format!("invalid choice: 'bad{}x'", escape)),
            "{:?} escaped in {:?}",
            point,
            err
        );
    }
}

#[test]
fn the_columns_reading_takes_the_python_int_grammar() {
    // Spec finding. The COLUMNS reading once took ASCII
    // digits alone behind an optional plus. The Python int
    // conversion also takes underscores between digits, digits
    // of any Nd script at their values, and integers past 64
    // bits, which settle at the stored bound here. A refused
    // value falls through to the terminal reading, which names
    // eighty in this harness, whose output is no terminal.
    let columns = |value: &str| {
        std::env::set_var("COLUMNS", value);
        deadheat::cli::terminal_columns()
    };
    assert_eq!(columns("1_0"), 10);
    assert_eq!(columns("9_0"), 90);
    assert_eq!(columns("٩٠"), 90);
    assert_eq!(columns("+٩٠"), 90);
    assert_eq!(columns("４２"), 42);
    assert_eq!(columns(" 42 "), 42);
    assert_eq!(columns("999999999999999999999999999999"), 1_000_000);
    assert_eq!(columns("1__0"), 80);
    assert_eq!(columns("90_"), 80);
    assert_eq!(columns("_90"), 80);
    assert_eq!(columns("0"), 80);
    assert_eq!(columns("-3"), 80);
    assert_eq!(columns("²"), 80);
    assert_eq!(columns("abc"), 80);
    std::env::remove_var("COLUMNS");
}

#[test]
fn a_standings_sum_past_the_digit_limit_writes_nothing_and_prints_nothing() {
    // Red team Major 1. Two attempts of one model each carry
    // a token count of 4300 digits, which both loaders accept,
    // and their standings sum needs 4301 digits, which the
    // Python rendering refuses. The Python command exits 1
    // with no output and no standings file, and the port now
    // matches that surface exactly.
    let dir = temp_dir("cli-bigsum");
    let big = format!("6{}", "0".repeat(4299));
    let race_text = format!(
        "{{\"task\": \"big sum\", \"attempts\": [{{\"model\": \"a\", \"environment\": \"e1\", \"result\": \"green\", \"finished_at\": 10, \"tokens\": {0}, \"patch\": \"\"}}, {{\"model\": \"a\", \"environment\": \"e2\", \"result\": \"red\", \"finished_at\": 20, \"tokens\": {0}, \"patch\": \"\"}}]}}",
        big
    );
    let race_path = dir.join("race.json");
    std::fs::write(&race_path, race_text).expect("the race");
    let standings_path = dir.join("standings.json");
    let (out, _, code) = run_cli(&[
        "race",
        &path_text(&race_path),
        "--standings",
        &path_text(&standings_path),
    ]);
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(!standings_path.exists());
}

#[test]
fn a_standings_sum_at_exactly_the_digit_limit_renders() {
    // The other side of the same limit. A sum of exactly 4300
    // digits renders, and the standings file is written.
    let dir = temp_dir("cli-bigsum-at-limit");
    let big = format!("1{}", "0".repeat(4299));
    let race_text = format!(
        "{{\"task\": \"big sum\", \"attempts\": [{{\"model\": \"a\", \"environment\": \"e1\", \"result\": \"green\", \"finished_at\": 10, \"tokens\": {0}, \"patch\": \"\"}}, {{\"model\": \"b\", \"environment\": \"e2\", \"result\": \"red\", \"finished_at\": 20, \"tokens\": null, \"patch\": \"\"}}]}}",
        big
    );
    let race_path = dir.join("race.json");
    std::fs::write(&race_path, race_text).expect("the race");
    let standings_path = dir.join("standings.json");
    let (_, _, code) = run_cli(&[
        "race",
        &path_text(&race_path),
        "--standings",
        &path_text(&standings_path),
    ]);
    assert_eq!(code, 0);
    let stored = std::fs::read_to_string(&standings_path).expect("the standings");
    assert!(stored.contains(&big), "the 4300 digit total stands");
}

#[cfg(unix)]
#[test]
fn a_filename_carrying_byte_ff_opens_and_echoes_raw() {
    // Red team Minor 3 and spec Minor 5. Byte 0xFF decodes to
    // the last surrogate escape unit, and the re-encode range
    // once stopped one short of it, so the byte opened as a
    // question mark and echoed as a replacement. The file
    // below carries the byte in its name and must open, and
    // the adopt echo must carry the raw byte back.
    use std::os::unix::ffi::OsStringExt;
    let dir = temp_dir("cli-ffname");
    let race_path = write_json(&dir, "race.json", &race_data());
    let mut name_bytes = dir.as_os_str().as_encoded_bytes().to_vec();
    name_bytes.push(b'/');
    name_bytes.extend_from_slice(b"race-");
    name_bytes.push(0xFF);
    name_bytes.extend_from_slice(b".json");
    let ff_path = std::path::PathBuf::from(std::ffi::OsString::from_vec(name_bytes.clone()));
    std::fs::copy(&race_path, &ff_path).expect("the byte named race");
    let args: Vec<deadheat::pytext::PText> = vec![
        deadheat::pytext::PText::from_text("race"),
        deadheat::cli::decode_arg(&name_bytes),
    ];
    let outcome = deadheat::cli::run(&args, 80);
    assert_eq!(outcome.code, 0);
    assert!(String::from_utf8_lossy(&outcome.stdout).contains("race fix the parser"));

    let mut folder_bytes = dir.as_os_str().as_encoded_bytes().to_vec();
    folder_bytes.extend_from_slice(b"/pat");
    folder_bytes.push(0xFF);
    folder_bytes.extend_from_slice(b"ch");
    let args: Vec<deadheat::pytext::PText> = vec![
        deadheat::pytext::PText::from_text("adopt"),
        deadheat::pytext::PText::from_text(&path_text(&race_path)),
        deadheat::cli::decode_arg(&folder_bytes),
    ];
    let outcome = deadheat::cli::run(&args, 80);
    assert_eq!(outcome.code, 0);
    assert!(
        outcome.stdout.contains(&0xFF),
        "the echo carries the raw byte, {:?}",
        outcome.stdout
    );
    let folder = std::path::PathBuf::from(std::ffi::OsString::from_vec(folder_bytes));
    assert!(folder.join("fix-the-parser-model-b.patch").exists());
}

#[test]
fn the_closed_pipe_exits_match_the_python_surface() {
    // Red team Minor 2 under PORT-10, and the middle band of
    // the port re-review under PORT-12. The binary is driven
    // through a python3 fixture that hands it a pipe whose
    // reader is already gone, the arrangement the red team
    // probed. The driver first measures each fixture output
    // on a live pipe and checks it lands in the band its case
    // means to pin, so a rendering drift cannot move a case
    // into another band silently.
    let dir = temp_dir("cli-pipes");
    let empty = dir.join("empty.json");
    std::fs::write(&empty, "{\"models\": {}}\n").expect("the empty standings");
    let standings = |name: &str, count: usize, full: bool| {
        let mut text = String::from("{\"models\": {");
        for index in 0..count {
            if index > 0 {
                text.push(',');
            }
            if full {
                text.push_str(&format!(
                    "\"model-{:04}\": {{\"wins\": 1, \"losses\": 2, \"no_results\": 3, \"tokens\": 123456, \"tokens_unreported\": 4}}",
                    index
                ));
            } else {
                text.push_str(&format!(
                    "\"model-{:04}\": {{\"wins\": 1, \"losses\": 2, \"no_results\": 3, \"tokens_unreported\": 4}}",
                    index
                ));
            }
        }
        text.push_str("}}\n");
        let path = dir.join(name);
        std::fs::write(&path, text).expect("the standings fixture");
        path
    };
    let big_path = standings("big.json", 400, true);
    let under_path = standings("under.json", 56, true);
    let band_low_path = standings("band-low.json", 57, true);
    let band_high_path = standings("band-high.json", 113, true);
    let over_path = standings("over.json", 114, true);
    let defects_path = standings("defects.json", 100, false);
    let missing = dir.join("nope.json");
    let script = dir.join("drive_pipes.py");
    std::fs::write(
        &script,
        r#"
import os, subprocess, sys
binary = sys.argv[1]
empty, big, under, band_low, band_high, over, defects, missing = sys.argv[2:10]
# label, argv, stream, wanted exit, band the open size must land in
CASES = [
    ("small", ["standings", empty], "stdout", 120, (0, 4096)),
    ("under", ["standings", under], "stdout", 120, (0, 4096)),
    ("band low", ["standings", band_low], "stdout", 0, (4097, 8191)),
    ("band high", ["standings", band_high], "stdout", 0, (4097, 8191)),
    ("band defects", ["standings", defects], "stdout", 1, (4097, 8191)),
    ("over", ["standings", over], "stdout", 1, (8192, 1 << 30)),
    ("large", ["standings", big], "stdout", 1, (8192, 1 << 30)),
    ("usage", [], "stderr", 120, None),
    ("help", ["race", "-h"], "stdout", 120, None),
    ("defects", ["standings", missing], "stdout", 120, None),
]
failed = []
for label, argv, stream, wanted, band in CASES:
    if band is not None:
        out = subprocess.run([binary] + argv, capture_output=True).stdout
        if not (band[0] <= len(out) <= band[1]):
            print(f"{label} open size {len(out)} outside band {band}")
            failed.append(label)
            continue
    read_fd, write_fd = os.pipe()
    os.close(read_fd)
    devnull = os.open(os.devnull, os.O_WRONLY)
    kwargs = {}
    if stream == "stdout":
        kwargs["stdout"] = write_fd
        kwargs["stderr"] = devnull
    else:
        kwargs["stderr"] = write_fd
        kwargs["stdout"] = devnull
    proc = subprocess.Popen([binary] + argv, **kwargs)
    os.close(write_fd)
    code = proc.wait()
    os.close(devnull)
    print(f"{label} exit {code} wanted {wanted}")
    if code != wanted:
        failed.append(label)
sys.exit(1 if failed else 0)
"#,
    )
    .expect("the driver");
    let output = std::process::Command::new("python3")
        .arg(&script)
        .arg(env!("CARGO_BIN_EXE_deadheat"))
        .arg(&empty)
        .arg(&big_path)
        .arg(&under_path)
        .arg(&band_low_path)
        .arg(&band_high_path)
        .arg(&over_path)
        .arg(&defects_path)
        .arg(&missing)
        .output()
        .expect("the driver runs");
    assert!(
        output.status.success(),
        "pipe exits: {}",
        String::from_utf8_lossy(&output.stdout)
    );
}
