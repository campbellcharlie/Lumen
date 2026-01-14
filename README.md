# Lumen

**A browser-like Markdown document renderer for modern terminals**

Lumen renders Markdown documents with rich layout, theming, and smooth scrolling—directly in your terminal. Unlike traditional Markdown viewers that style text with ANSI codes, Lumen treats rendering as a proper layout problem: Markdown → IR → Layout → Render.

---

## Installation

```bash
# Build the project
cargo build --release

# The binary will be at:
./target/release/lumen
```

## Usage

```bash
# View a single file
lumen README.md

# View multiple files (use Tab to switch between them)
lumen file1.md file2.md file3.md

# Use a different theme
lumen README.md neon

# View all markdown files in current directory
lumen *.md
```

### Available Themes

- `docs` — Clean, documentation-focused (default)
- `neon` — Vibrant, modern with bright colors
- `minimal` — Low visual noise, ASCII fallbacks

---

## Keyboard Shortcuts

### Navigation
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

### Links & Navigation
| Key | Action |
|-----|--------|
| `a` | Cycle through links (for table of contents navigation) |
| `Enter` | Follow selected link (jump to anchor) |

### File Management
| Key | Action |
|-----|--------|
| `Tab` | Switch to next file (when multiple files are open) |
| `Shift+Tab` | Switch to previous file |

### General
| Key | Action |
|-----|--------|
| `q` / `Esc` | Quit |

---

## Features

### Markdown Support
- Full GFM (GitHub Flavored Markdown) support
- Headings, paragraphs, lists (including deep nesting 4+ levels)
- Tables with accurate border rendering
- Code blocks, task lists, strikethrough
- Links, images, blockquotes with nesting
- Proper tight list handling for correct structure

### Theming
- 3 built-in themes: Docs (default), Neon, Minimal
- Color palettes with RGB/ANSI fallbacks
- Multiple border styles: Single, Double, Rounded, Heavy, ASCII
- Typography and spacing configuration via YAML

### Layout & Rendering
- Vertical flow layout with proper margins
- Smart text wrapping (word-boundary + long-word breaking)
- Table layout with column distribution
- Rich colors (24-bit RGB, 256-color, 16-color)
- Box-drawing characters for borders
- Double-buffered rendering for smooth scrolling

### Interactive Navigation
- Keyboard-driven navigation (vim-style bindings)
- Link cycling and anchor jumping for table of contents
- Visual link highlighting for selected links
- Multi-file support with tab switching
- Status bar with position indicator and file name
- Hit regions for interactive elements

---

## Architecture

Lumen uses a multi-stage rendering pipeline:

```
Markdown → IR (pulldown-cmark) → Theme + Layout → Terminal Renderer (Ratatui)
```

**Key design decisions:**
- Stable IR prevents downstream churn when parser changes
- Token-based theming (no CSS selectors, just element-type styling)
- Two-phase layout: measure intrinsic sizes, then assign positions
- Character-grid coordinates for natural terminal rendering

---

## Project Structure

```
Lumen/
├── src/
│   ├── ir/           # Intermediate representation
│   ├── parser/       # Markdown → IR
│   ├── theme/        # Theming system
│   ├── layout/       # Layout engine
│   ├── render/       # Terminal renderer
│   ├── lib.rs        # Public API
│   └── main.rs       # Interactive viewer binary
├── themes/           # YAML theme files
└── examples/         # Demo programs
```

---

## Dependencies

- [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark) — Fast, spec-compliant Markdown parser
- [Ratatui](https://github.com/ratatui/ratatui) — Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) — Cross-platform terminal manipulation

---

## Development

### Running Tests

```bash
cargo test
```

### Examples

```bash
# Theme demo
cargo run --example themes

# Layout demo
cargo run --example layout
```

---

## License

MIT
