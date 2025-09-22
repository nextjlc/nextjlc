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
pub fn process_d_codes(gerber_data: String, use_altium: bool) -> String {
    use dcode::GerberFlavor;

    let flavor = if use_altium {
        GerberFlavor::Altium
    } else {
        GerberFlavor::KiCad
    };

    dcode::process_d_codes(gerber_data, flavor)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn identify_software(content: &str) -> Option<String> {
    file_type::identify_software(content).map(|s| s.to_string())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn add_fingerprint(gerber_content: &str, is_foreign_board_file: bool) -> String {
    fingerprint::add_fingerprint(gerber_content, is_foreign_board_file)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn get_gerber_header() -> String {
    header::get_gerber_header()
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

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn map_filenames_ad(files: Vec<String>) -> js_sys::Map {
    let rename_map = rename::map_filenames(&files, rename::EdaType::Ad);
    let js_map = js_sys::Map::new();
    for (original, renamed) in rename_map {
        js_map.set(&JsValue::from(original), &JsValue::from(renamed));
    }
    js_map
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn map_filenames_kicad(files: Vec<String>) -> js_sys::Map {
    let rename_map = rename::map_filenames(&files, rename::EdaType::KiCad);
    let js_map = js_sys::Map::new();
    for (original, renamed) in rename_map {
        js_map.set(&JsValue::from(original), &JsValue::from(renamed));
    }
    js_map
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct ValidationResult {
    pub is_valid: bool,
    pub layer_count: u32,
    warnings: Vec<String>,
    errors: Vec<String>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl ValidationResult {
    #[wasm_bindgen(getter)]
    pub fn warnings(&self) -> Vec<String> {
        self.warnings.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn validate_gerber_files(files: Vec<String>) -> ValidationResult {
    match validation::validate_gerber_files(&files) {
        Ok(report) => ValidationResult {
            is_valid: true,
            layer_count: report.layer_count,
            warnings: report.warnings,
            errors: Vec::new(),
        },
        Err(errors) => ValidationResult {
            is_valid: false,
            layer_count: 0,
            warnings: Vec::new(),
            errors,
        },
    }
}
