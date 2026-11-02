"""Recorded races, the winner rule, the funeral, and the standings.

Every function in this module is a pure function of its inputs, and
the loaders never raise on malformed content. The obligations this
module carries are DH-1 to DH-4 in CONTRACT.md, in the full form
section.
"""

import json
import math
import pathlib
import re

_RESULTS = ("green", "red", "unfinished")
_STANDINGS_FIELDS = ("wins", "losses", "no_results", "tokens", "tokens_unreported")
_SLUG_RUNS = re.compile(r"[^a-z0-9]+")


def _is_number(value):
    return isinstance(value, (int, float)) and not isinstance(value, bool)


def _is_int(value):
    return isinstance(value, int) and not isinstance(value, bool)


def _is_finite_number(value):
    """Return whether a number is finite, without ever raising.

    An int too large to convert to float counts as not finite, so
    the loader names it with the finite number defect instead of
    raising OverflowError on it.
    """
    try:
        return math.isfinite(value)
    except OverflowError:
        return False


def _holds_unpaired_surrogate(text):
    """Return whether a string fails UTF-8 encoding.

    A string holding an unpaired surrogate cannot be encoded, and
    printed output carrying one cannot be printed, so the loader
    names it as a defect instead of letting it reach the printer.
    """
    try:
        text.encode("utf-8")
    except UnicodeEncodeError:
        return True
    return False


def format_seconds(value):
    """Render a number of seconds the same way everywhere it appears.

    Values render exactly at up to six decimal places. A nonzero
    value that rounds to zero at that precision renders at full
    precision instead, so a margin never prints as zero.
    """
    number = round(float(value), 6)
    if number == 0 and float(value) != 0:
        return repr(float(value))
    if number == int(number):
        return str(int(number))
    return f"{number:.6f}".rstrip("0").rstrip(".")


def _check_text_field(data, name, defects):
    if name not in data:
        defects.append(f"missing field {name}")
        return None
    value = data[name]
    if not isinstance(value, str):
        defects.append(f"field {name} has the wrong type")
        return None
    if value == "":
        defects.append(f"field {name} is empty")
        return None
    if _holds_unpaired_surrogate(value):
        defects.append(f"field {name} holds an unpaired surrogate")
        return None
    return value


def _check_attempt_text(entry, index, name, defects):
    if name not in entry:
        defects.append(f"attempt {index} is missing field {name}")
        return None
    value = entry[name]
    if not isinstance(value, str):
        defects.append(f"attempt {index} field {name} has the wrong type")
        return None
    if value == "":
        defects.append(f"attempt {index} field {name} is empty")
        return None
    if _holds_unpaired_surrogate(value):
        defects.append(
            f"attempt {index} field {name} holds an unpaired surrogate"
        )
        return None
    return value


def _check_finished_at(entry, index, result, defects):
    present = "finished_at" in entry
    value = entry.get("finished_at")
    if present:
        if not _is_number(value):
            defects.append(
                f"attempt {index} field finished_at has the wrong type"
            )
            return None
        if not _is_finite_number(value):
            defects.append(
                f"attempt {index} field finished_at is not a finite number"
            )
            return None
        if value < 0:
            defects.append(f"attempt {index} field finished_at is negative")
            return None
    if result in ("green", "red"):
        if not present:
            defects.append(f"attempt {index} is missing field finished_at")
            return None
        return value
    if result == "unfinished" and present:
        defects.append(
            f"attempt {index} field finished_at is present "
            "but the suite did not finish"
        )
    return None


def _load_attempt(entry, index, defects):
    if not isinstance(entry, dict):
        defects.append(f"attempt {index} is not an object")
        return None
    before = len(defects)
    model = _check_attempt_text(entry, index, "model", defects)
    environment = _check_attempt_text(entry, index, "environment", defects)
    result = None
    if "result" not in entry:
        defects.append(f"attempt {index} is missing field result")
    elif not isinstance(entry["result"], str):
        defects.append(f"attempt {index} field result has the wrong type")
    elif entry["result"] not in _RESULTS:
        defects.append(f"attempt {index} field result is not a suite result")
    else:
        result = entry["result"]
    tokens = None
    if "tokens" not in entry:
        defects.append(f"attempt {index} is missing field tokens")
    elif entry["tokens"] is not None:
        if not _is_int(entry["tokens"]):
            defects.append(f"attempt {index} field tokens has the wrong type")
        elif entry["tokens"] < 0:
            defects.append(f"attempt {index} field tokens is negative")
        else:
            tokens = entry["tokens"]
    patch = None
    if "patch" not in entry:
        defects.append(f"attempt {index} is missing field patch")
    elif not isinstance(entry["patch"], str):
        defects.append(f"attempt {index} field patch has the wrong type")
    elif _holds_unpaired_surrogate(entry["patch"]):
        defects.append(
            f"attempt {index} field patch holds an unpaired surrogate"
        )
    else:
        patch = entry["patch"]
    finished = _check_finished_at(entry, index, result, defects)
    if len(defects) != before:
        return None
    attempt = {
        "model": model,
        "environment": environment,
        "result": result,
        "tokens": tokens,
        "patch": patch,
    }
    if finished is not None:
        attempt["finished_at"] = finished
    return attempt


