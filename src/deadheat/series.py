"""Series over recorded races, DH-6 in CONTRACT.md.

A series file holds recorded races between the same two models
and a wins target. The loader names every defect it finds and
never raises on malformed content, and the series report is a
pure function of the loaded series.
"""

import json
import pathlib

from deadheat.races import (
    _check_text_field,
    _holds_line_break,
    _holds_unpaired_surrogate,
    _is_int,
    decide_winner,
    load_race_data,
)


def _load_models(data, defects):
    if "models" not in data:
        defects.append("missing field models")
        return None
    raw = data["models"]
    if not isinstance(raw, list):
        defects.append("field models has the wrong type")
        return None
    before = len(defects)
    if len(raw) != 2:
        defects.append("field models must hold exactly two models")
    clean = []
    for index, entry in enumerate(raw):
        if not isinstance(entry, str):
            defects.append(f"models entry {index} has the wrong type")
        elif entry == "":
            defects.append(f"models entry {index} is empty")
        elif _holds_unpaired_surrogate(entry):
            defects.append(
                f"models entry {index} holds an unpaired surrogate"
            )
        elif _holds_line_break(entry):
            defects.append(f"models entry {index} holds a newline")
        else:
            clean.append(entry)
    if len(clean) == 2 and clean[0] == clean[1]:
        defects.append("field models names the same model twice")
    if len(defects) != before:
        return None
    return clean


def _load_wins_target(data, defects):
    if "wins_target" not in data:
        defects.append("missing field wins_target")
        return None
    value = data["wins_target"]
    if not _is_int(value):
        defects.append("field wins_target has the wrong type")
        return None
    if value < 1:
        defects.append("field wins_target is not positive")
        return None
    return value


def _load_series_races(data, models, defects):
    if "races" not in data:
        defects.append("missing field races")
        return None
    raw = data["races"]
    if not isinstance(raw, list):
        defects.append("field races has the wrong type")
        return None
    before = len(defects)
    clean = []
    for index, entry in enumerate(raw):
        if not isinstance(entry, dict):
            defects.append(f"race {index} is not an object")
            continue
        race, race_defects = load_race_data(entry)
        for defect in race_defects:
            defects.append(f"race {index} {defect}")
        if race is None:
            continue
        if models is not None:
            named = [attempt["model"] for attempt in race["attempts"]]
            if any(model not in models for model in named):
                defects.append(
                    f"race {index} names a model outside the series"
                )
                continue
            if named[0] == named[1]:
                defects.append(
                    f"race {index} names the same model twice"
                )
                continue
        clean.append(race)
    if len(defects) != before:
        return None
    return clean


def load_series_text(text):
    """Parse series text into a series, naming every defect found.

    Returns a pair of the series and the defect list. A malformed
    input returns no series, and this function never raises on bad
    content.
    """
    try:
        data = json.loads(text)
    except (ValueError, RecursionError):
        return None, ["input is not valid JSON"]
    if not isinstance(data, dict):
        return None, ["input is not a JSON object"]
    defects = []
    task = _check_text_field(data, "task", defects)
    models = _load_models(data, defects)
    wins_target = _load_wins_target(data, defects)
    races = _load_series_races(data, models, defects)
    if defects:
        return None, defects
    return {
        "task": task,
        "models": models,
        "wins_target": wins_target,
        "races": races,
    }, []


def load_series_file(path):
    """Read a series from disk, with the same contract as text."""
    try:
        text = pathlib.Path(path).read_text(encoding="utf-8")
    except (OSError, UnicodeError):
        return None, ["file cannot be read"]
    return load_series_text(text)


def series_report(series):
    """Return the series report lines for one series.

    The first line names the task family and the second names the
    two models and the wins target. Each race then renders in
    record order with its outcome and the running tally, until the
    race where a model first reaches the target decides the
    series. A race recorded after the deciding race renders as
    beyond the decision and changes nothing. The closing line
    names the series winner or states that the series stands
    unfinished. The report is a pure function of the series.
    """
    first_model, second_model = series["models"]
    target = series["wins_target"]
    lines = [
        f"series {series['task']}",
        f"{first_model} against {second_model}, first to {target} wins",
    ]
    wins = {first_model: 0, second_model: 0}
    no_results = 0
    champion = None
    for index, race in enumerate(series["races"], start=1):
        if champion is not None:
            lines.append(f"race {index}, {race['task']}, beyond the decision")
            continue
        winner, _loser = decide_winner(race)
        if winner is None:
            no_results += 1
            lines.append(f"race {index}, {race['task']}, no result")
        else:
            wins[winner["model"]] += 1
            lines.append(
                f"race {index}, {race['task']}, {winner['model']} wins"
            )
        lines.append(
            f"tally, {first_model} {wins[first_model]} wins, "
            f"{second_model} {wins[second_model]} wins, "
            f"{no_results} no results"
        )
        if winner is not None and wins[winner["model"]] >= target:
            champion = winner["model"]
    if champion is not None:
        lines.append(f"series winner, {champion}")
    else:
        lines.append(
            f"series unfinished, the target of {target} wins "
            "is not reached"
        )
    return lines
