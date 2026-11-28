# Changelog

All notable product and packaging changes to `deadheat` stand recorded in this file.

## Unreleased

### Added

- Prebuilt five-target release workflow (`release.yml`) bundling shell completions, JSON Schemas, and manual pages into Linux, macOS, and Windows archives alongside a `SHA256SUMS` verification installer (`install.sh`).
- Crate packaging and `cargo-binstall` metadata in `Cargo.toml`, excluding internal gate binaries and handbook build sources so published `.crate` archives stay compact, plus `deny.toml` license and advisory checks.
- Draft 2020-12 JSON Schemas under `schemas/`, static tab-completion scripts under `completions/` (`bash`, `zsh`, `fish`, `PowerShell`), offline `man/deadheat.1` manual page, and root composite GitHub Action (`action.yml`).
- Searchable `mdBook` operator handbook under `book/` paired with GitHub Pages CI (`docs.yml`), clean-room installer verification (`clean-install.yml`), and three-OS matrix validation (`cross-platform.yml`).
- Documentation and visual media suite under `docs/` and `docs/media/` covering the operator guide, five-tool pipeline cookbook, pipeline architecture, Spectacle Line comparison, measured Apple Silicon release benchmarks, social preview cards, animated CLI showcase, and pinned-compatible storefront README draft.

## 0.1.0

### Added

- Local two lane duel runner executing competing model commands in isolated workspace directories and recording structured version 1 race artifacts.
- Deterministic race scorer combining verification check pass or fail status, elapsed wall clock seconds, per attempt handicaps, and the fixed five percent close call band.
- Recorded close call seat adjudication, dual judge panel synthesis, and loser patch adoption or discard commands.
- Thirty five offline analytical readings across races, standings, series, and championships including comeback, rally, deadlock, streak, cluster, reign, spell, recovery, and crosshead reports.
- Consolidated version 1 JSON format specifications in FORMATS.md with full defect reporting and terminal width aware help rendering.
