/* src/drill.rs */

/* SPDX-License-Identifier: MIT */
/*
 * Author Canmi <t@canmi.icu>
 */

use crate::header::get_drill_header;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::BTreeMap;

/// Hole plating type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoleType {
    Plated,    // PTH - Plated Through Hole
    NonPlated, // NPTH - Non-Plated Through Hole
}

/// Unit type for drill files
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrillUnit {
    Inch,
    Metric,
}

/// Drill command types - coordinates stored in mm
#[derive(Debug, Clone)]
pub enum DrillCommand {
    /// Simple hole at (x, y) - coordinates in mm
    Hole { x: f64, y: f64 },
    /// Routed slot: start point, then route to end point
    /// Will be converted to G85 format for JLC
    /// Coordinates in mm
    Slot {
        start_x: f64,
        start_y: f64,
        end_x: f64,
        end_y: f64,
    },
}

/// A tool definition with its associated drill commands
#[derive(Debug, Clone)]
pub struct DrillOperation {
    pub diameter: f64,       // Tool diameter in mm
    pub hole_type: HoleType, // PTH or NPTH
    pub commands: Vec<DrillCommand>,
}

/// Parsed drill file representation
#[derive(Debug, Clone)]
pub struct DrillFile {
    pub operations: Vec<DrillOperation>,
}

/// Result of processing drill files
#[derive(Debug)]
pub struct DrillResult {
    pub pth_content: Option<String>,
    pub npth_content: Option<String>,
    pub warnings: Vec<String>,
}

// Regex patterns for parsing
static AD_TOOL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^T(\d+)F\d+S\d+C([\d.]+)").expect("Invalid AD tool regex"));

static KICAD_TOOL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^T(\d+)C([\d.]+)").expect("Invalid KiCad tool regex"));

static COORD_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?:X([\d.-]+))?(?:Y([\d.-]+))?$").expect("Invalid coord regex"));

static TOOL_SELECT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^T(\d+)$").expect("Invalid tool select regex"));

static ROUTE_START_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^G00(?:X([\d.-]+))?(?:Y([\d.-]+))?").expect("Invalid route start regex")
});

static ROUTE_TO_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^G01(?:X([\d.-]+))?(?:Y([\d.-]+))?").expect("Invalid route to regex")
});

static FILE_FORMAT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"FILE_FORMAT=(\d+):(\d+)").expect("Invalid file format regex"));

const INCH_TO_MM: f64 = 25.4;

/// Detect if a file is a drill file based on filename
pub fn is_drill_file(filename: &str) -> bool {
    let lower = filename.to_lowercase();
    lower.ends_with(".drl")
        || lower.ends_with(".txt") && (lower.contains("hole") || lower.contains("drill"))
        || lower.ends_with(".tx1")
        || lower.ends_with(".tx2")
        || lower.ends_with(".tx3")
        || lower.ends_with(".tx4")
        || lower.ends_with(".tx5")
        || lower.ends_with(".tx6")
}

/// Detect if a drill file is a through-hole file (not blind/buried via)
pub fn is_through_drill(filename: &str) -> bool {
    let lower = filename.to_lowercase();
    // TX1, TX2, etc. are blind/buried vias
    !(lower.ends_with(".tx1")
        || lower.ends_with(".tx2")
        || lower.ends_with(".tx3")
        || lower.ends_with(".tx4")
        || lower.ends_with(".tx5")
        || lower.ends_with(".tx6"))
}

/// Detect EDA type from drill file content
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrillEdaType {
    Altium,
    KiCad,
    Unknown,
}

pub fn detect_drill_eda(content: &str) -> DrillEdaType {
    let lower = content.to_lowercase();
    if lower.contains("kicad") {
        DrillEdaType::KiCad
    } else if AD_TOOL_REGEX.is_match(content) {
        // AD uses T01F00S00C format
        DrillEdaType::Altium
    } else if lower.contains("altium") {
        DrillEdaType::Altium
    } else {
        DrillEdaType::Unknown
    }
}

