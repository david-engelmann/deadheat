//! Pins for the recorded close call pilot, DH-8 in
//! CONTRACT.md, translated from the Python corpus.
//!
//! The suite reads the committed races, envelopes, and records
//! and makes no live call. The run record pin recomputes every
//! figure from the envelopes and the failed call list by an
//! independent path, and the regeneration pin proves the
//! pilot maker rebuilds the committed records byte for byte
//! offline.

mod shared;

use deadheat::json::JsonValue;
use deadheat::pynum::PyDecimal;
use deadheat::races::{is_close_call, load_race_file};
use deadheat::seats::{
    adjudicated_race, entry_from_chat_envelope, entry_from_jev_envelope, has_seat,
};
use deadheat::series::{load_series_file, series_report};
use shared::*;

fn race_names() -> Vec<String> {
    (1..=10).map(|index| format!("race-{index:02}")).collect()
}

const ROUTES: [&str; 2] = ["jev", "openai"];

fn pilot() -> std::path::PathBuf {
    examples().join("pilot")
}

fn load_json(path: &std::path::Path) -> JsonValue {
    let text = std::fs::read_to_string(path).expect("a json file");
    deadheat::json::parse(&text).expect("json parses")
}

fn entry_for(
    route: &str,
    race: &deadheat::races::Race,
    envelope: &JsonValue,
) -> Option<deadheat::races::Adjudication> {
    if route == "jev" {
        entry_from_jev_envelope(race, envelope)
    } else {
        entry_from_chat_envelope(race, envelope)
    }
}

#[test]
fn every_pilot_race_loads_clean_and_sits_in_the_band() {
    for name in race_names() {
        let (race, defects) = load_race_file(&pilot().join("races").join(format!("{name}.json")));
        assert_eq!(defects, Vec::<String>::new(), "{name}");
        let race = race.expect("a race");
        assert!(is_close_call(&race), "{name}");
        assert!(has_seat(&race), "{name}");
        let labels: Vec<String> = race
            .attempts
            .iter()
            .map(|attempt| attempt.model.display())
            .collect();
        assert_eq!(labels, vec!["prime-1", "shadow-1"], "{name}");
    }
}

#[test]
fn every_record_is_its_envelope_read_through_the_seat() {
    for name in race_names() {
        let (race, defects) = load_race_file(&pilot().join("races").join(format!("{name}.json")));
        assert_eq!(defects, Vec::<String>::new(), "{name}");
        let race = race.expect("a race");
        for route in ROUTES {
            let envelope = load_json(
                &pilot()
                    .join("envelopes")
                    .join(format!("{name}-{route}.json")),
            );
            let (record, record_defects) =
                load_race_file(&pilot().join("records").join(format!("{name}-{route}.json")));
            assert_eq!(record_defects, Vec::<String>::new(), "{name} {route}");
            let rebuilt = adjudicated_race(&race, &envelope, route).expect("a rebuilt record");
            assert_eq!(record.expect("a record"), rebuilt, "{name} {route}");
        }
    }
}

#[test]
fn the_failed_list_stands_exactly_where_no_entry_stands() {
    let failed = load_json(&pilot().join("failed.json"));
    let pairs: Vec<(String, String)> = failed
        .as_array()
        .expect("a list")
        .iter()
        .map(|pair| {
            (
                pair.get("race")
                    .expect("a race name")
                    .as_text()
                    .expect("text")
                    .display(),
                pair.get("route")
                    .expect("a route")
                    .as_text()
                    .expect("text")
                    .display(),
            )
        })
        .collect();
    for name in race_names() {
        for route in ROUTES {
            let record = load_json(&pilot().join("records").join(format!("{name}-{route}.json")));
            let has_entry = record.get("adjudication").is_some();
            let listed = pairs.contains(&(name.clone(), route.to_string()));
            assert_eq!(listed, !has_entry, "{name} {route}");
        }
    }
}

