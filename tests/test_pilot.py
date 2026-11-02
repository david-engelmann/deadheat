"""Pins for the recorded close call pilot, DH-8 in CONTRACT.md.

The suite reads the committed races, envelopes, and records and
makes no live call. The run record pin recomputes every figure
from the envelopes and the failed call list by an independent
path, and the regeneration pin proves the pilot maker rebuilds
the committed records byte for byte offline.
"""

import hashlib
import json
import pathlib
import runpy
import shutil
import subprocess
import sys
from decimal import ROUND_HALF_UP, Decimal

from deadheat.races import is_close_call, load_race_file
from deadheat.seats import (
    adjudicated_race,
    entry_from_chat_envelope,
    entry_from_jev_envelope,
    has_seat,
)
from deadheat.series import load_series_file, series_report

ROOT = pathlib.Path(__file__).resolve().parent.parent
EXAMPLES = ROOT / "examples"
PILOT = EXAMPLES / "pilot"
RACE_NAMES = [f"race-{index:02d}" for index in range(1, 11)]
ROUTES = ("jev", "openai")
READERS = {"jev": entry_from_jev_envelope, "openai": entry_from_chat_envelope}


def _load_json(path):
    return json.loads(path.read_text(encoding="utf-8"))


def test_every_pilot_race_loads_clean_and_sits_in_the_band():
    for name in RACE_NAMES:
        race, defects = load_race_file(PILOT / "races" / f"{name}.json")
        assert defects == [], name
        assert is_close_call(race), name
        assert has_seat(race), name
        labels = [attempt["model"] for attempt in race["attempts"]]
        assert labels == ["prime-1", "shadow-1"], name


def test_every_record_is_its_envelope_read_through_the_seat():
    for name in RACE_NAMES:
        race, defects = load_race_file(PILOT / "races" / f"{name}.json")
        assert defects == []
        for route in ROUTES:
            envelope = _load_json(PILOT / "envelopes" / f"{name}-{route}.json")
            record, record_defects = load_race_file(
                PILOT / "records" / f"{name}-{route}.json"
            )
            assert record_defects == [], (name, route)
            rebuilt = adjudicated_race(race, envelope, route)
            assert rebuilt is not None, (name, route)
            assert record == rebuilt, (name, route)


def test_the_failed_list_stands_exactly_where_no_entry_stands():
    failed = _load_json(PILOT / "failed.json")
    failed_pairs = {(pair["race"], pair["route"]) for pair in failed}
    for name in RACE_NAMES:
        for route in ROUTES:
            record = _load_json(PILOT / "records" / f"{name}-{route}.json")
            has_entry = "adjudication" in record
            assert ((name, route) in failed_pairs) == (not has_entry), (
                name,
                route,
            )


def test_the_run_record_recomputes_from_the_envelopes():
    failed = _load_json(PILOT / "failed.json")
    recomputed = {}
    for name in RACE_NAMES:
        race, defects = load_race_file(PILOT / "races" / f"{name}.json")
        assert defects == []
        for route in ROUTES:
            envelope = _load_json(PILOT / "envelopes" / f"{name}-{route}.json")
            model = envelope["model"]
            figures = recomputed.setdefault(
                model,
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
            usage = envelope["usage"]
            if route == "jev":
                figures["input_tokens"] += usage["input_tokens"]
                figures["output_tokens"] += usage["output_tokens"]
            else:
                figures["input_tokens"] += usage["prompt_tokens"]
                figures["output_tokens"] += usage["completion_tokens"]
                figures["cost"] += Decimal(str(usage["cost"]))
            if READERS[route](race, envelope) is not None:
                figures["entries_recorded"] += 1
    for pair in failed:
        for model, figures in recomputed.items():
            if figures["route"] == pair["route"]:
                figures["failed_calls"] += 1
    for figures in recomputed.values():
        if figures["route"] == "openai":
            figures["cost"] = str(
                figures["cost"].quantize(
                    Decimal("0.000001"), rounding=ROUND_HALF_UP
                )
            )
        else:
            del figures["cost"]
    recorded = _load_json(PILOT / "run_record.json")
    assert recorded == {"models": recomputed}


def test_the_pilot_maker_regenerates_the_pilot_byte_for_byte():
    watched = sorted(
        list((PILOT / "records").glob("*"))
        + [PILOT / "failed.json", PILOT / "run_record.json"]
        + [EXAMPLES / "pilot-series.json", EXAMPLES / "pilot-series.txt"]
    )

    def digest():
        return {
            str(path): hashlib.sha256(path.read_bytes()).hexdigest()
            for path in watched
        }

    before = digest()
    runpy.run_path(str(PILOT / "make_pilot.py"), run_name="__main__")
    assert digest() == before


def test_the_maker_counts_an_attempt_where_it_reads_the_envelope(tmp_path):
    copied = tmp_path / "examples"
    shutil.copytree(EXAMPLES, copied)
    (copied / "pilot" / "envelopes" / "race-10-openai.json").unlink()
    runpy.run_path(
        str(copied / "pilot" / "make_pilot.py"), run_name="__main__"
    )
    recorded = _load_json(copied / "pilot" / "run_record.json")
    by_route = {
        figures["route"]: figures
        for figures in recorded["models"].values()
    }
    assert by_route["openai"]["calls_attempted"] == 9
    assert by_route["openai"]["failed_calls"] == 1
    assert by_route["jev"]["calls_attempted"] == 10
    assert by_route["jev"]["failed_calls"] == 0


def test_the_pilot_series_counts_the_jev_records():
    series, defects = load_series_file(EXAMPLES / "pilot-series.json")
    assert defects == []
    for index, name in enumerate(RACE_NAMES):
        record, record_defects = load_race_file(
            PILOT / "records" / f"{name}-jev.json"
        )
        assert record_defects == []
        assert series["races"][index] == record
    golden = (EXAMPLES / "pilot-series.txt").read_text(encoding="utf-8")
    assert "\n".join(series_report(series)) + "\n" == golden
    assert "race 1, pin the cache key, prime-1 wins" in golden
    assert "race 7, shrink the prompt budget, shadow-1 wins" in golden
    assert golden.splitlines()[-1] == "series winner, prime-1"


def test_the_scan_exempts_the_envelope_directory_alone(tmp_path):
    script_source = (ROOT / "scripts" / "codepoint_scan.py").read_text(
        encoding="utf-8"
    )
    (tmp_path / "scripts").mkdir()
    (tmp_path / "scripts" / "codepoint_scan.py").write_text(
        script_source, encoding="utf-8"
    )
    envelopes = tmp_path / "examples" / "pilot" / "envelopes"
    envelopes.mkdir(parents=True)
    (envelopes / "e.json").write_text(
        '{"model": "a\u2014b"}\n', encoding="utf-8"
    )
    clean = subprocess.run(
        [sys.executable, "scripts/codepoint_scan.py"],
        capture_output=True,
        text=True,
        cwd=tmp_path,
    )
    assert clean.returncode == 0
    assert "U+2014=0" in clean.stdout
    races = tmp_path / "examples" / "pilot" / "races"
    races.mkdir(parents=True)
    (races / "r.json").write_text('{"task": "a\u2014b"}\n', encoding="utf-8")
    dirty = subprocess.run(
        [sys.executable, "scripts/codepoint_scan.py"],
        capture_output=True,
        text=True,
        cwd=tmp_path,
    )
    assert dirty.returncode == 1
    assert "U+2014=1" in dirty.stdout
