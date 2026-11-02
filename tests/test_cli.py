"""Pins for the command line surface in CONTRACT.md.

The commands are driven through main with real files, and their
printed bytes and exit statuses are pinned against hand written
expectations.
"""

import json

import pytest

from deadheat.cli import main

RACE = {
    "task": "fix the parser",
    "attempts": [
        {
            "model": "model-a",
            "environment": "lane one",
            "result": "green",
            "finished_at": 120.5,
            "tokens": 4500,
            "patch": "diff text",
        },
        {
            "model": "model-b",
            "environment": "lane two",
            "result": "red",
            "finished_at": 90.25,
            "tokens": None,
            "patch": "other diff",
        },
    ],
}

NO_WINNER_RACE = {
    "task": "fix the parser",
    "attempts": [
        {
            "model": "model-a",
            "environment": "lane one",
            "result": "red",
            "finished_at": 10.0,
            "tokens": 100,
            "patch": "diff text",
        },
        {
            "model": "model-b",
            "environment": "lane two",
            "result": "red",
            "finished_at": 20.0,
            "tokens": 200,
            "patch": "other diff",
        },
    ],
}

EXPECTED_RACE_OUTPUT = (
    "race fix the parser\n"
    "model-a in lane one, green in 120.5 seconds, 4500 tokens\n"
    "model-b in lane two, red in 90.25 seconds, tokens unreported\n"
    "verdict, model-a wins\n"
    "funeral for fix the parser\n"
    "winner model-a in lane one went green in 120.5 seconds\n"
    "loser model-b in lane two went red in 90.25 seconds\n"
    "margin 30.25 seconds between the two finishes\n"
)


def write_json(tmp_path, name, data):
    path = tmp_path / name
    path.write_text(json.dumps(data), encoding="utf-8")
    return path


def test_the_race_command_prints_the_report_and_the_funeral(tmp_path, capsys):
    path = write_json(tmp_path, "race.json", RACE)
    assert main(["race", str(path)]) == 0
    assert capsys.readouterr().out == EXPECTED_RACE_OUTPUT


def test_the_race_command_is_deterministic(tmp_path, capsys):
    path = write_json(tmp_path, "race.json", RACE)
    assert main(["race", str(path)]) == 0
    first = capsys.readouterr().out
    assert main(["race", str(path)]) == 0
    assert capsys.readouterr().out == first


def test_the_race_command_names_the_defects_of_a_malformed_race(tmp_path, capsys):
    path = write_json(tmp_path, "race.json", {"task": "fix the parser"})
    assert main(["race", str(path)]) == 1
    out = capsys.readouterr().out
    assert out.splitlines()[0] == "invalid recorded race"
    assert "defect missing field attempts" in out


def test_the_race_command_names_an_unreadable_file(tmp_path, capsys):
    assert main(["race", str(tmp_path / "nope.json")]) == 1
    out = capsys.readouterr().out
    assert "defect file cannot be read" in out


def test_the_race_command_updates_a_standings_file_it_creates(tmp_path, capsys):
    path = write_json(tmp_path, "race.json", RACE)
    standings_path = tmp_path / "standings.json"
    assert main(["race", str(path), "--standings", str(standings_path)]) == 0
    capsys.readouterr()
    stored = json.loads(standings_path.read_text(encoding="utf-8"))
    assert stored["models"]["model-a"]["wins"] == 1
    assert stored["models"]["model-b"]["tokens_unreported"] == 1
    assert main(["standings", str(standings_path)]) == 0
    out = capsys.readouterr().out
    assert out.splitlines() == [
        "season standings",
        "model-a, 1 wins, 0 losses, 0 no results, 4500 tokens, 0 unreported",
        "model-b, 0 wins, 1 losses, 0 no results, 0 tokens, 1 unreported",
    ]


