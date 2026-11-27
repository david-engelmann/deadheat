<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="media/deadheat-lockup-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="media/deadheat-lockup-light.svg">
    <img alt="deadheat" src="media/deadheat-lockup-light.svg" width="380">
  </picture>
</p>

# Deadheat Documentation

Head-to-head coding agent duel runner, close-call adjudicator, and thirty-eight-command series analytics.

## Documentation Map

| Document | What it covers |
| --- | --- |
| [GUIDE.md](GUIDE.md) | Installation, 60-second walkthrough, categorized command matrix, and multi-agent pipeline recipes |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Four-stage processing pipeline, module map, exact arithmetic guarantees, and input fencing |
| [FAQ.md](FAQ.md) | Scope boundaries against sibling Spectacle tools, exit codes, determinism, and common troubleshooting |
| [../FORMATS.md](../FORMATS.md) | Complete version 1 JSON and text input schemas, field rules, and minimal examples |
| [../CONTRACT.md](../CONTRACT.md) | Normative behavioural specification and numbered round obligations |
| [../CHANGELOG.md](../CHANGELOG.md) | Release history and packaging milestones |

## Design Guarantees

- **Zero network sockets in `deadheat` itself.** `deadheat`'s own code makes zero network calls and phones home nowhere. Its thirty-five analytical reading commands are pure read-only functions and `adopt`, `discard`, and `race --standings` touch only the named local files, while external lane commands started by `deadheat run` execute unrestricted in the local shell environment and may use the network.
- **Exact rational timing and margins.** Finish adjustments, the five-percent close-call band, and aggregate championship margins run through `BigInt` and `BigRational` arithmetic with zero float drift.
- **Whole-input defect reporting.** Malformed specifications and records return structured defect lists naming every reachable schema violation before running or printing a report.
- **Single-line output fencing.** Task titles, model identifiers, environment labels, and adjudicator fields reject unpaired UTF-16 surrogates and line breaks (`\n` and `\r`) at both loader and library boundaries, though untrusted text displayed in a terminal should still pass through a pager or sanitizer.
