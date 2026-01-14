//! Search functionality for finding text in documents

use crate::layout::{LayoutElement, LayoutNode, Line};

/// Search match position in the document
#[derive(Debug, Clone, PartialEq)]
pub struct SearchMatch {
    pub y: u16,        // Document y position
    pub x: u16,        // X position in line
    pub length: usize, // Match length
    pub text: String,  // Matched text (for context)
}

/// Search state
#[derive(Debug, Clone)]
pub struct SearchState {
    pub needle: String,               // Search query
    pub matches: Vec<SearchMatch>,    // All matches found
    pub current_index: Option<usize>, // Currently selected match
    pub active: bool,                 // Whether search mode is active
}

impl Default for SearchState {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            needle: String::new(),
            matches: Vec::new(),
            current_index: None,
            active: false,
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.needle.clear();
        self.matches.clear();
        self.current_index = None;
    }

    pub fn accept(&mut self) {
        // Exit input mode but keep search results
        self.active = false;
    }

    pub fn deactivate(&mut self) {
        // Clear everything and exit search
        self.active = false;
        self.needle.clear();
        self.matches.clear();
        self.current_index = None;
    }

    pub fn add_char(&mut self, c: char) {
        self.needle.push(c);
    }

    pub fn backspace(&mut self) {
        self.needle.pop();
    }

    pub fn execute_search(&mut self, root: &LayoutNode) {
        self.matches.clear();
        self.current_index = None;

        if self.needle.is_empty() {
            return;
        }

        // Search case-insensitively
        let needle_lower = self.needle.to_lowercase();
        search_node(root, &needle_lower, &mut self.matches);

        // Select first match if any
        if !self.matches.is_empty() {
            self.current_index = Some(0);
        }
    }

    pub fn next_match(&mut self) {
        if let Some(current) = self.current_index {
            if !self.matches.is_empty() {
                self.current_index = Some((current + 1) % self.matches.len());
            }
        } else if !self.matches.is_empty() {
            self.current_index = Some(0);
        }
    }

    pub fn prev_match(&mut self) {
        if let Some(current) = self.current_index {
            if !self.matches.is_empty() {
                self.current_index = Some(if current == 0 {
                    self.matches.len() - 1
                } else {
                    current - 1
                });
            }
        } else if !self.matches.is_empty() {
            self.current_index = Some(self.matches.len() - 1);
        }
    }

    pub fn current_match(&self) -> Option<&SearchMatch> {
        self.current_index.and_then(|i| self.matches.get(i))
    }

    pub fn match_count(&self) -> usize {
        self.matches.len()
    }
}

/// Recursively search through layout nodes
fn search_node(node: &LayoutNode, needle: &str, matches: &mut Vec<SearchMatch>) {
    match &node.element {
        LayoutElement::Heading { text, .. } => {
            search_text(text, needle, node.rect.x, node.rect.y, matches);
        }
        LayoutElement::Paragraph { lines } => {
            for (line_idx, line) in lines.iter().enumerate() {
                let y = node.rect.y + line_idx as u16;
                search_line(line, needle, node.rect.x, y, matches);
            }
        }
        LayoutElement::CodeBlock { lines, .. } => {
            for (line_idx, line_text) in lines.iter().enumerate() {
                // Code blocks have padding, so y is offset by 1
                let y = node.rect.y + 1 + line_idx as u16;
                search_text(line_text, needle, node.rect.x + 1, y, matches);
            }
        }
        _ => {}
    }

    // Recursively search children
    for child in &node.children {
        search_node(child, needle, matches);
    }
}

/// Search within a line of text segments
fn search_line(line: &Line, needle: &str, x: u16, y: u16, matches: &mut Vec<SearchMatch>) {
    let mut current_x = x;
    for segment in &line.segments {
        search_text(&segment.text, needle, current_x, y, matches);
        current_x += segment.text.len() as u16;
    }
}

/// Search for needle in text at given position
fn search_text(text: &str, needle: &str, x: u16, y: u16, matches: &mut Vec<SearchMatch>) {
    let text_lower = text.to_lowercase();
    let mut start = 0;

    while let Some(pos) = text_lower[start..].find(needle) {
        let abs_pos = start + pos;
        matches.push(SearchMatch {
            y,
            x: x + abs_pos as u16,
            length: needle.len(),
            text: text[abs_pos..abs_pos + needle.len()].to_string(),
        });
        start = abs_pos + 1; // Continue searching for overlapping matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_state_navigation() {
        let mut state = SearchState::new();
        state.needle = "test".to_string();
        state.matches = vec![
            SearchMatch {
                y: 0,
                x: 0,
                length: 4,
                text: "test".to_string(),
            },
            SearchMatch {
                y: 5,
                x: 10,
                length: 4,
                text: "test".to_string(),
            },
            SearchMatch {
                y: 10,
                x: 20,
                length: 4,
                text: "test".to_string(),
            },
        ];

        state.current_index = Some(0);
        state.next_match();
        assert_eq!(state.current_index, Some(1));

        state.next_match();
        assert_eq!(state.current_index, Some(2));

        state.next_match(); // Wrap around
        assert_eq!(state.current_index, Some(0));

        state.prev_match();
        assert_eq!(state.current_index, Some(2));
    }

    #[test]
    fn test_search_text() {
        let mut matches = Vec::new();
        search_text("Hello world, hello universe", "hello", 0, 0, &mut matches);

        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].x, 0);
        assert_eq!(matches[1].x, 13);
    }
}
