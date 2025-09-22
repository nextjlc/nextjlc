/* src/header.rs */

/* SPDX-License-Identifier: Public Domain */
/*
 * Author Acha <acha@acha666.cn>
 */

use chrono::Local;
use rand::Rng;

/// This function generates and returns a dynamic Gerber header string.
/// The header includes a randomly selected software name, a randomized version number,
/// and the current timestamp. The returned string ends with a newline.
///
/// # Returns
///
/// A `String` containing the generated Gerber header.
pub fn get_gerber_header() -> String {
    // Initialize a thread-local random number generator.
    let mut rng = rand::rng();

    // Randomly choose between "EasyEDA Pro" and "EasyEDA".
    let name = if rng.random_bool(0.5) {
        "EasyEDA Pro"
    } else {
        "EasyEDA"
    };

    // Generate random numbers for the version string.
    let major = rng.random_range(2..=3);
    let minor = rng.random_range(1..=5);
    let patch = rng.random_range(1..=42);
    let build = rng.random_range(0..=2);
    let version = format!("v{}.{}.{}.{}", major, minor, patch, build);

    // Get the current local time.
    let now = Local::now();

    // Format the final header string.
    // It no longer includes the original content, just the two header lines
    // followed by a newline character to separate it from the actual content later.
    format!(
        "G04 {} {}, {}*\nG04 Gerber Generator version 0.3*\n",
        name,
        version,
        now.format("%Y-%m-%d %H:%M:%S"),
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
