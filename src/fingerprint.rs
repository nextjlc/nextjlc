/* src/fingerprint.rs */

/* SPDX-License-Identifier: Apache-2.0 */
/*
 * Author HalfSweet <halfsweet@halfsweet.cn>
 */

use md5::{Digest, Md5};
use rand::Rng;

// This is the main public function that orchestrates the entire fingerprinting process.
// It takes raw Gerber content and a boolean flag, and returns the content with an
// embedded, unique fingerprint based on the file's own data.
//
// The process involves:
// 1. Scanning for existing aperture definitions.
// 2. Selecting a random definition as a template.
// 3. Renumbering all subsequent apertures to create a space.
// 4. Generating a unique dimension based on an MD5 hash of the content.
// 5. Creating and inserting a new aperture definition line (the "fingerprint").
//
// If the file has too few aperture definitions (less than 6), it cannot be fingerprinted
// and the original content is returned unchanged.
//
// # Arguments
//
// * `gerber_content` - A string slice (`&str`) of the entire Gerber file.
// * `is_foreign_board_file` - A boolean indicating if a special prefix should be used for hashing.
//
// # Returns
//
// A new `String` containing the fingerprinted Gerber content.
pub fn add_fingerprint(gerber_content: &str, is_foreign_board_file: bool) -> String {
    // Collect lines for analysis.
    let content_lines: Vec<&str> = gerber_content.lines().collect();

    // Part 1a: Scan for existing definitions.
    let (existing_definitions, existing_aperture_ids) =
        scan_for_aperture_definitions(&content_lines);

    // The logic requires at least 6 definitions to safely pick a random one beyond the first 5.
    if existing_definitions.len() <= 5 {
        return gerber_content.to_string(); // Not enough data, return original content.
    }

    // Part 1b: Select a template for the new aperture.
    let (template_definition_line, injection_aperture_id, original_template_id) =
        select_injection_template(&existing_definitions, &existing_aperture_ids);

    // Part 2: Renumber apertures to make space.
    let content_with_shifted_ids = renumber_apertures(gerber_content, injection_aperture_id);

    // Part 3: Generate a unique dimension from a hash of the content.
    let final_dimension_str =
        generate_hashed_dimension(&content_with_shifted_ids, is_foreign_board_file);

    // Part 4a: Create the new aperture definition line.
    let final_fingerprint_line = create_fingerprint_aperture_line(
        &template_definition_line,
        injection_aperture_id,
        original_template_id,
        &final_dimension_str,
    );

    // Part 4b: Insert the new line into the content.
    insert_new_aperture_line(
        &content_with_shifted_ids,
        &final_fingerprint_line,
        injection_aperture_id,
    )
}

