"""Pins for DH-1 to DH-4 in the full form section of CONTRACT.md.

Every defect phrase the loader can name is pinned against the
malformed shape that draws it, the winner rule is pinned in each of
its outcomes, and the standings update is pinned as exact dicts.
"""

import copy
import json
from fractions import Fraction

from deadheat.races import (
    adopt_patch,
    decide_winner,
    discard_patch,
    empty_standings,
    format_seconds,
    funeral,
    load_race_file,
    load_race_text,
    load_standings_text,
    patch_file_name,
    race_report,
    slugify,
    standings_table,
    standings_text,
    update_standings,
)


def valid_attempt(**overrides):
    attempt = {
        "model": "model-a",
        "environment": "lane one",
        "result": "green",
        "finished_at": 120.5,
        "tokens": 4500,
        "patch": "diff text",
    }
    attempt.update(overrides)
    return attempt


def other_attempt(**overrides):
    attempt = {
        "model": "model-b",
        "environment": "lane two",
        "result": "red",
        "finished_at": 90.25,
        "tokens": None,
        "patch": "other diff",
    }
    attempt.update(overrides)
    return attempt


def valid_race(**overrides):
    race = {"task": "fix the parser", "attempts": [valid_attempt(), other_attempt()]}
    race.update(overrides)
    return race


def race_with_attempt(index, attempt):
    race = valid_race()
    race["attempts"] = list(race["attempts"])
    race["attempts"][index] = attempt
    return race


def without(data, *keys):
    out = dict(data)
    for key in keys:
        del out[key]
    return out


def defects_of(data):
    _race, defects = load_race_text(json.dumps(data))
    return defects


def loaded(data):
    race, defects = load_race_text(json.dumps(data))
    assert defects == []
    return race


# The loader, DH-1


def test_a_valid_race_loads_with_its_shape_kept():
    race = loaded(valid_race())
    assert race["task"] == "fix the parser"
    assert race["attempts"][0]["model"] == "model-a"
    assert race["attempts"][0]["finished_at"] == 120.5
    assert race["attempts"][1]["tokens"] is None


def test_fields_the_shape_does_not_name_are_ignored():
    data = valid_race(extra="ignored")
    data["attempts"] = [valid_attempt(surprise=1), other_attempt()]
    assert defects_of(data) == []


def test_text_that_is_not_json_draws_the_not_valid_json_defect():
    race, defects = load_race_text("{not json")
    assert race is None
    assert defects == ["input is not valid JSON"]


def test_deeply_nested_input_never_raises():
    race, defects = load_race_text("[" * 100000 + "]" * 100000)
    assert race is None
    assert defects != []


def test_a_non_object_root_draws_the_not_an_object_defect():
    race, defects = load_race_text("[1, 2]")
    assert race is None
    assert defects == ["input is not a JSON object"]


def test_missing_task_is_named():
    assert defects_of(without(valid_race(), "task")) == ["missing field task"]


def test_a_task_of_the_wrong_type_is_named():
    assert defects_of(valid_race(task=7)) == ["field task has the wrong type"]


def test_an_empty_task_is_named():
    assert defects_of(valid_race(task="")) == ["field task is empty"]


def test_a_task_holding_an_unpaired_surrogate_is_named():
    assert defects_of(valid_race(task="bad\ud800x")) == [
        "field task holds an unpaired surrogate"
    ]


def test_a_task_holding_a_newline_is_named():
    assert defects_of(valid_race(task="two\nlines")) == [
        "field task holds a newline"
    ]


def test_a_task_holding_a_carriage_return_is_named():
    assert defects_of(valid_race(task="two\rlines")) == [
        "field task holds a newline"
    ]


def test_missing_attempts_is_named():
    assert defects_of(without(valid_race(), "attempts")) == [
        "missing field attempts"
    ]


def test_attempts_of_the_wrong_type_is_named():
    assert defects_of(valid_race(attempts="two")) == [
        "field attempts has the wrong type"
    ]


def test_one_attempt_is_named_as_not_exactly_two():
    assert defects_of(valid_race(attempts=[valid_attempt()])) == [
        "field attempts must hold exactly two attempts"
    ]


