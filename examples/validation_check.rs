/* examples/validation_check.rs */

use nextjlc::validation::{ValidationReport, validate_gerber_files};

// A helper function to run a validation test case and print the results clearly.
fn run_test_case(title: &str, files: &[String]) {
    println!("--- Running Test Case: {} ---", title);
    println!("Input Files: {:?}", files);

    match validate_gerber_files(files) {
        Ok(ValidationReport {
            layer_count,
            warnings,
        }) => {
            println!("\n[VALIDATION PASSED]");
            println!("   - Detected Copper Layers: {}", layer_count);
            if warnings.is_empty() {
                println!("   - Warnings: None");
            } else {
                println!("   - Warnings:");
                for warning in warnings {
                    println!("     - {}", warning);
                }
            }
        }
        Err(errors) => {
            println!("\n[VALIDATION FAILED]");
            println!("   - Errors:");
            for error in errors {
                println!("     - {}", error);
            }
        }
    }
    println!("\n{}\n", "-".repeat(80));
}

fn main() {
    // --- Test Case 1: Complete and Valid 4-Layer Board ---
    // This is the ideal case, based on your provided file list.
    // It should pass with no errors and no warnings (assuming all standard files are present).
    let complete_files: Vec<String> = [
        "Drill_PTH_Through.DRL",
        "Gerber_BoardOutlineLayer.GKO",
        "Gerber_BottomLayer.GBL",
        "Gerber_BottomPasteMaskLayer.GBP",
        "Gerber_BottomSilkscreenLayer.GBO",
        "Gerber_BottomSolderMaskLayer.GBS",
        "Gerber_InnerLayer1.G1",
        "Gerber_InnerLayer2.G2",
        "Gerber_TopLayer.GTL",
        "Gerber_TopPasteMaskLayer.GTP",
        "Gerber_TopSilkscreenLayer.GTO",
        "Gerber_TopSolderMaskLayer.GTS",
        // "PCB下单必读.txt" is ignored as it doesn't match any standard names.
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    run_test_case("Complete 4-Layer Board", &complete_files);

    // --- Test Case 2: Missing Board Outline ---
    // This is a critical error and should cause validation to fail immediately.
    let mut files_no_outline = complete_files.clone();
    files_no_outline.retain(|f| !f.starts_with("Gerber_BoardOutlineLayer"));
    run_test_case("Missing Board Outline (Critical Error)", &files_no_outline);

    // --- Test Case 3: Standard 2-Layer Board (No Inner Layers) ---
    // This should pass, but might have warnings if we remove paste/silk layers.
    let mut two_layer_files = complete_files.clone();
    two_layer_files.retain(|f| !f.starts_with("Gerber_InnerLayer"));
    run_test_case("Standard 2-Layer Board", &two_layer_files);

    // --- Test Case 4: 2-Layer Board Missing Optional Silkscreen Layers ---
    // This should pass, but generate two warnings.
    let mut two_layer_no_silk = two_layer_files.clone();
    two_layer_no_silk.retain(|f| !f.contains("Silkscreen"));
    run_test_case(
        "2-Layer Board without Silkscreen (Warnings)",
        &two_layer_no_silk,
    );

    // --- Test Case 5: Invalid Multilayer Board (Missing Bottom Copper) ---
    // A board with Top and Inner layers MUST have a Bottom layer. This is a critical error.
    let mut invalid_multilayer = complete_files.clone();
    invalid_multilayer.retain(|f| !f.starts_with("Gerber_BottomLayer"));
    run_test_case(
        "Invalid Multilayer Board - Missing Bottom Copper",
        &invalid_multilayer,
    );

    // --- Test Case 6: Minimal Valid 1-Layer Board ---
    // The absolute minimum required files for a simple single-sided board.
    // Should pass, but with warnings about missing paste/silk layers.
    let minimal_one_layer = vec![
        "Gerber_BoardOutlineLayer.GKO".to_string(),
        "Gerber_TopLayer.GTL".to_string(),
        "Gerber_TopSolderMaskLayer.GTS".to_string(),
        "Drill_PTH_Through.DRL".to_string(), // Drill file is not strictly checked by our rules, but good to have.
    ];
    run_test_case("Minimal 1-Layer Board (with Warnings)", &minimal_one_layer);

    // --- Test Case 7: Bottom Layer Present but Missing its Solder Mask ---
    // This is another critical error.
    let mut bottom_no_mask = two_layer_files.clone();
    bottom_no_mask.retain(|f| !f.starts_with("Gerber_BottomSolderMaskLayer"));
    run_test_case(
        "2-Layer Board Missing Bottom Solder Mask (Critical Error)",
        &bottom_no_mask,
    );
}