/// Part 1a: Scan the beginning of the file to find existing aperture definitions.
fn scan_for_aperture_definitions(content_lines: &[&str]) -> (Vec<String>, Vec<u32>) {
    let mut existing_definitions = Vec::new();
    let mut existing_aperture_ids = Vec::new();
    // Only scan the top part of the file for efficiency, where definitions usually are.
    for line in content_lines.iter().take(200) {
        if line.starts_with("%ADD") {
            let potential_num_part = &line[4..]; // Skip "%ADD"
            if let Some(num_end_pos) = potential_num_part.find(|c: char| !c.is_ascii_digit()) {
                let num_str = &potential_num_part[..num_end_pos];
                if (2..=4).contains(&num_str.len()) {
                    if let Ok(num) = num_str.parse::<u32>() {
                        existing_definitions.push(line.to_string());
                        existing_aperture_ids.push(num);
                    }
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
) -> (String, u32, u32) {
    let mut rng = rand::rng();
    // Select a random index, avoiding the first few definitions.
    let injection_site_index = rng.random_range(5..existing_definitions.len());
    let template_definition_line = existing_definitions[injection_site_index].clone();
    let injection_aperture_id = existing_aperture_ids[injection_site_index]; // The ID to free up.
    let original_template_id = existing_aperture_ids[injection_site_index]; // The original ID from the template line.
    (
        template_definition_line,
        injection_aperture_id,
        original_template_id,
    )
}

/// Part 2: Renumber all subsequent apertures to make space for the new one.
fn renumber_apertures(content: &str, injection_aperture_id: u32) -> String {
    let mut renumbered_lines = Vec::new();
    for line in content.split('\n') {
        let mut prefix: Option<&str> = None;
        if line.starts_with("%ADD") {
            prefix = Some("%ADD");
        } else if line.starts_with("G54D") {
            prefix = Some("G54D");
        }
        let mut line_was_renumbered = false;
        if let Some(p) = prefix {
            let potential_num_part = &line[p.len()..];
            if let Some(num_end_pos) = potential_num_part.find(|c: char| !c.is_ascii_digit()) {
                let num_str = &potential_num_part[..num_end_pos];
                if (2..=4).contains(&num_str.len()) {
                    if let Ok(number) = num_str.parse::<u32>() {
                        if number >= injection_aperture_id {
                            let rest_of_line = &potential_num_part[num_end_pos..];
                            renumbered_lines.push(format!("{}{}{}", p, number + 1, rest_of_line));
                            line_was_renumbered = true;
                        }
                    }
                }
            }
        }
        if !line_was_renumbered {
            renumbered_lines.push(line.to_string());
        }
    }
    renumbered_lines.join("\n")
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
    let final_hex_chars = &hex_digest[hex_digest.len() - 2..];
    let decimal_from_hash = u32::from_str_radix(final_hex_chars, 16).unwrap_or(0) % 100;
    let hash_based_suffix = format!("{:02}", decimal_from_hash);
    let mut rng = rand::rng();
    let random_base_dimension: f64 = rng.random_range(0.0..1.0);
    let combined_dimension_str = format!("{:.2}{}", random_base_dimension, hash_based_suffix);
    if combined_dimension_str.parse::<f64>().unwrap_or(0.0) == 0.0 {
        "0.0100".to_string()
    } else {
        combined_dimension_str
    }
}

/// Part 4a: Create the new aperture definition line using the template.
fn create_fingerprint_aperture_line(
    template_definition_line: &str,
    injection_aperture_id: u32,
    original_template_id: u32,
    final_dimension_str: &str,
) -> String {
    let new_definition_from_template = if let Some(comma_pos) = template_definition_line.find(',') {
        let part_after_comma = &template_definition_line[comma_pos + 1..];
        let size_end_pos = part_after_comma
            .find(|c: char| !c.is_ascii_digit() && c != '.')
            .unwrap_or_else(|| part_after_comma.len());
        let original_size_part = &template_definition_line[comma_pos..comma_pos + 1 + size_end_pos];
        let new_size_part = format!(",{}", final_dimension_str);
        template_definition_line.replace(original_size_part, &new_size_part)
    } else {
        // Fallback if template is unusual.
        format!("%ADD{}C,{}*%", injection_aperture_id, final_dimension_str)
    };

    let old_id_in_template = format!("ADD{}", original_template_id);
    let new_id_str = format!("ADD{}", injection_aperture_id);
    new_definition_from_template.replace(&old_id_in_template, &new_id_str)
}

/// Part 4b: Intelligently insert the new definition line into the file.
fn insert_new_aperture_line(
    content_with_shifted_ids: &str,
    final_fingerprint_line: &str,
    injection_aperture_id: u32,
) -> String {
    let anchor_for_insertion = format!("%ADD{}", injection_aperture_id - 1);
    let mut output_lines: Vec<String> = Vec::new();
    let mut fingerprint_was_inserted = false;
    for line in content_with_shifted_ids.split('\n') {
        output_lines.push(line.to_string());
        if line.starts_with(&anchor_for_insertion) {
            output_lines.push(final_fingerprint_line.to_string());
            fingerprint_was_inserted = true;
        }
    }

    // If the primary insertion point wasn't found, use a fallback strategy.
    if !fingerprint_was_inserted {
        let mut final_lines: Vec<String> = Vec::new();
        let mut mo_section_found = false;
        for line in content_with_shifted_ids.split('\n') {
            if !mo_section_found && line.starts_with("%MO") {
                mo_section_found = true;
            } else if mo_section_found
                && !fingerprint_was_inserted
                && (line.starts_with("%LP") || line.starts_with('G'))
            {
                final_lines.push(final_fingerprint_line.to_string());
                fingerprint_was_inserted = true;
            }
            final_lines.push(line.to_string());
        }
        return final_lines.join("\n");
    }
    output_lines.join("\n")
}
