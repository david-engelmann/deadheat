"""The adjudication seat over decision models, and its readers.

The seat presents one recorded close call to a decision model in
one stated summary form, reads the envelope the call returns into
the adjudication entry DH-5 validates, and records that entry
into the race as data. The readers are pure functions of their
inputs and make no call themselves, and the live slice and the
suite share them, so a recorded envelope replays through the same
reading the live run used. The obligations this module carries
are DH-7 and DH-8 in CONTRACT.md, in the Round 3 section.
"""

import json

from deadheat.races import (
    _holds_unpaired_surrogate,
    _is_finite_number,
    _is_number,
    format_seconds,
)


def _holds_line_break(text):
    """Return whether a string holds a newline or a carriage return."""
    return "\n" in text or "\r" in text


def has_seat(race):
    """Return whether a race can be seated under DH-7.

    The seat presents the two attempts under their model labels,
    so a race whose attempts carry the same model label has no
    seat.
    """
    first, second = race["attempts"]
    return first["model"] != second["model"]


def attempt_summary(race):
    """Return the summary text the seat presents for one race.

    The form is the one DH-7 states. The first line names the
    task. Each attempt then stands in record order with its line
    and its patch, the patch verbatim after the line naming it.
    The parts are joined with single newlines, and the same race
    always builds the same summary.
    """
    parts = [f"task {race['task']}"]
    for attempt in race["attempts"]:
        if attempt["result"] == "unfinished":
            finish = "never finished"
        else:
            finish = (
                "finished in "
                f"{format_seconds(attempt['finished_at'])} seconds"
            )
        if attempt["tokens"] is None:
            spend = "tokens not reported"
        else:
            spend = f"{attempt['tokens']} tokens"
        parts.append(
            f"attempt, model {attempt['model']}, "
            f"environment {attempt['environment']}, "
            f"result {attempt['result']}, {finish}, {spend}"
        )
        parts.append(f"patch of {attempt['model']}")
        parts.append(attempt["patch"])
    return "\n".join(parts)


_INSTRUCTION_LINE = (
    "Adjudicate the recorded close call race below as one judge, "
    "choosing exactly one of the two attempts as the winner."
)

_REPLY_LINE = (
    "Reply with one JSON object and nothing else. The object "
    "holds the field choice, exactly one of the two model "
    "labels, the field confidence, your confidence in the "
    "choice as a number from 0 to 1, and the field "
    "probabilities, an object holding one number for each of "
    "the two model labels."
)


def chat_prompt(race):
    """Return the prompt the chat route sends for one race.

    The prompt is the summary standing between the instruction
    line and the reply line, each parted from the summary by a
    blank line, as DH-7 states.
    """
    return (
        f"{_INSTRUCTION_LINE}\n\n"
        f"{attempt_summary(race)}\n\n"
        f"{_REPLY_LINE}"
    )


def _render_confidence(confidence):
    """Render a confidence figure as its shortest round trip text.

    An integer confidence renders with no decimal point, and a
    fractional one renders as the shortest decimal text that
    parses back to the same value.
    """
    if isinstance(confidence, int):
        return str(confidence)
    return repr(confidence)


def _read_model(envelope):
    """Return the envelope model string, or None when unreadable."""
    if not isinstance(envelope, dict):
        return None
    model = envelope.get("model")
    if not isinstance(model, str) or model == "":
        return None
    if _holds_unpaired_surrogate(model):
        return None
    if _holds_line_break(model):
        return None
    return model


def _entry_from_decision(race, model, decision):
    """Build the adjudication entry from one decision, or None.

    The decision must hold a choice naming one attempt of the
    race, a finite confidence inside the range from zero to one,
    and probabilities holding a finite number for each of the two
    model labels, with every probability a finite number. The
    probabilities are read for shape only and never enter the
    entry. A choice or a model string holding a line break fails
    the call, so the reason the entry carries is one line by
    construction. Anything else is a call the reader cannot read,
    and the seat records nothing.
    """
    if not isinstance(decision, dict):
        return None
    labels = [attempt["model"] for attempt in race["attempts"]]
    choice = decision.get("choice")
    if not isinstance(choice, str):
        return None
    if _holds_unpaired_surrogate(choice):
        return None
    if _holds_line_break(choice):
        return None
    if choice not in labels:
        return None
    confidence = decision.get("confidence")
    if not _is_number(confidence):
        return None
    if not _is_finite_number(confidence):
        return None
    if confidence < 0 or confidence > 1:
        return None
    probabilities = decision.get("probabilities")
    if not isinstance(probabilities, dict):
        return None
    for value in probabilities.values():
        if not _is_number(value) or not _is_finite_number(value):
            return None
    for label in labels:
        if label not in probabilities:
            return None
    reason = (
        f"{model} chose {choice} with confidence "
        f"{_render_confidence(confidence)}"
    )
    return {
        "adjudicator": model,
        "chosen": choice,
        "confidence": confidence,
        "reason": reason,
    }


def entry_from_jev_envelope(race, envelope):
    """Return the entry from one Jev envelope, or None.

    The envelope is the Typesafe System One response as parsed
    JSON, holding the model string and an answers object whose
    decision object holds the decision.
    """
    model = _read_model(envelope)
    if model is None:
        return None
    answers = envelope.get("answers")
    if not isinstance(answers, dict):
        return None
    return _entry_from_decision(race, model, answers.get("decision"))


def entry_from_chat_envelope(race, envelope):
    """Return the entry from one chat envelope, or None.

    The envelope is an OpenAI compatible chat completion as
    parsed JSON, holding the model string and a choices list
    whose first entry holds a message whose content is one JSON
    object holding the decision. Content that does not parse as
    one JSON object is a failed call.
    """
    model = _read_model(envelope)
    if model is None:
        return None
    choices = envelope.get("choices")
    if not isinstance(choices, list) or not choices:
        return None
    first = choices[0]
    if not isinstance(first, dict):
        return None
    message = first.get("message")
    if not isinstance(message, dict):
        return None
    content = message.get("content")
    if not isinstance(content, str):
        return None
    try:
        decision = json.loads(content)
    except (ValueError, RecursionError):
        return None
    return _entry_from_decision(race, model, decision)


_READERS = {
    "jev": entry_from_jev_envelope,
    "openai": entry_from_chat_envelope,
}


def record_adjudication(race, entry):
    """Return a new race holding the entry under adjudication.

    The input race is never mutated, and the returned race is the
    input with the entry recorded as data under DH-7.
    """
    recorded = dict(race)
    recorded["adjudication"] = entry
    return recorded


def adjudicated_race(race, envelope, route):
    """Return the race with the envelope entry recorded, or None.

    The route word is jev or openai, naming the envelope shape.
    None stands for a failed call, which records no entry. An
    unknown route word is a programming error and raises
    ValueError, because it is not malformed content.
    """
    if route not in _READERS:
        raise ValueError(f"unknown adjudication route {route!r}")
    entry = _READERS[route](race, envelope)
    if entry is None:
        return None
    return record_adjudication(race, entry)
