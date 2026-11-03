"""Pins for the panel shape, reading, rendering, and join, DH-9 and
DH-10 in CONTRACT.md.

The recorded panel pilot under examples is assembled from the
Round 3 pilot records, and the census pin below recomputes its
panel cases from the recorded figures by an independent path.
The recorded pairs hold no divided panel and no absent judge, so
the divided case and the absent judge case stand pinned here on
constructed records, built as data in this file and named as
constructed in the tests that use them. A constructed record is
never presented as recorded anywhere in this suite.
"""

import copy
import hashlib
import json
import pathlib
import runpy
from fractions import Fraction

from deadheat.panels import join_panel_records
from deadheat.races import (
    close_call_report,
    decide_winner,
    funeral,
    load_race_data,
    load_race_file,
    race_report,
    update_standings,
    empty_standings,
)
from deadheat.series import load_series_text, series_report

ROOT = pathlib.Path(__file__).resolve().parent.parent
EXAMPLES = ROOT / "examples"
PILOT_RECORDS = EXAMPLES / "pilot" / "records"
PANEL = EXAMPLES / "panel"
RACE_NAMES = [f"race-{index:02d}" for index in range(1, 11)]
ROUTES = ("jev", "openai")


def _attempt(model, finished, environment="home worktree"):
    return {
        "model": model,
        "environment": environment,
        "result": "green",
        "finished_at": finished,
        "tokens": 100,
        "patch": "",
    }


def _entry(adjudicator, chosen, confidence, reason="a reason"):
    return {
        "adjudicator": adjudicator,
        "chosen": chosen,
        "confidence": confidence,
        "reason": reason,
    }


def _band_data(**extra):
    data = {
        "task": "tighten the retry budget",
        "attempts": [_attempt("prime-1", 300), _attempt("shadow-1", 288)],
    }
    data.update(extra)
    return data


def _load(data):
    race, defects = load_race_data(data)
    assert defects == []
    return race


def _panel_race(panel, **extra):
    return _load(_band_data(panel=panel, **extra))


# Loader validation of the panel shape


def test_a_two_entry_panel_loads_in_entry_order():
    first = _entry("judge-a", "prime-1", 0.9)
    second = _entry("judge-b", "shadow-1", 0.85)
    race = _panel_race([first, second])
    assert race["panel"] == [first, second]


def test_a_panel_entry_ignores_fields_it_does_not_name():
    entry = _entry("judge-a", "prime-1", 0.9)
    entry["extra"] = "ignored"
    race = _panel_race([entry])
    assert race["panel"] == [_entry("judge-a", "prime-1", 0.9)]


def test_a_panel_that_is_not_a_list_draws_its_defect():
    race, defects = load_race_data(_band_data(panel={"adjudicator": "j"}))
    assert race is None
    assert defects == ["field panel has the wrong type"]


def test_an_empty_panel_draws_its_defect():
    race, defects = load_race_data(_band_data(panel=[]))
    assert race is None
    assert defects == ["field panel holds no entries"]


def test_a_panel_of_three_entries_draws_its_defect():
    panel = [
        _entry("judge-a", "prime-1", 0.9),
        _entry("judge-b", "prime-1", 0.9),
        _entry("judge-c", "prime-1", 0.9),
    ]
    race, defects = load_race_data(_band_data(panel=panel))
    assert race is None
    assert defects == ["field panel holds more than two entries"]


def test_a_record_carrying_an_entry_and_a_panel_draws_its_defect():
    data = _band_data(
        adjudication=_entry("judge-a", "prime-1", 0.9),
        panel=[_entry("judge-b", "prime-1", 0.9)],
    )
    race, defects = load_race_data(data)
    assert race is None
    assert defects == ["field panel stands beside an adjudication entry"]


def test_two_entries_from_one_adjudicator_draw_their_defect():
    panel = [
        _entry("judge-a", "prime-1", 0.9),
        _entry("judge-a", "shadow-1", 0.9),
    ]
    race, defects = load_race_data(_band_data(panel=panel))
    assert race is None
    assert defects == ["field panel names the same adjudicator twice"]


def test_a_panel_entry_that_is_not_an_object_draws_its_defect():
    race, defects = load_race_data(_band_data(panel=["judge-a"]))
    assert race is None
    assert defects == ["panel entry 0 is not an object"]


