# Security Policy

## Scope and Trust Model

`deadheat` itself opens no network sockets and phones home nowhere. Its thirty-five analytical reading commands (`race` without `--standings`, `closecall`, `standings`, `series`, `flip`, `championship`, `totals`, `pairing`, and the twenty-seven trajectory readings) are pure read-only functions over local JSON files. Three explicit operator commands perform local filesystem or subprocess actions requested by the caller. `deadheat run <spec.json>` clears `<workspace>/attempt-1` and `<workspace>/attempt-2`, executes the two shell commands written in `spec.json` (which run under the operator's local environment and may use whatever tools or network access those commands invoke), and writes the target `<record>` file, so operators should run only trusted race specifications. `deadheat race --standings <file>` updates the named standings file, and `deadheat adopt` and `deadheat discard` write or delete `.patch` files under the named target directory. Whole-input schema validators check specifications, races, standings, series, and championships before rendering, while the runner parses lane `result.json` fields individually (`DH-2`), treating a malformed `tokens` field as absent while retaining a valid `result` value, and line-break fences prevent embedded `\n` or `\r` characters from forging extra report lines.

## Supported Versions

Security fixes land on the latest `0.1.x` release series and on `main`.

## Reporting a Vulnerability

Please report suspected vulnerabilities privately through GitHub Security Advisories at <https://github.com/david-engelmann/deadheat/security/advisories/new> rather than opening a public issue. Include a minimal synthetic input demonstrating the parser, runner, or line-break-fencing defect along with the commit SHA or release tag tested. Every report receives an initial triage response within seven days.
