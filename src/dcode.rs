/* src/dcode.rs */

/* SPDX-License-Identifier: Apache-2.0 */
/*
 * Author HalfSweet <halfsweet@halfsweet.cn>
 */

use once_cell::sync::Lazy;
use regex::Regex;

/// Statically compile the regex for performance. It will be created only once.
/// This regex looks for D-codes (D followed by 2-4 digits and a `*`) that are not
/// preceded by "G54". Using a negative lookbehind `(?<!...)` is efficient here.
/// Note: For this specific regex, the standard `regex` crate works perfectly.
/// For more complex look-arounds, `fancy-regex` would be needed.
static APERTURE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<!G54)(D\d{2,4}\*)").unwrap());

/// This function processes a string of Gerber data to prepend "G54"
/// to specific D-codes, specifically for KiCad compatibility.
///
/// It iterates through each line and uses a regular expression to find all
/// D-codes (e.g., D10*, D123*) that are not already prefixed with "G54".
/// For each valid D-code found, it prepends "G54".
///
/// The function skips any lines that contain aperture macro definitions ("%ADD")
/// to avoid incorrect modifications.
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
        // Skip lines that are macro definitions to prevent accidental replacement.
        if line.contains("%ADD") {
            processed_lines.push(line.to_string());
            continue;
        }

        // Use `replace_all` to find all occurrences on the line that match the
        // regex and prepend "G54" to them. The "$1" refers to the first capture group,
        // which is the D-code itself (e.g., "D10*").
        let modified_line = APERTURE_REGEX.replace_all(line, "G54$1");
        processed_lines.push(modified_line.to_string());
    }

    // Join all processed lines back into a single string.
    processed_lines.join("\n")
}
