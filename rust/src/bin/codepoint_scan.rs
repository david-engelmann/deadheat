//! The codepoint scan command line.
//!
//! A thin shell over the library scan. It takes the tree to scan
//! as its one optional argument and scans the working directory
//! when no argument stands, prints the line the Python scan
//! script printed, and exits with status 0 when the tree is
//! clean and status 1 when a dash stands anywhere in it, as the
//! script did.

use std::path::PathBuf;

fn main() {
    let root = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let (em, en) = deadheat::scan::scan_tree(&root);
    println!("codepoint scan U+2014={em} U+2013={en}");
    std::process::exit(if em == 0 && en == 0 { 0 } else { 1 });
}
