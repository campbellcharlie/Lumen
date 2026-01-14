//! Integration tests for multi-file management

use lumen::{parse_markdown, FileManager};
use std::path::PathBuf;

#[test]
fn test_file_manager_basic_operations() {
    let mut manager = FileManager::new();
    assert_eq!(manager.file_count(), 0);
    assert!(manager.current_file().is_none());

    // Add first file
    let doc1 = parse_markdown("# File 1\n\nContent of file 1");
    manager.add_file(PathBuf::from("/test/file1.md"), doc1);

    assert_eq!(manager.file_count(), 1);
    assert!(manager.current_file().is_some());
    assert_eq!(manager.current_file().unwrap().name, "file1.md");

    // Add second file
    let doc2 = parse_markdown("# File 2\n\nContent of file 2");
    manager.add_file(PathBuf::from("/test/file2.md"), doc2);

    assert_eq!(manager.file_count(), 2);
    assert!(manager.has_multiple_files());
}

#[test]
fn test_file_navigation() {
    let mut manager = FileManager::new();

    let doc1 = parse_markdown("# File 1");
    let doc2 = parse_markdown("# File 2");
    let doc3 = parse_markdown("# File 3");

    manager.add_file(PathBuf::from("/file1.md"), doc1);
    manager.add_file(PathBuf::from("/file2.md"), doc2);
    manager.add_file(PathBuf::from("/file3.md"), doc3);

    // Test next navigation
    assert_eq!(manager.current_file().unwrap().name, "file1.md");

    manager.next_file();
    assert_eq!(manager.current_file().unwrap().name, "file2.md");

    manager.next_file();
    assert_eq!(manager.current_file().unwrap().name, "file3.md");

    // Test wrap around
    manager.next_file();
    assert_eq!(manager.current_file().unwrap().name, "file1.md");

    // Test previous navigation
    manager.prev_file();
    assert_eq!(manager.current_file().unwrap().name, "file3.md");

    manager.prev_file();
    assert_eq!(manager.current_file().unwrap().name, "file2.md");
}

#[test]
fn test_direct_file_switch() {
    let mut manager = FileManager::new();

    for i in 1..=5 {
        let doc = parse_markdown(&format!("# File {}", i));
        manager.add_file(PathBuf::from(format!("/file{}.md", i)), doc);
    }

    // Switch directly to file at index 3 (zero-based)
    manager.switch_to(3);
    assert_eq!(manager.current_file().unwrap().name, "file4.md");

    // Switch to first file
    manager.switch_to(0);
    assert_eq!(manager.current_file().unwrap().name, "file1.md");

    // Invalid index should be ignored
    manager.switch_to(99);
    assert_eq!(manager.current_file().unwrap().name, "file1.md");
}

#[test]
fn test_scroll_position_persistence() {
    let mut manager = FileManager::new();

    let doc1 = parse_markdown("# File 1");
    let doc2 = parse_markdown("# File 2");

    manager.add_file(PathBuf::from("/file1.md"), doc1);
    manager.add_file(PathBuf::from("/file2.md"), doc2);

    // Set scroll position for file 1
    manager.save_scroll_position(42);
    assert_eq!(manager.get_scroll_position(), 42);

    // Switch to file 2
    manager.next_file();
    assert_eq!(manager.get_scroll_position(), 0); // Default scroll for new file

    // Set scroll for file 2
    manager.save_scroll_position(100);
    assert_eq!(manager.get_scroll_position(), 100);

    // Switch back to file 1 - should restore scroll
    manager.prev_file();
    assert_eq!(manager.get_scroll_position(), 42);
}