/// Normalize a JSON value for comparison the way a Python
/// dict comparison reads it, object keys in label order at
/// every level.
fn normalized(value: &JsonValue) -> JsonValue {
    match value {
        JsonValue::Obj(pairs) => {
            let mut sorted: Vec<(deadheat::pytext::PText, JsonValue)> = pairs
                .iter()
                .map(|(key, item)| (key.clone(), normalized(item)))
                .collect();
            sorted.sort_by(|a, b| a.0.cmp_code_points(&b.0));
            JsonValue::Obj(sorted)
        }
        JsonValue::Arr(items) => JsonValue::Arr(items.iter().map(normalized).collect()),
        other => other.clone(),
    }
}

#[test]
fn the_run_record_recomputes_from_the_envelopes() {
    let failed = load_json(&pilot().join("failed.json"));
    // Per model figures in first seen order, as the Python
    // recomputation builds them.
    struct Figures {
        model: String,
        route: String,
        attempted: i64,
        entries: i64,
        failed_calls: i64,
        input: i64,
        output: i64,
        cost: PyDecimal,
    }
    let mut all: Vec<Figures> = Vec::new();
    for name in race_names() {
        let (race, defects) = load_race_file(&pilot().join("races").join(format!("{name}.json")));
        assert_eq!(defects, Vec::<String>::new(), "{name}");
        let race = race.expect("a race");
        for route in ROUTES {
            let envelope = load_json(
                &pilot()
                    .join("envelopes")
                    .join(format!("{name}-{route}.json")),
            );
            let model = envelope
                .get("model")
                .expect("a model")
                .as_text()
                .expect("text")
                .display();
            let index = match all.iter().position(|figures| figures.model == model) {
                Some(index) => index,
                None => {
                    all.push(Figures {
                        model: model.clone(),
                        route: route.to_string(),
                        attempted: 0,
                        entries: 0,
                        failed_calls: 0,
                        input: 0,
                        output: 0,
                        cost: PyDecimal::zero(),
                    });
                    all.len() - 1
                }
            };
            let figures = &mut all[index];
            figures.attempted += 1;
            let usage = envelope.get("usage").expect("usage");
            let figure = |key: &str| -> i64 {
                usage
                    .get(key)
                    .expect("a usage figure")
                    .as_num()
                    .expect("a number")
                    .plain_text()
                    .parse::<i64>()
                    .expect("an integer figure")
            };
            if route == "jev" {
                figures.input += figure("input_tokens");
                figures.output += figure("output_tokens");
            } else {
                figures.input += figure("prompt_tokens");
                figures.output += figure("completion_tokens");
                let cost = usage
                    .get("cost")
                    .expect("a cost")
                    .as_num()
                    .expect("a number")
                    .plain_text();
                figures.cost = figures.cost.added(&PyDecimal::parse(&cost));
            }
            if entry_for(route, &race, &envelope).is_some() {
                figures.entries += 1;
            }
        }
    }
    for pair in failed.as_array().expect("a list") {
        let route = pair
            .get("route")
            .expect("a route")
            .as_text()
            .expect("text")
            .display();
        for figures in all.iter_mut() {
            if figures.route == route {
                figures.failed_calls += 1;
            }
        }
    }
    let models: Vec<(String, JsonValue)> = all
        .iter()
        .map(|figures| {
            let mut pairs = vec![
                ("route".to_string(), s(&figures.route)),
                ("calls_attempted".to_string(), i(figures.attempted)),
                ("entries_recorded".to_string(), i(figures.entries)),
                ("failed_calls".to_string(), i(figures.failed_calls)),
                ("input_tokens".to_string(), i(figures.input)),
                ("output_tokens".to_string(), i(figures.output)),
            ];
            if figures.route == "openai" {
                pairs.push((
                    "cost".to_string(),
                    s(&figures.cost.quantized_six().render()),
                ));
            }
            let object = JsonValue::object(
                pairs
                    .into_iter()
                    .map(|(key, value)| (deadheat::pytext::PText::from_text(&key), value))
                    .collect(),
            );
            (figures.model.clone(), object)
        })
        .collect();
    let recomputed = obj(vec![(
        "models",
        JsonValue::object(
            models
                .into_iter()
                .map(|(key, value)| (deadheat::pytext::PText::from_text(&key), value))
                .collect(),
        ),
    )]);
    let recorded = load_json(&pilot().join("run_record.json"));
    assert_eq!(normalized(&recorded), normalized(&recomputed));
}

