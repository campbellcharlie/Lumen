//! Lumen: Interactive Markdown viewer

use lumen::{layout_document, parse_markdown, render, LayoutTree, Theme, SearchState};
use lumen::layout::{Viewport, LayoutElement};
use std::fs;
use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, MouseEvent, MouseEventKind};
use std::time::{Duration, Instant};

fn main() -> io::Result<()> {
    // Set up panic handler to ensure terminal is always restored
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // Try to restore terminal on panic
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(
            io::stdout(),
            crossterm::terminal::LeaveAlternateScreen,
            crossterm::cursor::Show
        );
        original_hook(panic_info);
    }));

    // Parse command-line arguments
    let args: Vec<String> = std::env::args().collect();

    // Check for flags
    let no_images = args.iter().any(|arg| arg == "--no-images" || arg == "-n");
    let inline_images = args.iter().any(|arg| arg == "--inline-images" || arg == "-i");

    // Get non-flag arguments
    let non_flag_args: Vec<&String> = args.iter()
        .filter(|arg| !arg.starts_with('-'))
        .collect();

    // Read from file
    if non_flag_args.len() < 2 {
        eprintln!("Usage: lumen <file.md> [theme] [options]");
        eprintln!("\nOptions:");
        eprintln!("  --no-images, -n       Disable all image rendering");
        eprintln!("  --inline-images, -i   Render images inline (default: sidebar)");
        eprintln!("\nAvailable themes: {}", Theme::builtin_names().join(", "));
        eprintln!("\nExamples:");
        eprintln!("  lumen README.md");
        eprintln!("  lumen README.md neon");
        eprintln!("  lumen README.md --inline-images");
        eprintln!("  lumen README.md --no-images");
        std::process::exit(1);
    }

    let file_path = non_flag_args.get(1).unwrap();
    let theme_name = non_flag_args.get(2).map(|s| s.as_str()).unwrap_or("docs");

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

    // Initialize terminal and run - ensure cleanup happens even on error
    run_interactive(&document, &theme, no_images, inline_images, Some(file_path.to_string()))
}

