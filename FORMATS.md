# deadheat formats

This reference restates the formats deadheat reads and writes in one place, for a producer or a consumer built outside this repo. The contract at CONTRACT.md remains the spec of record. Where this reference and the contract read a shape differently, the contract governs and this reference is the defect.

Every format below stands at version 1 under the version rule of FIX-DH-17, with the envelope shapes standing as the one exception, named in their own section. A format object may carry the field version. Where the field stands absent, the object reads as version 1. Where it stands as the integer 1, the object reads as version 1. Where it stands in any other form, the surface that reads the object refuses it in that surface's own phrasing. Evolution within a version is additive only. A later version may add fields, and it may not remove a version 1 field and may not change the reading of one, and a reader refuses a stated version it does not know rather than reading the object in part. Every loader ignores fields the shape does not name.

## The JSON reading

The JSON reading of PORT-3, restated as format semantics.

- A repeated object key keeps its last value.
- A number is classified from its raw literal. An integer literal too large for a finite double and a float literal that overflows draw the finite defect where the contract states it.
- A string is a sequence of code units, and an unpaired surrogate is detected and refused where the contract states it.
- The depth boundary stands at 9997 open containers at once, measured the way PORT-3 states it.

## The race record

One recorded race, the shape DH-1 states. The race is one JSON object.

- task, text, required. The value is non empty, holds no unpaired surrogate, and holds no newline or carriage return, because the report and the funeral print it as a label.
- attempts, a list of exactly two attempt objects, required, in record order.
- adjudication, one adjudication entry object, optional. A race carries an adjudication entry or a panel, never both.
- panel, a list of one or two adjudication entry objects, optional.
- version, optional, under the version rule above.

Each attempt object holds the fields below.

- model, text, required. The value is non empty, holds no unpaired surrogate, and holds no line break.
- environment, text, required, under the same rule as model.
- result, text, required, standing as one of the words green, red, or unfinished.
- tokens, required, standing as a non negative integer or as null when the runner did not report spend.
- patch, text, required. The value is verbatim diff text, may be empty, and never holds an unpaired surrogate. Line breaks are its content and stand outside the one line rule that binds the labels.
- finished_at, a finite number at or above zero, present exactly when the result is green or red.
- adjustment, a finite number, optional. The value stands only on a finished attempt, and it never takes the finish below zero.

Version posture, version 1 under FIX-DH-17.
Refusal, the loader of DH-1 names every malformed field in one pass and returns no race, in the defect vocabulary DH-1, DH-5, DH-9, DH-11, and FIX-DH-13 state.
Pinned by rust/tests/test_races.rs, with the version refusals pinned by rust/tests/test_formats.rs.

## The runner result file

The file one attempt leaves in its lane under the name result.json, the shape the runner of DH-1 reads.

- result, optional, standing as one of the words green, red, or unfinished. A value the reader cannot use falls back to the exit code reading FIX-DH-7 states.
- tokens, optional, standing as a non negative integer. Any other shape reads as unreported.
- version, optional, under the version rule above, with the one stated exception. A result file carrying a version field in any form other than absent or the integer 1 is unreadable as a result file and is ignored whole, and the attempt falls back to the exit code reading.

Version posture, version 1 under FIX-DH-17.
Refusal, the reading draws no defects. A file that cannot be read as an object is ignored whole, and a file nested past the depth boundary is the runner's one error, escaping the run.
Pinned by rust/tests/test_runner.rs.

## The adjudication entry

One recorded adjudication of one recorded race, the shape DH-5 states, standing as the adjudication field of a race record.

- adjudicator, text, required. The value is non empty, holds no unpaired surrogate, and holds no line break.
- chosen, text, required, under the same rule, naming one of the two attempts by its model.
- reason, text, required, under the same rule.
- confidence, a finite number from zero to one, required.
- version, optional, standing on the entry itself, under the version rule above.

Version posture, version 1 under FIX-DH-17.
Refusal, the loader names a malformed entry in the adjudication phrasing of DH-5.
Pinned by rust/tests/test_close_calls.rs, with the version refusals pinned by rust/tests/test_formats.rs.

## The panel

One or two adjudication entries standing together as the panel field of a race record, the shape DH-9 states. Each entry stands in the adjudication entry shape above, and the version field stands on the entry itself. The panel as a whole carries no second version of its own. Two entries naming the same adjudicator are refused, and a panel standing beside an adjudication entry is refused.

Version posture, version 1 under FIX-DH-17.
Refusal, the loader names a malformed entry in the panel entry phrasing of DH-9, each entry numbered from zero in panel order.
Pinned by rust/tests/test_panels.rs, with the version refusals pinned by rust/tests/test_formats.rs.

## The series file

A set of recorded races between two models under one wins target, the shape DH-6 states. The series is one JSON object.

- task, text, required, under the label rule of the race record.
- models, a list of exactly two model labels, required. Each label is non empty text holding no unpaired surrogate and no line break, and the two labels are distinct.
- wins_target, a positive integer, required.
- races, a list of race records, required, possibly empty. A race naming a model outside the two models is refused, and a race naming the same model twice is refused.
- version, optional, under the version rule above.

