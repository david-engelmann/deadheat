//! The deadheat command line surface, as a library function.
//!
//! Six commands work on recorded races, series, and the
//! standings file. The dispatch mirrors the Python argparse
//! surface of the same name, the help and usage text wraps at
//! the width the Python surface wraps at under PORT-7, the
//! arguments are carried as the code unit sequences the
//! operating system bytes decode to under the surrogate escape
//! reading, and a usage refusal echoes an argument in the form
//! the Python refusal echoes it. The function returns the
//! stdout bytes, the stderr bytes, and the exit status, and
//! main.rs is a thin shell over it.

use std::path::PathBuf;

use crate::championship::{championship_report, load_championship_file};
use crate::closings::closing_report;
use crate::clusters::cluster_report;
use crate::comebacks::comeback_report;
use crate::crossings::crossings_report;
use crate::deadlocks::deadlock_report;
use crate::depths::depths_report;
use crate::distances::distance_report;
use crate::flips::flip_report;
use crate::leads::lead_report;
use crate::openings::opening_report;
use crate::pairings::pairing_report;
use crate::pytext::PText;
use crate::races::{
    close_call_report, decide_winner, empty_standings, funeral, load_race_file,
    load_standings_file, patch_file_name, race_report, race_to_json, standings_table,
    standings_text, update_standings,
};
use crate::rallies::rally_report;
use crate::series::{load_series_file, series_report};
use crate::spells::spells_report;
use crate::streaks::streak_report;
use crate::tenure::tenure_report;
use crate::totals::totals_report;
use crate::turns::turns_report;

/// The outcome of one command line run.
pub struct Outcome {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub code: i32,
}

impl Outcome {
    fn plain(lines: Vec<String>, code: i32) -> Outcome {
        let mut stdout = Vec::new();
        for line in lines {
            stdout.extend_from_slice(line.as_bytes());
            stdout.push(b'\n');
        }
        Outcome {
            stdout,
            stderr: Vec::new(),
            code,
        }
    }
}

/// Decode one argument's operating system bytes the way the
/// Python runtime decodes them, valid UTF-8 to the characters
/// it names and every undecodable byte to its surrogate escape
/// code unit.
pub fn decode_arg(bytes: &[u8]) -> PText {
    let mut units: Vec<u16> = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        match std::str::from_utf8(&bytes[index..]) {
            Ok(rest) => {
                units.extend(rest.encode_utf16());
                break;
            }
            Err(error) => {
                let valid = error.valid_up_to();
                let head = std::str::from_utf8(&bytes[index..index + valid])
                    .expect("the valid prefix is valid");
                units.extend(head.encode_utf16());
                let skip = error.error_len().unwrap_or(bytes.len() - (index + valid));
                for offset in 0..skip {
                    units.push(0xDC00 + bytes[index + valid + offset] as u16);
                }
                index += valid + skip;
            }
        }
    }
    PText::from_units(units)
}

/// Encode one argument back to the operating system bytes it
/// names, the inverse of the decode, for opening the file it
/// names.
fn arg_bytes(text: &PText) -> Vec<u8> {
    let units = text.units();
    let mut out = Vec::new();
    let mut index = 0;
    while index < units.len() {
        let unit = units[index];
        if (0xDC80..=0xDCFF).contains(&unit) {
            // The surrogate escape range runs from the escape of
            // byte 0x80 through the escape of byte 0xFF, its end
            // included, as the Python reading spans it.
            out.push((unit - 0xDC00) as u8);
            index += 1;
        } else if (0xD800..0xDC00).contains(&unit)
            && index + 1 < units.len()
            && (0xDC00..0xE000).contains(&units[index + 1])
        {
            let point =
                0x10000 + ((((unit as u32) - 0xD800) << 10) | ((units[index + 1] as u32) - 0xDC00));
            push_utf8(&mut out, char::from_u32(point).unwrap_or('\u{FFFD}'));
            index += 2;
        } else if (0xD800..0xE000).contains(&unit) {
            // A lone surrogate that did not arrive as an escaped
            // byte names no file; the byte stands in so the open
            // simply fails, as an unencodable path fails.
            out.push(b'?');
            index += 1;
        } else {
            push_utf8(&mut out, char::from_u32(unit as u32).unwrap_or('\u{FFFD}'));
            index += 1;
        }
    }
    out
}

fn push_utf8(out: &mut Vec<u8>, c: char) {
    let mut buffer = [0u8; 4];
    out.extend_from_slice(c.encode_utf8(&mut buffer).as_bytes());
}

#[cfg(unix)]
fn path_from_arg(text: &PText) -> PathBuf {
    use std::os::unix::ffi::OsStringExt;
    let bytes = arg_bytes(text);
    if bytes.is_empty() {
        // The Python path reading reads an empty path as the
        // current directory.
        return PathBuf::from(".");
    }
    PathBuf::from(std::ffi::OsString::from_vec(bytes))
}

#[cfg(not(unix))]
fn path_from_arg(text: &PText) -> PathBuf {
    PathBuf::from(text.display())
}

/// The echo form of a patches folder argument joined with a
/// file name, in the form the Python path reading prints the
/// joined path in, its dot segments and repeated separators
/// folded the way that reading folds them. The folding works
/// on the operating system bytes themselves, so a folder byte
/// no encoding accepts passes through to the echo exactly as
/// the Python reading prints it back.
fn joined_path_echo(folder: &PText, file_name: &str) -> Vec<u8> {
    let raw = arg_bytes(folder);
    let normalized = normalize_path_bytes(&raw);
    let mut out = Vec::new();
    if normalized == b"." {
        out.extend_from_slice(file_name.as_bytes());
    } else if normalized.ends_with(b"/") {
        out.extend_from_slice(&normalized);
        out.extend_from_slice(file_name.as_bytes());
    } else {
        out.extend_from_slice(&normalized);
        out.push(b'/');
        out.extend_from_slice(file_name.as_bytes());
    }
    out
}

fn normalize_path_bytes(bytes: &[u8]) -> Vec<u8> {
    if bytes.is_empty() {
        return b".".to_vec();
    }
    let leading: &[u8] = if bytes.starts_with(b"//") && !bytes.starts_with(b"///") {
        b"//"
    } else if bytes.starts_with(b"/") {
        b"/"
    } else {
        b""
    };
    let mut parts: Vec<&[u8]> = Vec::new();
    for segment in bytes.split(|byte| *byte == b'/') {
        if segment.is_empty() || segment == b"." {
            continue;
        }
        parts.push(segment);
    }
    if parts.is_empty() {
        if leading.is_empty() {
            return b".".to_vec();
        }
        return leading.to_vec();
    }
    let mut out = leading.to_vec();
    for (index, part) in parts.iter().enumerate() {
        if index > 0 {
            out.push(b'/');
        }
        out.extend_from_slice(part);
    }
    out
}

