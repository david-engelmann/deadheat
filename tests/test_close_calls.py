"""Pins for DH-5 in the Round 2 section of CONTRACT.md.

The band edges are pinned on both sides and exactly on the
margin, the confidence threshold is pinned at and below four
fifths, every adjudication defect phrase is pinned at its exact
text, and the Round 1 outcomes for races the band does not touch
are pinned again here so a drift in the new step shows against
them.
"""

import json

from deadheat.cli import main
from deadheat.races import (
    close_call_report,
    decide_winner,
    empty_standings,
    funeral,
    is_close_call,
    load_race_text,
    race_report,
    update_standings,
)


def attempt(**overrides):
    data = {
        "model": "model-a",
        "environment": "lane one",
        "result": "green",
        "finished_at": 100.0,
        "tokens": 100,
        "patch": "diff text",
    }
    data.update(overrides)
    return data


def other(**overrides):
    data = {
        "model": "model-b",
        "environment": "lane two",
        "result": "green",
        "finished_at": 96.0,
        "tokens": 200,
        "patch": "other diff",
    }
    data.update(overrides)
    return data


def entry(**overrides):
    data = {
        "adjudicator": "judge-1",
        "chosen": "model-a",
        "confidence": 0.9,
        "reason": "the steadier diff",
    }
    data.update(overrides)
    return data


def race_with(attempts, adjudication="absent"):
    data = {"task": "fix the parser", "attempts": attempts}
    if adjudication != "absent":
        data["adjudication"] = adjudication
    return data


def without(data, *keys):
    out = dict(data)
    for key in keys:
        del out[key]
    return out


def loaded(data):
    race, defects = load_race_text(json.dumps(data))
    assert defects == []
    return race


def defects_of(data):
    _race, defects = load_race_text(json.dumps(data))
    return defects


# The band, DH-5


def test_both_green_well_inside_the_margin_is_a_close_call():
    race = loaded(race_with([attempt(), other()]))
    assert is_close_call(race)


def test_both_green_exactly_at_the_margin_is_a_close_call():
    race = loaded(race_with([attempt(), other(finished_at=95.0)]))
    assert is_close_call(race)


def test_both_green_a_decimal_case_inside_the_margin():
    race = loaded(
        race_with([attempt(finished_at=200.5), other(finished_at=190.5)])
    )
    assert is_close_call(race)


def test_both_green_just_outside_the_margin_is_not_a_close_call():
    race = loaded(race_with([attempt(), other(finished_at=94.9)]))
    assert not is_close_call(race)


def test_both_green_a_decimal_case_outside_the_margin():
    race = loaded(
        race_with([attempt(finished_at=200.5), other(finished_at=190.4)])
    )
    assert not is_close_call(race)


def test_both_green_a_sub_precision_gap_is_a_close_call():
    race = loaded(
        race_with([attempt(finished_at=1.0), other(finished_at=1.0000004)])
    )
    assert is_close_call(race)


def test_an_exact_tie_of_green_finishes_is_not_a_close_call():
    race = loaded(race_with([attempt(), other(finished_at=100.0)]))
    assert not is_close_call(race)


def test_both_red_is_a_close_call_at_any_times():
    race = loaded(
        race_with(
            [
                attempt(result="red", finished_at=10.0),
                other(result="red", finished_at=900.0),
            ]
        )
    )
    assert is_close_call(race)


def test_both_unfinished_is_a_close_call():
    race = loaded(
        race_with(
            [
                without(attempt(result="unfinished"), "finished_at"),
                without(other(result="unfinished"), "finished_at"),
            ]
        )
    )
    assert is_close_call(race)


def test_mixed_outcomes_are_not_close_calls():
    red_unfinished = loaded(
        race_with(
            [
                attempt(result="red"),
                without(other(result="unfinished"), "finished_at"),
            ]
        )
    )
    assert not is_close_call(red_unfinished)
    green_red = loaded(race_with([attempt(), other(result="red")]))
    assert not is_close_call(green_red)


# The adjudication entry and its defects, DH-5


def test_a_valid_entry_loads_and_is_kept_on_the_race():
    race = loaded(race_with([attempt(), other()], entry()))
    assert race["adjudication"] == {
        "adjudicator": "judge-1",
        "chosen": "model-a",
        "confidence": 0.9,
        "reason": "the steadier diff",
    }


