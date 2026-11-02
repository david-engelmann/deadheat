"""Pins for the recorded example races and their golden renderings.

Each golden file holds the exact bytes the race command prints for
its recorded race, so a rendering drift anywhere in the report or
the funeral fails here. The Round 2 examples pin the race and
closecall commands for the close call races and the series command
for the series files the same way.
"""

import pathlib

from deadheat.cli import main
from deadheat.races import (
    close_call_report,
    funeral,
    load_race_file,
    race_report,
)
from deadheat.series import load_series_file, series_report

EXAMPLES = pathlib.Path(__file__).resolve().parent.parent / "examples"

ROUND_2_RACES = (
    "close-call-decided",
    "close-call-abstained",
    "close-call-unadjudicated",
    "outside-band-adjudicated",
)


def test_the_shadow_wins_example_matches_its_golden(capsys):
    source = EXAMPLES / "shadow-wins.json"
    assert main(["race", str(source)]) == 0
    out = capsys.readouterr().out
    golden = (EXAMPLES / "shadow-wins.txt").read_text(encoding="utf-8")
    assert out == golden
    assert "verdict, shadow-1 wins" in out


def test_the_neither_green_example_matches_its_golden(capsys):
    source = EXAMPLES / "neither-green.json"
    assert main(["race", str(source)]) == 0
    out = capsys.readouterr().out
    golden = (EXAMPLES / "neither-green.txt").read_text(encoding="utf-8")
    assert out == golden
    assert "verdict, no winner" in out


def test_the_goldens_equal_the_module_renderings():
    for name in ("shadow-wins", "neither-green"):
        race, defects = load_race_file(EXAMPLES / f"{name}.json")
        assert defects == []
        rendered = "\n".join(race_report(race) + funeral(race)) + "\n"
        golden = (EXAMPLES / f"{name}.txt").read_text(encoding="utf-8")
        assert rendered == golden


def test_the_round_2_race_examples_match_their_goldens(capsys):
    for name in ROUND_2_RACES:
        source = EXAMPLES / f"{name}.json"
        assert main(["race", str(source)]) == 0
        out = capsys.readouterr().out
        golden = (EXAMPLES / f"{name}.txt").read_text(encoding="utf-8")
        assert out == golden


def test_the_round_2_closecall_examples_match_their_goldens(capsys):
    for name in ROUND_2_RACES:
        source = EXAMPLES / f"{name}.json"
        assert main(["closecall", str(source)]) == 0
        out = capsys.readouterr().out
        golden = (EXAMPLES / f"{name}-closecall.txt").read_text(
            encoding="utf-8"
        )
        assert out == golden


def test_the_round_2_goldens_equal_the_module_renderings():
    for name in ROUND_2_RACES:
        race, defects = load_race_file(EXAMPLES / f"{name}.json")
        assert defects == []
        rendered = "\n".join(race_report(race) + funeral(race)) + "\n"
        golden = (EXAMPLES / f"{name}.txt").read_text(encoding="utf-8")
        assert rendered == golden
        rendered = "\n".join(close_call_report(race)) + "\n"
        golden = (EXAMPLES / f"{name}-closecall.txt").read_text(
            encoding="utf-8"
        )
        assert rendered == golden


def test_the_series_examples_match_their_goldens(capsys):
    for name in ("series-decided", "series-unfinished"):
        source = EXAMPLES / f"{name}.json"
        assert main(["series", str(source)]) == 0
        out = capsys.readouterr().out
        golden = (EXAMPLES / f"{name}.txt").read_text(encoding="utf-8")
        assert out == golden
        series, defects = load_series_file(source)
        assert defects == []
        assert "\n".join(series_report(series)) + "\n" == golden
