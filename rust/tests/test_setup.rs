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
    assert!(text.contains("Closed perfection rounds, 17 of 100"));
}

#[test]
fn contract_carries_the_setup_obligations() {
    let text = read(&repo_root().join("CONTRACT.md"));
    for oid in [
        "SETUP-1", "SETUP-2", "SETUP-3", "SETUP-4", "DH-1", "DH-5", "DH-6", "DH-7", "DH-8", "DH-9",
        "DH-10", "DH-11", "DH-12", "DH-13", "DH-14", "DH-15", "DH-16", "DH-17", "DH-18", "DH-19",
        "DH-20",
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
    // consequence, and again at the Round 6 slice for the same
    // consequence, and again at the Round 7 slice, and again
    // at the Round 8 slice, and again at the Round 9 slice,
    // and again at the Round 10 slice, and again at the
    // Round 11 slice, and again at the Round 12 slice, and
    // again at the Round 13 slice, and again at the
    // Round 14 slice, and again at the Round 15 slice,
    // and again at the Round 16 slice, and again at the
    // Round 17 slice. This count
    // holds the slices standing in review across the whole
    // README. It stood at zero when Round 4 closed, and slice
    // 6 stood in review while Round 5 was built, so the count
    // was one until the Round 5 closure landed slice 6 and the
    // count returned to zero. Slice 7 stood in review while
    // Round 6 was built, so the count was one again until the
    // Round 6 closure landed slice 7 and the count returned
    // to zero. Slice 8 stood in review while Round 7 was
    // built, so the count was one again until the Round 7
    // closure landed slice 8 and the count returned to zero.
    // Slice 9 stood in review while Round 8 was
    // built, so the count was one again until the Round 8
    // closure landed slice 9 and the count returned to zero.
    // Slice 10 stood in review while Round 9 was built, so
    // the count was one again until the Round 9 closure
    // landed slice 10 and the count returned to zero.
    // Slice 11 stood in review while Round 10 was built, so
    // the count was one again until the Round 10 closure
    // landed slice 11 and the count returned to zero.
    // Slice 12 stood in review while Round 11 was built, so
    // the count was one again until the Round 11 closure
    // landed slice 12 and the count returned to zero.
    // Slice 13 stood in review while Round 12 was built, so
    // the count was one again until the Round 12 closure
    // landed slice 13 and the count returned to zero.
    // Slice 14 stood in review while Round 13 was built, so
    // the count was one again until the Round 13 closure
    // landed slice 14 and the count returned to zero.
    // Slice 15 stood in review while Round 14 was built, so
    // the count was one again until the Round 14 closure
    // landed slice 15 and the count returned to zero.
    // Slice 16 stood in review while Round 15 was built,
    // so the count was one again until the Round 15 closure
    // landed slice 16 and the count returned to zero.
    // Slice 17 stood in review while Round 16 was built,
    // so the count was one again until the Round 16 closure
    // landed slice 17 and the count returned to zero.
    // Slice 18 stood in review while Round 17 was built,
    // so the count was one again until the Round 17 closure
    // landed slice 18 and the count returned to zero.
    // Slice 19 stood in review while Round 18 was built,
    // so the count was one again until the Round 18 closure
    // landed slice 19 and the count returned to zero.
    // The round five pin
    // below holds the slice 6 line itself, the round six pin
    // below holds the slice 7 line, the round seven pin
    // below holds the slice 8 line, the round eight pin
    // below holds the slice 9 line, the round nine pin
    // below holds the slice 10 line, the round ten pin
    // below holds the slice 11 line, the round eleven
    // pin below holds the slice 12 line, the round
    // twelve pin below holds the slice 13 line, the round
    // thirteen pin below holds the slice 14 line, the
    // round fourteen pin below holds the slice 15 line,
    // and the round fifteen pin below holds the slice 16
    // line, and the round sixteen pin below holds the
    // slice 17 line, and the round seventeen pin below
    // holds the slice 18 line, and the round eighteen
    // pin below holds the slice 19 line.
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
fn readme_reports_the_round_six_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-13 and DH-14 met. They stand in full form in CONTRACT.md under the Round 6 obligations, and Round 6 closed on the merged slice 7 after three reviews approving with no findings.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 7 has landed. It carries the championship, a reading across recorded series naming the series each model took and the championship table over series won, race wins, and the aggregate margin, with the championship example over the two recorded series and its golden under examples.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn readme_reports_the_round_seven_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-15 and DH-16 met. They stand in full form in CONTRACT.md under the Round 7 obligations, and Round 7 closed on the merged slice 8 after three reviews approving with no findings.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 8 has landed. It carries the flip reading over a recorded series, the scratch winner and the adjusted winner of each race and the standing of the race as flipped, held, or no result, with the closing counts of races, flipped, held, and no results, with the flip example over the handicap series and its golden under examples, and the flip command beside race, closecall, series, and championship.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn readme_reports_the_round_eight_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-17 and DH-18 met. They stand in full form in CONTRACT.md under the Round 8 obligations, and Round 8 closed on the merged slice 9 after three reviews, no repair pass, and the standards mutation condition discharged on executed runs.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 9 has landed. It carries the totals reading over a recorded championship, the races entered, won, lost, and left without a result by each model and the token spend its attempts reported, counted under the counting walk that stops at each series decision, with the closing counts of series, races, and models, with the totals example over the standing championship and its golden under examples, and the totals command beside race, closecall, series, championship, and flip.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn readme_reports_the_round_nine_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-19 and DH-20 met. They stand in full form in CONTRACT.md under the Round 9 obligations, and Round 9 closed on the merged slice 10 after three reviews, a repair pass, and a discharging re-review.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 10 has landed. It carries the pairing reading over a recorded championship, the head to head record of each pair of models over the series they stood in together, with the closing counts of pairings, series, and races, with the pairing example over the standing championship and its golden under examples, and the pairing command beside race, closecall, series, championship, flip, and totals, and the fencing of the printed fields at the library surface under FIX-DH-16, and the README brought to the fleet README standard.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn readme_reports_the_round_ten_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-21 and DH-22 met. They stand in full form in CONTRACT.md under the Round 10 obligations, and Round 10 closed on the merged slice 11 after three reviews approving with no findings.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 11 has landed. It carries the comeback reading over a recorded championship, the classification of each series by its first decided race as won from behind or won from the front, one section for each series won from behind naming its decided races in race order and the loss of its winner in the first decided race and its winner with its race split, with the closing counts of series, from behind, and from the front, with the comeback example over the standing championship in its empty form and the constructed comeback example and their goldens under examples, and the comeback command beside race, closecall, series, championship, flip, totals, and pairing.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn readme_reports_the_round_eleven_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-23 and DH-24 met. They stand in full form in CONTRACT.md under the Round 11 obligations, and Round 11 closed on the merged slice 12 after three reviews, with the standards lane's two findings repaired and corrected before landing.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 12 has landed. It carries the distance reading over a recorded championship, the classification of each series by the wins of its loser when the decision fell as standing the distance, standing without reply, or standing in between, one section for each series gone the distance naming its decided races in race order and its winner with its race split, with the closing counts of series, the distance, without reply, in between, and no winner, with the distance example over the standing championship in its empty form and the constructed distance example and their goldens under examples, and the distance command beside race, closecall, series, championship, flip, totals, pairing, and comeback.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn readme_reports_the_round_twelve_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-25 to DH-28 met, with FIX-DH-17, FIX-DH-18, and FIX-DH-19. They stand in full form in CONTRACT.md under the Round 12 obligations, and Round 12 closed on the merged slice 13 after three reviews, with the repair pass landed before the merge.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 13 has landed. It carries the format version rule under FIX-DH-17, every format this repo owns standing at version 1 with the version refused in the phrasing of the surface that reads it, the formats reference at FORMATS.md restating every shape in one place, the race specification and the run command that runs its two attempts through the local runner and writes its race record, with the duel example and its pins under examples, the rally reading over a recorded championship, the classification of each decided series by whether its winner ever stood behind on the tally of decided races as a rally or wire to wire, one section for each rally series naming its decided races in race order and the deepest its winner stood behind and its winner with its race split, with the closing counts of series, rally, wire to wire, and no winner, with the rally example over the standing championship in its empty form and the constructed rally example and their goldens under examples, the survey championship over six constructed series at drawn realistic shapes with its goldens under examples, and the switchyard later obligation retired under FIX-DH-18, and the run and rally commands beside race, closecall, series, championship, flip, totals, pairing, comeback, and distance.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn readme_reports_the_round_thirteen_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-29 to DH-31 met, with FIX-DH-20. They stand in full form in CONTRACT.md under the Round 13 obligations, and Round 13 closed on the merged slice 14 after three reviews, with the record corrections settled by execution before the merge.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 14 has landed. It carries the deadlock reading over a recorded championship, the classification of each decided series by whether its two tallies ever stood level on the way to the decision as deadlocked or clear, one section for each deadlocked series naming its decided races in race order and the count of its decided races that left the tallies level and its winner with its race split, with the closing counts of series, deadlocked, clear, and no winner, with the deadlock example over the standing championship in its empty form and the constructed deadlock example and their goldens under examples, the league championship over ten constructed series at drawn realistic shapes with its goldens under examples, the specification loader pins completed under FIX-DH-20, and the deadlock command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, and rally.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn readme_reports_the_round_fourteen_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-32 and DH-33 met. They stand in full form in CONTRACT.md under the Round 14 obligations, and Round 14 closed on the merged slice 15 after three reviews, with the record corrections settled by execution before the merge.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 15 has landed. It carries the streak reading over a recorded championship, the classification of each decided series by the longest run of consecutive decided races won by its winner as strung or single, one section for each strung series naming its decided races in race order and the longest run of its winner and its winner with its race split, with the closing counts of series, strung, single, and no winner, with the streak example over the standing championship in section form and the constructed streak example and their goldens under examples, the closing counts helper of the test suite standing in one home in the shared test module, and the streak command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, and deadlock.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn readme_reports_the_round_fifteen_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-34 and DH-35 met. They stand in full form in CONTRACT.md under the Round 15 obligations, and Round 15 closed on the merged slice 16 after three reviews approving with findings and no repair owed.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 16 has landed. It carries the cluster reading over a recorded championship, the classification of each decided series by the longest run of consecutive decided races won by its loser as clustered or scattered, one section for each clustered series naming its decided races in race order and the longest run of its loser and its winner with its race split, with the closing counts of series, clustered, scattered, and no winner, with the cluster example over the standing championship in its empty form and the constructed cluster example and their goldens under examples, and the cluster command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, and streak.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn readme_reports_the_round_sixteen_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-36 and DH-37 met. They stand in full form in CONTRACT.md under the Round 16 obligations, and Round 16 closed on the merged slice 17 after three reviews, with the two test text Minors of the reviews repaired in the slice.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 17 has landed. It carries the closing reading over a recorded championship, the classification of each decided series by the closing run of consecutive decided races won by its winner as in a run or alone, one section for each series in a run naming its decided races in race order and the closing run of its winner and its winner with its race split, with the closing counts of series, in a run, alone, and no winner, with the closing example over the standing championship with the panel series in its section alone and the constructed closing example and their goldens under examples, and the closing command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, and cluster.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn readme_reports_the_round_seventeen_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-38 and DH-39 met. They stand in full form in CONTRACT.md under the Round 17 obligations, and Round 17 closed on the merged slice 18 after three reviews, with the two Minors of the reviews repaired in the slice.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 18 has landed. It carries the opening reading over a recorded championship, the classification of each decided series by the opening run of consecutive decided races won by its winner as in a run, alone, or from behind, one section for each series in a run naming its decided races in race order and the opening run of its winner and its winner with its race split, with the closing counts of series, in a run, alone, from behind, and no winner, with the opening example over the standing championship with both of its series in section form and the constructed opening example and their goldens under examples, and the opening command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, and closing.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn readme_reports_the_round_eighteen_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, the Round 18 fold and the repair amendment FIX-DH-23 met. They stand in full form in CONTRACT.md under the Round 18 obligations, and Round 18 closed on the merged slice 19 after three reviews, with the fold's neutrality proof executed at the fold commit and re-executed by the red team.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 19 has landed. It carries the fold of the longest run computation of the streak reading and the cluster reading into the one helper of the championship module, both readings reading the longest run of a series from the one home from this round, with no command added, no figure moved, and no golden changed, under the repair amendment FIX-DH-23.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn contract_states_the_winner_line_in_the_house_form() {
    // The FIX-DH-21 pin. It fails on the pre repair tree by
    // content, where the DH-37 and DH-39 entries state the
    // winner line in the form no module prints and no
    // FIX-DH-21 entry stands, and it passes on the repaired
    // tree. It reads the contract text alone and executes
    // no reading.
    let text = read(&repo_root().join("CONTRACT.md"));
    let house = "the words series winner, a comma, the series winner, a comma, the first model of the series, the count of its counted race wins, and the words race wins, a comma, the second model of the series, the count of its counted race wins, and the words race wins, a comma, and the count of its counted races holding no result and the words no results";
    let wrong = "the words series winner, a comma, the winning model, a comma, the count of its decided races, the word decided, a comma, the count of its no results, and the words no results";
    let dh37 = text
        .lines()
        .find(|line| line.starts_with("- DH-37."))
        .expect("the DH-37 entry");
    assert!(dh37.contains(house));
    let dh39 = text
        .lines()
        .find(|line| line.starts_with("- DH-39."))
        .expect("the DH-39 entry");
    assert!(dh39.contains(house));
    // The wrong form stands nowhere outside the quotation
    // in the FIX-DH-21 entry that records the restatement.
    let wrong_lines: Vec<&str> = text.lines().filter(|line| line.contains(wrong)).collect();
    assert_eq!(wrong_lines.len(), 1);
    assert!(wrong_lines[0].starts_with("- FIX-DH-21."));
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

#[test]
fn formats_reference_names_every_format_its_version_and_its_test_file() {
    // The reference FIX-DH-17 names. Each versioned shape's
    // section states its version posture and the test file
    // that pins the shape, so a shape dropped from the
    // reference, a version dropped from a section, or a test
    // file dropped from a section fails here.
    let text = read(&repo_root().join("FORMATS.md"));
    let sections: Vec<&str> = text.split("\n## ").collect();
    let section = |name: &str| -> String {
        sections
            .iter()
            .find(|part| part.starts_with(name))
            .map(|part| part.to_string())
            .unwrap_or_else(|| panic!("FORMATS.md names no section {name}"))
    };
    let versioned = [
        ("The race record", "test_races.rs"),
        ("The runner result file", "test_runner.rs"),
        ("The adjudication entry", "test_close_calls.rs"),
        ("The panel", "test_panels.rs"),
        ("The series file", "test_series.rs"),
        ("The championship file", "test_championship.rs"),
        ("The standings file", "test_races.rs"),
        ("The pilot run record", "test_pilot.rs"),
        ("The race specification", "test_runs.rs"),
    ];
    for (name, test_file) in versioned {
        let body = section(name);
        assert!(
            body.contains("version 1 under FIX-DH-17"),
            "the {name} section states no version"
        );
        assert!(
            body.contains(test_file),
            "the {name} section names no test file"
        );
    }
}

#[test]
fn formats_reference_names_no_version_for_the_envelopes() {
    // The second pin FIX-DH-17 names. The envelope shapes are
    // the routes' own shapes and carry no deadheat version, so
    // a version stated for them in the reference fails here.
    let text = read(&repo_root().join("FORMATS.md"));
    let start = text
        .find("\n## The envelopes")
        .expect("FORMATS.md names the envelope section");
    let body = &text[start..];
    assert!(body.contains("no deadheat version"));
    assert!(!body.contains("version 1 under FIX-DH-17"));
    assert!(body.contains("test_seats.rs"));
}

#[test]
fn agents_annex_states_the_local_directory_runner() {
    // The annex refresh FIX-DH-18 states. The gate sentence
    // lost the switchyard sentence whole, and the refreshed
    // sentence stands in its place, so a stale annex fails
    // here.
    let agents = read(&repo_root().join("AGENTS.md"));
    assert!(agents.contains(
        "The runner runs each attempt in its own local directory under one race workspace."
    ));
    assert!(!agents.contains("switchyard"));
    assert!(!agents.contains("Switchyard"));
}

#[test]
fn readme_shows_the_comeback_distance_rally_deadlock_streak_cluster_closing_opening_and_run_sections(
) {
    // The Usage sections the Round 12, Round 13, Round
    // 14, Round 15, Round 16, and Round 17 rounds stand
    // on. Each section lead stands in the README, so a
    // dropped section fails here. Altered at the Round 15 slice
    // for its named mechanical consequence, the cluster
    // section standing new in the README, so the pin
    // gains its lead and its name grows by it. Altered
    // at the Round 16 slice for its named mechanical
    // consequence, the closing section standing new in
    // the README, so the pin gains its lead and its name
    // grows by it. Altered at the Round 17 slice for its
    // named mechanical consequence, the opening section
    // standing new in the README, so the pin gains its
    // lead and its name grows by it.
    let readme = read(&repo_root().join("README.md"));
    for lead in [
        "The comeback command reads a championship file",
        "The distance command reads a championship file",
        "The rally command reads a championship file",
        "The deadlock command reads a championship file",
        "The streak command reads a championship file",
        "The cluster command reads a championship file",
        "The closing command reads a championship file",
        "The opening command reads a championship file",
        "The run command takes a race specification file",
    ] {
        assert!(readme.contains(lead), "the README shows no section: {lead}");
    }
}