/// Run the interactive viewer with proper terminal cleanup
fn run_interactive(initial_document: &lumen::Document, theme: &Theme, no_images: bool, inline_images: bool, file_path: Option<String>) -> io::Result<()> {
    // Initialize terminal
    let mut terminal = render::init_terminal().map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to initialize terminal: {}. Make sure you're running in an interactive terminal.", e)
        )
    })?;

    // Ensure terminal is ALWAYS restored, even on error
    let cleanup_result = (|| -> io::Result<()> {
        let size = terminal.size()?;
        let mut viewport = Viewport::new(size.width, size.height.saturating_sub(1)); // -1 for status bar

        // Make document mutable for reloading
        let mut document = initial_document.clone();

        // Layout document
        let mut tree = layout_document(&document, theme, viewport);

        // Disable sidebar if requested
        if no_images {
            tree.images.clear();
        }

        // Frame rate limiting - target 60 FPS
        let frame_duration = Duration::from_millis(16);
        let mut last_render = Instant::now();
        let mut needs_render = true;
        let mut show_help = false;
        let mut mouse_enabled = false;  // Start with mouse disabled for text selection
        let mut search_state = SearchState::new();

        // Main event loop
        loop {
            // Render only if needed and enough time has passed
            let now = Instant::now();
            if needs_render && now.duration_since(last_render) >= frame_duration {
                render::render(&mut terminal, &tree, theme, show_help, &search_state)?;
                last_render = now;
                needs_render = false;
            }

            // Poll for events with short timeout
            if event::poll(Duration::from_millis(16))? {
                match event::read()? {
                    Event::Key(key) => {
                        // Handle search mode input
                        if search_state.active {
                            match key.code {
                                KeyCode::Esc => {
                                    search_state.deactivate();
                                    needs_render = true;
                                }
                                KeyCode::Enter => {
                                    search_state.execute_search(&tree.root);
                                    search_state.accept(); // Exit input mode but keep results
                                    if let Some(m) = search_state.current_match() {
                                        tree.viewport.scroll_to_clamped(m.y.saturating_sub(5), tree.document_height());
                                    }
                                    needs_render = true;
                                }
                                KeyCode::Backspace => {
                                    search_state.backspace();
                                    search_state.execute_search(&tree.root);
                                    if let Some(m) = search_state.current_match() {
                                        tree.viewport.scroll_to_clamped(m.y.saturating_sub(5), tree.document_height());
                                    }
                                    needs_render = true;
                                }
                                KeyCode::Char(c) => {
                                    search_state.add_char(c);
                                    search_state.execute_search(&tree.root);
                                    if let Some(m) = search_state.current_match() {
                                        tree.viewport.scroll_to_clamped(m.y.saturating_sub(5), tree.document_height());
                                    }
                                    needs_render = true;
                                }
                                _ => {}
                            }
                        } else if key.code == KeyCode::Esc && !search_state.matches.is_empty() {
                            // Clear search results when Esc is pressed and we have results
                            search_state.deactivate();
                            needs_render = true;
                        } else if key.code == KeyCode::Char('/') {
                            // Activate search mode
                            search_state.activate();
                            needs_render = true;
                        } else if key.code == KeyCode::Char('h') {
                            show_help = !show_help;
                            needs_render = true;
                        } else if key.code == KeyCode::Char('m') {
                            // Toggle mouse mode
                            mouse_enabled = !mouse_enabled;
                            if mouse_enabled {
                                crossterm::execute!(io::stdout(), crossterm::event::EnableMouseCapture)?;
                            } else {
                                crossterm::execute!(io::stdout(), crossterm::event::DisableMouseCapture)?;
                            }
                            needs_render = true;
                        } else if key.code == KeyCode::Char('r') && file_path.is_some() {
                            // Reload file
                            let path = file_path.as_ref().unwrap();
                            match fs::read_to_string(path) {
                                Ok(markdown) => {
                                    let old_scroll = tree.viewport.scroll_y;
                                    document = parse_markdown(&markdown);
                                    tree = layout_document(&document, theme, viewport);
                                    if no_images {
                                        tree.images.clear();
                                    }
                                    // Try to preserve scroll position
                                    tree.viewport.scroll_to_clamped(old_scroll, tree.document_height());
                                    needs_render = true;
                                }
                                Err(e) => {
                                    // TODO: Show error message to user
                                    eprintln!("Failed to reload file: {}", e);
                                }
                            }
                        } else if show_help && key.code == KeyCode::Esc {
                            show_help = false;
                            needs_render = true;
                        } else if !show_help {
                            match handle_key(key, &mut tree, &mut search_state) {
                                Action::Quit => break,
                                Action::Continue => {
                                    needs_render = true;  // Mark that we need to render
                                }
                            }
                        }
                    }
                    Event::Mouse(mouse) => {
                        if !show_help && mouse_enabled {
                            if handle_mouse(mouse, &mut tree) {
                                needs_render = true;
                            }
                        }
                    }
                    Event::Resize(_, _) => {
                        let size = terminal.size()?;
                        viewport = Viewport::new(size.width, size.height.saturating_sub(1));
                        tree = layout_document(&document, theme, viewport);
                        needs_render = true;
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    })();

    // ALWAYS restore terminal, regardless of success or error
    let restore_result = render::restore_terminal(&mut terminal);

    // Return the first error that occurred
    cleanup_result.and(restore_result)
}

enum Action {
    Quit,
    Continue,
}

fn handle_mouse(mouse: MouseEvent, tree: &mut LayoutTree) -> bool {
    let doc_height = tree.document_height();

    match mouse.kind {
        MouseEventKind::ScrollDown => {
            tree.viewport.scroll_by_clamped(3, doc_height);
            true
        }
        MouseEventKind::ScrollUp => {
            tree.viewport.scroll_by_clamped(-3, doc_height);
            true
        }
        _ => false,
    }
}

fn handle_key(key: KeyEvent, tree: &mut LayoutTree, search_state: &mut SearchState) -> Action {
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
            // If we have search results, jump to next match
            if !search_state.matches.is_empty() {
                search_state.next_match();
                if let Some(m) = search_state.current_match() {
                    tree.viewport.scroll_to_clamped(m.y.saturating_sub(5), doc_height);
                }
            } else {
                // Otherwise jump to next heading
                jump_to_next_heading(tree, true);
            }
            Action::Continue
        }
        KeyCode::Char('N') => {
            // Jump to previous search match (Shift-N)
            if !search_state.matches.is_empty() {
                search_state.prev_match();
                if let Some(m) = search_state.current_match() {
                    tree.viewport.scroll_to_clamped(m.y.saturating_sub(5), doc_height);
                }
            }
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