def test_a_panel_entry_missing_a_field_draws_its_defect():
    entry = _entry("judge-a", "prime-1", 0.9)
    del entry["chosen"]
    race, defects = load_race_data(_band_data(panel=[entry]))
    assert race is None
    assert defects == ["panel entry 0 is missing field chosen"]


def test_a_panel_entry_field_of_the_wrong_kind_draws_its_defect():
    entry = _entry("judge-a", 5, 0.9)
    race, defects = load_race_data(_band_data(panel=[entry]))
    assert race is None
    assert defects == ["panel entry 0 field chosen has the wrong type"]


def test_a_panel_entry_empty_text_field_draws_its_defect():
    entry = _entry("", "prime-1", 0.9)
    race, defects = load_race_data(_band_data(panel=[entry]))
    assert race is None
    assert defects == ["panel entry 0 field adjudicator is empty"]


def test_a_panel_entry_surrogate_text_field_draws_its_defect():
    entry = _entry("judge-a", "prime-1", 0.9, reason="bad\ud800reason")
    race, defects = load_race_data(_band_data(panel=[entry]))
    assert race is None
    assert defects == [
        "panel entry 0 field reason holds an unpaired surrogate"
    ]


def test_a_panel_entry_newline_in_any_text_field_draws_its_defect():
    for field in ("adjudicator", "chosen", "reason"):
        entry = _entry("judge-a", "prime-1", 0.9)
        entry[field] = "two\nlines"
        race, defects = load_race_data(_band_data(panel=[entry]))
        assert race is None
        assert defects == [
            f"panel entry 0 field {field} holds a newline"
        ], field


def test_a_panel_race_model_holding_a_forged_verdict_line_is_refused():
    data = {
        "task": "t",
        "attempts": [
            _attempt(
                "prime-1\nverdict, shadow-1 wins", 100, environment="box-a"
            ),
            _attempt("shadow-1", 104, environment="box-b"),
        ],
        "panel": [
            _entry("judge-a", "shadow-1", 0.9),
            _entry("judge-b", "shadow-1", 0.9),
        ],
    }
    data["attempts"][0]["tokens"] = 10
    data["attempts"][1]["tokens"] = 20
    race, defects = load_race_data(data)
    assert race is None
    assert defects == ["attempt 0 field model holds a newline"]


def test_a_panel_entry_missing_confidence_draws_its_defect():
    entry = _entry("judge-a", "prime-1", 0.9)
    del entry["confidence"]
    race, defects = load_race_data(_band_data(panel=[entry]))
    assert race is None
    assert defects == ["panel entry 0 is missing field confidence"]


def test_a_panel_entry_confidence_of_the_wrong_kind_draws_its_defect():
    entry = _entry("judge-a", "prime-1", "high")
    race, defects = load_race_data(_band_data(panel=[entry]))
    assert race is None
    assert defects == ["panel entry 0 field confidence has the wrong type"]


def test_a_panel_entry_non_finite_confidence_draws_its_defect():
    entry = _entry("judge-a", "prime-1", float("nan"))
    race, defects = load_race_data(_band_data(panel=[entry]))
    assert race is None
    assert defects == [
        "panel entry 0 field confidence is not a finite number"
    ]


def test_a_panel_entry_huge_integer_confidence_draws_the_finite_defect():
    entry = _entry("judge-a", "prime-1", 10**400)
    race, defects = load_race_data(_band_data(panel=[entry]))
    assert race is None
    assert defects == [
        "panel entry 0 field confidence is not a finite number"
    ]


def test_a_panel_entry_confidence_out_of_range_draws_its_defect():
    entry = _entry("judge-a", "prime-1", 1.5)
    race, defects = load_race_data(_band_data(panel=[entry]))
    assert race is None
    assert defects == [
        "panel entry 0 field confidence is outside the range zero to one"
    ]


def test_a_panel_entry_choosing_no_attempt_draws_its_defect():
    entry = _entry("judge-a", "nobody", 0.9)
    race, defects = load_race_data(_band_data(panel=[entry]))
    assert race is None
    assert defects == [
        "panel entry 0 field chosen names no attempt in this race"
    ]


