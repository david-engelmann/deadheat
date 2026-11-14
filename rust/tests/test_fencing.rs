//! Pins for FIX-DH-16 in the Round 9 section of CONTRACT.md.
//!
//! Every deterministic text rendering is handed a record
//! built by hand, with no loader standing in front of it,
//! carrying a line break in a printed field. Each rendering
//! must print that field as the empty field through the one
//! helper in the text module, so no line is minted and no
//! forged text stands at a line start. On the pre closure
//! rendering every one of these values prints its field raw,
//! so every pin here fails there. The attempt patch stands
//! outside the rule and the seat summary pin holds it
//! verbatim.

mod shared;

use num_bigint::BigInt;

use deadheat::championship::{championship_report, Championship};
use deadheat::closings::closing_report;
use deadheat::clusters::cluster_report;
use deadheat::comebacks::comeback_report;
use deadheat::crossings::crossings_report;
use deadheat::deadlocks::deadlock_report;
use deadheat::depths::depths_report;
use deadheat::distances::distance_report;
use deadheat::flips::flip_report;
use deadheat::leads::lead_report;
use deadheat::openings::opening_report;
use deadheat::pairings::pairing_report;
use deadheat::pynum::Num;
use deadheat::pytext::PText;
use deadheat::races::{
    close_call_report, funeral, race_report, standings_table, Adjudication, Attempt, Race,
    StandingEntry, Standings, SuiteResult,
};
use deadheat::rallies::rally_report;
use deadheat::seats::attempt_summary;
use deadheat::series::{series_report, Series};
use deadheat::spells::spells_report;
use deadheat::streaks::streak_report;
use deadheat::tenure::tenure_report;
use deadheat::totals::totals_report;
use deadheat::turns::turns_report;

const FORGED: &str = "FORGED";

fn text(value: &str) -> PText {
    PText::from_text(value)
}

fn bad(value: &str) -> PText {
    PText::from_text(&format!("{value}\n{FORGED}"))
}

fn attempt(model: PText, environment: &str, result: SuiteResult, finish: f64) -> Attempt {
    Attempt {
        model,
        environment: text(environment),
        result,
        tokens: Some(BigInt::from(10)),
        patch: text("a diff"),
        finished_at: Some(Num::from_f64(finish)),
        adjustment: None,
    }
}

/// A race whose task, first model, and first environment
/// all carry a line break. The first attempt wins on a
/// green against red reading outside the close call band.
fn bad_race() -> Race {
    Race {
        task: bad("task"),
        attempts: vec![
            attempt(bad("model"), "lane\nFORGED", SuiteResult::Green, 100.0),
            attempt(text("beta-1"), "lane two", SuiteResult::Red, 130.0),
        ],
        adjudication: None,
        panel: None,
    }
}

fn bad_series() -> Series {
    Series {
        task: bad("series task"),
        models: vec![bad("model"), text("beta-1")],
        wins_target: BigInt::from(1),
        races: vec![bad_race()],
    }
}

fn assert_no_forgery(lines: &[String]) {
    for line in lines {
        assert!(!line.contains(FORGED), "{}", line);
    }
}

#[test]
fn the_race_report_fences_its_fields() {
    let lines = race_report(&bad_race());
    assert_no_forgery(&lines);
    assert_eq!(
        lines,
        vec![
            "race ",
            " in , green in 100 seconds, 10 tokens",
            "beta-1 in lane two, red in 130 seconds, 10 tokens",
            "verdict,  wins",
        ]
    );
}

#[test]
fn the_funeral_fences_its_fields() {
    let lines = funeral(&bad_race());
    assert_no_forgery(&lines);
    assert_eq!(
        lines,
        vec![
            "funeral for ",
            "winner  in  went green in 100 seconds",
            "loser beta-1 in lane two went red in 130 seconds",
            "margin 30 seconds between the two finishes",
        ]
    );
}

