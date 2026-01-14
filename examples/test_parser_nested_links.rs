use lumen::parse_markdown;

fn main() {
    let markdown = r#"# Test

- [Parent](#parent)
  - [Child 1](#child1)
  - [Child 2](#child2)
"#;

    let doc = parse_markdown(markdown);

    println!("Parsed TOC document:");
    for block in &doc.blocks {
        println!("{:#?}", block);
    }
}
