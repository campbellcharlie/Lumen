//! Debug parser output for nested lists

use lumen::parse_markdown;

fn main() {
    let markdown = r#"# Test

-
  - Child item
  - Another child
"#;

    let doc = parse_markdown(markdown);

    println!("Parsed document:");
    for block in &doc.blocks {
        println!("{:#?}", block);
    }
}