def test_the_second_panel_entry_names_its_own_index_in_defects():
    panel = [_entry("judge-a", "prime-1", 0.9), _entry("judge-b", "x", 0.9)]
    race, defects = load_race_data(_band_data(panel=panel))
    assert race is None
    assert defects == [
        "panel entry 1 field chosen names no attempt in this race"
    ]


# The panel reading through the winner rule


def test_a_confirmed_panel_decides_for_the_agreed_attempt():
    race = _panel_race(
        [_entry("judge-a", "prime-1", 0.9), _entry("judge-b", "prime-1", 0.85)]
    )
    winner, loser = decide_winner(race)
    assert winner["model"] == "prime-1"
    assert loser["model"] == "shadow-1"


def test_a_constructed_divided_panel_decides_nothing():
    race = _panel_race(
        [
            _entry("judge-a", "prime-1", 0.9),
            _entry("judge-b", "shadow-1", 0.85),
        ]
    )
    winner, loser = decide_winner(race)
    assert winner is None
    assert loser is None


def test_a_panel_follows_its_one_deciding_entry():
    race = _panel_race(
        [
            _entry("judge-a", "shadow-1", 0.5),
            _entry("judge-b", "shadow-1", 0.9),
        ]
    )
    winner, loser = decide_winner(race)
    assert winner["model"] == "shadow-1"
    assert loser["model"] == "prime-1"


def test_a_constructed_panel_of_one_deciding_entry_decides():
    race = _panel_race([_entry("judge-a", "prime-1", 0.9)])
    winner, loser = decide_winner(race)
    assert winner["model"] == "prime-1"
    assert loser["model"] == "shadow-1"


def test_a_panel_no_entry_decides_decides_nothing():
    race = _panel_race(
        [_entry("judge-a", "prime-1", 0.5), _entry("judge-b", "shadow-1", 0.6)]
    )
    winner, loser = decide_winner(race)
    assert winner is None
    assert loser is None


def test_a_panel_entry_at_exactly_the_threshold_decides():
    race = _panel_race(
        [_entry("judge-a", "prime-1", 0.8), _entry("judge-b", "prime-1", 0.1)]
    )
    winner, _loser = decide_winner(race)
    assert winner["model"] == "prime-1"


def test_a_panel_entry_below_the_threshold_abstains():
    race = _panel_race(
        [
            _entry("judge-a", "prime-1", 0.75),
            _entry("judge-b", "prime-1", 0.75),
        ]
    )
    winner, _loser = decide_winner(race)
    assert winner is None


def test_a_panel_entry_with_integer_confidence_reads_by_its_figure():
    race = _panel_race(
        [_entry("judge-a", "prime-1", 1), _entry("judge-b", "prime-1", 0)]
    )
    winner, _loser = decide_winner(race)
    assert winner["model"] == "prime-1"


def test_a_panel_choice_names_the_first_attempt_carrying_the_label():
    data = {
        "task": "one model in both lanes",
        "attempts": [
            _attempt("prime-1", 300, environment="home worktree"),
            _attempt("prime-1", 288, environment="shadow worktree"),
        ],
        "panel": [_entry("judge-a", "prime-1", 0.9)],
    }
    race = _load(data)
    winner, loser = decide_winner(race)
    assert winner is race["attempts"][0]
    assert loser is race["attempts"][1]


def test_a_panel_outside_the_band_is_ignored_by_the_winner_rule():
    data = {
        "task": "a wide race",
        "attempts": [_attempt("prime-1", 300), _attempt("shadow-1", 200)],
        "panel": [
            _entry("judge-a", "prime-1", 0.99),
            _entry("judge-b", "prime-1", 0.99),
        ],
    }
    race = _load(data)
    winner, loser = decide_winner(race)
    assert winner["model"] == "shadow-1"
    assert loser["model"] == "prime-1"


# The panel rendering on the three surfaces


def test_the_confirmed_panel_lines_on_all_three_surfaces():
    race = _panel_race(
        [_entry("judge-a", "prime-1", 0.9), _entry("judge-b", "prime-1", 0.85)]
    )
    body = (
        "adjudication by judge-a chose prime-1 at confidence 0.9 and "
        "adjudication by judge-b chose prime-1 at confidence 0.85"
    )
    assert f"panel, {body}, the race is decided for prime-1" in (
        close_call_report(race)
    )
    assert f"close call, panel, {body}, the panel confirms prime-1" in (
        race_report(race)
    )
    assert f"decided by the panel, {body}" in funeral(race)


