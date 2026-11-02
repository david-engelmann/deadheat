"""Recorded races, the winner rule, the funeral, and the standings.

Every function in this module is a pure function of its inputs, and
the loaders never raise on malformed content. The obligations this
module carries are DH-1 to DH-4 in CONTRACT.md, in the full form
section, and DH-5 in the Round 2 section.
"""

import json
import math
import pathlib
import re
from fractions import Fraction

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


def _round_half_up_units(fraction, places):
    """Return the count of units a fraction rounds to at a precision.

    The count is taken on the exact value, and an exact half rounds
    up. Every value the renderers pass is non negative, a finish
    time, a confidence, or a gap taken as an absolute value, so the
    rule needs no sign case.
    """
    scaled = fraction * (10 ** places)
    units = scaled.numerator // scaled.denominator
    if 2 * (scaled - units) >= 1:
        units += 1
    return units


def _render_units(units, places):
    """Render a count of units at a precision in plain decimal."""
    whole, part = divmod(units, 10 ** places)
    if part == 0:
        return str(whole)
    digits = f"{part:0{places}d}".rstrip("0")
    return f"{whole}.{digits}"


def _exact_expansion(fraction):
    """Return the exact decimal expansion of a fraction, or None.

    The expansion terminates exactly when the denominator of the
    fraction in its lowest terms carries no prime factor other than
    two and five.
    """
    denominator = fraction.denominator
    twos = 0
    while denominator % 2 == 0:
        denominator //= 2
        twos += 1
    fives = 0
    while denominator % 5 == 0:
        denominator //= 5
        fives += 1
    if denominator != 1:
        return None
    places = max(twos, fives)
    return _render_units(int(fraction * (10 ** places)), places)


def format_seconds(value):
    """Render a number the same way everywhere it appears.

    Seconds and confidence figures both render through here, from
    the exact value and never through a float. A figure that needs
    six or fewer decimal places renders exactly. A figure that
    needs more renders rounded at six decimal places, with an
    exact half at the seventh place rounding up on the exact
    value. A nonzero figure that rounds to zero at six places
    renders its exact decimal expansion when that expansion
    terminates, and otherwise renders rounded at twelve decimal
    places with halves rounding up. Every figure renders in plain
    decimal notation and never in scientific notation, so a nonzero
    margin never prints as zero.
    The rule is FIX-DH-8 in CONTRACT.md, amending DH-3 and DH-5.
    """
    fraction = Fraction(value)
    if (fraction * 1000000).denominator == 1:
        return _render_units(int(fraction * 1000000), 6)
    if fraction != 0 and fraction < Fraction(1, 2000000):
        expansion = _exact_expansion(fraction)
        if expansion is not None:
            return expansion
        return _render_units(_round_half_up_units(fraction, 12), 12)
    return _render_units(_round_half_up_units(fraction, 6), 6)


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


def _check_adjudication_text(entry, name, defects):
    if name not in entry:
        defects.append(f"adjudication is missing field {name}")
        return None
    value = entry[name]
    if not isinstance(value, str):
        defects.append(f"adjudication field {name} has the wrong type")
        return None
    if value == "":
        defects.append(f"adjudication field {name} is empty")
        return None
    if _holds_unpaired_surrogate(value):
        defects.append(
            f"adjudication field {name} holds an unpaired surrogate"
        )
        return None
    return value


def _load_adjudication(data, attempts, defects):
    if "adjudication" not in data:
        return None
    entry = data["adjudication"]
    if not isinstance(entry, dict):
        defects.append("field adjudication is not an object")
        return None
    before = len(defects)
    adjudicator = _check_adjudication_text(entry, "adjudicator", defects)
    chosen = _check_adjudication_text(entry, "chosen", defects)
    reason = _check_adjudication_text(entry, "reason", defects)
    if reason is not None and ("\n" in reason or "\r" in reason):
        defects.append("adjudication field reason holds a newline")
    confidence = None
    if "confidence" not in entry:
        defects.append("adjudication is missing field confidence")
    elif not _is_number(entry["confidence"]):
        defects.append("adjudication field confidence has the wrong type")
    elif not _is_finite_number(entry["confidence"]):
        defects.append(
            "adjudication field confidence is not a finite number"
        )
    elif entry["confidence"] < 0 or entry["confidence"] > 1:
        defects.append(
            "adjudication field confidence is outside the range zero to one"
        )
    else:
        confidence = entry["confidence"]
    if chosen is not None and attempts is not None:
        if chosen not in [attempt["model"] for attempt in attempts]:
            defects.append(
                "adjudication field chosen names no attempt in this race"
            )
    if len(defects) != before:
        return None
    return {
        "adjudicator": adjudicator,
        "chosen": chosen,
        "confidence": confidence,
        "reason": reason,
    }


