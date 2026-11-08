//! The offline makers of DH-8 and DH-10.
//!
//! The pilot maker rebuilds each adjudicated pilot record from
//! the recorded races and envelopes through the seat readers,
//! and rewrites the records, their golden closecall reports,
//! the failed call list, the run record, and the pilot series.
//! The panel maker joins each pair of pilot records through the
//! panel join and rewrites the panel records, their goldens,
//! and the panel series. Both are offline assemblies over
//! recorded files and reproduce the committed files byte for
//! byte. The live calling half of the seat is a maintainer
//! action through the connector tools and is not reimplemented
//! here, as PORT-6 in CONTRACT.md states. The two Python maker
//! scripts under examples are the behavioral spec.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::json::{self, JsonValue};
use crate::pynum::{Num, PyDecimal};
use crate::pytext::PText;
use crate::races::{close_call_report, load_race_file, race_to_json, Race};
use crate::seats::adjudicated_race;
use crate::series::{series_report, Series};

const ROUTES: [&str; 2] = ["jev", "openai"];

/// Read back one pilot run record value under FIX-DH-17.
///
/// The maker writes the run record at version 1, and a record
/// read back carrying a stated version in any other form is
/// refused in the posture PORT-11 states for the maker, an
/// error and nothing written. A record carrying no version
/// field reads as version 1, as every other shape in this repo
/// reads. The suite's recomputation of the run record stands
/// on this reading beside the writer.
pub fn check_run_record_version(record: &JsonValue) -> Result<(), String> {
    match crate::races::version_standing(record) {
        crate::races::VersionStanding::Current => Ok(()),
        _ => Err("the run record names a version the maker cannot read".to_string()),
    }
}

fn race_names() -> Vec<String> {
    (1..=10).map(|index| format!("race-{:02}", index)).collect()
}

fn dump(path: &Path, value: &JsonValue) -> Result<(), String> {
    write_text(path, &dump_text(value))
}

fn dump_text(value: &JsonValue) -> String {
    let mut text = json::dumps(value, Some(1), true);
    text.push('\n');
    text
}

fn write_text(path: &Path, text: &str) -> Result<(), String> {
    std::fs::write(path, text)
        .map_err(|error| format!("cannot write {}: {}", path.display(), error))
}

/// Read one envelope file. A file that does not stand is no
/// envelope, as the Python maker reads it. A file that stands
/// but cannot be read or parsed is a refusal, because the
/// Python maker dies on that file where an absent file simply
/// leaves the race uncounted, as PORT-11 states.
fn read_envelope(path: &Path, name: &str, route: &str) -> Result<Option<JsonValue>, String> {
    if !path.exists() {
        return Ok(None);
    }
    let text = crate::races::read_text_file(path).ok_or_else(|| {
        format!(
            "the envelope for {} on route {} cannot be read",
            name, route
        )
    })?;
    json::parse(&text).map(Some).map_err(|_| {
        format!(
            "the envelope for {} on route {} is not valid JSON",
            name, route
        )
    })
}

/// Return whether a value is falsy under the Python reading,
/// the test the maker applies to a usage figure before it
/// reads any figure from it. A missing figure, a null, a false,
/// a zero, and an empty text, list, or object are falsy, and
/// every other value stands, a not a number figure included.
fn py_falsy(value: &JsonValue) -> bool {
    match value {
        JsonValue::Null => true,
        JsonValue::Bool(flag) => !flag,
        JsonValue::Num(Num::Int(digits)) => *digits == num_bigint::BigInt::from(0),
        JsonValue::Num(Num::Float(figure)) => *figure == 0.0,
        JsonValue::Str(text) => text.is_empty(),
        JsonValue::Arr(items) => items.is_empty(),
        JsonValue::Obj(pairs) => pairs.is_empty(),
    }
}

/// Return the input and output token counts of one envelope, in
/// the field names of its route, read the way the Python maker
/// reads them. A falsy usage reads as an empty object. A truthy
/// usage that is not an object is a refusal, as is a token
/// figure standing in a shape an addition cannot take, a text,
/// a list, an object, or a null. A boolean token figure counts
/// as its number, as the Python addition counts it.
fn usage_figures(route: &str, envelope: &JsonValue) -> Result<(Num, Num), String> {
    let usage = envelope.get("usage").cloned().unwrap_or(JsonValue::Null);
    let usage = if py_falsy(&usage) {
        JsonValue::Obj(Vec::new())
    } else if usage.as_object().is_some() {
        usage
    } else {
        return Err(
            "a counted envelope holds its usage in a shape that names no figures".to_string(),
        );
    };
    let (input_field, output_field) = if route == "jev" {
        ("input_tokens", "output_tokens")
    } else {
        ("prompt_tokens", "completion_tokens")
    };
    let read = |field: &str| -> Result<Num, String> {
        match usage.get(field) {
            None => Ok(Num::from_i64(0)),
            Some(JsonValue::Num(num)) => Ok(num.clone()),
            Some(JsonValue::Bool(flag)) => Ok(Num::from_i64(i64::from(*flag))),
            Some(_) => Err(
                "a counted envelope holds a token figure in a shape an addition cannot take"
                    .to_string(),
            ),
        }
    };
    Ok((read(input_field)?, read(output_field)?))
}