def test_fields_the_entry_does_not_name_are_ignored():
    race = loaded(
        race_with([attempt(), other()], entry(probabilities={"model-a": 0.9}))
    )
    assert set(race["adjudication"]) == {
        "adjudicator",
        "chosen",
        "confidence",
        "reason",
    }


def test_an_entry_that_is_not_an_object_is_named():
    data = race_with([attempt(), other()], "a verdict")
    assert defects_of(data) == ["field adjudication is not an object"]


def test_each_missing_entry_field_is_named():
    for field in ("adjudicator", "chosen", "confidence", "reason"):
        data = race_with([attempt(), other()], without(entry(), field))
        assert defects_of(data) == [f"adjudication is missing field {field}"]


def test_entry_text_fields_of_the_wrong_type_are_named():
    for field in ("adjudicator", "chosen", "reason"):
        data = race_with([attempt(), other()], entry(**{field: 7}))
        assert defects_of(data) == [
            f"adjudication field {field} has the wrong type"
        ]


def test_empty_entry_text_fields_are_named():
    for field in ("adjudicator", "chosen", "reason"):
        data = race_with([attempt(), other()], entry(**{field: ""}))
        assert defects_of(data) == [f"adjudication field {field} is empty"]


def test_entry_text_fields_holding_a_surrogate_are_named():
    for field in ("adjudicator", "chosen", "reason"):
        data = race_with([attempt(), other()], entry(**{field: "bad\ud800x"}))
        assert defects_of(data) == [
            f"adjudication field {field} holds an unpaired surrogate"
        ]


def test_a_reason_holding_a_newline_is_named():
    data = race_with([attempt(), other()], entry(reason="two\nlines"))
    assert defects_of(data) == ["adjudication field reason holds a newline"]


def test_a_reason_holding_a_carriage_return_is_named():
    data = race_with([attempt(), other()], entry(reason="two\rlines"))
    assert defects_of(data) == ["adjudication field reason holds a newline"]


def test_an_adjudicator_holding_a_newline_is_named():
    data = race_with([attempt(), other()], entry(adjudicator="two\nlines"))
    assert defects_of(data) == [
        "adjudication field adjudicator holds a newline"
    ]


def test_an_adjudicator_holding_a_carriage_return_is_named():
    data = race_with([attempt(), other()], entry(adjudicator="two\rlines"))
    assert defects_of(data) == [
        "adjudication field adjudicator holds a newline"
    ]


def test_a_chosen_label_holding_a_newline_is_named():
    data = race_with([attempt(), other()], entry(chosen="two\nlines"))
    assert defects_of(data) == ["adjudication field chosen holds a newline"]


def test_a_chosen_label_holding_a_carriage_return_is_named():
    data = race_with([attempt(), other()], entry(chosen="two\rlines"))
    assert defects_of(data) == ["adjudication field chosen holds a newline"]


def test_a_confidence_of_the_wrong_type_is_named():
    data = race_with([attempt(), other()], entry(confidence="high"))
    assert defects_of(data) == [
        "adjudication field confidence has the wrong type"
    ]


def test_a_non_finite_confidence_is_named():
    for value in (float("nan"), float("inf")):
        data = race_with([attempt(), other()], entry(confidence=value))
        assert defects_of(data) == [
            "adjudication field confidence is not a finite number"
        ]


def test_a_confidence_outside_the_range_is_named():
    for value in (-0.1, 1.1):
        data = race_with([attempt(), other()], entry(confidence=value))
        assert defects_of(data) == [
            "adjudication field confidence is outside the range zero to one"
        ]


def test_the_confidence_range_edges_load_clean():
    for value in (0, 1):
        race = loaded(race_with([attempt(), other()], entry(confidence=value)))
        assert race["adjudication"]["confidence"] == value


def test_a_chosen_label_naming_no_attempt_is_named():
    data = race_with([attempt(), other()], entry(chosen="model-c"))
    assert defects_of(data) == [
        "adjudication field chosen names no attempt in this race"
    ]


# The winner rule step, DH-5


def test_an_entry_at_the_threshold_decides_for_the_chosen_attempt():
    race = loaded(
        race_with([attempt(), other()], entry(chosen="model-b", confidence=0.8))
    )
    winner, loser = decide_winner(race)
    assert winner["model"] == "model-b"
    assert loser["model"] == "model-a"


