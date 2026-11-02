# deadheat

Every task is a race. Your agent in one lane, a shadow of yourself on a rival model in the other, and the loser gets a tiny funeral.

## What this is

You start a task and the race begins. Your agent works in your worktree. A fork of the same conversation, same context, same ticket, starts on a rival model in its own worktree. A scoreboard tracks both lanes live, files touched, tests green, tests red, tokens spent. The first suite to go fully green takes the flag. The loser's pane dims, a small funeral plays, and its diff arrives as a patch you can adopt or burn. The standings persist across sessions, so over a season you learn something no benchmark can tell you. In this repo, at this hour, which model actually wins your work, and what each victory costs.

## Status

Seeded in October 2026. Setup has landed and the build has not started. Part of the Spectacle line in the 39 repo storefront perfection programme.

## Programme status

- Closed perfection rounds, 0 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed. Seeding and setup are not rounds.
- Contract, SETUP-1 to SETUP-4 met by the setup slice. The first slice obligations DH-1 to DH-4 stand in stub form in CONTRACT.md and are not yet met.
- The first slice races two recorded or locally run attempts at the same task in separate environments, first green wins, the funeral rendered, and the season standings file started.
- Local gate, python3 scripts/codepoint_scan.py and python3 -m pytest tests -q. CI runs the same commands.

## Licence

MIT. See LICENSE.