/// Add two token figures the way the Python addition adds
/// them. An integer sum stays an integer. A sum touching a
/// double converts the integer, and an integer too large to
/// convert is a refusal, where the Python addition raises.
fn added_tokens(total: &Num, gained: &Num) -> Result<Num, String> {
    let converts = |num: &Num| -> Result<(), String> {
        if num.is_int() && !num.is_finite_number() {
            return Err(
                "a token sum meets an integer too large to convert to a double".to_string(),
            );
        }
        Ok(())
    };
    if matches!(total, Num::Float(_)) || matches!(gained, Num::Float(_)) {
        converts(total)?;
        converts(gained)?;
    }
    Ok(total.added(gained))
}

/// A running cost sum, the exact decimal of the figures added
/// so far, or a not a number figure once one has been added,
/// which then stands through every later addition, as the
/// Python decimal sum stands.
enum CostSum {
    Finite(PyDecimal),
    NotANumber { negative: bool },
}

impl CostSum {
    fn add(&mut self, literal: crate::pynum::DecimalLiteral) -> Result<(), String> {
        use crate::pynum::DecimalLiteral;
        match literal {
            DecimalLiteral::Infinite => Err(
                "a counted envelope holds an infinite cost figure, which no rounding takes"
                    .to_string(),
            ),
            DecimalLiteral::NotANumber { negative } => {
                if matches!(self, CostSum::Finite(_)) {
                    *self = CostSum::NotANumber { negative };
                }
                Ok(())
            }
            DecimalLiteral::Finite(value) => {
                if let CostSum::Finite(total) = self {
                    *total = total.added(&value);
                }
                Ok(())
            }
        }
    }

    /// Render the sum rounded to six places the way the run
    /// record renders it. A rounded figure carrying more
    /// digits than the Python decimal context holds is a
    /// refusal, where the Python rounding raises.
    fn render_rounded(&self) -> Result<String, String> {
        match self {
            CostSum::NotANumber { negative } => {
                Ok(if *negative { "-NaN" } else { "NaN" }.to_string())
            }
            CostSum::Finite(total) => {
                let rounded = total.quantized_six();
                if rounded.coefficient_digits() > 28 {
                    return Err(
                        "the cost sum rounds to a figure wider than the decimal context holds"
                            .to_string(),
                    );
                }
                Ok(rounded.render())
            }
        }
    }
}

/// Add the cost figure of one counted envelope on the openai
/// route to its sum, read the way the Python maker reads it.
/// A missing or falsy usage names no cost figure, and the
/// Python reading dies on the missing figure, as it dies on a
/// cost figure standing in a shape whose text is no decimal,
/// a boolean, a null, a list, or an object. A figure written
/// as text is summed as the decimal the text names.
fn add_cost(cost: &mut CostSum, envelope: &JsonValue) -> Result<(), String> {
    let usage = match envelope.get("usage") {
        Some(value) if !py_falsy(value) => value,
        _ => {
            return Err("a counted envelope names no cost figure".to_string());
        }
    };
    let usage = match usage.as_object() {
        Some(_) => usage,
        None => {
            return Err(
                "a counted envelope holds its usage in a shape that names no cost figure"
                    .to_string(),
            );
        }
    };
    let value = match usage.get("cost") {
        Some(value) => value,
        None => {
            return Err("a counted envelope names no cost figure".to_string());
        }
    };
    let literal = match value {
        JsonValue::Num(Num::Int(digits)) => crate::pynum::read_decimal_literal(&digits.to_string()),
        JsonValue::Num(Num::Float(figure)) => {
            crate::pynum::read_decimal_literal(&crate::pynum::py_float_repr(*figure))
        }
        JsonValue::Str(text) => crate::pynum::read_decimal_literal(&text.display()),
        _ => None,
    };
    match literal {
        Some(literal) => cost.add(literal),
        None => {
            Err("a counted envelope holds a cost figure the decimal reading refuses".to_string())
        }
    }
}

