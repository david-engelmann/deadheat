#!/usr/bin/env python3
"""Regenerate the Round 4 panel pilot from the Round 3 pilot records.

This script is an example, not part of the package. It reads the
twenty adjudicated pilot records under the pilot directory
beside this one, joins each Jev record and chat record pair
through the panel join in the package, and rewrites the panel
records, their golden closecall reports, and the panel series
with its golden. It makes no live call, and running it over the
committed tree reproduces the committed files byte for byte,
which the suite proves. The obligations behind the panel pilot
are DH-9 and DH-10 in CONTRACT.md.
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "src"))

from deadheat.panels import join_panel_records  # noqa: E402
from deadheat.races import close_call_report, load_race_file  # noqa: E402
from deadheat.series import series_report  # noqa: E402

PANEL = Path(__file__).resolve().parent
PILOT_RECORDS = PANEL.parent / "pilot" / "records"
RECORDS = PANEL / "records"
ROUTES = ("jev", "openai")
RACE_NAMES = [f"race-{index:02d}" for index in range(1, 11)]


def _dump(path, value):
    """Write one JSON value in the pilot file discipline."""
    path.write_text(
        json.dumps(value, indent=1, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def main():
    RECORDS.mkdir(exist_ok=True)
    panel_records = []
    for name in RACE_NAMES:
        loaded = []
        for route in ROUTES:
            record, defects = load_race_file(
                PILOT_RECORDS / f"{name}-{route}.json"
            )
            if defects:
                raise SystemExit(
                    f"{name}-{route} does not load clean: {defects}"
                )
            loaded.append(record)
        joined, defects = join_panel_records(loaded[0], loaded[1])
        if defects:
            raise SystemExit(f"{name} does not join clean: {defects}")
        _dump(RECORDS / f"{name}.json", joined)
        golden = "\n".join(close_call_report(joined)) + "\n"
        (RECORDS / f"{name}-closecall.txt").write_text(
            golden, encoding="utf-8"
        )
        panel_records.append(joined)
    series = {
        "task": "the recorded close call panel",
        "models": ["prime-1", "shadow-1"],
        "wins_target": 3,
        "races": panel_records,
    }
    _dump(PANEL.parent / "panel-series.json", series)
    (PANEL.parent / "panel-series.txt").write_text(
        "\n".join(series_report(series)) + "\n", encoding="utf-8"
    )


if __name__ == "__main__":
    main()
