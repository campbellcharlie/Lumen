//! Lumen: Interactive Markdown viewer

use crossterm::event::{self, Event, KeyCode, KeyEvent, MouseEvent, MouseEventKind};
use lumen::ir::{Block, Inline};
use lumen::layout::{LayoutElement, Viewport};
use lumen::{
    layout_document, parse_markdown, render, FileManager, LayoutTree, Preferences, SearchState,
    Theme,
};
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Check if a document contains any images
fn document_has_images(document: &lumen::Document) -> bool {
    fn block_has_images(block: &Block) -> bool {
        match block {
            Block::Paragraph { content } | Block::Heading { content, .. } => {
                content.iter().any(inline_has_images)
            }
            Block::BlockQuote { blocks }
            | Block::Callout {
                content: blocks, ..
            } => blocks.iter().any(block_has_images),
            Block::List { items, .. } => items
                .iter()
                .any(|item| item.content.iter().any(block_has_images)),
            Block::Table { headers, rows, .. } => {
                headers
                    .iter()
                    .any(|cell| cell.content.iter().any(inline_has_images))
                    || rows.iter().any(|row| {
                        row.iter()
                            .any(|cell| cell.content.iter().any(inline_has_images))
                    })
            }
            _ => false,
        }
    }

    fn inline_has_images(inline: &Inline) -> bool {
        match inline {
            Inline::Image { .. } => true,
            Inline::Strong(inlines)
            | Inline::Emphasis(inlines)
            | Inline::Strikethrough(inlines) => inlines.iter().any(inline_has_images),
            Inline::Link { text, .. } => text.iter().any(inline_has_images),
            _ => false,
        }
    }

    document.blocks.iter().any(block_has_images)
}

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
    let inline_images = args
        .iter()
        .any(|arg| arg == "--inline-images" || arg == "-i");

    // Get non-flag arguments (skip program name)
    let non_flag_args: Vec<&String> = args
        .iter()
        .skip(1)
        .filter(|arg| !arg.starts_with('-'))
        .collect();

    // Check for file arguments
    if non_flag_args.is_empty() {
        eprintln!("Usage: lumen <file.md> [file2.md ...] [theme] [options]");
        eprintln!("\nOptions:");
        eprintln!("  --no-images, -n       Disable all image rendering");
        eprintln!("  --inline-images, -i   Render images inline (default: sidebar)");
        eprintln!("\nAvailable themes: {}", Theme::builtin_names().join(", "));
        eprintln!("\nExamples:");
        eprintln!("  lumen README.md");
        eprintln!("  lumen README.md CHANGELOG.md");
        eprintln!("  lumen README.md neon");
        eprintln!("  lumen README.md --inline-images");
        eprintln!("  lumen README.md --no-images");
        eprintln!("\nKeyboard shortcuts:");
        eprintln!("  Tab / Shift+Tab       Switch between open files");
        eprintln!("  1-9                   Jump to file 1-9");
        eprintln!("  :N                    Jump to file N (e.g., :44)");
        std::process::exit(1);
    }

    // Load user preferences
    let mut preferences = Preferences::load();

    // Separate file paths from theme name
    // Last non-md argument is potentially a theme name
    let mut file_paths = Vec::new();
    let mut theme_name_override: Option<&str> = None;

    for arg in &non_flag_args {
        if arg.ends_with(".md") || arg.ends_with(".markdown") {
            file_paths.push(arg.as_str());
        } else {
            // Assume it's a theme name
            theme_name_override = Some(arg.as_str());
        }
    }

    // If no .md files found, treat all as file paths (last one might be theme)
    if file_paths.is_empty() {
        if non_flag_args.len() == 1 {
            file_paths.push(non_flag_args[0]);
        } else {
            file_paths.extend(
                non_flag_args[..non_flag_args.len() - 1]
                    .iter()
                    .map(|s| s.as_str()),
            );
            theme_name_override = Some(non_flag_args[non_flag_args.len() - 1]);
        }
    }

    // Determine which theme to use: command line override or saved preference
    let theme_name = theme_name_override.unwrap_or(&preferences.theme);

    // Create file manager and load files
    let mut file_manager = FileManager::new();

    for file_path in &file_paths {
        let markdown = fs::read_to_string(file_path).unwrap_or_else(|e| {
            eprintln!("Error reading file '{}': {}", file_path, e);
            std::process::exit(1);
        });

        let document = parse_markdown(&markdown);
        file_manager.add_file(PathBuf::from(file_path), document);
    }

    // Load theme
    let theme = Theme::builtin(theme_name).unwrap_or_else(|| {
        eprintln!("Unknown theme '{}', using 'docs'", theme_name);
        Theme::builtin("docs").expect("Built-in 'docs' theme should always exist")
    });

    // Update preferences with the theme we're using
    preferences.theme = theme_name.to_string();

    // Initialize terminal and run - ensure cleanup happens even on error
    run_interactive(file_manager, theme, preferences, no_images, inline_images)
}