fn copy_tree(from: &std::path::Path, to: &std::path::Path) {
    std::fs::create_dir_all(to).expect("a directory");
    for entry in std::fs::read_dir(from).expect("a listing") {
        let entry = entry.expect("an entry");
        let target = to.join(entry.file_name());
        if entry.file_type().expect("a type").is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), &target).expect("a copy");
        }
    }
}

fn watched_pilot_files(root: &std::path::Path) -> Vec<std::path::PathBuf> {
    let pilot = root.join("pilot");
    let mut watched: Vec<std::path::PathBuf> = std::fs::read_dir(pilot.join("records"))
        .expect("a listing")
        .map(|entry| entry.expect("an entry").path())
        .collect();
    watched.push(pilot.join("failed.json"));
    watched.push(pilot.join("run_record.json"));
    watched.push(root.join("pilot-series.json"));
    watched.push(root.join("pilot-series.txt"));
    watched.sort();
    watched
}

#[test]
fn the_pilot_maker_regenerates_the_pilot_byte_for_byte() {
    let dir = temp_dir("pilot-maker");
    let copied = dir.join("examples");
    copy_tree(&examples(), &copied);
    let watched = watched_pilot_files(&copied);
    let before: Vec<(std::path::PathBuf, Vec<u8>)> = watched
        .iter()
        .map(|path| (path.clone(), std::fs::read(path).expect("bytes")))
        .collect();
    deadheat::makers::make_pilot(&copied.join("pilot")).expect("the maker runs");
    for (path, bytes) in before {
        assert_eq!(
            std::fs::read(&path).expect("bytes"),
            bytes,
            "{}",
            path.display()
        );
    }
}

#[test]
fn the_maker_counts_an_attempt_where_it_reads_the_envelope() {
    let dir = temp_dir("pilot-maker-counts");
    let copied = dir.join("examples");
    copy_tree(&examples(), &copied);
    std::fs::remove_file(
        copied
            .join("pilot")
            .join("envelopes")
            .join("race-10-openai.json"),
    )
    .expect("the envelope removed");
    deadheat::makers::make_pilot(&copied.join("pilot")).expect("the maker runs");
    let recorded = load_json(&copied.join("pilot").join("run_record.json"));
    let models = recorded
        .get("models")
        .expect("models")
        .as_object()
        .expect("an object");
    let by_route = |route: &str| -> JsonValue {
        models
            .iter()
            .find(|(_, figures)| {
                figures.get("route").and_then(|held| held.as_text())
                    == Some(deadheat::pytext::PText::from_text(route)).as_ref()
            })
            .map(|(_, figures)| figures.clone())
            .expect("figures for the route")
    };
    let openai = by_route("openai");
    assert_eq!(openai.get("calls_attempted"), Some(&i(9)));
    assert_eq!(openai.get("failed_calls"), Some(&i(1)));
    let jev = by_route("jev");
    assert_eq!(jev.get("calls_attempted"), Some(&i(10)));
    assert_eq!(jev.get("failed_calls"), Some(&i(0)));
}

// The port repair pins below, for the maker findings of the
// port red team and the port spec review, under the refusal
// behavior PORT-11 states. Each was proven against the pre
// repair tree, where it fails, before it was allowed to stand
// here.

/// Copy only the inputs of the pilot, the races and the
/// envelopes, into a fresh pilot directory under dir, so a
/// refusal can be proven to have written nothing at all.
fn bare_pilot(dir: &std::path::Path) -> std::path::PathBuf {
    let pilot = dir.join("pilot");
    for part in ["races", "envelopes"] {
        copy_tree(&examples().join("pilot").join(part), &pilot.join(part));
    }
    pilot
}

fn doctor_envelope(path: &std::path::Path, edited: JsonValue) {
    std::fs::write(path, deadheat::json::dumps(&edited, Some(1), true) + "\n")
        .expect("the doctored envelope");
}

