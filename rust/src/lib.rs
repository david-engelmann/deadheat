//! Deadheat races two attempts at one task in isolated workspaces, scores
//! finishes by passing verification checks, elapsed seconds, and optional
//! handicaps, adjudicates close calls from recorded model verdicts, adopts
//! or discards loser patches, and renders thirty five analytical readings
//! across races, series, and championships.
//!
//! The loaders, readings, and renderers stand as pure functions over
//! version 1 JSON records, while `runner` executes two local command
//! attempts side by side from a race specification. The command line
//! binary in `cli` is a thin shell over these modules.

pub mod championship;
pub mod cli;
pub mod closings;
pub mod clusters;
pub mod comebacks;
pub mod crosshead;
pub mod crossings;
pub mod deadlocks;
pub mod depths;
pub mod distances;
pub mod firstreign;
pub mod firststand;
pub mod firststrike;
pub mod flips;
pub mod json;
pub mod lastreign;
pub mod laststand;
pub mod leadchanges;
pub mod leads;
pub mod makers;
pub mod openings;
pub mod outset;
pub mod pairings;
pub mod panels;
pub mod peak;
pub mod place;
pub mod pynum;
pub mod pytext;
pub mod pyunicode;
pub mod races;
pub mod rallies;
pub mod recovery;
pub mod reigns;
pub mod runner;
pub mod scan;
pub mod seats;
pub mod series;
pub mod specs;
pub mod spells;
pub mod streaks;
pub mod tail;
pub mod tenure;
pub mod totals;
pub mod turns;