def test_three_attempts_are_named_as_not_exactly_two():
    data = valid_race(
        attempts=[valid_attempt(), other_attempt(), valid_attempt(model="model-c")]
    )
    assert defects_of(data) == ["field attempts must hold exactly two attempts"]


def test_an_attempt_that_is_not_an_object_is_named():
    assert defects_of(race_with_attempt(0, "fast")) == [
        "attempt 0 is not an object"
    ]


def test_a_missing_model_is_named_with_its_position():
    assert defects_of(race_with_attempt(1, without(other_attempt(), "model"))) == [
        "attempt 1 is missing field model"
    ]


def test_a_model_of_the_wrong_type_is_named():
    assert defects_of(race_with_attempt(0, valid_attempt(model=3))) == [
        "attempt 0 field model has the wrong type"
    ]


def test_a_missing_environment_is_named_with_its_position():
    assert defects_of(race_with_attempt(0, without(valid_attempt(), "environment"))) == [
        "attempt 0 is missing field environment"
    ]


def test_an_environment_of_the_wrong_type_is_named():
    assert defects_of(race_with_attempt(0, valid_attempt(environment=3))) == [
        "attempt 0 field environment has the wrong type"
    ]


def test_an_empty_environment_is_named():
    assert defects_of(race_with_attempt(0, valid_attempt(environment=""))) == [
        "attempt 0 field environment is empty"
    ]


def test_an_environment_holding_an_unpaired_surrogate_is_named():
    assert defects_of(race_with_attempt(0, valid_attempt(environment="lane\ud800"))) == [
        "attempt 0 field environment holds an unpaired surrogate"
    ]


def test_a_model_holding_a_newline_is_named():
    assert defects_of(race_with_attempt(0, valid_attempt(model="two\nlines"))) == [
        "attempt 0 field model holds a newline"
    ]


def test_a_model_holding_a_carriage_return_is_named_with_its_position():
    assert defects_of(race_with_attempt(1, other_attempt(model="two\rlines"))) == [
        "attempt 1 field model holds a newline"
    ]


def test_an_environment_holding_a_newline_is_named():
    assert defects_of(
        race_with_attempt(0, valid_attempt(environment="two\nlines"))
    ) == ["attempt 0 field environment holds a newline"]


def test_an_environment_holding_a_carriage_return_is_named_with_its_position():
    assert defects_of(
        race_with_attempt(1, other_attempt(environment="two\rlines"))
    ) == ["attempt 1 field environment holds a newline"]


def test_an_empty_model_is_named():
    assert defects_of(race_with_attempt(0, valid_attempt(model=""))) == [
        "attempt 0 field model is empty"
    ]


def test_a_model_holding_an_unpaired_surrogate_is_named():
    assert defects_of(race_with_attempt(0, valid_attempt(model="m\ud800"))) == [
        "attempt 0 field model holds an unpaired surrogate"
    ]


def test_a_missing_result_is_named():
    assert defects_of(race_with_attempt(0, without(valid_attempt(), "result"))) == [
        "attempt 0 is missing field result"
    ]


def test_a_result_of_the_wrong_type_is_named():
    assert defects_of(race_with_attempt(0, valid_attempt(result=1))) == [
        "attempt 0 field result has the wrong type"
    ]


def test_a_result_outside_the_three_words_is_named():
    assert defects_of(race_with_attempt(0, valid_attempt(result="blue"))) == [
        "attempt 0 field result is not a suite result"
    ]


def test_missing_tokens_is_named():
    assert defects_of(race_with_attempt(0, without(valid_attempt(), "tokens"))) == [
        "attempt 0 is missing field tokens"
    ]


def test_fractional_tokens_are_named_as_the_wrong_type():
    assert defects_of(race_with_attempt(0, valid_attempt(tokens=1.5))) == [
        "attempt 0 field tokens has the wrong type"
    ]


def test_boolean_tokens_are_named_as_the_wrong_type():
    assert defects_of(race_with_attempt(0, valid_attempt(tokens=True))) == [
        "attempt 0 field tokens has the wrong type"
    ]


def test_negative_tokens_are_named():
    assert defects_of(race_with_attempt(0, valid_attempt(tokens=-1))) == [
        "attempt 0 field tokens is negative"
    ]


