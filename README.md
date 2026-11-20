# deadheat

Deadheat races two attempts at the same task, one attempt per model, and decides the winner from the recorded results.

Deadheat is a Rust library with a command line tool of the same name. It reads recorded race, series, and championship files in JSON, decides each race from the record alone, and prints readings over the records. A local runner can build a race record by running one shell command per attempt, each in its own directory, and taking the suite result, the elapsed time, the token spend, and the patch from what the command leaves behind.

[![ci](https://github.com/david-engelmann/deadheat/actions/workflows/ci.yml/badge.svg)](https://github.com/david-engelmann/deadheat/actions/workflows/ci.yml)

```text
totals
prime-1, 14 races, 6 wins, 1 losses, 7 no results, 1017000 tokens
shadow-1, 14 races, 1 wins, 6 losses, 7 no results, 1073200 tokens
totals, 2 series, 14 races, 2 models
```

## Install

```sh
git clone https://github.com/david-engelmann/deadheat.git
cd deadheat
cargo install --path .
```

## Quickstart

Run the totals command over the championship example from the root of the clone.

```sh
deadheat totals examples/championship.json
```

```text
totals
prime-1, 14 races, 6 wins, 1 losses, 7 no results, 1017000 tokens
shadow-1, 14 races, 1 wins, 6 losses, 7 no results, 1073200 tokens
totals, 2 series, 14 races, 2 models
```

## Usage

Deadheat decides the winner from the record alone. A green attempt beats an attempt that did not go green. Between two green attempts the earlier finish wins, and a per attempt adjustment in seconds is applied to the finish before the comparison. An exact tie goes to the attempt that stands first in the record. When the attempts land in the close call band, a recorded adjudication entry with a confidence of four fifths or higher decides the race, and a panel of recorded entries decides the race when the entries meeting that threshold agree. A close call that no entry decides has no winner. The commands work offline over the files they are given.

The race command prints the race report and the funeral for one recorded race, and with a standings file named it updates that file with the result.

```sh
deadheat race examples/shadow-wins.json
```

```text
race land the settings migration
prime-1 in home worktree, red in 812.4 seconds, 152300 tokens
shadow-1 in shadow worktree, green in 640.75 seconds, 121900 tokens
verdict, shadow-1 wins
funeral for land the settings migration
winner shadow-1 in shadow worktree went green in 640.75 seconds
loser prime-1 in home worktree went red in 812.4 seconds
margin 171.65 seconds between the two finishes
```

The run command takes a race specification file, runs its two attempts in local directories under the workspace the specification names, writes the race record to the path it names, and prints the report and the funeral over the record it built. The specification at examples/duel-spec.json names the task, the workspace, the record file, and the two attempts with their models, environments, and shell commands.

```json
{
 "attempts": [
  {
   "command": "printf 'attempt one patch line\n' > patch.diff",
   "environment": "local shell",
   "model": "duel-1"
  },
  {
   "command": "printf 'attempt two patch line\n' > patch.diff; exit 1",
   "environment": "local shell",
   "model": "duel-2"
  }
 ],
 "record": "duel-record.json",
 "task": "the example duel",
 "workspace": "duel-workspace"
}
```

```sh
deadheat run examples/duel-spec.json
```

```text
race the example duel
duel-1 in local shell, green in 0.03774 seconds, tokens unreported
duel-2 in local shell, red in 0.028695 seconds, tokens unreported
verdict, duel-1 wins
funeral for the example duel
winner duel-1 in local shell went green in 0.03774 seconds
loser duel-2 in local shell went red in 0.028695 seconds
margin 0.009045 seconds between the two finishes
```

The seconds in that output are the runner's measured figures of one run and differ between runs, while the verdict of this example does not, because a green attempt beats a red attempt at any figures.

The closecall command states whether one race sits in the close call band and what decided it.

```sh
deadheat closecall examples/close-call-decided.json
```

```text
closecall tighten the retry budget
in the band, both attempts went green and the gap of 12 seconds is within five percent of the slower finish of 300 seconds
adjudication by judge-2 chose prime-1 at confidence 0.87, the race is decided for prime-1
verdict, prime-1 wins
```

The series command reads a series file of races between the same two models and prints the result of each race and the winner of the series.

```sh
deadheat series examples/series-decided.json
```

```text
series the nightly import saga
prime-1 against shadow-1, first to 3 wins
race 1, import race one, prime-1 wins
tally, prime-1 1 wins, shadow-1 0 wins, 0 no results
race 2, import race two, shadow-1 wins
tally, prime-1 1 wins, shadow-1 1 wins, 0 no results
race 3, import race three, no result
tally, prime-1 1 wins, shadow-1 1 wins, 1 no results
race 4, import race four, prime-1 wins
tally, prime-1 2 wins, shadow-1 1 wins, 1 no results
race 5, import race five, prime-1 wins
tally, prime-1 3 wins, shadow-1 1 wins, 1 no results
race 6, import race six, beyond the decision
series winner, prime-1
```

The championship command reads a championship file naming series files and prints the championship table over them.

```sh
deadheat championship examples/championship.json
```

```text
championship
series 1, the recorded close call pilot
prime-1 against shadow-1, first to 3 wins
series winner, prime-1
series 2, the recorded close call panel
prime-1 against shadow-1, first to 3 wins
series winner, prime-1
championship table
prime-1, 2 series won, 2 series stood in, 6 race wins, 7 no results, aggregate margin 45.25 seconds
shadow-1, 0 series won, 2 series stood in, 1 race wins, 7 no results, aggregate margin 10 seconds
```

The flip command reads a series file and stands the scratch winner of each race beside the adjusted winner, stating whether the race flipped, held, or had no result.

```sh
deadheat flip examples/handicap-series.json
```

```text
series the ledger import season
prime-1 against shadow-1, first to 2 wins
race 1, ledger import one, scratch prime-1, adjusted prime-1, held
race 2, balance the ledger import, scratch prime-1, adjusted shadow-1, flipped
race 3, ledger import three, scratch shadow-1, adjusted shadow-1, held
flips, 3 races, 1 flipped, 2 held, 0 no results
```

The totals command reads a championship file and tallies the counted races per model, the races entered, won, lost, and left without a result, and the token spend the attempts reported.

```sh
deadheat totals examples/championship.json
```

```text
totals
prime-1, 14 races, 6 wins, 1 losses, 7 no results, 1017000 tokens
shadow-1, 14 races, 1 wins, 6 losses, 7 no results, 1073200 tokens
totals, 2 series, 14 races, 2 models
```

The pairing command reads a championship file and states the head to head record of each pair of models over the series they stood in together.

```sh
deadheat pairing examples/championship.json
```

```text
pairings
pairing, prime-1, against, shadow-1
series read, 2
races read, 14
prime-1, 2 series won, 6 race wins, 7 no results, aggregate margin 45.25 seconds
shadow-1, 0 series won, 1 race wins, 7 no results, aggregate margin 10 seconds
pairings, 1 pairings, 2 series, 14 races
```

The comeback command reads a championship file and reports the series that were won from behind, standing the decided races of each beside its winner and its race split.

```sh
deadheat comeback examples/championship.json
```

```text
comebacks
comebacks, 2 series, 0 from behind, 2 from the front
```

The distance command reads a championship file and reports the series that went the distance, whose loser stood one win short of the wins target when the decision fell.

```sh
deadheat distance examples/championship.json
```

```text
distance
distance, 2 series, 0 the distance, 1 without reply, 1 in between, 0 no winner
```

The rally command reads a championship file and reports the series whose winner ever stood behind on the tally of decided races, standing the decided races of each beside the deepest the winner stood behind and its race split.

```sh
deadheat rally examples/championship.json
```

```text
rally
rally, 2 series, 0 rally, 2 wire to wire, 0 no winner
```

The deadlock command reads a championship file and reports the series that stood deadlocked, whose two tallies stood level after some decided race before the decision, standing the decided races of each beside the count of its decided races that left the tallies level and its race split.

```sh
deadheat deadlock examples/championship.json
```

```text
deadlock
deadlock, 2 series, 0 deadlocked, 2 clear, 0 no winner
```

The streak command reads a championship file and reports the series that stood strung, whose winner took two or more decided races in a row on the way to the decision, standing the decided races of each beside the longest run of its winner and its race split.

```sh
deadheat streak examples/championship.json
```

```text
streak
series 1, the recorded close call pilot
prime-1 against shadow-1, first to 3 wins
race 1, pin the cache key, prime-1 wins
race 5, name the worker pool, prime-1 wins
race 7, shrink the prompt budget, shadow-1 wins
race 9, sort the seed list, prime-1 wins
longest run of 2 consecutive decided race wins
series winner, prime-1, prime-1 3 race wins, shadow-1 1 race wins, 5 no results
series 2, the recorded close call panel
prime-1 against shadow-1, first to 3 wins
race 1, pin the cache key, prime-1 wins
race 2, trim the import list, prime-1 wins
race 5, name the worker pool, prime-1 wins
longest run of 3 consecutive decided race wins
series winner, prime-1, prime-1 3 race wins, shadow-1 0 race wins, 2 no results
streak, 2 series, 2 strung, 0 single, 0 no winner
```

The cluster command reads a championship file and reports the series that stood clustered, whose loser took two or more decided races in a row on the way to the decision, standing the decided races of each beside the longest run of its loser and its race split.

```sh
deadheat cluster examples/championship.json
```

```text
cluster
cluster, 2 series, 0 clustered, 2 scattered, 0 no winner
```

The closing command reads a championship file and reports the series that stood in a run, whose winner took the deciding race at the end of a run of two or more decided race wins, standing the decided races of each beside the closing run of its winner and its race split.

```sh
deadheat closing examples/championship.json
```

```text
closing
series 2, the recorded close call panel
prime-1 against shadow-1, first to 3 wins
race 1, pin the cache key, prime-1 wins
race 2, trim the import list, prime-1 wins
race 5, name the worker pool, prime-1 wins
closing run of 3 consecutive decided race wins
series winner, prime-1, prime-1 3 race wins, shadow-1 0 race wins, 2 no results
closing, 2 series, 1 in a run, 1 alone, 0 no winner
```

The opening command reads a championship file and reports the series that stood in a run, whose winner opened the decided record with a run of two or more decided race wins, standing the decided races of each beside the opening run of its winner and its race split.

```sh
deadheat opening examples/championship.json
```

```text
opening
series 1, the recorded close call pilot
prime-1 against shadow-1, first to 3 wins
race 1, pin the cache key, prime-1 wins
race 5, name the worker pool, prime-1 wins
race 7, shrink the prompt budget, shadow-1 wins
race 9, sort the seed list, prime-1 wins
opening run of 2 consecutive decided race wins
series winner, prime-1, prime-1 3 race wins, shadow-1 1 race wins, 5 no results
series 2, the recorded close call panel
prime-1 against shadow-1, first to 3 wins
race 1, pin the cache key, prime-1 wins
race 2, trim the import list, prime-1 wins
race 5, name the worker pool, prime-1 wins
opening run of 3 consecutive decided race wins
series winner, prime-1, prime-1 3 race wins, shadow-1 0 race wins, 2 no results
opening, 2 series, 2 in a run, 0 alone, 0 from behind, 0 no winner
```

The turns command reads a championship file and reports the series that stood turned, whose decided record changed hands two or more times, standing the decided races of each beside the changes of hand of the series and its race split.

```sh
deadheat turns examples/championship.json
```

```text
turns
series 1, the recorded close call pilot
prime-1 against shadow-1, first to 3 wins
race 1, pin the cache key, prime-1 wins
race 5, name the worker pool, prime-1 wins
race 7, shrink the prompt budget, shadow-1 wins
race 9, sort the seed list, prime-1 wins
changes of hand 2 over the decided record
series winner, prime-1, prime-1 3 race wins, shadow-1 1 race wins, 5 no results
turns, 2 series, 1 turned, 1 steady, 0 no winner
```

The lead command reads a championship file and reports the series that stood wide, whose largest lead over the decided record stood above the winning margin of the series, standing the decided races of each beside the largest lead of the series and its race split.

```sh
deadheat lead examples/championship.json
```

```text
lead
lead, 2 series, 0 wide, 2 narrow, 0 no winner
```

The crossings command reads a championship file and reports the series that stood through, whose count of crossings over the decided record stood equal to the count of its changes of hand, standing the decided races of each beside the crossings of the series and its race split.

```sh
deadheat crossings examples/championship.json
```

```text
crossings
series 2, the recorded close call panel
prime-1 against shadow-1, first to 3 wins
race 1, pin the cache key, prime-1 wins
race 2, trim the import list, prime-1 wins
race 5, name the worker pool, prime-1 wins
crossings 0 over the decided record
series winner, prime-1, prime-1 3 race wins, shadow-1 0 race wins, 2 no results
crossings, 2 series, 1 through, 1 short, 0 no winner
```

The tenure command reads a championship file and reports the series that stood ahead, whose winner stood in front after more decided races than its loser did over the decided record, standing the decided races of each beside the front count of the series and its race split.

```sh
deadheat tenure examples/championship.json
```

```text
tenure
series 1, the recorded close call pilot
prime-1 against shadow-1, first to 3 wins
race 1, pin the cache key, prime-1 wins
race 5, name the worker pool, prime-1 wins
race 7, shrink the prompt budget, shadow-1 wins
race 9, sort the seed list, prime-1 wins
front count 4 over the decided record
series winner, prime-1, prime-1 3 race wins, shadow-1 1 race wins, 5 no results
series 2, the recorded close call panel
prime-1 against shadow-1, first to 3 wins
race 1, pin the cache key, prime-1 wins
race 2, trim the import list, prime-1 wins
race 5, name the worker pool, prime-1 wins
front count 3 over the decided record
series winner, prime-1, prime-1 3 race wins, shadow-1 0 race wins, 2 no results
tenure, 2 series, 2 ahead, 0 level, 0 behind, 0 no winner
```

The depths command reads a championship file and reports the series that stood deep, whose winner's largest deficit over the decided record stood above the margin the series was won by, standing the decided races of each beside the deepest deficit of the series and its race split.

```sh
deadheat depths examples/championship.json
```

```text
depths
depths, 2 series, 0 deep, 0 matched, 2 shallow, 0 no winner
```

The spells command reads a championship file and reports the series that stood broken, whose winner's longest spell behind over the decided record stood below the behind count of the series, standing the decided races of each beside the longest spell and behind count of the series and its race split.

```sh
deadheat spells examples/championship.json
```

```text
spells
spells, 2 series, 0 unbroken, 0 broken, 2 never behind, 0 no winner
```

The reigns command reads a championship file and reports the series that stood broken, whose winner's longest reign over the decided record stood below the front count of the series, standing the decided races of each beside the longest reign and front count of the series and its race split.

```sh
deadheat reigns examples/championship.json
```

```text
reigns
reigns, 2 series, 2 unbroken, 0 broken, 0 no winner
```

The place command reads a championship file and reports the series that stood late, whose winner's deepest deficit over the decided record was first stood at a place above the depth itself, standing the decided races of each beside the depth and place of the series and its race split.

```sh
deadheat place examples/championship.json
```

```text
place
place, 2 series, 0 early, 0 late, 2 never behind, 0 no winner
```

The peak command reads a championship file and reports the series that stood late, whose winner's largest lead over the decided record was first stood at a place above the lead itself, standing the decided races of each beside the largest lead and place of the series and its race split.

```sh
deadheat peak examples/championship.json
```

```text
peak
peak, 2 series, 2 early, 0 late, 0 no winner
```

The recovery command reads a championship file and reports the series that stood delayed, whose winner's answer to the deepest deficit over the decided record stood above the depth plus one, standing the decided races of each beside the deepest deficit and answer of the series and its race split.

```sh
deadheat recovery examples/championship.json
```

```text
recovery
recovery, 2 series, 0 immediate, 0 delayed, 2 never behind, 0 no winner
```

The laststand command reads a championship file and reports the series that stood held, whose loser's last run over the decided record stood at two or more, standing the decided races of each beside the last stand of the series and its race split.

```sh
deadheat laststand examples/championship.json
```

```text
laststand
laststand, 2 series, 0 held, 1 single, 1 without reply, 0 no winner
```

The firststand command reads a championship file and reports the series that stood held, whose loser's first run over the decided record stood at two or more, standing the decided races of each beside the first stand of the series and its race split.

```sh
deadheat firststand examples/championship.json
```

```text
firststand
firststand, 2 series, 0 held, 1 single, 1 without reply, 0 no winner
```

The firststrike command reads a championship file and reports the series that stood in a run, whose winner's first run over the decided record stood at two or more, standing the decided races of each beside the first strike of the series and its race split.

```sh
deadheat firststrike examples/championship.json
```

```text
firststrike
series 1, the recorded close call pilot
prime-1 against shadow-1, first to 3 wins
race 1, pin the cache key, prime-1 wins
race 5, name the worker pool, prime-1 wins
race 7, shrink the prompt budget, shadow-1 wins
race 9, sort the seed list, prime-1 wins
first strike 2 over the decided record
series winner, prime-1, prime-1 3 race wins, shadow-1 1 race wins, 5 no results
series 2, the recorded close call panel
prime-1 against shadow-1, first to 3 wins
race 1, pin the cache key, prime-1 wins
race 2, trim the import list, prime-1 wins
race 5, name the worker pool, prime-1 wins
first strike 3 over the decided record
series winner, prime-1, prime-1 3 race wins, shadow-1 0 race wins, 2 no results
firststrike, 2 series, 2 in a run, 0 alone, 0 no winner
```

The firstreign command reads a championship file and reports the series that stood held, whose winner's first run of ahead standings over the decided record stood at two or more, standing the decided races of each beside the first reign of the series and its race split.

```sh
deadheat firstreign examples/championship.json
```

```text
firstreign
series 1, the recorded close call pilot
prime-1 against shadow-1, first to 3 wins
race 1, pin the cache key, prime-1 wins
race 5, name the worker pool, prime-1 wins
race 7, shrink the prompt budget, shadow-1 wins
race 9, sort the seed list, prime-1 wins
first reign 4 over the decided record
series winner, prime-1, prime-1 3 race wins, shadow-1 1 race wins, 5 no results
series 2, the recorded close call panel
prime-1 against shadow-1, first to 3 wins
race 1, pin the cache key, prime-1 wins
race 2, trim the import list, prime-1 wins
race 5, name the worker pool, prime-1 wins
first reign 3 over the decided record
series winner, prime-1, prime-1 3 race wins, shadow-1 0 race wins, 2 no results
firstreign, 2 series, 2 held, 0 single, 0 no winner
```

The standings command prints the standings table held in a standings file. The adopt command writes the losing attempt patch into a patches folder and the discard command removes it.

## Status

Seeded in October 2026. Setup has landed, and Rounds 1 to 34 have closed, with all slices merged on main. Part of the Spectacle line in the 39 repo storefront perfection programme.

## Programme status

- Closed perfection rounds, 34 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed. Seeding and setup are not rounds.
- Contract, SETUP-1 to SETUP-4 and DH-1 to DH-6 met. DH-1 to DH-4 stand in full form in CONTRACT.md with the Round 1 repair amendments, and Round 1 closed on the merged slice 2 after three reviews, a repair pass, and an approving re-review.
- Slice 2 has landed. It carries the recorded race loader, the local runner, the winner rule, the race report and the funeral, the loser patch adopt and discard operations, the season standings, and the race, adopt, discard, and standings commands, with two synthetic recorded races and their golden renderings under examples.
- Contract, DH-5 and DH-6 met. They stand in full form in CONTRACT.md with the Round 2 repair amendments, and Round 2 closed on the merged slice 3 after three reviews, a repair pass, and an approving re-review.
- Slice 3 has landed. It carries the close call band and the recorded adjudication, and the series over recorded races, with the closecall and series commands, and with synthetic recorded races and series and their golden renderings under examples.
- Contract, DH-7 and DH-8 met. They stand in full form in CONTRACT.md with the Round 3 repair amendments, and Round 3 closed on the merged slice 4 after three reviews, a repair pass, and an approving re-review.
- Slice 4 has landed. It carries the adjudication seat over the two decision model routes, the live entry path into the race record, and the recorded close call pilot over both models, ten synthetic recorded races with their envelopes, adjudicated records, and run record under examples.
- Contract, DH-9 and DH-10 met. They stand in full form in CONTRACT.md with the Round 4 repair amendments, and Round 4 closed on the merged slice 5 after three reviews, a repair pass, and an approving re-review.
- Slice 5 has landed. It carries the panel reading over two recorded adjudication entries, the panel join over the Round 3 pilot records, and the recorded panel pilot over both models, ten panel records with their closecall goldens and the panel series under examples.
- Contract, DH-11 and DH-12 met. They stand in full form in CONTRACT.md under the Round 5 obligations, and Round 5 closed on the merged slice 6 after three reviews, no repair pass, and an owner gate discharging the standards verification condition with three mutation spot checks.
- Slice 6 has landed. It carries the handicap, a per attempt adjustment in seconds read as an adjusted finish by the winner rule and the close call band, with the handicapped example race and the handicap series and their goldens under examples.
- Contract, DH-13 and DH-14 met. They stand in full form in CONTRACT.md under the Round 6 obligations, and Round 6 closed on the merged slice 7 after three reviews approving with no findings.
- Slice 7 has landed. It carries the championship, a reading across recorded series naming the series each model took and the championship table over series won, race wins, and the aggregate margin, with the championship example over the two recorded series and its golden under examples.
- Contract, DH-15 and DH-16 met. They stand in full form in CONTRACT.md under the Round 7 obligations, and Round 7 closed on the merged slice 8 after three reviews approving with no findings.
- Slice 8 has landed. It carries the flip reading over a recorded series, the scratch winner and the adjusted winner of each race and the standing of the race as flipped, held, or no result, with the closing counts of races, flipped, held, and no results, with the flip example over the handicap series and its golden under examples, and the flip command beside race, closecall, series, and championship.
- Contract, DH-17 and DH-18 met. They stand in full form in CONTRACT.md under the Round 8 obligations, and Round 8 closed on the merged slice 9 after three reviews, no repair pass, and the standards mutation condition discharged on executed runs.
- Slice 9 has landed. It carries the totals reading over a recorded championship, the races entered, won, lost, and left without a result by each model and the token spend its attempts reported, counted under the counting walk that stops at each series decision, with the closing counts of series, races, and models, with the totals example over the standing championship and its golden under examples, and the totals command beside race, closecall, series, championship, and flip.
- Contract, DH-19 and DH-20 met. They stand in full form in CONTRACT.md under the Round 9 obligations, and Round 9 closed on the merged slice 10 after three reviews, a repair pass, and a discharging re-review.
- Slice 10 has landed. It carries the pairing reading over a recorded championship, the head to head record of each pair of models over the series they stood in together, with the closing counts of pairings, series, and races, with the pairing example over the standing championship and its golden under examples, and the pairing command beside race, closecall, series, championship, flip, and totals, and the fencing of the printed fields at the library surface under FIX-DH-16, and the README brought to the fleet README standard.
- Contract, DH-21 and DH-22 met. They stand in full form in CONTRACT.md under the Round 10 obligations, and Round 10 closed on the merged slice 11 after three reviews approving with no findings.
- Slice 11 has landed. It carries the comeback reading over a recorded championship, the classification of each series by its first decided race as won from behind or won from the front, one section for each series won from behind naming its decided races in race order and the loss of its winner in the first decided race and its winner with its race split, with the closing counts of series, from behind, and from the front, with the comeback example over the standing championship in its empty form and the constructed comeback example and their goldens under examples, and the comeback command beside race, closecall, series, championship, flip, totals, and pairing.
- Contract, DH-23 and DH-24 met. They stand in full form in CONTRACT.md under the Round 11 obligations, and Round 11 closed on the merged slice 12 after three reviews, with the standards lane's two findings repaired and corrected before landing.
- Slice 12 has landed. It carries the distance reading over a recorded championship, the classification of each series by the wins of its loser when the decision fell as standing the distance, standing without reply, or standing in between, one section for each series gone the distance naming its decided races in race order and its winner with its race split, with the closing counts of series, the distance, without reply, in between, and no winner, with the distance example over the standing championship in its empty form and the constructed distance example and their goldens under examples, and the distance command beside race, closecall, series, championship, flip, totals, pairing, and comeback.
- Contract, DH-25 to DH-28 met, with FIX-DH-17, FIX-DH-18, and FIX-DH-19. They stand in full form in CONTRACT.md under the Round 12 obligations, and Round 12 closed on the merged slice 13 after three reviews, with the repair pass landed before the merge.
- Slice 13 has landed. It carries the format version rule under FIX-DH-17, every format this repo owns standing at version 1 with the version refused in the phrasing of the surface that reads it, the formats reference at FORMATS.md restating every shape in one place, the race specification and the run command that runs its two attempts through the local runner and writes its race record, with the duel example and its pins under examples, the rally reading over a recorded championship, the classification of each decided series by whether its winner ever stood behind on the tally of decided races as a rally or wire to wire, one section for each rally series naming its decided races in race order and the deepest its winner stood behind and its winner with its race split, with the closing counts of series, rally, wire to wire, and no winner, with the rally example over the standing championship in its empty form and the constructed rally example and their goldens under examples, the survey championship over six constructed series at drawn realistic shapes with its goldens under examples, and the switchyard later obligation retired under FIX-DH-18, and the run and rally commands beside race, closecall, series, championship, flip, totals, pairing, comeback, and distance.
- Contract, DH-29 to DH-31 met, with FIX-DH-20. They stand in full form in CONTRACT.md under the Round 13 obligations, and Round 13 closed on the merged slice 14 after three reviews, with the record corrections settled by execution before the merge.
- Slice 14 has landed. It carries the deadlock reading over a recorded championship, the classification of each decided series by whether its two tallies ever stood level on the way to the decision as deadlocked or clear, one section for each deadlocked series naming its decided races in race order and the count of its decided races that left the tallies level and its winner with its race split, with the closing counts of series, deadlocked, clear, and no winner, with the deadlock example over the standing championship in its empty form and the constructed deadlock example and their goldens under examples, the league championship over ten constructed series at drawn realistic shapes with its goldens under examples, the specification loader pins completed under FIX-DH-20, and the deadlock command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, and rally.
- Contract, DH-32 and DH-33 met. They stand in full form in CONTRACT.md under the Round 14 obligations, and Round 14 closed on the merged slice 15 after three reviews, with the record corrections settled by execution before the merge.
- Slice 15 has landed. It carries the streak reading over a recorded championship, the classification of each decided series by the longest run of consecutive decided races won by its winner as strung or single, one section for each strung series naming its decided races in race order and the longest run of its winner and its winner with its race split, with the closing counts of series, strung, single, and no winner, with the streak example over the standing championship in section form and the constructed streak example and their goldens under examples, the closing counts helper of the test suite standing in one home in the shared test module, and the streak command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, and deadlock.
- Contract, DH-34 and DH-35 met. They stand in full form in CONTRACT.md under the Round 15 obligations, and Round 15 closed on the merged slice 16 after three reviews approving with findings and no repair owed.
- Slice 16 has landed. It carries the cluster reading over a recorded championship, the classification of each decided series by the longest run of consecutive decided races won by its loser as clustered or scattered, one section for each clustered series naming its decided races in race order and the longest run of its loser and its winner with its race split, with the closing counts of series, clustered, scattered, and no winner, with the cluster example over the standing championship in its empty form and the constructed cluster example and their goldens under examples, and the cluster command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, and streak.
- Contract, DH-36 and DH-37 met. They stand in full form in CONTRACT.md under the Round 16 obligations, and Round 16 closed on the merged slice 17 after three reviews, with the two test text Minors of the reviews repaired in the slice.
- Slice 17 has landed. It carries the closing reading over a recorded championship, the classification of each decided series by the closing run of consecutive decided races won by its winner as in a run or alone, one section for each series in a run naming its decided races in race order and the closing run of its winner and its winner with its race split, with the closing counts of series, in a run, alone, and no winner, with the closing example over the standing championship with the panel series in its section alone and the constructed closing example and their goldens under examples, and the closing command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, and cluster.
- Contract, DH-38 and DH-39 met. They stand in full form in CONTRACT.md under the Round 17 obligations, and Round 17 closed on the merged slice 18 after three reviews, with the two Minors of the reviews repaired in the slice.
- Slice 18 has landed. It carries the opening reading over a recorded championship, the classification of each decided series by the opening run of consecutive decided races won by its winner as in a run, alone, or from behind, one section for each series in a run naming its decided races in race order and the opening run of its winner and its winner with its race split, with the closing counts of series, in a run, alone, from behind, and no winner, with the opening example over the standing championship with both of its series in section form and the constructed opening example and their goldens under examples, and the opening command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, and closing.
- Contract, the Round 18 fold and the repair amendment FIX-DH-23 met. They stand in full form in CONTRACT.md under the Round 18 obligations, and Round 18 closed on the merged slice 19 after three reviews, with the fold's neutrality proof executed at the fold commit and re-executed by the red team.
- Slice 19 has landed. It carries the fold of the longest run computation of the streak reading and the cluster reading into the one helper of the championship module, both readings reading the longest run of a series from the one home from this round, with no command added, no figure moved, and no golden changed, under the repair amendment FIX-DH-23.
- Contract, DH-40 and DH-41 met. They stand in full form in CONTRACT.md under the Round 19 obligations, and Round 19 closed on the merged slice 20 after three reviews, with the pin comments restated to their executed postures in the slice.
- Slice 20 has landed. It carries the turns reading over a recorded championship, the classification of each decided series by the count of the changes of hand over its decided record as turned or steady, one section for each turned series naming its decided races in race order and the changes of hand of the series and its winner with its race split, with the closing counts of series, turned, steady, and no winner, with the turns example over the standing championship with the pilot series in its section alone and the constructed turns example and their goldens under examples, and the turns command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, and opening.
- Contract, DH-42 and DH-43 met. They stand in full form in CONTRACT.md under the Round 20 obligations, and Round 20 closed on the merged slice 21 after three reviews, with the red team clean on every charge, the formatting repair landed in the slice, and the two CodeRabbit findings repaired in the fix-forward under FIX-DH-25.
- Slice 21 has landed. It carries the lead reading over a recorded championship, the classification of each decided series by the largest lead either model held over its decided record read against the winning margin of the series as wide or narrow, one section for each wide series naming its decided races in race order and the largest lead of the series and its winner with its race split, with the closing counts of series, wide, narrow, and no winner, with the lead example over the standing championship in its empty form and the constructed lead example and their goldens under examples, and the lead command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, opening, and turns.
- Contract, DH-44 and DH-45, with FIX-DH-26, met. They stand in full form in CONTRACT.md under the Round 21 obligations, and Round 21 closed on the merged slice 22 after three reviews, with the red team approving on recomputation of every crossings closing, the relations pin's winner line parse repaired to the series report's final tally, and the M-F clauses of L-4, L-16, and L-17 restated to their computed postures.
- Slice 22 has landed. It carries the crossings reading over a recorded championship, the classification of each decided series by the count of the crossings over its decided record read against the count of its changes of hand as through or short, one section for each through series naming its decided races in race order and the crossings of the series and its winner with its race split, with the closing counts of series, through, short, and no winner, with the crossings example over the standing championship with the panel series in its section alone and the constructed crossings example and their goldens under examples, and the crossings command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, opening, turns, and lead.
- Contract, DH-46 and DH-47, with FIX-DH-27, met. They stand in full form in CONTRACT.md under the Round 22 obligations, and Round 22 closed on the merged slice 23 after three reviews, with the three reviews approving with findings and every finding cured in the repair commit, FIX-DH-27 restating the DH-47 examples clause in place with the wrong sentence confined to the amendment's quotation, pin M-21 asserting the restated form with its split executed both ways, and the M-13 and M-15 comments restated to their true and executed postures.
- Contract, DH-48 and DH-49 met. They stand in full form in CONTRACT.md under the Round 23 obligations, and Round 23 closed on the merged slice 24 after three reviews, with the standards review approving with findings at two Observations, the spec review approving with findings at one Major cured in the slice by the fencing repair and one Minor standing against the design record, the red team approving with findings at three Observations on executed recomputation, and CodeRabbit returning no comments.
- Contract, the Round 24 tally fold, with FIX-DH-28, met. They stand in full form in CONTRACT.md under the Round 24 obligations, and Round 24 closed on the merged slice 25 after three reviews, with FIX-DH-28 standing spent on the fold, the spec review approving with findings at two Majors, three Minors, and two Observations, its census Major standing against the design record, whose 894 stands as its own arithmetic erratum against the settled census of 893, and its evidence Major discharged by the executions, the standards review approving with findings at one Minor and one Observation, its Minor on the positional return dismissed as the contracted form FIX-DH-28 names, and the red team approving with findings at two Minors and three Observations, neutrality standing byte identical over 96 renderings and both identities attacked exhaustively with no reachable parting, the executions standing at 39 sets with 38 exact and one moved set, the deadlocks M-A set failing D-2, D-3, and D-9 against its stated D-2 and D-3, the movement mechanised to the mutant walk producing a level final standing past the decision, a shape the true walk cannot leave, proven by the pre fold control reproducing the stated set and no shipped behaviour moving, and the baseline tripwire settled at 398 recorded and 244 walked over the 44 series files now standing and at 386 recorded and 232 walked over the 42 that stood at the Round 23 base, the parting standing in the constructed depths pair Round 23 itself landed, with count_series standing byte identical.
- Contract, DH-50 and DH-51 met. They stand in full form in CONTRACT.md under the Round 25 obligations, and Round 25 closed on the merged slice 26 after three reviews, with the spec review approving with findings at one Minor and four Observations, its Minor standing against the design record on the sibling golden count, the standards review approving with findings at two Observations, and the red team approving with findings at one Minor and two Observations, its Minor standing against the design record on the comebacks closing prose, and CodeRabbit returning no comments on the slice, with FIX-DH-29 standing free and unspent, the sibling golden count standing as a named residual and not as met work, the design record's section 1.9 list naming depths among the spells pair's sibling readings while no depths golden stands over the pair, the depths closing over the pair standing computed in that section and driven true on the shipped binary, and the repair priced at one golden, one E-3 arm, and C-1 at 205 and 201, the design record standing corrected at four further places as its errata, its section 1.8 golden names standing as shorthand where the shipped goldens stand in the family's championship form, its W-13 prose standing at a depth of 3 where its own engine computes 2 and the shipped pin states 2, its section 1.7 naming the survey's bolt and crest and bolt and drift series and the league's apex and ember and bolt and drift series the other way round with every figure and closing unaffected, and its section 1.9 comebacks closing prose carrying a no winner segment the report never prints with E-3 asserting the true rendered tail, the build's fresh extraction gate re-run standing unexecuted on the write fault and carried as unexecuted, its place discharged by the staged tree standing blob identical to the pushed tree and by the hosted gate standing green on the exact head, and the red team's whole suite grain observation standing on its V-4 and V-7 spot executions, the split's universe standing as the pin file with both executions exact at that grain and the same mutants felling the enrolment, fencing, and sweep pins that assert the same renderings through other files at the whole suite grain.
- Slice 23 has landed. It carries the tenure reading over a recorded championship, the classification of each decided series by the count of the decided races after which its winner's tally stood above its loser's read against the same count for its loser as ahead, level, or behind, one section for each series standing ahead naming its decided races in race order and the front count of the series and its winner with its race split, with the closing counts of series, ahead, level, behind, and no winner, with the tenure example over the standing championship with both of its series in section form and the constructed tenure example and their goldens under examples, and the tenure command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, opening, turns, lead, and crossings.
- Slice 24 has landed. It carries the depths reading over a recorded championship, the classification of each decided series by the depth of its winner over the decided record read against the margin the series was won by as deep, matched, or shallow, one section for each series standing deep naming its decided races in race order and the deepest deficit of the series and its winner with its race split, with the closing counts of series, deep, matched, shallow, and no winner, with the depths example over the standing championship in its empty form and the constructed depths example and their goldens under examples, and the depths command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, opening, turns, lead, crossings, and tenure.
- Slice 25 has landed. It carries the fold of the tally computation of the lead, crossings, deadlock, rally, tenure, and depths readings into the one helper of the championship module, all six readings reading the tallies of a decided series from the one home from this round, with no command added, no figure moved, and no golden changed, under the repair amendment FIX-DH-28.
- Slice 26 has landed. It carries the spells reading over a recorded championship, the classification of each decided series by the longest spell its winner stood behind over the decided record read against the behind count of the series as unbroken, broken, or never behind, one section for each series standing broken naming its decided races in race order and the longest spell and behind count of the series and its winner with its race split, with the closing counts of series, unbroken, broken, never behind, and no winner, with the spells example over the standing championship in its empty form and the constructed spells example and their goldens under examples, and the spells command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, opening, turns, lead, crossings, tenure, and depths.
- Contract, DH-52 and DH-53 met. They stand in full form in CONTRACT.md under the Round 26 obligations, and Round 26 closed on the merged slice 27 after three reviews, with the spec review approving with findings at one Observation, its Observation standing against the design record on the section 1.8 standings enumeration for drill two, the standards review approving with findings at one Observation, and the red team approving with findings at two Observations, and CodeRabbit returning one inline comment on the slice adjudicated not valid, the printed number standing as DH-13's documented count of the race in the record and a renumbering of reigns alone breaking the family's one form, with FIX-DH-29 standing free and unspent, the fresh extraction gate re-run carried from the Round 25 closure standing discharged by this round's warm-up on main at 921 passed and 0 failed, the sibling golden residual standing discharged, the depths golden over the spells pair standing enrolled with its E-3 arm and the C-1 count restated at 221 and 217, the design record's R-15 account of V-4 standing as a records item, its prose reading a reign of five in a front count of eight where the computation over the tailed fixture reads a reign of five in a front count of seven classing the series broken and the pin failing V-4 on its figures either way, the design record's section 1.8 enumeration standing with its sixth standing for drill two reading level where its own winner sequence yields ahead, its figures and the shipped golden agreeing with the recount, the red team's parity record standing as stated, the run relations of DH-52 holding over the file corpus and a 20,000 series sweep and the loose parity paraphrase standing nowhere in DH-52 or the design record, the record count semantics standing unpinned at the section surface family-wide as a candidate pin for a later round, this round's survey reigns golden already pinning the shape in substance on the red team's note, the build lane standing ended without a handoff and the completion lane's adoption standing as the build report's section 6 records them, and the four errata of the Round 25 design record standing as records items.
- Contract, the record count statement, with FIX-DH-29, met. They stand in full form in CONTRACT.md under the Round 27 obligations, and Round 27 closed on the merged slice 28 after three reviews, with FIX-DH-29 standing spent on the record count statement at both seats, the sentence standing added in place to the stated relations of DH-51 and to the stated relations of DH-53, the two pins R-21 in rust/tests/test_reigns.rs and W-21 in rust/tests/test_spells.rs standing as its witnesses, the spec review approving with no findings, the standards review approving with findings at three Observations, and the red team approving with no findings, its first seating standing ended without a handoff or a banked report and its scratch standing adopted by the reseat only after hash verification of all 440 files per tree, the reseat's report standing as the review of record, and CodeRabbit returning no inline comments on the slice, its completed review standing read by the owner at the merge gate at zero inline comments, zero submitted reviews, and no actionable comments, with the design record's section 0.1 naming rust/src/main.rs for the command table where it stands at rust/src/cli.rs line 1137, declared 26 and counted 26, standing for the errata queue, the standards review's phrasing Observation standing recorded, the relations sentence standing a third time in lowercase inside the FIX-DH-29 entry's own quotation of the form it adds, the build report's count of exactly two naming the capitalised obligation instances, the red team's edge account standing as executed, forty drives at no disagreement, the statement's qualifier holding at the counted record's tail, a no-result race after the decision counting for nothing, and FIX-DH-30 standing free and unspent.
- Slice 28 has landed. It carries the record count statement of FIX-DH-29, the numbering of a section's decided races stated as the count of the race in the record in the stated relations of DH-51 and DH-53, with one pin in each of rust/tests/test_reigns.rs and rust/tests/test_spells.rs asserting its reading's whole report over a series whose first recorded race holds no winner, its section printing the decided races at their counts in the record with the count of the race holding no winner standing skipped, and no new command, no new reading, no figure moved, and no golden changed.
- Contract, DH-54 and DH-55 met. They stand in full form in CONTRACT.md under the Round 28 obligations, and Round 28 closed on the merged slice 29 after three reviews, with the standards review approving with findings at one Minor and one Observation, its Minor standing charged to the slice, the place module's doc head naming the posture of the depths module where the design charges the posture of the reigns module and the body follows it, cured in slice by the repair commit 6316c238 standing on the slice head, the one word depths standing replaced by reigns in the doc comment with no other word moving, the repair standing verified by the owner at exactly that one word with the repair head's hosted gate and gate-stable standing at success twice each, the spec review approving with findings at three Observations, each standing against the design record and none charged to the slice, and the red team approving with no findings, its independent oracle standing at no disagreement over all nineteen championships, its spot executions of V-5, V-8, and g15 standing name for name against the executed record, and its two points standing recorded against the owner's brief phrasing and not the slice, a first attainment at the last decided race standing arithmetically impossible under DH-54, the latest attainable shape standing executed in its place, and a series standing never behind bearing depth nought under the obligation's own text, and CodeRabbit returning no inline comments on the slice, its completed review standing read in full by the standards lane at zero inline comments and zero submitted reviews, with the slice standing merged as pull request 65 at de2b2324, main's tree standing verified equal to the repaired slice tree 9cd8314c, the census standing at 980, the design record's section 5 load bearing list naming the constructed pair's whole report pin as falling under V-4 where the pin's own comment and the build's corrected V-4 execution read it passing, the same list numbering that pin PL-1 where the committed file labels it PL-2 and labels the standing championship pin PL-1, the build's first V-4 construction standing void in the open with its mechanism proven, the same slip standing in the design lane's unexecuted draft of the variant, the design's gap hunt standing negative over its fifteen mutations, both passing variants standing proven equivalent and no gap standing charged, the five items of the design record's errata queue standing as records items untouched by this round's text, and FIX-DH-30 standing free and unspent.
- Slice 29 has landed. It carries the place reading over a recorded championship, the classification of each decided series by the place of the deepest deficit of its winner over the decided record read against the depth itself as early, late, or never behind, one section for each series standing late naming its decided races in race order and the depth and place of the series and its winner with its race split, with the closing counts of series, early, late, never behind, and no winner, with the place example over the standing championship in its empty form and the constructed place example and their goldens under examples, and the place command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, opening, turns, lead, crossings, tenure, depths, spells, and reigns.
- Contract, DH-56 and DH-57 met. They stand in full form in CONTRACT.md under the Round 29 obligations, and Round 29 closed on the merged slice 30 after three reviews, with the standards review approving with findings at one Minor and three Observations, its Minor standing charged to the slice, four pin posture comments in rust/tests/test_peak.rs stating passes the pins cannot hold, the PK-4 comment recording passes under W-7 and W-9 where the pin falls under both, the PK-8 and PK-11 comments each asserting a figure line verbatim that W-7 renders exchanged while recording W-7 as a pass, and the PK-14 comment recording a pass under W-9 where the pin falls, cured in slice by the repair commit 5c1bad21 standing on the slice head, the four comments' failing lists standing restated with no assertion moving and the census unmoved, the PK-18 pin standing as a characterization pin by design, failing under no variant and charged nowhere, the spec review approving with findings at four Observations, each standing against the design record and none charged to the slice, and the red team approving with findings at three Observations, no finding standing against the slice's content, its differential standing executed on the base side over 3,753 drives and its head side legs standing unexecuted on the write fault, and CodeRabbit returning one inline comment on the slice, standing valid in full as the same finding as the standards Minor at the same seat and not a second one, with no new comment standing on the repaired head, with the slice standing merged as pull request 67 at 632fcdcc, main's tree standing verified equal to the repaired slice tree 284f93bf, the census standing at 1008, the W family W-1 to W-10 standing unexecuted whole on the write fault against the form tree at thirteen refused attempts for W-1, twelve each for W-2, W-3, W-4, W-5, W-6, W-8, and W-10, fourteen for W-7, and thirteen for W-9, every refusal standing as a fault signature with no assertion failure anywhere, the H-3 and H-5 spot executions against the head tree standing unexecuted at twelve refused attempts each, the candidate pins standing unexecuted on the base tree, the fencing pin's posture under the W family standing undecided between the design note's section 7 account of the pin standing unmoved under the whole family and the build report's computed account of the pin falling under W-1, W-5, and W-7, the build's local form gate and the fresh extraction gate's local cargo legs standing unexecuted on the same fault with the hosted gate on the exact head standing as the suite's executed evidence of record, the design note's section 7 unions standing understated at the falls of PK-4, PK-8, and PK-11 under W-7 and of PK-4 and PK-14 under W-9, cured in the slice's comments by the repair and standing in the note as the record seat of the same understatement, the note's section 8 assembled contract figure standing at 347,761, one short of the executed 347,762 with the section standing byte equal either way, the note's section 7 fencing description standing parted from the built F-1 form, the note's section 8 grouping the handicap sweep's restatement with the generated step where the slice carries it in the tests step in the Round 28 executed form, the note's section 7 naming the standing championship peak goldens in the E-3 form where the built file labels that pin E-1 and the sibling readings pin E-3, the peak golden count standing at twenty against the note's nineteen named beside the pair's own, the build report's repair narrative standing inaccurate in the red team's first Observation, its account of the third repair commit's restatement of the round four pin's count parting from the remote, the status step standing shown leaving the pin untouched and the repair commit fe60d404 standing shown carrying the restatement alone, and FIX-DH-30 standing free and unspent.
- Slice 30 has landed. It carries the peak reading over a recorded championship, the classification of each decided series by the place of the largest lead of its winner over the decided record read against the lead itself as early or late, one section for each series standing late naming its decided races in race order and the largest lead and place of the series and its winner with its race split, with the closing counts of series, early, late, and no winner, with the peak example over the standing championship in its empty form and the constructed peak example and their goldens under examples, and the peak command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, opening, turns, lead, crossings, tenure, depths, spells, reigns, and place.
- Contract, DH-58 and DH-59 met. They stand in full form in CONTRACT.md under the Round 30 obligations, and Round 30 closed on the merged slice 31 after three reviews, with the standards review approving with findings at two Observations, none standing charged to the slice, the first pricing the X family as computed and unexecuted, an execution debt carried to this closure, and the second naming the build report's repair narrative understating the repair diff by one seat at RC-8's comment prose, a records item against the build report alone, the spec review approving with findings at four Observations, each standing against the design record and none charged to the slice, the note's section 2 naming bolt-ember's answer as seven where the data and the golden read five, the note's section 2 naming bolt-ember as the immediate and late witness against DH-56 where the series stands delayed and the contract's own section names the survey apex-drift series, a naming the red team's first Observation shares, the note's section 3 build seats naming the place module where the shipped recovery module's doc head stands in the reigns posture, and the note's section 8 drafting the slice line in landed form where its own count charge requires the in review opening the build carried, and the red team approving with findings at three Observations, no finding standing against the slice's content, its independent oracle reproducing the population whole over the 49 decided series at 18 immediate, 8 delayed, and 23 never behind, all 21 recovery goldens of the head tree standing byte equal to its own renders, the build's repair standing reproduced at the oracle, the original RC-8 shape reading (immediate, 2, 1) from the loser's seat at a target of four and the repaired pin reading (immediate, 3, 2) at a target of five, its break attempts standing exhaustive over 636 decided winner sequences with no break found, and its differential standing executed on the base side over 3,976 drives, identical on its re-run, with its head side standing unexecuted, and CodeRabbit returning no submitted review and no inline comment on the slice, with the slice standing merged as pull request 69 at ec73af90, main's tree standing verified equal to the slice tree b79e4e4e, the census standing at 1036, the build's seventh commit standing as a tests repair at 3e1281b8 beyond the six charged, curing defects of the build's own test authorship at RC-2, RC-13, and RC-8 in rust/tests/test_recovery.rs alone with the census unmoved, the 34 new goldens standing rendered by the build's transliteration proven byte for byte against all 166 standing sibling goldens and in ordered agreement with the design engine over all 49 series, the built binary standing unable to render them before the window closed and that substitution standing named in the open, the sweep standing restated at 305 goldens and 301 unadjusted, the control standing executed on the standing main tree at its fifth attempt at 1008 passed and 0 failed, W-1 standing executed at one attempt with exactly 24 falls, name for name equal to its expected set, the cured comments of Round 29 cashing under it at PK-4, PK-8, and PK-11, the fencing pin F1 falling under it and its posture standing decided there, its falls under W-5 and W-7 standing open, PK-14's entries under W-7 and W-9 standing open, and no W mutation passing whole anywhere executed, the carried family standing unexecuted on the write fault at eleven refused attempts for W-2, six for W-3, five each for W-4 to W-10, and five each for H-3 and H-5 across the build and red team lanes, the red team's 46 attempts standing counted inside those figures with its three refused tarball attempts and its 49 path blob grinder landing every file at zero bytes beside them, the X family standing computed and unexecuted whole, the candidate pins standing unexecuted on the pre-round tree, the form gate and the fresh extraction gate standing unexecuted on the same fault with the hosted gate on the exact head standing as the suite's executed evidence of record, the design note's RC-8 shape at a target of four standing refuted by execution and corroborated at two seats, and FIX-DH-30 standing free and unspent.
- Slice 31 has landed. It carries the recovery reading over a recorded championship, the classification of each decided series by the answer of its winner to the deepest deficit of the series over the decided record read against the depth plus one as immediate, delayed, or never behind, one section for each series standing delayed naming its decided races in race order and the depth and answer of the series and its winner with its race split, with the closing counts of series, immediate, delayed, never behind, and no winner, with the recovery example over the standing championship in its empty form and the constructed recovery example and their goldens under examples, and the recovery command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, opening, turns, lead, crossings, tenure, depths, spells, reigns, place, and peak.
- Contract, DH-60 and DH-61 met. They stand in full form in CONTRACT.md under the Round 31 obligations, and Round 31 closed on the merged slice 32 after three reviews, with the standards review approving with findings at one Minor, the DH-60 stated relations sentence at CONTRACT.md line 554 misstating the tie case, the two figures standing equal where a longest run of the loser ties the run standing last while the sentence predicated apartness on a longest run standing earlier, verified by hand computation over the counterexample CodeRabbit raised, its one inline comment standing as the same item and dispositioned valid, and the Minor standing cured in slice by the repair commit feb7a7fc, a two word restatement of the one sentence touching CONTRACT.md alone with the census unmoved and the whole suite sweep convicting no pin literal, the spec review approving with findings at four Observations, each standing against the design record and none charged to the slice, the note's section 3 stating the added contract text at 6,433 characters and the assembled contract at 362,070 against the measured 6,432 and 362,069 with the span's bytes standing exact, the note's section 8 drafting the slice line in landed form against its own count charge, the note's section 8 phrasing the round four pin's comment chain as extended at the status step against section 9's closure charge, and the stated relations sentence's definite article reading loosely at the tie, and the red team approving with findings at three Observations, none standing against any shipped text, its independent oracle re-deriving the population exactly over both enrolments at 14 held, 33 single, and 4 without reply over the 51 decided series of the base and 15 held, 34 single, and 4 without reply over the 53 of the head, its stated relations standing checked as theorems with no violation and its eight break constructions finding no break, the first Observation standing against the design record, the Y unions of the note's section 7 omitting the handicap sweep whose standing pin carries the slice's laststand arm over the same twenty two goldens, the executed unions standing one pin larger at every variant when executions land, priced in its report before the executions were read, the second against the design record, the closing sentence of the same section placing the characterization pin LS-13 under every variant where its own unions and the pin's posture comment place it under none, and the third standing as the priced boundary of the reading, twenty three enrolled series carrying wins of the loser after the deciding race that the walk counts nowhere, DH-60 classing over the decided record the walk of DH-13 leaves, with the slice standing merged as pull request 71 at a844a519, main's tree standing verified equal to the repaired slice tree 7607fc93, the census standing at 1064, the build's seventh and eighth commits standing as tests repairs at 7450976b and e8b4c33f beyond the six charged, curing defects of the build's own test authorship in rust/tests/test_laststand.rs alone with the census unmoved, the second repair restating the whole count pin LS-11 to twenty nine, the design's figure, the build's mid lane count of thirty standing corrected in its report as its own miscount and withdrawn, the thirty five new goldens standing rendered by the build's transliteration and proven byte for byte by the hosted suite itself, the built binary standing unable to render them in the fault's windows and that substitution standing named in the open, the sweep standing restated at 340 goldens and 336 unadjusted, the control standing executed whole on the extracted base tree at the build at 1036 passed and 0 failed and on the Round 29 closure extract copy at the red team at 1008 passed and 0 failed, W-1 standing executed at the Round 30 build at exactly 24 falls and W-2 standing executed whole at this round's build at 19 falls, name for name equal to its expected set, the cured comments of Round 29 cashing under both where executed, and no W mutation passing whole anywhere executed, the carried execution family of Round 30 standing carried still at the combined attempt counts the Round 30 closure recorded, raised by the attempts of this round's warm-up and of this closure as the closure report records them, to twenty seven refused attempts for W-3, twenty two each for W-4, W-5, W-7, and W-9, and twenty one each for W-6, W-8, W-10, H-3, and H-5, its two open verdicts standing open, the fencing pin's falls under W-5 and W-7 and the cured comments' entries at PK-4 under W-7 and W-9, PK-8 and PK-11 under W-7, and PK-14 under W-9, this closure's third control attempt standing as a fault poisoned run at 879 passed and 129 failed with 118 os error signatures in its output, voided as an execution and counted as an attempt, the X family standing computed and unexecuted whole at its unions, its ground's materialization standing refused at the red team and again at this closure, the Y family standing computed and unexecuted whole at its unions with the sweep's priced expectation standing untested, its form tree assembly standing refused at the build and its tarball write refused at this closure after the fetch landed, the Round 31 candidate pins standing unexecuted on the pre-round tree, the form gate standing discharged in its hosted form on the slice head, the fresh extraction gate standing unexecuted on the same fault with the hosted gate on the exact head standing as the suite's executed evidence of record, the design note's section 3 carrying the cured sentence's original form and its one high figures standing as records items for the errata queue, and FIX-DH-31 standing free and unspent.
- Slice 32 has landed. It carries the last stand reading over a recorded championship, the classification of each decided series by the last run of its loser over the decided record as held, single, or without reply, one section for each series standing held naming its decided races in race order and the last stand of the series and its winner with its race split, with the closing counts of series, held, single, without reply, and no winner, with the laststand example over the standing championship in its empty form and the constructed laststand example and their goldens under examples, and the laststand command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, opening, turns, lead, crossings, tenure, depths, spells, reigns, place, peak, and recovery.
- Contract, DH-62 and DH-63 stand met on the strength of the census and the discrimination plan of the design note, the warm-up's unexecuted legs standing carried as the closure report names them. They stand in full form in CONTRACT.md under the Round 32 obligations, and Round 32 closed on the merged slice 33 after three reviews, with the spec review approving with findings at one Minor against the slice, DH-63's layout paragraph as shipped misstating the rendered report in five particulars, a header naming the championship where the report names the reading, a section first line naming the series and its winner over the loser where the section names the series by its place and its task, the figure line standing before the race lines where it stands after them, the race lines carrying no task where each carries its task, and no winner line where the section closes with the winner line of DH-30, with two Majors and two Minors against the design record and three Observations against the build record, CodeRabbit's contract comment standing as the same layout item and dispositioned valid and its fencing comment dismissed, the standards review approving with findings at the same one Minor with its cure recommended in slice, with one Major, one Minor, and two Observations against the design record and one Observation against the build record, and the red team approving with findings at the same item at Major and one Observation against the slice, the Championship struct's fields standing public so the restated load boundary's truth rests on the two load functions standing as the only constructors in the shipped code, which its count verifies, with two Majors, three Minors, and one Observation against the design record and three Observations against the build record, the item standing cured in slice before the merge by the fifth repair commit 39d781e1, DH-63's two layout sentences restated in DH-61's descriptive form, CONTRACT.md alone moving and no code, no pin, and no golden moving, both gates green on the repaired head with the whole suite at 1,092 passed and 0 failed, with the slice standing merged as pull request 75 at 0a37a396, main's tree standing verified equal to the slice tree d4320f8e, the census standing at 1092, the sweep standing restated at 376 goldens and 372 unadjusted, the build's repair commits standing at seven beyond the six charged steps besides the fifth repair, the enrolment repair at 66d61f95, repair3 at 9918dfda restating the firststand cli pins in the standing form, the golden read cure at 799ce0d7 and the single element loop cure at f06ef6bc, the pair constants removed at 439442b4 under the owner's ruling on the evidence that no pin constructs a fixture from the pair's models, and the fourth repair at d03522b7 and 1c26b8c0 restating FS-14 and DH-63's zero series sentence to the load boundary the shipped loader holds, the design's premise standing refuted by the loader and by the contract's own standing loader obligations, each cure standing confined to its charged path and proven on the next head's gate, the warm-up standing as the closure report records it, leg 1 of the design's charge standing executed whole at the red team's hunt on the verified forests, the control at 1,008 passed and 0 failed, W-3 at nine falls reproducing the build's executed set name for name, W-4 at six falls, W-5 at 23, W-7 at the cured union of twelve, W-9 at the cured union of ten, W-8 at one, and W-10 at eight, each at its computed set, W-6 at six falls against its computed seven, the executed set standing adopted whole as the corrected expectation, the parting convicting no shipped pin's comment and FIX-DH-31 standing unengaged by it, the two open verdicts standing decided by those executions, the fencing pin falling under W-5 and under W-7 as both computations placed it with no conviction standing against it, and the cured peak comments standing confirmed at PK-4, PK-8, and PK-11 under W-7 and PK-4 and PK-14 under W-9, the cures the unions price standing proven by execution, H-3 and H-5 standing executed at nine falls each on the forest ground, their tenth seat, the constructed peak example pin, standing carried as a head tree computation, leg 2's Z family standing executed whole at the red team on its fresh extraction of the slice tree, the form control at 1,092 passed and 0 failed and the eight variants at 10, 15, 23, 15, 18, 10, 11, and 21 falls summing to 123, the Z-1 union standing settled at ten pins with FS-15 inside it against the printed nine, no variant passing whole and no fall standing outside its seat in any variant, FS-13, FS-14, and the table pin standing under no variant, their characterization posture confirmed by execution across the whole family, the X family standing executed at eight of its ten variants, X-1 at the red team at its computed union of twenty two falls and X-2 through X-6 at this closure at thirteen, eleven, six, twenty two, and fifteen falls, each name for name at its computed union with no fault signature in any of the five logs, X-7 standing carried at its computed union of nine, its one run at this closure standing fault-poisoned at 887 passed and 149 failed with 119 os error failure blocks and its ninth computed pin standing among the falls by a fixture write panic alone, the run voided as an execution and counted as an attempt, X-8 standing carried at RC-4 alone, X-9 at eleven, and X-10 at seven, at one refused tree copy attempt each at this closure, the Y family standing carried whole at its printed unions of eight, ten, seventeen, eight, twelve, five, five, and seventeen with the sweep's priced expectation standing beside them untested still, its ground's copies at this closure standing refused with errors at every leg and landing once at 579 files of zero bytes each under the fault's silent form, the Round 31 candidate pins and leg 3's Round 32 candidate pins standing carried, their trees built and verified at the red team and their runs refused at the copy at this closure at one attempt each, the form gate standing discharged in its local form at the red team on the same extraction, the full suite at 1,092 passed and 0 failed in the Rust-active mode with the slice's formatting proof executed over its nine touched Rust files, its no-rust half standing carried at one refused copy attempt at this closure beside the red team's two, the fresh extraction gate standing discharged in its local form by that same extraction, verified blob for blob on its load-bearing files and census exact at 1092, with the hosted gate on the closure head standing as the verdict of record, the design record's items standing for the errata queue, the FS-14 premise and DH-63's zero series sentence as drafted, the layout paragraph's drafting against the note's own charged form, the Z-1 union at nine and FS-15's posture entry omitting Z-1, the W-3 summary count at seven, the W-6 computed set over-predicting the constructed peak example pin by one fall, the DH-60 parting split stated at nine and one in the note's section 2 and the design report's section 3 where the record stands at eight and two, the crossings second series standing held under DH-60 by its shipped golden's figure line, the note's DH-38 and DH-36 neighbour crosses standing unverified downstream of the refuted enumeration, the note's section 8 quoting the slice line in landed form, and the Round 30 closure's golden counts of 306 and 302 standing one high against its own tree's measured 305, and FIX-DH-31 standing free and unspent, no W or X mutation passing whole anywhere executed.
- Slice 33 has landed. It carries the first stand reading of the decided record, the constructed firststand championship beside the standing championships, and the firststand command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, opening, turns, lead, crossings, tenure, depths, spells, reigns, place, peak, recovery, laststand, and firststand.
- Contract, DH-64 and DH-65 stand met on the strength of the census and the discrimination plan of the design note, the warm-up standing executed whole as the closure report names it. They stand in full form in CONTRACT.md under the Round 33 obligations, and Round 33 closed on the merged slice 34 after three reviews, with the spec review approving at no finding against the slice, with one Observation against the build record, the warm-up's carried families standing unexecuted at the build on the write fault and since executed whole at the red team, the standards review approving with findings at one Minor against the slice, SK-12's pin comment claiming a pass under AA-7 where the pin falls, its series standing alone and AA-7 carrying the report's sections for the alone series, with one Minor against the design record at the same root in its seventh section, CodeRabbit's one inline comment standing as the same item and dispositioned valid, its hyphen style notes dismissed as the contract's standing house form, and the red team approving with findings at one Minor and one Observation against the slice, the Minor standing as the two shipped pin comments, SK-18's claiming the span pin passes under AA-1 and AA-2 where it falls at both and SK-12's beside it, the pins' assertions standing sound at every seat and each falling exactly where its assertion reaches, the Observation standing as the warm-up's executed partings adopted whole as corrected expectations, with one Major, two Minors, and one Observation against the design record and two Observations against the build record, the comment item standing cured in slice before the merge by the repair commit 6a06fcc6, the two comments restated to their executed postures, no assertion, no figure, and no code moving, both gates green on the repaired head with the whole suite at 1,120 passed and 0 failed, with the slice standing merged as pull request 79 at 47f235c0, main's tree standing verified equal to the slice tree 16f803d8, the census standing at 1120, the sweep standing restated at 413 goldens and 409 unadjusted, the build's repair commits standing at three beyond the six charged steps, the new module's field seats fenced through the standing display_line helper at 73348d8f, SK-4's section filter restated to exclude the series winner lines at b9d89c5c, and the comment cure at 6a06fcc6, each cure standing confined to its charged path and proven on the next head's gate, the warm-up standing executed whole across the line's hunts, leg 1's carried family standing executed at the Round 32 red team on the verified forests, the control at 1,008 passed and 0 failed, W-3 at nine falls, W-4 at six, W-5 at 23, W-6 at six falls against its computed seven with the executed set adopted whole as the corrected expectation, W-7 at the cured union of twelve, W-8 at one, W-9 at the cured union of ten, and W-10 at eight, W-1 and W-2 standing executed in the rounds before at twenty four and nineteen falls, the two open verdicts standing decided by those executions, the fencing pin falling under W-5 and under W-7 with no conviction standing against it, and the cured peak comments standing confirmed at PK-4, PK-8, and PK-11 under W-7 and PK-4 and PK-14 under W-9, H-3 and H-5 standing executed at nine falls each on the forest ground, their tenth seat, the constructed peak example pin, standing closed by computation at this round's red team over the head tree, the pin falling under both variants and H-3 and H-5 standing at ten falls each, the Z family standing executed whole at the Round 32 red team at 10, 15, 23, 15, 18, 10, 11, and 21 falls summing to 123, the X family standing executed whole at ten of ten variants, X-1 through X-6 at the Round 32 hunts at 22, 13, 11, 6, 22, and 15 falls and the tail at this round's build at X-7 at its computed union of nine with RC-6 falling by assertion, X-8 at RC-4 alone, X-9 at eleven, and X-10 at seven, each name for name with no fault signature in any log, the Y family standing executed whole at this round's red team on the Round 31 slice ground, its control at 1,064 passed and 0 failed, its laststand pins standing name for name at their printed unions at every variant, its executed totals standing at 12, 14, 22, 13, 17, 7, 10, and 22 falls, the whole difference standing in the shared pins, the sweep and the championship examples pin under all eight, the constructed example pin and the command prints pin under every variant but Y-6, and the fencing pin under Y-3, Y-4, Y-5, Y-7, and Y-8 in the legitimate class and figure mode with no conviction standing against it, the executed sets standing adopted whole as the corrected expectations, leg 2's AA family standing executed whole at the red team on its verified fresh extraction of the slice head, its control at 1,120 passed and 0 failed, the executed sets standing at 14, 17, 21, 13, 17, 15, 12, and 22 falls summing to 131 against the computed 128, the three falls beyond the unions standing adopted whole, SK-18 under AA-1 and AA-2 and SK-12 under AA-7, SK-13, SK-14, SK-20, and the command table pin standing under no variant, their characterization, boundary, amendment guard, and table offices confirmed by execution, leg 3 standing executed in the candidate form at the red team, the Round 31 candidate pins stopping at compile with two E0433 errors on the absent laststand module and the Round 32 candidate pins with three on the absent firststand module, leg 4's form gate standing green in both modes, the Rust-active half at the build and the no-rust half at the red team, each at 1,120 passed and 0 failed, and leg 5's fresh extraction gate standing discharged at the build and at the red team, the head extraction verified file for file against the head tree's 662 blobs with its census exact at 1120, with the hosted gate on the closure head standing as the verdict of record, the design record's items standing for the errata queue, the second run correction standing refuted on its figures and its attribution, none at 28 against the true 29 and two or more at 4 against the true 3, the Round 32 red team's report carrying no second run correction at all, the seventh section's posture lists for SK-18 and SK-12 as computed against their executed falls, the Y family's carried expectation understating the executed sets at all eight variants on the Round 31 prose postures for the prints and fencing pins, and the seventh section's render counts standing as computed claims witnessed through the golden pins' executed postures, and FIX-DH-31 standing spent beside the reading at its charged seat in the slice's contract step, FIX-DH-32 standing free and unengaged, no mutation of any family passing whole anywhere executed.
- Slice 34 has landed. It carries the first strike reading of the decided record and the firststrike report, the constructed firststrike championship beside the standing championships, the repair amendment FIX-DH-31 beside the reading, and the firststrike command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, opening, turns, lead, crossings, tenure, depths, spells, reigns, place, peak, recovery, laststand, firststand, and firststrike.
- Contract, DH-66 and DH-67 stand met on the strength of the census and the discrimination plan of the design note, the warm-up standing largely carried under the write fault as the closure report names it. They stand in full form in CONTRACT.md under the Round 34 obligations, and Round 34 closed on the merged slice 35 after three reviews, with the spec review approving at no finding against the slice and no finding against the design record, with two Observations against the build record, the head gate failure of the build's own authorship standing cured in slice and the warm-up ledger's completeness at three seats resting on batch markers, the standards review approving at no finding against the slice, with one Observation against the design record, the seating brief's stale base figures standing named and corrected in the design note's own text, and two Observations against the build record at the same seats, CodeRabbit standing as one summary comment stating no actionable comments were generated, with no inline comment standing on the pull request, and the red team approving at no finding against the slice, its verdict standing banked by its third seating after the first seating banked its premise, population, and hunt sections and died mid-hunt and the second seating banked nothing before a runtime database failure, with one Observation against the design record and two Observations against the build record at the same seats, with the slice standing merged as pull request 83 at f1efedb3, main's tree standing verified equal to the slice tree 025d7980, the census standing at 1148, the sweep standing restated at 451 goldens and 447 unadjusted, the build's repair commit standing at one beyond the six charged steps, the repair 681a5c05 removing two dead helpers and restating one vector in the pins file, no assertion, no figure, and no comment word moving, both gates green on the repaired head, the warm-up standing carried in large part under the write fault as the closure report names it, leg 1's W-5 standing executed at the build at 23 falls with the fencing pin in its set as the decided verdict places it and W-6 at six falls adopted whole as the corrected expectation, the X family's X-2, X-3, and X-4 standing executed at the red team at 13, 11, and 6 falls name for name with the Round 32 closure's sets, the remaining legs of leg 1 standing carried at their corrected expectations on counted attempts refused on the fault's signatures across the build, the red team, and this closure, the two verdicts standing decided, the fencing pin falling under W-5 and under W-7 with no conviction standing against it, and the cured peak comments standing confirmed, leg 2's BB family standing carried whole at the adopted unions of 14, 16, 15, 8, 17, 25, 11, and 20 falls summing to 126 over the 28 pins, the three reviews agreeing at every figure, no variant standing a dud, leg 3 standing executed in the candidate form at the build at one E0432 on the absent firstreign module, its whole-tree form standing carried, leg 4 standing discharged in part, its no-rust suite executed whole at the build at 1,148 passed and 0 failed with its format check and codepoint scan exiting zero, its Rust-active suite and clippy re-run standing carried with the hosted gate on the exact head standing as their verdict of record, and leg 5's fresh extraction gate standing discharged at the build and at the red team, the head extraction verified file for file against the head tree's 705 blobs with its census exact at 1148, with the hosted gate on the closure head standing as the verdict of record, the design record's items standing for the errata queue, the Round 33 note's second run correction standing refuted on its figures and its attribution both, none at 28 against the true 29 and two or more at 4 against the true 3, the Round 32 red team's report carrying no second run correction at all, and FIX-DH-32 standing free and unengaged, this round carrying no repair amendment, no mutation of any family passing whole anywhere executed.
- Slice 35 has landed. It carries the first reign reading of the decided record and the firstreign report, the constructed firstreign championship beside the standing championships, and the firstreign command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, opening, turns, lead, crossings, tenure, depths, spells, reigns, place, peak, recovery, laststand, firststand, firststrike, and firstreign.
- Slice 27 has landed. It carries the reigns reading over a recorded championship, the classification of each decided series by the longest reign of its winner over the decided record read against the front count of the series as unbroken or broken, one section for each series standing broken naming its decided races in race order and the longest reign and front count of the series and its winner with its race split, with the closing counts of series, unbroken, broken, and no winner, with the reigns example over the standing championship in its empty form and the constructed reigns example and their goldens under examples, and the reigns command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, opening, turns, lead, crossings, tenure, depths, and spells.
- The Rust port has landed. It carries the behavior the Python package carried, checked against the same goldens, and the Python tree stands removed under PORT-13.
- Local gate, cargo run --bin codepoint_scan, cargo fmt --check, cargo clippy --all-targets -- -D warnings, and cargo test. CI runs the same commands.

## Licence

MIT. See LICENSE.
