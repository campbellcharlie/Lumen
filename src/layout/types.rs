//! Core layout types and structures

use crate::theme::{Color, FontStyle, FontWeight};

/// Unique identifier for layout nodes
pub type NodeId = usize;

/// Complete layout tree with viewport
#[derive(Debug, Clone)]
pub struct LayoutTree {
    pub root: LayoutNode,
    pub viewport: Viewport,
    pub hit_regions: Vec<HitRegion>,
}

/// A positioned and sized layout node
#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub id: NodeId,
    pub rect: Rectangle,
    pub element: LayoutElement,
    pub children: Vec<LayoutNode>,
    pub style: ComputedStyle,
}

/// Rectangle in terminal coordinate space (character cells)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    pub x: u16,      // Column (0-based)
    pub y: u16,      // Row (0-based)
    pub width: u16,  // Width in columns
    pub height: u16, // Height in rows
}

impl Rectangle {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self { x, y, width, height }
    }

    pub fn contains(&self, x: u16, y: u16) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

/// Layout element types
#[derive(Debug, Clone)]
pub enum LayoutElement {
    Document,
    Heading {
        level: u8,
        text: String,
    },
    Paragraph {
        lines: Vec<Line>,
    },
    CodeBlock {
        lang: Option<String>,
        lines: Vec<String>,
    },
    BlockQuote,
    List {
        ordered: bool,
        start: usize,
    },
    ListItem {
        marker: String,
        task: Option<bool>,
    },
    Table {
        column_widths: Vec<u16>,
    },
    TableRow {
        is_header: bool,
    },
    TableCell,
    HorizontalRule,
}

/// A line of text (result of inline layout)
#[derive(Debug, Clone)]
pub struct Line {
    pub segments: Vec<TextSegment>,
}

impl Line {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    pub fn add_segment(&mut self, text: String, style: TextStyle) {
        self.segments.push(TextSegment { text, style });
    }

    pub fn width(&self) -> u16 {
        self.segments.iter().map(|s| s.text.len() as u16).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }
}

/// A styled text segment within a line
#[derive(Debug, Clone)]
pub struct TextSegment {
    pub text: String,
    pub style: TextStyle,
}

/// Computed style for a layout node
#[derive(Debug, Clone)]
pub struct ComputedStyle {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub padding: EdgeSizes,
    pub margin: EdgeSizes,
}

impl Default for ComputedStyle {
    fn default() -> Self {
        Self {
            foreground: None,
            background: None,
            weight: FontWeight::Normal,
            style: FontStyle::Normal,
            padding: EdgeSizes::zero(),
            margin: EdgeSizes::zero(),
        }
    }
}

/// Edge sizes (top, right, bottom, left)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EdgeSizes {
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
    pub left: u16,
}

impl EdgeSizes {
    pub fn zero() -> Self {
        Self {
            top: 0,
            right: 0,
            bottom: 0,
            left: 0,
        }
    }

    pub fn all(size: u16) -> Self {
        Self {
            top: size,
            right: size,
            bottom: size,
            left: size,
        }
    }

    pub fn vertical(size: u16) -> Self {
        Self {
            top: size,
            right: 0,
            bottom: size,
            left: 0,
        }
    }

    pub fn horizontal(size: u16) -> Self {
        Self {
            top: 0,
            right: size,
            bottom: 0,
            left: size,
        }
    }
}

/// Text style (subset of ComputedStyle for inline elements)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextStyle {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub weight: FontWeight,
    pub style: FontStyle,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            foreground: None,
            background: None,
            weight: FontWeight::Normal,
            style: FontStyle::Normal,
        }
    }
}

/// Viewport (terminal window)
#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub width: u16,      // Terminal width (columns)
    pub height: u16,     // Terminal height (rows)
    pub scroll_x: u16,   // Horizontal scroll offset
    pub scroll_y: u16,   // Vertical scroll offset
}

impl Viewport {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            scroll_x: 0,
            scroll_y: 0,
        }
    }

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

    pub fn scroll_to_clamped(&mut self, y: u16, doc_height: u16) {
        // Clamp scroll position so viewport doesn't go past document end
        let max_scroll = doc_height.saturating_sub(self.height);
        self.scroll_y = y.min(max_scroll);
    }

    pub fn scroll_by(&mut self, delta_y: i16) {
        self.scroll_y = (self.scroll_y as i16 + delta_y).max(0) as u16;
    }

    pub fn scroll_by_clamped(&mut self, delta_y: i16, doc_height: u16) {
        let new_y = (self.scroll_y as i16 + delta_y).max(0) as u16;
        let max_scroll = doc_height.saturating_sub(self.height);
        self.scroll_y = new_y.min(max_scroll);
    }

    pub fn contains_point(&self, x: u16, y: u16) -> bool {
        let rect = self.visible_rect();
        rect.contains(x, y)
    }
}

/// Hit testing region
#[derive(Debug, Clone)]
pub struct HitRegion {
    pub rect: Rectangle,
    pub element: HitElement,
}

/// Interactive elements
#[derive(Debug, Clone)]
pub enum HitElement {
    Link { url: String, text: String },
    CodeBlock { lang: Option<String> },
    Heading { level: u8, id: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_contains() {
        let rect = Rectangle::new(10, 20, 30, 40);
        assert!(rect.contains(10, 20)); // Top-left
        assert!(rect.contains(39, 59)); // Bottom-right - 1
        assert!(!rect.contains(9, 20)); // Left of rect
        assert!(!rect.contains(40, 20)); // Right of rect
    }

    #[test]
    fn test_rectangle_intersects() {
        let rect1 = Rectangle::new(10, 10, 20, 20);
        let rect2 = Rectangle::new(20, 20, 20, 20); // Overlaps
        let rect3 = Rectangle::new(50, 50, 10, 10); // No overlap

        assert!(rect1.intersects(&rect2));
        assert!(rect2.intersects(&rect1));
        assert!(!rect1.intersects(&rect3));
    }

    #[test]
    fn test_viewport() {
        let mut viewport = Viewport::new(80, 24);
        assert_eq!(viewport.width, 80);
        assert_eq!(viewport.height, 24);
        assert_eq!(viewport.scroll_y, 0);

        viewport.scroll_by(10);
        assert_eq!(viewport.scroll_y, 10);

        viewport.scroll_by(-5);
        assert_eq!(viewport.scroll_y, 5);

        viewport.scroll_by(-10); // Should clamp to 0
        assert_eq!(viewport.scroll_y, 0);
    }

    #[test]
    fn test_line_width() {
        let mut line = Line::new();
        line.add_segment("Hello".to_string(), TextStyle::default());
        line.add_segment(" ".to_string(), TextStyle::default());
        line.add_segment("World".to_string(), TextStyle::default());

        assert_eq!(line.width(), 11); // 5 + 1 + 5
    }

    #[test]
    fn test_edge_sizes() {
        let zero = EdgeSizes::zero();
        assert_eq!(zero.top, 0);
        assert_eq!(zero.left, 0);

        let all = EdgeSizes::all(2);
        assert_eq!(all.top, 2);
        assert_eq!(all.right, 2);

        let vertical = EdgeSizes::vertical(3);
        assert_eq!(vertical.top, 3);
        assert_eq!(vertical.bottom, 3);
        assert_eq!(vertical.left, 0);
    }
}
