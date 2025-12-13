# Lumen Demo Document

Welcome to **Lumen**, a browser-like Markdown viewer for modern terminals!

## Features

Lumen renders Markdown with:
- **Rich typography**: Bold, *italic*, and `inline code`
- **Color themes**: Multiple built-in themes
- **Smooth scrolling**: Keyboard navigation
- **Tables**: With proper alignment
- **Code blocks**: With syntax-aware badges
- **Block quotes**: With visual borders

---

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `j` / `↓` | Scroll down one line |
| `k` / `↑` | Scroll up one line |
| `d` | Scroll down half page |
| `u` | Scroll up half page |
| `Space` | Scroll down one page |
| `PageDown` | Scroll down one page |
| `PageUp` | Scroll up one page |
| `g` / `Home` | Go to top |
| `G` / `End` | Go to bottom |
| `q` / `Esc` | Quit |

---

## Code Example

Here's a simple Rust function:

```rust
fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}
```

---

## Lists

### Unordered List

- First item
- Second item
  - Nested item 1
  - Nested item 2
- Third item

### Ordered List

1. Step one
2. Step two
3. Step three

### Task List

- [x] Implemented parser
- [x] Implemented layout engine
- [x] Implemented renderer
- [ ] Add syntax highlighting
- [ ] Add image support

---

## Tables

### Feature Comparison

| Feature | Lumen | Traditional Viewers |
|---------|-------|---------------------|
| Rich colors | ✓ | Limited |
| Layout engine | ✓ | ✗ |
| Themes | ✓ | ✗ |
| Box borders | ✓ | ✗ |

---

## Block Quotes

> "The only way to do great work is to love what you do."
>
> — Steve Jobs

> **Note**: Block quotes can contain
>
> Multiple paragraphs and *formatting*.

---

## Technical Details

Lumen uses a **browser-like rendering pipeline**:

```
Markdown → IR → Layout Tree → Terminal Renderer
```

Each phase is optimized for:
- **Performance**: Fast rendering, minimal reflows
- **Correctness**: Proper text wrapping, layout
- **Beauty**: Rich typography, smooth scrolling

---

## Themes

Try different themes by passing them as arguments:

```bash
lumen DEMO.md docs    # Documentation theme (default)
lumen DEMO.md neon    # Neon theme (vibrant colors)
lumen DEMO.md minimal # Minimal theme (low visual noise)
```

---

## More Examples

### Inline Formatting

You can combine **bold** and *italic* like ***this***, or use ~~strikethrough~~.

### Links

Visit the [Lumen GitHub repository](https://github.com) for more information.

### Horizontal Rules

Horizontal rules separate sections:

---

That's all for now! Press `q` to quit or scroll around to explore the rendering.