#[test]
fn the_closecall_reading_fences_its_fields() {
    let lines = close_call_report(&bad_race());
    assert_no_forgery(&lines);
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0], "closecall ");
    assert_eq!(lines[2], "verdict,  wins");
}

#[test]
fn the_adjudication_and_panel_labels_are_fenced_in_the_race_readings() {
    let mut decided = bad_race();
    decided.attempts[0].environment = text("lane one");
    decided.attempts[1].result = SuiteResult::Green;
    decided.attempts[1].finished_at = Some(Num::from_f64(104.0));
    decided.adjudication = Some(Adjudication {
        adjudicator: bad("judge"),
        chosen: text("beta-1"),
        confidence: Num::from_f64(0.9),
        reason: text("the steadier run"),
    });
    // The adjudication decides for beta, the second attempt,
    // whose model stands clean, so the only forged field on
    // the decided lines is the adjudicator itself.
    decided.attempts[0].model = text("alpha-1");
    let lines = race_report(&decided);
    assert_no_forgery(&lines);
    assert_eq!(
        lines.last().expect("a decided line"),
        "close call, adjudication by  chose beta-1 at confidence 0.9"
    );
    let lines = close_call_report(&decided);
    assert_no_forgery(&lines);
    assert!(lines.iter().any(|line| line.as_str()
        == "adjudication by  chose beta-1 at confidence 0.9, the race is decided for beta-1"));

    let mut panelled = decided.clone();
    panelled.adjudication = None;
    panelled.panel = Some(vec![
        Adjudication {
            adjudicator: bad("judge"),
            chosen: bad("model"),
            confidence: Num::from_f64(0.9),
            reason: text("the steadier run"),
        },
        Adjudication {
            adjudicator: text("judge-b"),
            chosen: bad("model"),
            confidence: Num::from_f64(0.95),
            reason: text("the cleaner patch"),
        },
    ]);
    // The panel chooses the first attempt, whose model
    // carries the break, so the chosen field and the
    // confirmed winner both stand fenced.
    panelled.attempts[0].model = bad("model");
    let lines = race_report(&panelled);
    assert_no_forgery(&lines);
    assert_eq!(
        lines.last().expect("a panel line"),
        "close call, panel, adjudication by  chose  at confidence 0.9 and adjudication by judge-b chose  at confidence 0.95, the panel confirms "
    );
}

#[test]
fn the_series_report_fences_its_fields() {
    let lines = series_report(&bad_series());
    assert_no_forgery(&lines);
    assert_eq!(
        lines,
        vec![
            "series ",
            " against beta-1, first to 1 wins",
            "race 1, ,  wins",
            "tally,  1 wins, beta-1 0 wins, 0 no results",
            "series winner, ",
        ]
    );
}

#[test]
fn the_championship_report_fences_its_fields() {
    let championship = Championship {
        series: vec![bad_series()],
    };
    let lines = championship_report(&championship);
    assert_no_forgery(&lines);
    assert_eq!(
        lines,
        vec![
            "championship",
            "series 1, ",
            " against beta-1, first to 1 wins",
            "series winner, ",
            "championship table",
            ", 1 series won, 1 series stood in, 1 race wins, 0 no results, aggregate margin 30 seconds",
            "beta-1, 0 series won, 1 series stood in, 0 race wins, 0 no results, aggregate margin 0 seconds",
        ]
    );
}

#[test]
fn the_flip_report_fences_its_fields() {
    let lines = flip_report(&bad_series());
    assert_no_forgery(&lines);
    assert_eq!(
        lines,
        vec![
            "series ",
            " against beta-1, first to 1 wins",
            "race 1, , scratch , adjusted , held",
            "flips, 1 races, 0 flipped, 1 held, 0 no results",
        ]
    );
}

