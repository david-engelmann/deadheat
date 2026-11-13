//! The deadheat library, the Rust port of the Python package.
//!
//! The landed Python implementation is the behavioral spec of
//! this port, as PORT-1 in CONTRACT.md states. The loaders, the
//! readings, and the renderings are pure library functions over
//! recorded files, and the command line is a thin shell over
//! them.

pub mod championship;
pub mod cli;
pub mod closings;
pub mod clusters;
pub mod comebacks;
pub mod crossings;
pub mod deadlocks;
pub mod distances;
pub mod flips;
pub mod json;
pub mod leads;
pub mod makers;
pub mod openings;
pub mod pairings;
pub mod panels;
pub mod pynum;
pub mod pytext;
pub mod pyunicode;
pub mod races;
pub mod rallies;
pub mod runner;
pub mod scan;
pub mod seats;
pub mod series;
pub mod specs;
pub mod streaks;
pub mod tenure;
pub mod totals;
pub mod turns;
