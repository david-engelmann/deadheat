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
    assert!(text.contains("Closed perfection rounds, 31 of 100"));
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
    // Slice 20 stood in review while Round 19 was built,
    // so the count was one again until the Round 19 closure
    // landed slice 20 and the count returned to zero.
    // Slice 21 stood in review while Round 20 was built,
    // so the count was one again until the Round 20 closure
    // landed slice 21 and the count returned to zero.
    // Slice 22 stood in review while Round 21 was built,
    // so the count was one again until the Round 21 closure
    // landed slice 22 and the count returned to zero.
    // Slice 23 stood in review while Round 22 was built,
    // so the count was one again until the Round 22 closure
    // landed slice 23 and the count returned to zero.
    // Slice 24 stood in review while Round 23 was built,
    // so the count was one again until the Round 23 closure
    // landed slice 24 and the count returned to zero.
    // Slice 25 stood in review while Round 24 was built,
    // so the count was one again until the Round 24 closure
    // landed slice 25 and the count returned to zero.
    // Slice 26 stood in review while Round 25 was built,
    // so the count was one again until the Round 25 closure
    // landed slice 26 and the count returned to zero.
    // Slice 27 stood in review while Round 26 was built,
    // so the count was one again until the Round 26 closure
    // landed slice 27 and the count returned to zero.
    // Slice 28 stood in review while Round 27 was built,
    // so the count was one again until the Round 27 closure
    // landed slice 28 and the count returned to zero.
    // Slice 29 stood in review while Round 28 was built,
    // so the count was one again until the Round 28 closure
    // landed slice 29 and the count returned to zero.
    // Slice 30 stood in review while Round 29 was built,
    // so the count was one again until the Round 29 closure
    // landed slice 30 and the count returned to zero.
    // Slice 31 stood in review while Round 30 was built,
    // so the count was one again until the Round 30 closure
    // landed slice 31 and the count returned to zero.
    // Slice 32 stood in review while Round 31 was built,
    // so the count was one again until the Round 31 closure
    // landed slice 32 and the count returned to zero. The round five pin
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
    // pin below holds the slice 19 line, and the round
    // nineteen pin below holds the slice 20 line, and
    // the round twenty pin below holds the slice 21
    // line, and the round twenty one pin below holds
    // the slice 22 line, and the round twenty two pin
    // below holds the slice 23 line, and the round
    // twenty three pin below holds the slice 24
    // line, and the round twenty four pin below holds
    // the slice 25 line, and the round twenty five
    // pin below holds the slice 26 line, and the round
    // twenty six pin below holds the slice 27 line,
    // and the round twenty seven pin below holds the
    // slice 28 line, and the round twenty eight pin
    // below holds the slice 29 line, and the round
    // twenty nine pin below holds the slice 30 line, and the round
    // thirty pin below holds the slice 31 line, and the round
    // thirty one pin below holds the slice 32 line.
    assert_eq!(text.matches("built and in review").count(), 1);
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
fn readme_reports_the_round_nineteen_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-40 and DH-41 met. They stand in full form in CONTRACT.md under the Round 19 obligations, and Round 19 closed on the merged slice 20 after three reviews, with the pin comments restated to their executed postures in the slice.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 20 has landed. It carries the turns reading over a recorded championship, the classification of each decided series by the count of the changes of hand over its decided record as turned or steady, one section for each turned series naming its decided races in race order and the changes of hand of the series and its winner with its race split, with the closing counts of series, turned, steady, and no winner, with the turns example over the standing championship with the pilot series in its section alone and the constructed turns example and their goldens under examples, and the turns command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, and opening.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn readme_reports_the_round_twenty_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-42 and DH-43 met. They stand in full form in CONTRACT.md under the Round 20 obligations, and Round 20 closed on the merged slice 21 after three reviews, with the red team clean on every charge, the formatting repair landed in the slice, and the two CodeRabbit findings repaired in the fix-forward under FIX-DH-25.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 21 has landed. It carries the lead reading over a recorded championship, the classification of each decided series by the largest lead either model held over its decided record read against the winning margin of the series as wide or narrow, one section for each wide series naming its decided races in race order and the largest lead of the series and its winner with its race split, with the closing counts of series, wide, narrow, and no winner, with the lead example over the standing championship in its empty form and the constructed lead example and their goldens under examples, and the lead command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, opening, and turns.";
    assert!(text.lines().any(|line| line == slice_line));
}
#[test]
fn readme_reports_the_round_twenty_one_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-44 and DH-45, with FIX-DH-26, met. They stand in full form in CONTRACT.md under the Round 21 obligations, and Round 21 closed on the merged slice 22 after three reviews, with the red team approving on recomputation of every crossings closing, the relations pin's winner line parse repaired to the series report's final tally, and the M-F clauses of L-4, L-16, and L-17 restated to their computed postures.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 22 has landed. It carries the crossings reading over a recorded championship, the classification of each decided series by the count of the crossings over its decided record read against the count of its changes of hand as through or short, one section for each through series naming its decided races in race order and the crossings of the series and its winner with its race split, with the closing counts of series, through, short, and no winner, with the crossings example over the standing championship with the panel series in its section alone and the constructed crossings example and their goldens under examples, and the crossings command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, opening, turns, and lead.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn readme_reports_the_round_twenty_two_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-46 and DH-47, with FIX-DH-27, met. They stand in full form in CONTRACT.md under the Round 22 obligations, and Round 22 closed on the merged slice 23 after three reviews, with the three reviews approving with findings and every finding cured in the repair commit, FIX-DH-27 restating the DH-47 examples clause in place with the wrong sentence confined to the amendment's quotation, pin M-21 asserting the restated form with its split executed both ways, and the M-13 and M-15 comments restated to their true and executed postures.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 23 has landed. It carries the tenure reading over a recorded championship, the classification of each decided series by the count of the decided races after which its winner's tally stood above its loser's read against the same count for its loser as ahead, level, or behind, one section for each series standing ahead naming its decided races in race order and the front count of the series and its winner with its race split, with the closing counts of series, ahead, level, behind, and no winner, with the tenure example over the standing championship with both of its series in section form and the constructed tenure example and their goldens under examples, and the tenure command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, opening, turns, lead, and crossings.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn readme_reports_the_round_twenty_three_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-48 and DH-49 met. They stand in full form in CONTRACT.md under the Round 23 obligations, and Round 23 closed on the merged slice 24 after three reviews, with the standards review approving with findings at two Observations, the spec review approving with findings at one Major cured in the slice by the fencing repair and one Minor standing against the design record, the red team approving with findings at three Observations on executed recomputation, and CodeRabbit returning no comments.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 24 has landed. It carries the depths reading over a recorded championship, the classification of each decided series by the depth of its winner over the decided record read against the margin the series was won by as deep, matched, or shallow, one section for each series standing deep naming its decided races in race order and the deepest deficit of the series and its winner with its race split, with the closing counts of series, deep, matched, shallow, and no winner, with the depths example over the standing championship in its empty form and the constructed depths example and their goldens under examples, and the depths command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, opening, turns, lead, crossings, and tenure.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn readme_reports_the_round_twenty_four_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, the Round 24 tally fold, with FIX-DH-28, met. They stand in full form in CONTRACT.md under the Round 24 obligations, and Round 24 closed on the merged slice 25 after three reviews, with FIX-DH-28 standing spent on the fold, the spec review approving with findings at two Majors, three Minors, and two Observations, its census Major standing against the design record, whose 894 stands as its own arithmetic erratum against the settled census of 893, and its evidence Major discharged by the executions, the standards review approving with findings at one Minor and one Observation, its Minor on the positional return dismissed as the contracted form FIX-DH-28 names, and the red team approving with findings at two Minors and three Observations, neutrality standing byte identical over 96 renderings and both identities attacked exhaustively with no reachable parting, the executions standing at 39 sets with 38 exact and one moved set, the deadlocks M-A set failing D-2, D-3, and D-9 against its stated D-2 and D-3, the movement mechanised to the mutant walk producing a level final standing past the decision, a shape the true walk cannot leave, proven by the pre fold control reproducing the stated set and no shipped behaviour moving, and the baseline tripwire settled at 398 recorded and 244 walked over the 44 series files now standing and at 386 recorded and 232 walked over the 42 that stood at the Round 23 base, the parting standing in the constructed depths pair Round 23 itself landed, with count_series standing byte identical.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 25 has landed. It carries the fold of the tally computation of the lead, crossings, deadlock, rally, tenure, and depths readings into the one helper of the championship module, all six readings reading the tallies of a decided series from the one home from this round, with no command added, no figure moved, and no golden changed, under the repair amendment FIX-DH-28.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn readme_reports_the_round_twenty_five_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-50 and DH-51 met. They stand in full form in CONTRACT.md under the Round 25 obligations, and Round 25 closed on the merged slice 26 after three reviews, with the spec review approving with findings at one Minor and four Observations, its Minor standing against the design record on the sibling golden count, the standards review approving with findings at two Observations, and the red team approving with findings at one Minor and two Observations, its Minor standing against the design record on the comebacks closing prose, and CodeRabbit returning no comments on the slice, with FIX-DH-29 standing free and unspent, the sibling golden count standing as a named residual and not as met work, the design record's section 1.9 list naming depths among the spells pair's sibling readings while no depths golden stands over the pair, the depths closing over the pair standing computed in that section and driven true on the shipped binary, and the repair priced at one golden, one E-3 arm, and C-1 at 205 and 201, the design record standing corrected at four further places as its errata, its section 1.8 golden names standing as shorthand where the shipped goldens stand in the family's championship form, its W-13 prose standing at a depth of 3 where its own engine computes 2 and the shipped pin states 2, its section 1.7 naming the survey's bolt and crest and bolt and drift series and the league's apex and ember and bolt and drift series the other way round with every figure and closing unaffected, and its section 1.9 comebacks closing prose carrying a no winner segment the report never prints with E-3 asserting the true rendered tail, the build's fresh extraction gate re-run standing unexecuted on the write fault and carried as unexecuted, its place discharged by the staged tree standing blob identical to the pushed tree and by the hosted gate standing green on the exact head, and the red team's whole suite grain observation standing on its V-4 and V-7 spot executions, the split's universe standing as the pin file with both executions exact at that grain and the same mutants felling the enrolment, fencing, and sweep pins that assert the same renderings through other files at the whole suite grain.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 26 has landed. It carries the spells reading over a recorded championship, the classification of each decided series by the longest spell its winner stood behind over the decided record read against the behind count of the series as unbroken, broken, or never behind, one section for each series standing broken naming its decided races in race order and the longest spell and behind count of the series and its winner with its race split, with the closing counts of series, unbroken, broken, never behind, and no winner, with the spells example over the standing championship in its empty form and the constructed spells example and their goldens under examples, and the spells command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, opening, turns, lead, crossings, tenure, and depths.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn readme_reports_the_round_twenty_six_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-52 and DH-53 met. They stand in full form in CONTRACT.md under the Round 26 obligations, and Round 26 closed on the merged slice 27 after three reviews, with the spec review approving with findings at one Observation, its Observation standing against the design record on the section 1.8 standings enumeration for drill two, the standards review approving with findings at one Observation, and the red team approving with findings at two Observations, and CodeRabbit returning one inline comment on the slice adjudicated not valid, the printed number standing as DH-13's documented count of the race in the record and a renumbering of reigns alone breaking the family's one form, with FIX-DH-29 standing free and unspent, the fresh extraction gate re-run carried from the Round 25 closure standing discharged by this round's warm-up on main at 921 passed and 0 failed, the sibling golden residual standing discharged, the depths golden over the spells pair standing enrolled with its E-3 arm and the C-1 count restated at 221 and 217, the design record's R-15 account of V-4 standing as a records item, its prose reading a reign of five in a front count of eight where the computation over the tailed fixture reads a reign of five in a front count of seven classing the series broken and the pin failing V-4 on its figures either way, the design record's section 1.8 enumeration standing with its sixth standing for drill two reading level where its own winner sequence yields ahead, its figures and the shipped golden agreeing with the recount, the red team's parity record standing as stated, the run relations of DH-52 holding over the file corpus and a 20,000 series sweep and the loose parity paraphrase standing nowhere in DH-52 or the design record, the record count semantics standing unpinned at the section surface family-wide as a candidate pin for a later round, this round's survey reigns golden already pinning the shape in substance on the red team's note, the build lane standing ended without a handoff and the completion lane's adoption standing as the build report's section 6 records them, and the four errata of the Round 25 design record standing as records items.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 27 has landed. It carries the reigns reading over a recorded championship, the classification of each decided series by the longest reign of its winner over the decided record read against the front count of the series as unbroken or broken, one section for each series standing broken naming its decided races in race order and the longest reign and front count of the series and its winner with its race split, with the closing counts of series, unbroken, broken, and no winner, with the reigns example over the standing championship in its empty form and the constructed reigns example and their goldens under examples, and the reigns command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, opening, turns, lead, crossings, tenure, depths, and spells.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn readme_reports_the_round_twenty_seven_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, the record count statement, with FIX-DH-29, met. They stand in full form in CONTRACT.md under the Round 27 obligations, and Round 27 closed on the merged slice 28 after three reviews, with FIX-DH-29 standing spent on the record count statement at both seats, the sentence standing added in place to the stated relations of DH-51 and to the stated relations of DH-53, the two pins R-21 in rust/tests/test_reigns.rs and W-21 in rust/tests/test_spells.rs standing as its witnesses, the spec review approving with no findings, the standards review approving with findings at three Observations, and the red team approving with no findings, its first seating standing ended without a handoff or a banked report and its scratch standing adopted by the reseat only after hash verification of all 440 files per tree, the reseat's report standing as the review of record, and CodeRabbit returning no inline comments on the slice, its completed review standing read by the owner at the merge gate at zero inline comments, zero submitted reviews, and no actionable comments, with the design record's section 0.1 naming rust/src/main.rs for the command table where it stands at rust/src/cli.rs line 1137, declared 26 and counted 26, standing for the errata queue, the standards review's phrasing Observation standing recorded, the relations sentence standing a third time in lowercase inside the FIX-DH-29 entry's own quotation of the form it adds, the build report's count of exactly two naming the capitalised obligation instances, the red team's edge account standing as executed, forty drives at no disagreement, the statement's qualifier holding at the counted record's tail, a no-result race after the decision counting for nothing, and FIX-DH-30 standing free and unspent.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 28 has landed. It carries the record count statement of FIX-DH-29, the numbering of a section's decided races stated as the count of the race in the record in the stated relations of DH-51 and DH-53, with one pin in each of rust/tests/test_reigns.rs and rust/tests/test_spells.rs asserting its reading's whole report over a series whose first recorded race holds no winner, its section printing the decided races at their counts in the record with the count of the race holding no winner standing skipped, and no new command, no new reading, no figure moved, and no golden changed.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn readme_reports_the_round_twenty_eight_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-54 and DH-55 met. They stand in full form in CONTRACT.md under the Round 28 obligations, and Round 28 closed on the merged slice 29 after three reviews, with the standards review approving with findings at one Minor and one Observation, its Minor standing charged to the slice, the place module's doc head naming the posture of the depths module where the design charges the posture of the reigns module and the body follows it, cured in slice by the repair commit 6316c238 standing on the slice head, the one word depths standing replaced by reigns in the doc comment with no other word moving, the repair standing verified by the owner at exactly that one word with the repair head's hosted gate and gate-stable standing at success twice each, the spec review approving with findings at three Observations, each standing against the design record and none charged to the slice, and the red team approving with no findings, its independent oracle standing at no disagreement over all nineteen championships, its spot executions of V-5, V-8, and g15 standing name for name against the executed record, and its two points standing recorded against the owner's brief phrasing and not the slice, a first attainment at the last decided race standing arithmetically impossible under DH-54, the latest attainable shape standing executed in its place, and a series standing never behind bearing depth nought under the obligation's own text, and CodeRabbit returning no inline comments on the slice, its completed review standing read in full by the standards lane at zero inline comments and zero submitted reviews, with the slice standing merged as pull request 65 at de2b2324, main's tree standing verified equal to the repaired slice tree 9cd8314c, the census standing at 980, the design record's section 5 load bearing list naming the constructed pair's whole report pin as falling under V-4 where the pin's own comment and the build's corrected V-4 execution read it passing, the same list numbering that pin PL-1 where the committed file labels it PL-2 and labels the standing championship pin PL-1, the build's first V-4 construction standing void in the open with its mechanism proven, the same slip standing in the design lane's unexecuted draft of the variant, the design's gap hunt standing negative over its fifteen mutations, both passing variants standing proven equivalent and no gap standing charged, the five items of the design record's errata queue standing as records items untouched by this round's text, and FIX-DH-30 standing free and unspent.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 29 has landed. It carries the place reading over a recorded championship, the classification of each decided series by the place of the deepest deficit of its winner over the decided record read against the depth itself as early, late, or never behind, one section for each series standing late naming its decided races in race order and the depth and place of the series and its winner with its race split, with the closing counts of series, early, late, never behind, and no winner, with the place example over the standing championship in its empty form and the constructed place example and their goldens under examples, and the place command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, opening, turns, lead, crossings, tenure, depths, spells, and reigns.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn readme_reports_the_round_twenty_nine_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-56 and DH-57 met. They stand in full form in CONTRACT.md under the Round 29 obligations, and Round 29 closed on the merged slice 30 after three reviews, with the standards review approving with findings at one Minor and three Observations, its Minor standing charged to the slice, four pin posture comments in rust/tests/test_peak.rs stating passes the pins cannot hold, the PK-4 comment recording passes under W-7 and W-9 where the pin falls under both, the PK-8 and PK-11 comments each asserting a figure line verbatim that W-7 renders exchanged while recording W-7 as a pass, and the PK-14 comment recording a pass under W-9 where the pin falls, cured in slice by the repair commit 5c1bad21 standing on the slice head, the four comments' failing lists standing restated with no assertion moving and the census unmoved, the PK-18 pin standing as a characterization pin by design, failing under no variant and charged nowhere, the spec review approving with findings at four Observations, each standing against the design record and none charged to the slice, and the red team approving with findings at three Observations, no finding standing against the slice's content, its differential standing executed on the base side over 3,753 drives and its head side legs standing unexecuted on the write fault, and CodeRabbit returning one inline comment on the slice, standing valid in full as the same finding as the standards Minor at the same seat and not a second one, with no new comment standing on the repaired head, with the slice standing merged as pull request 67 at 632fcdcc, main's tree standing verified equal to the repaired slice tree 284f93bf, the census standing at 1008, the W family W-1 to W-10 standing unexecuted whole on the write fault against the form tree at thirteen refused attempts for W-1, twelve each for W-2, W-3, W-4, W-5, W-6, W-8, and W-10, fourteen for W-7, and thirteen for W-9, every refusal standing as a fault signature with no assertion failure anywhere, the H-3 and H-5 spot executions against the head tree standing unexecuted at twelve refused attempts each, the candidate pins standing unexecuted on the base tree, the fencing pin's posture under the W family standing undecided between the design note's section 7 account of the pin standing unmoved under the whole family and the build report's computed account of the pin falling under W-1, W-5, and W-7, the build's local form gate and the fresh extraction gate's local cargo legs standing unexecuted on the same fault with the hosted gate on the exact head standing as the suite's executed evidence of record, the design note's section 7 unions standing understated at the falls of PK-4, PK-8, and PK-11 under W-7 and of PK-4 and PK-14 under W-9, cured in the slice's comments by the repair and standing in the note as the record seat of the same understatement, the note's section 8 assembled contract figure standing at 347,761, one short of the executed 347,762 with the section standing byte equal either way, the note's section 7 fencing description standing parted from the built F-1 form, the note's section 8 grouping the handicap sweep's restatement with the generated step where the slice carries it in the tests step in the Round 28 executed form, the note's section 7 naming the standing championship peak goldens in the E-3 form where the built file labels that pin E-1 and the sibling readings pin E-3, the peak golden count standing at twenty against the note's nineteen named beside the pair's own, the build report's repair narrative standing inaccurate in the red team's first Observation, its account of the third repair commit's restatement of the round four pin's count parting from the remote, the status step standing shown leaving the pin untouched and the repair commit fe60d404 standing shown carrying the restatement alone, and FIX-DH-30 standing free and unspent.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 30 has landed. It carries the peak reading over a recorded championship, the classification of each decided series by the place of the largest lead of its winner over the decided record read against the lead itself as early or late, one section for each series standing late naming its decided races in race order and the largest lead and place of the series and its winner with its race split, with the closing counts of series, early, late, and no winner, with the peak example over the standing championship in its empty form and the constructed peak example and their goldens under examples, and the peak command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, opening, turns, lead, crossings, tenure, depths, spells, reigns, and place.";
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
fn readme_shows_the_comeback_distance_rally_deadlock_streak_cluster_closing_opening_turns_lead_and_run_sections(
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
    // lead and its name grows by it. Altered at the
    // Round 19 slice for its named mechanical
    // consequence, the turns section standing new in
    // the README, so the pin gains its lead and its
    // name grows by it. Altered at the Round 20 slice
    // for its named mechanical consequence, the lead
    // section standing new in the README, so the pin
    // gains its lead and its name grows by it.
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
        "The turns command reads a championship file",
        "The lead command reads a championship file",
        "The run command takes a race specification file",
    ] {
        assert!(readme.contains(lead), "the README shows no section: {lead}");
    }
}