#[test]
fn the_totals_report_fences_its_fields() {
    let championship = Championship {
        series: vec![bad_series()],
    };
    let lines = totals_report(&championship);
    assert_no_forgery(&lines);
    assert_eq!(
        lines,
        vec![
            "totals",
            ", 1 races, 1 wins, 0 losses, 0 no results, 10 tokens",
            "beta-1, 1 races, 0 wins, 1 losses, 0 no results, 10 tokens",
            "totals, 1 series, 1 races, 2 models",
        ]
    );
}

#[test]
fn the_standings_table_fences_its_labels() {
    let standings = Standings {
        models: vec![(
            bad("model"),
            StandingEntry {
                wins: BigInt::from(1),
                losses: BigInt::from(2),
                no_results: BigInt::from(3),
                tokens: BigInt::from(4),
                tokens_unreported: BigInt::from(5),
            },
        )],
    };
    let lines = standings_table(&standings);
    assert_no_forgery(&lines);
    assert_eq!(
        lines,
        vec![
            "season standings",
            ", 1 wins, 2 losses, 3 no results, 4 tokens, 5 unreported",
        ]
    );
}

#[test]
fn the_seat_summary_fences_its_labels_and_holds_the_patch_verbatim() {
    let mut race = bad_race();
    race.attempts[0].patch = text("first patch line\nsecond patch line");
    let summary = attempt_summary(&race);
    let lines: Vec<&str> = summary.lines().collect();
    assert_eq!(lines[0], "task ");
    assert!(!summary.contains(FORGED), "{}", summary);
    assert!(summary.contains(
        "attempt, model , environment , result green, finished in 100 seconds, 10 tokens"
    ));
    assert!(summary.contains("patch of \nfirst patch line\nsecond patch line"));
}

#[test]
fn the_pairing_report_fences_its_model_names() {
    let mut series = bad_series();
    // The second model also carries a break, a carriage
    // return where the first model carries a newline, so
    // both break kinds stand fenced on the pairing
    // surface. The race reads as it did, the first attempt
    // green against red, so the first model takes the win
    // and the margin and the second model takes nothing.
    series.models[1] = text("beta\rFORGED");
    series.races[0].attempts[1].model = text("beta\rFORGED");
    let championship = Championship {
        series: vec![series],
    };
    let lines = pairing_report(&championship);
    assert_no_forgery(&lines);
    assert_eq!(
        lines,
        vec![
            "pairings",
            "pairing, , against, ",
            "series read, 1",
            "races read, 1",
            ", 1 series won, 1 race wins, 0 no results, aggregate margin 30 seconds",
            ", 0 series won, 0 race wins, 0 no results, aggregate margin 0 seconds",
            "pairings, 1 pairings, 1 series, 1 races",
        ]
    );
}