struct ModelFigures {
    route: String,
    calls_attempted: Num,
    entries_recorded: Num,
    input_tokens: Num,
    output_tokens: Num,
    cost: CostSum,
}

impl ModelFigures {
    fn new(route: &str) -> ModelFigures {
        ModelFigures {
            route: route.to_string(),
            calls_attempted: Num::from_i64(0),
            entries_recorded: Num::from_i64(0),
            input_tokens: Num::from_i64(0),
            output_tokens: Num::from_i64(0),
            cost: CostSum::Finite(PyDecimal::zero()),
        }
    }
}

/// Regenerate the pilot tree under the given pilot directory,
/// which holds the races and envelopes folders. The pilot
/// series lands in the parent of the pilot directory, as the
/// Python maker writes it. A race that does not load clean is
/// an error naming the race and its defects.
///
/// Every output is computed before any is written. The Python
/// maker writes as it goes and its crashes leave a half
/// rewritten tree behind, and the port does not imitate that
/// wreckage, as PORT-11 states. A refusal anywhere in the
/// reading, a figure the Python maker cannot read, a sum past
/// the digit limit, a cost too wide to round, fails the whole
/// run with an error and writes nothing.
pub fn make_pilot(pilot_dir: &Path) -> Result<(), String> {
    let races_dir = pilot_dir.join("races");
    let envelopes_dir = pilot_dir.join("envelopes");
    let records_dir = pilot_dir.join("records");
    let mut pending: Vec<(PathBuf, String)> = Vec::new();
    let mut failed: Vec<JsonValue> = Vec::new();
    let mut model_order: Vec<PText> = Vec::new();
    let mut models: BTreeMap<Vec<u16>, ModelFigures> = BTreeMap::new();
    let mut jev_records: Vec<Race> = Vec::new();
    for name in race_names() {
        let (race, defects) = load_race_file(&races_dir.join(format!("{}.json", name)));
        if !defects.is_empty() {
            return Err(format!("{} does not load clean: {:?}", name, defects));
        }
        let race = race.expect("a clean race");
        for route in ROUTES {
            let envelope_path = envelopes_dir.join(format!("{}-{}.json", name, route));
            let envelope = read_envelope(&envelope_path, &name, route)?;
            let record = match &envelope {
                Some(envelope) => adjudicated_race(&race, envelope, route),
                None => None,
            };
            let counted_model = match &envelope {
                Some(envelope) if envelope.as_object().is_some() => envelope
                    .get("model")
                    .and_then(|value| value.as_text().cloned()),
                _ => None,
            };
            if let (Some(model), Some(envelope)) = (&counted_model, &envelope) {
                let key: Vec<u16> = model.units().to_vec();
                if !models.contains_key(&key) {
                    model_order.push(model.clone());
                    models.insert(key.clone(), ModelFigures::new(route));
                }
                let figures = models.get_mut(&key).expect("counted figures");
                figures.calls_attempted = figures.calls_attempted.added(&Num::from_i64(1));
                let (gained_in, gained_out) = usage_figures(route, envelope)?;
                figures.input_tokens = added_tokens(&figures.input_tokens, &gained_in)?;
                figures.output_tokens = added_tokens(&figures.output_tokens, &gained_out)?;
                if record.is_some() {
                    figures.entries_recorded = figures.entries_recorded.added(&Num::from_i64(1));
                }
                if route == "openai" {
                    add_cost(&mut figures.cost, envelope)?;
                }
            }
            let record = match record {
                Some(record) => record,
                None => {
                    failed.push(JsonValue::object(vec![
                        (
                            PText::from_text("race"),
                            JsonValue::Str(PText::from_text(&name)),
                        ),
                        (
                            PText::from_text("route"),
                            JsonValue::Str(PText::from_text(route)),
                        ),
                    ]));
                    race.clone()
                }
            };
            pending.push((
                records_dir.join(format!("{}-{}.json", name, route)),
                dump_text(&race_to_json(&record)),
            ));
            let golden = close_call_report(&record).join("\n") + "\n";
            pending.push((
                records_dir.join(format!("{}-{}-closecall.txt", name, route)),
                golden,
            ));
            if route == "jev" {
                jev_records.push(record);
            }
        }
    }
    let mut model_pairs: Vec<(PText, JsonValue)> = Vec::new();
    for model in &model_order {
        let key: Vec<u16> = model.units().to_vec();
        let figures = models.get(&key).expect("counted figures");
        let failed_calls = failed
            .iter()
            .filter(|pair| {
                pair.get("route").and_then(|value| value.as_text())
                    == Some(&PText::from_text(&figures.route))
            })
            .count();
        let mut pairs = vec![
            (
                PText::from_text("calls_attempted"),
                JsonValue::Num(figures.calls_attempted.clone()),
            ),
            (
                PText::from_text("entries_recorded"),
                JsonValue::Num(figures.entries_recorded.clone()),
            ),
            (
                PText::from_text("failed_calls"),
                JsonValue::Num(Num::from_i64(failed_calls as i64)),
            ),
            (
                PText::from_text("input_tokens"),
                JsonValue::Num(figures.input_tokens.clone()),
            ),
            (
                PText::from_text("output_tokens"),
                JsonValue::Num(figures.output_tokens.clone()),
            ),
            (
                PText::from_text("route"),
                JsonValue::Str(PText::from_text(&figures.route)),
            ),
        ];
        if figures.route == "openai" {
            pairs.push((
                PText::from_text("cost"),
                JsonValue::Str(PText::from_text(&figures.cost.render_rounded()?)),
            ));
        }
        model_pairs.push((model.clone(), JsonValue::object(pairs)));
    }
    let run_record = JsonValue::object(vec![
        (PText::from_text("models"), JsonValue::object(model_pairs)),
        (
            PText::from_text("version"),
            JsonValue::Num(Num::from_i64(1)),
        ),
    ]);
    if json::has_int_past_digit_limit(&run_record) {
        return Err(
            "a summed figure in the run record passes the digit limit the rendering enforces"
                .to_string(),
        );
    }
    let series = Series {
        task: PText::from_text("the recorded close call pilot"),
        models: vec![PText::from_text("prime-1"), PText::from_text("shadow-1")],
        wins_target: num_bigint::BigInt::from(3),
        races: jev_records,
    };
    let parent: PathBuf = pilot_dir
        .parent()
        .map(|parent| parent.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));
    std::fs::create_dir_all(&records_dir)
        .map_err(|error| format!("cannot create the records folder: {}", error))?;
    for (path, text) in &pending {
        write_text(path, text)?;
    }
    dump(&pilot_dir.join("failed.json"), &JsonValue::Arr(failed))?;
    dump(&pilot_dir.join("run_record.json"), &run_record)?;
    dump(
        &parent.join("pilot-series.json"),
        &crate::series::series_to_json(&series),
    )?;
    write_text(
        &parent.join("pilot-series.txt"),
        &(series_report(&series).join("\n") + "\n"),
    )?;
    Ok(())
}

