/* src/rename.rs */

/* SPDX-License-Identifier: MIT */
/*
 * Author Canmi <t@canmi.icu>
 */

use fancy_regex::Regex;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;

/// Defines the supported EDA
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdaType {
    Ad,    // Represents Altium Designer
    KiCad, // Represents KiCad
}

/// A struct to hold a single renaming rule.
/// It pairs a logical name (e.g., "Gerber_TopLayer") with a compiled Regex pattern.
struct Rule {
    logical_name: &'static str,
    pattern: Regex,
}

/// A helper function to create a Rule, panicking if the regex is invalid.
/// This ensures that all regex patterns are validated at compile time.
fn rule(logical_name: &'static str, pattern_str: &'static str) -> Rule {
    Rule {
        logical_name,
        pattern: Regex::new(pattern_str).expect("Invalid regex pattern"),
    }
}

/// Static list of rules for Altium Designer, initialized lazily and only once.
static AD_RULES: Lazy<Vec<Rule>> = Lazy::new(|| {
    vec![
        rule("Gerber_BoardOutlineLayer", "(?i)\\.GM(1|13)$"),
        rule("Gerber_DocumentLayer", "(?i)\\.GM$"),
        rule("Gerber_TopLayer", "(?i)\\.GTL$"),
        rule("Gerber_TopSilkscreenLayer", "(?i)\\.GTO$"),
        rule("Gerber_TopSolderMaskLayer", "(?i)\\.GTS$"),
        rule("Gerber_TopPasteMaskLayer", "(?i)\\.GTP$"),
        rule("Gerber_BottomLayer", "(?i)\\.GBL$"),
        rule("Gerber_BottomSilkscreenLayer", "(?i)\\.GBP$"), // Note: Often GBO, but following spec.
        rule("Gerber_BottomSolderMaskLayer", "(?i)\\.GBS$"),
        rule("Gerber_BottomPasteMaskLayer", "(?i)\\.GPB$"),
        rule("Gerber_InnerLayer1", "(?i)\\.G1$"),
        rule("Gerber_InnerLayer2", "(?i)\\.G2$"),
        rule("Gerber_InnerLayer3", "(?i)\\.G3$"),
        rule("Gerber_InnerLayer4", "(?i)\\.G4$"),
        rule("Gerber_InnerLayer5", "(?i)\\.G5$"),
        rule("Gerber_InnerLayer6", "(?i)\\.G6$"),
        rule("Drill_NPTH_Through", "(?i).*slot\\s?h?oles.*\\.txt$"),
        rule("Drill_PTH_Through", "(?i).*round\\s?h?oles.*\\.txt$"),
        rule("Drill_PTH_Through_Via", "(?i)\\.REP$|.*via.*\\.txt$"),
        rule("Drill_PTH_Through_GBR", "(?i)\\.GD1$"),
        rule("Drill_PTH_Through_Via_GBR", "(?i)\\.GG1$"),
    ]
});

/// Static list of rules for KiCad, initialized lazily and only once.
static KICAD_RULES: Lazy<Vec<Rule>> = Lazy::new(|| {
    vec![
        rule("Gerber_BoardOutlineLayer", "(?i).*Edge_Cuts.*"),
        rule("Gerber_DocumentLayer", "(?i).*GM.*"),
        rule("Gerber_TopLayer", "(?i).*F_Cu.*"),
        rule("Gerber_TopSilkscreenLayer", "(?i).*F_Silkscreen.*"),
        rule("Gerber_TopSolderMaskLayer", "(?i).*F_Mask.*"),
        rule("Gerber_TopPasteMaskLayer", "(?i).*F_Paste.*"),
        rule("Gerber_BottomLayer", "(?i).*B_Cu.*"),
        rule("Gerber_BottomSilkscreenLayer", "(?i).*B_Silkscreen.*"),
        rule("Gerber_BottomSolderMaskLayer", "(?i).*B_Mask.*"),
        rule("Gerber_BottomPasteMaskLayer", "(?i).*B_Paste.*"),
        rule("Gerber_InnerLayer1", "(?i).*In1_Cu.*"),
        rule("Gerber_InnerLayer2", "(?i).*In2_Cu.*"),
        rule("Gerber_InnerLayer3", "(?i).*In3_Cu.*"),
        rule("Gerber_InnerLayer4", "(?i).*In4_Cu.*"),
        rule("Gerber_InnerLayer5", "(?i).*In5_Cu.*"),
        rule("Gerber_InnerLayer6", "(?i).*In6_Cu.*"),
        // Drill rules are ordered from most to least specific.
        rule("Drill_PTH_Through_Via", "(?i)^.*\\bVIA\\b.*\\.DRL$"),
        rule("Drill_NPTH_Through", "(?i)^.*\\bNPTH\\b.*\\.DRL$"),
        rule("Drill_PTH_Through", "(?i)^(?!.*NPTH).*\\.DRL$"), // Matches .DRL if not NPTH
        rule("Drill_PTH_Through_Via_GBR", "(?i)^.*\\bVIA\\b.*\\.GBR$"),
        rule("Drill_NPTH_Through_GBR", "(?i)^.*\\bNPTH\\b.*\\.GBR$"),
        rule("Drill_PTH_Through_GBR", "(?i)^[^N]*PTH[^N]*\\.GBR$"),
        // KiCAD match -drl_map.gbr
        rule("Drill_MAP_GBR", "(?i).*?-drl_map$"),
    ]
});

