"""Pins for the adjudication seat and its envelope readers.

Every pin here runs offline over synthetic envelopes built in the
test. The summary and prompt pins hold the exact stated forms
from DH-7, the reader pins walk the failure enumeration in DH-7,
and the recording pins hold the entry path into the race record
and its reading under DH-5.
"""

import copy
import json

import pytest

from deadheat.races import decide_winner, load_race_data
from deadheat.seats import (
    adjudicated_race,
    attempt_summary,
    chat_prompt,
    entry_from_chat_envelope,
    entry_from_jev_envelope,
    has_seat,
    record_adjudication,
)

PATCH_A = "--- a/retry.py\n+++ b/retry.py\n@@ -1,1 +1,1 @@\n-budget = 5\n+budget = 3\n"
PATCH_B = "--- a/retry.py\n+++ b/retry.py\n@@ -1,1 +1,1 @@\n-budget = 5\n+budget = 2\n"


def _race():
    return {
        "task": "tighten the retry budget",
        "attempts": [
            {
                "model": "prime-1",
                "environment": "home worktree",
                "result": "green",
                "finished_at": 300.0,
                "tokens": 88100,
                "patch": PATCH_A,
            },
            {
                "model": "shadow-1",
                "environment": "shadow worktree",
                "result": "green",
                "finished_at": 288.0,
                "tokens": 91400,
                "patch": PATCH_B,
            },
        ],
    }


def _jev_envelope(choice="shadow-1", confidence=0.85):
    return {
        "model": "jev-1.13.0",
        "answers": {
            "decision": {
                "type": "choice",
                "choice": choice,
                "confidence": confidence,
                "probabilities": {"prime-1": 0.15, "shadow-1": 0.85},
            }
        },
        "usage": {"input_tokens": 437, "output_tokens": 37},
    }


def _chat_envelope(choice="prime-1", confidence=0.9):
    decision = {
        "choice": choice,
        "confidence": confidence,
        "probabilities": {"prime-1": 0.9, "shadow-1": 0.1},
    }
    return {
        "model": "openai/gpt-6-luna",
        "choices": [
            {"message": {"role": "assistant", "content": json.dumps(decision)}}
        ],
        "usage": {"prompt_tokens": 227, "completion_tokens": 101},
    }


def test_the_summary_states_the_stated_form():
    expected = (
        "task tighten the retry budget\n"
        "attempt, model prime-1, environment home worktree, "
        "result green, finished in 300 seconds, 88100 tokens\n"
        f"patch of prime-1\n{PATCH_A}\n"
        "attempt, model shadow-1, environment shadow worktree, "
        "result green, finished in 288 seconds, 91400 tokens\n"
        f"patch of shadow-1\n{PATCH_B}"
    )
    assert attempt_summary(_race()) == expected


def test_the_summary_renders_fractional_seconds_exactly():
    race = _race()
    race["attempts"][0]["finished_at"] = 505.25
    race["attempts"][1]["finished_at"] = 398.5
    summary = attempt_summary(race)
    assert "finished in 505.25 seconds" in summary
    assert "finished in 398.5 seconds" in summary


def test_the_summary_states_an_unfinished_attempt():
    race = _race()
    race["attempts"][1] = {
        "model": "shadow-1",
        "environment": "shadow worktree",
        "result": "unfinished",
        "tokens": None,
        "patch": PATCH_B,
    }
    summary = attempt_summary(race)
    assert (
        "attempt, model shadow-1, environment shadow worktree, "
        "result unfinished, never finished, tokens not reported"
    ) in summary


def test_the_summary_is_a_pure_function_of_the_race():
    race = _race()
    snapshot = copy.deepcopy(race)
    assert attempt_summary(race) == attempt_summary(race)
    assert race == snapshot


def test_the_chat_prompt_wraps_the_summary_as_stated():
    race = _race()
    prompt = chat_prompt(race)
    expected = (
        "Adjudicate the recorded close call race below as one "
        "judge, choosing exactly one of the two attempts as the "
        "winner.\n\n"
        + attempt_summary(race)
        + "\n\nReply with one JSON object and nothing else. The "
        "object holds the field choice, exactly one of the two "
        "model labels, the field confidence, your confidence in "
        "the choice as a number from 0 to 1, and the field "
        "probabilities, an object holding one number for each "
        "of the two model labels."
    )
    assert prompt == expected


