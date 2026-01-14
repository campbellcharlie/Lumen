//! Integration tests using test corpus

use lumen::{ir::Block, parse_markdown};
use std::fs;

fn load_fixture(name: &str) -> String {
    let path = format!("tests/fixtures/{}", name);
    fs::read_to_string(&path).expect(&format!("Failed to read fixture: {}", path))
}

#[test]
fn test_basic_markdown() {
    let markdown = load_fixture("basic.md");
    let doc = parse_markdown(&markdown);

    // Should have headings and paragraphs
    assert!(doc
        .blocks
        .iter()
        .any(|b| matches!(b, Block::Heading { level: 1, .. })));
    assert!(doc
        .blocks
        .iter()
        .any(|b| matches!(b, Block::Heading { level: 2, .. })));
    assert!(doc
        .blocks
        .iter()
        .any(|b| matches!(b, Block::Heading { level: 6, .. })));
    assert!(doc
        .blocks
        .iter()
        .any(|b| matches!(b, Block::Paragraph { .. })));
}

#[test]
fn test_lists_markdown() {
    let markdown = load_fixture("lists.md");
    let doc = parse_markdown(&markdown);

    // Should have both ordered and unordered lists
    let has_unordered = doc
        .blocks
        .iter()
        .any(|b| matches!(b, Block::List { ordered: false, .. }));
    let has_ordered = doc
        .blocks
        .iter()
        .any(|b| matches!(b, Block::List { ordered: true, .. }));

    assert!(has_unordered, "Should have unordered list");
    assert!(has_ordered, "Should have ordered list");
}

#[test]
fn test_code_markdown() {
    let markdown = load_fixture("code.md");
    let doc = parse_markdown(&markdown);

    // Should have code blocks
    let code_blocks: Vec<_> = doc
        .blocks
        .iter()
        .filter_map(|b| match b {
            Block::CodeBlock { lang, code } => Some((lang, code)),
            _ => None,
        })
        .collect();

    assert!(code_blocks.len() >= 2, "Should have at least 2 code blocks");

    // Check for rust code block
    assert!(
        code_blocks
            .iter()
            .any(|(lang, _)| { lang.as_deref() == Some("rust") }),
        "Should have rust code block"
    );
}

#[test]
fn test_tables_markdown() {
    let markdown = load_fixture("tables.md");
    let doc = parse_markdown(&markdown);

    // Should have tables
    let tables: Vec<_> = doc
        .blocks
        .iter()
        .filter_map(|b| match b {
            Block::Table {
                headers,
                rows,
                alignment,
            } => Some((headers, rows, alignment)),
            _ => None,
        })
        .collect();

    assert!(tables.len() >= 1, "Should have at least 1 table");

    // Check first table structure
    let (headers, rows, _) = tables[0];
    assert!(headers.len() > 0, "Table should have headers");
    assert!(rows.len() > 0, "Table should have rows");
}

#[test]
fn test_links_images_markdown() {
    let markdown = load_fixture("links_images.md");
    let doc = parse_markdown(&markdown);

    // Count blocks to ensure document parsed
    assert!(doc.blocks.len() > 0, "Should have parsed blocks");
}

#[test]
fn test_blockquotes_markdown() {
    let markdown = load_fixture("blockquotes.md");
    let doc = parse_markdown(&markdown);

    // Should have blockquotes
    let has_blockquote = doc
        .blocks
        .iter()
        .any(|b| matches!(b, Block::BlockQuote { .. }));

    assert!(has_blockquote, "Should have blockquote");
}

#[test]
fn test_complex_markdown() {
    let markdown = load_fixture("complex.md");
    let doc = parse_markdown(&markdown);

    // Should have various block types
    assert!(doc
        .blocks
        .iter()
        .any(|b| matches!(b, Block::Heading { .. })));
    assert!(doc
        .blocks
        .iter()
        .any(|b| matches!(b, Block::Paragraph { .. })));
    assert!(doc.blocks.iter().any(|b| matches!(b, Block::List { .. })));
    assert!(doc
        .blocks
        .iter()
        .any(|b| matches!(b, Block::CodeBlock { .. })));
    assert!(doc.blocks.iter().any(|b| matches!(b, Block::Table { .. })));
    assert!(doc
        .blocks
        .iter()
        .any(|b| matches!(b, Block::HorizontalRule)));
}
