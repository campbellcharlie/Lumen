//! Parser tests

use super::parse_markdown;
use crate::ir::{Block, Inline};

#[test]
fn test_parse_simple_paragraph() {
    let markdown = "Hello, world!";
    let doc = parse_markdown(markdown);

    assert_eq!(doc.blocks.len(), 1);
    match &doc.blocks[0] {
        Block::Paragraph { content } => {
            assert_eq!(content.len(), 1);
            match &content[0] {
                Inline::Text(text) => assert_eq!(text, "Hello, world!"),
                _ => panic!("Expected Text inline"),
            }
        }
        _ => panic!("Expected Paragraph block"),
    }
}

#[test]
fn test_parse_heading() {
    let markdown = "# Heading 1\n\n## Heading 2";
    let doc = parse_markdown(markdown);

    assert_eq!(doc.blocks.len(), 2);

    match &doc.blocks[0] {
        Block::Heading { level, content } => {
            assert_eq!(*level, 1);
            assert_eq!(content.len(), 1);
        }
        _ => panic!("Expected Heading block"),
    }

    match &doc.blocks[1] {
        Block::Heading { level, .. } => {
            assert_eq!(*level, 2);
        }
        _ => panic!("Expected Heading block"),
    }
}

#[test]
fn test_parse_emphasis() {
    let markdown = "Text with **bold** and *italic* and ***both***.";
    let doc = parse_markdown(markdown);

    assert_eq!(doc.blocks.len(), 1);
    match &doc.blocks[0] {
        Block::Paragraph { content } => {
            assert!(content.iter().any(|i| matches!(i, Inline::Strong(_))));
            assert!(content.iter().any(|i| matches!(i, Inline::Emphasis(_))));
        }
        _ => panic!("Expected Paragraph block"),
    }
}

#[test]
fn test_parse_code() {
    let markdown = "Inline `code` here.";
    let doc = parse_markdown(markdown);

    assert_eq!(doc.blocks.len(), 1);
    match &doc.blocks[0] {
        Block::Paragraph { content } => {
            assert!(content.iter().any(|i| matches!(i, Inline::Code(_))));
        }
        _ => panic!("Expected Paragraph block"),
    }
}

#[test]
fn test_parse_code_block() {
    let markdown = "```rust\nfn main() {}\n```";
    let doc = parse_markdown(markdown);

    assert_eq!(doc.blocks.len(), 1);
    match &doc.blocks[0] {
        Block::CodeBlock { lang, code } => {
            assert_eq!(lang.as_deref(), Some("rust"));
            assert!(code.contains("fn main()"));
        }
        _ => panic!("Expected CodeBlock"),
    }
}

#[test]
fn test_parse_list() {
    let markdown = "- Item 1\n- Item 2\n- Item 3";
    let doc = parse_markdown(markdown);

    assert_eq!(doc.blocks.len(), 1);
    match &doc.blocks[0] {
        Block::List { ordered, items, .. } => {
            assert!(!ordered);
            assert_eq!(items.len(), 3);
        }
        _ => panic!("Expected List block"),
    }
}

#[test]
fn test_parse_ordered_list() {
    let markdown = "1. First\n2. Second\n3. Third";
    let doc = parse_markdown(markdown);

    assert_eq!(doc.blocks.len(), 1);
    match &doc.blocks[0] {
        Block::List {
            ordered,
            items,
            start,
        } => {
            assert!(ordered);
            assert_eq!(*start, 1);
            assert_eq!(items.len(), 3);
        }
        _ => panic!("Expected List block"),
    }
}

#[test]
fn test_parse_link() {
    let markdown = "[Link text](https://example.com)";
    let doc = parse_markdown(markdown);

    assert_eq!(doc.blocks.len(), 1);
    match &doc.blocks[0] {
        Block::Paragraph { content } => {
            assert!(content.iter().any(|i| matches!(i, Inline::Link { .. })));
        }
        _ => panic!("Expected Paragraph block"),
    }
}

#[test]
fn test_parse_image() {
    let markdown = "![Alt text](image.png)";
    let doc = parse_markdown(markdown);

    assert_eq!(doc.blocks.len(), 1);
    match &doc.blocks[0] {
        Block::Paragraph { content } => match &content[0] {
            Inline::Image { url, alt, .. } => {
                assert_eq!(url, "image.png");
                assert_eq!(alt, "Alt text");
            }
            _ => panic!("Expected Image inline"),
        },
        _ => panic!("Expected Paragraph block"),
    }
}

#[test]
fn test_parse_blockquote() {
    let markdown = "> This is a quote";
    let doc = parse_markdown(markdown);

    assert_eq!(doc.blocks.len(), 1);
    match &doc.blocks[0] {
        Block::BlockQuote { blocks } => {
            assert!(!blocks.is_empty());
        }
        _ => panic!("Expected BlockQuote block"),
    }
}

#[test]
fn test_parse_horizontal_rule() {
    let markdown = "Text above\n\n---\n\nText below";
    let doc = parse_markdown(markdown);

    assert!(doc
        .blocks
        .iter()
        .any(|b| matches!(b, Block::HorizontalRule)));
}

#[test]
fn test_parse_table() {
    let markdown = "| A | B |\n|---|---|\n| 1 | 2 |";
    let doc = parse_markdown(markdown);

    assert_eq!(doc.blocks.len(), 1);
    match &doc.blocks[0] {
        Block::Table { headers, rows, .. } => {
            assert_eq!(headers.len(), 2);
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0].len(), 2);
        }
        _ => panic!("Expected Table block"),
    }
}
