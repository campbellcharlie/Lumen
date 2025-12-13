# Phase 1: Markdown Ingestion + Stable IR

## Decision: Parsing Strategy

### Option 1: Pandoc (External Process)
**Pros**:
- Mature, battle-tested, handles edge cases
- Huge feature surface (GFM, tables, footnotes, etc.)
- Consistent HTML output
- No parser maintenance burden

**Cons**:
- External dependency (requires Pandoc installed)
- Process overhead (spawn + IPC)
- Less control over AST structure
- Harder to debug parsing issues

### Option 2: pulldown-cmark (Rust Native)
**Pros**:
- Zero external dependencies
- Fast (zero-copy parsing)
- Direct AST access (full control)
- Rust ecosystem fit (can emit HTML too)
- Easier to extend/customize
- Better error messages in our control

**Cons**:
- Must handle edge cases ourselves
- GFM extensions require extra crates
- More initial implementation work

### **Recommendation: pulldown-cmark**

**Rationale**:
1. **Performance**: Native Rust, zero-copy, no process spawning
2. **Control**: Direct AST manipulation, easier to map to our IR
3. **Deployment**: Single binary, no external dependencies
4. **Ecosystem fit**: Integrates naturally with Ratatui/terminal ecosystem
5. **M1 Max optimization**: Native code can leverage ARM64 efficiently

**GFM support**: Use `pulldown-cmark` + `pulldown-cmark-gfm` crate for tables, strikethrough, task lists, etc.

---

## IR Node Structure (v1)

Our intermediate representation must be:
- **Stable**: Don't change it frequently (avoid IR churn)
- **Flat-ish**: Easy to traverse for layout engine
- **Semantic**: Preserve document structure, not just styling

### Core IR Types

```rust
/// Top-level document
pub struct Document {
    pub metadata: Metadata,
    pub blocks: Vec<Block>,
}

pub struct Metadata {
    pub title: Option<String>,
    pub frontmatter: HashMap<String, String>,
}

/// Block-level elements
pub enum Block {
    Heading { level: u8, content: Vec<Inline> },
    Paragraph { content: Vec<Inline> },
    CodeBlock { lang: Option<String>, code: String },
    BlockQuote { blocks: Vec<Block> },
    List { ordered: bool, start: usize, items: Vec<ListItem> },
    Table { headers: Vec<TableCell>, rows: Vec<Vec<TableCell>>, alignment: Vec<Alignment> },
    HorizontalRule,
    Callout { kind: CalloutKind, title: Option<String>, content: Vec<Block> },
}

pub struct ListItem {
    pub content: Vec<Block>,
    pub task: Option<bool>, // Some(true) = checked, Some(false) = unchecked, None = not a task
}

pub struct TableCell {
    pub content: Vec<Inline>,
}

pub enum Alignment {
    Left,
    Center,
    Right,
    None,
}

pub enum CalloutKind {
    Note,
    Warning,
    Important,
    Tip,
}

/// Inline elements
pub enum Inline {
    Text(String),
    Strong(Vec<Inline>),
    Emphasis(Vec<Inline>),
    Code(String),
    Link { url: String, title: Option<String>, text: Vec<Inline> },
    Image { url: String, alt: String, title: Option<String> },
    LineBreak,
    SoftBreak,
}
```

### Design Principles

1. **Recursive structure**: Blocks can contain Blocks (for quotes, lists), Inlines can contain Inlines (for nested emphasis)
2. **Semantic, not visual**: `Strong` not "bold", `Emphasis` not "italic" (theming decides visuals)
3. **Preserve intent**: Code blocks remember their language, links remember URLs, tables remember alignment
4. **Flat where possible**: Avoid deep nesting that complicates layout

---

## Implementation Plan

### Step 1: Rust Project Setup
```bash
cargo init --name lumen
cargo add pulldown-cmark
cargo add pulldown-cmark-to-cmark  # For debugging/roundtrip testing
```

### Step 2: Parser Module
```
src/
  lib.rs          # Public API
  ir/
    mod.rs        # IR types (Document, Block, Inline)
  parser/
    mod.rs        # Markdown → IR conversion
    markdown.rs   # pulldown-cmark integration
  tests/
    parser_tests.rs
```

### Step 3: Core Parser Logic
1. Iterate over pulldown-cmark events
2. Build IR tree via state machine
3. Handle GFM extensions (tables, task lists, strikethrough)
4. Normalize whitespace/breaks

### Step 4: Test Corpus
Create `tests/fixtures/`:
- `basic.md` — Headings, paragraphs, emphasis
- `lists.md` — Ordered, unordered, nested, task lists
- `code.md` — Fenced blocks, inline code, language hints
- `tables.md` — Simple tables, aligned columns
- `links_images.md` — Links, images, reference links
- `callouts.md` — GitHub-style callouts
- `complex.md` — Real-world document (README-like)

---

## Success Criteria

Before exiting Phase 1:
✅ IR types defined and documented
✅ Markdown → IR parser working for all test cases
✅ Test corpus covers GFM features
✅ Parser handles malformed input gracefully
✅ Zero external runtime dependencies (pulldown-cmark is compiled in)

---

## Risks

| Risk | Mitigation |
|------|------------|
| **IR changes break downstream** | Freeze IR early; use semver for breaking changes |
| **GFM edge cases** | Comprehensive test corpus; compare against GitHub's rendering |
| **Performance on large docs** | Benchmark with 10MB+ markdown files; optimize if needed |
| **Missing features** | Document unsupported features explicitly; add later if needed |
