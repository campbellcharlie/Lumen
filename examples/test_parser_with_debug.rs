use lumen::parse_markdown;

fn main() {
    // Test case that used to trigger the merge fix
    let markdown = r#"- **Research:**
  - Item 1
"#;

    let doc = parse_markdown(markdown);

    println!("=== After parsing ===");
    for block in &doc.blocks {
        println!("{:#?}", block);
    }
}