def test_zero_tokens_load_clean():
    assert defects_of(race_with_attempt(0, valid_attempt(tokens=0))) == []


def test_missing_patch_is_named():
    assert defects_of(race_with_attempt(1, without(other_attempt(), "patch"))) == [
        "attempt 1 is missing field patch"
    ]


def test_a_patch_of_the_wrong_type_is_named():
    assert defects_of(race_with_attempt(0, valid_attempt(patch=9))) == [
        "attempt 0 field patch has the wrong type"
    ]


def test_a_patch_holding_an_unpaired_surrogate_is_named():
    assert defects_of(race_with_attempt(0, valid_attempt(patch="d\ud800"))) == [
        "attempt 0 field patch holds an unpaired surrogate"
    ]


def test_a_green_attempt_without_finished_at_is_named():
    data = race_with_attempt(0, without(valid_attempt(), "finished_at"))
    assert defects_of(data) == ["attempt 0 is missing field finished_at"]


def test_a_red_attempt_without_finished_at_is_named():
    data = race_with_attempt(1, without(other_attempt(), "finished_at"))
    assert defects_of(data) == ["attempt 1 is missing field finished_at"]


def test_an_unfinished_attempt_with_finished_at_is_named():
    data = race_with_attempt(
        1, other_attempt(result="unfinished", finished_at=12.0)
    )
    assert defects_of(data) == [
        "attempt 1 field finished_at is present but the suite did not finish"
    ]


def test_an_unfinished_attempt_without_finished_at_loads_clean():
    data = race_with_attempt(
        1, without(other_attempt(result="unfinished"), "finished_at")
    )
    race = loaded(data)
    assert "finished_at" not in race["attempts"][1]


def test_a_finished_at_of_the_wrong_type_is_named():
    assert defects_of(race_with_attempt(0, valid_attempt(finished_at="soon"))) == [
        "attempt 0 field finished_at has the wrong type"
    ]


def test_an_infinite_finished_at_is_named_as_not_finite():
    data = race_with_attempt(0, valid_attempt(finished_at=float("inf")))
    assert defects_of(data) == [
        "attempt 0 field finished_at is not a finite number"
    ]


def test_a_huge_integer_finished_at_is_named_as_not_finite():
    data = race_with_attempt(0, valid_attempt(finished_at=10**400))
    assert defects_of(data) == [
        "attempt 0 field finished_at is not a finite number"
    ]


def test_a_negative_finished_at_is_named():
    assert defects_of(race_with_attempt(0, valid_attempt(finished_at=-0.5))) == [
        "attempt 0 field finished_at is negative"
    ]


def test_several_defects_arrive_in_one_pass():
    data = without(valid_race(), "task", "attempts")
    assert defects_of(data) == ["missing field task", "missing field attempts"]


def test_load_race_file_names_an_unreadable_path(tmp_path):
    race, defects = load_race_file(tmp_path / "nope.json")
    assert race is None
    assert defects == ["file cannot be read"]


def test_load_race_file_reads_a_written_race(tmp_path):
    path = tmp_path / "race.json"
    path.write_text(json.dumps(valid_race()), encoding="utf-8")
    race, defects = load_race_file(path)
    assert defects == []
    assert race["task"] == "fix the parser"


# The winner rule, DH-2


def test_only_the_first_attempt_green_wins():
    winner, loser = decide_winner(loaded(valid_race()))
    assert winner["model"] == "model-a"
    assert loser["model"] == "model-b"


def test_only_the_second_attempt_green_wins():
    data = valid_race(
        attempts=[
            valid_attempt(result="red", finished_at=30.0),
            other_attempt(result="green", finished_at=44.0, tokens=10),
        ]
    )
    winner, loser = decide_winner(loaded(data))
    assert winner["model"] == "model-b"
    assert loser["model"] == "model-a"


def test_both_green_the_earlier_finish_wins():
    data = valid_race(
        attempts=[
            valid_attempt(finished_at=200.0),
            other_attempt(result="green", finished_at=150.0),
        ]
    )
    winner, _loser = decide_winner(loaded(data))
    assert winner["model"] == "model-b"