/// Parse coordinate string according to FILE_FORMAT and convert to mm
fn parse_ad_coordinate(
    coord: &str,
    integer_places: u32,
    decimal_places: u32,
    is_lz: bool,
    unit: DrillUnit,
) -> f64 {
    // If already contains decimal point, parse directly
    if coord.contains('.') {
        let val: f64 = coord.parse().unwrap_or(0.0);
        return if unit == DrillUnit::Inch {
            val * INCH_TO_MM
        } else {
            val
        };
    }

    let sign = if coord.starts_with('-') { -1.0 } else { 1.0 };
    let abs_coord = coord.trim_start_matches(|c| c == '+' || c == '-');

    let val = if is_lz {
        // LZ (Leading Zero) mode: Integer part has fixed length
        // In Excellon, LZ usually means leading zeros are kept, trailing zeros suppressed
        let split_pos = integer_places as usize;
        if abs_coord.len() <= split_pos {
            abs_coord.parse::<f64>().unwrap_or(0.0)
        } else {
            let int_part = &abs_coord[0..split_pos];
            let dec_part = &abs_coord[split_pos..];
            let int_val: f64 = int_part.parse().unwrap_or(0.0);
            let dec_val: f64 = if !dec_part.is_empty() {
                dec_part.parse::<f64>().unwrap_or(0.0) / 10_f64.powi(dec_part.len() as i32)
            } else {
                0.0
            };
            int_val + dec_val
        }
    } else {
        // TZ (Trailing Zero) mode: Decimal part has fixed length
        if let Ok(v) = abs_coord.parse::<i64>() {
            v as f64 / 10_f64.powi(decimal_places as i32)
        } else {
            0.0
        }
    };

    let result = sign * val;
    if unit == DrillUnit::Inch {
        result * INCH_TO_MM
    } else {
        result
    }
}

