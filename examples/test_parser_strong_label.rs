use lumen::parse_markdown;

fn main() {
    let markdown = r#"- **Research:**
  - Item 1
  - Item 2
"#;

    let doc = parse_markdown(markdown);

    println!("Parsed strong label document:");
    for block in &doc.blocks {
        println!("{:#?}", block);
    }
}
