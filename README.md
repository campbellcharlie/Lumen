# Lumen

**A browser-like Markdown document renderer for modern terminals**

Lumen renders Markdown documents with rich layout, theming, and media support—directly in your terminal. Unlike traditional Markdown viewers that style text with ANSI codes, Lumen treats rendering as a proper layout problem: Markdown → HTML+CSS → scene graph → terminal primitives.

---

## Philosophy

- **Browser-like, not browser-complex**: Rich document layout without JavaScript, DOM, or network complexity
- **iTerm2-first, gracefully degrading**: Optimized for modern terminal capabilities with smart fallbacks
- **Bling with purpose**: Visual enhancements that improve comprehension, not just aesthetics
- **Deterministic and auditable**: No magic, no hallucinated features, just clean rendering pipelines

---

## Architecture

```
Markdown → HTML (via converter) → IR (intermediate representation)
                                    ↓
                              Theme + Layout Engine
                                    ↓
                              Terminal Renderer → iTerm2/Kitty/etc.
```

Key design decisions:
- **HTML as intermediate format**: Leverage proven Markdown→HTML converters (Pandoc, pulldown-cmark)
- **Custom IR for control**: Parse HTML into our own IR to avoid layout engine complexity
- **Theme-driven CSS subset**: Declarative token system, not arbitrary CSS parsing
- **Terminal protocol abstraction**: Unified media layer with feature detection + fallbacks

---

## Roadmap

- **[Phase 0](./PHASE0.md)**: Scope + constraints ← *Current*
- **Phase 1**: Markdown → IR parsing pipeline
- **Phase 2**: CSS theming layer
- **Phase 3**: Layout engine (flow + inline + tables)
- **Phase 4**: Terminal renderer (text + scrolling + navigation)
- **Phase 5**: Media layer (images via iTerm2/Kitty/SIXEL)
- **Phase 6**: Enhanced UX (hover states, code block chrome, minimap)
- **Phase 7**: Packaging + hardening

See [Project.MD](./Project.MD) for full phase breakdown.

---

## Supported Features (Target v1)

### Markdown Elements
- Headings, paragraphs, lists (ordered/unordered)
- Tables (fixed and proportional columns)
- Code blocks (fenced, with language hints)
- Blockquotes, callouts/admonitions
- Links, inline emphasis (bold/italic/code)
- Images (via terminal protocols)
- Horizontal rules

### Terminal Capabilities
- True color (24-bit RGB)
- Unicode box-drawing characters
- Inline images (iTerm2 protocol, Kitty graphics protocol)
- Clickable links (OSC 8)
- Smooth scrolling, search, outline navigation

### Themes
- Pre-built theme packs (Docs, Neon, Minimal)
- Custom theme support (declarative YAML/TOML)

---

## Building Blocks

Lumen leverages proven open-source technologies:

**Markdown parsing**:
- [Pandoc](https://github.com/jgm/pandoc)
- [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark)

**Terminal UI**:
- [Ratatui](https://github.com/ratatui/ratatui)
- [Notcurses](https://github.com/dankamongmen/notcurses)

**Terminal protocols**:
- [iTerm2 inline images](https://iterm2.com/documentation-images.html)
- [Kitty graphics protocol](https://sw.kovidgoyal.net/kitty/graphics-protocol/)
- [SIXEL](https://github.com/saitoha/libsixel)

**Inspiration**:
- [Carbonyl](https://github.com/fathyb/carbonyl) — Browser in terminal
- [Browsh](https://github.com/browsh-org/browsh) — Browser proxy to terminal
- [Glow](https://github.com/charmbracelet/glow) — Markdown TUI viewer

---

## Development Environment

- **Target platform**: macOS (M1+ recommended for GPU acceleration)
- **Primary terminal**: iTerm2 (with fallback support for Kitty, WezTerm, Alacritty, etc.)
- **Language**: TBD (Rust preferred for performance + terminal ecosystem fit)

---

## License

TBD