Version posture, version 1 under FIX-DH-17.
Refusal, the loader names every malformed field in one pass, in the series phrasing of DH-6, with a defect inside one race standing under that race's prefix.
Pinned by rust/tests/test_series.rs, with the version refusals pinned by rust/tests/test_formats.rs.

## The championship file

A set of named series files read together, the shape DH-13 states. The championship is one JSON object.

- series, a list of one or more series file names, required. Each name is non empty text holding no unpaired surrogate and no line break. A relative name reads against the folder holding the championship file. A name may stand more than once, and the series it names is counted once for each time it stands.
- version, optional, under the version rule above.

Version posture, version 1 under FIX-DH-17.
Refusal, a named series that fails to load refuses the championship whole, each of its defects standing under that series' prefix, as DH-13 states.
Pinned by rust/tests/test_championship.rs, with the version refusals pinned by rust/tests/test_formats.rs.

## The survey championship fixture

The survey championship of DH-28 stands at examples/survey-championship.json over six constructed series files under examples, in the series file and championship file shapes above. Its figures are drawn, not hand rounded. Finish times are drawn from a log normal distribution with a median of 420 seconds and a multiplicative spread of 0.55, the shape read against the spread of per run durations in the public run logs of the Holistic Agent Leaderboard at Princeton, and token counts are drawn log normally over the tens of thousands to the low hundreds of thousands on the same source's token spread, each figure carried at the precision the draw produces, up to three decimal places. About one race in eight stands with no result. The token counts are drawn log normally with a median of 60000 and a multiplicative spread of 0.6, and the drawing seed 20261108 covers both the finish draw and the token draw. Every figure in the fixture is synthetic, and its file names and task texts are labelled as constructed.

## The league championship fixture

The league championship of DH-31 stands at examples/league-championship.json over ten constructed series files under examples, in the series file and championship file shapes above. Its figures are drawn, not hand rounded. Finish times are drawn from a log normal distribution with a median of 420 seconds and a multiplicative spread of 0.55, the shape read against the spread of per run durations in the public run logs of the Holistic Agent Leaderboard at Princeton, and token counts are drawn log normally with a median of 60000 and a multiplicative spread of 0.6 on the same source's token spread, each figure carried at the precision the draw produces, up to three decimal places. About one race in eight stands with no result. The drawing seed 20261113 covers both the finish draw and the token draw. Every figure in the fixture is synthetic, and its file names and task texts are labelled as constructed.

## The standings file

The standings table the race command keeps, the shape DH-4 states. The standings file is one JSON object.

- models, an object keyed by model label, required. Each entry holds the integer counts wins, losses, no_results, tokens, and tokens_unreported, each at or above zero. A model key holds no line break and no unpaired surrogate.
- version, optional, under the version rule above.

Version posture, version 1 under FIX-DH-17.
Refusal, the loader names every malformed field in one pass, in the standings phrasing of DH-4.
Pinned by rust/tests/test_races.rs, with the version refusals pinned by rust/tests/test_formats.rs.

## The pilot run record

The record of one pilot run of recorded adjudication calls, the shape DH-8 states, written by the maker to examples/pilot/run_record.json.

- models, an object keyed by model string, required. Each entry holds the route under route, the counts calls_attempted, entries_recorded, and failed_calls, the token totals input_tokens and output_tokens, and, for the chat route, the run's cost under cost as a decimal string.
- version, written by the maker with the value 1. A record read back carrying a stated version in any other form is refused by the maker readings, in the refusal posture PORT-11 states.

Version posture, version 1 under FIX-DH-17.
Refusal, the maker fails with an error and writes nothing, as PORT-11 states.
Pinned by rust/tests/test_pilot.rs.

## The race specification

The input of the run command, the shape DH-27 states. The specification is one JSON object.

- task, text, required, under the label rule of the race record.
- workspace, text, required, naming one directory. The directory is created when it does not stand there.
- record, text, required, naming the race record file the run writes. Its folder is created before the writing.
- attempts, a list of exactly two attempt objects, required, in record order.
- version, optional, under the version rule above.

Each attempt object holds the fields below.

- model, text, required, under the label rule of the race record.
- environment, text, required, under the same rule.
- command, text, required. The value is non empty and carries no one line rule, because a shell command may stand on more than one line and no rendering prints it.
- timeout_seconds, a finite number at or above zero, optional.

Version posture, version 1 under FIX-DH-17.
Refusal, the loader of DH-27 names every malformed field in one pass in the defect vocabulary DH-27 states, and the run command runs nothing over a malformed specification.
Pinned by rust/tests/test_runs.rs.

## The envelopes

The recorded response shapes of the two decision model routes, read by the seat reader of DH-7.

- The Jev envelope is one JSON object holding a model string and an answers object. The answers object holds a decision object, and the decision object holds a choice, a confidence, and the probabilities the route returns.
- The chat envelope is one JSON object holding a model string and a choices list. The first entry of the list holds a message, and the message holds its content as text that parses as the decision object.

Version posture, the envelopes carry no deadheat version. They are the response shapes of their routes and carry the versions of those routes, and the seat reader reads them exactly as DH-7 states.
Pinned by rust/tests/test_seats.rs.
