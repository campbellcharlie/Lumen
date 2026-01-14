//! Debug parser output for nested lists

use lumen::parse_markdown;

fn main() {
    let markdown = r#"# Test

- Parent 1
  - Child 1a
  - Child 1b
- Parent 2
  - Child 2a

- Level 1
  - Level 2
    - Level 3
      - Level 4
"#;

    let doc = parse_markdown(markdown);

    println!("Parsed document:");
    for block in &doc.blocks {
        println!("{:#?}", block);
    }
}
