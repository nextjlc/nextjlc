/* src/header.rs */

/* SPDX-License-Identifier: Public Domain */
/*
 * Author Acha <acha@acha666.cn>
 */

use chrono::Local;
use rand::Rng;

/// Internal struct holding the generated header components.
struct HeaderInfo {
    software_name: String,
    version: String,
    timestamp: String,
}

/// Core function that generates randomized header information.
/// This is shared between Gerber and Excellon header generators.
fn generate_header_info() -> HeaderInfo {
    let mut rng = rand::rng();

    let software_name = if rng.random_bool(0.5) {
        "EasyEDA Pro".to_string()
    } else {
        "EasyEDA".to_string()
    };

    let major = rng.random_range(2..=3);
    let minor = rng.random_range(1..=5);
    let patch = rng.random_range(1..=42);
    let build = rng.random_range(0..=2);
    let version = format!("v{}.{}.{}.{}", major, minor, patch, build);

    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

    HeaderInfo {
        software_name,
        version,
        timestamp,
    }
}

/// Generates a dynamic Gerber header string with G04 comment format.
/// Used for Gerber files (.GTL, .GBL, .GTO, etc.)
pub fn get_gerber_header() -> String {
    let info = generate_header_info();
    format!(
        "G04 {} {}, {}*\nG04 Gerber Generator version 0.3*\n",
        info.software_name, info.version, info.timestamp,
    )
}

/// Generates a dynamic Excellon drill header string with semicolon comment format.
/// Used for drill files (.DRL, .TXT)
///
/// # Arguments
/// * `hole_type` - "PLATED" or "NON_PLATED"
/// * `layer_name` - Layer name like "PTH_Through" or "NPTH_Through"
pub fn get_drill_header(hole_type: &str, layer_name: &str) -> String {
    let info = generate_header_info();
    format!(
        ";TYPE={}\n;Layer: {}\n;{} {}, {}\n;Gerber Generator version 0.3\n",
        hole_type, layer_name, info.software_name, info.version, info.timestamp,
    )
}

/// This function provides a static help message that directs users to the
/// official documentation for placing a PCB order.
///
/// # Returns
///
/// A static string slice (`&'static str`) containing the help text in Chinese.
pub fn get_order_guide_text() -> &'static str {
    r#"如何进行PCB下单

请查看：
https://prodocs.lceda.cn/cn/pcb/order-order-pcb/index.html"#
}
