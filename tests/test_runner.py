"""Pins for the local runner in the DH-1 runner text of CONTRACT.md.

Every command here is trivial and local, a shell exit, a small file
write, or a short sleep against a shorter timeout. Nothing touches
the network and nothing heavy runs.
"""

import json

from deadheat.races import load_race_text
from deadheat.runner import run_attempt, run_race


def test_a_passing_command_records_green_with_a_finish(tmp_path):
    attempt = run_attempt(tmp_path, "lane", "model-a", "local shell", "exit 0")
    assert attempt["result"] == "green"
    assert attempt["finished_at"] >= 0
    assert attempt["tokens"] is None
    assert attempt["patch"] == ""
    assert (tmp_path / "lane").is_dir()


def test_a_failing_command_records_red_with_a_finish(tmp_path):
    attempt = run_attempt(tmp_path, "lane", "model-a", "local shell", "exit 3")
    assert attempt["result"] == "red"
    assert "finished_at" in attempt


def test_a_named_result_wins_over_the_exit_code(tmp_path):
    command = "printf '%s' '{\"result\": \"red\"}' > result.json"
    attempt = run_attempt(tmp_path, "lane", "model-a", "local shell", command)
    assert attempt["result"] == "red"


def test_a_named_unfinished_result_carries_no_finish(tmp_path):
    command = "printf '%s' '{\"result\": \"unfinished\"}' > result.json; exit 0"
    attempt = run_attempt(tmp_path, "lane", "model-a", "local shell", command)
    assert attempt["result"] == "unfinished"
    assert "finished_at" not in attempt


def test_reported_tokens_are_recorded(tmp_path):
    command = "printf '%s' '{\"tokens\": 321}' > result.json"
    attempt = run_attempt(tmp_path, "lane", "model-a", "local shell", command)
    assert attempt["result"] == "green"
    assert attempt["tokens"] == 321


def test_a_negative_token_count_in_the_result_file_is_not_recorded(tmp_path):
    command = "printf '%s' '{\"tokens\": -4}' > result.json"
    attempt = run_attempt(tmp_path, "lane", "model-a", "local shell", command)
    assert attempt["tokens"] is None


def test_a_malformed_result_file_falls_back_to_the_exit_code(tmp_path):
    command = "printf '%s' 'not json at all' > result.json; exit 3"
    attempt = run_attempt(tmp_path, "lane", "model-a", "local shell", command)
    assert attempt["result"] == "red"
    assert attempt["tokens"] is None


def test_a_patch_file_the_command_wrote_is_kept_verbatim(tmp_path):
    command = "printf '%s' 'fake diff body' > patch.diff"
    attempt = run_attempt(tmp_path, "lane", "model-a", "local shell", command)
    assert attempt["patch"] == "fake diff body"


def test_an_unreadable_result_word_falls_back_but_its_tokens_still_count(tmp_path):
    command = "printf '%s' '{\"result\": \"blue\", \"tokens\": 321}' > result.json"
    attempt = run_attempt(tmp_path, "lane", "model-a", "local shell", command)
    assert attempt["result"] == "green"
    assert attempt["tokens"] == 321


def test_a_second_attempt_in_the_same_lane_reads_only_its_own_files(tmp_path):
    first_command = (
        "printf '%s' '{\"result\": \"green\", \"tokens\": 99}' > result.json; "
        "printf '%s' 'first patch' > patch.diff"
    )
    first = run_attempt(tmp_path, "lane", "model-a", "local shell", first_command)
    assert first["result"] == "green"
    assert first["tokens"] == 99
    assert first["patch"] == "first patch"

    second = run_attempt(tmp_path, "lane", "model-a", "local shell", "exit 1")
    assert second["result"] == "red"
    assert "finished_at" in second
    assert second["tokens"] is None
    assert second["patch"] == ""
    assert not (tmp_path / "lane" / "result.json").exists()
    assert not (tmp_path / "lane" / "patch.diff").exists()


def test_a_second_race_on_one_workspace_reads_only_its_own_files(tmp_path):
    first_specs = [
        {
            "model": "model-a",
            "environment": "local shell",
            "command": (
                "printf '%s' '{\"result\": \"green\", \"tokens\": 5}' "
                "> result.json; printf '%s' 'first patch' > patch.diff"
            ),
        },
        {"model": "model-b", "environment": "local shell", "command": "exit 0"},
    ]
    first_race = run_race(tmp_path, "fix the parser", first_specs)
    assert first_race["attempts"][0]["tokens"] == 5
    assert first_race["attempts"][0]["patch"] == "first patch"

    second_specs = [
        {"model": "model-a", "environment": "local shell", "command": "exit 1"},
        {"model": "model-b", "environment": "local shell", "command": "exit 0"},
    ]
    second_race = run_race(tmp_path, "fix the parser", second_specs)
    assert second_race["attempts"][0]["result"] == "red"
    assert second_race["attempts"][0]["tokens"] is None
    assert second_race["attempts"][0]["patch"] == ""


def test_a_command_past_its_timeout_is_unfinished(tmp_path):
    attempt = run_attempt(
        tmp_path,
        "lane",
        "model-a",
        "local shell",
        "sleep 2",
        timeout_seconds=0.2,
    )
    assert attempt["result"] == "unfinished"
    assert "finished_at" not in attempt
    assert attempt["tokens"] is None


def test_each_attempt_runs_in_its_own_directory(tmp_path):
    specs = [
        {"model": "model-a", "environment": "local shell", "command": "printf a > mark.txt"},
        {"model": "model-b", "environment": "local shell", "command": "printf b > mark.txt; exit 1"},
    ]
    race = run_race(tmp_path, "fix the parser", specs)
    assert (tmp_path / "attempt-1" / "mark.txt").read_text(encoding="utf-8") == "a"
    assert (tmp_path / "attempt-2" / "mark.txt").read_text(encoding="utf-8") == "b"
    assert race["attempts"][0]["result"] == "green"
    assert race["attempts"][1]["result"] == "red"


def test_a_run_race_record_loads_clean(tmp_path):
    specs = [
        {"model": "model-a", "environment": "local shell", "command": "exit 0"},
        {"model": "model-b", "environment": "local shell", "command": "exit 1"},
    ]
    race = run_race(tmp_path, "fix the parser", specs)
    loaded_race, defects = load_race_text(json.dumps(race))
    assert defects == []
    assert loaded_race["task"] == "fix the parser"
    assert loaded_race["attempts"][0]["result"] == "green"
    assert loaded_race["attempts"][1]["result"] == "red"
