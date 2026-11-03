"""Setup pins. Each test can fail for a real reason against a repo that
lost a setup file or let its status drift from its contract."""
import pathlib, re
ROOT = pathlib.Path(__file__).resolve().parent.parent

def test_agents_carries_the_canonical_marker():
    text = (ROOT / "AGENTS.md").read_text(encoding="utf-8")
    assert "<!-- repo annex below this line -->" in text
    assert "39 repo storefront perfection program" in text

def test_readme_states_the_closed_round_count():
    text = (ROOT / "README.md").read_text(encoding="utf-8")
    assert re.search(r"Closed perfection rounds, 4 of 100", text)

def test_contract_carries_the_setup_obligations():
    text = (ROOT / "CONTRACT.md").read_text(encoding="utf-8")
    for oid in ("SETUP-1", "SETUP-2", "SETUP-3", "SETUP-4", "DH-1", "DH-5", "DH-6", "DH-7", "DH-8", "DH-9", "DH-10"):
        assert oid in text, oid

def test_readme_reports_the_contract_status():
    text = (ROOT / "README.md").read_text(encoding="utf-8")
    expected = (
        "- Contract, SETUP-1 to SETUP-4 and DH-1 to DH-6 met. "
        "DH-1 to DH-4 stand in full form in CONTRACT.md with the Round 1 "
        "repair amendments, and Round 1 closed on the merged slice 2 after "
        "three reviews, a repair pass, and an approving re-review."
    )
    assert expected in text.splitlines()

def test_readme_reports_the_round_three_status():
    text = (ROOT / "README.md").read_text(encoding="utf-8")
    lines = text.splitlines()
    standing = (
        "- Contract, DH-7 and DH-8 met. They stand in full form in "
        "CONTRACT.md with the Round 3 repair amendments, and Round 3 "
        "closed on the merged slice 4 after three reviews, a repair "
        "pass, and an approving re-review."
    )
    assert standing in lines
    slice_line = (
        "- Slice 4 has landed. It carries the "
        "adjudication seat over the two decision model routes, "
        "the live entry path into the race record, and the "
        "recorded close call pilot over both models, ten "
        "synthetic recorded races with their envelopes, "
        "adjudicated records, and run record under examples."
    )
    assert slice_line in lines

def test_readme_reports_the_round_four_status():
    text = (ROOT / "README.md").read_text(encoding="utf-8")
    lines = text.splitlines()
    standing = (
        "- Contract, DH-9 and DH-10 met. They stand in full form "
        "in CONTRACT.md with the Round 4 repair amendments, and "
        "Round 4 closed on the merged slice 5 after three "
        "reviews, a repair pass, and an approving re-review."
    )
    assert standing in lines
    slice_line = (
        "- Slice 5 has landed. It carries the panel "
        "reading over two recorded adjudication entries, the "
        "panel join over the Round 3 pilot records, and the "
        "recorded panel pilot over both models, ten panel "
        "records with their closecall goldens and the panel "
        "series under examples."
    )
    assert slice_line in lines
    assert text.count("built and in review") == 0

def test_the_gate_lines_name_both_gates():
    readme = (ROOT / "README.md").read_text(encoding="utf-8")
    gate = (
        "- Local gate, python3 scripts/codepoint_scan.py, "
        "python3 -m pytest tests -q, and the cargo gate of "
        "cargo fmt --check, cargo clippy --all-targets -- "
        "-D warnings, and cargo test. CI runs the same commands."
    )
    assert gate in readme.splitlines()
    agents = (ROOT / "AGENTS.md").read_text(encoding="utf-8")
    assert (
        "The local gate is python3 scripts/codepoint_scan.py, "
        "python3 -m pytest tests -q, and the cargo gate of "
        "cargo fmt --check, cargo clippy --all-targets -- "
        "-D warnings, and cargo test"
    ) in agents

def test_ci_runs_the_same_gate_as_the_local_gate():
    text = (ROOT / ".github" / "workflows" / "ci.yml").read_text(encoding="utf-8")
    live = "\n".join(line.split("#", 1)[0] for line in text.splitlines())
    assert "python3 scripts/codepoint_scan.py" in live
    assert "python3 -m pytest tests -q" in live
