"""Pins for DH-6 in the Round 2 section of CONTRACT.md.

Every series defect phrase is pinned at its exact text, the
running tally is pinned race by race, and the decided, beyond the
decision, and unfinished readings are pinned as full line lists.
"""

import json

from deadheat.cli import main
from deadheat.series import load_series_file, load_series_text, series_report

PRIME = "prime-1"
SHADOW = "shadow-1"


def clear_win(task, winner, loser):
    return {
        "task": task,
        "attempts": [
            {
                "model": winner,
                "environment": "lane one",
                "result": "green",
                "finished_at": 100.0,
                "tokens": 10,
                "patch": "winner diff",
            },
            {
                "model": loser,
                "environment": "lane two",
                "result": "red",
                "finished_at": 130.0,
                "tokens": 20,
                "patch": "loser diff",
            },
        ],
    }


def no_result(task, first, second):
    return {
        "task": task,
        "attempts": [
            {
                "model": first,
                "environment": "lane one",
                "result": "red",
                "finished_at": 100.0,
                "tokens": 10,
                "patch": "first diff",
            },
            {
                "model": second,
                "environment": "lane two",
                "result": "red",
                "finished_at": 140.0,
                "tokens": 20,
                "patch": "second diff",
            },
        ],
    }


def adjudicated_win(task, winner, loser):
    race = {
        "task": task,
        "attempts": [
            {
                "model": winner,
                "environment": "lane one",
                "result": "green",
                "finished_at": 104.0,
                "tokens": 10,
                "patch": "winner diff",
            },
            {
                "model": loser,
                "environment": "lane two",
                "result": "green",
                "finished_at": 100.0,
                "tokens": 20,
                "patch": "loser diff",
            },
        ],
        "adjudication": {
            "adjudicator": "judge-1",
            "chosen": winner,
            "confidence": 0.9,
            "reason": "the steadier diff",
        },
    }
    return race


def series_with(races, wins_target=2, models=(PRIME, SHADOW), task="the import saga"):
    return {
        "task": task,
        "models": list(models),
        "wins_target": wins_target,
        "races": races,
    }


def without(data, *keys):
    out = dict(data)
    for key in keys:
        del out[key]
    return out


def loaded(data):
    series, defects = load_series_text(json.dumps(data))
    assert defects == []
    return series


def defects_of(data):
    _series, defects = load_series_text(json.dumps(data))
    return defects


# The loader, DH-6


def test_a_valid_series_loads_with_its_shape_kept():
    series = loaded(series_with([clear_win("race one", PRIME, SHADOW)]))
    assert series["task"] == "the import saga"
    assert series["models"] == [PRIME, SHADOW]
    assert series["wins_target"] == 2
    assert series["races"][0]["task"] == "race one"


def test_a_series_with_no_races_loads_clean():
    series = loaded(series_with([]))
    assert series["races"] == []


def test_non_json_and_non_object_inputs_are_named():
    _series, defects = load_series_text("not json")
    assert defects == ["input is not valid JSON"]
    _series, defects = load_series_text("[1, 2]")
    assert defects == ["input is not a JSON object"]


def test_load_series_file_names_an_unreadable_path(tmp_path):
    series, defects = load_series_file(tmp_path / "nope.json")
    assert series is None
    assert defects == ["file cannot be read"]


def test_the_task_field_draws_the_race_loader_phrases():
    assert defects_of(without(series_with([]), "task")) == ["missing field task"]
    assert defects_of(series_with([], task=7)) == [
        "field task has the wrong type"
    ]
    assert defects_of(series_with([], task="")) == ["field task is empty"]


def test_a_missing_or_misshapen_models_field_is_named():
    assert defects_of(without(series_with([]), "models")) == [
        "missing field models"
    ]
    data = series_with([])
    data["models"] = "prime-1"
    assert defects_of(data) == ["field models has the wrong type"]


def test_a_models_list_of_the_wrong_size_is_named():
    for models in ([PRIME], [PRIME, SHADOW, "third-3"], []):
        assert defects_of(series_with([], models=models)) == [
            "field models must hold exactly two models"
        ]


def test_models_entries_are_named_by_index():
    assert defects_of(series_with([], models=(PRIME, 7))) == [
        "models entry 1 has the wrong type"
    ]
    assert defects_of(series_with([], models=("", SHADOW))) == [
        "models entry 0 is empty"
    ]
    assert defects_of(series_with([], models=("bad\ud800x", SHADOW))) == [
        "models entry 0 holds an unpaired surrogate"
    ]


def test_the_same_model_twice_in_the_models_field_is_named():
    assert defects_of(series_with([], models=(PRIME, PRIME))) == [
        "field models names the same model twice"
    ]


def test_the_wins_target_is_named_when_missing_misshapen_or_not_positive():
    assert defects_of(without(series_with([]), "wins_target")) == [
        "missing field wins_target"
    ]
    assert defects_of(series_with([], wins_target="two")) == [
        "field wins_target has the wrong type"
    ]
    assert defects_of(series_with([], wins_target=True)) == [
        "field wins_target has the wrong type"
    ]
    for value in (0, -2):
        assert defects_of(series_with([], wins_target=value)) == [
            "field wins_target is not positive"
        ]


def test_a_missing_or_misshapen_races_field_is_named():
    assert defects_of(without(series_with([]), "races")) == [
        "missing field races"
    ]
    data = series_with([])
    data["races"] = "not a list"
    assert defects_of(data) == ["field races has the wrong type"]


