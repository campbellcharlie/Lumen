//! Layout engine for positioning document elements

pub mod types;
pub mod text;
pub mod engine;

pub use types::*;
pub use engine::layout_document;

impl LayoutTree {
    /// Find hit region at given coordinates
    pub fn hit_test(&self, x: u16, y: u16) -> Option<&HitRegion> {
        self.hit_regions.iter().find(|region| region.rect.contains(x, y))
    }

    /// Get total document height
    pub fn document_height(&self) -> u16 {
        self.root.rect.height
    }

    /// Check if viewport can scroll down
    pub fn can_scroll_down(&self) -> bool {
        self.viewport.scroll_y + self.viewport.height < self.document_height()
    }

    /// Check if viewport can scroll up
    pub fn can_scroll_up(&self) -> bool {
        self.viewport.scroll_y > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Block, Document, Inline};
    use crate::theme;

    #[test]
    fn test_hit_testing() {
        let theme = theme::docs_theme();
        let viewport = Viewport::new(80, 24);

        let doc = Document::with_blocks(vec![
            Block::Heading {
                level: 1,
                content: vec![Inline::Text("Title".to_string())],
            },
        ]);

        let tree = layout_document(&doc, &theme, viewport);

        // Hit test on heading
        let hit = tree.hit_test(10, 2); // Assuming heading is around y=2
        assert!(hit.is_some());
    }

    #[test]
    fn test_scroll_bounds() {
        let theme = theme::docs_theme();
        let mut viewport = Viewport::new(80, 24);

        let doc = Document::with_blocks(vec![
            Block::Paragraph {
                content: vec![Inline::Text("Line".to_string())],
            };
            50 // Many paragraphs to exceed viewport
        ]);

        let tree = layout_document(&doc, &theme, viewport);

        assert!(tree.can_scroll_down());
        assert!(!tree.can_scroll_up());

        viewport.scroll_to(100);
        let tree2 = LayoutTree {
            viewport,
            ..tree
        };

        assert!(tree2.can_scroll_up());
    }
}