def test_an_entry_above_the_threshold_can_pick_the_slower_attempt():
    race = loaded(
        race_with([attempt(), other()], entry(chosen="model-a", confidence=0.95))
    )
    winner, _loser = decide_winner(race)
    assert winner["model"] == "model-a"


def test_an_entry_below_the_threshold_abstains():
    for value in (0.79, 0.7999999, 0.5):
        race = loaded(
            race_with([attempt(), other()], entry(confidence=value))
        )
        assert decide_winner(race) == (None, None)


def test_the_double_just_below_the_threshold_abstains():
    race = loaded(
        race_with(
            [attempt(), other()], entry(confidence=0.7999999999999999)
        )
    )
    assert decide_winner(race) == (None, None)


def test_a_close_call_with_no_entry_stands_as_no_result():
    race = loaded(race_with([attempt(), other()]))
    assert decide_winner(race) == (None, None)


# The abstention rendering, FIX-DH-10


FULL_PRECISION = "0.799999899999999986022203302127309143543243408203125"


def test_an_abstention_rounding_to_the_threshold_renders_in_full_precision():
    race = loaded(
        race_with([attempt(), other()], entry(confidence=0.7999999))
    )
    assert decide_winner(race) == (None, None)
    assert (
        "close call, adjudication by judge-1 abstained at confidence "
        f"{FULL_PRECISION}"
    ) in race_report(race)
    assert (
        "close call, adjudication by judge-1 abstained at confidence "
        f"{FULL_PRECISION}, the race stands as a dead heat"
    ) in funeral(race)
    assert (
        "adjudication by judge-1 abstained at confidence "
        f"{FULL_PRECISION}, below the threshold of 0.8"
    ) in close_call_report(race)


def test_a_confidence_at_the_threshold_renders_as_the_threshold_figure():
    race = loaded(
        race_with(
            [attempt(), other()], entry(chosen="model-b", confidence=0.8)
        )
    )
    assert (
        "adjudication by judge-1 chose model-b at confidence 0.8, "
        "the race is decided for model-b"
    ) in close_call_report(race)
    assert (
        "close call, adjudication by judge-1 chose model-b "
        "at confidence 0.8"
    ) in race_report(race)


def test_an_abstention_rounding_below_the_threshold_keeps_its_rendering():
    race = loaded(
        race_with([attempt(), other()], entry(confidence=0.7999994))
    )
    assert decide_winner(race) == (None, None)
    assert (
        "adjudication by judge-1 abstained at confidence 0.799999, "
        "below the threshold of 0.8"
    ) in close_call_report(race)
    assert (
        "close call, adjudication by judge-1 abstained at confidence "
        "0.799999"
    ) in race_report(race)


def test_a_both_red_close_call_is_decided_by_an_entry():
    race = loaded(
        race_with(
            [attempt(result="red"), other(result="red")],
            entry(chosen="model-b", confidence=0.85),
        )
    )
    winner, loser = decide_winner(race)
    assert winner["model"] == "model-b"
    assert loser["model"] == "model-a"


def test_a_chosen_label_carried_by_both_attempts_picks_the_first():
    twins = [
        attempt(model="model-a", result="red"),
        other(model="model-a", result="red"),
    ]
    race = loaded(race_with(twins, entry(chosen="model-a", confidence=0.9)))
    winner, _loser = decide_winner(race)
    assert winner["environment"] == "lane one"


def test_an_entry_on_a_race_outside_the_band_is_ignored():
    race = loaded(
        race_with(
            [attempt(finished_at=200.0), other(finished_at=150.0)],
            entry(chosen="model-a", confidence=0.99),
        )
    )
    winner, _loser = decide_winner(race)
    assert winner["model"] == "model-b"
    assert race_report(race)[-1] == (
        "adjudication by judge-1 not needed, "
        "the race stands outside the close call band"
    )
    assert not any("adjudication" in line for line in funeral(race))


# The close call rendering, DH-5