def load_race_data(data):
    """Build a race from a parsed JSON object, naming every defect.

    Returns a pair of the race and the defect list. A malformed
    object returns no race, and this function never raises on bad
    content. The text and file entry points parse first and then
    read the race through here.
    """
    defects = []
    task = _check_text_field(data, "task", defects)
    attempts = _load_attempts(data, defects)
    adjudication = _load_adjudication(data, attempts, defects)
    if defects:
        return None, defects
    race = {"task": task, "attempts": attempts}
    if adjudication is not None:
        race["adjudication"] = adjudication
    return race, []


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
    return load_race_data(data)


def load_race_file(path):
    """Read a recorded race from disk, with the same contract as text."""
    try:
        text = pathlib.Path(path).read_text(encoding="utf-8")
    except (OSError, UnicodeError):
        return None, ["file cannot be read"]
    return load_race_text(text)


def _exact(value):
    """Return the exact fraction a recorded number names.

    A float converts to the fraction of its exact binary value and
    an int to itself, so comparisons built on this never round.
    """
    return Fraction(value)


def is_close_call(race):
    """Return whether a race sits in the close call band of DH-5.

    Both attempts holding the same non green outcome is a close
    call. Both attempts green is a close call when the finish times
    differ and the gap is no more than five percent of the slower
    finish. The comparison multiplies the exact gap by twenty and
    compares it against the exact slower finish, so no rounding at
    any precision can move a race across the band edge. An exact
    tie of green finish times is not a close call.
    """
    first, second = race["attempts"]
    if first["result"] != second["result"]:
        return False
    if first["result"] != "green":
        return True
    gap = abs(_exact(first["finished_at"]) - _exact(second["finished_at"]))
    slower = max(_exact(first["finished_at"]), _exact(second["finished_at"]))
    return gap > 0 and gap * 20 <= slower


_CONFIDENCE_THRESHOLD = Fraction(4, 5)


def _confidence_meets(confidence):
    """Return whether a recorded confidence meets the DH-5 threshold.

    The threshold is the constant _CONFIDENCE_THRESHOLD, four
    fifths, and this comparison is its one home. The recorded
    figure is compared against it on exact fractions, so a
    recorded 0.8 meets the threshold and no rounding can move a
    verdict across it.
    """
    return _exact(confidence) >= _CONFIDENCE_THRESHOLD


def _format_confidence(confidence):
    """Render a confidence for a line reporting it below the threshold.

    The figure renders the way seconds render, with one exception.
    A confidence below the threshold whose rendering at six places
    would print the threshold figure itself renders in full
    precision instead, its exact decimal expansion when that
    expansion terminates and otherwise rounded at twelve decimal
    places with halves rounding up on the exact value. A line
    reporting an abstention therefore never shows a confidence
    equal to the threshold. The rule is FIX-DH-10 in CONTRACT.md,
    amending DH-5.
    """
    fraction = _exact(confidence)
    if fraction < _CONFIDENCE_THRESHOLD and format_seconds(
        confidence
    ) == format_seconds(_CONFIDENCE_THRESHOLD):
        expansion = _exact_expansion(fraction)
        if expansion is not None:
            return expansion
        return _render_units(_round_half_up_units(fraction, 12), 12)
    return format_seconds(confidence)


def _adjudication_state(race):
    """Name what the adjudication machinery does to a race.

    The words are decided when an entry in the band meets the
    confidence threshold, abstained when an entry in the band falls
    below it, unrecorded when the band holds no entry, not needed
    when an entry rides on a race outside the band, and absent when
    the race carries no entry and sits outside the band.
    """
    entry = race.get("adjudication")
    if not is_close_call(race):
        return "not needed" if entry is not None else "absent"
    if entry is None:
        return "unrecorded"
    if _confidence_meets(entry["confidence"]):
        return "decided"
    return "abstained"


