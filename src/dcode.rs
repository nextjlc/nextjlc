/* src/dcode.rs */

/* SPDX-License-Identifier: Apache-2.0 */
/*
 * Author HalfSweet <halfsweet@halfsweet.cn>
 */

/// This function processes a string of Gerber data to prepend "G54"
/// to specific D-codes.
///
/// It iterates through each line of the input data and searches for D-codes
/// (e.g., D10*, D123*) that consist of 'D' followed by 2 to 4 digits and an asterisk.
/// For each valid D-code found, it prepends "G54".
///
/// The function skips any lines that already contain aperture definitions ("%ADD")
/// or the "G54D" prefix to avoid redundant processing.
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

    for current_line in input_lines {
        // Skip lines that are already definitions or use G54D codes.
        if current_line.contains("%ADD") || current_line.contains("G54D") {
            processed_lines.push(current_line.to_string());
            continue;
        }

        let mut updated_line = String::with_capacity(current_line.len() + 10);
        let mut last_pos = 0;
        let mut search_pos = 0;

        // Manually search for D-codes like D10*, D123*, etc.
        while let Some(d_pos) = current_line[search_pos..].find('D') {
            let absolute_d_pos = search_pos + d_pos;
            // Get the part of the string immediately after 'D'.
            let suffix = &current_line[absolute_d_pos + 1..];
            // Count how many digits follow 'D'.
            let num_len = suffix.chars().take_while(|c| c.is_ascii_digit()).count();

            // Check if the number has 2-4 digits and is followed by a '*'.
            if (2..=4).contains(&num_len)
                && suffix.len() > num_len
                && suffix.as_bytes()[num_len] == b'*'
            {
                let end_of_match = absolute_d_pos + 1 + num_len + 1;
                // Append content before the match.
                updated_line.push_str(&current_line[last_pos..absolute_d_pos]);
                // Prepend the required G54 prefix.
                updated_line.push_str("G54");
                // Append the original D-code.
                updated_line.push_str(&current_line[absolute_d_pos..end_of_match]);
                // Update positions for the next search.
                last_pos = end_of_match;
                search_pos = end_of_match;
            } else {
                // Move search position past the current 'D' if it's not a valid code.
                search_pos = absolute_d_pos + 1;
            }
        }

        // Append any remaining part of the line after the last match.
        updated_line.push_str(&current_line[last_pos..]);
        processed_lines.push(updated_line);
    }

    // Join all processed lines back into a single string.
    processed_lines.join("\n")
}
