//! Debug parser output for nested lists

use lumen::parse_markdown;

fn main() {
    let markdown = r#"# Test

- [Getting Started](#getting-started)
  - [Installation](#installation)
  - [Configuration](#configuration)
"#;

    let doc = parse_markdown(markdown);

    println!("Parsed document:");
    println!("{:#?}", doc);
}
