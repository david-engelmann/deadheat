# deadheat

Every task is a race. Your agent in one lane, a shadow of yourself on a rival model in the other, and the loser gets a tiny funeral.

## What this is

You start a task and the race begins. Your agent works in your worktree. A fork of the same conversation, same context, same ticket, starts on a rival model in its own worktree. A scoreboard tracks both lanes live, files touched, tests green, tests red, tokens spent. The first suite to go fully green takes the flag. The loser's pane dims, a small funeral plays, and its diff arrives as a patch you can adopt or burn. The standings persist across sessions, so over a season you learn something no benchmark can tell you. In this repo, at this hour, which model actually wins your work, and what each victory costs.

## Status

Seeded in October 2026. Setup has landed, and Rounds 1 to 3 have closed, with all three slices merged on main. Part of the Spectacle line in the 39 repo storefront perfection programme.

## Programme status

- Closed perfection rounds, 3 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed. Seeding and setup are not rounds.
- Contract, SETUP-1 to SETUP-4 and DH-1 to DH-6 met. DH-1 to DH-4 stand in full form in CONTRACT.md with the Round 1 repair amendments, and Round 1 closed on the merged slice 2 after three reviews, a repair pass, and an approving re-review.
- Slice 2 has landed. It carries the recorded race loader, the local runner, the winner rule, the race report and the funeral, the loser patch adopt and discard operations, the season standings, and the race, adopt, discard, and standings commands, with two synthetic recorded races and their golden renderings under examples.
- Contract, DH-5 and DH-6 met. They stand in full form in CONTRACT.md with the Round 2 repair amendments, and Round 2 closed on the merged slice 3 after three reviews, a repair pass, and an approving re-review.
- Slice 3 has landed. It carries the close call band and the recorded adjudication, and the series over recorded races, with the closecall and series commands, and with synthetic recorded races and series and their golden renderings under examples.
- Contract, DH-7 and DH-8 met. They stand in full form in CONTRACT.md with the Round 3 repair amendments, and Round 3 closed on the merged slice 4 after three reviews, a repair pass, and an approving re-review.
- Slice 4 has landed. It carries the adjudication seat over the two decision model routes, the live entry path into the race record, and the recorded close call pilot over both models, ten synthetic recorded races with their envelopes, adjudicated records, and run record under examples.
- Contract, DH-9 and DH-10 now stand in full form in CONTRACT.md under the Round 4 obligations, and their standing is not claimed here, as Round 4 has not closed.
- Slice 5 is built and in review. It carries the panel reading over two recorded adjudication entries, the panel join over the Round 3 pilot records, and the recorded panel pilot over both models, ten panel records with their closecall goldens and the panel series under examples.
- Local gate, python3 scripts/codepoint_scan.py and python3 -m pytest tests -q. CI runs the same commands.

## Licence

MIT. See LICENSE.
