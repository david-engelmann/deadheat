# deadheat roadmap

The contract holds what is built. This file holds what is planned,
in the order the programme intends to build it. Nothing here is
claimed as met anywhere in the README programme status until a
round closes on it.

## Decision models in deadheat

Deadheat has one honest seat for judgment, and it is narrow. The
winner rule decides every race with a clear margin, and most races
have one. A race is a close call when both attempts finish inside a
margin the contract will state, for example both green with finish
times within a stated band, or both red with the same outcome, and
a close call is where a decision model earns its place. Typesafe
Jev is the primary route, connected and verified live on 2026-11-02
through the programme's own connector, answering a choice question
with a chosen label, a confidence, and a probability per label. The
OpenAI decision model is the comparison point, and the first
adjudication round records both models over the same recorded close
calls before any adoption widens beyond Jev.

Where judgment enters is the adjudication of a recorded close call
and nowhere else. The adjudication call presents the two recorded
attempts, their results, their finish times, their token counts,
and their patches in stated summary form, and asks the model to
choose the winner. A verdict below a confidence the contract will
state is an abstention, and an abstained race stands as a dead heat
with no winner, which the standings already record as a no result.
The verdict and its confidence are written into the race record as
data the moment they land, so the race replays and the standings
recompute from the record forever with no live call.

What the model decides is a choice between the two attempts, and
nothing else. What stays deterministic is everything Round 1 built.
The runner still runs each attempt in its own emptied lane. The
loader still names malformed races and never raises. The winner
rule still decides every race outside the close call band, and the
band itself is a stated deterministic test on the recorded figures,
so whether a race goes to adjudication is never itself a judgment
call. The standings still update as a pure function of recorded
races.

Tests stay offline on recorded fixtures. An adjudication call
returns an envelope holding the model name, the decision with its
choice, confidence, and probabilities, and the token usage, and
that envelope is what a fixture records. The suite replays recorded
verdicts through the same adapter the live slice uses, covering a
choice each way and an abstention, so the adjudication path is
exercised with no live call in the gate. Live calls happen only
inside a gated live slice whose recordings then join the fixtures.

The bounded live budget proposal is a cap of 100 adjudication calls
in any one round. Close calls are rare by construction, since the
band is narrow and most races carry a clear margin, and each call
carries both attempt summaries, so the pilot round first measures
the real per call token cost over recorded close calls before any
live race is adjudicated. Every live round logs its estimate before
it runs and its actual spend after, in the Spectacle line ledger,
under the standing cap of 50 dollars per repo per round that the
owner set on 2026-11-02. Spend beyond this proposal waits for the
owner's word through the main chat.
