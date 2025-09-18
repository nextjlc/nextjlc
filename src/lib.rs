/* src/lib.rs */

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub mod dcode;
pub mod file_type;
pub mod fingerprint;
pub mod header;
pub mod outline;
pub mod rename;
pub mod validation;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn process_d_codes(gerber_data: String) -> String {
    dcode::process_d_codes(gerber_data)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn identify_software(content: &str) -> Option<String> {
    // We convert &'static str to String because wasm-bindgen handles owned types better.
    file_type::identify_software(content).map(|s| s.to_string())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn add_fingerprint(gerber_content: &str, is_foreign_board_file: bool) -> String {
    fingerprint::add_fingerprint(gerber_content, is_foreign_board_file)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn add_gerber_header(content: &str) -> String {
    header::add_gerber_header(content)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn get_order_guide_text() -> String {
    header::get_order_guide_text().to_string()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn sort_gerber_files(files: Vec<String>) -> Vec<String> {
    let mut mutable_files = files;
    outline::sort_gerber_files(&mut mutable_files)
}