def test_both_green_an_exact_tie_goes_to_the_first_attempt():
    data = valid_race(
        attempts=[
            valid_attempt(finished_at=100.0),
            other_attempt(result="green", finished_at=100.0),
        ]
    )
    winner, loser = decide_winner(loaded(data))
    assert winner["model"] == "model-a"
    assert loser["model"] == "model-b"


def test_green_beats_unfinished():
    data = valid_race(
        attempts=[
            without(other_attempt(result="unfinished", model="model-b"), "finished_at"),
            valid_attempt(model="model-a"),
        ]
    )
    winner, _loser = decide_winner(loaded(data))
    assert winner["model"] == "model-a"


def test_red_against_unfinished_has_no_winner():
    data = valid_race(
        attempts=[
            valid_attempt(result="red", finished_at=10.0),
            without(other_attempt(result="unfinished"), "finished_at"),
        ]
    )
    assert decide_winner(loaded(data)) == (None, None)


def test_both_red_has_no_winner():
    data = valid_race(attempts=[valid_attempt(result="red"), other_attempt()])
    assert decide_winner(loaded(data)) == (None, None)


def test_both_unfinished_has_no_winner():
    data = valid_race(
        attempts=[
            without(valid_attempt(result="unfinished"), "finished_at"),
            without(other_attempt(result="unfinished"), "finished_at"),
        ]
    )
    assert decide_winner(loaded(data)) == (None, None)


# The report and the funeral, DH-3


def test_the_report_lines_for_a_decided_race():
    assert race_report(loaded(valid_race())) == [
        "race fix the parser",
        "model-a in lane one, green in 120.5 seconds, 4500 tokens",
        "model-b in lane two, red in 90.25 seconds, tokens unreported",
        "verdict, model-a wins",
    ]


def test_the_report_states_no_winner_when_neither_goes_green():
    data = valid_race(attempts=[valid_attempt(result="red"), other_attempt()])
    lines = race_report(loaded(data))
    assert lines[-2] == "verdict, no winner"
    assert lines[-1] == "close call, no adjudication on record"


def test_the_funeral_lines_for_a_decided_race():
    assert funeral(loaded(valid_race())) == [
        "funeral for fix the parser",
        "winner model-a in lane one went green in 120.5 seconds",
        "loser model-b in lane two went red in 90.25 seconds",
        "margin 30.25 seconds between the two finishes",
    ]


def test_the_funeral_lines_for_a_race_with_no_winner():
    data = valid_race(
        attempts=[
            valid_attempt(result="red", finished_at=10.0),
            without(other_attempt(result="unfinished"), "finished_at"),
        ]
    )
    assert funeral(loaded(data)) == [
        "funeral for fix the parser",
        "no winner, neither suite went green",
        "model-a in lane one went red in 10 seconds",
        "model-b in lane two never finished",
    ]


def test_the_margin_names_the_gap_when_the_loser_finished_first():
    data = valid_race(
        attempts=[
            valid_attempt(result="red", finished_at=40.0, model="model-b"),
            other_attempt(result="green", finished_at=70.0, model="model-a"),
        ]
    )
    lines = funeral(loaded(data))
    assert lines[-1] == "margin 30 seconds between the two finishes"


def test_no_margin_line_when_the_loser_never_finished():
    data = valid_race(
        attempts=[
            valid_attempt(),
            without(other_attempt(result="unfinished"), "finished_at"),
        ]
    )
    lines = funeral(loaded(data))
    assert not any(line.startswith("margin") for line in lines)
    assert lines[2] == "loser model-b in lane two never finished"


def test_a_sub_precision_margin_renders_at_full_precision():
    # Altered under FIX-DH-8, a named mechanical consequence of the
    # amendment. The pin first asserted the float repr prefix of
    # the margin line, and the amendment retires float rendering,
    # so the line now carries the exact expansion of the gap in
    # plain decimal notation. The guard against a zero margin
    # stands.
    data = valid_race(
        attempts=[
            valid_attempt(finished_at=1.0),
            other_attempt(result="green", finished_at=1.0000004),
        ]
    )
    lines = funeral(loaded(data))
    assert lines[-1] == (
        "margin 0.0000004000000000115022658064845018088817596435546875 "
        "seconds between the two finishes"
    )
    assert "margin 0 seconds" not in lines[-1]