fn assert_refused_and_nothing_written(pilot: &std::path::Path) {
    let outcome = deadheat::makers::make_pilot(pilot);
    assert!(outcome.is_err(), "the maker refuses");
    assert!(!pilot.join("records").exists(), "no records written");
    assert!(!pilot.join("failed.json").exists(), "no failed list");
    assert!(!pilot.join("run_record.json").exists(), "no run record");
    let parent = pilot.parent().expect("a parent");
    assert!(!parent.join("pilot-series.json").exists(), "no series");
}

#[test]
fn the_maker_refuses_a_counted_envelope_missing_its_cost() {
    // Red team Major 2. The Python maker dies on the missing
    // figure where the port once wrote a run record with the
    // cost silently zeroed.
    let dir = temp_dir("pilot-refuses-missing-cost");
    let pilot = bare_pilot(&dir);
    let path = pilot.join("envelopes").join("race-01-openai.json");
    let edited = remove_path(&load_json(&path), &["usage", "cost"]);
    doctor_envelope(&path, edited);
    assert_refused_and_nothing_written(&pilot);
}

#[test]
fn the_maker_refuses_a_usage_that_is_not_an_object() {
    // Red team Major 2. A truthy text usage names no figures,
    // and the Python maker dies reading them.
    let dir = temp_dir("pilot-refuses-text-usage");
    let pilot = bare_pilot(&dir);
    let path = pilot.join("envelopes").join("race-01-openai.json");
    let edited = set_path(&load_json(&path), &["usage"], s("yes"));
    doctor_envelope(&path, edited);
    assert_refused_and_nothing_written(&pilot);
}

#[test]
fn the_maker_refuses_an_envelope_file_that_is_not_json() {
    // The confirmed third member of the same crash class. The
    // Python maker dies in its reading of the file where the
    // port once recorded the race as simply failed.
    let dir = temp_dir("pilot-refuses-bad-envelope");
    let pilot = bare_pilot(&dir);
    let path = pilot.join("envelopes").join("race-03-jev.json");
    std::fs::write(&path, "this is not json").expect("the doctored envelope");
    assert_refused_and_nothing_written(&pilot);
}

#[test]
fn the_maker_refuses_a_token_figure_an_addition_cannot_take() {
    // The reading half of PORT-11. A token figure written as
    // text or as null crashes the Python addition, so the
    // maker refuses it in either shape.
    for (name, edited_marker) in [("text", 0), ("null", 1)] {
        let dir = temp_dir(&format!("pilot-refuses-token-shape-{name}"));
        let pilot = bare_pilot(&dir);
        let path = pilot.join("envelopes").join("race-01-openai.json");
        let edited = if edited_marker == 0 {
            set_path(&load_json(&path), &["usage", "prompt_tokens"], s("227"))
        } else {
            set_path(
                &load_json(&path),
                &["usage", "completion_tokens"],
                JsonValue::Null,
            )
        };
        doctor_envelope(&path, edited);
        assert_refused_and_nothing_written(&pilot);
    }
}

#[test]
fn the_maker_refuses_token_sums_past_the_digit_limit() {
    // Red team Major 2, the sum half. Two envelopes of one
    // model each carry a prompt token count of 4300 digits,
    // and their run record sum needs 4301 digits, which the
    // Python rendering refuses.
    let dir = temp_dir("pilot-refuses-bigsum");
    let pilot = bare_pilot(&dir);
    let big = format!("6{}", "0".repeat(4299));
    for name in ["race-01-openai.json", "race-02-openai.json"] {
        let path = pilot.join("envelopes").join(name);
        let edited = set_path(&load_json(&path), &["usage", "prompt_tokens"], bigint(&big));
        doctor_envelope(&path, edited);
    }
    assert_refused_and_nothing_written(&pilot);
}

#[test]
fn the_maker_refuses_a_cost_too_wide_to_round() {
    // The rounding half of the same discipline. A cost of
    // 1e30 rounds to a figure wider than the Python decimal
    // context holds, and the Python maker dies at its
    // rounding, where the port once rendered the wide figure.
    let dir = temp_dir("pilot-refuses-wide-cost");
    let pilot = bare_pilot(&dir);
    let path = pilot.join("envelopes").join("race-01-openai.json");
    let wide = format!("1{}", "0".repeat(30));
    let edited = set_path(&load_json(&path), &["usage", "cost"], bigint(&wide));
    doctor_envelope(&path, edited);
    assert_refused_and_nothing_written(&pilot);
}

