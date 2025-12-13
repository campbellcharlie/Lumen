//! Lumen: Interactive Markdown viewer

use lumen::{layout_document, parse_markdown, render, LayoutTree, Theme};
use lumen::layout::{Viewport, LayoutElement};
use std::fs;
use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::{Duration, Instant};

fn main() -> io::Result<()> {
    // Parse command-line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: lumen <file.md> [theme]");
        eprintln!("\nAvailable themes: {}", Theme::builtin_names().join(", "));
        std::process::exit(1);
    }

    let file_path = &args[1];
    let theme_name = args.get(2).map(|s| s.as_str()).unwrap_or("docs");

    // Load markdown file
    let markdown = fs::read_to_string(file_path)
        .unwrap_or_else(|e| {
            eprintln!("Error reading file '{}': {}", file_path, e);
            std::process::exit(1);
        });

    // Parse markdown
    let document = parse_markdown(&markdown);

    // Load theme
    let theme = Theme::builtin(theme_name)
        .unwrap_or_else(|| {
            eprintln!("Unknown theme '{}', using 'docs'", theme_name);
            Theme::builtin("docs").unwrap()
        });

    // Initialize terminal
    let mut terminal = render::init_terminal()?;
    let size = terminal.size()?;
    let mut viewport = Viewport::new(size.width, size.height.saturating_sub(1)); // -1 for status bar

    // Layout document
    let mut tree = layout_document(&document, &theme, viewport);

    // Frame rate limiting - target 60 FPS
    let frame_duration = Duration::from_millis(16);
    let mut last_render = Instant::now();
    let mut needs_render = true;
    let mut show_help = false;

    // Main event loop
    loop {
        // Render only if needed and enough time has passed
        let now = Instant::now();
        if needs_render && now.duration_since(last_render) >= frame_duration {
            render::render(&mut terminal, &tree, &theme, show_help)?;
            last_render = now;
            needs_render = false;
        }

        // Poll for events with short timeout
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                // Handle help menu toggle specially
                if key.code == KeyCode::Char('h') {
                    show_help = !show_help;
                    needs_render = true;
                } else if show_help && key.code == KeyCode::Esc {
                    show_help = false;
                    needs_render = true;
                } else if !show_help {
                    match handle_key(key, &mut tree) {
                        Action::Quit => break,
                        Action::Continue => {
                            needs_render = true;  // Mark that we need to render
                        }
                    }
                }
            } else if let Event::Resize(_, _) = event::read()? {
                let size = terminal.size()?;
                viewport = Viewport::new(size.width, size.height.saturating_sub(1));
                tree = layout_document(&document, &theme, viewport);
                needs_render = true;
            }
        }
    }

    // Restore terminal
    render::restore_terminal(&mut terminal)?;

    Ok(())
}

enum Action {
    Quit,
    Continue,
}

fn handle_key(key: KeyEvent, tree: &mut LayoutTree) -> Action {
    let doc_height = tree.document_height();

    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => Action::Quit,
        KeyCode::Char('j') | KeyCode::Down => {
            tree.viewport.scroll_by_clamped(1, doc_height);
            Action::Continue
        }
        KeyCode::Char('k') | KeyCode::Up => {
            tree.viewport.scroll_by_clamped(-1, doc_height);
            Action::Continue
        }
        KeyCode::Char('d') => {
            tree.viewport.scroll_by_clamped(tree.viewport.height as i16 / 2, doc_height);
            Action::Continue
        }
        KeyCode::Char('u') => {
            tree.viewport.scroll_by_clamped(-(tree.viewport.height as i16 / 2), doc_height);
            Action::Continue
        }
        KeyCode::PageDown | KeyCode::Char(' ') => {
            tree.viewport.scroll_by_clamped(tree.viewport.height as i16, doc_height);
            Action::Continue
        }
        KeyCode::PageUp => {
            tree.viewport.scroll_by_clamped(-(tree.viewport.height as i16), doc_height);
            Action::Continue
        }
        KeyCode::Home | KeyCode::Char('g') => {
            tree.viewport.scroll_to(0);
            Action::Continue
        }
        KeyCode::End | KeyCode::Char('G') => {
            tree.viewport.scroll_to_clamped(tree.document_height(), doc_height);
            Action::Continue
        }
        KeyCode::Char('n') => {
            // Jump to next heading
            jump_to_next_heading(tree, true);
            Action::Continue
        }
        KeyCode::Char('p') => {
            // Jump to previous heading
            jump_to_next_heading(tree, false);
            Action::Continue
        }
        _ => Action::Continue,
    }
}

fn jump_to_next_heading(tree: &mut LayoutTree, forward: bool) {
    let current_y = tree.viewport.scroll_y;
    let doc_height = tree.document_height();
    let mut headings: Vec<u16> = Vec::new();

    // Collect all heading positions
    fn collect_headings(node: &lumen::layout::LayoutNode, headings: &mut Vec<u16>) {
        if matches!(node.element, LayoutElement::Heading { .. }) {
            headings.push(node.rect.y);
        }
        for child in &node.children {
            collect_headings(child, headings);
        }
    }

    collect_headings(&tree.root, &mut headings);
    headings.sort();

    // Find next or previous heading
    if forward {
        // Find first heading after current position
        if let Some(&next_y) = headings.iter().find(|&&y| y > current_y) {
            tree.viewport.scroll_to_clamped(next_y, doc_height);
        }
    } else {
        // Find last heading before current position
        if let Some(&prev_y) = headings.iter().rev().find(|&&y| y < current_y) {
            tree.viewport.scroll_to_clamped(prev_y, doc_height);
        }
    }
}
