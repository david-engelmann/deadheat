# Deadheat FAQ

## How does Deadheat differ from Myriad and Grandjury?

- **`deadheat`** races **two** attempts at one task with an objective pass/fail verification check and elapsed clock to declare a winner.
- **`myriad`** maps an open-ended problem into a tree of hypotheses (`open`, `taken`, `abandoned`) without ever declaring a winner.
- **`grandjury`** polls a panel of models on a discrete multiple-choice question to measure consensus, abstention, and dissent.

## Where does `deadheat run` capture patch diffs and token counts?

Inside each attempt's lane directory (`<workspace>/attempt-1` and `<workspace>/attempt-2`), the runner checks for an optional `result.json` (`{"result": "green" | "red" | "unfinished", "tokens": 12345}`) and an optional `patch.diff` file written by your harness command (`DH-1`). When `result.json` is absent, `deadheat run` falls back to the command's exit status (`0` -> `green`, non-zero -> `red`). Note that `deadheat run` invokes `/bin/sh -c`, so native Windows environments without a POSIX `/bin/sh` record attempts as `unfinished` while all thirty-five reading commands work natively on Windows.

## Why do series and totals stop counting after `wins_target` is reached?

Under `DH-6` and `DH-17`, once one model reaches `wins_target` (for example 3 wins in a first-to-3 series), the series is decided. Any trailing races stored in the JSON array after the deciding race are ignored by championship tallies and token counters so extra diagnostic runs never skew competitive metrics.

## What do the exit codes mean?

- **`0`** Race executed cleanly (`run`), patch adopted/discarded (`adopt`/`discard`), or report printed cleanly.
- **`1`** Malformed JSON specification or record file refused with structured defect lines on `stdout`, or `adopt`/`discard` invoked on a valid race that has no winner (`DH-3`).
- **`2`** Unknown command or invalid CLI flags on `stderr`.
