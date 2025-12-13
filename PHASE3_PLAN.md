# Phase 3: Layout Engine

## Goal

Compute a layout tree (positions + sizes) from IR + Theme, producing a structure ready for terminal rendering.

**Key principle**: Browser-like layout without browser complexity.

---

## Architecture

```
Document (IR) + Theme → Layout Engine → Layout Tree → Renderer
                           ↓
                    Viewport/Scroll
                           ↓
                      Hit Testing
```

---

## Layout Model

### Core Concepts

1. **Flow Layout**: Blocks stack vertically, inlines flow horizontally
2. **Box Model**: Each element has content, padding, border, margin
3. **Constraints**: Parent width constrains child width; child height informs parent
4. **Wrapping**: Text wraps at word boundaries within available width
5. **Positioning**: Absolute coordinates computed from layout tree

### Layout Types

```rust
pub struct LayoutTree {
    pub root: LayoutNode,
    pub viewport: Viewport,
}

pub struct LayoutNode {
    pub id: NodeId,
    pub rect: Rectangle,           // Position + size
    pub element: LayoutElement,    // What kind of element
    pub children: Vec<LayoutNode>,
    pub style: ComputedStyle,      // Resolved from theme
}

pub struct Rectangle {
    pub x: u16,      // Column
    pub y: u16,      // Row
    pub width: u16,  // Columns
    pub height: u16, // Rows
}

pub enum LayoutElement {
    Document,
    Heading { level: u8, text: String },
    Paragraph { lines: Vec<Line> },
    CodeBlock { lang: Option<String>, lines: Vec<String> },
    BlockQuote { children: Vec<LayoutNode> },
    List { ordered: bool, items: Vec<LayoutNode> },
    ListItem { marker: String, content: Vec<LayoutNode> },
    Table { headers: Vec<Cell>, rows: Vec<Vec<Cell>> },
    HorizontalRule,
}

pub struct Line {
    pub segments: Vec<TextSegment>,
}

pub struct TextSegment {
    pub text: String,
    pub style: TextStyle,
}
```

---

## Layout Phases

### Phase 1: Measure

Compute intrinsic sizes (minimum and preferred):
- **Block elements**: Width = parent width, height = sum of children + spacing
- **Inline elements**: Width = text length (with wrapping), height = line count
- **Tables**: Width = sum of column widths, height = sum of row heights

### Phase 2: Position

Assign absolute coordinates:
- **Vertical flow**: Each block starts after previous block + margin
- **Inline flow**: Segments flow left-to-right, wrap at width boundary
- **Tables**: Grid layout with column widths and row heights

### Phase 3: Finalize

Apply borders, padding, compute hit regions:
- **Borders**: Add border characters around content
- **Padding**: Add empty space inside borders
- **Hit regions**: Track clickable areas (links, code blocks)

---

## Vertical Flow Layout

### Algorithm

```rust
fn layout_blocks(blocks: &[Block], width: u16, theme: &Theme) -> Vec<LayoutNode> {
    let mut nodes = Vec::new();
    let mut y = 0;

    for block in blocks {
        let node = layout_block(block, 0, y, width, theme);
        y += node.rect.height + spacing_after(block, theme);
        nodes.push(node);
    }

    nodes
}
```

### Block Spacing Rules

```rust
fn spacing_after(block: &Block, theme: &Theme) -> u16 {
    match block {
        Block::Paragraph { .. } => theme.spacing.paragraph_spacing,
        Block::Heading { .. } => theme.spacing.heading_margin_bottom,
        Block::CodeBlock { .. } => theme.spacing.code_block_padding,
        // ...
        _ => 1,
    }
}
```

---

## Inline Layout (Text Wrapping)

### Algorithm

```rust
fn layout_text(inlines: &[Inline], max_width: u16) -> Vec<Line> {
    let mut lines = Vec::new();
    let mut current_line = Line::new();
    let mut current_width = 0;

    for inline in inlines {
        match inline {
            Inline::Text(text) => {
                for word in text.split_whitespace() {
                    let word_width = word.len() as u16;

                    if current_width + word_width > max_width {
                        // Wrap to next line
                        lines.push(current_line);
                        current_line = Line::new();
                        current_width = 0;
                    }

                    current_line.add_segment(word, style);
                    current_width += word_width + 1; // +1 for space
                }
            }
            Inline::Strong(nested) => {
                // Recursively layout with updated style
                layout_text(nested, max_width - current_width);
            }
            // ...
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}
```

### Word Breaking Rules

1. **Prefer word boundaries**: Break at whitespace
2. **Long words**: If single word > max_width, break mid-word
3. **Preserve spaces**: Single space between words
4. **Collapse whitespace**: Multiple spaces → single space

---

## Table Layout

### Column Width Algorithm