/// Maps a logical file type name to its final, standardized filename.
fn get_final_filename(logical_name: &str) -> String {
    match logical_name {
        "Gerber_TopSolderMaskLayer" => "Gerber_TopSolderMaskLayer.GTS".to_string(),
        "Gerber_TopSilkscreenLayer" => "Gerber_TopSilkscreenLayer.GTO".to_string(),
        "Gerber_TopPasteMaskLayer" => "Gerber_TopPasteMaskLayer.GTP".to_string(),
        "Gerber_TopLayer" => "Gerber_TopLayer.GTL".to_string(),
        "Gerber_InnerLayer1" => "Gerber_InnerLayer1.G1".to_string(),
        "Gerber_InnerLayer2" => "Gerber_InnerLayer2.G2".to_string(),
        "Gerber_InnerLayer3" => "Gerber_InnerLayer3.G3".to_string(),
        "Gerber_InnerLayer4" => "Gerber_InnerLayer4.G4".to_string(),
        "Gerber_InnerLayer5" => "Gerber_InnerLayer5.G5".to_string(),
        "Gerber_InnerLayer6" => "Gerber_InnerLayer6.G6".to_string(),
        "Gerber_BottomSolderMaskLayer" => "Gerber_BottomSolderMaskLayer.GBS".to_string(),
        "Gerber_BottomSilkscreenLayer" => "Gerber_BottomSilkscreenLayer.GBO".to_string(),
        "Gerber_BottomPasteMaskLayer" => "Gerber_BottomPasteMaskLayer.GBP".to_string(),
        "Gerber_BottomLayer" => "Gerber_BottomLayer.GBL".to_string(),
        "Gerber_BoardOutlineLayer" => "Gerber_BoardOutlineLayer.GKO".to_string(),
        "Drill_PTH_Through" => "Drill_PTH_Through.DRL".to_string(),
        "Drill_PTH_Through_Via" => "Drill_PTH_Through_Via.DRL".to_string(),
        "Drill_NPTH_Through" => "Drill_NPTH_Through.DRL".to_string(),
        // A fallback for any other matched logical names not in the primary list.
        _ => format!("{}.gbr", logical_name),
    }
}

/// The main function of this module. It takes a list of filenames and an EDA type,
/// and returns a map of original filenames to their proposed new, standardized names.
pub fn map_filenames(files: &[String], eda_type: EdaType) -> BTreeMap<String, String> {
    let rules = match eda_type {
        EdaType::Ad => &AD_RULES,
        EdaType::KiCad => &KICAD_RULES,
    };

    let mut rename_map = BTreeMap::new();

    for file in files {
        let mut new_name = file.clone(); // Default to original name if no match is found.
        let mut matched = false;

        for rule in rules.iter() {
            // Handle the Result from is_match
            if let Ok(true) = rule.pattern.is_match(file) {
                new_name = get_final_filename(rule.logical_name);
                matched = true;
                break; // Stop after the first successful match.
            }
        }
        if !matched {
            new_name = file.clone();
        }

        rename_map.insert(file.clone(), new_name);
    }

    rename_map
}