def test_format_seconds_renders_plain_values():
    assert format_seconds(30.25) == "30.25"
    assert format_seconds(10.0) == "10"
    assert format_seconds(0) == "0"


def test_format_seconds_renders_a_large_integer_digit_for_digit():
    assert format_seconds(123456789012345678) == "123456789012345678"
    assert format_seconds(100000000000000000001) == (
        "100000000000000000001"
    )


def test_format_seconds_rounds_an_exact_half_up_on_the_exact_value():
    assert format_seconds(0.0078125) == "0.007813"


def test_format_seconds_renders_a_terminating_sub_floor_figure_in_full():
    assert format_seconds(1 / 2**22) == "0.0000002384185791015625"


def test_format_seconds_rounds_a_non_terminating_sub_floor_figure():
    assert format_seconds(Fraction(1, 30000000)) == "0.000000033333"


# The loser patch operations, DH-3


def test_slugify_reduces_labels():
    assert slugify("Model A!") == "model-a"
    assert slugify("weird__name") == "weird-name"
    assert slugify("a  b") == "a-b"
    assert slugify("!!!") == "unnamed"


def test_the_patch_file_name_joins_the_slugs():
    assert patch_file_name(loaded(valid_race())) == "fix-the-parser-model-b.patch"


def test_adopt_writes_the_loser_patch_verbatim(tmp_path):
    race = loaded(valid_race())
    target = adopt_patch(race, tmp_path / "patches")
    assert target is not None
    assert target.read_bytes() == b"other diff"
    assert target.name == "fix-the-parser-model-b.patch"


def test_adopt_keeps_an_unusual_patch_byte_for_byte(tmp_path):
    race = loaded(
        race_with_attempt(1, other_attempt(patch="line one\nline two  \n"))
    )
    target = adopt_patch(race, tmp_path)
    assert target.read_bytes() == b"line one\nline two  \n"


def test_discard_removes_an_adopted_patch(tmp_path):
    race = loaded(valid_race())
    target = adopt_patch(race, tmp_path)
    returned = discard_patch(race, tmp_path)
    assert returned == target
    assert not target.exists()


def test_discard_succeeds_when_no_patch_stands_there(tmp_path):
    race = loaded(valid_race())
    returned = discard_patch(race, tmp_path)
    assert returned == tmp_path / "fix-the-parser-model-b.patch"


def test_adopt_and_discard_refuse_a_race_with_no_loser(tmp_path):
    data = valid_race(attempts=[valid_attempt(result="red"), other_attempt()])
    race = loaded(data)
    assert adopt_patch(race, tmp_path) is None
    assert discard_patch(race, tmp_path) is None
    assert list(tmp_path.iterdir()) == []


# The standings, DH-4


def test_recording_a_decided_race():
    standings = update_standings(empty_standings(), loaded(valid_race()))
    assert standings == {
        "models": {
            "model-a": {
                "wins": 1,
                "losses": 0,
                "no_results": 0,
                "tokens": 4500,
                "tokens_unreported": 0,
            },
            "model-b": {
                "wins": 0,
                "losses": 1,
                "no_results": 0,
                "tokens": 0,
                "tokens_unreported": 1,
            },
        }
    }


def test_standings_accumulate_across_two_races():
    first = update_standings(empty_standings(), loaded(valid_race()))
    second_race = loaded(
        valid_race(
            task="fix the parser again",
            attempts=[
                valid_attempt(result="red", finished_at=12.0, tokens=100),
                without(other_attempt(result="unfinished"), "finished_at"),
            ],
        )
    )
    standings = update_standings(first, second_race)
    assert standings["models"]["model-a"] == {
        "wins": 1,
        "losses": 0,
        "no_results": 1,
        "tokens": 4600,
        "tokens_unreported": 0,
    }
    assert standings["models"]["model-b"] == {
        "wins": 0,
        "losses": 1,
        "no_results": 1,
        "tokens": 0,
        "tokens_unreported": 2,
    }


