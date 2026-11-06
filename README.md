# deadheat

Deadheat races two attempts at the same task, one attempt per model, and decides the winner from the recorded results.

## What this is

Deadheat is a Rust library with a command line tool of the same name. A race pairs two attempts at one task. Each attempt record names the model that made the attempt, the environment it ran in, its suite result of green, red, or unfinished, its finish time in seconds where the suite finished, its token spend where the spend was reported, and the patch it left. The records live in JSON files. A local runner can build a race record by running one shell command per attempt, each in its own directory, and taking the suite result, the elapsed time, the token spend, and the patch from what the command leaves behind.

Deadheat decides the winner from the record alone. A green attempt beats an attempt that did not go green. Between two green attempts the earlier finish wins, and a per attempt adjustment in seconds is applied to the finish before the comparison. An exact tie goes to the attempt that stands first in the record. When the attempts land in the close call band, a recorded adjudication entry with a confidence of four fifths or higher decides the race, and a panel of recorded entries decides the race when the entries meeting that threshold agree. A close call that no entry decides has no winner.

The commands print readings over the recorded files and keep the score. The race command prints the race report and the funeral for one recorded race, and with a standings file named it updates that file with the result. The closecall command states whether one race sits in the close call band and what decided it. The series command reads a series file of races between the same two models and prints the result of each race and the winner of the series. The championship command reads a championship file naming series files and prints the championship table over them. The flip command reads a series file and stands the scratch winner of each race beside the adjusted winner, stating whether the race flipped, held, or had no result. The standings command prints the standings table held in a standings file. The adopt command writes the losing attempt patch into a patches folder and the discard command removes it. The commands work offline over the files they are given.

## Status

Seeded in October 2026. Setup has landed, and Rounds 1 to 8 have closed, with all slices merged on main. Part of the Spectacle line in the 39 repo storefront perfection programme.

## Programme status

- Closed perfection rounds, 8 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed. Seeding and setup are not rounds.
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
- The Rust port has landed. It carries the behavior the Python package carried, checked against the same goldens, and the Python tree stands removed under PORT-13.
- Local gate, cargo run --bin codepoint_scan, cargo fmt --check, cargo clippy --all-targets -- -D warnings, and cargo test. CI runs the same commands.

## Licence

MIT. See LICENSE.