/// Regenerate the panel tree under the given panel directory,
/// whose sibling pilot directory holds the records the join
/// reads. The panel series lands in the parent of the panel
/// directory. A record that does not load clean, or a pair that
/// does not join clean, is an error naming it.
pub fn make_panel(panel_dir: &Path) -> Result<(), String> {
    let pilot_records = panel_dir
        .parent()
        .map(|parent| parent.join("pilot").join("records"))
        .unwrap_or_else(|| PathBuf::from("pilot/records"));
    let records_dir = panel_dir.join("records");
    std::fs::create_dir_all(&records_dir)
        .map_err(|error| format!("cannot create the records folder: {}", error))?;
    let mut panel_records: Vec<Race> = Vec::new();
    for name in race_names() {
        let mut loaded = Vec::new();
        for route in ROUTES {
            let (record, defects) =
                load_race_file(&pilot_records.join(format!("{}-{}.json", name, route)));
            if !defects.is_empty() {
                return Err(format!(
                    "{}-{} does not load clean: {:?}",
                    name, route, defects
                ));
            }
            loaded.push(record.expect("a clean record"));
        }
        let (joined, defects) = crate::panels::join_panel_records(&loaded[0], &loaded[1]);
        if !defects.is_empty() {
            return Err(format!("{} does not join clean: {:?}", name, defects));
        }
        let joined = joined.expect("a joined record");
        dump(
            &records_dir.join(format!("{}.json", name)),
            &race_to_json(&joined),
        )?;
        let golden = close_call_report(&joined).join("\n") + "\n";
        write_text(
            &records_dir.join(format!("{}-closecall.txt", name)),
            &golden,
        )?;
        panel_records.push(joined);
    }
    let series = Series {
        task: PText::from_text("the recorded close call panel"),
        models: vec![PText::from_text("prime-1"), PText::from_text("shadow-1")],
        wins_target: num_bigint::BigInt::from(3),
        races: panel_records,
    };
    let parent: PathBuf = panel_dir
        .parent()
        .map(|parent| parent.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));
    dump(
        &parent.join("panel-series.json"),
        &crate::series::series_to_json(&series),
    )?;
    write_text(
        &parent.join("panel-series.txt"),
        &(series_report(&series).join("\n") + "\n"),
    )?;
    Ok(())
}
