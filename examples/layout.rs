use lumen::layout::Viewport;
use lumen::{layout_document, parse_markdown, Theme};

fn main() {
    println!("=== Lumen Layout Engine Demo ===\n");

    // Parse markdown
    let markdown = r#"
# Layout Engine Test

This is a **test document** with *various* elements.

## Features

- Vertical flow layout
- Text wrapping
- Hit testing

## Code Example

```rust
fn main() {
    println!("Hello, Lumen!");
}
```

## Table Example

| Feature | Status |
|---------|--------|
| Layout | ✓ |
| Themes | ✓ |

> This is a blockquote with some content.

---

End of document.
"#;

    let doc = parse_markdown(markdown);
    let theme = Theme::builtin("docs").unwrap();
    let viewport = Viewport::new(80, 24);

    // Layout document
    let tree = layout_document(&doc, &theme, viewport, false);

    println!("Document Layout:");
    println!("- Total height: {} rows", tree.document_height());
    println!("- Viewport: {}x{}", viewport.width, viewport.height);
    println!("- Can scroll down: {}", tree.can_scroll_down());
    println!("- Number of hit regions: {}\n", tree.hit_regions.len());

    // Show layout tree structure
    println!("Layout Tree:");
    print_node(&tree.root, 0);

    println!("\n=== Hit Regions ===\n");
    for (i, region) in tree.hit_regions.iter().enumerate() {
        println!(
            "{}. {:?} at ({}, {}) {}x{}",
            i + 1,
            region.element,
            region.rect.x,
            region.rect.y,
            region.rect.width,
            region.rect.height
        );
    }

    // Test hit testing
    println!("\n=== Hit Testing ===\n");
    let test_points = vec![(10, 5), (40, 15), (50, 20)];

    for (x, y) in test_points {
        match tree.hit_test(x, y) {
            Some(region) => {
                println!("Hit at ({}, {}): {:?}", x, y, region.element);
            }
            None => {
                println!("No hit at ({}, {})", x, y);
            }
        }
    }

    println!("\n=== Scroll Simulation ===\n");
    let mut viewport_mut = viewport;
    println!("Initial scroll: {}", viewport_mut.scroll_y);

    viewport_mut.scroll_by(10);
    println!("After scroll_by(10): {}", viewport_mut.scroll_y);

    viewport_mut.scroll_to(50);
    println!("After scroll_to(50): {}", viewport_mut.scroll_y);

    viewport_mut.scroll_by(-20);
    println!("After scroll_by(-20): {}", viewport_mut.scroll_y);

    viewport_mut.scroll_by(-100); // Should clamp to 0
    println!("After scroll_by(-100): {} (clamped)", viewport_mut.scroll_y);
}

fn print_node(node: &lumen::layout::LayoutNode, indent: usize) {
    let prefix = "  ".repeat(indent);
    println!(
        "{}{:?} at ({}, {}) {}x{}",
        prefix, node.element, node.rect.x, node.rect.y, node.rect.width, node.rect.height
    );

    for child in &node.children {
        print_node(child, indent + 1);
    }
}