def test_a_race_with_two_model_labels_has_a_seat():
    assert has_seat(_race()) is True


def test_a_race_with_one_model_label_has_no_seat():
    race = _race()
    race["attempts"][1]["model"] = "prime-1"
    assert has_seat(race) is False


def test_the_jev_reader_builds_the_entry_by_the_stated_construction():
    entry = entry_from_jev_envelope(_race(), _jev_envelope())
    assert entry == {
        "adjudicator": "jev-1.13.0",
        "chosen": "shadow-1",
        "confidence": 0.85,
        "reason": "jev-1.13.0 chose shadow-1 with confidence 0.85",
    }


def test_the_jev_reader_renders_an_integer_confidence_plainly():
    entry = entry_from_jev_envelope(_race(), _jev_envelope(confidence=1))
    assert entry["confidence"] == 1
    assert entry["reason"].endswith("with confidence 1")


def test_the_reader_records_a_below_threshold_entry():
    entry = entry_from_jev_envelope(_race(), _jev_envelope(confidence=0.3))
    assert entry is not None
    assert entry["confidence"] == 0.3


def test_the_reader_reads_probabilities_for_shape_only():
    envelope = _jev_envelope(confidence=0.85)
    envelope["answers"]["decision"]["probabilities"] = {
        "prime-1": 0.99,
        "shadow-1": 0.01,
        "a third model": 0.5,
    }
    entry = entry_from_jev_envelope(_race(), envelope)
    assert entry is not None
    assert entry["chosen"] == "shadow-1"
    assert "probabilities" not in entry


@pytest.mark.parametrize(
    "mutate",
    [
        lambda e: e.update(model=None),
        lambda e: e.update(model=""),
        lambda e: e.update(model=7),
        lambda e: e.update(model="jev\n1.13.0"),
        lambda e: e.update(model="jev\ud800"),
        lambda e: e.pop("answers"),
        lambda e: e.update(answers=[]),
        lambda e: e["answers"].pop("decision"),
        lambda e: e["answers"].update(decision="shadow-1"),
        lambda e: e["answers"]["decision"].pop("choice"),
        lambda e: e["answers"]["decision"].update(choice="third-1"),
        lambda e: e["answers"]["decision"].update(choice="prime\n1"),
        lambda e: e["answers"]["decision"].update(choice="prime\ud8001"),
        lambda e: e["answers"]["decision"].pop("confidence"),
        lambda e: e["answers"]["decision"].update(confidence=True),
        lambda e: e["answers"]["decision"].update(confidence="0.9"),
        lambda e: e["answers"]["decision"].update(confidence=1.5),
        lambda e: e["answers"]["decision"].update(confidence=-0.1),
        lambda e: e["answers"]["decision"].pop("probabilities"),
        lambda e: e["answers"]["decision"].update(probabilities=[]),
        lambda e: e["answers"]["decision"].update(
            probabilities={"prime-1": 0.5}
        ),
        lambda e: e["answers"]["decision"].update(
            probabilities={"prime-1": 0.5, "shadow-1": "high"}
        ),
    ],
)
def test_the_jev_reader_fails_the_call_on_an_unreadable_envelope(mutate):
    envelope = _jev_envelope()
    mutate(envelope)
    assert entry_from_jev_envelope(_race(), envelope) is None


def test_the_jev_reader_fails_a_non_object_envelope():
    assert entry_from_jev_envelope(_race(), ["not", "an", "object"]) is None
    assert entry_from_jev_envelope(_race(), None) is None


