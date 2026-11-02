"""The deadheat command line surface.

Six commands work on recorded races, series, and the standings
file. The race command prints the report and the funeral for one
recorded race and can update a standings file. The adopt and
discard commands perform the loser patch file operations. The
standings command prints the standings table. The closecall
command prints whether one recorded race sits in the close call
band and why, and the series command prints the series report.
Nothing here touches the network.
"""

import argparse
import pathlib
import sys

from deadheat.races import (
    adopt_patch,
    close_call_report,
    discard_patch,
    empty_standings,
    funeral,
    load_race_file,
    load_standings_file,
    race_report,
    standings_table,
    standings_text,
    update_standings,
)
from deadheat.series import load_series_file, series_report


def _load_race_or_print_defects(path):
    race, defects = load_race_file(path)
    if defects:
        print("invalid recorded race")
        for defect in defects:
            print(f"defect {defect}")
        return None
    return race


def _print_defects(headline, defects):
    print(headline)
    for defect in defects:
        print(f"defect {defect}")


def _command_race(args):
    race = _load_race_or_print_defects(args.file)
    if race is None:
        return 1
    if args.standings is not None:
        standings_path = pathlib.Path(args.standings)
        if standings_path.exists():
            standings, defects = load_standings_file(standings_path)
            if defects:
                _print_defects("invalid standings file", defects)
                return 1
        else:
            standings = empty_standings()
        updated = update_standings(standings, race)
        standings_path.parent.mkdir(parents=True, exist_ok=True)
        standings_path.write_text(standings_text(updated), encoding="utf-8")
    for line in race_report(race):
        print(line)
    for line in funeral(race):
        print(line)
    return 0


def _command_adopt(args):
    race = _load_race_or_print_defects(args.file)
    if race is None:
        return 1
    target = adopt_patch(race, args.patches)
    if target is None:
        print("no loser patch, the race had no winner")
        return 1
    print(f"adopted the loser patch to {target}")
    return 0


def _command_discard(args):
    race = _load_race_or_print_defects(args.file)
    if race is None:
        return 1
    target = discard_patch(race, args.patches)
    if target is None:
        print("no loser patch, the race had no winner")
        return 1
    print(f"discarded the loser patch at {target}")
    return 0


def _command_standings(args):
    standings, defects = load_standings_file(args.file)
    if defects:
        _print_defects("invalid standings file", defects)
        return 1
    for line in standings_table(standings):
        print(line)
    return 0


def _command_closecall(args):
    race = _load_race_or_print_defects(args.file)
    if race is None:
        return 1
    for line in close_call_report(race):
        print(line)
    return 0


def _command_series(args):
    series, defects = load_series_file(args.file)
    if defects:
        _print_defects("invalid series file", defects)
        return 1
    for line in series_report(series):
        print(line)
    return 0


def main(argv=None):
    """Run the deadheat command line and return the exit status."""
    parser = argparse.ArgumentParser(
        prog="deadheat",
        description="Race two attempts at one task and keep the standings.",
    )
    commands = parser.add_subparsers(dest="command", required=True)
    race = commands.add_parser("race", help="print the report and funeral for one recorded race")
    race.add_argument("file")
    race.add_argument("--standings", help="standings file to update after the race")
    race.set_defaults(handler=_command_race)
    adopt = commands.add_parser("adopt", help="write the loser patch into a patches folder")
    adopt.add_argument("file")
    adopt.add_argument("patches")
    adopt.set_defaults(handler=_command_adopt)
    discard = commands.add_parser("discard", help="remove the loser patch from a patches folder")
    discard.add_argument("file")
    discard.add_argument("patches")
    discard.set_defaults(handler=_command_discard)
    standings = commands.add_parser("standings", help="print the standings table")
    standings.add_argument("file")
    standings.set_defaults(handler=_command_standings)
    closecall = commands.add_parser("closecall", help="print whether one recorded race sits in the close call band and why")
    closecall.add_argument("file")
    closecall.set_defaults(handler=_command_closecall)
    series = commands.add_parser("series", help="print the series report for one series file")
    series.add_argument("file")
    series.set_defaults(handler=_command_series)
    args = parser.parse_args(argv)
    return args.handler(args)


if __name__ == "__main__":
    sys.exit(main())
