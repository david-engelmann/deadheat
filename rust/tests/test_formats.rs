//! The version rule of FIX-DH-17, pinned on every surface that
//! reads a format object this repo owns.
//!
//! A format object may carry the field version. Absent or the
//! integer 1 reads as version 1. An integer other than 1 draws
//! the unsupported version defect of the surface that reads it,
//! and any other form draws that surface's wrong type defect.
//! The refusal pins in this file fail against a loader that
//! leaves the version field unread, because the defects they
//! assert never stand on such a loader's output. The clean
//! load pins pass on such a loader by design, since a version
//! 1 object is exactly what an unread field leaves behind.

mod shared;

use deadheat::championship::load_championship_text;
use deadheat::json::{self, JsonValue};
use deadheat::races::{load_race_data, load_standings_text};
use deadheat::series::load_series_text;
use shared::{arr, f, i, obj, s, set, temp_dir};

fn attempt(model: &str, result: &str) -> JsonValue {
    let mut pairs = vec![
        ("model", s(model)),
        ("environment", s("local")),
        ("result", s(result)),
        ("tokens", i(10)),
        ("patch", s("")),
    ];
    if result != "unfinished" {
        pairs.push(("finished_at", f(12.5)));
    }
    obj(pairs)
}

fn race() -> JsonValue {
    obj(vec![
        ("task", s("the recorded race")),
        (
            "attempts",
            arr(vec![
                attempt("prime-1", "green"),
                attempt("shadow-1", "red"),
            ]),
        ),
    ])
}

fn series() -> JsonValue {
    obj(vec![
        ("task", s("the recorded series")),
        ("models", arr(vec![s("prime-1"), s("shadow-1")])),
        ("wins_target", i(2)),
        ("races", arr(vec![race()])),
    ])
}

fn adjudication_entry() -> JsonValue {
    obj(vec![
        ("adjudicator", s("judge-1")),
        ("chosen", s("prime-1")),
        ("reason", s("the greener patch")),
        ("confidence", f(0.9)),
    ])
}

fn text(value: &JsonValue) -> String {
    json::dumps(value, None, true)
}

#[test]
fn a_race_with_no_version_field_loads_clean() {
    let (loaded, defects) = load_race_data(&race());
    assert!(loaded.is_some());
    assert_eq!(defects, Vec::<String>::new());
}

#[test]
fn a_race_at_version_1_loads_clean() {
    let (loaded, defects) = load_race_data(&set(&race(), "version", i(1)));
    assert!(loaded.is_some());
    assert_eq!(defects, Vec::<String>::new());
}

#[test]
fn a_race_at_version_2_draws_the_unsupported_version_defect() {
    let (loaded, defects) = load_race_data(&set(&race(), "version", i(2)));
    assert!(loaded.is_none());
    assert_eq!(
        defects,
        vec!["field version names an unsupported version".to_string()]
    );
}

#[test]
fn a_race_with_a_text_version_draws_the_wrong_type_defect() {
    let (loaded, defects) = load_race_data(&set(&race(), "version", s("1")));
    assert!(loaded.is_none());
    assert_eq!(
        defects,
        vec!["field version has the wrong type".to_string()]
    );
}

#[test]
fn a_race_with_a_fractional_version_draws_the_wrong_type_defect() {
    let (loaded, defects) = load_race_data(&set(&race(), "version", f(1.5)));
    assert!(loaded.is_none());
    assert_eq!(
        defects,
        vec!["field version has the wrong type".to_string()]
    );
}

#[test]
fn a_race_with_a_whole_double_version_draws_the_wrong_type_defect() {
    let (loaded, defects) = load_race_data(&set(&race(), "version", f(1.0)));
    assert!(loaded.is_none());
    assert_eq!(
        defects,
        vec!["field version has the wrong type".to_string()]
    );
}

#[test]
fn a_series_at_version_1_loads_clean() {
    let (loaded, defects) = load_series_text(&text(&set(&series(), "version", i(1))));
    assert!(loaded.is_some());
    assert_eq!(defects, Vec::<String>::new());
}

#[test]
fn a_series_at_version_2_draws_the_unsupported_version_defect() {
    let (loaded, defects) = load_series_text(&text(&set(&series(), "version", i(2))));
    assert!(loaded.is_none());
    assert_eq!(
        defects,
        vec!["field version names an unsupported version".to_string()]
    );
}

#[test]
fn a_series_with_a_text_version_draws_the_wrong_type_defect() {
    let (loaded, defects) = load_series_text(&text(&set(&series(), "version", s("1"))));
    assert!(loaded.is_none());
    assert_eq!(
        defects,
        vec!["field version has the wrong type".to_string()]
    );
}

#[test]
fn a_race_inside_a_series_carries_its_version_defect_under_the_race_prefix() {
    let bad_race = set(&race(), "version", i(2));
    let value = set(&series(), "races", arr(vec![bad_race]));
    let (loaded, defects) = load_series_text(&text(&value));
    assert!(loaded.is_none());
    assert_eq!(
        defects,
        vec!["race 0 field version names an unsupported version".to_string()]
    );
}

