/* src/fingerprint.rs */

/* SPDX-License-Identifier: Apache-2.0 */
/*
 * Author HalfSweet <halfsweet@halfsweet.cn>
 */

use md5::{Digest, Md5};
use rand::Rng;
use regex::Regex;

/// This is the main public function that orchestrates the entire fingerprinting process.
/// It takes raw Gerber content and a boolean flag, and returns the content with an
/// embedded, unique fingerprint based on the file's own data.
///
/// The process involves:
/// 1. Scanning for existing aperture definitions.
/// 2. Selecting a random definition as a template.
/// 3. Renumbering all subsequent apertures to create a space.
/// 4. Generating a unique dimension based on an MD5 hash of the content.
/// 5. Creating and inserting a new aperture definition line (the "fingerprint").
///
/// If the file has too few aperture definitions (less than 6), it cannot be fingerprinted
/// and the original content is returned unchanged.
///
/// # Arguments
///
/// * `gerber_content` - A string slice (`&str`) of the entire Gerber file.
/// * `is_foreign_board_file` - A boolean indicating if a special prefix should be used for hashing.
///
/// # Returns
///
/// A new `String` containing the fingerprinted Gerber content.
pub fn add_fingerprint(gerber_content: &str, is_foreign_board_file: bool) -> String {
    // If the file is very large, skip fingerprinting for performance reasons.
    if gerber_content.len() > 30_000_000 {
        return gerber_content.to_string();
    }

    let content_lines: Vec<&str> = gerber_content.lines().collect();

    // Part 1a: Scan for existing definitions.
    let (existing_definitions, existing_aperture_ids) =
        scan_for_aperture_definitions(&content_lines);

    // Part 1b: Select a template for the new aperture.
    // If there aren't enough apertures, this will gracefully select a default.
    let (template_definition_line, injection_aperture_id) =
        select_injection_template(&existing_definitions, &existing_aperture_ids);

    // Part 2: Renumber apertures to make space.
    let content_with_shifted_ids = renumber_apertures(gerber_content, injection_aperture_id);

    // Part 3: Generate a unique dimension from a hash of the content.
    let final_dimension_str =
        generate_hashed_dimension(&content_with_shifted_ids, is_foreign_board_file);

    // Part 4a: Create the new aperture definition line.
    let final_fingerprint_line = create_fingerprint_aperture_line(
        template_definition_line,
        injection_aperture_id,
        &final_dimension_str,
    );

    // Part 4b: Insert the new line into the content.
    insert_new_aperture_line(&content_with_shifted_ids, &final_fingerprint_line)
}

/// Part 1a: Scan the beginning of the file to find existing aperture definitions.
fn scan_for_aperture_definitions(content_lines: &[&str]) -> (Vec<String>, Vec<u32>) {
    let mut existing_definitions = Vec::new();
    let mut existing_aperture_ids = Vec::new();
    let aperture_regex = Regex::new(r"^%ADD(\d{2,4})\D.*").unwrap();

    // Scan the top part of the file for efficiency.
    for line in content_lines.iter().take(200) {
        if let Some(caps) = aperture_regex.captures(line) {
            if let Some(num_str) = caps.get(1) {
                if let Ok(num) = num_str.as_str().parse::<u32>() {
                    existing_definitions.push(line.to_string());
                    existing_aperture_ids.push(num);
                }
            }
        }
    }
    (existing_definitions, existing_aperture_ids)
}

/// Part 1b: Choose a random existing aperture to use as a template.
fn select_injection_template(
    existing_definitions: &[String],
    existing_aperture_ids: &[u32],
) -> (Option<String>, u32) {
    if existing_definitions.is_empty() {
        return (None, 10); // Default if no apertures are found.
    }

    let mut rng = rand::rng();
    let selection_index = if existing_definitions.len() <= 5 {
        existing_definitions.len() - 1
    } else {
        // Choose a random index between 5 and the end of the list.
        rng.random_range(5..existing_definitions.len())
    };

    let template = existing_definitions.get(selection_index).cloned();
    let injection_id = existing_aperture_ids
        .get(selection_index)
        .copied()
        .unwrap_or(10);

    (template, injection_id)
}