def test_the_constructed_divided_panel_lines_on_all_three_surfaces():
    race = _panel_race(
        [
            _entry("judge-a", "prime-1", 0.9),
            _entry("judge-b", "shadow-1", 0.85),
        ]
    )
    body = (
        "adjudication by judge-a chose prime-1 at confidence 0.9 and "
        "adjudication by judge-b chose shadow-1 at confidence 0.85"
    )
    assert f"panel, {body}, the panel stands divided" in close_call_report(
        race
    )
    assert f"close call, panel, {body}, the panel stands divided" in (
        race_report(race)
    )
    assert (
        f"close call, panel, {body}, the panel stands divided, "
        "the race stands as a dead heat"
    ) in funeral(race)


def test_the_follows_panel_lines_name_the_deciding_adjudicator():
    race = _panel_race(
        [
            _entry("judge-a", "shadow-1", 0.5),
            _entry("judge-b", "shadow-1", 0.9),
        ]
    )
    body = (
        "adjudication by judge-a abstained at confidence 0.5 and "
        "adjudication by judge-b chose shadow-1 at confidence 0.9"
    )
    assert f"panel, {body}, the race is decided for shadow-1" in (
        close_call_report(race)
    )
    assert f"close call, panel, {body}, the panel follows judge-b" in (
        race_report(race)
    )
    assert f"decided by the panel, {body}" in funeral(race)


def test_the_constructed_absent_judge_lines_name_the_absence():
    race = _panel_race([_entry("judge-a", "prime-1", 0.9)])
    body = (
        "adjudication by judge-a chose prime-1 at confidence 0.9 "
        "and the other judge is absent"
    )
    assert f"panel, {body}, the race is decided for prime-1" in (
        close_call_report(race)
    )
    assert f"close call, panel, {body}, the panel follows judge-a" in (
        race_report(race)
    )
    assert f"decided by the panel, {body}" in funeral(race)


def test_the_abstained_panel_lines_on_all_three_surfaces():
    race = _panel_race(
        [_entry("judge-a", "prime-1", 0.5), _entry("judge-b", "shadow-1", 0.6)]
    )
    body = (
        "adjudication by judge-a abstained at confidence 0.5 and "
        "adjudication by judge-b abstained at confidence 0.6"
    )
    assert f"panel, {body}, below the threshold of 0.8" in (
        close_call_report(race)
    )
    assert f"close call, panel, {body}" in race_report(race)
    assert f"close call, panel, {body}, the race stands as a dead heat" in (
        funeral(race)
    )


def test_the_constructed_absent_abstention_lines_name_both_cases():
    race = _panel_race([_entry("judge-a", "prime-1", 0.5)])
    body = (
        "adjudication by judge-a abstained at confidence 0.5 "
        "and the other judge is absent"
    )
    assert f"panel, {body}, below the threshold of 0.8" in (
        close_call_report(race)
    )
    assert (
        f"close call, panel, {body}, the race stands as a dead heat"
        in funeral(race)
    )


def test_an_abstaining_clause_never_shows_the_threshold_figure():
    race = _panel_race(
        [
            _entry("judge-a", "prime-1", 0.7999999),
            _entry("judge-b", "prime-1", 0.9),
        ]
    )
    line = close_call_report(race)[2]
    assert "abstained at confidence 0.8" not in line
    assert (
        "abstained at confidence "
        "0.799999899999999986022203302127309143543243408203125"
    ) in line
    assert line.endswith("the race is decided for prime-1")


def test_a_panel_outside_the_band_renders_the_not_needed_lines():
    data = {
        "task": "a wide race",
        "attempts": [_attempt("prime-1", 300), _attempt("shadow-1", 200)],
        "panel": [
            _entry("judge-a", "prime-1", 0.99),
            _entry("judge-b", "prime-1", 0.99),
        ],
    }
    race = _load(data)
    line = "panel not needed, the race stands outside the close call band"
    assert line in close_call_report(race)
    assert line in race_report(race)
    assert all("panel" not in rendered for rendered in funeral(race))


