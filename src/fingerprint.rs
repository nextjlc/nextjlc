/* src/fingerprint.rs */

/* SPDX-License-Identifier: Apache-2.0 */
/*
 * Author HalfSweet <halfsweet@halfsweet.cn>
 */

use md5::{Digest, Md5};
use regex::Regex;
use std::collections::BTreeMap;

/// The main public function for embedding a fingerprint aperture.
pub fn add_fingerprint(gerber_content: &str, is_foreign_board_file: bool) -> String {
    // Normalize line endings and strip BOM to ensure consistent scanning.
    let mut normalized = gerber_content.replace("\r\n", "\n");
    normalized = normalized.trim_start_matches('\u{FEFF}').to_string();

    if normalized.len() > 30_000_000 {
        return normalized;
    }

    let (definitions, numbers) = scan_for_aperture_definitions(&normalized);
    if definitions.len() < 5 {
        return normalized;
    }

    let (template, target_number, original_number) =
        select_injection_template(&definitions, &numbers);

    let content_with_shifted_ids = renumber_apertures(&normalized, original_number);

    let final_dimension_str =
        generate_hashed_dimension(&content_with_shifted_ids, is_foreign_board_file);

    let final_fingerprint_line = create_fingerprint_aperture_line(
        &template,
        target_number,
        original_number,
        &final_dimension_str,
    );

    insert_new_aperture_line(
        &content_with_shifted_ids,
        &final_fingerprint_line,
        target_number,
    )
}

fn scan_for_aperture_definitions(content: &str) -> (Vec<String>, Vec<u32>) {
    let mut definitions = Vec::new();
    let mut numbers = Vec::new();
    let re = Regex::new(r"^%ADD(\d{2,4})\D.*").unwrap();

    for line in content.lines().take(200) {
        let line_trim = line.trim();
        if let Some(caps) = re.captures(line_trim) {
            if let Ok(num) = caps[1].parse::<u32>() {
                definitions.push(line_trim.to_string());
                numbers.push(num);
            }
        }
    }
    (definitions, numbers)
}

/// Deterministic selection: group by numeric id then choose 6th smallest number.
fn select_injection_template(definitions: &[String], numbers: &[u32]) -> (String, u32, u32) {
    let mut map: BTreeMap<u32, Vec<String>> = BTreeMap::new();
    for (num, def) in numbers.iter().copied().zip(definitions.iter().cloned()) {
        map.entry(num).or_default().push(def);
    }

    let keys: Vec<u32> = map.keys().copied().collect();
    let original_number = if keys.len() > 5 {
        keys[5]
    } else {
        *keys.last().unwrap()
    };

    // choose canonical template among candidates (lexicographically smallest)
    let templates = map.get(&original_number).unwrap();
    let template = templates.iter().min().unwrap().clone();

    let target_number = original_number; // keep same semantics as native implementation
    (template, target_number, original_number)
}

fn renumber_apertures(content: &str, original_number: u32) -> String {
    let re = Regex::new(r"(?m)^(%ADD|G54D)(\d{2,4})").unwrap();
    re.replace_all(content, |caps: &regex::Captures| {
        let prefix = &caps[1];
        let number: u32 = caps[2].parse().unwrap_or(0);

        if number >= original_number {
            format!("{}{}", prefix, number + 1)
        } else {
            caps[0].to_string()
        }
    })
    .to_string()
}

fn generate_hashed_dimension(content: &str, is_foreign: bool) -> String {
    let hash_content = if is_foreign {
        format!("494d{}", content)
    } else {
        content.to_string()
    };

    let mut hasher = Md5::new();
    hasher.update(hash_content.as_bytes());
    let hash_result = hasher.finalize();
    let hash_hex = format!("{:x}", hash_result);

    let last_two_hex = &hash_hex[hash_hex.len() - 2..];
    let hash_number = u32::from_str_radix(last_two_hex, 16).unwrap_or(0) % 100;
    let hash_suffix = format!("{:02}", hash_number);

    let base_size_int: u32 = 42;
    let final_size_int = format!("{:02}{}", base_size_int, hash_suffix);
    let final_str = format!("0.{}{}", &final_size_int[..2], &final_size_int[2..]);

    if final_str == "0.0000" {
        "0.0100".to_string()
    } else {
        final_str
    }
}

fn create_fingerprint_aperture_line(
    template: &str,
    target_number: u32,
    original_number: u32,
    final_size: &str,
) -> String {
    let size_re = Regex::new(r",([\d.]+)").unwrap();
    let hash_aperture = if let Some(cap) = size_re.captures(template) {
        let original_size_part = cap.get(0).unwrap().as_str();
        let new_size_part = format!(",{}", final_size);
        template.replace(original_size_part, &new_size_part)
    } else {
        format!("%ADD{}C,{}*%", target_number, final_size)
    };

    let id_re = Regex::new(&format!("ADD{}", original_number)).unwrap();
    id_re
        .replace(&hash_aperture, &format!("ADD{}", target_number))
        .to_string()
}

fn insert_new_aperture_line(content: &str, fingerprint_line: &str, target_number: u32) -> String {
    let mut result_lines: Vec<String> = Vec::new();
    let mut inserted = false;
    let insertion_anchor = format!("%ADD{}", target_number - 1);

    for line in content.split('\n') {
        result_lines.push(line.to_string());
        if line.starts_with(&insertion_anchor) {
            result_lines.push(fingerprint_line.to_string());
            inserted = true;
        }
    }

    if inserted {
        return result_lines.join("\n");
    }

    let mut final_lines: Vec<String> = Vec::new();
    let mut mo_found = false;
    for line in content.split('\n') {
        if !mo_found && line.starts_with("%MO") {
            mo_found = true;
        } else if mo_found && !inserted && (line.starts_with("%LP") || line.starts_with('G')) {
            final_lines.push(fingerprint_line.to_string());
            inserted = true;
        }
        final_lines.push(line.to_string());
    }

    if !inserted {
        final_lines.push(fingerprint_line.to_string());
    }
    final_lines.join("\n")
}