def _load_attempts(data, defects):
    if "attempts" not in data:
        defects.append("missing field attempts")
        return None
    raw = data["attempts"]
    if not isinstance(raw, list):
        defects.append("field attempts has the wrong type")
        return None
    if len(raw) != 2:
        defects.append("field attempts must hold exactly two attempts")
    clean = []
    for index, entry in enumerate(raw):
        attempt = _load_attempt(entry, index, defects)
        if attempt is not None:
            clean.append(attempt)
    if len(raw) != 2 or len(clean) != len(raw):
        return None
    return clean


def load_race_text(text):
    """Parse recorded race text into a race, naming every defect found.

    Returns a pair of the race and the defect list. A malformed
    input returns no race, and this function never raises on bad
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
    attempts = _load_attempts(data, defects)
    if defects:
        return None, defects
    return {"task": task, "attempts": attempts}, []


def load_race_file(path):
    """Read a recorded race from disk, with the same contract as text."""
    try:
        text = pathlib.Path(path).read_text(encoding="utf-8")
    except (OSError, UnicodeError):
        return None, ["file cannot be read"]
    return load_race_text(text)


def decide_winner(race):
    """Return the winning and losing attempts of a race.

    A green attempt beats a red or unfinished attempt. When both
    attempts are green the earlier finish wins, and an exact tie of
    finish times goes to the attempt that stands first in the
    record. When neither attempt is green both returns are None.
    """
    first, second = race["attempts"]
    greens = [attempt for attempt in race["attempts"] if attempt["result"] == "green"]
    if not greens:
        return None, None
    if len(greens) == 1:
        winner = greens[0]
    elif first["finished_at"] <= second["finished_at"]:
        winner = first
    else:
        winner = second
    loser = second if winner is first else first
    return winner, loser


def _outcome_phrase(attempt):
    if attempt["result"] == "unfinished":
        return "never finished"
    return (
        f"went {attempt['result']} "
        f"in {format_seconds(attempt['finished_at'])} seconds"
    )


def _report_line(attempt):
    if attempt["result"] == "unfinished":
        outcome = "unfinished"
    else:
        outcome = (
            f"{attempt['result']} "
            f"in {format_seconds(attempt['finished_at'])} seconds"
        )
    if attempt["tokens"] is None:
        spend = "tokens unreported"
    else:
        spend = f"{attempt['tokens']} tokens"
    return f"{attempt['model']} in {attempt['environment']}, {outcome}, {spend}"


def race_report(race):
    """Return the report lines for one race.

    One line per attempt, then a verdict line naming the winner or
    stating that there is no winner. The report is a pure function
    of the race.
    """
    lines = [f"race {race['task']}"]
    for attempt in race["attempts"]:
        lines.append(_report_line(attempt))
    winner, _loser = decide_winner(race)
    if winner is None:
        lines.append("verdict, no winner")
    else:
        lines.append(f"verdict, {winner['model']} wins")
    return lines


def funeral(race):
    """Return the funeral lines for one race.

    The first line names the task. The winner or the absence of one
    is stated, the loser is named when one exists, and a margin
    line names the seconds between the two finish times when both
    attempts finished. The funeral is a pure function of the race.
    """
    lines = [f"funeral for {race['task']}"]
    winner, loser = decide_winner(race)
    if winner is None:
        lines.append("no winner, neither suite went green")
        for attempt in race["attempts"]:
            lines.append(
                f"{attempt['model']} in {attempt['environment']} "
                f"{_outcome_phrase(attempt)}"
            )
    else:
        lines.append(
            f"winner {winner['model']} in {winner['environment']} "
            f"{_outcome_phrase(winner)}"
        )
        lines.append(
            f"loser {loser['model']} in {loser['environment']} "
            f"{_outcome_phrase(loser)}"
        )
    first, second = race["attempts"]
    if "finished_at" in first and "finished_at" in second:
        margin = abs(first["finished_at"] - second["finished_at"])
        lines.append(
            f"margin {format_seconds(margin)} seconds "
            "between the two finishes"
        )
    return lines


def slugify(label):
    """Reduce a label to a slug of lower case letters and digits.

    Runs of other characters become single dashes, dashes at the
    edges are dropped, and a label that reduces to nothing becomes
    the word unnamed.
    """
    slug = _SLUG_RUNS.sub("-", label.lower()).strip("-")
    return slug if slug else "unnamed"


def patch_file_name(race):
    """Return the file name the loser patch lives under."""
    _winner, loser = decide_winner(race)
    return f"{slugify(race['task'])}-{slugify(loser['model'])}.patch"


def adopt_patch(race, patches_dir):
    """Write the loser patch verbatim into the patches folder.

    Returns the path written, or None when the race has no loser.
    """
    _winner, loser = decide_winner(race)
    if loser is None:
        return None
    folder = pathlib.Path(patches_dir)
    folder.mkdir(parents=True, exist_ok=True)
    target = folder / patch_file_name(race)
    target.write_text(loser["patch"], encoding="utf-8")
    return target


def discard_patch(race, patches_dir):
    """Remove the loser patch file when it stands there.

    Returns the path the patch lives under, or None when the race
    has no loser. Removing a file that is not there still succeeds.
    """
    _winner, loser = decide_winner(race)
    if loser is None:
        return None
    target = pathlib.Path(patches_dir) / patch_file_name(race)
    target.unlink(missing_ok=True)
    return target


def empty_standings():
    """Return the standings a season starts from."""
    return {"models": {}}


def _blank_entry():
    return {
        "wins": 0,
        "losses": 0,
        "no_results": 0,
        "tokens": 0,
        "tokens_unreported": 0,
    }


def update_standings(standings, race):
    """Return new standings with one race recorded.

    The input standings are never mutated. A win and a loss are
    recorded for a decided race, one no result for both models of
    an undecided race, and token spend per attempt, where a null
    count records one unreported spend and never reads as zero.
    """
    models = {
        name: dict(entry) for name, entry in standings["models"].items()
    }

    def entry_for(model):
        return models.setdefault(model, _blank_entry())

    winner, loser = decide_winner(race)
    if winner is None:
        for attempt in race["attempts"]:
            entry_for(attempt["model"])["no_results"] += 1
    else:
        entry_for(winner["model"])["wins"] += 1
        entry_for(loser["model"])["losses"] += 1
    for attempt in race["attempts"]:
        entry = entry_for(attempt["model"])
        if attempt["tokens"] is None:
            entry["tokens_unreported"] += 1
        else:
            entry["tokens"] += attempt["tokens"]
    return {"models": models}


def _load_standings_entry(name, entry, defects):
    if not isinstance(entry, dict):
        defects.append(f"standings entry {name} is not an object")
        return None
    before = len(defects)
    clean = {}
    for field in _STANDINGS_FIELDS:
        if field not in entry:
            defects.append(f"standings entry {name} is missing field {field}")
        elif not _is_int(entry[field]):
            defects.append(
                f"standings entry {name} field {field} has the wrong type"
            )
        elif entry[field] < 0:
            defects.append(f"standings entry {name} field {field} is negative")
        else:
            clean[field] = entry[field]
    if len(defects) != before:
        return None
    return clean


def load_standings_text(text):
    """Parse standings text, naming every defect found.

    Returns a pair of the standings and the defect list, and never
    raises on bad content.
    """
    try:
        data = json.loads(text)
    except (ValueError, RecursionError):
        return None, ["input is not valid JSON"]
    if not isinstance(data, dict):
        return None, ["input is not a JSON object"]
    defects = []
    models = None
    if "models" not in data:
        defects.append("missing field models")
    elif not isinstance(data["models"], dict):
        defects.append("field models has the wrong type")
    else:
        models = {}
        for name, entry in data["models"].items():
            if _holds_unpaired_surrogate(name):
                defects.append("field models holds an unpaired surrogate")
                continue
            clean = _load_standings_entry(name, entry, defects)
            if clean is not None:
                models[name] = clean
    if defects:
        return None, defects
    return {"models": models}, []


def load_standings_file(path):
    """Read standings from disk, with the same contract as text."""
    try:
        text = pathlib.Path(path).read_text(encoding="utf-8")
    except (OSError, UnicodeError):
        return None, ["file cannot be read"]
    return load_standings_text(text)


def standings_text(standings):
    """Render standings as the JSON text the standings file holds."""
    return json.dumps(standings, indent=2, sort_keys=True) + "\n"


def standings_table(standings):
    """Return the standings table lines, one per model in label order."""
    lines = ["season standings"]
    models = standings["models"]
    if not models:
        lines.append("no races recorded")
    for name in sorted(models):
        entry = models[name]
        lines.append(
            f"{name}, {entry['wins']} wins, {entry['losses']} losses, "
            f"{entry['no_results']} no results, {entry['tokens']} tokens, "
            f"{entry['tokens_unreported']} unreported"
        )
    return lines
