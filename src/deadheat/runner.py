"""The local runner for deadheat.

The runner runs one shell command per attempt, each in its own
directory under one race workspace, and builds the attempt record
the loader in races.py accepts. It is offline and local only. The
obligations this module carries are in the DH-1 runner text in
CONTRACT.md, in the full form section.
"""

import json
import pathlib
import shutil
import subprocess
import time

_RESULTS = ("green", "red", "unfinished")


def _empty_directory(directory):
    """Create the lane directory and remove everything it holds."""
    directory.mkdir(parents=True, exist_ok=True)
    for child in directory.iterdir():
        if child.is_dir() and not child.is_symlink():
            shutil.rmtree(child)
        else:
            child.unlink()


def _read_result_file(directory):
    """Return the result file the command wrote, as an object.

    A result file that cannot be read as a JSON object is ignored
    and an empty object comes back.
    """
    try:
        data = json.loads((directory / "result.json").read_text(encoding="utf-8"))
    except (OSError, UnicodeError, ValueError):
        return {}
    if not isinstance(data, dict):
        return {}
    return data


def _read_patch(directory):
    """Return the patch file text the command wrote, or empty text."""
    try:
        return (directory / "patch.diff").read_text(encoding="utf-8")
    except (OSError, UnicodeError):
        return ""


def run_attempt(workspace, lane, model, environment, command, timeout_seconds=None):
    """Run one attempt command and return its attempt record.

    The command runs in the directory the lane names under the
    workspace. The lane is emptied before the command starts, so
    the record can only be built from files this command writes.
    The record follows the DH-1 runner text in CONTRACT.md.
    """
    directory = pathlib.Path(workspace) / lane
    _empty_directory(directory)
    started = time.monotonic()
    try:
        completed = subprocess.run(
            command,
            shell=True,
            cwd=str(directory),
            timeout=timeout_seconds,
            capture_output=True,
        )
    except subprocess.TimeoutExpired:
        return {
            "model": model,
            "environment": environment,
            "result": "unfinished",
            "tokens": None,
            "patch": _read_patch(directory),
        }
    elapsed = time.monotonic() - started
    named = _read_result_file(directory)
    result = named.get("result")
    if result not in _RESULTS:
        result = "green" if completed.returncode == 0 else "red"
    tokens = named.get("tokens")
    if not (
        isinstance(tokens, int)
        and not isinstance(tokens, bool)
        and tokens >= 0
    ):
        tokens = None
    attempt = {
        "model": model,
        "environment": environment,
        "result": result,
        "tokens": tokens,
        "patch": _read_patch(directory),
    }
    if result != "unfinished":
        attempt["finished_at"] = elapsed
    return attempt


def run_race(workspace, task, specs):
    """Run the two attempts of one race and return the race record.

    Each spec holds a model, an environment, a command, and an
    optional timeout_seconds. The lanes are named attempt-1 and
    attempt-2 under the workspace, in record order.
    """
    attempts = []
    for index, spec in enumerate(specs):
        attempts.append(
            run_attempt(
                workspace,
                f"attempt-{index + 1}",
                spec["model"],
                spec["environment"],
                spec["command"],
                timeout_seconds=spec.get("timeout_seconds"),
            )
        )
    return {"task": task, "attempts": attempts}