#[test]
fn a_championship_at_version_2_draws_the_unsupported_version_defect() {
    let value = obj(vec![
        ("series", arr(vec![s("pilot-series.json")])),
        ("version", i(2)),
    ]);
    let (loaded, defects) = load_championship_text(&text(&value), &shared::examples());
    assert!(loaded.is_none());
    assert_eq!(
        defects,
        vec!["field version names an unsupported version".to_string()]
    );
}

#[test]
fn a_championship_at_version_1_loads_clean() {
    let value = obj(vec![
        ("series", arr(vec![s("pilot-series.json")])),
        ("version", i(1)),
    ]);
    let (loaded, defects) = load_championship_text(&text(&value), &shared::examples());
    assert!(loaded.is_some());
    assert_eq!(defects, Vec::<String>::new());
}

#[test]
fn a_named_series_carrying_version_2_refuses_the_championship_under_its_prefix() {
    let dir = temp_dir("formats-championship");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("bad-series.json"),
        text(&set(&series(), "version", i(2))),
    )
    .unwrap();
    let value = obj(vec![("series", arr(vec![s("bad-series.json")]))]);
    let (loaded, defects) = load_championship_text(&text(&value), &dir);
    assert!(loaded.is_none());
    assert_eq!(
        defects,
        vec!["series 0 field version names an unsupported version".to_string()]
    );
}

#[test]
fn a_standings_file_at_version_1_loads_clean() {
    let value = obj(vec![("models", obj(vec![])), ("version", i(1))]);
    let (loaded, defects) = load_standings_text(&text(&value));
    assert!(loaded.is_some());
    assert_eq!(defects, Vec::<String>::new());
}

#[test]
fn a_standings_file_at_version_2_draws_the_unsupported_version_defect() {
    let value = obj(vec![("models", obj(vec![])), ("version", i(2))]);
    let (loaded, defects) = load_standings_text(&text(&value));
    assert!(loaded.is_none());
    assert_eq!(
        defects,
        vec!["field version names an unsupported version".to_string()]
    );
}

#[test]
fn a_standings_file_with_a_text_version_draws_the_wrong_type_defect() {
    let value = obj(vec![("models", obj(vec![])), ("version", s("1"))]);
    let (loaded, defects) = load_standings_text(&text(&value));
    assert!(loaded.is_none());
    assert_eq!(
        defects,
        vec!["field version has the wrong type".to_string()]
    );
}

#[test]
fn an_adjudication_entry_at_version_2_draws_its_unsupported_version_defect() {
    let entry = set(&adjudication_entry(), "version", i(2));
    let value = set(&race(), "adjudication", entry);
    let (loaded, defects) = load_race_data(&value);
    assert!(loaded.is_none());
    assert_eq!(
        defects,
        vec!["adjudication field version names an unsupported version".to_string()]
    );
}

#[test]
fn an_adjudication_entry_with_a_text_version_draws_its_wrong_type_defect() {
    let entry = set(&adjudication_entry(), "version", s("1"));
    let value = set(&race(), "adjudication", entry);
    let (loaded, defects) = load_race_data(&value);
    assert!(loaded.is_none());
    assert_eq!(
        defects,
        vec!["adjudication field version has the wrong type".to_string()]
    );
}

#[test]
fn an_adjudication_entry_at_version_1_loads_clean() {
    let entry = set(&adjudication_entry(), "version", i(1));
    let value = set(&race(), "adjudication", entry);
    let (loaded, defects) = load_race_data(&value);
    assert!(loaded.is_some());
    assert_eq!(defects, Vec::<String>::new());
}

#[test]
fn a_panel_entry_at_version_2_draws_its_unsupported_version_defect() {
    let entry = set(&adjudication_entry(), "version", i(2));
    let value = set(&race(), "panel", arr(vec![entry]));
    let (loaded, defects) = load_race_data(&value);
    assert!(loaded.is_none());
    assert_eq!(
        defects,
        vec!["panel entry 0 field version names an unsupported version".to_string()]
    );
}

#[test]
fn a_panel_entry_with_a_text_version_draws_its_wrong_type_defect() {
    let entry = set(&adjudication_entry(), "version", s("1"));
    let value = set(&race(), "panel", arr(vec![entry]));
    let (loaded, defects) = load_race_data(&value);
    assert!(loaded.is_none());
    assert_eq!(
        defects,
        vec!["panel entry 0 field version has the wrong type".to_string()]
    );
}

#[test]
fn a_panel_entry_at_version_1_loads_clean() {
    let entry = set(&adjudication_entry(), "version", i(1));
    let value = set(&race(), "panel", arr(vec![entry]));
    let (loaded, defects) = load_race_data(&value);
    assert!(loaded.is_some());
    assert_eq!(defects, Vec::<String>::new());
}