def decide_winner(race):
    """Return the winning and losing attempts of a race.

    A close call under DH-5 is decided for the chosen attempt when
    its adjudication entry meets the confidence threshold, and
    stands with no winner otherwise. Outside the band a green
    attempt beats a red or unfinished attempt. When both attempts
    are green the earlier finish wins, and an exact tie of finish
    times goes to the attempt that stands first in the record.
    When neither attempt is green both returns are None.
    """
    first, second = race["attempts"]
    if is_close_call(race):
        entry = race.get("adjudication")
        if entry is not None and _confidence_meets(entry["confidence"]):
            winner = first if first["model"] == entry["chosen"] else second
            loser = second if winner is first else first
            return winner, loser
        return None, None
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
    entry = race.get("adjudication")
    state = _adjudication_state(race)
    if state == "decided":
        lines.append(
            f"close call, adjudication by {entry['adjudicator']} "
            f"chose {winner['model']} at confidence "
            f"{format_seconds(entry['confidence'])}"
        )
    elif state == "abstained":
        lines.append(
            f"close call, adjudication by {entry['adjudicator']} "
            f"abstained at confidence {_format_confidence(entry['confidence'])}"
        )
    elif state == "unrecorded":
        lines.append("close call, no adjudication on record")
    elif state == "not needed":
        lines.append(
            f"adjudication by {entry['adjudicator']} not needed, "
            "the race stands outside the close call band"
        )
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
    first, second = race["attempts"]
    if winner is None:
        if first["result"] == "green" and second["result"] == "green":
            lines.append(
                "no winner, both suites went green "
                "inside the close call band"
            )
        else:
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
    entry = race.get("adjudication")
    state = _adjudication_state(race)
    if state == "decided":
        lines.append(
            f"decided by adjudication, {entry['adjudicator']} "
            f"chose {winner['model']} at confidence "
            f"{format_seconds(entry['confidence'])}"
        )
    elif state == "abstained":
        lines.append(
            f"close call, adjudication by {entry['adjudicator']} "
            f"abstained at confidence "
            f"{_format_confidence(entry['confidence'])}, "
            "the race stands as a dead heat"
        )
    elif state == "unrecorded":
        lines.append(
            "close call, no adjudication on record, "
            "the race stands as a dead heat"
        )
    if "finished_at" in first and "finished_at" in second:
        margin = abs(
            _exact(first["finished_at"]) - _exact(second["finished_at"])
        )
        lines.append(
            f"margin {format_seconds(margin)} seconds "
            "between the two finishes"
        )
    return lines


def close_call_report(race):
    """Return the closecall lines for one race.

    The first line names the race. The second states whether the
    race sits in the close call band and why. The next states what
    the adjudication entry does when one matters, and the last is
    the verdict line the race report uses. The report is a pure
    function of the race.
    """
    first, second = race["attempts"]
    lines = [f"closecall {race['task']}"]
    if first["result"] == "green" and second["result"] == "green":
        gap = abs(
            _exact(first["finished_at"]) - _exact(second["finished_at"])
        )
        slower = max(
            _exact(first["finished_at"]), _exact(second["finished_at"])
        )
        if gap == 0:
            lines.append(
                "outside the band, the finish times are tied and "
                "the tie goes to the first attempt in the record"
            )
        elif is_close_call(race):
            lines.append(
                "in the band, both attempts went green and "
                f"the gap of {format_seconds(gap)} seconds is within "
                "five percent of the slower finish of "
                f"{format_seconds(slower)} seconds"
            )
        else:
            lines.append(
                "outside the band, the gap of "
                f"{format_seconds(gap)} seconds is wider than "
                "five percent of the slower finish of "
                f"{format_seconds(slower)} seconds"
            )
    elif first["result"] == second["result"]:
        if first["result"] == "red":
            lines.append("in the band, both attempts went red")
        else:
            lines.append("in the band, both attempts never finished")
    else:
        lines.append(
            "outside the band, the two attempts hold different outcomes"
        )
    entry = race.get("adjudication")
    state = _adjudication_state(race)
    if state == "decided":
        winner, _loser = decide_winner(race)
        lines.append(
            f"adjudication by {entry['adjudicator']} "
            f"chose {winner['model']} at confidence "
            f"{format_seconds(entry['confidence'])}, "
            f"the race is decided for {winner['model']}"
        )
    elif state == "abstained":
        lines.append(
            f"adjudication by {entry['adjudicator']} "
            f"abstained at confidence "
            f"{_format_confidence(entry['confidence'])}, "
            "below the threshold of "
            f"{format_seconds(_CONFIDENCE_THRESHOLD)}"
        )
    elif state == "unrecorded":
        lines.append(
            "no adjudication on record, the race stands as no result"
        )
    elif state == "not needed":
        lines.append(
            f"adjudication by {entry['adjudicator']} not needed, "
            "the race stands outside the close call band"
        )
    winner, _loser = decide_winner(race)
    if winner is None:
        lines.append("verdict, no winner")
    else:
        lines.append(f"verdict, {winner['model']} wins")
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
