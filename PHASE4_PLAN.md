# Phase 4: Terminal Renderer

## Goal

Render the layout tree to the terminal with crisp typography, smooth scrolling, and zero jitter.

**Key principle**: Text-first, fast rendering with graceful fallbacks.

---

## Architecture

```
Layout Tree + Theme + Viewport → Renderer → Terminal Buffer → Display
                                      ↓
                                  ANSI Codes
                                      ↓
                              Box Drawing Characters
```

---

## Rendering Strategy

### Backend Choice: Ratatui

**Why Ratatui**:
- Most popular Rust TUI framework (actively maintained)
- Double-buffering (renders only changes)
- Terminal capability detection
- Cross-platform (macOS, Linux, Windows)
- Low-level control when needed
- Good event handling (keyboard, mouse)

**What we use from Ratatui**:
- Terminal initialization and cleanup
- Buffer management (double-buffering)
- Event polling (keyboard, resize)
- ANSI escape sequence generation
- Unicode support

**What we implement ourselves**:
- Layout tree traversal
- Style application from theme
- Border rendering logic
- Text wrapping (already done in Phase 3)
- Hit testing (already done in Phase 3)

---

## Rendering Pipeline

### Phase 1: Prepare

1. **Initialize terminal**: Raw mode, alternate screen
2. **Create buffer**: Double-buffer for flicker-free rendering
3. **Apply viewport**: Calculate visible region

### Phase 2: Render

1. **Clear buffer**: Reset to background color
2. **Traverse layout tree**: Depth-first
3. **For each node**:
   - Check if visible (viewport clipping)
   - Apply theme styles (colors, weight, style)
   - Render borders (if any)
   - Render content (text, tables, etc.)
   - Render children (recursive)

### Phase 3: Present

1. **Compute diff**: Compare current buffer to previous
2. **Emit ANSI codes**: Only for changed cells
3. **Flush to terminal**: Write escape sequences
4. **Swap buffers**: Prepare for next frame

---

## ANSI Rendering

### Color Codes

```rust
pub fn set_foreground(color: Color) -> String {
    match color {
        Color::Reset => "\x1b[39m".to_string(),
        Color::Rgb(r, g, b) => format!("\x1b[38;2;{};{};{}m", r, g, b),
        Color::Ansi256(idx) => format!("\x1b[38;5;{}m", idx),
        Color::Ansi(ansi) => format!("\x1b[{}m", ansi_fg_code(ansi)),
    }
}

pub fn set_background(color: Color) -> String {
    match color {
        Color::Reset => "\x1b[49m".to_string(),
        Color::Rgb(r, g, b) => format!("\x1b[48;2;{};{};{}m", r, g, b),
        Color::Ansi256(idx) => format!("\x1b[48;5;{}m", idx),
        Color::Ansi(ansi) => format!("\x1b[{}m", ansi_bg_code(ansi)),
    }
}
```

### Text Styles

```rust
pub fn set_bold() -> &'static str { "\x1b[1m" }
pub fn set_italic() -> &'static str { "\x1b[3m" }
pub fn set_underline() -> &'static str { "\x1b[4m" }
pub fn reset_all() -> &'static str { "\x1b[0m" }
```

### Cursor Movement

```rust
pub fn move_to(x: u16, y: u16) -> String {
    format!("\x1b[{};{}H", y + 1, x + 1) // ANSI is 1-indexed
}

pub fn hide_cursor() -> &'static str { "\x1b[?25l" }
pub fn show_cursor() -> &'static str { "\x1b[?25h" }
```

---

## Border Rendering

### Box-Drawing Characters

```rust
pub enum BorderStyle {
    Single,  // ┌─┐ │ └─┘
    Double,  // ╔═╗ ║ ╚═╝
    Rounded, // ╭─╮ │ ╰─╯
    Heavy,   // ┏━┓ ┃ ┗━┛
    Ascii,   // +--+ | +--+
}

impl BorderStyle {
    pub fn chars(&self) -> BorderChars {
        match self {
            Single => BorderChars {
                top_left: '┌', top: '─', top_right: '┐',
                left: '│', right: '│',
                bottom_left: '└', bottom: '─', bottom_right: '┘',
            },
            Double => BorderChars {
                top_left: '╔', top: '═', top_right: '╗',
                left: '║', right: '║',
                bottom_left: '╚', bottom: '═', bottom_right: '╝',
            },
            // ... other styles
        }
    }
}
```

