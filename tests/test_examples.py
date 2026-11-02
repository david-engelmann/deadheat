"""Pins for the recorded example races and their golden renderings.

Each golden file holds the exact bytes the race command prints for
its recorded race, so a rendering drift anywhere in the report or
the funeral fails here.
"""

import pathlib

from deadheat.cli import main
from deadheat.races import funeral, load_race_file, race_report

EXAMPLES = pathlib.Path(__file__).resolve().parent.parent / "examples"


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
