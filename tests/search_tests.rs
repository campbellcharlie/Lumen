//! Integration tests for search functionality

use lumen::layout::Viewport;
use lumen::search::SearchState;
use lumen::{layout_document, parse_markdown, Theme};

#[test]
fn test_search_basic() {
    let markdown = r#"
# Test Document

This is a test document with multiple test occurrences.
We want to test the search functionality.
"#;

    let doc = parse_markdown(markdown);
    let theme = Theme::builtin("docs").unwrap();
    let viewport = Viewport::new(80, 24);
    let tree = layout_document(&doc, &theme, viewport, false);

    let mut search = SearchState::new();
    search.needle = "test".to_string();
    search.execute_search(&tree.root);

    // Should find multiple matches
    assert!(search.match_count() > 2);
    assert!(search.current_match().is_some());
}

#[test]
fn test_search_case_insensitive() {
    let markdown = "# TEST\n\nTest test TeSt";

    let doc = parse_markdown(markdown);
    let theme = Theme::builtin("docs").unwrap();
    let viewport = Viewport::new(80, 24);
    let tree = layout_document(&doc, &theme, viewport, false);

    let mut search = SearchState::new();
    search.needle = "test".to_string();
    search.execute_search(&tree.root);

    // Should match all case variants
    assert_eq!(search.match_count(), 4);
}

#[test]
fn test_search_no_matches() {
    let markdown = "# Document\n\nSome content here";

    let doc = parse_markdown(markdown);
    let theme = Theme::builtin("docs").unwrap();
    let viewport = Viewport::new(80, 24);
    let tree = layout_document(&doc, &theme, viewport, false);

    let mut search = SearchState::new();
    search.needle = "nonexistent".to_string();
    search.execute_search(&tree.root);

    assert_eq!(search.match_count(), 0);
    assert!(search.current_match().is_none());
}

#[test]
fn test_search_navigation() {
    let markdown = "test1 test2 test3 test4 test5";

    let doc = parse_markdown(markdown);
    let theme = Theme::builtin("docs").unwrap();
    let viewport = Viewport::new(80, 24);
    let tree = layout_document(&doc, &theme, viewport, false);

    let mut search = SearchState::new();
    search.needle = "test".to_string();
    search.execute_search(&tree.root);

    assert_eq!(search.match_count(), 5);

    // First match selected by default
    let first_y = search.current_match().unwrap().y;
    let first_x = search.current_match().unwrap().x;

    // Navigate forward
    search.next_match();
    search.next_match();
    search.next_match();
    search.next_match();

    // At 5th match now
    search.next_match(); // Should wrap to first
    let wrapped_y = search.current_match().unwrap().y;
    let wrapped_x = search.current_match().unwrap().x;
    assert_eq!(wrapped_y, first_y);
    assert_eq!(wrapped_x, first_x);

    // Navigate backward
    search.prev_match(); // Should wrap to last
    assert!(search.current_match().is_some());
}

#[test]
fn test_search_empty_query() {
    let markdown = "# Test\n\nContent";

    let doc = parse_markdown(markdown);
    let theme = Theme::builtin("docs").unwrap();
    let viewport = Viewport::new(80, 24);
    let tree = layout_document(&doc, &theme, viewport, false);

    let mut search = SearchState::new();
    search.needle = "".to_string();
    search.execute_search(&tree.root);

    assert_eq!(search.match_count(), 0);
}

#[test]
fn test_search_in_code_blocks() {
    let markdown = r#"
# Code Example

```rust
fn test() {
    println!("test");
}
```

Regular test outside code.
"#;

    let doc = parse_markdown(markdown);
    let theme = Theme::builtin("docs").unwrap();
    let viewport = Viewport::new(80, 24);
    let tree = layout_document(&doc, &theme, viewport, false);

    let mut search = SearchState::new();
    search.needle = "test".to_string();
    search.execute_search(&tree.root);

    // Should find "test" in both code block and regular text
    assert!(search.match_count() >= 2);
}

#[test]
fn test_search_special_characters() {
    let markdown = "Test with (parentheses) and [brackets]";

    let doc = parse_markdown(markdown);
    let theme = Theme::builtin("docs").unwrap();
    let viewport = Viewport::new(80, 24);
    let tree = layout_document(&doc, &theme, viewport, false);

    let mut search = SearchState::new();
    search.needle = "parentheses".to_string();
    search.execute_search(&tree.root);

    assert_eq!(search.match_count(), 1);

    search.needle = "brackets".to_string();
    search.execute_search(&tree.root);

    assert_eq!(search.match_count(), 1);
}