/// The width the help and usage text wraps at, under the rule
/// PORT-7 states. The COLUMNS value when it is set and
/// positive, else the terminal width, else eighty.
pub fn terminal_columns() -> usize {
    if let Ok(value) = std::env::var("COLUMNS") {
        if let Some(columns) = parse_columns(&value) {
            return columns;
        }
    }
    terminal_width().unwrap_or(80)
}

/// Read a COLUMNS value the way the Python surface reads it.
///
/// The Python surface converts the value with its int
/// conversion, whose grammar the port mirrors here, as the
/// spec review of the port required. The conversion strips
/// the whitespace set of the Python string reading from both
/// ends, takes one leading sign, and then takes digits with
/// single underscores standing between digits and nowhere
/// else, where a digit is any code point of category Nd at
/// its decimal value. A value the conversion refuses, and a
/// value of zero or below, names no width. A value past the
/// stored bound settles at the bound, 1000000 columns, past
/// which every layout this surface wraps is already at its
/// widest and no printed line can change.
fn parse_columns(value: &str) -> Option<usize> {
    let held: Vec<char> = value.chars().collect();
    let mut start = 0;
    let mut end = held.len();
    while start < end && crate::pyunicode::is_stripped_space(held[start] as u32) {
        start += 1;
    }
    while end > start && crate::pyunicode::is_stripped_space(held[end - 1] as u32) {
        end -= 1;
    }
    let mut rest: &[char] = &held[start..end];
    let mut negative = false;
    if let Some(first) = rest.first() {
        if *first == '+' || *first == '-' {
            negative = *first == '-';
            rest = &rest[1..];
        }
    }
    let mut magnitude: u64 = 0;
    let mut digits = 0usize;
    let mut underscores = 0usize;
    for point in rest {
        if *point == '_' {
            underscores += 1;
            continue;
        }
        let digit = crate::pyunicode::nd_digit_value(*point as u32)?;
        if underscores > 1 || (underscores == 1 && digits == 0) {
            return None;
        }
        underscores = 0;
        digits += 1;
        magnitude = magnitude
            .saturating_mul(10)
            .saturating_add(digit as u64)
            .min(1_000_000);
    }
    if digits == 0 || underscores > 0 || negative || magnitude == 0 {
        return None;
    }
    Some(magnitude as usize)
}

#[cfg(unix)]
fn terminal_width() -> Option<usize> {
    #[repr(C)]
    struct Winsize {
        ws_row: u16,
        ws_col: u16,
        ws_xpixel: u16,
        ws_ypixel: u16,
    }
    extern "C" {
        fn ioctl(fd: i32, request: u64, ...) -> i32;
    }
    let mut size = Winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    // TIOCGWINSZ on Linux.
    let result = unsafe { ioctl(1, 0x5413, &mut size as *mut Winsize) };
    if result == 0 && size.ws_col > 0 {
        Some(size.ws_col as usize)
    } else {
        None
    }
}

#[cfg(not(unix))]
fn terminal_width() -> Option<usize> {
    None
}

/// One action line of a help screen.
struct ActionSpec {
    invocation: &'static str,
    help: Option<&'static str>,
    subactions: Vec<ActionSpec>,
}

/// One parser of the surface, described for the formatter.
struct ParserSpec {
    prog: &'static str,
    description: Option<&'static str>,
    opt_usage: &'static str,
    pos_usage: &'static str,
    opt_parts: Vec<&'static str>,
    pos_parts: Vec<&'static str>,
    positionals: Vec<ActionSpec>,
    options: Vec<ActionSpec>,
    max_invocation: usize,
}

fn action(invocation: &'static str, help: Option<&'static str>) -> ActionSpec {
    ActionSpec {
        invocation,
        help,
        subactions: Vec::new(),
    }
}

fn help_action() -> ActionSpec {
    action("-h, --help", Some("show this help message and exit"))
}

fn spec_max_invocation(spec: &ParserSpec) -> usize {
    let mut max = 0;
    for item in spec.positionals.iter().chain(spec.options.iter()) {
        max = max.max(item.invocation.len());
        for sub in &item.subactions {
            max = max.max(sub.invocation.len());
        }
    }
    max
}

fn top_spec() -> ParserSpec {
    let mut commands = action(
        "{race,adopt,discard,standings,closecall,series,championship,flip,totals,pairing,comeback,distance,run,rally,deadlock,streak,cluster,closing,opening,turns,lead,crossings,tenure,depths,spells}",
        None,
    );
    commands.subactions = vec![
        action(
            "race",
            Some("print the report and funeral for one recorded race"),
        ),
        action("adopt", Some("write the loser patch into a patches folder")),
        action(
            "discard",
            Some("remove the loser patch from a patches folder"),
        ),
        action("standings", Some("print the standings table")),
        action(
            "closecall",
            Some("print whether one recorded race sits in the close call band and why"),
        ),
        action(
            "series",
            Some("print the series report for one series file"),
        ),
        action(
            "championship",
            Some("print the championship report for one championship file"),
        ),
        action("flip", Some("print the flip report for one series file")),
        action(
            "totals",
            Some("print the totals report for one championship file"),
        ),
        action(
            "pairing",
            Some("print the pairing report for one championship file"),
        ),
        action(
            "comeback",
            Some("print the comeback report for one championship file"),
        ),
        action(
            "distance",
            Some("print the distance report for one championship file"),
        ),
        action(
            "run",
            Some("run the two attempts of one race specification and print its report and funeral"),
        ),
        action(
            "rally",
            Some("print the rally report for one championship file"),
        ),
        action(
            "deadlock",
            Some("print the deadlock report for one championship file"),
        ),
        action(
            "streak",
            Some("print the streak report for one championship file"),
        ),
        action(
            "cluster",
            Some("print the cluster report for one championship file"),
        ),
        action(
            "closing",
            Some("print the closing report for one championship file"),
        ),
        action(
            "opening",
            Some("print the opening report for one championship file"),
        ),
        action(
            "turns",
            Some("print the turns report for one championship file"),
        ),
        action(
            "lead",
            Some("print the lead report for one championship file"),
        ),
        action(
            "crossings",
            Some("print the crossings report for one championship file"),
        ),
        action(
            "tenure",
            Some("print the tenure report for one championship file"),
        ),
        action(
            "depths",
            Some("print the depths report for one championship file"),
        ),
        action(
            "spells",
            Some("print the spells report for one championship file"),
        ),
    ];
    let mut spec = ParserSpec {
        prog: "deadheat",
        description: Some("Race two attempts at one task and keep the standings."),
        opt_usage: "[-h]",
        pos_usage:
            "{race,adopt,discard,standings,closecall,series,championship,flip,totals,pairing,comeback,distance,run,rally,deadlock,streak,cluster,closing,opening,turns,lead,crossings,tenure,depths,spells} ...",
        opt_parts: vec!["[-h]"],
        pos_parts: vec![
            "{race,adopt,discard,standings,closecall,series,championship,flip,totals,pairing,comeback,distance,run,rally,deadlock,streak,cluster,closing,opening,turns,lead,crossings,tenure,depths,spells}",
            "...",
        ],
        positionals: vec![commands],
        options: vec![help_action()],
        max_invocation: 0,
    };
    spec.max_invocation = spec_max_invocation(&spec);
    spec
}

