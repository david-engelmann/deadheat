#!/usr/bin/env python3
"""Regenerate the Round 3 pilot records from the recorded envelopes.

This script is an example, not part of the package. It reads the
ten pilot races and the twenty recorded envelopes under this
directory, rebuilds each adjudicated record through the seat
readers in the package, and rewrites the records, their golden
closecall reports, the failed call list, the run record, and
the pilot series with its golden. It makes no live call, and
running it over the committed tree reproduces the committed
files byte for byte, which the suite proves. The obligations
behind the pilot are DH-7 and DH-8 in CONTRACT.md.
"""

import json
import sys
from decimal import ROUND_HALF_UP, Decimal
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "src"))

from deadheat.races import close_call_report, load_race_file  # noqa: E402
from deadheat.seats import adjudicated_race  # noqa: E402
from deadheat.series import series_report  # noqa: E402

PILOT = Path(__file__).resolve().parent
RACES = PILOT / "races"
ENVELOPES = PILOT / "envelopes"
RECORDS = PILOT / "records"
ROUTES = ("jev", "openai")
RACE_NAMES = [f"race-{index:02d}" for index in range(1, 11)]


def _dump(path, value):
    """Write one JSON value in the pilot file discipline."""
    path.write_text(
        json.dumps(value, indent=1, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def _usage_figures(route, envelope):
    """Return the input and output token counts of one envelope."""
    usage = envelope.get("usage") or {}
    if route == "jev":
        return (
            usage.get("input_tokens", 0),
            usage.get("output_tokens", 0),
        )
    return (
        usage.get("prompt_tokens", 0),
        usage.get("completion_tokens", 0),
    )


def main():
    RECORDS.mkdir(exist_ok=True)
    failed = []
    models = {}
    jev_records = []
    for name in RACE_NAMES:
        race, defects = load_race_file(RACES / f"{name}.json")
        if defects:
            raise SystemExit(f"{name} does not load clean: {defects}")
        for route in ROUTES:
            envelope_path = ENVELOPES / f"{name}-{route}.json"
            envelope = None
            if envelope_path.exists():
                envelope = json.loads(
                    envelope_path.read_text(encoding="utf-8")
                )
            record = None
            if envelope is not None:
                record = adjudicated_race(race, envelope, route)
            if isinstance(envelope, dict) and isinstance(
                envelope.get("model"), str
            ):
                figures = models.setdefault(
                    envelope["model"],
                    {
                        "route": route,
                        "calls_attempted": 0,
                        "entries_recorded": 0,
                        "failed_calls": 0,
                        "input_tokens": 0,
                        "output_tokens": 0,
                        "cost": Decimal("0"),
                    },
                )
                figures["calls_attempted"] += 1
                gained_in, gained_out = _usage_figures(route, envelope)
                figures["input_tokens"] += gained_in
                figures["output_tokens"] += gained_out
                if record is not None:
                    figures["entries_recorded"] += 1
                if route == "openai":
                    figures["cost"] += Decimal(
                        str((envelope.get("usage") or {})["cost"])
                    )
            if record is None:
                record = race
                failed.append({"race": name, "route": route})
            _dump(RECORDS / f"{name}-{route}.json", record)
            golden = "\n".join(close_call_report(record)) + "\n"
            (RECORDS / f"{name}-{route}-closecall.txt").write_text(
                golden, encoding="utf-8"
            )
            if route == "jev":
                jev_records.append(record)
    for model, figures in models.items():
        route = figures["route"]
        figures["failed_calls"] = sum(
            1 for pair in failed if pair["route"] == route
        )
        if route == "openai":
            figures["cost"] = str(
                figures["cost"].quantize(
                    Decimal("0.000001"), rounding=ROUND_HALF_UP
                )
            )
        else:
            del figures["cost"]
    _dump(PILOT / "failed.json", failed)
    _dump(PILOT / "run_record.json", {"models": models})
    series = {
        "task": "the recorded close call pilot",
        "models": ["prime-1", "shadow-1"],
        "wins_target": 3,
        "races": jev_records,
    }
    _dump(PILOT.parent / "pilot-series.json", series)
    (PILOT.parent / "pilot-series.txt").write_text(
        "\n".join(series_report(series)) + "\n", encoding="utf-8"
    )


if __name__ == "__main__":
    main()
