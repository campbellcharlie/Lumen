//! Lumen: Interactive Markdown viewer

use lumen::{layout_document, parse_markdown, render, LayoutTree, Theme};
use lumen::layout::Viewport;
use std::fs;
use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::Duration;

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

    // Main event loop
    loop {
        // Render
        render::render(&mut terminal, &tree, &theme)?;

        // Handle events
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match handle_key(key, &mut tree) {
                    Action::Quit => break,
                    Action::Continue => {
                        // Re-layout with updated viewport
                        tree = layout_document(&document, &theme, tree.viewport);
                    }
                }
            } else if let Event::Resize(_, _) = event::read()? {
                let size = terminal.size()?;
                viewport = Viewport::new(size.width, size.height.saturating_sub(1));
                tree = layout_document(&document, &theme, viewport);
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
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => Action::Quit,
        KeyCode::Char('j') | KeyCode::Down => {
            tree.viewport.scroll_by(1);
            Action::Continue
        }
        KeyCode::Char('k') | KeyCode::Up => {
            tree.viewport.scroll_by(-1);
            Action::Continue
        }
        KeyCode::Char('d') => {
            tree.viewport.scroll_by(tree.viewport.height as i16 / 2);
            Action::Continue
        }
        KeyCode::Char('u') => {
            tree.viewport.scroll_by(-(tree.viewport.height as i16 / 2));
            Action::Continue
        }
        KeyCode::PageDown | KeyCode::Char(' ') => {
            tree.viewport.scroll_by(tree.viewport.height as i16);
            Action::Continue
        }
        KeyCode::PageUp => {
            tree.viewport.scroll_by(-(tree.viewport.height as i16));
            Action::Continue
        }
        KeyCode::Home | KeyCode::Char('g') => {
            tree.viewport.scroll_to(0);
            Action::Continue
        }
        KeyCode::End | KeyCode::Char('G') => {
            tree.viewport.scroll_to(tree.document_height());
            Action::Continue
        }
        _ => Action::Continue,
    }
}