def test_every_panel_line_is_one_line():
    panels = [
        [_entry("judge-a", "prime-1", 0.9), _entry("judge-b", "prime-1", 0.85)],
        [_entry("judge-a", "prime-1", 0.9), _entry("judge-b", "shadow-1", 0.85)],
        [_entry("judge-a", "prime-1", 0.5), _entry("judge-b", "shadow-1", 0.6)],
        [_entry("judge-a", "prime-1", 0.9)],
        [_entry("judge-a", "prime-1", 0.5)],
    ]
    for panel in panels:
        race = _panel_race(panel)
        for lines in (
            close_call_report(race),
            race_report(race),
            funeral(race),
        ):
            for line in lines:
                assert "\n" not in line
                assert "\r" not in line


# The join


def _record(task, attempts, entry=None):
    race = {"task": task, "attempts": attempts}
    if entry is not None:
        race["adjudication"] = entry
    loaded, defects = load_race_data(race)
    assert defects == []
    return loaded


def _join_attempts():
    return [_attempt("prime-1", 300), _attempt("shadow-1", 288)]


def test_a_clean_join_holds_both_entries_in_the_order_given():
    first = _record(
        "one race", _join_attempts(), _entry("judge-a", "prime-1", 0.9)
    )
    second = _record(
        "one race", _join_attempts(), _entry("judge-b", "prime-1", 0.85)
    )
    joined, defects = join_panel_records(first, second)
    assert defects == []
    assert joined["task"] == "one race"
    assert joined["attempts"] == first["attempts"]
    assert joined["panel"] == [
        _entry("judge-a", "prime-1", 0.9),
        _entry("judge-b", "prime-1", 0.85),
    ]


def test_a_joined_record_loads_under_dh9_with_no_defects():
    first = _record(
        "one race", _join_attempts(), _entry("judge-a", "prime-1", 0.9)
    )
    second = _record(
        "one race", _join_attempts(), _entry("judge-b", "prime-1", 0.85)
    )
    joined, defects = join_panel_records(first, second)
    assert defects == []
    loaded, load_defects = load_race_data(
        json.loads(json.dumps(joined))
    )
    assert load_defects == []
    assert loaded == joined


def test_a_join_of_records_naming_different_tasks_is_refused():
    first = _record(
        "one race", _join_attempts(), _entry("judge-a", "prime-1", 0.9)
    )
    second = _record(
        "another race", _join_attempts(), _entry("judge-b", "prime-1", 0.9)
    )
    joined, defects = join_panel_records(first, second)
    assert joined is None
    assert defects == ["the two records name different tasks"]


def test_a_join_of_records_holding_different_attempts_is_refused():
    changed = _join_attempts()
    changed[1] = _attempt("shadow-1", 290)
    first = _record(
        "one race", _join_attempts(), _entry("judge-a", "prime-1", 0.9)
    )
    second = _record(
        "one race", changed, _entry("judge-b", "prime-1", 0.9)
    )
    joined, defects = join_panel_records(first, second)
    assert joined is None
    assert defects == ["the two records hold different attempts"]


def test_a_join_of_records_with_swapped_attempts_is_refused():
    swapped = list(reversed(_join_attempts()))
    first = _record(
        "one race", _join_attempts(), _entry("judge-a", "prime-1", 0.9)
    )
    second = _record(
        "one race", swapped, _entry("judge-b", "prime-1", 0.9)
    )
    joined, defects = join_panel_records(first, second)
    assert joined is None
    assert defects == ["the two records hold different attempts"]


def test_a_join_holding_no_entry_between_the_pair_is_refused():
    first = _record("one race", _join_attempts())
    second = _record("one race", _join_attempts())
    joined, defects = join_panel_records(first, second)
    assert joined is None
    assert defects == [
        "the two records hold no adjudication entry between them"
    ]


def test_a_join_refuses_a_first_record_carrying_a_panel():
    panel_record = _panel_race(
        [_entry("judge-a", "prime-1", 0.9), _entry("judge-b", "prime-1", 0.85)]
    )
    second = _record(
        "tighten the retry budget",
        _join_attempts(),
        _entry("judge-c", "prime-1", 0.9),
    )
    joined, defects = join_panel_records(panel_record, second)
    assert joined is None
    assert defects == ["one of the two records carries a panel"]