/// Parse an Altium Designer Excellon drill file
pub fn parse_ad_excellon(content: &str) -> DrillFile {
    let mut operations: Vec<DrillOperation> = Vec::new();
    let mut tool_map: BTreeMap<u32, (f64, HoleType)> = BTreeMap::new();
    let mut current_hole_type = HoleType::Plated;
    let mut in_header = true;

    // Default values
    let mut unit = DrillUnit::Metric;
    let mut integer_places: u32 = 2;
    let mut decimal_places: u32 = 4; // Default FILE_FORMAT assumption
    let mut is_lz = true; // AD default is LZ

    // First pass: parse header to get unit, format, and tool definitions
    for line in content.lines() {
        let line = line.trim();

        if line == "%" {
            in_header = false;
            continue;
        }

        if in_header {
            // Check for unit
            let upper = line.to_uppercase();
            if upper.starts_with("INCH") {
                unit = DrillUnit::Inch;
                if upper.contains("LZ") {
                    is_lz = true;
                } else if upper.contains("TZ") {
                    is_lz = false;
                }
            } else if upper.starts_with("METRIC") {
                unit = DrillUnit::Metric;
                if upper.contains("LZ") {
                    is_lz = true;
                } else if upper.contains("TZ") {
                    is_lz = false;
                }
            }

            // Check for FILE_FORMAT
            if let Some(caps) = FILE_FORMAT_REGEX.captures(line) {
                // FILE_FORMAT=2:5 means 2 integer digits, 5 decimal digits
                integer_places = caps[1].parse().unwrap_or(2);
                decimal_places = caps[2].parse().unwrap_or(4);
            }

            // Check for TYPE markers
            if line.contains("TYPE=PLATED") && !line.contains("NON_PLATED") {
                current_hole_type = HoleType::Plated;
            } else if line.contains("TYPE=NON_PLATED") {
                current_hole_type = HoleType::NonPlated;
            }

            // Parse tool definition
            if let Some(caps) = AD_TOOL_REGEX.captures(line) {
                let tool_num: u32 = caps[1].parse().unwrap_or(0);
                let diameter_raw: f64 = caps[2].parse().unwrap_or(0.0);
                // Convert diameter to mm if needed
                let diameter_mm = if unit == DrillUnit::Inch {
                    diameter_raw * INCH_TO_MM
                } else {
                    diameter_raw
                };
                tool_map.insert(tool_num, (diameter_mm, current_hole_type));
            }
        }
    }

    // Initialize operations for each tool
    let mut tool_operations: BTreeMap<u32, DrillOperation> = BTreeMap::new();
    for (tool_num, (diameter, hole_type)) in &tool_map {
        tool_operations.insert(
            *tool_num,
            DrillOperation {
                diameter: *diameter,
                hole_type: *hole_type,
                commands: Vec::new(),
            },
        );
    }

    // Second pass: parse drill commands
    let mut current_tool: Option<u32> = None;
    let mut in_route = false;
    let mut last_x: f64 = 0.0;
    let mut last_y: f64 = 0.0;
    in_header = true;

    for line in content.lines() {
        let line = line.trim();

        if line == "%" {
            in_header = false;
            continue;
        }

        if in_header || line.is_empty() || line.starts_with(';') {
            continue;
        }

        // Tool selection
        if let Some(caps) = TOOL_SELECT_REGEX.captures(line) {
            let tool_num: u32 = caps[1].parse().unwrap_or(0);
            if tool_map.contains_key(&tool_num) {
                current_tool = Some(tool_num);
            }
            continue;
        }

        // Route start (G00)
        if let Some(caps) = ROUTE_START_REGEX.captures(line) {
            if let Some(x_match) = caps.get(1) {
                last_x = parse_ad_coordinate(
                    x_match.as_str(),
                    integer_places,
                    decimal_places,
                    is_lz,
                    unit,
                );
            }
            if let Some(y_match) = caps.get(2) {
                last_y = parse_ad_coordinate(
                    y_match.as_str(),
                    integer_places,
                    decimal_places,
                    is_lz,
                    unit,
                );
            }
            continue;
        }

        // M15 - start routing (drill down)
        if line == "M15" {
            in_route = true;
            continue;
        }

        // Route to (G01) - during routing
        if in_route {
            if let Some(caps) = ROUTE_TO_REGEX.captures(line) {
                let start_x = last_x;
                let start_y = last_y;

                if let Some(x_match) = caps.get(1) {
                    last_x = parse_ad_coordinate(
                        x_match.as_str(),
                        integer_places,
                        decimal_places,
                        is_lz,
                        unit,
                    );
                }
                if let Some(y_match) = caps.get(2) {
                    last_y = parse_ad_coordinate(
                        y_match.as_str(),
                        integer_places,
                        decimal_places,
                        is_lz,
                        unit,
                    );
                }

                if let Some(tool) = current_tool {
                    if let Some(op) = tool_operations.get_mut(&tool) {
                        op.commands.push(DrillCommand::Slot {
                            start_x,
                            start_y,
                            end_x: last_x,
                            end_y: last_y,
                        });
                    }
                }
                continue;
            }
        }

        // M16 - end routing (drill up)
        if line == "M16" {
            in_route = false;
            continue;
        }

        // Simple hole coordinate (X...Y...)
        if let Some(caps) = COORD_REGEX.captures(line) {
            if caps.get(1).is_none() && caps.get(2).is_none() {
                continue;
            }
            if let Some(tool) = current_tool {
                if let Some(op) = tool_operations.get_mut(&tool) {
                    if let Some(x_match) = caps.get(1) {
                        last_x = parse_ad_coordinate(
                            x_match.as_str(),
                            integer_places,
                            decimal_places,
                            is_lz,
                            unit,
                        );
                    }
                    if let Some(y_match) = caps.get(2) {
                        last_y = parse_ad_coordinate(
                            y_match.as_str(),
                            integer_places,
                            decimal_places,
                            is_lz,
                            unit,
                        );
                    }
                    op.commands.push(DrillCommand::Hole {
                        x: last_x,
                        y: last_y,
                    });
                }
            }
        }
    }

    // Collect non-empty operations
    for (_, op) in tool_operations {
        if !op.commands.is_empty() {
            operations.push(op);
        }
    }

    DrillFile { operations }
}

