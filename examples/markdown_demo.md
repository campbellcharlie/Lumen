# Lumen Markdown Feature Demo

This document demonstrates all supported markdown features in Lumen, optimized for viewing AI-generated content.

---

## Headings

Lumen supports all six heading levels with proper visual hierarchy:

# Heading 1 - Main Title
## Heading 2 - Section
### Heading 3 - Subsection
#### Heading 4 - Minor Section
##### Heading 5 - Detail
###### Heading 6 - Fine Detail

---

## Paragraphs & Text Formatting

Regular paragraphs support **bold text**, *italic text*, and even ***bold italic text***. You can also use ~~strikethrough~~ for deleted content.

Inline `code` is perfect for highlighting variable names, function calls, or short snippets. This is especially common in AI-generated technical documentation.

Multiple paragraphs are separated by proper spacing for readability.

---

## Lists

### Unordered Lists

- Top level item
- Another top level item
  - Nested item level 2
  - Another nested item
    - Even deeper nesting level 3
    - More at level 3
      - Level 4 nesting works perfectly
      - Critical for AI-generated documentation

### Ordered Lists

1. First step
2. Second step
   1. Sub-step A
   2. Sub-step B
      1. Detailed instruction
      2. Another detail
3. Third step

### Mixed Lists

1. **Installation**
   - Download the binary
   - Extract to `/usr/local/bin`
   - Verify with `lumen --version`
2. **Configuration**
   - Create config file
   - Set theme preference
3. **Usage**
   - Open markdown files
   - Navigate with keyboard shortcuts

### Task Lists

- [x] Parse markdown correctly
- [x] Implement deep nesting support
- [x] Add table rendering
- [ ] Implement syntax highlighting
- [ ] Add image protocol support

---

## Links & Navigation

[Visit the Lumen repository](https://github.com/campbellcharlie/Lumen)

[Jump to Tables section](#tables)

Internal links work with anchor navigation - press `a` to cycle through links, then `Enter` to jump.

---

## Tables

### Simple Table

| Feature | Status | Priority |
|---------|--------|----------|
| Markdown Parsing | Complete | High |
| Deep Nesting | Complete | High |
| Themes | Complete | Medium |
| Search | Complete | Medium |

### Complex Table with Alignment

| Left Aligned | Centered | Right Aligned |
|:-------------|:--------:|-------------:|
| Default | Text | Numbers |
| AI Tools | Claude | 100% |
| Rendering | Gemini | 95% |
| Accuracy | Codex | 90% |

### Wide Table

| Model | Context Window | Max Output | Streaming | Vision | Function Calling | Price (Input) | Price (Output) |
|-------|----------------|------------|-----------|--------|------------------|---------------|----------------|
| Claude Sonnet 4 | 200k | 8k | Yes | Yes | Yes | $3/1M | $15/1M |
| GPT-4 Turbo | 128k | 4k | Yes | Yes | Yes | $10/1M | $30/1M |
| Gemini Pro | 1M | 8k | Yes | Yes | Yes | Free | Free |

---

## Code Blocks

### Inline Code

Use `lumen demo.md` to view this file.

### Fenced Code Blocks

```rust
// Rust code with syntax detection
fn main() {
    let markdown = "# Hello, Lumen!";
    let doc = parse_markdown(markdown);
    render_to_terminal(doc);
}
```

```python
# Python example
def analyze_ai_output(file_path):
    with open(file_path, 'r') as f:
        content = f.read()
    return parse_markdown(content)
```

```javascript
// JavaScript example
const lumen = require('lumen');
const doc = lumen.parse('**bold** and *italic*');
console.log(doc);
```

```bash
# Shell commands
cargo build --release
lumen examples/markdown_demo.md neon
lumen *.md
```

---

## Blockquotes

> Single level blockquote
>
> Supports multiple paragraphs within the same quote.

> **Nested blockquotes:**
>
> > This is nested one level deep
> >
> > > And this is nested two levels
> > >
> > > Perfect for showing conversation threads or nested context

> **Note:** Blockquotes are commonly used by AI tools to highlight important information, warnings, or side notes.

---

## Deep Nesting Demo

This demonstrates the complex nesting that AI tools often generate:

- **Research:**
  - [Literature Review](#)
    - Academic Papers
      - "Deep Learning for NLP" (2023)
      - "Transformer Architecture" (2017)
        - Original attention mechanism
        - Multi-head attention details
  - [Industry Reports](#)
    - Market Analysis
    - Technology Trends

- **Implementation:**
  - Backend Development
    - API Design
      - REST endpoints
      - GraphQL schema
        - Query types
        - Mutation types
          - User mutations
          - Data mutations
  - Frontend Development
    - Component architecture
    - State management
      - Redux setup
      - Action creators

---

## Mixed Content

Combining different markdown elements is common in AI responses:

1. **First, understand the problem:**

   > The issue occurs when parsing deeply nested lists in tight mode.

   ```rust
   // This was the problematic code
   if content.is_empty() {
       skip_marker = true;
   }
   ```

2. **Second, analyze the root cause:**

   | Symptom | Cause | Solution |
   |---------|-------|----------|
   | Double bullets | Empty parent | Skip marker rendering |
   | Wrong indentation | Merged text | Paragraph wrapping |

3. **Finally, implement the fix:**

   - [x] Add paragraph wrapping on nested list start
   - [x] Test with 4+ nesting levels
   - [x] Verify tight list handling
   - [ ] Add regression tests

---

## Horizontal Rules

Use horizontal rules to separate major sections:

---

***

---

## Special Characters & Edge Cases

Test special characters: & < > " '

Test URLs in text: https://github.com/campbellcharlie/Lumen

Test email: user@example.com

**Edge cases:**
- Empty list items work correctly
- Lists with only nested children
- Tables with empty cells
- Code blocks without language tags

---

## Conclusion

This demo showcases all GFM features that Lumen handles correctly, with special attention to:

- Deep nesting (4+ levels) ✓
- Complex tables ✓
- Mixed content ✓
- Proper spacing and layout ✓

Perfect for viewing markdown generated by Claude, Gemini, Codex, and other AI coding tools.
