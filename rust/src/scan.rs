//! The codepoint scan, counting the em dash and the en dash
//! in the text files of a tree.
//!
//! The scan covers files whose suffix is .md, .txt, .py,
//! .json, .toml, .yml, .yaml, or .rs, and extensionless files
//! such as LICENSE. Files inside ignored environment and build
//! directories are skipped, and the pilot envelope directory
//! stands exempt under the scan exemption in CONTRACT.md,
//! because recorded envelopes stand exactly as their routes
//! returned them. This reading carries the job the scan script
//! under scripts does today, so the cargo suite holds the rule
//! on its own, as the port plan states.

use std::path::Path;

const IGNORED_DIRS: [&str; 8] = [
    ".git",
    ".venv",
    "venv",
    "dist",
    "build",
    "target",
    "node_modules",
    "__pycache__",
];

const SUFFIXES: [&str; 9] = [
    ".md", ".txt", ".py", ".json", ".toml", ".yml", ".yaml", ".rs", "",
];

/// Count the em dashes and the en dashes in the covered files
/// of the tree at the given root, returning the pair of counts
/// in that order.
pub fn scan_tree(root: &Path) -> (usize, usize) {
    let mut counts = (0usize, 0usize);
    let mut stack: Vec<std::path::PathBuf> = vec![root.to_path_buf()];
    while let Some(directory) = stack.pop() {
        let entries = match std::fs::read_dir(&directory) {
            Ok(entries) => entries,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let file_type = match entry.file_type() {
                Ok(file_type) => file_type,
                Err(_) => continue,
            };
            if file_type.is_dir() {
                stack.push(path);
                continue;
            }
            if !file_type.is_file() {
                continue;
            }
            let relative = match path.strip_prefix(root) {
                Ok(relative) => relative.to_path_buf(),
                Err(_) => continue,
            };
            let parts: Vec<String> = relative
                .components()
                .map(|part| part.as_os_str().to_string_lossy().into_owned())
                .collect();
            if parts.len() >= 3
                && parts[0] == "examples"
                && parts[1] == "pilot"
                && parts[2] == "envelopes"
            {
                continue;
            }
            let parents = &parts[..parts.len() - 1];
            if parents
                .iter()
                .any(|part| IGNORED_DIRS.contains(&part.as_str()) || part.ends_with(".egg-info"))
            {
                continue;
            }
            let file_name = parts[parts.len() - 1].clone();
            let suffix = suffix_of(&file_name);
            if !SUFFIXES.contains(&suffix.as_str()) {
                continue;
            }
            let bytes = match std::fs::read(&path) {
                Ok(bytes) => bytes,
                Err(_) => continue,
            };
            let text = String::from_utf8_lossy(&bytes);
            counts.0 += text.matches('\u{2014}').count();
            counts.1 += text.matches('\u{2013}').count();
        }
    }
    counts
}

fn suffix_of(file_name: &str) -> String {
    match file_name.rfind('.') {
        None => String::new(),
        Some(0) => String::new(),
        Some(index) => file_name[index..].to_string(),
    }
}