fn sub_spec(command: &str) -> ParserSpec {
    let (prog, positionals, mut options, opt_usage, pos_usage, pos_parts): (
        &'static str,
        Vec<ActionSpec>,
        Vec<ActionSpec>,
        &'static str,
        &'static str,
        Vec<&'static str>,
    ) = match command {
        "race" => {
            let mut options = vec![help_action()];
            options.push(action(
                "--standings STANDINGS",
                Some("standings file to update after the race"),
            ));
            (
                "deadheat race",
                vec![action("file", None)],
                options,
                "[-h] [--standings STANDINGS]",
                "file",
                vec!["file"],
            )
        }
        "adopt" => (
            "deadheat adopt",
            vec![action("file", None), action("patches", None)],
            vec![help_action()],
            "[-h]",
            "file patches",
            vec!["file", "patches"],
        ),
        "discard" => (
            "deadheat discard",
            vec![action("file", None), action("patches", None)],
            vec![help_action()],
            "[-h]",
            "file patches",
            vec!["file", "patches"],
        ),
        "standings" => (
            "deadheat standings",
            vec![action("file", None)],
            vec![help_action()],
            "[-h]",
            "file",
            vec!["file"],
        ),
        "closecall" => (
            "deadheat closecall",
            vec![action("file", None)],
            vec![help_action()],
            "[-h]",
            "file",
            vec!["file"],
        ),
        "series" => (
            "deadheat series",
            vec![action("file", None)],
            vec![help_action()],
            "[-h]",
            "file",
            vec!["file"],
        ),
        "championship" => (
            "deadheat championship",
            vec![action("file", None)],
            vec![help_action()],
            "[-h]",
            "file",
            vec!["file"],
        ),
        "flip" => (
            "deadheat flip",
            vec![action("file", None)],
            vec![help_action()],
            "[-h]",
            "file",
            vec!["file"],
        ),
        "totals" => (
            "deadheat totals",
            vec![action("file", None)],
            vec![help_action()],
            "[-h]",
            "file",
            vec!["file"],
        ),
        "pairing" => (
            "deadheat pairing",
            vec![action("file", None)],
            vec![help_action()],
            "[-h]",
            "file",
            vec!["file"],
        ),
        "comeback" => (
            "deadheat comeback",
            vec![action("file", None)],
            vec![help_action()],
            "[-h]",
            "file",
            vec!["file"],
        ),
        "distance" => (
            "deadheat distance",
            vec![action("file", None)],
            vec![help_action()],
            "[-h]",
            "file",
            vec!["file"],
        ),
        "run" => (
            "deadheat run",
            vec![action("file", None)],
            vec![help_action()],
            "[-h]",
            "file",
            vec!["file"],
        ),
        "rally" => (
            "deadheat rally",
            vec![action("file", None)],
            vec![help_action()],
            "[-h]",
            "file",
            vec!["file"],
        ),
        "deadlock" => (
            "deadheat deadlock",
            vec![action("file", None)],
            vec![help_action()],
            "[-h]",
            "file",
            vec!["file"],
        ),
        "streak" => (
            "deadheat streak",
            vec![action("file", None)],
            vec![help_action()],
            "[-h]",
            "file",
            vec!["file"],
        ),
        "cluster" => (
            "deadheat cluster",
            vec![action("file", None)],
            vec![help_action()],
            "[-h]",
            "file",
            vec!["file"],
        ),
        "closing" => (
            "deadheat closing",
            vec![action("file", None)],
            vec![help_action()],
            "[-h]",
            "file",
            vec!["file"],
        ),
        "opening" => (
            "deadheat opening",
            vec![action("file", None)],
            vec![help_action()],
            "[-h]",
            "file",
            vec!["file"],
        ),
        "turns" => (
            "deadheat turns",
            vec![action("file", None)],
            vec![help_action()],
            "[-h]",
            "file",
            vec!["file"],
        ),
        "lead" => (
            "deadheat lead",
            vec![action("file", None)],
            vec![help_action()],
            "[-h]",
            "file",
            vec!["file"],
        ),
        "crossings" => (
            "deadheat crossings",
            vec![action("file", None)],
            vec![help_action()],
            "[-h]",
            "file",
            vec!["file"],
        ),
        "tenure" => (
            "deadheat tenure",
            vec![action("file", None)],
            vec![help_action()],
            "[-h]",
            "file",
            vec!["file"],
        ),
        "depths" => (
            "deadheat depths",
            vec![action("file", None)],
            vec![help_action()],
            "[-h]",
            "file",
            vec!["file"],
        ),
        "spells" => (
            "deadheat spells",
            vec![action("file", None)],
            vec![help_action()],
            "[-h]",
            "file",
            vec!["file"],
        ),
        _ => unreachable!("a known command"),
    };
    let _ = &mut options;
    let mut spec = ParserSpec {
        prog,
        description: None,
        opt_usage,
        pos_usage,
        opt_parts: if command == "race" {
            vec!["[-h]", "[--standings STANDINGS]"]
        } else {
            vec!["[-h]"]
        },
        pos_parts,
        positionals,
        options,
        max_invocation: 0,
    };
    spec.max_invocation = spec_max_invocation(&spec);
    spec
}