/// Run the interactive viewer with proper terminal cleanup
fn run_interactive(
    mut file_manager: FileManager,
    mut theme: Theme,
    mut preferences: Preferences,
    no_images: bool,
    inline_images: bool,
) -> io::Result<()> {
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
        let full_width = size.width;
        let height = size.height.saturating_sub(1); // -1 for status bar

        // File sidebar visibility (from preferences, can be toggled by user)
        let mut file_sidebar_visible = preferences.file_sidebar_visible;

        // Calculate layout based on sidebars
        // File sidebar: shown when multiple files open AND user hasn't hidden it (20% width on left)
        // Image sidebar: shown when document has images (30% width on right)
        let show_file_sidebar = file_manager.has_multiple_files() && file_sidebar_visible;

        let file_sidebar_width = if show_file_sidebar {
            (full_width * 20) / 100
        } else {
            0
        };

        // Get current document for image check
        let current_file = file_manager
            .current_file()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No files open"))?;

        let has_images =
            !inline_images && !no_images && document_has_images(&current_file.document);
        let image_sidebar_width = if has_images {
            (full_width * 30) / 100
        } else {
            0
        };

        // Content area is remainder
        let layout_width = full_width.saturating_sub(file_sidebar_width + image_sidebar_width);

        let mut viewport = Viewport::new(layout_width, height);

        // Layout current document
        let mut tree = {
            let current_file = file_manager
                .current_file()
                .expect("Bug: file_manager should always have at least one file");
            layout_document(&current_file.document, &theme, viewport, inline_images)
        };

        // Disable image sidebar if requested
        if no_images {
            tree.images.clear();
        }

        // Frame rate limiting - target 60 FPS
        let frame_duration = Duration::from_millis(16);
        let mut last_render = Instant::now();
        let mut needs_render = true;
        let mut show_help = false;
        let mut mouse_enabled = preferences.mouse_enabled;
        let mut search_state = SearchState::new();
        let mut file_jump_buffer = String::new(); // Buffer for typing file numbers
        let mut file_jump_mode = false; // Whether we're in file jump mode

        // Enable mouse capture if preference is set
        if mouse_enabled {
            crossterm::execute!(io::stdout(), crossterm::event::EnableMouseCapture)?;
        }

        // Main event loop
        loop {
            // Render only if needed and enough time has passed
            let now = Instant::now();
            if needs_render && now.duration_since(last_render) >= frame_duration {
                let show_file_sidebar = file_manager.has_multiple_files() && file_sidebar_visible;
                render::render(
                    &mut terminal,
                    &tree,
                    &theme,
                    show_help,
                    &search_state,
                    &file_manager,
                    show_file_sidebar,
                    file_jump_mode,
                    &file_jump_buffer,
                )?;
                last_render = now;
                needs_render = false;
            }

            // Poll for events with short timeout
            if event::poll(Duration::from_millis(16))? {
                match event::read()? {
                    Event::Key(key) => {
                        // Handle file jump mode input
                        if file_jump_mode {
                            match key.code {
                                KeyCode::Esc => {
                                    file_jump_mode = false;
                                    file_jump_buffer.clear();
                                    needs_render = true;
                                }
                                KeyCode::Enter => {
                                    if let Ok(file_num) = file_jump_buffer.parse::<usize>() {
                                        if file_num > 0 && file_num <= file_manager.file_count() {
                                            // Save current scroll before switching
                                            file_manager
                                                .save_scroll_position(tree.viewport.scroll_y);

                                            // Switch to new file
                                            file_manager.switch_to(file_num - 1);

                                            // Recalculate layout
                                            (viewport, tree) = recalculate_layout(
                                                &file_manager,
                                                &terminal,
                                                &theme,
                                                file_sidebar_visible,
                                                no_images,
                                                inline_images,
                                            )?;

                                            // Restore saved scroll for the new file
                                            let saved_scroll = file_manager.get_scroll_position();
                                            tree.viewport.scroll_to_clamped(
                                                saved_scroll,
                                                tree.document_height(),
                                            );

                                            // Clear search state when switching files
                                            search_state.deactivate();
                                        }
                                    }
                                    file_jump_mode = false;
                                    file_jump_buffer.clear();
                                    needs_render = true;
                                }
                                KeyCode::Backspace => {
                                    file_jump_buffer.pop();
                                    needs_render = true;
                                }
                                KeyCode::Char(c) if c.is_ascii_digit() => {
                                    file_jump_buffer.push(c);
                                    needs_render = true;
                                }
                                _ => {}
                            }
                        } else if search_state.active {
                            match key.code {
                                KeyCode::Esc => {
                                    search_state.deactivate();
                                    needs_render = true;
                                }
                                KeyCode::Enter => {
                                    search_state.execute_search(&tree.root);
                                    search_state.accept(); // Exit input mode but keep results
                                    if let Some(m) = search_state.current_match() {
                                        tree.viewport.scroll_to_clamped(
                                            m.y.saturating_sub(5),
                                            tree.document_height(),
                                        );
                                    }
                                    needs_render = true;
                                }
                                KeyCode::Backspace => {
                                    search_state.backspace();
                                    search_state.execute_search(&tree.root);
                                    if let Some(m) = search_state.current_match() {
                                        tree.viewport.scroll_to_clamped(
                                            m.y.saturating_sub(5),
                                            tree.document_height(),
                                        );
                                    }
                                    needs_render = true;
                                }
                                KeyCode::Char(c) => {
                                    search_state.add_char(c);
                                    search_state.execute_search(&tree.root);
                                    if let Some(m) = search_state.current_match() {
                                        tree.viewport.scroll_to_clamped(
                                            m.y.saturating_sub(5),
                                            tree.document_height(),
                                        );
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
                        } else if key.code == KeyCode::Char(':') && file_manager.file_count() > 1 {
                            // Activate file jump mode
                            file_jump_mode = true;
                            file_jump_buffer.clear();
                            needs_render = true;
                        } else if key.code == KeyCode::Char('h') {
                            show_help = !show_help;
                            needs_render = true;
                        } else if key.code == KeyCode::Char('m') {
                            // Toggle mouse mode
                            mouse_enabled = !mouse_enabled;
                            preferences.mouse_enabled = mouse_enabled;
                            let _ = preferences.save();

                            if mouse_enabled {
                                crossterm::execute!(
                                    io::stdout(),
                                    crossterm::event::EnableMouseCapture
                                )?;
                            } else {
                                crossterm::execute!(
                                    io::stdout(),
                                    crossterm::event::DisableMouseCapture
                                )?;
                            }
                            needs_render = true;
                        } else if key.code == KeyCode::Char('f') && file_manager.file_count() > 1 {
                            // Toggle file sidebar visibility
                            file_sidebar_visible = !file_sidebar_visible;
                            preferences.file_sidebar_visible = file_sidebar_visible;
                            let _ = preferences.save();

                            // Save current scroll before recalculating
                            let old_scroll = tree.viewport.scroll_y;

                            // Recalculate layout with new sidebar state
                            (viewport, tree) = recalculate_layout(
                                &file_manager,
                                &terminal,
                                &theme,
                                file_sidebar_visible,
                                no_images,
                                inline_images,
                            )?;

                            // Restore scroll position
                            tree.viewport
                                .scroll_to_clamped(old_scroll, tree.document_height());
                            needs_render = true;
                        } else if key.code == KeyCode::Char('t') {
                            // Cycle to next theme
                            let theme_names = Theme::builtin_names();
                            let current_index = theme_names
                                .iter()
                                .position(|&n| n == preferences.theme)
                                .unwrap_or(0);
                            let next_index = (current_index + 1) % theme_names.len();
                            preferences.theme = theme_names[next_index].to_string();

                            // Load new theme
                            theme = Theme::builtin(&preferences.theme)
                                .expect("Built-in theme from theme_names should always exist");

                            // Save preferences
                            let _ = preferences.save();

                            // Save current scroll
                            let old_scroll = tree.viewport.scroll_y;

                            // Recalculate layout with new theme
                            (viewport, tree) = recalculate_layout(
                                &file_manager,
                                &terminal,
                                &theme,
                                file_sidebar_visible,
                                no_images,
                                inline_images,
                            )?;

                            // Restore scroll position
                            tree.viewport
                                .scroll_to_clamped(old_scroll, tree.document_height());
                            needs_render = true;
                        } else if key.code == KeyCode::Tab {
                            // Save current scroll before switching
                            file_manager.save_scroll_position(tree.viewport.scroll_y);

                            // Switch to next file
                            file_manager.next_file();

                            // Recalculate layout
                            (viewport, tree) = recalculate_layout(
                                &file_manager,
                                &terminal,
                                &theme,
                                file_sidebar_visible,
                                no_images,
                                inline_images,
                            )?;

                            // Restore saved scroll for the new file
                            let saved_scroll = file_manager.get_scroll_position();
                            tree.viewport
                                .scroll_to_clamped(saved_scroll, tree.document_height());

                            // Clear search state when switching files
                            search_state.deactivate();
                            needs_render = true;
                        } else if key.code == KeyCode::BackTab {
                            // Save current scroll before switching
                            file_manager.save_scroll_position(tree.viewport.scroll_y);

                            // Switch to previous file (Shift+Tab)
                            file_manager.prev_file();

                            // Recalculate layout
                            (viewport, tree) = recalculate_layout(
                                &file_manager,
                                &terminal,
                                &theme,
                                file_sidebar_visible,
                                no_images,
                                inline_images,
                            )?;

                            // Restore saved scroll for the new file
                            let saved_scroll = file_manager.get_scroll_position();
                            tree.viewport
                                .scroll_to_clamped(saved_scroll, tree.document_height());

                            // Clear search state when switching files
                            search_state.deactivate();
                            needs_render = true;
                        } else if key.code >= KeyCode::Char('1')
                            && key.code <= KeyCode::Char('9')
                            && file_manager.file_count() > 1
                        {
                            // Jump to file by number (1-9)
                            if let KeyCode::Char(c) = key.code {
                                let index = (c as u8 - b'1') as usize;
                                if index < file_manager.file_count() {
                                    // Save current scroll before switching
                                    file_manager.save_scroll_position(tree.viewport.scroll_y);

                                    // Switch to file
                                    file_manager.switch_to(index);

                                    // Recalculate layout
                                    (viewport, tree) = recalculate_layout(
                                        &file_manager,
                                        &terminal,
                                        &theme,
                                        file_sidebar_visible,
                                        no_images,
                                        inline_images,
                                    )?;

                                    // Restore saved scroll for the new file
                                    let saved_scroll = file_manager.get_scroll_position();
                                    tree.viewport
                                        .scroll_to_clamped(saved_scroll, tree.document_height());

                                    // Clear search state when switching files
                                    search_state.deactivate();
                                    needs_render = true;
                                }
                            }
                        } else if key.code == KeyCode::Char('r') {
                            // Save scroll before reload
                            let old_scroll = tree.viewport.scroll_y;

                            if let Err(e) = file_manager.reload_current() {
                                eprintln!("Failed to reload file: {}", e);
                            } else {
                                // Recalculate layout
                                (viewport, tree) = recalculate_layout(
                                    &file_manager,
                                    &terminal,
                                    &theme,
                                    file_sidebar_visible,
                                    no_images,
                                    inline_images,
                                )?;

                                // Restore scroll position
                                tree.viewport
                                    .scroll_to_clamped(old_scroll, tree.document_height());
                                needs_render = true;
                            }
                        } else if show_help && key.code == KeyCode::Esc {
                            show_help = false;
                            needs_render = true;
                        } else if !show_help {
                            match handle_key(key, &mut tree, &mut search_state) {
                                Action::Quit => break,
                                Action::Continue => {
                                    needs_render = true; // Mark that we need to render
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
                        // Save scroll before resize
                        let old_scroll = tree.viewport.scroll_y;

                        // Recalculate layout with new terminal size
                        (viewport, tree) = recalculate_layout(
                            &file_manager,
                            &terminal,
                            &theme,
                            file_sidebar_visible,
                            no_images,
                            inline_images,
                        )?;

                        // Restore scroll position
                        tree.viewport
                            .scroll_to_clamped(old_scroll, tree.document_height());
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

/// Helper function to recalculate layout for the current file
/// Returns (viewport, tree) tuple
fn recalculate_layout(
    file_manager: &FileManager,
    terminal: &render::Terminal,
    theme: &Theme,
    file_sidebar_visible: bool,
    no_images: bool,
    inline_images: bool,
) -> io::Result<(Viewport, LayoutTree)> {
    let size = terminal.size()?;
    let current_file = file_manager
        .current_file()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No files open"))?;

    let show_file_sidebar = file_manager.has_multiple_files() && file_sidebar_visible;
    let file_sidebar_width = if show_file_sidebar {
        (size.width * 20) / 100
    } else {
        0
    };
    let has_images = !inline_images && !no_images && document_has_images(&current_file.document);
    let image_sidebar_width = if has_images {
        (size.width * 30) / 100
    } else {
        0
    };
    let layout_width = size
        .width
        .saturating_sub(file_sidebar_width + image_sidebar_width);
    let viewport = Viewport::new(layout_width, size.height.saturating_sub(1));

    let mut tree = layout_document(&current_file.document, theme, viewport, inline_images);
    if no_images {
        tree.images.clear();
    }

    Ok((viewport, tree))
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
            tree.viewport
                .scroll_by_clamped(tree.viewport.height as i16 / 2, doc_height);
            Action::Continue
        }
        KeyCode::Char('u') => {
            tree.viewport
                .scroll_by_clamped(-(tree.viewport.height as i16 / 2), doc_height);
            Action::Continue
        }
        KeyCode::PageDown | KeyCode::Char(' ') => {
            tree.viewport
                .scroll_by_clamped(tree.viewport.height as i16, doc_height);
            Action::Continue
        }
        KeyCode::PageUp => {
            tree.viewport
                .scroll_by_clamped(-(tree.viewport.height as i16), doc_height);
            Action::Continue
        }
        KeyCode::Home | KeyCode::Char('g') => {
            tree.viewport.scroll_to(0);
            Action::Continue
        }
        KeyCode::End | KeyCode::Char('G') => {
            tree.viewport
                .scroll_to_clamped(tree.document_height(), doc_height);
            Action::Continue
        }
        KeyCode::Char('n') => {
            // If we have search results, jump to next match
            if !search_state.matches.is_empty() {
                search_state.next_match();
                if let Some(m) = search_state.current_match() {
                    scroll_to_search_match(tree, m.y, doc_height);
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
                    scroll_to_search_match(tree, m.y, doc_height);
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

/// Scroll viewport to center a search match with some padding from the top
fn scroll_to_search_match(tree: &mut LayoutTree, match_y: u16, doc_height: u16) {
    // Position match 5 lines from top of viewport for context
    tree.viewport
        .scroll_to_clamped(match_y.saturating_sub(5), doc_height);
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
