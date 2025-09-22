/* src/dcode.rs */

/* SPDX-License-Identifier: Apache-2.0 */
/*
 * Author HalfSweet <halfsweet@halfsweet.cn>
 * Author Canmi <t@canmi.icu>
 */

use once_cell::sync::Lazy;
use regex::Regex;

// This regex matches any standard D-code (Dxx*).
static DCODE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(D\d{2,4}\*)").expect("Failed to compile D-Code regex"));

/// Gerber flavor for D-code processing
pub enum GerberFlavor {
    KiCad,
    Altium,
}

/// Process Gerber data to prepend "G54" to D-codes according to the CAD flavor.
///
/// KiCad: process all non-%ADD, non-G54D D-codes.
/// Altium: skip D-codes on lines starting with G01/G02/G36/G37 with coordinates
/// or lines that only contain a single Dxx*; other D-codes (including %ADD/%AM/G04) are processed.
///
/// # Arguments
/// * `gerber_data` - Raw Gerber file content
/// * `flavor` - KiCad or Altium
///
/// # Returns
/// Processed Gerber content with appropriate D-codes prefixed with "G54"
pub fn process_d_codes(gerber_data: String, flavor: GerberFlavor) -> String {
    let input_lines: Vec<&str> = gerber_data.split('\n').collect();
    let mut processed_lines = Vec::with_capacity(input_lines.len());

    for line in input_lines {
        let mut should_skip = false;

        match flavor {
            GerberFlavor::KiCad => {
                if line.contains("%ADD") || line.contains("G54D") {
                    should_skip = true;
                }
            }
            GerberFlavor::Altium => {
                // Skip if line already contains G54D
                if line.contains("G54D") {
                    should_skip = true;
                }

                // Skip lines starting with G01/G02/G36/G37 with coordinates or single Dxx*
                if line.starts_with("G01")
                    || line.starts_with("G02")
                    || line.starts_with("G36")
                    || line.starts_with("G37")
                {
                    let trimmed = line.trim();
                    // Skip lines that are only Dxx* or contain coordinates
                    if trimmed == "D01*"
                        || trimmed == "D02*"
                        || trimmed == "D03*"
                        || trimmed.contains("X")
                        || trimmed.contains("Y")
                    {
                        should_skip = true;
                    }
                }
            }
        }

        if should_skip {
            processed_lines.push(line.to_string());
        } else {
            let modified_line = DCODE_REGEX.replace_all(line, "G54$1");
            processed_lines.push(modified_line.to_string());
        }
    }

    processed_lines.join("\n")
}