#[test]
fn the_maker_sums_costs_written_as_text_and_regenerates_byte_for_byte() {
    // Spec Minor 4. A cost figure written as text is summed
    // by the Python maker as the decimal the text names, where
    // the port once skipped it and summed only the numeric
    // costs. With every openai cost of the pilot rewritten as
    // the text of the same figure, the whole regenerated tree
    // must still stand byte for byte with the committed tree.
    let dir = temp_dir("pilot-maker-text-costs");
    let copied = dir.join("examples");
    copy_tree(&examples(), &copied);
    for name in race_names() {
        let path = copied
            .join("pilot")
            .join("envelopes")
            .join(format!("{name}-openai.json"));
        let envelope = load_json(&path);
        let cost = get_path(&envelope, &["usage", "cost"])
            .expect("a cost")
            .as_num()
            .expect("a number")
            .plain_text();
        let edited = set_path(&envelope, &["usage", "cost"], s(&cost));
        doctor_envelope(&path, edited);
    }
    let watched = watched_pilot_files(&copied);
    let before: Vec<(std::path::PathBuf, Vec<u8>)> = watched
        .iter()
        .map(|path| (path.clone(), std::fs::read(path).expect("bytes")))
        .collect();
    deadheat::makers::make_pilot(&copied.join("pilot")).expect("the maker runs");
    for (path, bytes) in before {
        assert_eq!(
            std::fs::read(&path).expect("bytes"),
            bytes,
            "{}",
            path.display()
        );
    }
}

#[test]
fn the_maker_counts_a_boolean_token_figure_as_one() {
    // Spec Minor 4. A boolean token figure counts as its
    // number in the Python addition, where the port once read
    // it as absent. Race 01 of the pilot reports 227 prompt
    // tokens of the model total of 2335, so a true in its
    // place moves the total to 2109 and nothing else moves.
    let dir = temp_dir("pilot-maker-bool-tokens");
    let copied = dir.join("examples");
    copy_tree(&examples(), &copied);
    let path = copied
        .join("pilot")
        .join("envelopes")
        .join("race-01-openai.json");
    let edited = set_path(
        &load_json(&path),
        &["usage", "prompt_tokens"],
        JsonValue::Bool(true),
    );
    doctor_envelope(&path, edited);
    deadheat::makers::make_pilot(&copied.join("pilot")).expect("the maker runs");
    let recorded = load_json(&copied.join("pilot").join("run_record.json"));
    let models = recorded
        .get("models")
        .expect("models")
        .as_object()
        .expect("an object");
    let openai = models
        .iter()
        .find(|(_, figures)| {
            figures.get("route").and_then(|held| held.as_text())
                == Some(deadheat::pytext::PText::from_text("openai")).as_ref()
        })
        .map(|(_, figures)| figures)
        .expect("the openai figures");
    assert_eq!(openai.get("input_tokens"), Some(&i(2109)));
    assert_eq!(openai.get("output_tokens"), Some(&i(1843)));
    assert_eq!(
        openai.get("cost").and_then(|held| held.as_text()),
        Some(&deadheat::pytext::PText::from_text("0.001155"))
    );
}

#[test]
fn the_pilot_series_counts_the_jev_records() {
    let (series, defects) = load_series_file(&examples().join("pilot-series.json"));
    assert_eq!(defects, Vec::<String>::new());
    let series = series.expect("a series");
    for (index, name) in race_names().iter().enumerate() {
        let (record, record_defects) =
            load_race_file(&pilot().join("records").join(format!("{name}-jev.json")));
        assert_eq!(record_defects, Vec::<String>::new(), "{name}");
        assert_eq!(series.races[index], record.expect("a record"), "{name}");
    }
    let golden = std::fs::read_to_string(examples().join("pilot-series.txt")).expect("a golden");
    assert_eq!(series_report(&series).join("\n") + "\n", golden);
    assert!(golden.contains("race 1, pin the cache key, prime-1 wins"));
    assert!(golden.contains("race 7, shrink the prompt budget, shadow-1 wins"));
    assert_eq!(golden.lines().last(), Some("series winner, prime-1"));
}