/// Parse a KiCad Excellon drill file
/// KiCad uses METRIC and decimal coordinates by default
pub fn parse_kicad_excellon(content: &str) -> (DrillFile, HoleType) {
    let mut operations: Vec<DrillOperation> = Vec::new();
    let mut tool_map: BTreeMap<u32, f64> = BTreeMap::new();

    // Determine hole type from file function comment
    let hole_type = if content.contains("NonPlated") || content.contains("NPTH") {
        HoleType::NonPlated
    } else {
        HoleType::Plated
    };

    let mut in_header = true;

    // First pass: parse header
    for line in content.lines() {
        let line = line.trim();

        if line == "%" {
            in_header = false;
            continue;
        }

        if in_header {
            if let Some(caps) = KICAD_TOOL_REGEX.captures(line) {
                let tool_num: u32 = caps[1].parse().unwrap_or(0);
                let diameter: f64 = caps[2].parse().unwrap_or(0.0);
                tool_map.insert(tool_num, diameter);
            }
        }
    }

    // Initialize operations
    let mut tool_operations: BTreeMap<u32, DrillOperation> = BTreeMap::new();
    for (tool_num, diameter) in &tool_map {
        tool_operations.insert(
            *tool_num,
            DrillOperation {
                diameter: *diameter,
                hole_type,
                commands: Vec::new(),
            },
        );
    }

    // Second pass: parse commands
    let mut current_tool: Option<u32> = None;
    let mut in_route = false;
    let mut route_start: Option<(f64, f64)> = None;
    let mut last_y: f64 = 0.0;
    in_header = true;

    for line in content.lines() {
        let line = line.trim();

        if line == "%" {
            in_header = false;
            continue;
        }

        if in_header || line.is_empty() || line.starts_with(';') {
            continue;
        }

        // Tool selection
        if let Some(caps) = TOOL_SELECT_REGEX.captures(line) {
            let tool_num: u32 = caps[1].parse().unwrap_or(0);
            if tool_map.contains_key(&tool_num) {
                current_tool = Some(tool_num);
            }
            continue;
        }

        // Route start
        if let Some(caps) = ROUTE_START_REGEX.captures(line) {
            let x: f64 = caps[1].parse().unwrap_or(0.0);
            let y: f64 = caps[2].parse().unwrap_or(0.0);
            route_start = Some((x, y));
            last_y = y;
            continue;
        }

        if line == "M15" {
            in_route = true;
            continue;
        }

        if in_route {
            if let Some(caps) = ROUTE_TO_REGEX.captures(line) {
                let end_x: f64 = caps[1].parse().unwrap_or(0.0);
                let end_y: f64 = caps
                    .get(2)
                    .map_or(last_y, |m| m.as_str().parse().unwrap_or(last_y));

                if let (Some(tool), Some((start_x, start_y))) = (current_tool, route_start) {
                    if let Some(op) = tool_operations.get_mut(&tool) {
                        op.commands.push(DrillCommand::Slot {
                            start_x,
                            start_y,
                            end_x,
                            end_y,
                        });
                    }
                }
                last_y = end_y;
                continue;
            }
        }

        if line == "M16" {
            in_route = false;
            route_start = None;
            continue;
        }

        // KiCad uses decimal coordinates in mm
        if line.starts_with('X') && line.contains('Y') {
            let coord_re = Regex::new(r"^X([\d.-]+)Y([\d.-]+)").unwrap();
            if let Some(caps) = coord_re.captures(line) {
                if let Some(tool) = current_tool {
                    if let Some(op) = tool_operations.get_mut(&tool) {
                        let x: f64 = caps[1].parse().unwrap_or(0.0);
                        let y: f64 = caps[2].parse().unwrap_or(0.0);
                        op.commands.push(DrillCommand::Hole { x, y });
                    }
                }
            }
        }
    }

    for (_, op) in tool_operations {
        if !op.commands.is_empty() {
            operations.push(op);
        }
    }

    (DrillFile { operations }, hole_type)
}

/// Merge multiple drill files and split by hole type
pub fn merge_and_split_drills(files: Vec<DrillFile>) -> (Option<DrillFile>, Option<DrillFile>) {
    let mut pth_ops: Vec<DrillOperation> = Vec::new();
    let mut npth_ops: Vec<DrillOperation> = Vec::new();

    for file in files {
        for op in file.operations {
            match op.hole_type {
                HoleType::Plated => pth_ops.push(op),
                HoleType::NonPlated => npth_ops.push(op),
            }
        }
    }

    // Merge operations with same diameter
    let pth_merged = merge_operations_by_diameter(pth_ops);
    let npth_merged = merge_operations_by_diameter(npth_ops);

    let pth_file = if pth_merged.is_empty() {
        None
    } else {
        Some(DrillFile {
            operations: pth_merged,
        })
    };

    let npth_file = if npth_merged.is_empty() {
        None
    } else {
        Some(DrillFile {
            operations: npth_merged,
        })
    };

    (pth_file, npth_file)
}

/// Merge operations that have the same diameter
fn merge_operations_by_diameter(ops: Vec<DrillOperation>) -> Vec<DrillOperation> {
    let mut diameter_map: BTreeMap<u64, DrillOperation> = BTreeMap::new();

    for op in ops {
        // Use diameter * 100000 as key to handle floating point comparison
        let key = (op.diameter * 100000.0) as u64;

        if let Some(existing) = diameter_map.get_mut(&key) {
            existing.commands.extend(op.commands);
        } else {
            diameter_map.insert(key, op);
        }
    }

    diameter_map.into_values().collect()
}

