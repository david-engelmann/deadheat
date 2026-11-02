# deadheat Contract

This Contract is the executable spec for deadheat. Every obligation is checked by a named test or by a named check. The Contract is reported as met or not met in the README programme status.

## Setup obligations

- SETUP-1. The repo carries AGENTS.md with the canonical template verbatim
  above its marked line and the repo annex below it. Test,
  tests/test_setup.py.
- SETUP-2. The README programme status states the closed round count out
  of 100 and reports this Contract as met or not met. Test,
  tests/test_setup.py.
- SETUP-3. The codepoint scan at scripts/codepoint_scan.py counts U+2014
  and U+2013 in every Markdown and text file in the repo and requires
  zero of each. The scan runs in the local gate and in CI.
- SETUP-4. Every fixture and example committed to this repo is synthetic
  and written for this repo. Nothing real is ever committed.

## First slice obligations, stub form

The full Contract for the first slice is the first build act of this repo. These stubs state the obligations the first slice will be held to, in the form they will be extended.

- DH-1. A race holds exactly two attempts at the same task, each attempt in its own environment, and each attempt recording its suite result, its elapsed time, and its token spend where the runner reports it. Test, the first slice tests.
- DH-2. The winner is the first attempt whose full suite is green. If both suites finish green the earlier finish wins, and if neither goes green there is no winner and the race says so. Test, the first slice tests.
- DH-3. The losing attempt gets a rendered funeral naming the task, the winner, and the margin, and its diff is kept as a patch that can be adopted or discarded. Test, the first slice tests.
- DH-4. Season standings persist across races in one standings file, recording wins, losses, and spend per model. Test, the first slice tests.