```rust
pub enum ColumnWidth {
    Fixed(u16),          // Absolute width in columns
    Proportional(f32),   // Fraction of available width (0.0-1.0)
    Auto,                // Fit to content
}

fn compute_column_widths(
    table: &Table,
    available_width: u16,
    theme: &Theme,
) -> Vec<u16> {
    // Phase 1: Compute content widths
    let content_widths = measure_table_columns(table);

    // Phase 2: Apply constraints
    let mut widths = Vec::new();
    let mut remaining_width = available_width;
    let mut auto_columns = Vec::new();

    for (i, alignment) in table.alignment.iter().enumerate() {
        match get_column_width_hint(alignment) {
            ColumnWidth::Fixed(w) => {
                widths.push(w);
                remaining_width -= w;
            }
            ColumnWidth::Proportional(ratio) => {
                let w = (available_width as f32 * ratio) as u16;
                widths.push(w);
                remaining_width -= w;
            }
            ColumnWidth::Auto => {
                auto_columns.push(i);
                widths.push(0); // Placeholder
            }
        }
    }

    // Phase 3: Distribute remaining width to auto columns
    if !auto_columns.is_empty() {
        let width_per_col = remaining_width / auto_columns.len() as u16;
        for i in auto_columns {
            widths[i] = width_per_col.max(content_widths[i]);
        }
    }

    widths
}
```

### Table Layout Rules (v1)

1. **No spanning**: Cells don't span multiple rows/columns
2. **No nesting**: Tables don't contain tables
3. **Fixed headers**: Header row always visible (no horizontal scroll)
4. **Equal distribution**: Auto columns split remaining width equally

---

## Viewport and Scrolling

### Viewport Model

```rust
pub struct Viewport {
    pub width: u16,      // Terminal width (columns)
    pub height: u16,     // Terminal height (rows)
    pub scroll_x: u16,   // Horizontal scroll offset
    pub scroll_y: u16,   // Vertical scroll offset
}

impl Viewport {
    pub fn visible_rect(&self) -> Rectangle {
        Rectangle {
            x: self.scroll_x,
            y: self.scroll_y,
            width: self.width,
            height: self.height,
        }
    }

    pub fn scroll_to(&mut self, y: u16) {
        self.scroll_y = y;
    }

    pub fn scroll_by(&mut self, delta_y: i16) {
        self.scroll_y = (self.scroll_y as i16 + delta_y).max(0) as u16;
    }

    pub fn contains_point(&self, x: u16, y: u16) -> bool {
        x >= self.scroll_x
            && x < self.scroll_x + self.width
            && y >= self.scroll_y
            && y < self.scroll_y + self.height
    }
}
```

### Scroll Behavior

1. **Vertical only** (v1): No horizontal scrolling
2. **Line-based**: Scroll by line (not pixel)
3. **Bounds checking**: Can't scroll past document end
4. **Smooth scrolling**: Animate in Phase 4 (renderer)

---

## Hit Testing

### Hit Region Tracking

```rust
pub struct HitRegion {
    pub rect: Rectangle,
    pub element: HitElement,
}

pub enum HitElement {
    Link { url: String },
    CodeBlock { lang: Option<String> },
    Heading { level: u8, id: String },
}

impl LayoutTree {
    pub fn hit_test(&self, x: u16, y: u16) -> Option<&HitRegion> {
        // Find deepest node containing (x, y)
        self.find_region_at(x, y)
    }

    fn find_region_at(&self, x: u16, y: u16) -> Option<&HitRegion> {
        // Traverse layout tree depth-first
        // Return first hit region containing point
    }
}
```

### Interactive Elements

1. **Links**: Entire link text is clickable
2. **Code blocks**: Entire block is selectable
3. **Headings**: Clickable for anchor navigation

---

## Implementation Plan

### Step 1: Layout Types

Create `src/layout/` module:
- `types.rs` — LayoutNode, Rectangle, Viewport
- `measure.rs` — Intrinsic size computation
- `position.rs` — Coordinate assignment
- `text.rs` — Inline layout and wrapping
- `table.rs` — Table layout
- `hit.rs` — Hit testing

### Step 2: Vertical Flow

Implement block stacking:
- Heading layout
- Paragraph layout
- Code block layout
- Blockquote layout (recursive)
- List layout

### Step 3: Inline Layout

Implement text wrapping:
- Text segmentation
- Word breaking
- Style application
- Line assembly

### Step 4: Table Layout

Implement grid layout:
- Column width computation
- Row height computation
- Cell positioning
- Border rendering

### Step 5: Viewport + Scroll

Implement scroll model:
- Viewport bounds
- Scroll offset tracking
- Visible region calculation
- Clipping

### Step 6: Hit Testing

Implement interaction:
- Build hit region map
- Point-in-rectangle tests
- Region lookup

---

## Success Criteria

Before exiting Phase 3:

✅ Layout tree generated from IR + Theme
✅ Vertical flow layout working (blocks stack correctly)
✅ Inline layout working (text wraps at boundaries)
✅ Table layout working (columns sized appropriately)
✅ Viewport and scrolling working
✅ Hit testing returns correct elements
✅ Tests for all layout algorithms

---

## Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| **Table layout complexity** | Cap v1: no spanning, no nesting, equal auto-width |
| **Text wrapping edge cases** | Comprehensive test corpus with long words, CJK, etc. |
| **Performance on large docs** | Lazy layout; only compute visible nodes |
| **Coordinate precision** | Use u16 (fits terminal dimensions); document max size |

---

## Non-Goals (Phase 3)

❌ **Horizontal scrolling** — v1 is vertical-only
❌ **Constraint solvers** — Use simple proportional/fixed widths
❌ **Floating elements** — No floats or absolute positioning
❌ **Multi-column layout** — Single column only
❌ **Bidirectional text** — LTR only in v1
❌ **Pixel-perfect positioning** — Character grid only
