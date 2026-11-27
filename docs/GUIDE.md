# Deadheat User Guide

Deadheat runs head-to-head duels between two model attempts on the same coding or engineering task, adjudicates close calls, adopts or discards loser patches, and prints thirty-five analytical reports across races, standings, first-to-N series, and multi-series championships.

## Installation

### Prebuilt binary (Linux and macOS)

```sh
curl -fsSL https://raw.githubusercontent.com/david-engelmann/deadheat/main/install.sh | sh
```

### Cargo Binstall

```sh
cargo binstall deadheat
```

### Build from source

```sh
git clone https://github.com/david-engelmann/deadheat.git
cd deadheat
cargo install --path . --bin deadheat --locked
```

## 60-Second Walkthrough

Run a live local duel using the synthetic specification in `examples/duel-spec.json`, inspect the resulting race report, and then explore close-call and multi-series championship analytics on the shipped fixtures.

```sh
deadheat run examples/duel-spec.json
deadheat race duel-record.json
deadheat closecall examples/close-call-decided.json
deadheat championship examples/championship.json
deadheat totals examples/league-championship.json
deadheat pairing examples/league-championship.json
```

Clean up the local scratch artifacts created by `duel-spec.json` at any time with `rm -rf duel-workspace duel-record.json`.

## How Winner Selection and Close Calls Work

1. **Single green beats red or unfinished.** When one attempt finishes `"green"` and the other finishes `"red"` or `"unfinished"`, the `"green"` attempt wins outright. When neither attempt finishes `"green"` and at least one is `"unfinished"`, the race records **no result** (`DH-1`, `DH-2`).
2. **Adjusted finish time and exact ties.** Each finished attempt's effective time is `finished_at + adjustment` (`DH-11`). When both attempts finish `"green"` at the exact same adjusted finish (`gap == 0`), the tie lies outside the close-call band and the first attempt in record order wins outright (`DH-2`, `FIX-DH-9`).
3. **Five-percent close-call band and both-red adjudication.** A race enters the close-call band when both attempts finish `"green"` with `0 < gap <= 5%` of the slower adjusted finish, **or when both attempts finish `"red"`** (`DH-5`). Outside the band on `"green"`, the faster attempt wins outright. Inside the band, `deadheat` consults a recorded single-judge `adjudication` (`confidence >= 0.8`, `DH-5`) or two-judge `panel` (`DH-9`, crowning a winner when both judges decide at `>= 0.8` for the same model, or when one judge decides at `>= 0.8` while the other abstains below `0.8` or stands as a one-judge panel). Unadjudicated, low-confidence, or split-panel close calls record no result.

## Command Reference Matrix (38 Commands)

All input JSON schemas stand documented in [../FORMATS.md](../FORMATS.md).

### Execution, patch management, race, standings, and series commands (8 commands)

| Command | Arguments | Purpose |
| --- | --- | --- |
| `run` | `<spec.json>` | Execute two competing attempt commands in isolated workspace lanes and write a race record |
| `race` | `<race.json> [--standings <file>]` | Print the race report and verdict, optionally updating a cumulative standings JSON file |
| `closecall` | `<race.json>` | Print band status and single-judge or dual-judge panel adjudication details for a race |
| `adopt` | `<race.json> <patches-dir>` | Write the losing attempt's `patch` text into `<patches-dir>/<slug-task>-<slug-loser-model>.patch` (`DH-3`) |
| `discard` | `<race.json> <patches-dir>` | Remove any previously adopted loser `.patch` file for a decided race (`DH-3`) |
| `standings` | `<standings.json>` | Render cumulative wins, losses, no-results, and token spend from a standings file (`DH-4`) |
| `series` | `<series.json>` | Score a first-to-N series between two models and report the series winner and race split |
| `flip` | `<series.json>` | Compare scratch winners against handicap-adjusted winners (`flipped`, `held`, `no result`) |

### Championship table and spend commands (3 commands, take `<championship.json>`)

| Command | Purpose |
| --- | --- |
| `championship` | Overall series wins, race wins, and aggregate time margin table across all models |
| `totals` | Races entered, won, lost, no-results, and total token spend up to each series decision |
| `pairing` | Head-to-head series and race records for every model pair that met in the championship |

### Championship momentum, lead, and trajectory commands (27 commands, take `<championship.json>`)

| Family | Commands | What they classify across decided series |
| --- | --- | --- |
| Opening & closing (6) | `comeback`, `opening`, `closing`, `firststrike`, `firststand`, `laststand` | Whether the champion dropped race 1 (`comeback`), opened/closed in a multi-win run, or faced a multi-win loser stand |
| Competitiveness & swings (6) | `distance`, `rally`, `deadlock`, `turns`, `leadchanges`, `crossings` | Full-distance deciders, trailing deficits (`rally`), tied tallies (`deadlock`), momentum handovers, and lead changes |
| Run lengths (2) | `streak`, `cluster` | Longest consecutive win streak by the champion (`streak`) or by the defeated opponent (`cluster`) |
| Ahead & behind spells (7) | `lead`, `tenure`, `depths`, `spells`, `reigns`, `firstreign`, `lastreign` | Peak lead vs final margin, races spent ahead vs behind, deepest deficit, and unbroken/broken reigns and spells |
| Timing & symmetry (6) | `place`, `peak`, `recovery`, `outset`, `tail`, `crosshead` | Where the longest reign stood (`place`), where peak lead arrived (`peak`), speed of deficit recovery (`recovery`), and reign-vs-spell comparisons (`outset`, `tail`, `crosshead`) |

## Multi-Agent Pipeline Integration Recipe

To benchmark two coding agents, two prompt revisions, or two model providers head to head on the same task suite on Unix runners, wire `deadheat` into your evaluation harness.

1. **Author a `duel-spec.json`.** Point each attempt's `command` at an isolated wrapper script. Inside its working directory (`<workspace>/attempt-1` or `attempt-2`), have the script run your test suite, write `git diff > patch.diff` so `deadheat` captures the exact diff text (`DH-1`), and write `{"result": "green", "tokens": 48210}` (or `"red"`) to `result.json`.
2. **Execute, score, and salvage loser patches.** Run `deadheat run duel-spec.json`, inspect `deadheat race duel-record.json --standings standings.json`, and optionally call `deadheat adopt duel-record.json patches/` to archive the defeated attempt's diff from its funeral (`DH-3`).
3. **Aggregate multi-task series.** Bundle repeated runs per task into `series.json` files (`wins_target: 3` filters single-run noise) and group the benchmark suite into `championship.json`. Run `deadheat championship`, `deadheat totals`, and `deadheat pairing` to compare win rates and decided-walk token spend.
