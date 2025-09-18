/* src/validation.rs */

/* SPDX-License-Identifier: MIT */
/*
 * Author Canmi <t@canmi.icu>
 */

// A struct to hold the successful result of a validation check.
// It contains the calculated number of copper layers and a list of non-critical warnings.
#[derive(Debug, PartialEq, Eq)]
pub struct ValidationReport {
    pub layer_count: u32,
    pub warnings: Vec<String>,
}

// Defines the prefixes for files that are absolutely required for a valid Gerber set.
const REQUIRED_PREFIXES: &[&str] = &[
    "Gerber_BoardOutlineLayer",
    "Gerber_TopLayer",
    "Gerber_TopSolderMaskLayer",
];

// Validates a list of standardized Gerber filenames against a set of manufacturing rules.
// This function is pure: it takes a list of names and returns a result without any I/O.
//
// # Arguments
//
// * `files` - A slice of strings, where each string is a filename that has already
//   been renamed to the standard format (e.g., by the `rename` module).
//
// # Returns
//
// * `Ok(ValidationReport)` - If all critical rules pass. The report includes the
//   detected copper layer count and a list of warnings for non-critical issues
//   (e.g., missing silkscreen or paste layers).
// * `Err(Vec<String>)` - If any critical rules fail. The vector contains a list of
//   all error messages detailing what is missing or incorrect.
pub fn validate_gerber_files(files: &[String]) -> Result<ValidationReport, Vec<String>> {
    let mut errors: Vec<String> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();

    // --- 1. Check for the presence of absolutely required files ---
    for &prefix in REQUIRED_PREFIXES {
        if !files.iter().any(|f| f.starts_with(prefix)) {
            errors.push(format!("Missing required file starting with: '{}'", prefix));
        }
    }

    // --- 2. Gather facts about the file set for conditional checks ---
    let has_top_copper = files.iter().any(|f| f.starts_with("Gerber_TopLayer"));
    let has_bottom_copper = files.iter().any(|f| f.starts_with("Gerber_BottomLayer"));
    let inner_layer_count = files
        .iter()
        .filter(|f| f.starts_with("Gerber_InnerLayer"))
        .count() as u32;

    // --- 3. Perform conditional checks based on layer presence ---

    // If a top copper layer exists, its associated layers should also exist.
    if has_top_copper {
        if !files
            .iter()
            .any(|f| f.starts_with("Gerber_TopSolderMaskLayer"))
        {
            // This is a required file, but we check it here for a more descriptive error message.
            // Using a separate check to avoid duplicate messages if it's already in REQUIRED_PREFIXES.
            if !errors
                .iter()
                .any(|e| e.contains("Gerber_TopSolderMaskLayer"))
            {
                errors.push("Top copper layer is present, but the required 'Gerber_TopSolderMaskLayer' is missing.".to_string());
            }
        }
        if !files
            .iter()
            .any(|f| f.starts_with("Gerber_TopSilkscreenLayer"))
        {
            warnings.push(
                "Warning: 'Gerber_TopSilkscreenLayer' is missing for the top side.".to_string(),
            );
        }
        if !files
            .iter()
            .any(|f| f.starts_with("Gerber_TopPasteMaskLayer"))
        {
            warnings.push("Warning: 'Gerber_TopPasteMaskLayer' is missing. This is usually needed for SMD components.".to_string());
        }
    }

    // If a bottom copper layer exists, its associated layers should also exist.
    if has_bottom_copper {
        if !files
            .iter()
            .any(|f| f.starts_with("Gerber_BottomSolderMaskLayer"))
        {
            errors.push(
                "Bottom copper layer is present, but 'Gerber_BottomSolderMaskLayer' is missing."
                    .to_string(),
            );
        }
        if !files
            .iter()
            .any(|f| f.starts_with("Gerber_BottomSilkscreenLayer"))
        {
            warnings.push(
                "Warning: 'Gerber_BottomSilkscreenLayer' is missing for the bottom side."
                    .to_string(),
            );
        }
        if !files
            .iter()
            .any(|f| f.starts_with("Gerber_BottomPasteMaskLayer"))
        {
            warnings.push(
                "Warning: 'Gerber_BottomPasteMaskLayer' is missing for the bottom side."
                    .to_string(),
            );
        }
    }

    // A multilayer board (top + inner) must have a bottom layer.
    if has_top_copper && inner_layer_count > 0 && !has_bottom_copper {
        errors.push("Invalid layer stackup: A board with top and inner copper layers must also have a bottom copper layer.".to_string());
    }

    // --- 4. Calculate final layer count ---
    let total_layer_count =
        (has_top_copper as u32) + (has_bottom_copper as u32) + inner_layer_count;

    // --- 5. Return the final result ---
    if errors.is_empty() {
        Ok(ValidationReport {
            layer_count: total_layer_count,
            warnings,
        })
    } else {
        Err(errors)
    }
}