@pytest.mark.parametrize(
    "figure", [float("nan"), float("inf"), float("-inf")]
)
def test_the_jev_reader_fails_a_non_finite_figure(figure):
    envelope = _jev_envelope(confidence=figure)
    envelope = json.loads(json.dumps(envelope))
    assert entry_from_jev_envelope(_race(), envelope) is None
    envelope = _jev_envelope()
    envelope["answers"]["decision"]["probabilities"]["prime-1"] = figure
    envelope = json.loads(json.dumps(envelope))
    assert entry_from_jev_envelope(_race(), envelope) is None


def test_the_chat_reader_builds_the_entry():
    entry = entry_from_chat_envelope(_race(), _chat_envelope())
    assert entry == {
        "adjudicator": "openai/gpt-6-luna",
        "chosen": "prime-1",
        "confidence": 0.9,
        "reason": "openai/gpt-6-luna chose prime-1 with confidence 0.9",
    }


def test_the_chat_reader_accepts_a_choice_either_way():
    entry = entry_from_chat_envelope(
        _race(), _chat_envelope(choice="shadow-1", confidence=0.5)
    )
    assert entry["chosen"] == "shadow-1"


@pytest.mark.parametrize(
    "content",
    [
        "I choose prime-1",
        '["prime-1"]',
        '"prime-1"',
        '{"choice": "third-1", "confidence": 0.9, '
        '"probabilities": {"prime-1": 0.5, "shadow-1": 0.5}}',
        '{"choice": "prime-1", "confidence": 2, '
        '"probabilities": {"prime-1": 0.5, "shadow-1": 0.5}}',
    ],
)
def test_the_chat_reader_fails_an_unreadable_reply(content):
    envelope = _chat_envelope()
    envelope["choices"][0]["message"]["content"] = content
    assert entry_from_chat_envelope(_race(), envelope) is None


@pytest.mark.parametrize(
    "mutate",
    [
        lambda e: e.update(choices=[]),
        lambda e: e.pop("choices"),
        lambda e: e.update(choices=[{}]),
        lambda e: e["choices"][0].update(message=None),
        lambda e: e["choices"][0]["message"].update(content={"choice": "x"}),
        lambda e: e.update(model=""),
    ],
)
def test_the_chat_reader_fails_a_malformed_envelope(mutate):
    envelope = _chat_envelope()
    mutate(envelope)
    assert entry_from_chat_envelope(_race(), envelope) is None


def test_recording_an_entry_never_mutates_the_race():
    race = _race()
    entry = entry_from_jev_envelope(race, _jev_envelope())
    recorded = record_adjudication(race, entry)
    assert "adjudication" not in race
    assert recorded["adjudication"] == entry
    loaded, defects = load_race_data(recorded)
    assert defects == []
    assert loaded == recorded


def test_a_recorded_deciding_entry_decides_under_dh_5():
    race = _race()
    entry = entry_from_jev_envelope(race, _jev_envelope(confidence=0.85))
    recorded = record_adjudication(race, entry)
    winner, _loser = decide_winner(recorded)
    assert winner["model"] == "shadow-1"


def test_a_recorded_low_entry_abstains_under_dh_5():
    race = _race()
    entry = entry_from_jev_envelope(race, _jev_envelope(confidence=0.3))
    recorded = record_adjudication(race, entry)
    winner, _loser = decide_winner(recorded)
    assert winner is None


def test_adjudicated_race_records_through_either_route():
    race = _race()
    by_jev = adjudicated_race(race, _jev_envelope(), "jev")
    assert by_jev["adjudication"]["adjudicator"] == "jev-1.13.0"
    by_chat = adjudicated_race(race, _chat_envelope(), "openai")
    assert by_chat["adjudication"]["adjudicator"] == "openai/gpt-6-luna"


def test_adjudicated_race_returns_none_for_a_failed_call():
    envelope = _jev_envelope()
    envelope["answers"]["decision"]["confidence"] = 7
    assert adjudicated_race(_race(), envelope, "jev") is None


def test_adjudicated_race_rejects_an_unknown_route_word():
    with pytest.raises(ValueError):
        adjudicated_race(_race(), _jev_envelope(), "carrier-pigeon")


def test_the_seat_number_test_is_the_races_number_test():
    from deadheat import races, seats

    assert seats._is_number is races._is_number
