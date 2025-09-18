/* src/file_type.rs */

/* SPDX-License-Identifier: MIT */
/*
 * Author Canmi <t@canmi.icu>
 */

// This function attempts to identify the CAD software that generated a file
// based on the presence of specific keywords in its content.
//
// The matching is case-insensitive and checks for keywords in the following order:
// 1. "altium"
// 2. "kicad"
// 3. "easyeda"
// It returns the first match it finds.
//
// # Arguments
//
// * `content` - A string slice (`&str`) representing the content of the file.
//
// # Returns
//
// An `Option<&'static str>`:
// - `Some("Altium")` if "altium" is found.
// - `Some("KiCad")` if "kicad" is found.
// - `Some("EasyEDA")` if "easyeda" is found.
// - `None` if none of the keywords are found.
pub fn identify_software(content: &str) -> Option<&'static str> {
    // Convert the entire content to lowercase for case-insensitive matching.
    let lowercased_content = content.to_lowercase();

    // Check for keywords in a specific order of priority.
    if lowercased_content.contains("altium") {
        return Some("Altium");
    }

    if lowercased_content.contains("kicad") {
        return Some("KiCad");
    }

    if lowercased_content.contains("easyeda") {
        return Some("EasyEDA");
    }

    // If no keywords are matched, return None.
    None
}