def test_the_report_and_funeral_of_a_decided_close_call():
    race = loaded(
        race_with([attempt(), other()], entry(chosen="model-b", confidence=0.9))
    )
    assert race_report(race) == [
        "race fix the parser",
        "model-a in lane one, green in 100 seconds, 100 tokens",
        "model-b in lane two, green in 96 seconds, 200 tokens",
        "verdict, model-b wins",
        "close call, adjudication by judge-1 chose model-b at confidence 0.9",
    ]
    lines = funeral(race)
    assert lines == [
        "funeral for fix the parser",
        "winner model-b in lane two went green in 96 seconds",
        "loser model-a in lane one went green in 100 seconds",
        "decided by adjudication, judge-1 chose model-b at confidence 0.9",
        "margin 4 seconds between the two finishes",
    ]


def test_the_report_and_funeral_of_an_abstained_close_call():
    race = loaded(
        race_with([attempt(), other()], entry(confidence=0.62))
    )
    assert race_report(race)[-2:] == [
        "verdict, no winner",
        "close call, adjudication by judge-1 abstained at confidence 0.62",
    ]
    lines = funeral(race)
    assert lines[1] == (
        "no winner, both suites went green inside the close call band"
    )
    assert lines[-2] == (
        "close call, adjudication by judge-1 abstained at confidence 0.62, "
        "the race stands as a dead heat"
    )
    assert lines[-1] == "margin 4 seconds between the two finishes"


def test_the_funeral_of_an_unrecorded_both_red_close_call():
    race = loaded(
        race_with([attempt(result="red"), other(result="red")])
    )
    lines = funeral(race)
    assert lines[1] == "no winner, neither suite went green"
    assert lines[-2] == (
        "close call, no adjudication on record, "
        "the race stands as a dead heat"
    )
    assert lines[-1] == "margin 4 seconds between the two finishes"


# The standings read adjudicated races through the winner rule, DH-5


def test_standings_record_an_adjudicated_win_and_an_abstention():
    decided = loaded(
        race_with([attempt(), other()], entry(chosen="model-b", confidence=0.9))
    )
    standings = update_standings(empty_standings(), decided)
    assert standings["models"]["model-b"]["wins"] == 1
    assert standings["models"]["model-a"]["losses"] == 1
    abstained = loaded(race_with([attempt(), other()], entry(confidence=0.5)))
    standings = update_standings(standings, abstained)
    assert standings["models"]["model-a"]["no_results"] == 1
    assert standings["models"]["model-b"]["no_results"] == 1


# Round 1 outcomes the band does not touch, pinned again under DH-5


def test_round_1_a_clear_both_green_margin_still_goes_to_the_earlier_finish():
    race = loaded(
        race_with([attempt(finished_at=200.0), other(finished_at=150.0)])
    )
    winner, _loser = decide_winner(race)
    assert winner["model"] == "model-b"
    assert race_report(race) == [
        "race fix the parser",
        "model-a in lane one, green in 200 seconds, 100 tokens",
        "model-b in lane two, green in 150 seconds, 200 tokens",
        "verdict, model-b wins",
    ]


def test_round_1_an_exact_tie_still_goes_to_the_first_attempt():
    race = loaded(race_with([attempt(), other(finished_at=100.0)]))
    winner, loser = decide_winner(race)
    assert winner["model"] == "model-a"
    assert loser["model"] == "model-b"
    assert not any("close call" in line for line in race_report(race))


def test_round_1_a_green_against_a_red_still_goes_to_the_green():
    race = loaded(race_with([attempt(), other(result="red")]))
    winner, _loser = decide_winner(race)
    assert winner["model"] == "model-a"
    assert funeral(race) == [
        "funeral for fix the parser",
        "winner model-a in lane one went green in 100 seconds",
        "loser model-b in lane two went red in 96 seconds",
        "margin 4 seconds between the two finishes",
    ]


def test_round_1_both_red_with_no_entry_still_records_a_no_result():
    race = loaded(race_with([attempt(result="red"), other(result="red")]))
    standings = update_standings(empty_standings(), race)
    assert standings["models"]["model-a"]["no_results"] == 1
    assert standings["models"]["model-b"]["no_results"] == 1


# The closecall command, Round 2 command line surface


def write_race(tmp_path, data):
    path = tmp_path / "race.json"
    path.write_text(json.dumps(data), encoding="utf-8")
    return path


