# Phase 0: Scope + Constraints

**Duration**: 1–2 days
**Goal**: Lock the "browser-like" target without accidentally building a full browser.

---

## 1. Rendering Contract

### HTML Subset Support (v1)

We support a **minimal, deterministic subset** of HTML that maps cleanly to terminal primitives:

#### Block-level elements
- `<p>` — Paragraph (vertical spacing)
- `<h1>` through `<h6>` — Headings (hierarchy + visual weight)
- `<blockquote>` — Quoted blocks (indentation + optional styling)
- `<pre>` — Preformatted text blocks
- `<ul>`, `<ol>`, `<li>` — Lists (ordered and unordered)
- `<table>`, `<thead>`, `<tbody>`, `<tr>`, `<th>`, `<td>` — Tables (fixed/proportional column sizing)
- `<div>` — Generic container (for callouts, panels, etc.)
- `<hr>` — Horizontal rule / separator

#### Inline elements
- `<strong>`, `<b>` — Bold emphasis
- `<em>`, `<i>` — Italic emphasis
- `<code>` — Inline code
- `<a>` — Links (href tracking for navigation)
- `<span>` — Generic inline container (for color/styling)
- `<br>` — Line break

#### Media
- `<img>` — Images (inline display via terminal protocols)

#### Special markdown extensions
- Fenced code blocks with language hints (```lang)
- GitHub-flavored callouts/admonitions
- Task lists (`[ ]` / `[x]`)

---

### CSS Subset Support (v1)

We support a **theme-driven CSS-like system**, not full CSS parsing:

#### Supported properties
- **Spacing**: `margin`, `padding` (top/bottom/left/right)
- **Colors**: `color`, `background-color` (16-color, 256-color, or true-color palettes)
- **Borders**: `border`, `border-style` (single/double/rounded box-drawing characters)
- **Typography**: `font-weight` (normal/bold), `font-style` (normal/italic)
- **Text**: `text-align` (left/center/right for headings/tables)

#### Theming model
- Themes are **declarative token sets** (not arbitrary CSS)
- Each element type has a default style mapping
- Themes override defaults via a simple key-value format

Example theme structure:
```yaml
h1:
  color: cyan
  border-bottom: double
  margin-bottom: 1

code:
  background-color: gray-dark
  padding: 0 1

table:
  border-style: single
  header-background: blue
```

---

### Anti-goals (what we explicitly DO NOT support)

❌ **JavaScript / DOM manipulation** — No scripting layer
❌ **CSS layout engines** (flexbox, grid, float) — We use a custom flow layout model
❌ **Forms / interactive widgets** — Read-only document rendering only
❌ **Video / audio** — Static images only
❌ **Custom fonts / web fonts** — Terminal fonts only
❌ **Complex selectors** — Only element-type styling (no class/ID/descendant selectors in v1)
❌ **Network fetching** — Offline-first; external resources must be pre-downloaded

---

## 2. Capability Matrix

### Terminal Feature Detection

We optimize for **iTerm2 on macOS** but gracefully degrade for other environments.

| Feature | iTerm2 | Kitty | WezTerm | Alacritty | Tmux | Fallback |
|---------|--------|-------|---------|-----------|------|----------|
| **True color (24-bit RGB)** | ✅ | ✅ | ✅ | ✅ | ✅ | 256-color palette |
| **Inline images (iTerm2 protocol)** | ✅ | ❌ | ✅ | ❌ | ⚠️ | ASCII art placeholder |
| **Kitty graphics protocol** | ❌ | ✅ | ✅ | ❌ | ❌ | ASCII art placeholder |
| **SIXEL graphics** | ❌ | ❌ | ❌ | ❌ | ⚠️ | ASCII art placeholder |
| **Italic text** | ✅ | ✅ | ✅ | ✅ | ✅ | Background band emphasis |
| **Bold text** | ✅ | ✅ | ✅ | ✅ | ✅ | Color shift |
| **Clickable links (OSC 8)** | ✅ | ✅ | ✅ | ❌ | ❌ | Show URL inline |
| **Unicode box drawing** | ✅ | ✅ | ✅ | ✅ | ✅ | ASCII fallback (`+--\|`) |
| **Cursor positioning** | ✅ | ✅ | ✅ | ✅ | ✅ | Required |
| **Alternate screen buffer** | ✅ | ✅ | ✅ | ✅ | ✅ | Required |

### Detection strategy

1. **Query terminal capabilities** via:
   - `$TERM` environment variable
   - `$TERM_PROGRAM` (e.g., `iTerm.app`)
   - `terminfo` / `termcap` queries
   - Feature probe sequences (e.g., CSI queries for color support)

2. **Graceful degradation**:
   - If iTerm2 inline images fail → try Kitty protocol → try SIXEL → fall back to `[IMAGE: filename.png]`
   - If italic not supported → use background color band behind text
   - If Unicode box-drawing fails → use ASCII art borders

3. **Manual override**:
   - Config file option: `--terminal-mode=<iTerm2|kitty|basic|ascii>`

---

## 3. Scope Boundaries

### In Scope (Phase 0 → Phase 7)
✅ Markdown → HTML → Layout → Terminal rendering pipeline
✅ Smooth scrolling, search, outline navigation
✅ Inline images (via terminal protocols)
✅ Theme system with pre-built themes
✅ Tables with fixed/proportional column sizing
✅ Code block syntax awareness (language badges, no highlighting in v1)
✅ Link tracking (navigation, not execution)

### Out of Scope (v1)
⛔ Syntax highlighting (delegate to external tools like `bat` or `delta`)
⛔ Live reload / file watching (run as one-shot viewer)
⛔ Network requests (input is local files only)
⛔ Plugins / extension system
⛔ WYSIWYG editing

### Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| **Scope creep into full browser** | Hard-code anti-goals; reject JS/DOM/network features |
| **Table layout complexity** | Cap v1 to fixed/proportional columns; defer spanning/nesting |
| **Terminal compatibility hell** | Feature detection + fallback matrix + manual override |
| **Image protocol edge cases** | Test matrix for tmux/SSH; fail gracefully with placeholders |
| **Performance with large docs** | Lazy rendering; viewport-based culling; image caching |

---

## 4. Success Criteria for Phase 0

Before moving to Phase 1, we must have:

✅ **Rendering contract** locked (HTML/CSS subset documented)
✅ **Capability matrix** defined (iTerm2-first + fallbacks)
✅ **Anti-goals** explicitly stated (no scope creep)
✅ **Risk log** initialized (known unknowns tracked)

---

## Next Steps

Once Phase 0 is approved:
1. **Phase 1**: Choose Markdown → IR path (Pandoc vs pulldown-cmark)
2. **Phase 1**: Define IR node structure
3. **Phase 1**: Build test corpus (GFM samples)
