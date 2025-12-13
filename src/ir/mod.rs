//! Intermediate Representation (IR) for Lumen documents
//!
//! This module defines the stable IR that all Markdown parsers emit.
//! The IR is semantic (not visual) and maps cleanly to terminal rendering primitives.
//!
//! Design principles:
//! - Stable: Avoid frequent changes (prevent IR churn)
//! - Semantic: Preserve document structure, not just styling
//! - Flat-ish: Easy to traverse for layout engine
//! - Recursive: Blocks can contain blocks, inlines can contain inlines

use std::collections::HashMap;

/// Top-level document structure
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    /// Document metadata (title, frontmatter, etc.)
    pub metadata: Metadata,
    /// Block-level content
    pub blocks: Vec<Block>,
}

/// Document metadata
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Metadata {
    /// Document title (if specified)
    pub title: Option<String>,
    /// Frontmatter key-value pairs
    pub frontmatter: HashMap<String, String>,
}

/// Block-level elements (vertical stacking)
#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    /// Heading with level (1-6) and inline content
    Heading {
        level: u8,
        content: Vec<Inline>,
    },

    /// Paragraph with inline content
    Paragraph {
        content: Vec<Inline>,
    },

    /// Code block with optional language hint
    CodeBlock {
        lang: Option<String>,
        code: String,
    },

    /// Block quote containing other blocks
    BlockQuote {
        blocks: Vec<Block>,
    },

    /// List (ordered or unordered)
    List {
        ordered: bool,
        start: usize,
        items: Vec<ListItem>,
    },

    /// Table with headers, rows, and column alignment
    Table {
        headers: Vec<TableCell>,
        rows: Vec<Vec<TableCell>>,
        alignment: Vec<Alignment>,
    },

    /// Horizontal rule / separator
    HorizontalRule,

    /// Callout / admonition (GitHub-style)
    Callout {
        kind: CalloutKind,
        title: Option<String>,
        content: Vec<Block>,
    },
}

/// List item (can contain multiple blocks for nested content)
#[derive(Debug, Clone, PartialEq)]
pub struct ListItem {
    /// Block-level content of this item
    pub content: Vec<Block>,
    /// Task list checkbox state: Some(true) = checked, Some(false) = unchecked, None = not a task
    pub task: Option<bool>,
}

/// Table cell containing inline content
#[derive(Debug, Clone, PartialEq)]
pub struct TableCell {
    pub content: Vec<Inline>,
}

/// Column alignment for tables
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
    None,
}

/// Callout / admonition type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalloutKind {
    Note,
    Warning,
    Important,
    Tip,
    Caution,
}

/// Inline elements (horizontal flow within blocks)
#[derive(Debug, Clone, PartialEq)]
pub enum Inline {
    /// Plain text
    Text(String),

    /// Strong emphasis (semantic bold)
    Strong(Vec<Inline>),

    /// Emphasis (semantic italic)
    Emphasis(Vec<Inline>),

    /// Strikethrough text
    Strikethrough(Vec<Inline>),

    /// Inline code
    Code(String),

    /// Hyperlink
    Link {
        url: String,
        title: Option<String>,
        text: Vec<Inline>,
    },

    /// Image reference
    Image {
        url: String,
        alt: String,
        title: Option<String>,
    },

    /// Hard line break
    LineBreak,

    /// Soft line break (rendered as space)
    SoftBreak,
}

impl Document {
    /// Create a new empty document
    pub fn new() -> Self {
        Self {
            metadata: Metadata::default(),
            blocks: Vec::new(),
        }
    }

    /// Create a document with blocks
    pub fn with_blocks(blocks: Vec<Block>) -> Self {
        Self {
            metadata: Metadata::default(),
            blocks,
        }
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

impl Inline {
    /// Extract plain text from inline content (recursive)
    pub fn to_plain_text(&self) -> String {
        match self {
            Inline::Text(s) => s.clone(),
            Inline::Strong(inlines) | Inline::Emphasis(inlines) | Inline::Strikethrough(inlines) => {
                inlines.iter().map(|i| i.to_plain_text()).collect()
            }
            Inline::Code(s) => s.clone(),
            Inline::Link { text, .. } => text.iter().map(|i| i.to_plain_text()).collect(),
            Inline::Image { alt, .. } => alt.clone(),
            Inline::LineBreak => "\n".to_string(),
            Inline::SoftBreak => " ".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inline_to_plain_text() {
        let inline = Inline::Strong(vec![
            Inline::Text("Hello ".to_string()),
            Inline::Emphasis(vec![Inline::Text("world".to_string())]),
        ]);
        assert_eq!(inline.to_plain_text(), "Hello world");
    }

    #[test]
    fn test_document_creation() {
        let doc = Document::new();
        assert_eq!(doc.blocks.len(), 0);
        assert_eq!(doc.metadata.title, None);
    }
}