def test_a_race_entry_that_is_not_an_object_is_named():
    assert defects_of(series_with(["not a race"])) == [
        "race 0 is not an object"
    ]


def test_a_malformed_race_names_its_defects_with_its_index():
    bad = clear_win("race two", PRIME, SHADOW)
    del bad["attempts"][0]["model"]
    data = series_with([clear_win("race one", PRIME, SHADOW), bad])
    assert defects_of(data) == ["race 1 attempt 0 is missing field model"]


def test_an_adjudication_defect_inside_a_series_race_is_prefixed():
    bad = adjudicated_win("race one", PRIME, SHADOW)
    bad["adjudication"]["confidence"] = 1.5
    assert defects_of(series_with([bad])) == [
        "race 0 adjudication field confidence is outside the range "
        "zero to one"
    ]


def test_a_race_naming_a_third_model_is_named():
    data = series_with([clear_win("race one", PRIME, "intruder-9")])
    assert defects_of(data) == ["race 0 names a model outside the series"]


def test_a_race_naming_the_same_model_twice_is_named():
    data = series_with([clear_win("race one", PRIME, PRIME)])
    assert defects_of(data) == ["race 0 names the same model twice"]


# The series report, DH-6


def test_the_series_report_tallies_race_by_race_and_names_the_winner():
    series = loaded(
        series_with(
            [
                clear_win("race one", PRIME, SHADOW),
                no_result("race two", PRIME, SHADOW),
                clear_win("race three", SHADOW, PRIME),
                adjudicated_win("race four", PRIME, SHADOW),
                clear_win("race five", SHADOW, PRIME),
            ]
        )
    )
    assert series_report(series) == [
        "series the import saga",
        "prime-1 against shadow-1, first to 2 wins",
        "race 1, race one, prime-1 wins",
        "tally, prime-1 1 wins, shadow-1 0 wins, 0 no results",
        "race 2, race two, no result",
        "tally, prime-1 1 wins, shadow-1 0 wins, 1 no results",
        "race 3, race three, shadow-1 wins",
        "tally, prime-1 1 wins, shadow-1 1 wins, 1 no results",
        "race 4, race four, prime-1 wins",
        "tally, prime-1 2 wins, shadow-1 1 wins, 1 no results",
        "race 5, race five, beyond the decision",
        "series winner, prime-1",
    ]


def test_a_series_that_reaches_its_target_at_the_last_race():
    series = loaded(
        series_with(
            [clear_win("race one", SHADOW, PRIME), clear_win("race two", SHADOW, PRIME)],
            wins_target=2,
        )
    )
    lines = series_report(series)
    assert lines[-1] == "series winner, shadow-1"
    assert not any("beyond the decision" in line for line in lines)


def test_an_unfinished_series_states_the_target_is_not_reached():
    series = loaded(
        series_with(
            [clear_win("race one", PRIME, SHADOW), no_result("race two", PRIME, SHADOW)],
            wins_target=3,
        )
    )
    assert series_report(series) == [
        "series the import saga",
        "prime-1 against shadow-1, first to 3 wins",
        "race 1, race one, prime-1 wins",
        "tally, prime-1 1 wins, shadow-1 0 wins, 0 no results",
        "race 2, race two, no result",
        "tally, prime-1 1 wins, shadow-1 0 wins, 1 no results",
        "series unfinished, the target of 3 wins is not reached",
    ]


def test_an_empty_series_stands_unfinished():
    series = loaded(series_with([], wins_target=2))
    assert series_report(series) == [
        "series the import saga",
        "prime-1 against shadow-1, first to 2 wins",
        "series unfinished, the target of 2 wins is not reached",
    ]


def test_a_target_of_one_decides_the_series_at_the_first_race():
    series = loaded(
        series_with(
            [clear_win("race one", PRIME, SHADOW), clear_win("race two", SHADOW, PRIME)],
            wins_target=1,
        )
    )
    lines = series_report(series)
    assert lines[4] == "race 2, race two, beyond the decision"
    assert lines[-1] == "series winner, prime-1"


def test_the_models_field_order_governs_the_tally_order():
    series = loaded(
        series_with(
            [clear_win("race one", PRIME, SHADOW)], models=(SHADOW, PRIME)
        )
    )
    lines = series_report(series)
    assert lines[1] == "shadow-1 against prime-1, first to 2 wins"
    assert lines[3] == "tally, shadow-1 0 wins, prime-1 1 wins, 0 no results"


# The series command, Round 2 command line surface


def write_series(tmp_path, data):
    path = tmp_path / "series.json"
    path.write_text(json.dumps(data), encoding="utf-8")
    return path


def test_the_series_command_prints_the_report(tmp_path, capsys):
    data = series_with([clear_win("race one", PRIME, SHADOW)], wins_target=1)
    path = write_series(tmp_path, data)
    assert main(["series", str(path)]) == 0
    out = capsys.readouterr().out
    assert out == "\n".join(series_report(loaded(data))) + "\n"


def test_the_series_command_names_the_defects_of_a_malformed_series(
    tmp_path, capsys
):
    path = write_series(tmp_path, series_with([clear_win("race one", PRIME, "intruder-9")]))
    assert main(["series", str(path)]) == 1
    out = capsys.readouterr().out
    assert out.splitlines() == [
        "invalid series file",
        "defect race 0 names a model outside the series",
    ]


def test_the_series_command_names_an_unreadable_file(tmp_path, capsys):
    assert main(["series", str(tmp_path / "nope.json")]) == 1
    out = capsys.readouterr().out
    assert "defect file cannot be read" in out
