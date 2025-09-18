/* examples/add_header.rs */

use nextjlc::header::add_gerber_header;

fn main() {
    // Define some sample Gerber content with mixed line endings (CRLF).
    let original_content = "G70*\r\n%MOIN*%\r\nD01*\r\nX0Y0D02*";

    println!("--- Original Content ---");
    println!("{}\n", original_content);

    // Call the function to add the header.
    let content_with_header = add_gerber_header(original_content);

    println!("--- Content with Dynamic Header ---");
    // Print the full result, which includes the new header and the processed content.
    // The header itself will be different each time you run the example due to its random nature.
    println!("{}", content_with_header);
}