/// Generate JLC format Excellon content
/// All coordinates are already in mm
pub fn generate_jlc_excellon(drill: &DrillFile, hole_type: HoleType) -> String {
    let mut output = String::new();

    // Add header
    let (type_str, layer_name) = match hole_type {
        HoleType::Plated => ("PLATED", "PTH_Through"),
        HoleType::NonPlated => ("NON_PLATED", "NPTH_Through"),
    };
    output.push_str(&get_drill_header(type_str, layer_name));

    // File header
    output.push_str("M48\n");
    output.push_str("METRIC,LZ,0000.00000\n");

    // Tool definitions
    for (i, op) in drill.operations.iter().enumerate() {
        let tool_num = i + 1;
        output.push_str(&format!(
            ";Hole size {} = {:.5} METRIC\n",
            tool_num, op.diameter
        ));
        output.push_str(&format!("T{:02}C{:.5}\n", tool_num, op.diameter));
    }

    output.push_str("%\n");
    output.push_str("G05\n");
    output.push_str("G90\n");

    // Drill commands - coordinates are already in mm
    for (i, op) in drill.operations.iter().enumerate() {
        let tool_num = i + 1;
        output.push_str(&format!("T{:02}\n", tool_num));

        for cmd in &op.commands {
            match cmd {
                DrillCommand::Hole { x, y } => {
                    output.push_str(&format!("X{:.5}Y{:.5}\n", x, y));
                }
                DrillCommand::Slot {
                    start_x,
                    start_y,
                    end_x,
                    end_y,
                } => {
                    // G85 slot format
                    output.push_str(&format!(
                        "X{:.5}Y{:.5}G85X{:.5}Y{:.5}\n",
                        start_x, start_y, end_x, end_y
                    ));
                }
            }
        }
    }

    output.push_str("M30\n");
    output
}

/// Main entry point: process multiple drill files and return PTH/NPTH content
pub fn process_drill_files(contents: &[String], filenames: &[String]) -> DrillResult {
    let mut all_files: Vec<DrillFile> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();
    let mut has_kicad_pth = false;
    let mut has_kicad_npth = false;
    let mut kicad_pth_content: Option<String> = None;
    let mut kicad_npth_content: Option<String> = None;

    for (content, filename) in contents.iter().zip(filenames.iter()) {
        // Check for blind/buried vias
        if !is_through_drill(filename) {
            warnings.push(format!(
                "Skipped blind/buried via file: {}. JLC only supports through holes.",
                filename
            ));
            continue;
        }

        let eda_type = detect_drill_eda(content);

        match eda_type {
            DrillEdaType::KiCad => {
                // KiCad already separates PTH and NPTH
                let (drill_file, hole_type) = parse_kicad_excellon(content);

                match hole_type {
                    HoleType::Plated => {
                        if !has_kicad_pth {
                            kicad_pth_content =
                                Some(generate_jlc_excellon(&drill_file, HoleType::Plated));
                            has_kicad_pth = true;
                        } else {
                            // Multiple PTH files - merge
                            all_files.push(drill_file);
                        }
                    }
                    HoleType::NonPlated => {
                        if !has_kicad_npth {
                            kicad_npth_content =
                                Some(generate_jlc_excellon(&drill_file, HoleType::NonPlated));
                            has_kicad_npth = true;
                        } else {
                            all_files.push(drill_file);
                        }
                    }
                }
            }
            DrillEdaType::Altium | DrillEdaType::Unknown => {
                let drill_file = parse_ad_excellon(content);
                all_files.push(drill_file);
            }
        }
    }

    // If we have AD files to merge
    if !all_files.is_empty() {
        let (pth_file, npth_file) = merge_and_split_drills(all_files);

        let pth_content = pth_file.map(|f| generate_jlc_excellon(&f, HoleType::Plated));
        let npth_content = npth_file.map(|f| generate_jlc_excellon(&f, HoleType::NonPlated));

        // Merge with any KiCad files
        let final_pth = pth_content.or(kicad_pth_content);
        let final_npth = npth_content.or(kicad_npth_content);

        DrillResult {
            pth_content: final_pth,
            npth_content: final_npth,
            warnings,
        }
    } else {
        // Only KiCad files
        DrillResult {
            pth_content: kicad_pth_content,
            npth_content: kicad_npth_content,
            warnings,
        }
    }
}