/// Part 2: Renumber all subsequent apertures to make space for the new one.
fn renumber_apertures(content: &str, injection_aperture_id: u32) -> String {
    let re = Regex::new(r"(?m)^(%ADD|G54D)(\d{2,4})(.*)$").unwrap();
    const MAX_APERTURE_NUM: u32 = 9999;

    re.replace_all(content, |caps: &regex::Captures| {
        let prefix = &caps[1];
        let number: u32 = caps[2].parse().unwrap_or(0);
        let suffix = &caps[3];

        // Only renumber apertures at or after the injection point, and avoid overflowing the max number.
        if number >= injection_aperture_id && number < MAX_APERTURE_NUM {
            format!("{}{}{}", prefix, number + 1, suffix)
        } else {
            caps[0].to_string() // Return the original match if no change is needed.
        }
    })
    .to_string()
}

/// Part 3: Generate a "magic number" size based on a content hash.
fn generate_hashed_dimension(
    content_with_shifted_ids: &str,
    is_foreign_board_file: bool,
) -> String {
    let data_to_hash = if is_foreign_board_file {
        format!("494d{}", content_with_shifted_ids)
    } else {
        content_with_shifted_ids.to_string()
    };

    let mut md5_hasher = Md5::new();
    md5_hasher.update(data_to_hash.as_bytes());
    let digest = md5_hasher.finalize();
    let hex_digest = format!("{:x}", digest);

    // Take the last two hex characters and convert to a number between 00 and 99.
    let final_hex_chars = &hex_digest[hex_digest.len() - 2..];
    let decimal_from_hash = u32::from_str_radix(final_hex_chars, 16).unwrap_or(0) % 100;
    let hash_based_suffix = format!("{:02}", decimal_from_hash);

    let mut rng = rand::rng();
    let random_base_dimension: f64 = rng.random_range(0.0..1.0);
    let combined_dimension_str = format!("{:.2}{}", random_base_dimension, hash_based_suffix);

    // Ensure the final size is not zero.
    if combined_dimension_str.parse::<f64>().unwrap_or(0.0) == 0.0 {
        "0.0100".to_string()
    } else {
        combined_dimension_str
    }
}

/// Part 4a: Create the new aperture definition line using the template.
fn create_fingerprint_aperture_line(
    template_definition_line: Option<String>,
    injection_aperture_id: u32,
    final_dimension_str: &str,
) -> String {
    if let Some(template) = template_definition_line {
        // Use regex to replace only the first size parameter after the comma.
        let size_regex = Regex::new(r",([\d.]+)").unwrap();
        let new_definition = size_regex
            .replace(&template, |_: &regex::Captures| {
                format!(",{}", final_dimension_str)
            })
            .to_string();

        // Now, correctly replace the aperture number itself.
        let id_regex = Regex::new(r"%ADD\d{2,4}").unwrap();
        id_regex
            .replace(&new_definition, &format!("%ADD{}", injection_aperture_id))
            .to_string()
    } else {
        // If no template was available, create a default circular aperture.
        format!("%ADD{}C,{}*%", injection_aperture_id, final_dimension_str)
    }
}

/// Part 4b: Intelligently insert the new definition line into the file.
fn insert_new_aperture_line(
    content_with_shifted_ids: &str,
    final_fingerprint_line: &str,
) -> String {
    let mut lines: Vec<String> = content_with_shifted_ids.lines().map(String::from).collect();
    let mut inserted = false;
    let mut mo_section_found = false;
    let mut insert_index = None;

    for (i, line) in lines.iter().enumerate() {
        if !mo_section_found && line.starts_with("%MO") {
            mo_section_found = true;
        } else if mo_section_found && (line.starts_with("%LP") || line.starts_with('G')) {
            // Found the ideal insertion point: right before the first drawing command
            // after the unit/format section.
            insert_index = Some(i);
            break;
        }
    }

    if let Some(index) = insert_index {
        lines.insert(index, final_fingerprint_line.to_string());
        inserted = true;
    }

    if !inserted {
        // Fallback: if no suitable location is found, append it to the end.
        lines.push(final_fingerprint_line.to_string());
    }

    lines.join("\n")
}
