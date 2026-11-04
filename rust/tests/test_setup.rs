//! Setup pins, translated from the Python corpus. Each test
//! can fail for a real reason against a repo that lost a setup
//! file or let its status drift from its contract.
//!
//! The last two pins are port additions. The README carries
//! the Rust port status line, and the CI gate carries the
//! cargo steps and the scan binary alone since the removal
//! under PORT-13, as the port section of the contract states.

mod shared;

use shared::*;

fn read(path: &std::path::Path) -> String {
    std::fs::read_to_string(path).expect("a text file")
}

#[test]
fn agents_carries_the_canonical_marker() {
    let text = read(&repo_root().join("AGENTS.md"));
    assert!(text.contains("<!-- repo annex below this line -->"));
    assert!(text.contains("39 repo storefront perfection program"));
}

#[test]
fn readme_states_the_closed_round_count() {
    let text = read(&repo_root().join("README.md"));
    assert!(text.contains("Closed perfection rounds, 5 of 100"));
}

#[test]
fn contract_carries_the_setup_obligations() {
    let text = read(&repo_root().join("CONTRACT.md"));
    for oid in [
        "SETUP-1", "SETUP-2", "SETUP-3", "SETUP-4", "DH-1", "DH-5", "DH-6", "DH-7", "DH-8", "DH-9",
        "DH-10", "DH-11", "DH-12",
    ] {
        assert!(text.contains(oid), "{oid}");
    }
}

#[test]
fn readme_reports_the_contract_status() {
    let text = read(&repo_root().join("README.md"));
    let expected = "- Contract, SETUP-1 to SETUP-4 and DH-1 to DH-6 met. DH-1 to DH-4 stand in full form in CONTRACT.md with the Round 1 repair amendments, and Round 1 closed on the merged slice 2 after three reviews, a repair pass, and an approving re-review.";
    assert!(text.lines().any(|line| line == expected));
}

#[test]
fn readme_reports_the_round_three_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-7 and DH-8 met. They stand in full form in CONTRACT.md with the Round 3 repair amendments, and Round 3 closed on the merged slice 4 after three reviews, a repair pass, and an approving re-review.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 4 has landed. It carries the adjudication seat over the two decision model routes, the live entry path into the race record, and the recorded close call pilot over both models, ten synthetic recorded races with their envelopes, adjudicated records, and run record under examples.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn readme_reports_the_round_four_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-9 and DH-10 met. They stand in full form in CONTRACT.md with the Round 4 repair amendments, and Round 4 closed on the merged slice 5 after three reviews, a repair pass, and an approving re-review.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 5 has landed. It carries the panel reading over two recorded adjudication entries, the panel join over the Round 3 pilot records, and the recorded panel pilot over both models, ten panel records with their closecall goldens and the panel series under examples.";
    assert!(text.lines().any(|line| line == slice_line));
    // Altered at the Round 5 slice for its named mechanical
    // consequence. This count holds the slices standing in
    // review across the whole README. It stood at zero when
    // Round 4 closed, and slice 6 stood in review while
    // Round 5 was built, so the count was one until the
    // Round 5 closure landed slice 6 and the count returned
    // to zero. The round five pin below holds the slice line
    // itself.
    assert_eq!(text.matches("built and in review").count(), 0);
}

#[test]
fn readme_reports_the_round_five_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-11 and DH-12 met. They stand in full form in CONTRACT.md under the Round 5 obligations, and Round 5 closed on the merged slice 6 after three reviews, no repair pass, and an owner gate discharging the standards verification condition with three mutation spot checks.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 6 has landed. It carries the handicap, a per attempt adjustment in seconds read as an adjusted finish by the winner rule and the close call band, with the handicapped example race and the handicap series and their goldens under examples.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn the_gate_lines_name_the_standing_gate() {
    // A port repair pin, updated at the removal under PORT-13.
    // The README gate line and the AGENTS annex gate sentence
    // named the Python gate while it stood, so both lines are
    // pinned in their full form here, now naming the scan
    // binary and the cargo gate alone.
    let readme = read(&repo_root().join("README.md"));
    let gate = "- Local gate, cargo run --bin codepoint_scan, cargo fmt --check, cargo clippy --all-targets -- -D warnings, and cargo test. CI runs the same commands.";
    assert!(readme.lines().any(|line| line == gate));
    let agents = read(&repo_root().join("AGENTS.md"));
    assert!(agents.contains(
        "The local gate is cargo run --bin codepoint_scan, cargo fmt --check, cargo clippy --all-targets -- -D warnings, and cargo test"
    ));
}

#[test]
fn ci_runs_the_same_gate_as_the_local_gate() {
    let text = read(&repo_root().join(".github").join("workflows").join("ci.yml"));
    let live: String = text
        .lines()
        .map(|line| line.split('#').next().expect("a head"))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(live.contains("cargo run --bin codepoint_scan"));
    assert!(!live.contains("python3"), "the Python gate is gone");
    assert!(!live.contains("pytest"), "the Python gate is gone");
}

#[test]
fn readme_reports_the_rust_port_status() {
    let text = read(&repo_root().join("README.md"));
    let line = "- The Rust port has landed. It carries the behavior the Python package carried, checked against the same goldens, and the Python tree stands removed under PORT-13.";
    assert!(text.lines().any(|held| held == line));
}

#[test]
fn ci_carries_the_cargo_steps() {
    let text = read(&repo_root().join(".github").join("workflows").join("ci.yml"));
    let live: String = text
        .lines()
        .map(|line| line.split('#').next().expect("a head"))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(live.contains("cargo fmt --check"), "the format step");
    assert!(
        live.contains("cargo clippy --all-targets -- -D warnings"),
        "the lint step"
    );
    assert!(live.contains("cargo test"), "the test step");
}