### Border Rendering Logic

```rust
fn render_border(
    rect: Rectangle,
    style: BorderStyle,
    color: Option<Color>,
    sides: &[BorderSide],
) {
    let chars = style.chars();

    // Top border
    if sides.contains(&BorderSide::Top) {
        for x in rect.x..rect.x + rect.width {
            if x == rect.x {
                put_char(x, rect.y, chars.top_left, color);
            } else if x == rect.x + rect.width - 1 {
                put_char(x, rect.y, chars.top_right, color);
            } else {
                put_char(x, rect.y, chars.top, color);
            }
        }
    }

    // Left and right borders
    for y in rect.y + 1..rect.y + rect.height - 1 {
        if sides.contains(&BorderSide::Left) {
            put_char(rect.x, y, chars.left, color);
        }
        if sides.contains(&BorderSide::Right) {
            put_char(rect.x + rect.width - 1, y, chars.right, color);
        }
    }

    // Bottom border
    if sides.contains(&BorderSide::Bottom) {
        // ... similar to top
    }
}
```

---

## Viewport Clipping

### Visibility Testing

```rust
fn is_visible(node_rect: Rectangle, viewport: &Viewport) -> bool {
    let visible_rect = viewport.visible_rect();
    node_rect.intersects(&visible_rect)
}

fn clip_to_viewport(
    node_rect: Rectangle,
    viewport: &Viewport,
) -> Option<Rectangle> {
    let visible = viewport.visible_rect();

    // Calculate intersection
    let x1 = node_rect.x.max(visible.x);
    let y1 = node_rect.y.max(visible.y);
    let x2 = (node_rect.x + node_rect.width).min(visible.x + visible.width);
    let y2 = (node_rect.y + node_rect.height).min(visible.y + visible.height);

    if x2 > x1 && y2 > y1 {
        Some(Rectangle::new(x1, y1, x2 - x1, y2 - y1))
    } else {
        None
    }
}
```

---

## Rendering Elements

### Headings

```rust
fn render_heading(
    node: &LayoutNode,
    theme: &Theme,
    viewport: &Viewport,
) {
    if let LayoutElement::Heading { level, text } = &node.element {
        let style = get_heading_style(*level, theme);

        // Render border (if any)
        if let Some(border) = &style.border {
            render_border(node.rect, border.style, border.color, &border.sides);
        }

        // Render prefix (e.g., "# " or "▶ ")
        let mut x = node.rect.x;
        if let Some(prefix) = &style.prefix {
            render_text(x, node.rect.y, prefix, style.color);
            x += prefix.len() as u16;
        }

        // Render text
        render_text(x, node.rect.y, text, style.color);
    }
}
```

### Paragraphs

```rust
fn render_paragraph(
    node: &LayoutNode,
    theme: &Theme,
    viewport: &Viewport,
) {
    if let LayoutElement::Paragraph { lines } = &node.element {
        let mut y = node.rect.y;

        for line in lines {
            let mut x = node.rect.x;

            for segment in &line.segments {
                render_styled_text(
                    x,
                    y,
                    &segment.text,
                    &segment.style,
                    theme,
                );
                x += segment.text.len() as u16;
            }

            y += 1;
        }
    }
}
```

### Code Blocks

```rust
fn render_code_block(
    node: &LayoutNode,
    theme: &Theme,
    viewport: &Viewport,
) {
    if let LayoutElement::CodeBlock { lang, lines } = &node.element {
        let style = &theme.blocks.code_block;

        // Render border
        if let Some(border) = &style.border {
            render_border(node.rect, border.style, border.color, &border.sides);
        }

        // Render language badge (top-right corner)
        if style.show_language_badge {
            if let Some(lang) = lang {
                let badge = format!(" {} ", lang);
                let x = node.rect.x + node.rect.width - badge.len() as u16 - 1;
                render_text(x, node.rect.y, &badge, theme.colors.accent);
            }
        }

        // Render code lines
        let padding = theme.spacing.code_block_padding;
        let mut y = node.rect.y + padding;

        for line in lines {
            render_styled_text(
                node.rect.x + padding,
                y,
                line,
                &TextStyle {
                    foreground: Some(style.foreground),
                    background: Some(style.background),
                    ..Default::default()
                },
                theme,
            );
            y += 1;
        }
    }
}
```

