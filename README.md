# Lumen

**A browser-like Markdown document renderer for modern terminals**

Lumen renders Markdown documents with rich layout, theming, and smooth scrolling—directly in your terminal. Unlike traditional Markdown viewers that style text with ANSI codes, Lumen treats rendering as a proper layout problem: Markdown → IR → Layout → Render.

---

## Quick Start

```bash
# Build the project
cargo build --release

# View a markdown file
cargo run --release -- DEMO.md

# Use a different theme
cargo run --release -- DEMO.md neon

# Or use the binary directly
./target/release/lumen README.md
```

### Available Themes

- `docs` — Clean, documentation-focused (default)
- `neon` — Vibrant, modern with bright colors
- `minimal` — Low visual noise, ASCII fallbacks

---

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `j` / `↓` | Scroll down one line |
| `k` / `↑` | Scroll up one line |
| `d` | Scroll down half page |
| `u` | Scroll up half page |
| `Space` / `PageDown` | Scroll down one page |
| `PageUp` | Scroll up one page |
| `g` / `Home` | Go to top of document |
| `G` / `End` | Go to bottom of document |
| `a` | Cycle through links (table of contents navigation) |
| `Enter` | Follow selected link (jump to anchor) |
| `q` / `Esc` | Quit |

---

## Features

### ✓ Implemented (Phases 0-4)

- **Phase 1**: Markdown parser with GFM support
  - Headings, paragraphs, lists (including deep nesting 4+ levels), tables, code blocks
  - Task lists, strikethrough, links, images
  - Blockquotes with nesting
  - Proper tight list handling for correct structure

- **Phase 2**: CSS-like theming system
  - 3 built-in themes (Docs, Neon, Minimal)
  - Color palettes with RGB/ANSI fallbacks
  - Border styles (Single, Double, Rounded, Heavy, ASCII)
  - Typography and spacing configuration

- **Phase 3**: Layout engine
  - Vertical flow layout with proper margins
  - Inline text wrapping (word-boundary + long-word breaking)
  - Table layout with column distribution and accurate border rendering
  - Viewport and scrolling model
  - Hit regions for interactive elements (links, anchors)

- **Phase 4**: Terminal renderer
  - Ratatui-based rendering with double-buffering
  - Rich colors (24-bit RGB, 256-color, 16-color)
  - Box-drawing characters for borders
  - Keyboard navigation with link cycling and anchor jumping
  - Visual link highlighting for selected links
  - Status bar with position indicator

### ⏳ Planned (Future Phases)

- **Phase 5**: Media layer (iTerm2/Kitty/SIXEL images)
- **Phase 6**: Enhanced UX (search, syntax highlighting, minimap)
- **Phase 7**: Packaging and distribution

---

## Architecture

```
Markdown → IR (pulldown-cmark)
            ↓
     Theme + Layout Engine
            ↓
     Terminal Renderer (Ratatui)
```

### Key Design Decisions

1. **Stable IR**: Frozen intermediate representation prevents downstream churn
2. **Token-based theming**: No CSS selectors, just element-type styling
3. **Two-phase layout**: Measure intrinsic sizes, then assign positions
4. **Character-grid coordinates**: Natural fit for terminal rendering
5. **Vertical-only scrolling**: Simpler v1, matches terminal usage

---

## Project Structure

```
Lumen/
├── src/
│   ├── ir/           # Intermediate representation (Phase 1)
│   ├── parser/       # Markdown → IR (Phase 1)
│   ├── theme/        # Theming system (Phase 2)
│   ├── layout/       # Layout engine (Phase 3)
│   ├── render/       # Terminal renderer (Phase 4)
│   ├── lib.rs        # Public API
│   └── main.rs       # Interactive viewer binary
├── themes/           # YAML theme files
├── tests/
│   ├── fixtures/     # Test markdown documents
│   └── *_tests.rs    # Integration tests
└── examples/         # Usage examples
```

---

## Building Blocks

**Core Dependencies**:
- [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark) — Fast, spec-compliant Markdown parser
- [Ratatui](https://github.com/ratatui/ratatui) — Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) — Cross-platform terminal manipulation

**Inspiration**:
- [Carbonyl](https://github.com/fathyb/carbonyl) — Browser in terminal
- [Glow](https://github.com/charmbracelet/glow) — Markdown TUI viewer

---

## Development

### Running Tests

```bash
# All tests (56 passing)
cargo test

# Specific test suite
cargo test theme
cargo test layout
```

### Examples

```bash
# Theme demo
cargo run --example themes

# Layout demo
cargo run --example layout
```

---

## Technical Specs

- **Language**: Rust 1.91+
- **Target platform**: macOS (M1 Max with 32GB RAM)
- **Terminal support**: iTerm2, Kitty, WezTerm, Alacritty
- **Color support**: True color (24-bit RGB) with graceful fallbacks
- **Test coverage**: 56 tests across all modules

---

## License

MIT OR Apache-2.0