def test_the_closecall_command_explains_a_decided_close_call(tmp_path, capsys):
    path = write_race(
        tmp_path,
        race_with([attempt(), other()], entry(chosen="model-b", confidence=0.9)),
    )
    assert main(["closecall", str(path)]) == 0
    assert capsys.readouterr().out.splitlines() == [
        "closecall fix the parser",
        "in the band, both attempts went green and the gap of 4 seconds "
        "is within five percent of the slower finish of 100 seconds",
        "adjudication by judge-1 chose model-b at confidence 0.9, "
        "the race is decided for model-b",
        "verdict, model-b wins",
    ]


def test_the_closecall_command_explains_an_abstention(tmp_path, capsys):
    path = write_race(tmp_path, race_with([attempt(), other()], entry(confidence=0.62)))
    assert main(["closecall", str(path)]) == 0
    lines = capsys.readouterr().out.splitlines()
    assert lines[2] == (
        "adjudication by judge-1 abstained at confidence 0.62, "
        "below the threshold of 0.8"
    )
    assert lines[-1] == "verdict, no winner"


def test_the_closecall_command_explains_a_race_outside_the_band(tmp_path, capsys):
    path = write_race(
        tmp_path, race_with([attempt(finished_at=200.0), other(finished_at=150.0)])
    )
    assert main(["closecall", str(path)]) == 0
    assert capsys.readouterr().out.splitlines() == [
        "closecall fix the parser",
        "outside the band, the gap of 50 seconds is wider than "
        "five percent of the slower finish of 200 seconds",
        "verdict, model-b wins",
    ]


def test_the_closecall_command_explains_a_tie(tmp_path, capsys):
    path = write_race(tmp_path, race_with([attempt(), other(finished_at=100.0)]))
    assert main(["closecall", str(path)]) == 0
    lines = capsys.readouterr().out.splitlines()
    assert lines[1] == (
        "outside the band, the finish times are tied and "
        "the tie goes to the first attempt in the record"
    )
    assert lines[-1] == "verdict, model-a wins"


def test_the_closecall_command_explains_a_both_red_close_call(tmp_path, capsys):
    path = write_race(
        tmp_path, race_with([attempt(result="red"), other(result="red")])
    )
    assert main(["closecall", str(path)]) == 0
    lines = capsys.readouterr().out.splitlines()
    assert lines[1] == "in the band, both attempts went red"
    assert lines[2] == "no adjudication on record, the race stands as no result"


def test_the_closecall_command_reports_the_lines_it_prints(tmp_path, capsys):
    path = write_race(tmp_path, race_with([attempt(), other()]))
    assert main(["closecall", str(path)]) == 0
    assert capsys.readouterr().out.splitlines() == [
        "closecall fix the parser",
        "in the band, both attempts went green and the gap of 4 seconds "
        "is within five percent of the slower finish of 100 seconds",
        "no adjudication on record, the race stands as no result",
        "verdict, no winner",
    ]


def test_a_gap_floats_cannot_see_is_not_reported_as_a_tie():
    race = loaded(
        race_with(
            [
                attempt(finished_at=100000000000000000001),
                other(finished_at=1e20),
            ],
            entry(chosen="model-a", confidence=0.9),
        )
    )
    assert close_call_report(race) == [
        "closecall fix the parser",
        "in the band, both attempts went green and the gap of 1 seconds "
        "is within five percent of the slower finish of "
        "100000000000000000001 seconds",
        "adjudication by judge-1 chose model-a at confidence 0.9, "
        "the race is decided for model-a",
        "verdict, model-a wins",
    ]
    assert funeral(race) == [
        "funeral for fix the parser",
        "winner model-a in lane one went green in "
        "100000000000000000001 seconds",
        "loser model-b in lane two went green in "
        "100000000000000000000 seconds",
        "decided by adjudication, judge-1 chose model-a at confidence 0.9",
        "margin 1 seconds between the two finishes",
    ]


def test_the_closecall_command_names_the_defects_of_a_malformed_race(
    tmp_path, capsys
):
    path = write_race(
        tmp_path,
        race_with([attempt(), other()], entry(confidence=1.5)),
    )
    assert main(["closecall", str(path)]) == 1
    out = capsys.readouterr().out
    assert out.splitlines() == [
        "invalid recorded race",
        "defect adjudication field confidence is outside the range "
        "zero to one",
    ]


def test_the_closecall_command_names_an_unreadable_file(tmp_path, capsys):
    assert main(["closecall", str(tmp_path / "nope.json")]) == 1
    out = capsys.readouterr().out
    assert "defect file cannot be read" in out
