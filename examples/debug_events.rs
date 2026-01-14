//! Debug pulldown-cmark events for nested lists

use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

fn main() {
    let markdown = r#"- Level 1
  - Level 2
    - Level 3
      - Level 4"#;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(markdown, options);

    println!("Events for nested list:");
    for (i, event) in parser.enumerate() {
        println!("{:3}: {:?}", i, event);
    }
}
