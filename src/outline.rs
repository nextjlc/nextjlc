/* src/outline.rs */

/* SPDX-License-Identifier: MIT */
/*
 * Author Canmi <t@canmi.icu>
 */

// Define the priority order for KiCad names.
const KICAD_NAMES: &[&str] = &["Edge_Cuts", "F_Cu", "F_Mask"];

// Define the priority order for Gerber extensions.
const GERBER_EXTENSIONS: &[&str] = &["gto", "gtl", "gbl"];

// This function sorts a list of file paths based on predefined Gerber file patterns.
// It prioritizes files with KiCad specific names, then standard Gerber extensions.
//
// # Arguments
//
// * `files` - A mutable slice of `String`s, where each string is a file path.
//
// # Returns
//
// A new `Vec<String>` containing the sorted file paths.
pub fn sort_gerber_files(files: &mut [String]) -> Vec<String> {
    files.sort_by(|a, b| {
        // Determine the priority of file 'a'.
        let a_priority = get_file_priority(a);
        // Determine the priority of file 'b'.
        let b_priority = get_file_priority(b);
        // Compare the priorities. Lower numbers have higher priority.
        a_priority.cmp(&b_priority)
    });
    // Return a new Vec containing the sorted file paths.
    files.to_vec()
}

// This helper function determines the priority of a file based on its name.
//
// # Arguments
//
// * `file_path` - A reference to a string slice (`&str`) representing the file path.
//
// # Returns
//
// An `isize` value representing the priority. Lower values indicate higher priority.
fn get_file_priority(file_path: &str) -> isize {
    // First, check for KiCad specific names in the file path.
    for (index, name) in KICAD_NAMES.iter().enumerate() {
        if file_path.contains(name) {
            // Return a high priority (low number) if a KiCad name is found.
            return index as isize;
        }
    }

    // If no KiCad name is found, check for standard Gerber file extensions.
    for (index, ext) in GERBER_EXTENSIONS.iter().enumerate() {
        if file_path.to_lowercase().ends_with(&format!(".{}", ext)) {
            // Return a medium priority if a Gerber extension is found.
            // The offset by KICAD_NAMES.len() ensures these are lower priority than KiCad files.
            return (index + KICAD_NAMES.len()) as isize;
        }
    }

    // If no specific patterns are matched, assign the lowest priority.
    isize::MAX
}
