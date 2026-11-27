# Changelog

All notable product and packaging changes to `deadheat` stand recorded in this file.

## Unreleased

### Added

- Prebuilt release workflow covering Linux, macOS, and Windows archives alongside a `SHA256SUMS` verification installer (`install.sh`) for Linux and macOS.
- Crate packaging and `cargo-binstall` metadata in `Cargo.toml`, excluding the internal `codepoint_scan` gate binary from published crates, plus `deny.toml` license and advisory checks.
- Documentation suite under `docs/` covering the operator guide, pipeline architecture, multi-agent orchestration recipes, and frequently asked questions, paired with dark and light vector diagrams under `docs/media/`.

## 0.1.0

### Added

- Local two lane duel runner executing competing model commands in isolated workspace directories and recording structured version 1 race artifacts.
- Deterministic race scorer combining verification check pass or fail status, elapsed wall clock seconds, per attempt handicaps, and the fixed five percent close call band.
- Recorded close call seat adjudication, dual judge panel synthesis, and loser patch adoption or discard commands.
- Thirty five offline analytical readings across races, standings, series, and championships including comeback, rally, deadlock, streak, cluster, reign, spell, recovery, and crosshead reports.
- Consolidated version 1 JSON format specifications in FORMATS.md with full defect reporting and terminal width aware help rendering.
