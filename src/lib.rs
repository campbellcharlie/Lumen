//! Lumen: A browser-like Markdown document renderer for modern terminals
//!
//! # Architecture
//!
//! ```text
//! Markdown → HTML → IR (intermediate representation)
//!                     ↓
//!               Theme + Layout Engine
//!                     ↓
//!               Terminal Renderer → iTerm2/Kitty/etc.
//! ```
//!
//! # Modules
//!
//! - `ir`: Intermediate representation types (Document, Block, Inline)
//! - `parser`: Markdown → IR conversion
//! - `theme`: CSS-like theming system
//! - `layout`: Layout engine (positions + sizes)
//! - `render`: Terminal renderer
//! - `search`: Search functionality

pub mod ir;
pub mod parser;
pub mod theme;
pub mod layout;
pub mod render;
pub mod search;

pub use ir::Document;
pub use parser::parse_markdown;
pub use theme::Theme;
pub use layout::{layout_document, LayoutTree};
pub use search::SearchState;