def test_null_tokens_count_as_unreported_and_never_as_zero():
    race = loaded(
        valid_race(
            attempts=[valid_attempt(tokens=None), other_attempt(tokens=None)]
        )
    )
    standings = update_standings(empty_standings(), race)
    assert standings["models"]["model-a"]["tokens"] == 0
    assert standings["models"]["model-a"]["tokens_unreported"] == 1
    assert standings["models"]["model-b"]["tokens_unreported"] == 1


def test_the_update_never_mutates_its_input():
    before = update_standings(empty_standings(), loaded(valid_race()))
    snapshot = copy.deepcopy(before)
    update_standings(before, loaded(valid_race(task="another task")))
    assert before == snapshot


def test_one_model_in_both_lanes_records_both_sides():
    race = loaded(
        valid_race(
            attempts=[
                valid_attempt(model="solo", finished_at=10.0, tokens=5),
                other_attempt(model="solo", result="green", finished_at=20.0, tokens=7),
            ]
        )
    )
    standings = update_standings(empty_standings(), race)
    assert standings["models"]["solo"] == {
        "wins": 1,
        "losses": 1,
        "no_results": 0,
        "tokens": 12,
        "tokens_unreported": 0,
    }


def test_the_standings_table_renders_in_label_order():
    standings = update_standings(empty_standings(), loaded(valid_race()))
    assert standings_table(standings) == [
        "season standings",
        "model-a, 1 wins, 0 losses, 0 no results, 4500 tokens, 0 unreported",
        "model-b, 0 wins, 1 losses, 0 no results, 0 tokens, 1 unreported",
    ]


def test_the_empty_standings_table_says_no_races_recorded():
    assert standings_table(empty_standings()) == [
        "season standings",
        "no races recorded",
    ]


def test_standings_text_round_trips_through_the_loader():
    standings = update_standings(empty_standings(), loaded(valid_race()))
    parsed, defects = load_standings_text(standings_text(standings))
    assert defects == []
    assert parsed == standings


def test_standings_loader_names_a_missing_models_field():
    _standings, defects = load_standings_text("{}")
    assert defects == ["missing field models"]


def test_standings_loader_names_a_models_field_of_the_wrong_type():
    _standings, defects = load_standings_text('{"models": [1]}')
    assert defects == ["field models has the wrong type"]


def test_standings_loader_names_a_bad_entry():
    text = json.dumps({"models": {"model-a": {"wins": 1}}})
    _standings, defects = load_standings_text(text)
    assert "standings entry model-a is missing field losses" in defects


def test_standings_loader_names_a_negative_count():
    entry = {
        "wins": -1,
        "losses": 0,
        "no_results": 0,
        "tokens": 0,
        "tokens_unreported": 0,
    }
    _standings, defects = load_standings_text(json.dumps({"models": {"m": entry}}))
    assert defects == ["standings entry m field wins is negative"]


def test_standings_loader_names_an_entry_that_is_not_an_object():
    _standings, defects = load_standings_text(
        json.dumps({"models": {"model-a": 7}})
    )
    assert defects == ["standings entry model-a is not an object"]


def test_standings_loader_names_an_entry_field_of_the_wrong_type():
    entry = {
        "wins": 0,
        "losses": 0,
        "no_results": 0,
        "tokens": "many",
        "tokens_unreported": 0,
    }
    _standings, defects = load_standings_text(
        json.dumps({"models": {"model-a": entry}})
    )
    assert defects == [
        "standings entry model-a field tokens has the wrong type"
    ]


def test_standings_loader_names_an_unpaired_surrogate_in_a_model_key():
    entry = {
        "wins": 0,
        "losses": 0,
        "no_results": 0,
        "tokens": 0,
        "tokens_unreported": 0,
    }
    standings, defects = load_standings_text(
        json.dumps({"models": {"bad\ud800x": entry}})
    )
    assert standings is None
    assert defects == ["field models holds an unpaired surrogate"]


def test_standings_loader_names_a_newline_in_a_model_key():
    entry = {
        "wins": 0,
        "losses": 0,
        "no_results": 0,
        "tokens": 0,
        "tokens_unreported": 0,
    }
    standings, defects = load_standings_text(
        json.dumps({"models": {"bad\nkey": entry}})
    )
    assert standings is None
    assert defects == ["field models holds a newline"]
