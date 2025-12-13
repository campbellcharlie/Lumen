Build a Markdown-first document renderer that feels like a browser in a modern terminal (iTerm2-first), by converting:
Markdown → HTML(+CSS) → Terminal scene graph → Render.
Key difference vs Glow/smd: you’re not “styling text with ANSI”; you’re doing document layout (spacing, callouts, tables, images) and then mapping that layout to terminal primitives (cells + colors + images). This unlocks “bling” without fighting terminal font limitations.
We’ll lean on proven building blocks:
Markdown → HTML converters (Pandoc, pulldown-cmark, markdown-it, goldmark) 
GitHub
+3
GitHub
+3
GitHub
+3
Terminal UI / rendering frameworks (Ratatui, Notcurses, Textual, Termwiz) 
Docs.rs
+3
GitHub
+3
GitHub
+3
Modern terminal media protocols (iTerm2 inline images, Kitty graphics protocol, SIXEL) 
iTerm2
+2
Kovid Goyal's Software Projects
+2
Reference “browser-in-terminal” projects for inspiration/validation (Carbonyl, Browsh) 
GitHub
+1
Draft build phases
Phase 0 — Scope + constraints (1–2 days)
Goal: lock the “browser-like” target without accidentally building a full browser.
Deliverables:
Rendering contract: what HTML/CSS subset you support (start tiny: block layout + inline emphasis + tables + images).
Capability matrix: iTerm2-first; graceful fallback paths for other terminals (no hard dependency on iTerm2-only features, but optimize for it).
Risks:
scope creep into JS/DOM → kill it early.
Phase 1 — Markdown ingestion + stable IR (foundation)
Goal: parse Markdown into a stable intermediate representation (IR) you control.
Two viable approaches:
Markdown → HTML using a mature converter, then parse HTML into your IR:
Pandoc gives you a huge feature surface and consistent output 
GitHub
Markdown AST directly (Rust-native if you want performance/control):
pulldown-cmark is reliable and fast, and can emit HTML too 
GitHub
Deliverables:
IR nodes: paragraphs, headings, lists, code blocks, tables, callouts, links, images.
Test corpus: GitHub-flavored markdown samples.
Risk:
“IR churn” if you don’t freeze it early.
Phase 2 — HTML+CSS theming layer (make it feel modern)
Goal: define a CSS-like theme system that drives layout + visuals (spacing, borders, callouts, etc.).
Deliverables:
Minimal CSS subset (tokens + rules): margins, padding, background rectangles, borders, typography “roles” (h1/h2/code/quote/callout).
Theme packs (“Docs”, “Neon”, “Minimal”, etc.) with a toggle.
Payoff:
this is where “bling” becomes consistent and not gimmicky.
Phase 3 — Layout engine (browser-like without browser complexity)
Goal: compute a layout tree (positions + sizes) from IR + theme.
Start simple:
Vertical flow layout (block stacking)
Inline layout (wrapping, emphasis spans)
Tables v1 (fixed + proportional column sizing)
Optional upgrade later:
Constraint-based layout for tables/sidebars (Cassowary-style solvers are often used for UI layout; Ratatui’s ecosystem even includes cassowary work) 
GitHub
Deliverables:
Layout tree + viewport/scroll model
Hit-testing (cursor on link, code block, etc.)
Risk:
tables can eat your life; keep v1 intentionally limited.
Phase 4 — Terminal renderer v1 (text-first, fast)
Goal: render the layout tree to the terminal with crisp typography and zero jitter.
Tech choices:
Rust TUI: Ratatui for widgets/layout + your own renderer for special elements 
GitHub
If you want “character graphics” bling (gradients, panels, media), Notcurses is literally built for that 
GitHub
If you want a more “web dev model” but Python: Textual 
GitHub
Deliverables:
Smooth scrolling
Search, outline view, link navigation
“Emphasis fallback”: if italics unsupported, use a visual emphasis band (background geometry), not fake slanted glyphs.
Phase 5 — Media layer (iTerm2-first images + fallbacks)
Goal: inline images and rich embeds.
Primary (iTerm2):
Implement iTerm2 Inline Images Protocol (OSC 1337) 
iTerm2
+1
Optional cross-terminal:
Kitty graphics protocol (many modern terminals support or emulate it) 
Kovid Goyal's Software Projects
+1
SIXEL as another fallback option 
GitHub
Deliverables:
Image sizing rules: constrain to viewport width; zoom; collapse/expand
Caching + lazy decode (don’t tank scroll performance)
Risk:
protocols + tmux/ssh edge cases; feature-detect and degrade gracefully.
Phase 6 — “Bling that helps” (donut-class enhancements, but meaningful)
Goal: add modern effects that increase comprehension.
Examples:
Section depth: subtle shading by heading level
Link affordances: hover states + clickable links (OSC 8 later if you want)
Code block chrome: language badge, copy button, fold/unfold
Optional minimap / structure radar
Reference inspiration:
Carbonyl shows what’s possible when you go full-renderer in a terminal 
GitHub
Browsh shows the “render with real browser, then present in terminal” approach 
GitHub
Phase 7 — Packaging + compatibility hardening
Goal: make it installable and not fragile.
Deliverables:
Single static binary (ideal)
Integration hooks: man-like pager mode, git integration, mdx/frontmatter support
Terminal capability detection & config
Existing technologies to build on (repo links)
# Markdown → HTML / parsing
https://github.com/jgm/pandoc
https://github.com/pulldown-cmark/pulldown-cmark
https://github.com/markdown-it/markdown-it
https://github.com/yuin/goldmark

# Terminal UI / rendering
https://github.com/ratatui/ratatui
https://github.com/dankamongmen/notcurses
https://github.com/Textualize/textual
https://github.com/wezterm/wezterm        # includes termwiz/term crates (internal-focused) 

# Terminal image / graphics protocols
https://github.com/gnachman/iTerm2        # iTerm2 source
https://iterm2.com/documentation-images.html
https://github.com/kovidgoyal/kitty
https://sw.kovidgoyal.net/kitty/graphics-protocol/
https://github.com/saitoha/libsixel

# Reference projects (browser-ish rendering in terminal)
https://github.com/fathyb/carbonyl
https://github.com/browsh-org/browsh

# Baseline “current gen” markdown TUI viewers (for comparison)
https://github.com/charmbracelet/glow
Pandoc, pulldown-cmark, markdown-it, and goldmark are established Markdown tooling options 
GitHub
+3
GitHub
+3
GitHub
+3
. Ratatui/Textual/Notcurses are mainstream paths for building rich TUIs 
GitHub
+2
GitHub
+2
. iTerm2 inline images + Kitty graphics protocol + SIXEL give you the “modern terminal media” surface area 
iTerm2
+2
Kovid Goyal's Software Projects
+2
. Carbonyl/Browsh are useful north-stars for terminal rendering ambition 
GitHub
+1
.
