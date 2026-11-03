//! The panel join over recorded adjudication records.
//!
//! Two records of one race, each carrying at most its single
//! DH-5 adjudication entry, join into one panel record holding
//! the contributed entries under the field panel, in the order
//! the records were given. The join is a pure function of the
//! two loaded records and never mutates its inputs. A pair that
//! fails the join returns no record. The obligation this module
//! carries is DH-10 in CONTRACT.md, in the Round 4 section, and
//! the panel shape and reading it assembles are DH-9. The
//! Python module of the same name is the behavioral spec.

use crate::races::{Adjudication, Race};

/// Join two loaded records of one race into a panel record.
///
/// Returns a pair of the panel record and the defect list. The
/// two records must name the same task and hold the same
/// attempts, as loaded. Each record contributes its adjudication
/// entry when it carries one and contributes absence when it
/// carries none, and a pair holding no entry between them is
/// refused. A record carrying a panel is refused, because its
/// entries already stand joined and the join assembles a panel
/// from single entries alone, and where a panel stands in the
/// pair the pair draws no defect for holding no entry, since the
/// refused panel is where the entries stand. The refusal is
/// FIX-DH-14 in CONTRACT.md, amending DH-10. Two entries naming
/// the same adjudicator are refused, because a panel stands one
/// entry per adjudicating model. The panel record holds the task
/// and the attempts of the two records and the contributed
/// entries under the field panel, and it loads under DH-9 with
/// no defects.
pub fn join_panel_records(first: &Race, second: &Race) -> (Option<Race>, Vec<String>) {
    let mut defects = Vec::new();
    if first.task != second.task {
        defects.push("the two records name different tasks".to_string());
    }
    if first.attempts != second.attempts {
        defects.push("the two records hold different attempts".to_string());
    }
    let panel_seen = first.panel.is_some() || second.panel.is_some();
    if panel_seen {
        defects.push("one of the two records carries a panel".to_string());
    }
    let mut entries: Vec<Adjudication> = Vec::new();
    for record in [first, second] {
        if let Some(entry) = &record.adjudication {
            entries.push(entry.clone());
        }
    }
    if entries.is_empty() && !panel_seen {
        defects.push("the two records hold no adjudication entry between them".to_string());
    } else if entries.len() == 2 && entries[0].adjudicator == entries[1].adjudicator {
        defects.push("the two entries name the same adjudicator".to_string());
    }
    if !defects.is_empty() {
        return (None, defects);
    }
    (
        Some(Race {
            task: first.task.clone(),
            attempts: first.attempts.clone(),
            adjudication: None,
            panel: Some(entries),
        }),
        Vec::new(),
    )
}