def test_a_join_refuses_a_second_record_carrying_a_panel():
    first = _record(
        "tighten the retry budget",
        _join_attempts(),
        _entry("judge-c", "prime-1", 0.9),
    )
    panel_record = _panel_race(
        [_entry("judge-a", "prime-1", 0.9), _entry("judge-b", "prime-1", 0.85)]
    )
    joined, defects = join_panel_records(first, panel_record)
    assert joined is None
    assert defects == ["one of the two records carries a panel"]


def test_a_join_of_two_panel_records_names_the_panel_once():
    first = _panel_race(
        [_entry("judge-a", "prime-1", 0.9), _entry("judge-b", "prime-1", 0.85)]
    )
    second = _panel_race(
        [_entry("judge-c", "prime-1", 0.9), _entry("judge-d", "prime-1", 0.85)]
    )
    joined, defects = join_panel_records(first, second)
    assert joined is None
    assert defects == ["one of the two records carries a panel"]


def test_a_join_of_a_panel_and_an_entry_less_record_names_only_the_panel():
    panel_record = _panel_race(
        [_entry("judge-a", "prime-1", 0.9), _entry("judge-b", "prime-1", 0.85)]
    )
    plain = _record("tighten the retry budget", _join_attempts())
    joined, defects = join_panel_records(panel_record, plain)
    assert joined is None
    assert defects == ["one of the two records carries a panel"]


def test_a_join_of_entries_from_one_adjudicator_is_refused():
    first = _record(
        "one race", _join_attempts(), _entry("judge-a", "prime-1", 0.9)
    )
    second = _record(
        "one race", _join_attempts(), _entry("judge-a", "shadow-1", 0.9)
    )
    joined, defects = join_panel_records(first, second)
    assert joined is None
    assert defects == ["the two entries name the same adjudicator"]


def test_a_constructed_join_with_one_entry_holds_the_absent_case():
    first = _record(
        "one race", _join_attempts(), _entry("judge-a", "prime-1", 0.9)
    )
    second = _record("one race", _join_attempts())
    joined, defects = join_panel_records(first, second)
    assert defects == []
    assert joined["panel"] == [_entry("judge-a", "prime-1", 0.9)]
    winner, _loser = decide_winner(joined)
    assert winner["model"] == "prime-1"
    body = (
        "adjudication by judge-a chose prime-1 at confidence 0.9 "
        "and the other judge is absent"
    )
    assert f"panel, {body}, the race is decided for prime-1" in (
        close_call_report(joined)
    )


def test_a_constructed_join_of_two_deciding_records_can_stand_divided():
    first = _record(
        "one race", _join_attempts(), _entry("judge-a", "prime-1", 0.9)
    )
    second = _record(
        "one race", _join_attempts(), _entry("judge-b", "shadow-1", 0.9)
    )
    joined, defects = join_panel_records(first, second)
    assert defects == []
    winner, _loser = decide_winner(joined)
    assert winner is None
    assert (
        "the panel stands divided" in close_call_report(joined)[2]
    )


def test_the_join_never_mutates_its_inputs():
    first = _record(
        "one race", _join_attempts(), _entry("judge-a", "prime-1", 0.9)
    )
    second = _record(
        "one race", _join_attempts(), _entry("judge-b", "prime-1", 0.85)
    )
    first_before = copy.deepcopy(first)
    second_before = copy.deepcopy(second)
    join_panel_records(first, second)
    assert first == first_before
    assert second == second_before


# Series and standings counting of panel races


