/* examples/dcode_processing.rs */

use nextjlc::dcode::process_d_codes;

fn main() {
    let gerber_sample = r#"
%FSLAX46Y46*%
%MOMM*%
%ADD10C,0.15000*%
G01*
X-1125Y-965D02*
X-1125Y965D01*
X-1075Y915D01*
X-1075Y-915D01*
X1075Y-915D01*
X1075Y915D01*
G04 A simple D-code*
X100Y200D11*
G04 Multiple D-codes on one line*
X50D12*Y50D13*
G04 An already processed line, should be skipped*
X-175Y-915G54D10*
G04 A D-code with too few digits, should be skipped*
X20Y30D3*
G04 A D-code with too many digits, should be skipped*
X40Y50D12345*
M02*"#;

    println!("--- Original Gerber Data ---");
    println!("{}\n", gerber_sample);

    // Process the Gerber data using the function.
    let processed_data = process_d_codes(gerber_sample.to_string());

    println!("--- Processed Gerber Data ---");
    println!("{}", processed_data);
}
