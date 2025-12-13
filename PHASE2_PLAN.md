# Phase 2: CSS Theming Layer

## Goal

Define a CSS-like theme system that drives layout + visuals (spacing, borders, callouts) without the complexity of full CSS parsing.

**Key principle**: Themes are **declarative token sets**, not arbitrary CSS.

---

## Theme Architecture

### Design Philosophy

1. **Tokens, not selectors**: Element types map directly to styles (no CSS selectors)
2. **Terminal-native**: Colors, box-drawing characters, spacing units
3. **Composable**: Themes override defaults, not replace them
4. **Type-safe**: Rust types enforce valid theme definitions

### Theme Structure

```rust
pub struct Theme {
    pub name: String,
    pub colors: ColorPalette,
    pub typography: Typography,
    pub spacing: Spacing,
    pub blocks: BlockStyles,
    pub inlines: InlineStyles,
}
```

---

## Supported Properties

### Colors

```rust
pub struct ColorPalette {
    pub foreground: Color,
    pub background: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub muted: Color,
    pub error: Color,
    pub warning: Color,
    pub success: Color,
}

pub enum Color {
    Reset,
    Rgb(u8, u8, u8),      // True color
    Ansi256(u8),          // 256-color palette
    Ansi16(AnsiColor),    // 16-color fallback
}
```

### Typography

```rust
pub struct Typography {
    pub base_size: u8,           // Base "size" (weight for emphasis)
    pub code_font: CodeFontStyle,
    pub emphasis: EmphasisStyle,
}

pub enum EmphasisStyle {
    Native,              // Use terminal bold/italic if available
    ColorShift,          // Use color change
    BackgroundBand,      // Use background highlight
}
```

### Spacing

```rust
pub struct Spacing {
    pub paragraph_spacing: u16,  // Lines between paragraphs
    pub heading_margin_top: u16,
    pub heading_margin_bottom: u16,
    pub list_indent: u16,        // Columns
    pub blockquote_indent: u16,
    pub code_block_padding: u16,
}
```

### Block Styles

```rust
pub struct BlockStyles {
    pub heading: [HeadingStyle; 6],  // H1-H6
    pub paragraph: ParagraphStyle,
    pub code_block: CodeBlockStyle,
    pub blockquote: BlockQuoteStyle,
    pub list: ListStyle,
    pub table: TableStyle,
    pub horizontal_rule: HorizontalRuleStyle,
    pub callout: CalloutStyles,
}

pub struct HeadingStyle {
    pub color: Color,
    pub background: Option<Color>,
    pub border: Option<Border>,
    pub padding: (u16, u16),      // (vertical, horizontal)
    pub margin: (u16, u16),       // (top, bottom)
    pub prefix: Option<String>,   // e.g., "# " or "▶ "
}

pub struct CodeBlockStyle {
    pub background: Color,
    pub foreground: Color,
    pub border: BorderStyle,
    pub padding: (u16, u16),
    pub show_language_badge: bool,
}

pub struct TableStyle {
    pub border_style: BorderStyle,
    pub header_background: Option<Color>,
    pub header_foreground: Option<Color>,
    pub row_separator: bool,
    pub padding: u16,
}
```

### Inline Styles

```rust
pub struct InlineStyles {
    pub strong: TextStyle,
    pub emphasis: TextStyle,
    pub code: TextStyle,
    pub link: LinkStyle,
    pub strikethrough: TextStyle,
}

pub struct TextStyle {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub weight: FontWeight,
    pub style: FontStyle,
}

pub struct LinkStyle {
    pub foreground: Color,
    pub underline: bool,
    pub show_url: UrlDisplayMode,
}

pub enum UrlDisplayMode {
    Inline,      // Show URL after text: [text](url)
    Hover,       // Show on hover only (if terminal supports)
    Hidden,      // Don't show URL
}
```

### Borders

```rust
pub enum BorderStyle {
    None,
    Single,      // ┌─┐ │ └─┘
    Double,      // ╔═╗ ║ ╚═╝
    Rounded,     // ╭─╮ │ ╰─╯
    Heavy,       // ┏━┓ ┃ ┗━┛
    Ascii,       // +--+ | +--+
}

pub struct Border {
    pub style: BorderStyle,
    pub color: Option<Color>,
    pub sides: BorderSides,  // top, right, bottom, left
}
```

