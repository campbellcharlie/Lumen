//! Integration tests for layout edge cases and large documents

use lumen::layout::Viewport;
use lumen::{layout_document, parse_markdown, Theme};

#[test]
fn test_empty_document() {
    let markdown = "";
    let doc = parse_markdown(markdown);
    let theme = Theme::builtin("docs").unwrap();
    let viewport = Viewport::new(80, 24);

    let tree = layout_document(&doc, &theme, viewport, false);

    // Document height should be valid (u16 is always >= 0, but verify it exists)
    let _ = tree.document_height();
    assert!(!tree.can_scroll_down());
}

#[test]
fn test_very_long_line() {
    let long_word = "a".repeat(500);
    let markdown = format!("# Test\n\n{}", long_word);

    let doc = parse_markdown(&markdown);
    let theme = Theme::builtin("docs").unwrap();
    let viewport = Viewport::new(80, 24);

    let tree = layout_document(&doc, &theme, viewport, false);

    // Should handle word breaking
    assert!(tree.document_height() > 2);
}

#[test]
fn test_many_headings() {
    let mut markdown = String::new();
    for i in 1..=100 {
        markdown.push_str(&format!("## Heading {}\n\nSome content.\n\n", i));
    }

    let doc = parse_markdown(&markdown);
    let theme = Theme::builtin("docs").unwrap();
    let viewport = Viewport::new(80, 24);

    let tree = layout_document(&doc, &theme, viewport, false);

    assert!(tree.document_height() > 200);
    assert!(tree.hit_regions.len() >= 100); // At least one hit region per heading
}

#[test]
fn test_nested_lists() {
    let markdown = r#"
- Item 1
  - Nested 1.1
    - Deeply nested 1.1.1
  - Nested 1.2
- Item 2
  - Nested 2.1
"#;

    let doc = parse_markdown(markdown);
    let theme = Theme::builtin("docs").unwrap();
    let viewport = Viewport::new(80, 24);

    let tree = layout_document(&doc, &theme, viewport, false);

    assert!(tree.document_height() > 0);
}

#[test]
fn test_mixed_content_types() {
    let markdown = r#"
# Main Title

Regular paragraph with **bold** and *italic*.

## Subheading

- List item 1
- List item 2

```rust
fn code() {
    println!("test");
}
```

| Col1 | Col2 |
|------|------|
| A    | B    |

> Blockquote

[Link](https://example.com)

![Image](test.png)
"#;

    let doc = parse_markdown(markdown);
    let theme = Theme::builtin("docs").unwrap();
    let viewport = Viewport::new(80, 24);

    let tree = layout_document(&doc, &theme, viewport, false);

    assert!(tree.document_height() > 15);
    assert!(tree.hit_regions.len() > 2);
}

#[test]
fn test_narrow_viewport() {
    let markdown = "# Test\n\nThis is a paragraph that will need wrapping.";

    let doc = parse_markdown(markdown);
    let theme = Theme::builtin("docs").unwrap();

    // Very narrow viewport
    let viewport = Viewport::new(20, 10);
    let tree = layout_document(&doc, &theme, viewport, false);

    // Should handle wrapping
    assert!(tree.document_height() > 2);
}

#[test]
fn test_wide_viewport() {
    let markdown = "# Test\n\nShort text.";

    let doc = parse_markdown(markdown);
    let theme = Theme::builtin("docs").unwrap();

    // Very wide viewport
    let viewport = Viewport::new(200, 50);
    let tree = layout_document(&doc, &theme, viewport, false);

    assert!(tree.document_height() < 10);
}

#[test]
fn test_multiple_code_blocks() {
    let mut markdown = String::from("# Code Examples\n\n");

    for i in 1..=10 {
        markdown.push_str(&format!(
            "## Example {}\n\n```rust\nfn test{}() {{}}\n```\n\n",
            i, i
        ));
    }

    let doc = parse_markdown(&markdown);
    let theme = Theme::builtin("docs").unwrap();
    let viewport = Viewport::new(80, 24);

    let tree = layout_document(&doc, &theme, viewport, false);

    assert!(tree.document_height() > 50);
}

#[test]
fn test_theme_switching() {
    let markdown = "# Test\n\nSome content";
    let doc = parse_markdown(markdown);
    let viewport = Viewport::new(80, 24);

    // Test with different themes
    let themes = ["docs", "neon", "minimal", "dracula"];

    for theme_name in themes {
        let theme = Theme::builtin(theme_name).unwrap();
        let tree = layout_document(&doc, &theme, viewport, false);

        assert!(tree.document_height() > 0);
    }
}

#[test]
fn test_inline_vs_sidebar_images() {
    let markdown = "# Images\n\n![Test](image.png)\n\nMore content.";

    let doc = parse_markdown(markdown);
    let theme = Theme::builtin("docs").unwrap();
    let viewport = Viewport::new(80, 24);

    // Sidebar mode
    let tree_sidebar = layout_document(&doc, &theme, viewport, false);
    assert_eq!(tree_sidebar.images.len(), 1);

    // Inline mode
    let tree_inline = layout_document(&doc, &theme, viewport, true);
    // In inline mode, images are rendered differently
    assert!(tree_inline.document_height() > 0);
}