def test_a_hand_counted_series_of_panel_races():
    def panel_series_race(panel):
        race = _panel_race(panel)
        return race

    races = [
        panel_series_race(
            [
                _entry("judge-a", "prime-1", 0.9),
                _entry("judge-b", "prime-1", 0.85),
            ]
        ),
        panel_series_race(
            [
                _entry("judge-a", "prime-1", 0.9),
                _entry("judge-b", "shadow-1", 0.85),
            ]
        ),
        panel_series_race(
            [
                _entry("judge-a", "shadow-1", 0.5),
                _entry("judge-b", "shadow-1", 0.9),
            ]
        ),
        panel_series_race(
            [
                _entry("judge-a", "prime-1", 0.5),
                _entry("judge-b", "shadow-1", 0.6),
            ]
        ),
        panel_series_race(
            [
                _entry("judge-a", "prime-1", 0.95),
                _entry("judge-b", "prime-1", 0.9),
            ]
        ),
    ]
    series_data = {
        "task": "panel counting",
        "models": ["prime-1", "shadow-1"],
        "wins_target": 2,
        "races": races,
    }
    series, defects = load_series_text(json.dumps(series_data))
    assert defects == []
    assert series_report(series) == [
        "series panel counting",
        "prime-1 against shadow-1, first to 2 wins",
        "race 1, tighten the retry budget, prime-1 wins",
        "tally, prime-1 1 wins, shadow-1 0 wins, 0 no results",
        "race 2, tighten the retry budget, no result",
        "tally, prime-1 1 wins, shadow-1 0 wins, 1 no results",
        "race 3, tighten the retry budget, shadow-1 wins",
        "tally, prime-1 1 wins, shadow-1 1 wins, 1 no results",
        "race 4, tighten the retry budget, no result",
        "tally, prime-1 1 wins, shadow-1 1 wins, 2 no results",
        "race 5, tighten the retry budget, prime-1 wins",
        "tally, prime-1 2 wins, shadow-1 1 wins, 2 no results",
        "series winner, prime-1",
    ]


def test_the_standings_count_panel_races_as_the_winner_rule_decides():
    confirmed = _panel_race(
        [_entry("judge-a", "prime-1", 0.9), _entry("judge-b", "prime-1", 0.85)]
    )
    standings = update_standings(empty_standings(), confirmed)
    assert standings["models"]["prime-1"]["wins"] == 1
    assert standings["models"]["shadow-1"]["losses"] == 1
    divided = _panel_race(
        [
            _entry("judge-a", "prime-1", 0.9),
            _entry("judge-b", "shadow-1", 0.85),
        ]
    )
    standings = update_standings(standings, divided)
    assert standings["models"]["prime-1"]["no_results"] == 1
    assert standings["models"]["shadow-1"]["no_results"] == 1
    assert standings["models"]["prime-1"]["wins"] == 1


# The recorded panel pilot


def test_every_panel_record_is_the_join_of_its_pilot_records():
    for name in RACE_NAMES:
        loaded = []
        for route in ROUTES:
            record, defects = load_race_file(
                PILOT_RECORDS / f"{name}-{route}.json"
            )
            assert defects == [], (name, route)
            loaded.append(record)
        joined, defects = join_panel_records(loaded[0], loaded[1])
        assert defects == [], name
        committed, committed_defects = load_race_file(
            PANEL / "records" / f"{name}.json"
        )
        assert committed_defects == [], name
        assert committed == joined, name
        assert committed["panel"][0]["adjudicator"] == "jev-1.13.0", name


def test_the_recorded_panel_census_recomputed_from_the_figures():
    counts = {"confirmed": 0, "divided": 0, "follows": 0, "abstained": 0}
    for name in RACE_NAMES:
        record, defects = load_race_file(PANEL / "records" / f"{name}.json")
        assert defects == [], name
        panel = record["panel"]
        assert len(panel) == 2, name
        deciders = [
            entry
            for entry in panel
            if Fraction(entry["confidence"]) >= Fraction(4, 5)
        ]
        if len(deciders) == 2:
            if deciders[0]["chosen"] == deciders[1]["chosen"]:
                counts["confirmed"] += 1
            else:
                counts["divided"] += 1
        elif len(deciders) == 1:
            counts["follows"] += 1
        else:
            counts["abstained"] += 1
    assert counts == {
        "confirmed": 3,
        "divided": 0,
        "follows": 3,
        "abstained": 4,
    }


def test_the_panel_maker_regenerates_the_panel_byte_for_byte():
    watched = sorted(
        list((PANEL / "records").glob("*"))
        + [EXAMPLES / "panel-series.json", EXAMPLES / "panel-series.txt"]
    )

    def digest():
        return {
            str(path): hashlib.sha256(path.read_bytes()).hexdigest()
            for path in watched
        }

    before = digest()
    runpy.run_path(str(PANEL / "make_panel.py"), run_name="__main__")
    assert digest() == before
