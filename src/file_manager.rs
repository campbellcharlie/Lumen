//! File manager for handling multiple open files
//!
//! This module provides state management for working with multiple markdown files
//! simultaneously, including tracking scroll positions and current selections.

use crate::{Document, LayoutTree};
use std::path::PathBuf;

/// Represents a single open file with its state.
///
/// Each open file tracks its path, parsed document, layout cache, and scroll position
/// to allow seamless switching between files without losing navigation state.
#[derive(Clone)]
pub struct OpenFile {
    /// Path to the file
    pub path: PathBuf,
    /// Display name (filename only)
    pub name: String,
    /// Parsed document
    pub document: Document,
    /// Current layout tree (cached)
    pub layout: Option<LayoutTree>,
    /// Current scroll position
    pub scroll_position: u16,
}

impl OpenFile {
    pub fn new(path: PathBuf, document: Document) -> Self {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("untitled")
            .to_string();

        Self {
            path,
            name,
            document,
            layout: None,
            scroll_position: 0,
        }
    }
}

/// Manages multiple open files
pub struct FileManager {
    /// List of open files
    pub files: Vec<OpenFile>,
    /// Index of currently active file
    pub current_index: usize,
}

impl Default for FileManager {
    fn default() -> Self {
        Self::new()
    }
}

impl FileManager {
    /// Create a new empty file manager.
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            current_index: 0,
        }
    }

    /// Add a new file to the manager.
    ///
    /// The file will be parsed and added to the end of the file list.
    /// This does not change the currently selected file.
    pub fn add_file(&mut self, path: PathBuf, document: Document) {
        self.files.push(OpenFile::new(path, document));
    }

    /// Get a reference to the currently active file.
    ///
    /// Returns `None` if no files are open.
    pub fn current_file(&self) -> Option<&OpenFile> {
        self.files.get(self.current_index)
    }

    /// Get a mutable reference to the currently active file.
    ///
    /// Returns `None` if no files are open.
    pub fn current_file_mut(&mut self) -> Option<&mut OpenFile> {
        self.files.get_mut(self.current_index)
    }

    /// Switch to the next file in the list (cycles to first after last).
    pub fn next_file(&mut self) {
        if !self.files.is_empty() {
            self.current_index = (self.current_index + 1) % self.files.len();
        }
    }

    /// Switch to the previous file in the list (cycles to last from first).
    pub fn prev_file(&mut self) {
        if !self.files.is_empty() {
            self.current_index = if self.current_index == 0 {
                self.files.len() - 1
            } else {
                self.current_index - 1
            };
        }
    }

    /// Switch to specific file by index (0-based)
    pub fn switch_to(&mut self, index: usize) {
        if index < self.files.len() {
            self.current_index = index;
        }
    }

    /// Get number of open files
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// Check if multiple files are open
    pub fn has_multiple_files(&self) -> bool {
        self.files.len() > 1
    }

    /// Reload current file from disk
    pub fn reload_current(&mut self) -> std::io::Result<()> {
        if let Some(file) = self.current_file_mut() {
            let markdown = std::fs::read_to_string(&file.path)?;
            file.document = crate::parse_markdown(&markdown);
            file.layout = None; // Force relayout
        }
        Ok(())
    }

    /// Save scroll position for current file
    pub fn save_scroll_position(&mut self, scroll_y: u16) {
        if let Some(file) = self.current_file_mut() {
            file.scroll_position = scroll_y;
        }
    }

    /// Get saved scroll position for current file
    pub fn get_scroll_position(&self) -> u16 {
        self.current_file().map(|f| f.scroll_position).unwrap_or(0)
    }
}