#[test]
fn readme_reports_the_round_thirty_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-58 and DH-59 met. They stand in full form in CONTRACT.md under the Round 30 obligations, and Round 30 closed on the merged slice 31 after three reviews, with the standards review approving with findings at two Observations, none standing charged to the slice, the first pricing the X family as computed and unexecuted, an execution debt carried to this closure, and the second naming the build report's repair narrative understating the repair diff by one seat at RC-8's comment prose, a records item against the build report alone, the spec review approving with findings at four Observations, each standing against the design record and none charged to the slice, the note's section 2 naming bolt-ember's answer as seven where the data and the golden read five, the note's section 2 naming bolt-ember as the immediate and late witness against DH-56 where the series stands delayed and the contract's own section names the survey apex-drift series, a naming the red team's first Observation shares, the note's section 3 build seats naming the place module where the shipped recovery module's doc head stands in the reigns posture, and the note's section 8 drafting the slice line in landed form where its own count charge requires the in review opening the build carried, and the red team approving with findings at three Observations, no finding standing against the slice's content, its independent oracle reproducing the population whole over the 49 decided series at 18 immediate, 8 delayed, and 23 never behind, all 21 recovery goldens of the head tree standing byte equal to its own renders, the build's repair standing reproduced at the oracle, the original RC-8 shape reading (immediate, 2, 1) from the loser's seat at a target of four and the repaired pin reading (immediate, 3, 2) at a target of five, its break attempts standing exhaustive over 636 decided winner sequences with no break found, and its differential standing executed on the base side over 3,976 drives, identical on its re-run, with its head side standing unexecuted, and CodeRabbit returning no submitted review and no inline comment on the slice, with the slice standing merged as pull request 69 at ec73af90, main's tree standing verified equal to the slice tree b79e4e4e, the census standing at 1036, the build's seventh commit standing as a tests repair at 3e1281b8 beyond the six charged, curing defects of the build's own test authorship at RC-2, RC-13, and RC-8 in rust/tests/test_recovery.rs alone with the census unmoved, the 34 new goldens standing rendered by the build's transliteration proven byte for byte against all 166 standing sibling goldens and in ordered agreement with the design engine over all 49 series, the built binary standing unable to render them before the window closed and that substitution standing named in the open, the sweep standing restated at 305 goldens and 301 unadjusted, the control standing executed on the standing main tree at its fifth attempt at 1008 passed and 0 failed, W-1 standing executed at one attempt with exactly 24 falls, name for name equal to its expected set, the cured comments of Round 29 cashing under it at PK-4, PK-8, and PK-11, the fencing pin F1 falling under it and its posture standing decided there, its falls under W-5 and W-7 standing open, PK-14's entries under W-7 and W-9 standing open, and no W mutation passing whole anywhere executed, the carried family standing unexecuted on the write fault at eleven refused attempts for W-2, six for W-3, five each for W-4 to W-10, and five each for H-3 and H-5 across the build and red team lanes, the red team's 46 attempts standing counted inside those figures with its three refused tarball attempts and its 49 path blob grinder landing every file at zero bytes beside them, the X family standing computed and unexecuted whole, the candidate pins standing unexecuted on the pre-round tree, the form gate and the fresh extraction gate standing unexecuted on the same fault with the hosted gate on the exact head standing as the suite's executed evidence of record, the design note's RC-8 shape at a target of four standing refuted by execution and corroborated at two seats, and FIX-DH-30 standing free and unspent.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 31 has landed. It carries the recovery reading over a recorded championship, the classification of each decided series by the answer of its winner to the deepest deficit of the series over the decided record read against the depth plus one as immediate, delayed, or never behind, one section for each series standing delayed naming its decided races in race order and the depth and answer of the series and its winner with its race split, with the closing counts of series, immediate, delayed, never behind, and no winner, with the recovery example over the standing championship in its empty form and the constructed recovery example and their goldens under examples, and the recovery command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, opening, turns, lead, crossings, tenure, depths, spells, reigns, place, and peak.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn readme_reports_the_round_thirty_one_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-60 and DH-61 met. They stand in full form in CONTRACT.md under the Round 31 obligations, and Round 31 closed on the merged slice 32 after three reviews, with the standards review approving with findings at one Minor, the DH-60 stated relations sentence at CONTRACT.md line 554 misstating the tie case, the two figures standing equal where a longest run of the loser ties the run standing last while the sentence predicated apartness on a longest run standing earlier, verified by hand computation over the counterexample CodeRabbit raised, its one inline comment standing as the same item and dispositioned valid, and the Minor standing cured in slice by the repair commit feb7a7fc, a two word restatement of the one sentence touching CONTRACT.md alone with the census unmoved and the whole suite sweep convicting no pin literal, the spec review approving with findings at four Observations, each standing against the design record and none charged to the slice, the note's section 3 stating the added contract text at 6,433 characters and the assembled contract at 362,070 against the measured 6,432 and 362,069 with the span's bytes standing exact, the note's section 8 drafting the slice line in landed form against its own count charge, the note's section 8 phrasing the round four pin's comment chain as extended at the status step against section 9's closure charge, and the stated relations sentence's definite article reading loosely at the tie, and the red team approving with findings at three Observations, none standing against any shipped text, its independent oracle re-deriving the population exactly over both enrolments at 14 held, 33 single, and 4 without reply over the 51 decided series of the base and 15 held, 34 single, and 4 without reply over the 53 of the head, its stated relations standing checked as theorems with no violation and its eight break constructions finding no break, the first Observation standing against the design record, the Y unions of the note's section 7 omitting the handicap sweep whose standing pin carries the slice's laststand arm over the same twenty two goldens, the executed unions standing one pin larger at every variant when executions land, priced in its report before the executions were read, the second against the design record, the closing sentence of the same section placing the characterization pin LS-13 under every variant where its own unions and the pin's posture comment place it under none, and the third standing as the priced boundary of the reading, twenty three enrolled series carrying wins of the loser after the deciding race that the walk counts nowhere, DH-60 classing over the decided record the walk of DH-13 leaves, with the slice standing merged as pull request 71 at a844a519, main's tree standing verified equal to the repaired slice tree 7607fc93, the census standing at 1064, the build's seventh and eighth commits standing as tests repairs at 7450976b and e8b4c33f beyond the six charged, curing defects of the build's own test authorship in rust/tests/test_laststand.rs alone with the census unmoved, the second repair restating the whole count pin LS-11 to twenty nine, the design's figure, the build's mid lane count of thirty standing corrected in its report as its own miscount and withdrawn, the thirty five new goldens standing rendered by the build's transliteration and proven byte for byte by the hosted suite itself, the built binary standing unable to render them in the fault's windows and that substitution standing named in the open, the sweep standing restated at 340 goldens and 336 unadjusted, the control standing executed whole on the extracted base tree at the build at 1036 passed and 0 failed and on the Round 29 closure extract copy at the red team at 1008 passed and 0 failed, W-1 standing executed at the Round 30 build at exactly 24 falls and W-2 standing executed whole at this round's build at 19 falls, name for name equal to its expected set, the cured comments of Round 29 cashing under both where executed, and no W mutation passing whole anywhere executed, the carried execution family of Round 30 standing carried still at the combined attempt counts the Round 30 closure recorded, raised by the attempts of this round's warm-up and of this closure as the closure report records them, to twenty seven refused attempts for W-3, twenty two each for W-4, W-5, W-7, and W-9, and twenty one each for W-6, W-8, W-10, H-3, and H-5, its two open verdicts standing open, the fencing pin's falls under W-5 and W-7 and the cured comments' entries at PK-4 under W-7 and W-9, PK-8 and PK-11 under W-7, and PK-14 under W-9, this closure's third control attempt standing as a fault poisoned run at 879 passed and 129 failed with 118 os error signatures in its output, voided as an execution and counted as an attempt, the X family standing computed and unexecuted whole at its unions, its ground's materialization standing refused at the red team and again at this closure, the Y family standing computed and unexecuted whole at its unions with the sweep's priced expectation standing untested, its form tree assembly standing refused at the build and its tarball write refused at this closure after the fetch landed, the Round 31 candidate pins standing unexecuted on the pre-round tree, the form gate standing discharged in its hosted form on the slice head, the fresh extraction gate standing unexecuted on the same fault with the hosted gate on the exact head standing as the suite's executed evidence of record, the design note's section 3 carrying the cured sentence's original form and its one high figures standing as records items for the errata queue, and FIX-DH-31 standing free and unspent.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 32 has landed. It carries the last stand reading over a recorded championship, the classification of each decided series by the last run of its loser over the decided record as held, single, or without reply, one section for each series standing held naming its decided races in race order and the last stand of the series and its winner with its race split, with the closing counts of series, held, single, without reply, and no winner, with the laststand example over the standing championship in its empty form and the constructed laststand example and their goldens under examples, and the laststand command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, opening, turns, lead, crossings, tenure, depths, spells, reigns, place, peak, and recovery.";
    assert!(text.lines().any(|line| line == slice_line));
}

#[test]
fn readme_reports_the_round_thirty_two_status() {
    let text = read(&repo_root().join("README.md"));
    let standing = "- Contract, DH-62 and DH-63 now stand in full form in CONTRACT.md under the Round 32 obligations, and their standing is not claimed here, as Round 32 has not closed.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 33 is built and in review. It carries the first stand reading of the decided record, the constructed firststand championship beside the standing championships, and the firststand command beside race, closecall, series, championship, flip, totals, pairing, comeback, distance, run, rally, deadlock, streak, cluster, closing, opening, turns, lead, crossings, tenure, depths, spells, reigns, place, peak, recovery, laststand, and firststand.";
    assert!(text.lines().any(|line| line == slice_line));
}
