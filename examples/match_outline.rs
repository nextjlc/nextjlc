/* examples/match_outline.rs */

use nextjlc_core::outline::sort_gerber_files;

fn main() {
    // --- Test case 1: ad-gerber ---
    let mut hotbed_files = vec![
        "Gerber_MechanicalLayer1.GM1".to_string(),
        "Gerber_TopLayer.GTL".to_string(),
        "Gerber_TopSilkscreenLayer.GTO".to_string(),
        "Gerber_TopSolderMaskLayer.GTS".to_string(),
    ];

    println!("Original ad gerber files: {:?}", hotbed_files);
    // Sort the list of ad-gerber files.
    let sorted_hotbed = sort_gerber_files(&mut hotbed_files);
    println!("Sorted ad gerber files: {:?}", sorted_hotbed);
    // Print the highest priority file.
    if let Some(first_file) = sorted_hotbed.first() {
        println!("Highest priority file for ad gerber: {}\n", first_file);
    }

    // --- Test case 2: KICADGERBER ---
    let mut kicad_files = vec![
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
        "KICAD-job.gbrjob".to_string(),
        "KICAD.drl".to_string(),
    ];

    println!("Original KICADGERBER files: {:?}", kicad_files);
    // Sort the list of KICAD gerber files.
    let sorted_kicad = sort_gerber_files(&mut kicad_files);
    println!("Sorted KICADGERBER files: {:?}", sorted_kicad);
    // Print the highest priority file.
    if let Some(first_file) = sorted_kicad.first() {
        println!("Highest priority file for KICAD: {}", first_file);
    }
}
