//! Pins for the codepoint scan scope stated in SETUP-3 as
//! widened by the amendment in the full form section of
//! CONTRACT.md, translated from the Python corpus.
//!
//! The Python pins run the scan script as a subprocess against
//! a planted tree, because the rule is about which files on
//! disk the scan opens. The port carries the scan in the
//! library, so the same planted trees are read through the
//! library scan here, and the same counts are pinned. These
//! pins are the ones the port report names as standing in the
//! cargo suite, so the scan script's removal at the close of
//! the port checkpoint loses nothing.
//!
//! One pin is an addition under the port lessons. A dash in a
//! Rust source file counts, pinning the widened suffix list
//! that covers the Rust tree.

mod shared;

use deadheat::scan::scan_tree;
use shared::*;

const EM_DASH: &str = "\u{2014}";
const EN_DASH: &str = "\u{2013}";

fn write(root: &std::path::Path, name: &str, text: &str) {
    let path = root.join(name);
    std::fs::create_dir_all(path.parent().expect("a parent")).expect("dirs");
    std::fs::write(&path, text).expect("write");
}

#[test]
fn a_clean_planted_tree_passes() {
    let root = temp_dir("scan-clean");
    write(&root, "notes.md", "plain text\n");
    assert_eq!(scan_tree(&root), (0, 0));
}

#[test]
fn a_dash_in_an_extensionless_file_fails_the_scan() {
    let root = temp_dir("scan-license");
    write(
        &root,
        "LICENSE",
        &format!("licence text {EM_DASH} more text\n"),
    );
    let (em, _) = scan_tree(&root);
    assert_eq!(em, 1);
}

#[test]
fn a_dash_in_a_python_file_fails_the_scan() {
    let root = temp_dir("scan-python");
    write(&root, "module.py", &format!("# a comment {EN_DASH} here\n"));
    let (_, en) = scan_tree(&root);
    assert_eq!(en, 1);
}

#[test]
fn a_dash_in_a_json_file_fails_the_scan() {
    let root = temp_dir("scan-json");
    write(
        &root,
        "data.json",
        &format!("{{\"note\": \"one {EM_DASH} two\"}}\n"),
    );
    let (em, _) = scan_tree(&root);
    assert_eq!(em, 1);
}

#[test]
fn dashes_inside_an_ignored_directory_pass_the_scan() {
    let root = temp_dir("scan-ignored");
    write(
        &root,
        ".venv/planted.md",
        &format!("{EM_DASH} and {EN_DASH}\n"),
    );
    assert_eq!(scan_tree(&root), (0, 0));
}

#[test]
fn a_dash_in_a_rust_file_fails_the_scan() {
    // The addition pinning the widened suffix list.
    let root = temp_dir("scan-rust");
    write(
        &root,
        "module.rs",
        &format!("// a comment {EM_DASH} here\n"),
    );
    let (em, _) = scan_tree(&root);
    assert_eq!(em, 1);
}

#[test]
fn the_scan_exempts_the_envelope_directory_alone() {
    // The exemption pin carried from the Python pilot corpus.
    let root = temp_dir("scan-exemption");
    write(
        &root,
        "examples/pilot/envelopes/e.json",
        &format!("{{\"model\": \"a{EM_DASH}b\"}}\n"),
    );
    assert_eq!(scan_tree(&root), (0, 0));
    write(
        &root,
        "examples/pilot/races/r.json",
        &format!("{{\"task\": \"a{EM_DASH}b\"}}\n"),
    );
    let (em, _) = scan_tree(&root);
    assert_eq!(em, 1);
}

#[test]
fn the_repo_itself_scans_clean() {
    assert_eq!(scan_tree(&repo_root()), (0, 0));
}

#[test]
fn a_dash_inside_the_cargo_target_directory_does_not_count() {
    // The build output directory is not repo content. CI builds
    // into it before the test step runs, so a scan that opened
    // it would read compiled artifacts as if they were files
    // of the repo.
    let root = temp_dir("scan-target");
    write(
        &root,
        "target/debug/build/output",
        &format!("built {EM_DASH} here\n"),
    );
    write(&root, "notes.md", "plain text\n");
    assert_eq!(scan_tree(&root), (0, 0));
}
