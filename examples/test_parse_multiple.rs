//! Test parsing multiple files

use lumen::parse_markdown;
use std::fs;

fn main() {
    let test_files = ["test_simple.md", "test_nested_list.md", "test_table_borders.md"];

    for file in &test_files {
        println!("\n=== Parsing {} ===", file);
        match fs::read_to_string(file) {
            Ok(content) => {
                match std::panic::catch_unwind(|| parse_markdown(&content)) {
                    Ok(doc) => println!("✓ Parsed successfully ({} blocks)", doc.blocks.len()),
                    Err(e) => println!("✗ PANIC: {:?}", e),
                }
            }
            Err(e) => println!("✗ Could not read file: {}", e),
        }
    }
}