---

## Theme File Format (YAML)

```yaml
name: "Docs"
version: "1.0"

colors:
  foreground: { rgb: [220, 220, 220] }
  background: { rgb: [30, 30, 30] }
  primary: { rgb: [100, 180, 255] }
  accent: { rgb: [255, 200, 100] }

typography:
  emphasis: Native

spacing:
  paragraph_spacing: 1
  heading_margin_top: 2
  heading_margin_bottom: 1
  list_indent: 2
  blockquote_indent: 2

blocks:
  heading:
    h1:
      color: { rgb: [100, 180, 255] }
      border:
        style: Double
        sides: [bottom]
      margin: [2, 1]
    h2:
      color: { rgb: [100, 180, 255] }
      border:
        style: Single
        sides: [bottom]
      margin: [2, 1]
    h3:
      color: { rgb: [150, 200, 255] }
      margin: [1, 1]

  code_block:
    background: { rgb: [40, 40, 40] }
    border:
      style: Rounded
      color: { rgb: [80, 80, 80] }
    padding: [1, 2]
    show_language_badge: true

  table:
    border_style: Single
    header_background: { rgb: [50, 50, 80] }
    row_separator: true

inlines:
  strong:
    foreground: { rgb: [255, 255, 255] }
    weight: Bold

  emphasis:
    foreground: { rgb: [200, 200, 255] }
    style: Italic

  code:
    background: { rgb: [50, 50, 50] }
    foreground: { rgb: [255, 150, 100] }

  link:
    foreground: { rgb: [100, 180, 255] }
    underline: true
    show_url: Hover
```

---

## Implementation Plan

### Step 1: Theme Types

Create `src/theme/` module with:
- `types.rs` — Core theme types
- `color.rs` — Color handling (RGB, ANSI256, ANSI16)
- `defaults.rs` — Default theme values

### Step 2: Theme Parser

- Add `serde_yaml` dependency
- Implement `Theme::from_yaml()`
- Validation + error handling

### Step 3: Default Themes

Create 3 built-in themes:

**Docs** (documentation-focused):
- Clean, high contrast
- Clear heading hierarchy
- Subtle borders
- Optimized for readability

**Neon** (vibrant, modern):
- Bright accent colors
- Rounded borders
- Visual "pop"
- Good for demos/presentations

**Minimal** (stripped-down):
- Monochrome or near-monochrome
- ASCII borders fallback
- Maximum compatibility
- Low visual noise

### Step 4: Style Application

Create `src/styled/` module:
```rust
pub struct StyledDocument {
    pub document: Document,
    pub theme: Theme,
    pub styles: StyleMap,
}

// Map from IR node to computed styles
pub struct StyleMap {
    block_styles: HashMap<BlockId, ComputedBlockStyle>,
    inline_styles: HashMap<InlineId, ComputedInlineStyle>,
}
```

### Step 5: Computed Styles

Resolve theme tokens to concrete values:
- Inherit from parent contexts
- Handle terminal capability fallbacks
- Cache computed results

---

## Success Criteria

Before exiting Phase 2:

✅ Theme type system defined and documented
✅ YAML theme parser working
✅ 3 default themes implemented (Docs, Neon, Minimal)
✅ Theme validation (reject invalid colors, borders, etc.)
✅ Default theme applied to IR successfully
✅ Tests for theme loading and application

---

## Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| **Theme complexity creep** | Hard-limit properties; no CSS selectors or cascading |
| **Color fallback complexity** | Pre-compute fallback palettes for each theme |
| **YAML parsing fragility** | Strict validation; fail early with clear errors |
| **Performance (style lookup)** | Cache computed styles; don't recompute per-render |

---

## Non-Goals (Phase 2)

❌ **Syntax highlighting** — Delegate to external tools (Phase 6)
❌ **Custom fonts** — Terminal fonts only
❌ **Animations** — Static rendering
❌ **User-defined theme compilation** — v1 uses built-in themes only (user themes in v2)