def test_the_race_command_creates_the_folder_for_a_new_standings_file(
    tmp_path, capsys
):
    path = write_json(tmp_path, "race.json", RACE)
    standings_path = tmp_path / "season" / "nested" / "standings.json"
    assert main(["race", str(path), "--standings", str(standings_path)]) == 0
    assert capsys.readouterr().out == EXPECTED_RACE_OUTPUT
    assert standings_path.parent.is_dir()
    stored = json.loads(standings_path.read_text(encoding="utf-8"))
    assert stored["models"]["model-a"]["wins"] == 1
    assert stored["models"]["model-b"]["losses"] == 1


def test_the_race_command_refuses_a_malformed_standings_file(tmp_path, capsys):
    path = write_json(tmp_path, "race.json", RACE)
    standings_path = tmp_path / "standings.json"
    standings_path.write_text('{"models": [1]}', encoding="utf-8")
    assert main(["race", str(path), "--standings", str(standings_path)]) == 1
    out = capsys.readouterr().out
    assert out.splitlines() == [
        "invalid standings file",
        "defect field models has the wrong type",
    ]
    assert standings_path.read_text(encoding="utf-8") == '{"models": [1]}'


def test_the_adopt_command_writes_the_loser_patch(tmp_path, capsys):
    path = write_json(tmp_path, "race.json", RACE)
    patches = tmp_path / "patches"
    assert main(["adopt", str(path), str(patches)]) == 0
    target = patches / "fix-the-parser-model-b.patch"
    assert target.read_bytes() == b"other diff"
    out = capsys.readouterr().out
    assert "fix-the-parser-model-b.patch" in out


def test_the_adopt_command_refuses_a_race_with_no_winner(tmp_path, capsys):
    path = write_json(tmp_path, "race.json", NO_WINNER_RACE)
    patches = tmp_path / "patches"
    assert main(["adopt", str(path), str(patches)]) == 1
    out = capsys.readouterr().out
    assert out.strip() == "no loser patch, the race had no winner"
    assert not patches.exists()


def test_the_discard_command_removes_the_patch_and_succeeds_twice(tmp_path, capsys):
    path = write_json(tmp_path, "race.json", RACE)
    patches = tmp_path / "patches"
    assert main(["adopt", str(path), str(patches)]) == 0
    capsys.readouterr()
    target = patches / "fix-the-parser-model-b.patch"
    assert target.exists()
    assert main(["discard", str(path), str(patches)]) == 0
    capsys.readouterr()
    assert not target.exists()
    assert main(["discard", str(path), str(patches)]) == 0


def test_the_discard_command_refuses_a_race_with_no_winner(tmp_path, capsys):
    path = write_json(tmp_path, "race.json", NO_WINNER_RACE)
    assert main(["discard", str(path), str(tmp_path / "patches")]) == 1
    out = capsys.readouterr().out
    assert out.strip() == "no loser patch, the race had no winner"


def test_the_standings_command_names_the_defects_of_a_malformed_file(tmp_path, capsys):
    path = tmp_path / "standings.json"
    path.write_text("{}", encoding="utf-8")
    assert main(["standings", str(path)]) == 1
    out = capsys.readouterr().out
    assert out.splitlines()[0] == "invalid standings file"
    assert "defect missing field models" in out


def test_the_standings_command_names_an_unpaired_surrogate_in_a_model_key(
    tmp_path, capsys
):
    entry = {
        "wins": 0,
        "losses": 0,
        "no_results": 0,
        "tokens": 0,
        "tokens_unreported": 0,
    }
    path = write_json(tmp_path, "standings.json", {"models": {"bad\ud800x": entry}})
    assert main(["standings", str(path)]) == 1
    out = capsys.readouterr().out
    assert out.splitlines() == [
        "invalid standings file",
        "defect field models holds an unpaired surrogate",
    ]


def test_running_without_a_command_is_a_usage_error():
    with pytest.raises(SystemExit) as excinfo:
        main([])
    assert excinfo.value.code == 2