### Tables

```rust
fn render_table(
    node: &LayoutNode,
    theme: &Theme,
    viewport: &Viewport,
) {
    let style = &theme.blocks.table;

    // Render table borders
    render_table_border(node.rect, style.border_style, theme);

    // Render rows (children)
    for child in &node.children {
        render_table_row(child, theme, viewport);
    }
}
```

---

## Event Handling

### Keyboard Navigation

```rust
pub enum Action {
    Quit,
    ScrollUp,
    ScrollDown,
    PageUp,
    PageDown,
    Home,
    End,
    Search,
    NextLink,
    PrevLink,
}

pub fn handle_key(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => Some(Action::Quit),
        KeyCode::Up | KeyCode::Char('k') => Some(Action::ScrollUp),
        KeyCode::Down | KeyCode::Char('j') => Some(Action::ScrollDown),
        KeyCode::PageUp => Some(Action::PageUp),
        KeyCode::PageDown => Some(Action::PageDown),
        KeyCode::Home => Some(Action::Home),
        KeyCode::End => Some(Action::End),
        KeyCode::Char('/') => Some(Action::Search),
        KeyCode::Tab => Some(Action::NextLink),
        KeyCode::BackTab => Some(Action::PrevLink),
        _ => None,
    }
}
```

---

## Smooth Scrolling

### Animation

```rust
pub struct ScrollAnimation {
    from: u16,
    to: u16,
    duration: Duration,
    start_time: Instant,
}

impl ScrollAnimation {
    pub fn current_position(&self, now: Instant) -> u16 {
        let elapsed = now.duration_since(self.start_time);
        if elapsed >= self.duration {
            return self.to;
        }

        let progress = elapsed.as_millis() as f32 / self.duration.as_millis() as f32;
        let eased = ease_out_cubic(progress);

        self.from + ((self.to - self.from) as f32 * eased) as u16
    }
}

fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}
```

---

## Implementation Plan

### Step 1: Renderer Module

Create `src/render/` module:
- `ansi.rs` — ANSI escape sequence generation
- `border.rs` — Box-drawing character rendering
- `renderer.rs` — Main rendering logic
- `terminal.rs` — Terminal initialization/cleanup

### Step 2: ANSI Codes

Implement color and style escape sequences:
- RGB, ANSI256, ANSI16 color codes
- Bold, italic, underline
- Cursor movement
- Screen clearing

### Step 3: Border Rendering

Implement box-drawing:
- BorderChars for each style
- Corner, edge, and side rendering
- Partial border support (top/bottom/left/right)

### Step 4: Element Rendering

Implement rendering for each element type:
- Headings (with borders, prefixes)
- Paragraphs (with wrapped lines)
- Code blocks (with borders, badges)
- Tables (with grid borders)
- Lists (with markers)
- Blockquotes (with left border)

### Step 5: Main Render Loop

Implement event loop:
- Initialize terminal
- Render layout tree
- Handle keyboard events
- Update viewport
- Re-render on changes
- Cleanup on exit

### Step 6: Interactive Viewer

Create binary:
- Parse markdown file from args
- Apply theme (from args or default)
- Layout document
- Enter render loop
- Handle navigation
- Exit cleanly

---

## Success Criteria

Before exiting Phase 4:

✅ Terminal renderer working (displays all element types)
✅ Colors and styles applied from theme
✅ Borders rendered correctly
✅ Smooth scrolling working
✅ Keyboard navigation working
✅ Viewport clipping working
✅ Clean terminal initialization/cleanup
✅ Interactive viewer binary

---

## Non-Goals (Phase 4)

❌ **Syntax highlighting** — Defer to Phase 6
❌ **Images** — Defer to Phase 5
❌ **Mouse support** — Keyboard-first in v1
❌ **Search** — Defer to Phase 6
❌ **Animations** (beyond smooth scroll) — Keep it simple
