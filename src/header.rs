/* src/header.rs */

/* SPDX-License-Identifier: Public Domain */
/*
 * Author Acha <acha@acha666.cn>
 * Author Canmi <t@canmi.icu>
 */

use chrono::Local;
use rand::Rng;

// This function prepends a dynamically generated Gerber header to the given content.
// The header includes a randomly selected software name, a randomized version number,
// and the current timestamp. It also standardizes line endings to LF ('\n').
//
// # Arguments
//
// * `content` - A string slice (`&str`) representing the original file content.
//
// # Returns
//
// A new `String` with the generated header prepended to the processed content.
pub fn add_gerber_header(content: &str) -> String {
    // Initialize a random number generator.
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

    // Format the final string.
    // The header is added, and the original content's line endings are normalized to LF.
    format!(
        "G04 {} {}, {}*\nG04 Gerber Generator version 0.3*\n{}",
        name,
        version,
        now.format("%Y-%m-%d %H:%M:%S"),
        content.replace("\r\n", "\n")
    )
}
