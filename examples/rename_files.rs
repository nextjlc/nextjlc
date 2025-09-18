/* examples/rename_files.rs */

use nextjlc::rename::{EdaType, map_filenames};
use std::collections::BTreeMap;

fn print_results(title: &str, mapping: &BTreeMap<String, String>) {
    println!("--- {} ---", title);
    println!("{:<40} -> {:<40}", "Original Name", "New Name");
    println!("{:-<84}", "");
    for (original, new) in mapping {
        println!("{:<40} -> {:<40}", original, new);
    }
    println!(); // Add a blank line for spacing
}

fn main() {
    // --- Test case 1: Altium Designer Gerber Files ---
    let ad_files = vec![
        "Gerber_MechanicalLayer1.GM1".to_string(),
        "Gerber_TopLayer.GTL".to_string(),
        "Gerber_TopSilkscreenLayer.GTO".to_string(),
        "Gerber_TopSolderMaskLayer.GTS".to_string(),
    ];

    // --- Test case 2: KiCad Gerber Files ---
    let kicad_files = vec![
        "KICAD-B_Cu.gbr".to_string(),
        "KICAD-B_Mask.gbr".to_string(),
        "KICAD-B_Paste.gbr".to_string(),
        "KICAD-B_Silkscreen.gbr".to_string(),
        "KICAD-drl_map.gbr".to_string(),
        "KICAD-Edge_Cuts.gbr".to_string(),
        "KICAD-F_Cu.gbr".to_string(),
        "KICAD-F_Mask.gbr".to_string(),
        "KICAD-F_Paste.gbr".to_string(),
        "KICAD-F_Silkscreen.gbr".to_string(),
        "KICAD-In1_Cu.gbr".to_string(),
        "KICAD-In2_Cu.gbr".to_string(),
        "KICAD-job.gbrjob".to_string(), // This should not be renamed
        "KICAD.drl".to_string(),        // This is the main PTH drill file
    ];

    // Process and print results for Altium Designer
    let ad_map = map_filenames(&ad_files, EdaType::Ad);
    print_results("Altium Designer Renaming Plan", &ad_map);

    // Process and print results for KiCad
    let kicad_map = map_filenames(&kicad_files, EdaType::KiCad);
    print_results("KiCad Renaming Plan", &kicad_map);
}
