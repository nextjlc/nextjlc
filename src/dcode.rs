/* src/dcode.rs */

/* SPDX-License-Identifier: Apache-2.0 */
/*
 * Author HalfSweet <halfsweet@halfsweet.cn>
 */

use once_cell::sync::Lazy;
use regex::Regex;

// This regex is simple and performant, matching any standard D-code.
// It is used conditionally to avoid modifying lines that shouldn't be touched.
static DCODE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(D\d{2,4}\*)").expect("Failed to compile D-Code regex"));

/// This function processes a string of Gerber data to prepend "G54"
/// to specific D-codes, primarily for KiCad file compatibility.
///
/// It follows a safe and performant two-step process for each line:
/// 1. It first checks for "%ADD" or "G54D" to quickly skip lines that
///    should not be modified.
/// 2. On all other lines, it uses a simple regex to find and replace
///    all applicable D-codes.
///
/// This approach avoids complex regex features and ensures both correctness
/// and speed, making it suitable for a WASM environment.
///
/// # Arguments
///
/// * `gerber_data` - A `String` containing the raw Gerber file content.
///
/// # Returns
///
/// A new `String` with the processed Gerber data where appropriate D-codes
/// have been prefixed with "G54".
pub fn process_d_codes(gerber_data: String) -> String {
    let input_lines: Vec<&str> = gerber_data.split('\n').collect();
    let mut processed_lines = Vec::with_capacity(input_lines.len());

    for line in input_lines {
        // This fast check ensures we only process relevant lines.
        if line.contains("%ADD") || line.contains("G54D") {
            processed_lines.push(line.to_string());
        } else {
            // The simple regex is only run on lines that are safe to modify.
            // `$1` refers to the captured D-code (e.g., "D10*").
            let modified_line = DCODE_REGEX.replace_all(line, "G54$1");
            processed_lines.push(modified_line.to_string());
        }
    }

    processed_lines.join("\n")
}