#[test]
fn the_comeback_report_fences_its_fields() {
    let mut series = bad_series();
    // The series is rebuilt to a target of two over three
    // races. The second model takes the first race and the
    // first model takes the next two, so the series stands
    // won from behind and its section prints the series
    // task, the race tasks, and both model labels, and
    // every field holding a break stands fenced to the
    // empty field on this surface as on the others.
    series.wins_target = BigInt::from(2);
    series.races = vec![
        Race {
            task: text("race one"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Red, 130.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Green, 100.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: bad("race task"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: text("race three"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
    ];
    let championship = Championship {
        series: vec![series],
    };
    let lines = comeback_report(&championship);
    assert_no_forgery(&lines);
    assert_eq!(
        lines,
        vec![
            "comebacks",
            "series 1, ",
            " against beta-1, first to 2 wins",
            "race 1, race one, beta-1 wins",
            "race 2, ,  wins",
            "race 3, race three,  wins",
            "first decided race, race 1,  lost to beta-1",
            "series winner, ,  2 race wins, beta-1 1 race wins, 0 no results",
            "comebacks, 1 series, 1 from behind, 0 from the front",
        ]
    );
}

#[test]
fn the_distance_report_fences_its_fields() {
    let mut series = bad_series();
    // The series is rebuilt to a target of two over three
    // races. The second model takes the first race and the
    // first model takes the next two, so the series stands
    // the distance and its section prints the series task,
    // the race tasks, and both model labels, and every
    // field holding a break stands fenced to the empty
    // field on this surface as on the others.
    series.wins_target = BigInt::from(2);
    series.races = vec![
        Race {
            task: text("race one"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Red, 130.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Green, 100.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: bad("race task"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: text("race three"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
    ];
    let championship = Championship {
        series: vec![series],
    };
    let lines = distance_report(&championship);
    assert_no_forgery(&lines);
    assert_eq!(
        lines,
        vec![
            "distance",
            "series 1, ",
            " against beta-1, first to 2 wins",
            "race 1, race one, beta-1 wins",
            "race 2, ,  wins",
            "race 3, race three,  wins",
            "series winner, ,  2 race wins, beta-1 1 race wins, 0 no results",
            "distance, 1 series, 1 the distance, 0 without reply, 0 in between, 0 no winner",
        ]
    );
}

#[test]
fn the_rally_report_fences_its_fields() {
    let mut series = bad_series();
    // The series is rebuilt to a target of two over three
    // races, the same shape the comeback and distance pins
    // stand on. The second model takes the first race and
    // the first model takes the next two, so the series
    // stands a rally and its section prints the series
    // task, the race tasks, and both model labels, and
    // every field holding a break stands fenced to the
    // empty field on this surface as on the others.
    series.wins_target = BigInt::from(2);
    series.races = vec![
        Race {
            task: text("race one"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Red, 130.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Green, 100.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: bad("race task"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: text("race three"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
    ];
    let championship = Championship {
        series: vec![series],
    };
    let lines = rally_report(&championship);
    assert_no_forgery(&lines);
    assert_eq!(
        lines,
        vec![
            "rally",
            "series 1, ",
            " against beta-1, first to 2 wins",
            "race 1, race one, beta-1 wins",
            "race 2, ,  wins",
            "race 3, race three,  wins",
            "stood behind by 1 race wins at the deepest",
            "series winner, ,  2 race wins, beta-1 1 race wins, 0 no results",
            "rally, 1 series, 1 rally, 0 wire to wire, 0 no winner",
        ]
    );
}

#[test]
fn the_deadlock_report_fences_its_fields() {
    // F-1 of the Round 13 discrimination plan. The same
    // rebuilt series the rally pin stands on, read under
    // DH-30. The second model takes the first race and the
    // first model takes the next two, so the tallies stand
    // level after the second decided race and the series
    // stands deadlocked, and its section prints the series
    // task, the race tasks, and both model labels, every
    // field holding a break standing fenced to the empty
    // field through the one helper of FIX-DH-16, as on the
    // sibling surfaces. Its failure mode is a second
    // fencing path standing in the new module.
    let mut series = bad_series();
    series.wins_target = BigInt::from(2);
    series.races = vec![
        Race {
            task: text("race one"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Red, 130.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Green, 100.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: bad("race task"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: text("race three"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
    ];
    let championship = Championship {
        series: vec![series],
    };
    let lines = deadlock_report(&championship);
    assert_no_forgery(&lines);
    assert_eq!(
        lines,
        vec![
            "deadlock",
            "series 1, ",
            " against beta-1, first to 2 wins",
            "race 1, race one, beta-1 wins",
            "race 2, ,  wins",
            "race 3, race three,  wins",
            "stood level after 1 of its decided races",
            "series winner, ,  2 race wins, beta-1 1 race wins, 0 no results",
            "deadlock, 1 series, 1 deadlocked, 0 clear, 0 no winner",
        ]
    );
}

#[test]
fn the_streak_report_fences_its_fields() {
    // F-1 of the Round 14 discrimination plan. The same
    // rebuilt series the deadlock pin stands on, read
    // under DH-33. The second model takes the first race
    // and the first model takes the next two, so the
    // first model's wins stand in a run of two closing
    // the series and the series stands strung, and its
    // section prints the series task, the race tasks, and
    // both model labels, every field holding a break
    // standing fenced to the empty field through the one
    // helper of FIX-DH-16, as on the sibling surfaces.
    // Its failure mode is a second fencing path standing
    // in the new module.
    let mut series = bad_series();
    series.wins_target = BigInt::from(2);
    series.races = vec![
        Race {
            task: text("race one"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Red, 130.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Green, 100.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: bad("race task"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: text("race three"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
    ];
    let championship = Championship {
        series: vec![series],
    };
    let lines = streak_report(&championship);
    assert_no_forgery(&lines);
    assert_eq!(
        lines,
        vec![
            "streak",
            "series 1, ",
            " against beta-1, first to 2 wins",
            "race 1, race one, beta-1 wins",
            "race 2, ,  wins",
            "race 3, race three,  wins",
            "longest run of 2 consecutive decided race wins",
            "series winner, ,  2 race wins, beta-1 1 race wins, 0 no results",
            "streak, 1 series, 1 strung, 0 single, 0 no winner",
        ]
    );
}

#[test]
fn the_cluster_report_fences_its_fields() {
    // F-1 of the Round 15 discrimination plan. A series
    // rebuilt by hand on the bad series frame, read under
    // DH-35. The second model takes the first two races
    // and the first model takes the next three, so the
    // second model's wins stand in a run of two and the
    // series stands clustered, its loser standing as the
    // side the figure reads, and its section prints the
    // series task, the race tasks, and both model labels,
    // every field holding a break standing fenced to the
    // empty field through the one helper of FIX-DH-16, as
    // on the sibling surfaces. Its failure mode is a
    // second fencing path standing in the new module.
    let mut series = bad_series();
    series.wins_target = BigInt::from(3);
    series.races = vec![
        Race {
            task: text("race one"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Red, 130.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Green, 100.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: bad("race task"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Red, 130.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Green, 100.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: text("race three"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: text("race four"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: text("race five"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
    ];
    let championship = Championship {
        series: vec![series],
    };
    let lines = cluster_report(&championship);
    assert_no_forgery(&lines);
    assert_eq!(
        lines,
        vec![
            "cluster",
            "series 1, ",
            " against beta-1, first to 3 wins",
            "race 1, race one, beta-1 wins",
            "race 2, , beta-1 wins",
            "race 3, race three,  wins",
            "race 4, race four,  wins",
            "race 5, race five,  wins",
            "longest run against 2 consecutive decided race wins",
            "series winner, ,  3 race wins, beta-1 2 race wins, 0 no results",
            "cluster, 1 series, 1 clustered, 0 scattered, 0 no winner",
        ]
    );
}

#[test]
fn the_closing_report_fences_its_fields() {
    // F-1 of the Round 16 discrimination plan. The same
    // rebuilt series the streak pin stands on, read under
    // DH-37. The second model takes the first race and the
    // first model takes the next two, so the first model's
    // wins stand in a closing run of two and the series
    // stands in a run, and its section prints the series
    // task, the race tasks, and both model labels, every
    // field holding a break standing fenced to the empty
    // field through the one helper of FIX-DH-16, as on the
    // sibling surfaces. Its failure mode is a second
    // fencing path standing in the new module.
    let mut series = bad_series();
    series.wins_target = BigInt::from(2);
    series.races = vec![
        Race {
            task: text("race one"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Red, 130.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Green, 100.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: bad("race task"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: text("race three"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
    ];
    let championship = Championship {
        series: vec![series],
    };
    let lines = closing_report(&championship);
    assert_no_forgery(&lines);
    assert_eq!(
        lines,
        vec![
            "closing",
            "series 1, ",
            " against beta-1, first to 2 wins",
            "race 1, race one, beta-1 wins",
            "race 2, ,  wins",
            "race 3, race three,  wins",
            "closing run of 2 consecutive decided race wins",
            "series winner, ,  2 race wins, beta-1 1 race wins, 0 no results",
            "closing, 1 series, 1 in a run, 0 alone, 0 no winner",
        ]
    );
}

#[test]
fn the_opening_report_fences_its_fields() {
    // F-1 of the Round 17 discrimination plan. The same
    // rebuilt series the closing pin stands on, read
    // under DH-39, its races standing in the order the
    // opening reading classes: the first model takes the
    // first two decided races, so its wins stand in an
    // opening run of two and the series stands in a run,
    // and its section prints the series task, the race
    // tasks, and both model labels, every field holding a
    // break standing fenced to the empty field through
    // the one helper of FIX-DH-16, as on the sibling
    // surfaces. Its failure mode is a second fencing path
    // standing in the new module. It fails on the pre
    // round tree by compile absence, the opening entry
    // point not standing there.
    let mut series = bad_series();
    series.wins_target = BigInt::from(2);
    series.races = vec![
        Race {
            task: text("race one"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: bad("race task"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: text("race three"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Red, 130.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Green, 100.0),
            ],
            adjudication: None,
            panel: None,
        },
    ];
    let championship = Championship {
        series: vec![series],
    };
    let lines = opening_report(&championship);
    assert_no_forgery(&lines);
    assert_eq!(
        lines,
        vec![
            "opening",
            "series 1, ",
            " against beta-1, first to 2 wins",
            "race 1, race one,  wins",
            "race 2, ,  wins",
            "opening run of 2 consecutive decided race wins",
            "series winner, ,  2 race wins, beta-1 0 race wins, 0 no results",
            "opening, 1 series, 1 in a run, 0 alone, 0 from behind, 0 no winner",
        ]
    );
}

#[test]
fn the_turns_report_fences_its_fields() {
    // F-1 of the Round 19 discrimination plan. A
    // four-race series rebuilt from the same hand-built
    // shape the sibling pins stand on, read under
    // DH-41: the first model takes the first race, the
    // second model the second, and the first model the
    // third and fourth at a wins target of three, so
    // its record changes hands twice and the series
    // stands turned, and its section prints the series
    // task, the race tasks, and both model labels,
    // every field holding a break standing fenced to
    // the empty field through the one helper of
    // FIX-DH-16, as on the sibling surfaces. Its
    // failure mode is a second fencing path standing
    // in the new module. It fails on the pre round
    // tree by compile absence, the turns module
    // standing absent there.
    let mut series = bad_series();
    series.wins_target = BigInt::from(3);
    series.races = vec![
        Race {
            task: text("race one"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: bad("race task"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Red, 130.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Green, 100.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: text("race three"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: text("race four"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
    ];
    let championship = Championship {
        series: vec![series],
    };
    let lines = turns_report(&championship);
    assert_no_forgery(&lines);
    assert_eq!(
        lines,
        vec![
            "turns",
            "series 1, ",
            " against beta-1, first to 3 wins",
            "race 1, race one,  wins",
            "race 2, , beta-1 wins",
            "race 3, race three,  wins",
            "race 4, race four,  wins",
            "changes of hand 2 over the decided record",
            "series winner, ,  3 race wins, beta-1 1 race wins, 0 no results",
            "turns, 1 series, 1 turned, 0 steady, 0 no winner",
        ]
    );
}

#[test]
fn the_lead_report_fences_its_fields() {
    // F-1 of the Round 20 discrimination plan. A
    // five-race series rebuilt from the same hand-built
    // shape the sibling pins stand on, read under
    // DH-43: the first model takes the first two races,
    // the second model the next two, and the first
    // model the fifth at a wins target of three, so the
    // first model's largest lead stands at two against
    // its winning margin of one and the series stands
    // wide, and its section prints the series task, the
    // race tasks, and both model labels, every field
    // holding a break standing fenced to the empty
    // field through the one helper of FIX-DH-16, as on
    // the sibling surfaces. Its failure mode is a
    // second fencing path standing in the new module.
    // It fails on the pre round tree by compile
    // absence, the leads module standing absent there.
    let mut series = bad_series();
    series.wins_target = BigInt::from(3);
    series.races = vec![
        Race {
            task: text("race one"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: bad("race task"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: text("race three"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Red, 130.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Green, 100.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: text("race four"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Red, 130.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Green, 100.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: text("race five"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(text("beta-1"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
    ];
    let championship = Championship {
        series: vec![series],
    };
    let lines = lead_report(&championship);
    assert_no_forgery(&lines);
    assert_eq!(
        lines,
        vec![
            "lead",
            "series 1, ",
            " against beta-1, first to 3 wins",
            "race 1, race one,  wins",
            "race 2, ,  wins",
            "race 3, race three, beta-1 wins",
            "race 4, race four, beta-1 wins",
            "race 5, race five,  wins",
            "largest lead 2 over the decided record",
            "series winner, ,  3 race wins, beta-1 2 race wins, 0 no results",
            "lead, 1 series, 1 wide, 0 narrow, 0 no winner",
        ]
    );
}

#[test]
fn the_crossings_report_fences_its_fields() {
    // F-1 of the Round 21 discrimination plan. The
    // hand-built series the sibling pins stand on, at
    // its wins target of one with its one race won by
    // its first model, read under DH-45: its crossings
    // stand at none and its changes of hand at none,
    // so the series stands through and its section
    // prints the series task, the race task, and both
    // model labels, every field holding a break
    // standing fenced to the empty field through the
    // one helper of FIX-DH-16, as on the sibling
    // surfaces. Its failure mode is a second fencing
    // path standing in the new module. It fails on
    // the pre round tree by compile absence, the
    // crossings module standing absent there.
    let championship = Championship {
        series: vec![bad_series()],
    };
    let lines = crossings_report(&championship);
    assert_no_forgery(&lines);
    assert_eq!(
        lines,
        vec![
            "crossings",
            "series 1, ",
            " against beta-1, first to 1 wins",
            "race 1, ,  wins",
            "crossings 0 over the decided record",
            "series winner, ,  1 race wins, beta-1 0 race wins, 0 no results",
            "crossings, 1 series, 1 through, 0 short, 0 no winner",
        ]
    );
}

#[test]
fn the_tenure_report_fences_its_fields() {
    // F-1 of the Round 22 discrimination plan. The
    // hand-built series the sibling pins stand on, at
    // its wins target of one with its one race won by
    // its first model, read under DH-47: its winner's
    // front count stands at one against its loser's
    // none, so the series stands ahead and its section
    // prints the series task, the race task, and both
    // model labels, every field holding a break
    // standing fenced to the empty field through the
    // one helper of FIX-DH-16, as on the sibling
    // surfaces. Its failure mode is a second fencing
    // path standing in the new module. It fails on
    // the pre round tree by compile absence, the
    // tenure module standing absent there.
    let championship = Championship {
        series: vec![bad_series()],
    };
    let lines = tenure_report(&championship);
    assert_no_forgery(&lines);
    assert_eq!(
        lines,
        vec![
            "tenure",
            "series 1, ",
            " against beta-1, first to 1 wins",
            "race 1, ,  wins",
            "front count 1 over the decided record",
            "series winner, ,  1 race wins, beta-1 0 race wins, 0 no results",
            "tenure, 1 series, 1 ahead, 0 level, 0 behind, 0 no winner",
        ]
    );
}

#[test]
fn the_depths_report_fences_its_fields() {
    // F-1 of the Round 23 discrimination plan. A
    // series rebuilt by hand on the bad series frame,
    // read under DH-49, its second model standing
    // break bearing as its first does. The second
    // model takes the first two races and the first
    // model takes the next three at a wins target of
    // three, so the first model stood at a deepest
    // deficit of two against its winning margin of
    // one and the series stands deep, and its section
    // prints the series task, the race tasks, and
    // both model labels, every field holding a break
    // standing fenced to the empty field through the
    // one helper of FIX-DH-16, as on the sibling
    // surfaces. Its failure mode is a second fencing
    // path standing in the new module. It fails on
    // the pre round tree by compile absence, the
    // depths module standing absent there.
    let mut series = bad_series();
    series.models[1] = bad("beta");
    series.wins_target = BigInt::from(3);
    series.races = vec![
        Race {
            task: text("race one"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Red, 130.0),
                attempt(bad("beta"), "lane two", SuiteResult::Green, 100.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: bad("race task"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Red, 130.0),
                attempt(bad("beta"), "lane two", SuiteResult::Green, 100.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: text("race three"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(bad("beta"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: text("race four"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(bad("beta"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: text("race five"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(bad("beta"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
    ];
    let championship = Championship {
        series: vec![series],
    };
    let lines = depths_report(&championship);
    assert_no_forgery(&lines);
    assert_eq!(
        lines,
        vec![
            "depths",
            "series 1, ",
            " against , first to 3 wins",
            "race 1, race one,  wins",
            "race 2, ,  wins",
            "race 3, race three,  wins",
            "race 4, race four,  wins",
            "race 5, race five,  wins",
            "deepest deficit 2 over the decided record",
            "series winner, ,  3 race wins,  2 race wins, 0 no results",
            "depths, 1 series, 1 deep, 0 matched, 0 shallow, 0 no winner",
        ]
    );
}

#[test]
fn the_spells_report_fences_its_fields() {
    // F-1 of the Round 25 discrimination plan. A
    // series rebuilt by hand on the bad series frame,
    // read under DH-51, its second model standing
    // break bearing as its first does. The second
    // model takes the first and third races and the
    // first model takes the second, fourth, and fifth
    // at a wins target of three, so the first model
    // stood behind in two runs of one against a behind
    // count of two and the series stands broken, and
    // its section prints the series task, the race
    // tasks, and both model labels, every field
    // holding a break standing fenced to the empty
    // field through the one helper of FIX-DH-16, as on
    // the sibling surfaces. Its failure mode is a
    // second fencing path standing in the new module.
    // It fails on the pre round tree by compile
    // absence, the spells module standing absent
    // there.
    let mut series = bad_series();
    series.models[1] = bad("beta");
    series.wins_target = BigInt::from(3);
    series.races = vec![
        Race {
            task: text("race one"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Red, 130.0),
                attempt(bad("beta"), "lane two", SuiteResult::Green, 100.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: bad("race task"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(bad("beta"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: text("race three"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Red, 130.0),
                attempt(bad("beta"), "lane two", SuiteResult::Green, 100.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: text("race four"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(bad("beta"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
        Race {
            task: text("race five"),
            attempts: vec![
                attempt(bad("model"), "lane one", SuiteResult::Green, 100.0),
                attempt(bad("beta"), "lane two", SuiteResult::Red, 130.0),
            ],
            adjudication: None,
            panel: None,
        },
    ];
    let championship = Championship {
        series: vec![series],
    };
    let lines = spells_report(&championship);
    assert_no_forgery(&lines);
    assert_eq!(
        lines,
        vec![
            "spells",
            "series 1, ",
            " against , first to 3 wins",
            "race 1, race one,  wins",
            "race 2, ,  wins",
            "race 3, race three,  wins",
            "race 4, race four,  wins",
            "race 5, race five,  wins",
            "longest spell 1, behind count 2 over the decided record",
            "series winner, ,  3 race wins,  2 race wins, 0 no results",
            "spells, 1 series, 0 unbroken, 1 broken, 0 never behind, 0 no winner",
        ]
    );
}