/// Wrap one text the way the Python text wrapper wraps it, a
/// greedy word wrap at the given width with a word longer than
/// the width split at the width.
fn text_wrap(text: &str, width: i64) -> Vec<String> {
    if width <= 0 {
        return Vec::new();
    }
    let width = width as usize;
    let mut lines: Vec<String> = Vec::new();
    let mut current = String::new();
    for word in text.split(' ') {
        if word.is_empty() {
            continue;
        }
        let mut rest = word;
        loop {
            if current.is_empty() {
                if rest.len() <= width {
                    current = rest.to_string();
                    rest = "";
                } else {
                    lines.push(rest[..width].to_string());
                    rest = &rest[width..];
                }
            } else if current.len() + 1 + rest.len() <= width {
                current.push(' ');
                current.push_str(rest);
                rest = "";
            } else {
                lines.push(std::mem::take(&mut current));
            }
            if rest.is_empty() {
                break;
            }
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

/// The usage block of a parser at a width, wrapped when it
/// overflows, in the form both the help screen and the refusal
/// lines carry.
fn format_usage(spec: &ParserSpec, width: i64) -> String {
    let prefix = "usage: ";
    let full = format!("{} {} {}", spec.prog, spec.opt_usage, spec.pos_usage);
    let usage = if (prefix.len() + full.len()) as i64 > width {
        wrap_usage(spec, width)
    } else {
        full
    };
    format!("{}{}", prefix, usage)
}

fn get_lines(parts: &[&str], indent: &str, prefix: Option<&str>, text_width: i64) -> Vec<String> {
    let indent_length = indent.len() as i64;
    let mut lines: Vec<String> = Vec::new();
    let mut line: Vec<&str> = Vec::new();
    let mut line_len: i64 = match prefix {
        Some(prefix) => prefix.len() as i64 - 1,
        None => indent_length - 1,
    };
    for part in parts {
        if line_len + 1 + part.len() as i64 > text_width && !line.is_empty() {
            lines.push(format!("{}{}", indent, line.join(" ")));
            line = Vec::new();
            line_len = indent_length - 1;
        }
        line.push(part);
        line_len += part.len() as i64 + 1;
    }
    if !line.is_empty() {
        lines.push(format!("{}{}", indent, line.join(" ")));
    }
    if prefix.is_some() && !lines.is_empty() {
        lines[0] = lines[0][indent.len()..].to_string();
    }
    lines
}

fn wrap_usage(spec: &ParserSpec, text_width: i64) -> String {
    let prefix = "usage: ";
    let prog = spec.prog;
    let short = (prefix.len() + prog.len()) as f64 <= 0.75 * text_width as f64;
    let lines: Vec<String> = if short {
        let indent = " ".repeat(prefix.len() + prog.len() + 1);
        if !spec.opt_parts.is_empty() {
            let mut first: Vec<&str> = vec![prog];
            first.extend(spec.opt_parts.iter());
            let mut lines = get_lines(&first, &indent, Some(prefix), text_width);
            lines.extend(get_lines(&spec.pos_parts, &indent, None, text_width));
            lines
        } else if !spec.pos_parts.is_empty() {
            let mut first: Vec<&str> = vec![prog];
            first.extend(spec.pos_parts.iter());
            get_lines(&first, &indent, Some(prefix), text_width)
        } else {
            vec![prog.to_string()]
        }
    } else {
        let indent = " ".repeat(prefix.len());
        let mut all: Vec<&str> = Vec::new();
        all.extend(spec.opt_parts.iter());
        all.extend(spec.pos_parts.iter());
        let mut lines = get_lines(&all, &indent, None, text_width);
        if lines.len() > 1 {
            lines = get_lines(&spec.opt_parts, &indent, None, text_width);
            lines.extend(get_lines(&spec.pos_parts, &indent, None, text_width));
        }
        let mut with_prog = vec![prog.to_string()];
        with_prog.extend(lines);
        with_prog
    };
    lines.join("\n")
}

fn format_action(
    spec_action: &ActionSpec,
    indent: usize,
    help_position: i64,
    width: i64,
    out: &mut String,
) {
    let help_width = (width - help_position).max(11);
    let action_width = help_position - indent as i64 - 2;
    match spec_action.help {
        None => {
            out.push_str(&" ".repeat(indent));
            out.push_str(spec_action.invocation);
            out.push('\n');
        }
        Some(help) => {
            let help_lines = text_wrap(help, help_width);
            if (spec_action.invocation.len() as i64) <= action_width {
                out.push_str(&" ".repeat(indent));
                out.push_str(&format!(
                    "{:<width$}",
                    spec_action.invocation,
                    width = action_width as usize
                ));
                out.push_str("  ");
                if let Some(first) = help_lines.first() {
                    out.push_str(first);
                }
                out.push('\n');
                for line in help_lines.iter().skip(1) {
                    out.push_str(&" ".repeat(help_position as usize));
                    out.push_str(line);
                    out.push('\n');
                }
            } else {
                out.push_str(&" ".repeat(indent));
                out.push_str(spec_action.invocation);
                out.push('\n');
                for line in help_lines.iter() {
                    out.push_str(&" ".repeat(help_position as usize));
                    out.push_str(line);
                    out.push('\n');
                }
            }
        }
    }
    for sub in &spec_action.subactions {
        format_action(sub, indent + 2, help_position, width, out);
    }
}

/// The full help text of a parser at the given terminal
/// columns, the text printed to stdout with a trailing newline
/// on its last line.
fn format_help(spec: &ParserSpec, columns: usize) -> String {
    let width = columns as i64 - 2;
    let max_help_position = 24.min((width - 20).max(4));
    let help_position = (spec.max_invocation as i64 + 4).min(max_help_position);
    let mut out = String::new();
    out.push_str(&format_usage(spec, width));
    out.push_str("\n\n");
    if let Some(description) = spec.description {
        // The text formatter floors its width at eleven, as
        // the Python formatter does for the description.
        for (index, line) in text_wrap(description, width.max(11)).iter().enumerate() {
            if index > 0 {
                out.push('\n');
            }
            out.push_str(line);
        }
        out.push_str("\n\n");
    }
    let sections: [(&str, &Vec<ActionSpec>); 2] = [
        ("positional arguments", &spec.positionals),
        ("options", &spec.options),
    ];
    for (index, (heading, actions)) in sections.iter().enumerate() {
        if index > 0 {
            out.push('\n');
        }
        out.push_str(heading);
        out.push_str(":\n");
        for item in actions.iter() {
            format_action(item, 2, help_position, width, &mut out);
        }
    }
    out
}

/// A usage refusal, the usage block and the error line on
/// stderr with exit status 2.
fn usage_error(spec: &ParserSpec, columns: usize, message: &str) -> Outcome {
    let width = columns as i64 - 2;
    let mut stderr = Vec::new();
    stderr.extend_from_slice(format_usage(spec, width).as_bytes());
    stderr.push(b'\n');
    stderr.extend_from_slice(format!("{}: error: {}\n", spec.prog, message).as_bytes());
    Outcome {
        stdout: Vec::new(),
        stderr,
        code: 2,
    }
}

/// The echo form of an argument in a plain, unquoted error
/// message. The text renders as itself, with every unpaired
/// surrogate rendered as its unicode escape, the form the
/// Python writes to its error stream under backslashreplace.
fn echo_text(text: &PText) -> String {
    let units = text.units();
    let mut out = String::new();
    let mut index = 0;
    while index < units.len() {
        let unit = units[index];
        if (0xD800..0xDC00).contains(&unit)
            && index + 1 < units.len()
            && (0xDC00..0xE000).contains(&units[index + 1])
        {
            let point =
                0x10000 + ((((unit as u32) - 0xD800) << 10) | ((units[index + 1] as u32) - 0xDC00));
            out.push(char::from_u32(point).unwrap_or('\u{FFFD}'));
            index += 2;
        } else if (0xD800..0xE000).contains(&unit) {
            out.push_str(&format!("\\u{:04x}", unit));
            index += 1;
        } else {
            out.push(char::from_u32(unit as u32).unwrap_or('\u{FFFD}'));
            index += 1;
        }
    }
    out
}

/// The text form the Python repr gives a string, which the
/// invalid choice refusal and the ignored explicit argument
/// refusal echo the refused argument in. One walk applies the
/// Python rule whole. The quote is a double quote when the
/// text holds a single quote and no double quote, and a
/// single quote otherwise, and the chosen quote is escaped
/// where it stands in the text. The backslash and the three
/// named control characters take their short escapes. Every
/// other code point stands as itself when the Python
/// printable test holds it printable, read through the
/// category table in pyunicode, and takes its hexadecimal
/// escape at its width otherwise. An unpaired surrogate takes
/// its escape at the four digit width, as the repr of the
/// Python string renders it.
fn py_repr(text: &PText) -> String {
    let units = text.units();
    let has_single = units.contains(&0x27);
    let has_double = units.contains(&0x22);
    let quote = if has_single && !has_double { '"' } else { '\'' };
    let mut out = String::new();
    out.push(quote);
    let mut index = 0;
    while index < units.len() {
        let unit = units[index];
        if (0xD800..0xDC00).contains(&unit)
            && index + 1 < units.len()
            && (0xDC00..0xE000).contains(&units[index + 1])
        {
            let point =
                0x10000 + ((((unit as u32) - 0xD800) << 10) | ((units[index + 1] as u32) - 0xDC00));
            push_repr_point(&mut out, point, quote);
            index += 2;
            continue;
        }
        if (0xD800..0xE000).contains(&unit) {
            out.push_str(&format!("\\u{:04x}", unit));
            index += 1;
            continue;
        }
        push_repr_point(&mut out, unit as u32, quote);
        index += 1;
    }
    out.push(quote);
    out
}

fn push_repr_point(out: &mut String, point: u32, quote: char) {
    match point {
        0x5C => out.push_str("\\\\"),
        0x0A => out.push_str("\\n"),
        0x0D => out.push_str("\\r"),
        0x09 => out.push_str("\\t"),
        point if point == quote as u32 => {
            out.push('\\');
            out.push(char::from_u32(point).expect("the quote is a character"));
        }
        point if crate::pyunicode::is_printable(point) => {
            out.push(char::from_u32(point).expect("a printable point is a character"));
        }
        point if point < 0x100 => out.push_str(&format!("\\x{:02x}", point)),
        point if point < 0x10000 => out.push_str(&format!("\\u{:04x}", point)),
        point => out.push_str(&format!("\\U{:08x}", point)),
    }
}

/// Return whether a token reads as an option rather than a
/// value, under the argparse reading, a leading dash on more
/// than one character that does not start a negative number.
fn is_option_like(text: &PText) -> bool {
    let units = text.units();
    if units.len() < 2 || units[0] != 0x2D {
        return false;
    }
    if units[1] >= 0x30 && units[1] <= 0x39 {
        return false;
    }
    if units[1] == 0x2E && units.len() > 2 && units[2] >= 0x30 && units[2] <= 0x39 {
        return false;
    }
    true
}

fn text_is(text: &PText, word: &str) -> bool {
    *text == PText::from_text(word)
}

fn starts_with_dashdash(text: &PText) -> bool {
    let units = text.units();
    units.len() >= 2 && units[0] == 0x2D && units[1] == 0x2D
}

/// The outcome of matching one long option token against the
/// long options of a parser.
enum LongMatch {
    Help,
    Standings,
    Unknown,
    HelpWithValue(PText),
    StandingsWithValue(PText),
}

/// Match a token that starts with two dashes against the long
/// options in play. An unambiguous prefix matches, as the
/// Python surface matches it, and a value rides after an equals
/// sign. The match works on the code units of the token, so a
/// value carrying a byte no encoding accepts rides through as
/// its surrogate escape units, the way the Python reading
/// holds it, rather than through a rendered text that would
/// trade those units for a replacement character.
fn match_long(text: &PText, has_standings: bool) -> LongMatch {
    let units = text.units();
    let (name, value) = match units.iter().position(|unit| *unit == 0x3D) {
        Some(split) => (
            &units[..split],
            Some(PText::from_units(units[split + 1..].to_vec())),
        ),
        None => (units, None),
    };
    if name.len() <= 2 {
        // The bare separator carries no option name, and the
        // Python matcher never reads it as an abbreviation.
        return LongMatch::Unknown;
    }
    let mut candidates: Vec<&'static str> = vec!["--help"];
    if has_standings {
        candidates.push("--standings");
    }
    let matched: Vec<&&str> = candidates
        .iter()
        .filter(|candidate| {
            let held: Vec<u16> = candidate.encode_utf16().collect();
            held == name || held.starts_with(name)
        })
        .collect();
    if matched.len() != 1 {
        return LongMatch::Unknown;
    }
    match *matched[0] {
        "--help" => match value {
            Some(value) => LongMatch::HelpWithValue(value),
            None => LongMatch::Help,
        },
        _ => match value {
            Some(value) => LongMatch::StandingsWithValue(value),
            None => LongMatch::Standings,
        },
    }
}

fn is_help_token(text: &PText, has_standings: bool) -> bool {
    if text_is(text, "-h") {
        return true;
    }
    let units = text.units();
    if units.len() >= 2 && units[0] == 0x2D && units[1] == 0x68 && !starts_with_dashdash(text) {
        // A short cluster opening with the help flag.
        return true;
    }
    if starts_with_dashdash(text) {
        return matches!(match_long(text, has_standings), LongMatch::Help);
    }
    false
}

const COMMANDS: [&str; 25] = [
    "race",
    "adopt",
    "discard",
    "standings",
    "closecall",
    "series",
    "championship",
    "flip",
    "totals",
    "pairing",
    "comeback",
    "distance",
    "run",
    "rally",
    "deadlock",
    "streak",
    "cluster",
    "closing",
    "opening",
    "turns",
    "lead",
    "crossings",
    "tenure",
    "depths",
    "spells",
];

/// Run the command line over the given arguments at the given
/// terminal width, returning the outcome.
pub fn run(args: &[PText], columns: usize) -> Outcome {
    let top = top_spec();
    let mut extras: Vec<PText> = Vec::new();
    let mut index = 0;
    let command: String;
    let sub_args: &[PText];
    loop {
        if index >= args.len() {
            return usage_error(
                &top,
                columns,
                "the following arguments are required: command",
            );
        }
        let token = &args[index];
        if is_help_token(token, false) {
            return Outcome::plain(
                format_help(&top, columns)
                    .lines()
                    .map(|line| line.to_string())
                    .collect(),
                0,
            );
        }
        if starts_with_dashdash(token) {
            if let LongMatch::HelpWithValue(value) = match_long(token, false) {
                return usage_error(
                    &top,
                    columns,
                    &format!(
                        "argument -h/--help: ignored explicit argument {}",
                        py_repr(&value)
                    ),
                );
            }
        }
        if text_is(token, "--") {
            // The separator reached while the command is still
            // unsought is itself the refused value when any
            // token follows it, and the required error when
            // none does, as the Python surface reads it.
            if index + 1 >= args.len() {
                return usage_error(
                    &top,
                    columns,
                    "the following arguments are required: command",
                );
            }
            return invalid_choice(&top, columns, token);
        }
        if let Some(found) = COMMANDS.iter().find(|name| text_is(token, name)) {
            command = found.to_string();
            sub_args = &args[index + 1..];
            break;
        }
        if is_option_like(token) {
            extras.push(token.clone());
            index += 1;
            continue;
        }
        return invalid_choice(&top, columns, token);
    }
    let _ = &extras;
    run_subcommand(&command, sub_args, extras, columns)
}

fn invalid_choice(spec: &ParserSpec, columns: usize, value: &PText) -> Outcome {
    usage_error(
        spec,
        columns,
        &format!(
            "argument command: invalid choice: {} (choose from 'race', 'adopt', 'discard', 'standings', 'closecall', 'series', 'championship', 'flip', 'totals', 'pairing', 'comeback', 'distance', 'run', 'rally', 'deadlock', 'streak', 'cluster', 'closing', 'opening', 'turns', 'lead', 'crossings', 'tenure', 'depths', 'spells')",
            py_repr(value)
        ),
    )
}

enum SubParse {
    Done(Outcome),
    Parsed {
        values: Vec<PText>,
        standings: Option<PText>,
        extras: Vec<PText>,
    },
}

fn parse_subcommand(
    command: &str,
    args: &[PText],
    top_extras: Vec<PText>,
    columns: usize,
) -> SubParse {
    let spec = sub_spec(command);
    let positional_names: &[&str] = match command {
        "adopt" | "discard" => &["file", "patches"],
        _ => &["file"],
    };
    let has_standings = command == "race";
    let mut values: Vec<PText> = Vec::new();
    let mut extras: Vec<PText> = top_extras;
    let mut standings: Option<PText> = None;
    let mut positional_only = false;
    let mut index = 0;
    while index < args.len() {
        let token = &args[index];
        if !positional_only {
            if text_is(token, "--") {
                positional_only = true;
                index += 1;
                continue;
            }
            if is_help_token(token, has_standings) {
                return SubParse::Done(Outcome::plain(
                    format_help(&spec, columns)
                        .lines()
                        .map(|line| line.to_string())
                        .collect(),
                    0,
                ));
            }
            if starts_with_dashdash(token) {
                match match_long(token, has_standings) {
                    LongMatch::Help => {
                        return SubParse::Done(Outcome::plain(
                            format_help(&spec, columns)
                                .lines()
                                .map(|line| line.to_string())
                                .collect(),
                            0,
                        ));
                    }
                    LongMatch::HelpWithValue(value) => {
                        return SubParse::Done(usage_error(
                            &spec,
                            columns,
                            &format!(
                                "argument -h/--help: ignored explicit argument {}",
                                py_repr(&value)
                            ),
                        ));
                    }
                    LongMatch::StandingsWithValue(value) => {
                        standings = Some(value);
                        index += 1;
                        continue;
                    }
                    LongMatch::Standings => {
                        if index + 1 >= args.len() || is_option_like(&args[index + 1]) {
                            return SubParse::Done(usage_error(
                                &spec,
                                columns,
                                "argument --standings: expected one argument",
                            ));
                        }
                        standings = Some(args[index + 1].clone());
                        index += 2;
                        continue;
                    }
                    LongMatch::Unknown => {
                        extras.push(token.clone());
                        index += 1;
                        continue;
                    }
                }
            }
            if is_option_like(token) {
                extras.push(token.clone());
                index += 1;
                continue;
            }
        }
        if values.len() < positional_names.len() {
            values.push(token.clone());
        } else {
            extras.push(token.clone());
        }
        index += 1;
    }
    if values.len() < positional_names.len() {
        let missing = &positional_names[values.len()..];
        return SubParse::Done(usage_error(
            &spec,
            columns,
            &format!(
                "the following arguments are required: {}",
                missing.join(", ")
            ),
        ));
    }
    SubParse::Parsed {
        values,
        standings,
        extras,
    }
}

fn run_subcommand(
    command: &str,
    args: &[PText],
    top_extras: Vec<PText>,
    columns: usize,
) -> Outcome {
    let (values, standings, extras) = match parse_subcommand(command, args, top_extras, columns) {
        SubParse::Done(outcome) => return outcome,
        SubParse::Parsed {
            values,
            standings,
            extras,
        } => (values, standings, extras),
    };
    if !extras.is_empty() {
        let top = top_spec();
        let joined = extras
            .iter()
            .map(echo_text)
            .collect::<Vec<String>>()
            .join(" ");
        return usage_error(
            &top,
            columns,
            &format!("unrecognized arguments: {}", joined),
        );
    }
    match command {
        "race" => command_race(&values[0], standings.as_ref()),
        "adopt" => command_adopt(&values[0], &values[1]),
        "discard" => command_discard(&values[0], &values[1]),
        "standings" => command_standings(&values[0]),
        "closecall" => command_closecall(&values[0]),
        "series" => command_series(&values[0]),
        "championship" => command_championship(&values[0]),
        "flip" => command_flip(&values[0]),
        "totals" => command_totals(&values[0]),
        "pairing" => command_pairing(&values[0]),
        "comeback" => command_comeback(&values[0]),
        "distance" => command_distance(&values[0]),
        "run" => command_run(&values[0]),
        "rally" => command_rally(&values[0]),
        "deadlock" => command_deadlock(&values[0]),
        "streak" => command_streak(&values[0]),
        "cluster" => command_cluster(&values[0]),
        "closing" => command_closing(&values[0]),
        "opening" => command_opening(&values[0]),
        "turns" => command_turns(&values[0]),
        "lead" => command_lead(&values[0]),
        "crossings" => command_crossings(&values[0]),
        "tenure" => command_tenure(&values[0]),
        "depths" => command_depths(&values[0]),
        "spells" => command_spells(&values[0]),
        _ => unreachable!("a known command"),
    }
}

fn load_race_or_print_defects(path: &PText) -> Result<crate::races::Race, Outcome> {
    let (race, defects) = load_race_file(&path_from_arg(path));
    if !defects.is_empty() {
        let mut lines = vec!["invalid recorded race".to_string()];
        for defect in defects {
            lines.push(format!("defect {}", defect));
        }
        return Err(Outcome::plain(lines, 1));
    }
    Ok(race.expect("a clean race"))
}

fn print_defects(headline: &str, defects: Vec<String>) -> Outcome {
    let mut lines = vec![headline.to_string()];
    for defect in defects {
        lines.push(format!("defect {}", defect));
    }
    Outcome::plain(lines, 1)
}

fn command_race(file: &PText, standings_arg: Option<&PText>) -> Outcome {
    let race = match load_race_or_print_defects(file) {
        Ok(race) => race,
        Err(outcome) => return outcome,
    };
    if let Some(standings_path_arg) = standings_arg {
        let standings_path = path_from_arg(standings_path_arg);
        let standings = if standings_path.exists() {
            let (standings, defects) = load_standings_file(&standings_path);
            if !defects.is_empty() {
                return print_defects("invalid standings file", defects);
            }
            standings.expect("clean standings")
        } else {
            empty_standings()
        };
        let updated = update_standings(&standings, &race);
        if let Some(parent) = standings_path.parent() {
            if !parent.as_os_str().is_empty() {
                let _ = std::fs::create_dir_all(parent);
            }
        }
        if crate::json::has_int_past_digit_limit(&updated.to_json()) {
            // A summed figure past the digit limit cannot be
            // rendered, and the Python command fails at exactly
            // this point, after the folder stands and before
            // the file is written or anything is printed, as
            // PORT-11 states.
            return Outcome::plain(Vec::new(), 1);
        }
        if std::fs::write(&standings_path, standings_text(&updated)).is_err() {
            return Outcome::plain(Vec::new(), 1);
        }
    }
    let mut lines = race_report(&race);
    lines.extend(funeral(&race));
    Outcome::plain(lines, 0)
}

fn command_rally(file: &PText) -> Outcome {
    let (championship, defects) = load_championship_file(&path_from_arg(file));
    if !defects.is_empty() {
        return print_defects("invalid championship file", defects);
    }
    Outcome::plain(
        rally_report(&championship.expect("a clean championship")),
        0,
    )
}

fn command_deadlock(file: &PText) -> Outcome {
    let (championship, defects) = load_championship_file(&path_from_arg(file));
    if !defects.is_empty() {
        return print_defects("invalid championship file", defects);
    }
    Outcome::plain(
        deadlock_report(&championship.expect("a clean championship")),
        0,
    )
}

fn command_streak(file: &PText) -> Outcome {
    let (championship, defects) = load_championship_file(&path_from_arg(file));
    if !defects.is_empty() {
        return print_defects("invalid championship file", defects);
    }
    Outcome::plain(
        streak_report(&championship.expect("a clean championship")),
        0,
    )
}

fn command_cluster(file: &PText) -> Outcome {
    let (championship, defects) = load_championship_file(&path_from_arg(file));
    if !defects.is_empty() {
        return print_defects("invalid championship file", defects);
    }
    Outcome::plain(
        cluster_report(&championship.expect("a clean championship")),
        0,
    )
}

fn command_closing(file: &PText) -> Outcome {
    let (championship, defects) = load_championship_file(&path_from_arg(file));
    if !defects.is_empty() {
        return print_defects("invalid championship file", defects);
    }
    Outcome::plain(
        closing_report(&championship.expect("a clean championship")),
        0,
    )
}

fn command_opening(file: &PText) -> Outcome {
    let (championship, defects) = load_championship_file(&path_from_arg(file));
    if !defects.is_empty() {
        return print_defects("invalid championship file", defects);
    }
    Outcome::plain(
        opening_report(&championship.expect("a clean championship")),
        0,
    )
}

fn command_turns(file: &PText) -> Outcome {
    let (championship, defects) = load_championship_file(&path_from_arg(file));
    if !defects.is_empty() {
        return print_defects("invalid championship file", defects);
    }
    Outcome::plain(
        turns_report(&championship.expect("a clean championship")),
        0,
    )
}

fn command_lead(file: &PText) -> Outcome {
    let (championship, defects) = load_championship_file(&path_from_arg(file));
    if !defects.is_empty() {
        return print_defects("invalid championship file", defects);
    }
    Outcome::plain(lead_report(&championship.expect("a clean championship")), 0)
}

fn command_crossings(file: &PText) -> Outcome {
    let (championship, defects) = load_championship_file(&path_from_arg(file));
    if !defects.is_empty() {
        return print_defects("invalid championship file", defects);
    }
    Outcome::plain(
        crossings_report(&championship.expect("a clean championship")),
        0,
    )
}

fn command_tenure(file: &PText) -> Outcome {
    let (championship, defects) = load_championship_file(&path_from_arg(file));
    if !defects.is_empty() {
        return print_defects("invalid championship file", defects);
    }
    Outcome::plain(
        tenure_report(&championship.expect("a clean championship")),
        0,
    )
}

fn command_depths(file: &PText) -> Outcome {
    let (championship, defects) = load_championship_file(&path_from_arg(file));
    if !defects.is_empty() {
        return print_defects("invalid championship file", defects);
    }
    Outcome::plain(
        depths_report(&championship.expect("a clean championship")),
        0,
    )
}

fn command_spells(file: &PText) -> Outcome {
    let (championship, defects) = load_championship_file(&path_from_arg(file));
    if !defects.is_empty() {
        return print_defects("invalid championship file", defects);
    }
    Outcome::plain(
        spells_report(&championship.expect("a clean championship")),
        0,
    )
}

fn command_adopt(file: &PText, patches: &PText) -> Outcome {
    let race = match load_race_or_print_defects(file) {
        Ok(race) => race,
        Err(outcome) => return outcome,
    };
    let (_, loser) = decide_winner(&race);
    let loser = match loser {
        None => {
            return Outcome::plain(
                vec!["no loser patch, the race had no winner".to_string()],
                1,
            )
        }
        Some(loser) => loser,
    };
    let folder = path_from_arg(patches);
    if std::fs::create_dir_all(&folder).is_err() {
        return Outcome::plain(Vec::new(), 1);
    }
    let name = patch_file_name(&race);
    let target = folder.join(&name);
    if std::fs::write(&target, race.attempts[loser].patch.display()).is_err() {
        return Outcome::plain(Vec::new(), 1);
    }
    let mut stdout = b"adopted the loser patch to ".to_vec();
    stdout.extend_from_slice(&joined_path_echo(patches, &name));
    stdout.push(b'\n');
    Outcome {
        stdout,
        stderr: Vec::new(),
        code: 0,
    }
}

fn command_discard(file: &PText, patches: &PText) -> Outcome {
    let race = match load_race_or_print_defects(file) {
        Ok(race) => race,
        Err(outcome) => return outcome,
    };
    let (_, loser) = decide_winner(&race);
    if loser.is_none() {
        return Outcome::plain(
            vec!["no loser patch, the race had no winner".to_string()],
            1,
        );
    }
    let name = patch_file_name(&race);
    let target = path_from_arg(patches).join(&name);
    let _ = std::fs::remove_file(&target);
    let mut stdout = b"discarded the loser patch at ".to_vec();
    stdout.extend_from_slice(&joined_path_echo(patches, &name));
    stdout.push(b'\n');
    Outcome {
        stdout,
        stderr: Vec::new(),
        code: 0,
    }
}

fn command_standings(file: &PText) -> Outcome {
    let (standings, defects) = load_standings_file(&path_from_arg(file));
    if !defects.is_empty() {
        return print_defects("invalid standings file", defects);
    }
    Outcome::plain(standings_table(&standings.expect("clean standings")), 0)
}

fn command_closecall(file: &PText) -> Outcome {
    let race = match load_race_or_print_defects(file) {
        Ok(race) => race,
        Err(outcome) => return outcome,
    };
    Outcome::plain(close_call_report(&race), 0)
}

fn command_series(file: &PText) -> Outcome {
    let (series, defects) = load_series_file(&path_from_arg(file));
    if !defects.is_empty() {
        return print_defects("invalid series file", defects);
    }
    Outcome::plain(series_report(&series.expect("a clean series")), 0)
}

fn command_championship(file: &PText) -> Outcome {
    let (championship, defects) = load_championship_file(&path_from_arg(file));
    if !defects.is_empty() {
        return print_defects("invalid championship file", defects);
    }
    Outcome::plain(
        championship_report(&championship.expect("a clean championship")),
        0,
    )
}

fn command_flip(file: &PText) -> Outcome {
    let (series, defects) = load_series_file(&path_from_arg(file));
    if !defects.is_empty() {
        return print_defects("invalid series file", defects);
    }
    Outcome::plain(flip_report(&series.expect("a clean series")), 0)
}

fn command_totals(file: &PText) -> Outcome {
    let (championship, defects) = load_championship_file(&path_from_arg(file));
    if !defects.is_empty() {
        return print_defects("invalid championship file", defects);
    }
    Outcome::plain(
        totals_report(&championship.expect("a clean championship")),
        0,
    )
}

fn command_pairing(file: &PText) -> Outcome {
    let (championship, defects) = load_championship_file(&path_from_arg(file));
    if !defects.is_empty() {
        return print_defects("invalid championship file", defects);
    }
    Outcome::plain(
        pairing_report(&championship.expect("a clean championship")),
        0,
    )
}

fn command_comeback(file: &PText) -> Outcome {
    let (championship, defects) = load_championship_file(&path_from_arg(file));
    if !defects.is_empty() {
        return print_defects("invalid championship file", defects);
    }
    Outcome::plain(
        comeback_report(&championship.expect("a clean championship")),
        0,
    )
}

fn command_distance(file: &PText) -> Outcome {
    let (championship, defects) = load_championship_file(&path_from_arg(file));
    if !defects.is_empty() {
        return print_defects("invalid championship file", defects);
    }
    Outcome::plain(
        distance_report(&championship.expect("a clean championship")),
        0,
    )
}

fn command_run(file: &PText) -> Outcome {
    let (spec, defects) = crate::specs::load_spec_file(&path_from_arg(file));
    if !defects.is_empty() {
        return print_defects("invalid race specification file", defects);
    }
    let spec = spec.expect("a clean specification");
    let workspace = PathBuf::from(spec.workspace.display());
    let race = match crate::runner::run_race(&workspace, &spec.task, &spec.attempts) {
        Ok(race) => race,
        Err(error) => return Outcome::plain(vec![error], 1),
    };
    let record_value = race_to_json(&race);
    if crate::json::has_int_past_digit_limit(&record_value) {
        return Outcome::plain(Vec::new(), 1);
    }
    let record_path = PathBuf::from(spec.record.display());
    if let Some(parent) = record_path.parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }
    let record_text = format!("{}\n", crate::json::dumps(&record_value, Some(1), true));
    if std::fs::write(&record_path, record_text).is_err() {
        return Outcome::plain(Vec::new(), 1);
    }
    let mut lines = race_report(&race);
    lines.extend(funeral(&race));
    Outcome::plain(lines, 0)
}
