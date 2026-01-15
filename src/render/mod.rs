//! Terminal rendering

use crate::layout::{ImageReference, LayoutElement, LayoutNode, LayoutTree, Line, TextSegment};
use crate::search::SearchState;
use crate::theme::{BorderStyle, Color, FontStyle, FontWeight, Theme};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color as RatatuiColor, Modifier, Style},
    text::{Span, Text as RatatuiText},
    widgets::{Block, Borders, Clear, Paragraph},
    Terminal as RatatuiTerminal,
};
use ratatui_image::{picker::Picker, protocol::StatefulProtocol, StatefulImage};
use std::fs;
use std::io;
use std::path::Path;

pub type Terminal = RatatuiTerminal<CrosstermBackend<io::Stdout>>;

/// Initialize terminal for rendering
pub fn init_terminal() -> io::Result<Terminal> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::Hide
    )?;
    let backend = CrosstermBackend::new(stdout);
    RatatuiTerminal::new(backend)
}

/// Restore terminal to normal state
pub fn restore_terminal(terminal: &mut Terminal) -> io::Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture,
        crossterm::cursor::Show
    )?;
    Ok(())
}

/// Render a layout tree to the terminal with all UI elements.
///
/// This is the main rendering function that draws the entire UI including:
/// - The document content with syntax highlighting
/// - File sidebar (if enabled)
/// - Image sidebar (if images are present)
/// - Search results highlighting
/// - Help menu overlay (if shown)
/// - Status bar with navigation info
///
/// # Arguments
///
/// * `terminal` - The ratatui terminal instance to draw to
/// * `tree` - Positioned layout tree from `layout_document`
/// * `theme` - Theme for colors, borders, and styling
/// * `show_help` - Whether to show the help menu overlay
/// * `search_state` - Current search state (query, matches, selection)
/// * `file_manager` - File manager with open files
/// * `show_file_sidebar` - Whether to show the file navigation sidebar
/// * `file_jump_mode` - Whether file jump mode is active (entering file number)
/// * `file_jump_buffer` - Buffer containing the file number being entered
///
/// # Returns
///
/// `Ok(())` on successful render, or an I/O error if drawing fails.
///
/// # Example
///
/// ```no_run
/// use lumen::render::{init_terminal, render};
/// use lumen::{parse_markdown, layout_document, Theme, FileManager};
/// use lumen::layout::Viewport;
/// use lumen::search::SearchState;
///
/// let mut terminal = init_terminal().unwrap();
/// let markdown = "# Hello";
/// let doc = parse_markdown(markdown);
/// let theme = Theme::builtin("docs").unwrap();
/// let viewport = Viewport::new(80, 24);
/// let tree = layout_document(&doc, &theme, viewport, false);
/// let mut file_manager = FileManager::new();
/// let search_state = SearchState::new();
///
/// render(&mut terminal, &tree, &theme, false, &search_state,
///        &file_manager, false, false, "", None).unwrap();
/// ```
pub fn render(
    terminal: &mut Terminal,
    tree: &LayoutTree,
    theme: &Theme,
    show_help: bool,
    search_state: &SearchState,
    file_manager: &crate::FileManager,
    show_file_sidebar: bool,
    file_jump_mode: bool,
    file_jump_buffer: &str,
    selected_link_index: Option<usize>,
) -> io::Result<()> {
    terminal.draw(|frame| {
        let area = frame.area();

        // Calculate layout areas
        let (file_sidebar_area, remaining_area) = if show_file_sidebar {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(20), // File sidebar
                    Constraint::Percentage(80), // Content + images
                ])
                .split(area);
            (Some(chunks[0]), chunks[1])
        } else {
            (None, area)
        };

        // Split remaining area for content and images
        let (content_area, image_sidebar_area) = if !tree.images.is_empty() {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(70), // Main content (of remaining area)
                    Constraint::Percentage(30), // Image sidebar
                ])
                .split(remaining_area);
            (chunks[0], Some(chunks[1]))
        } else {
            (remaining_area, None)
        };

        // Render document starting from scroll position
        let scroll_y = tree.viewport.scroll_y;

        // Calculate X offset for content (to account for file sidebar)
        let content_x_offset = content_area.x;

        // Get selected link region if any
        let selected_link_rect = selected_link_index.and_then(|idx| {
            tree.hit_regions
                .iter()
                .filter(|r| matches!(r.element, crate::layout::HitElement::Link { .. }))
                .nth(idx)
                .map(|r| r.rect)
        });

        for node in &tree.root.children {
            render_node(
                frame,
                node,
                theme,
                scroll_y,
                content_area,
                search_state,
                content_x_offset,
                selected_link_rect,
            );
        }

        // Render file sidebar if present
        if let Some(file_sidebar) = file_sidebar_area {
            render_file_sidebar(frame, file_manager, file_sidebar, theme);
        }

        // Render images in sidebar if present
        if let Some(image_sidebar) = image_sidebar_area {
            render_image_sidebar(frame, &tree.images, scroll_y, image_sidebar, theme);
        }

        // Render status bar (use full area width)
        render_status_bar(
            frame,
            tree,
            area,
            search_state,
            file_jump_mode,
            file_jump_buffer,
        );

        // Render help menu if active
        if show_help {
            render_help_menu(frame, area);
        }
    })?;
    Ok(())
}

fn render_node(
    frame: &mut ratatui::Frame,
    node: &LayoutNode,
    theme: &Theme,
    scroll_y: u16,
    area: ratatui::layout::Rect,
    search_state: &SearchState,
    x_offset: u16,
    selected_link_rect: Option<crate::layout::Rectangle>,
) {
    // Calculate display position
    let display_y = node.rect.y.saturating_sub(scroll_y);

    // For container elements (callouts, blockquotes, lists), check if ANY part is visible
    let is_container = matches!(
        node.element,
        LayoutElement::Callout { .. }
            | LayoutElement::BlockQuote
            | LayoutElement::CodeBlock { .. }
            | LayoutElement::List { .. }
            | LayoutElement::ListItem { .. }
            | LayoutElement::Table { .. }
            | LayoutElement::TableRow { .. }
    );

    if is_container {
        // For containers, check if the bottom is above viewport or top is below viewport
        let node_bottom = node.rect.y + node.rect.height;
        if node_bottom <= scroll_y || node.rect.y >= scroll_y + area.height {
            return; // Completely off-screen
        }
    } else {
        // For leaf elements, check if top is visible
        if node.rect.y < scroll_y || display_y >= area.height {
            return; // Above or below viewport
        }
    }

    match &node.element {
        LayoutElement::Heading { level, text } => {
            render_heading(frame, node, *level, text, theme, display_y, area, x_offset);
        }
        LayoutElement::Paragraph { lines } => {
            render_paragraph(
                frame,
                lines,
                theme,
                node.rect.x + x_offset,
                display_y,
                area,
                node.rect.y,
                scroll_y,
                search_state,
                selected_link_rect,
            );
            // Render children (e.g., inline images)
            for child in &node.children {
                render_node(
                    frame,
                    child,
                    theme,
                    scroll_y,
                    area,
                    search_state,
                    x_offset,
                    selected_link_rect,
                );
            }
        }
        LayoutElement::CodeBlock { lang, lines } => {
            render_code_block(
                frame,
                lang,
                lines,
                theme,
                node.rect.x + x_offset,
                display_y,
                node.rect.width,
                area,
                node.rect.y,
                scroll_y,
            );
        }
        LayoutElement::List { .. } => {
            for child in &node.children {
                render_node(
                    frame,
                    child,
                    theme,
                    scroll_y,
                    area,
                    search_state,
                    x_offset,
                    selected_link_rect,
                );
            }
        }
        LayoutElement::ListItem { marker, .. } => {
            // Check if this is an empty parent (only contains a nested list, no content)
            let is_empty_parent = node.children.len() == 1
                && matches!(node.children[0].element, LayoutElement::List { .. });

            // Only render marker if this item has actual content (not just a nested list)
            if !is_empty_parent {
                // Render marker with style
                let marker_style =
                    Style::default().fg(to_ratatui_color(theme.blocks.list.marker_color));

                // Calculate display width (not byte length)
                // The bullet "•" is 3 bytes but displays as 1 char width
                let marker_display_width = marker.chars().count() as u16;

                // Calculate allocated marker space based on where the content starts
                // This allows us to right-align the marker for consistent alignment
                let allocated_marker_space = if let Some(first_child) = node.children.first() {
                    // Content starts at first_child.rect.x, with 1-space gap after marker
                    first_child
                        .rect
                        .x
                        .saturating_sub(node.rect.x)
                        .saturating_sub(1)
                } else {
                    marker_display_width
                };

                // Right-align the marker within the allocated space
                let marker_x_offset = allocated_marker_space.saturating_sub(marker_display_width);

                let marker_area = ratatui::layout::Rect {
                    x: node.rect.x + x_offset + marker_x_offset,
                    y: display_y,
                    width: marker_display_width,
                    height: 1,
                };

                frame.render_widget(
                    Paragraph::new(RatatuiText::from(Span::styled(
                        marker.as_str(),
                        marker_style,
                    ))),
                    marker_area,
                );
            }

            // Render children - they are positioned by the layout
            for child in &node.children {
                render_node(
                    frame,
                    child,
                    theme,
                    scroll_y,
                    area,
                    search_state,
                    x_offset,
                    selected_link_rect,
                );
            }
        }
        LayoutElement::BlockQuote => {
            // Draw left border manually using vertical line characters
            let border_style = Style::default().fg(to_ratatui_color(theme.blocks.blockquote.color));
            let border_height = node.rect.height.min(area.height.saturating_sub(display_y));

            for i in 0..border_height {
                let border_line = Span::styled("│", border_style);
                let border_area = ratatui::layout::Rect {
                    x: node.rect.x + x_offset,
                    y: display_y + i,
                    width: 1,
                    height: 1,
                };
                frame.render_widget(Paragraph::new(RatatuiText::from(border_line)), border_area);
            }

            // Render children
            for child in &node.children {
                render_node(
                    frame,
                    child,
                    theme,
                    scroll_y,
                    area,
                    search_state,
                    x_offset,
                    selected_link_rect,
                );
            }
        }
        LayoutElement::Callout { kind } => {
            use crate::ir::CalloutKind;

            // Get style for this callout type
            let callout_style = match kind {
                CalloutKind::Note => &theme.blocks.callout.note,
                CalloutKind::Warning => &theme.blocks.callout.warning,
                CalloutKind::Important => &theme.blocks.callout.important,
                CalloutKind::Tip => &theme.blocks.callout.tip,
                CalloutKind::Caution => &theme.blocks.callout.caution,
            };

            // Calculate visible portion of callout
            let visible_start_y = if node.rect.y < scroll_y {
                0 // Callout starts above viewport
            } else {
                display_y
            };

            let visible_height = if node.rect.y < scroll_y {
                // Callout extends above viewport, calculate visible portion
                let hidden_lines = scroll_y - node.rect.y;
                node.rect
                    .height
                    .saturating_sub(hidden_lines)
                    .min(area.height)
            } else {
                // Callout starts in viewport
                node.rect.height.min(area.height.saturating_sub(display_y))
            };

            // Render background if specified (using filled spaces instead of Block)
            if let Some(bg) = callout_style.background {
                let bg_style = Style::default().bg(to_ratatui_color(bg));
                for i in 0..visible_height {
                    let bg_line = " ".repeat(node.rect.width as usize);
                    let bg_span = Span::styled(bg_line, bg_style);
                    let bg_line_area = ratatui::layout::Rect {
                        x: node.rect.x + x_offset,
                        y: visible_start_y + i,
                        width: node.rect.width,
                        height: 1,
                    };
                    frame.render_widget(Paragraph::new(RatatuiText::from(bg_span)), bg_line_area);
                }
            }

            // Draw left border manually using vertical line characters
            let border_style = Style::default().fg(to_ratatui_color(callout_style.border_color));
            for i in 0..visible_height {
                let border_line = Span::styled("│", border_style);
                let border_area = ratatui::layout::Rect {
                    x: node.rect.x + x_offset,
                    y: visible_start_y + i,
                    width: 1,
                    height: 1,
                };
                frame.render_widget(Paragraph::new(RatatuiText::from(border_line)), border_area);
            }

            // Render icon at the top left (only if the top of the callout is visible)
            if node.rect.y >= scroll_y {
                let icon_span = Span::styled(
                    &callout_style.icon,
                    Style::default().fg(to_ratatui_color(callout_style.color)),
                );
                let icon_area = ratatui::layout::Rect {
                    x: node.rect.x + x_offset,
                    y: display_y,
                    width: 2,
                    height: 1,
                };
                frame.render_widget(Paragraph::new(RatatuiText::from(icon_span)), icon_area);
            }

            // Render children
            for child in &node.children {
                render_node(
                    frame,
                    child,
                    theme,
                    scroll_y,
                    area,
                    search_state,
                    x_offset,
                    selected_link_rect,
                );
            }
        }
        LayoutElement::Table { .. } => {
            let border_chars = get_border_chars(theme.blocks.table.border_style);
            let border_color = theme.colors.foreground;
            let border_style = Style::default().fg(to_ratatui_color(border_color));

            // Get column separator positions from first row (adjusted for offset)
            // Skip first cell since we only need internal separator positions for T-junctions
            let column_positions: Vec<u16> = if let Some(first_row) = node.children.first() {
                if let LayoutElement::TableRow { .. } = first_row.element {
                    first_row
                        .children
                        .iter()
                        .skip(1) // Skip first cell - we only want internal separators
                        .map(|cell| cell.rect.x + x_offset)
                        .collect()
                } else {
                    vec![]
                }
            } else {
                vec![]
            };

            let table_right = node.rect.x + x_offset + node.rect.width - 1;

            // Render table rows first
            // (top border will be drawn after to avoid being overwritten by vertical bars)
            for (i, child) in node.children.iter().enumerate() {
                render_node(
                    frame,
                    child,
                    theme,
                    scroll_y,
                    area,
                    search_state,
                    x_offset,
                    selected_link_rect,
                );

                // Draw row separator or bottom border
                let row_bottom_y = child.rect.y + child.rect.height;
                let is_last_row = i == node.children.len() - 1;
                let draw_separator = is_last_row || theme.blocks.table.row_separator;

                if draw_separator
                    && row_bottom_y >= scroll_y
                    && row_bottom_y < scroll_y + area.height
                {
                    let sep_display_y = row_bottom_y - scroll_y;

                    if sep_display_y < area.height {
                        // Build separator line segment by segment
                        let mut sep_line = String::new();
                        let table_left = node.rect.x + x_offset;

                        if is_last_row {
                            // Bottom border
                            sep_line.push_str(border_chars.bottom_left);

                            // For each column position, add segment and T-up junction
                            for i in 0..column_positions.len() {
                                let start_x = if i == 0 {
                                    table_left
                                } else {
                                    column_positions[i - 1]
                                };
                                let end_x = column_positions[i];
                                let segment_width = (end_x - start_x - 1) as usize;
                                sep_line.push_str(&border_chars.horizontal.repeat(segment_width));
                                sep_line.push_str(border_chars.t_up);
                            }

                            // Final segment to right edge (minus 1 for corner)
                            if let Some(&last_col) = column_positions.last() {
                                let final_width = (table_right - last_col - 1) as usize;
                                sep_line.push_str(&border_chars.horizontal.repeat(final_width));
                            } else {
                                let full_width = (table_right - table_left - 1) as usize;
                                sep_line.push_str(&border_chars.horizontal.repeat(full_width));
                            }

                            sep_line.push_str(border_chars.bottom_right);
                        } else {
                            // Middle row separator
                            sep_line.push_str(border_chars.t_right);

                            // For each column position, add segment and cross junction
                            for i in 0..column_positions.len() {
                                let start_x = if i == 0 {
                                    table_left
                                } else {
                                    column_positions[i - 1]
                                };
                                let end_x = column_positions[i];
                                let segment_width = (end_x - start_x - 1) as usize;
                                sep_line.push_str(&border_chars.horizontal.repeat(segment_width));
                                sep_line.push_str(border_chars.cross);
                            }

                            // Final segment to right edge (minus 1 for t_left junction)
                            if let Some(&last_col) = column_positions.last() {
                                let final_width = (table_right - last_col - 1) as usize;
                                sep_line.push_str(&border_chars.horizontal.repeat(final_width));
                            } else {
                                let full_width = (table_right - table_left - 1) as usize;
                                sep_line.push_str(&border_chars.horizontal.repeat(full_width));
                            }

                            sep_line.push_str(border_chars.t_left);
                        }

                        let sep_area = ratatui::layout::Rect {
                            x: table_left,
                            y: sep_display_y,
                            width: (table_right - table_left + 1), // Full table width
                            height: 1,
                        };
                        frame.render_widget(
                            Paragraph::new(RatatuiText::from(Span::styled(sep_line, border_style))),
                            sep_area,
                        );
                    }
                }
            }

            // Draw top border AFTER rows to avoid being overwritten
            if display_y < area.height {
                // Build top border line segment by segment
                let mut top_line = String::new();

                let table_left = node.rect.x + x_offset;

                // Start with top-left corner
                top_line.push_str(border_chars.top_left);

                // For each adjacent pair of column positions, add horizontal segment and T-junction
                for i in 0..column_positions.len() {
                    let start_x = if i == 0 {
                        table_left
                    } else {
                        column_positions[i - 1]
                    };
                    let end_x = column_positions[i];
                    let segment_width = (end_x - start_x - 1) as usize;
                    top_line.push_str(&border_chars.horizontal.repeat(segment_width));
                    top_line.push_str(border_chars.t_down);
                }

                // Add final segment from last column to right edge (minus 1 for corner)
                if let Some(&last_col) = column_positions.last() {
                    let final_width = (table_right - last_col - 1) as usize;
                    top_line.push_str(&border_chars.horizontal.repeat(final_width));
                } else {
                    // No columns, just fill the space (minus 2 for both corners)
                    let full_width = (table_right - table_left - 1) as usize;
                    top_line.push_str(&border_chars.horizontal.repeat(full_width));
                }

                // End with top-right corner
                top_line.push_str(border_chars.top_right);

                let top_area = ratatui::layout::Rect {
                    x: table_left,
                    y: display_y,
                    width: (table_right - table_left + 1), // Full table width
                    height: 1,
                };
                frame.render_widget(
                    Paragraph::new(RatatuiText::from(Span::styled(top_line, border_style))),
                    top_area,
                );
            }
        }
        LayoutElement::TableRow { is_header: _ } => {
            let border_chars = get_border_chars(theme.blocks.table.border_style);
            let border_color = theme.colors.foreground;
            let border_style = Style::default().fg(to_ratatui_color(border_color));

            // Render cell content first
            for child in &node.children {
                render_node(
                    frame,
                    child,
                    theme,
                    scroll_y,
                    area,
                    search_state,
                    x_offset,
                    selected_link_rect,
                );
            }

            // Collect column X positions (adjusted for offset)
            let column_positions: Vec<u16> = node
                .children
                .iter()
                .map(|cell| cell.rect.x + x_offset)
                .collect();
            let table_right = if let Some(last_cell) = node.children.last() {
                last_cell.rect.x + x_offset + last_cell.rect.width - 1
            } else {
                node.rect.x + x_offset + node.rect.width - 1
            };

            // Draw vertical borders for the entire row
            let row_top_y = node.rect.y.max(scroll_y);
            let row_bottom_y = (node.rect.y + node.rect.height).min(scroll_y + area.height);

            for y in row_top_y..row_bottom_y {
                if y >= scroll_y && y < scroll_y + area.height {
                    let border_display_y = y - scroll_y;

                    // Draw vertical border at each column position
                    for &col_x in &column_positions {
                        let border_area = ratatui::layout::Rect {
                            x: col_x,
                            y: border_display_y,
                            width: 1,
                            height: 1,
                        };
                        frame.render_widget(
                            Paragraph::new(RatatuiText::from(Span::styled(
                                border_chars.vertical,
                                border_style,
                            ))),
                            border_area,
                        );
                    }

                    // Draw right border of table
                    let right_border_area = ratatui::layout::Rect {
                        x: table_right,
                        y: border_display_y,
                        width: 1,
                        height: 1,
                    };
                    frame.render_widget(
                        Paragraph::new(RatatuiText::from(Span::styled(
                            border_chars.vertical,
                            border_style,
                        ))),
                        right_border_area,
                    );
                }
            }
        }
        LayoutElement::TableCell => {
            // Just render cell content - borders handled by TableRow
            for child in &node.children {
                render_node(
                    frame,
                    child,
                    theme,
                    scroll_y,
                    area,
                    search_state,
                    x_offset,
                    selected_link_rect,
                );
            }
        }
        LayoutElement::HorizontalRule => {
            // Use the content area width, not the full layout width
            // (important when sidebar is present)
            let hr_width = area.width.saturating_sub(node.rect.x + x_offset);
            let hr = "─".repeat(hr_width as usize);
            let hr_text = RatatuiText::from(hr);

            let hr_area = ratatui::layout::Rect {
                x: node.rect.x + x_offset,
                y: display_y,
                width: hr_width,
                height: 1,
            };

            frame.render_widget(Paragraph::new(hr_text), hr_area);
        }
        LayoutElement::Image { path, alt_text } => {
            // Render inline image
            render_inline_image(
                frame,
                path,
                alt_text,
                node.rect.x + x_offset,
                display_y,
                node.rect.width,
                node.rect.height,
                area,
            );
        }
        _ => {
            // Render children for other types
            for child in &node.children {
                render_node(
                    frame,
                    child,
                    theme,
                    scroll_y,
                    area,
                    search_state,
                    x_offset,
                    selected_link_rect,
                );
            }
        }
    }
}

fn render_heading(
    frame: &mut ratatui::Frame,
    node: &LayoutNode,
    level: u8,
    text: &str,
    theme: &Theme,
    display_y: u16,
    _area: ratatui::layout::Rect,
    x_offset: u16,
) {
    let heading_style = match level {
        1 => &theme.blocks.heading.h1,
        2 => &theme.blocks.heading.h2,
        3 => &theme.blocks.heading.h3,
        4 => &theme.blocks.heading.h4,
        5 => &theme.blocks.heading.h5,
        _ => &theme.blocks.heading.h6,
    };

    let mut style = Style::default().fg(to_ratatui_color(heading_style.color));
    if level <= 2 {
        style = style.add_modifier(Modifier::BOLD);
    }

    let mut full_text = String::new();
    if let Some(prefix) = &heading_style.prefix {
        full_text.push_str(prefix);
    }
    full_text.push_str(text);

    let span = Span::styled(full_text, style);
    let para = Paragraph::new(RatatuiText::from(span));

    let heading_area = ratatui::layout::Rect {
        x: node.rect.x + x_offset,
        y: display_y,
        width: node.rect.width,
        height: 1,
    };

    frame.render_widget(para, heading_area);
}

fn render_paragraph(
    frame: &mut ratatui::Frame,
    lines: &[Line],
    theme: &Theme,
    x: u16,
    display_y: u16,
    area: ratatui::layout::Rect,
    node_y: u16,
    _scroll_y: u16,
    search_state: &SearchState,
    selected_link_rect: Option<crate::layout::Rectangle>,
) {
    for (i, line) in lines.iter().enumerate() {
        let y = display_y + i as u16;
        if y >= area.height {
            break;
        }

        let line_y_in_doc = node_y + i as u16;
        let mut spans: Vec<Span> = Vec::new();

        // IMPORTANT: Start with an explicit style reset to prevent bleed from previous widgets
        // This is critical when paragraphs are rendered after styled elements on the same line
        if x > 0 {
            // Only add reset if we're not at the start of the line
            // This prevents style bleed from list markers and other inline elements
            spans.push(Span::styled("", Style::reset()));
        }

        // Get search matches for this line
        let line_matches: Vec<&crate::search::SearchMatch> = search_state
            .matches
            .iter()
            .filter(|m| m.y == line_y_in_doc)
            .collect();

        // If no matches on this line, render normally
        if line_matches.is_empty() {
            let mut current_x = x;
            for seg in &line.segments {
                let is_selected_link = if let Some(sel_rect) = selected_link_rect {
                    // Check if this segment overlaps the selected link rectangle
                    current_x >= sel_rect.x
                        && current_x < sel_rect.x + sel_rect.width
                        && line_y_in_doc >= sel_rect.y
                        && line_y_in_doc < sel_rect.y + sel_rect.height
                        && seg.link_url.is_some()
                } else {
                    false
                };

                spans.push(text_segment_to_span(seg, theme, is_selected_link));
                current_x += seg.text.chars().count() as u16;
            }
        } else {
            // Render with highlighting
            let mut current_x = x;
            for seg in &line.segments {
                let seg_end = current_x + seg.text.len() as u16;
                let mut last_pos = 0;

                for match_ref in &line_matches {
                    let match_start = match_ref.x;
                    let match_end = match_ref.x + match_ref.length as u16;

                    // Check if this match overlaps with this segment
                    if match_start < seg_end && match_end > current_x {
                        let overlap_start = match_start.saturating_sub(current_x) as usize;
                        let overlap_end =
                            (match_end.saturating_sub(current_x) as usize).min(seg.text.len());

                        // Add text before the match
                        if overlap_start > last_pos {
                            let before_text = &seg.text[last_pos..overlap_start];
                            spans.push(Span::styled(
                                before_text.to_string(),
                                segment_to_style(seg, theme),
                            ));
                        }

                        // Add highlighted match
                        if overlap_end > overlap_start {
                            let match_text = &seg.text[overlap_start..overlap_end];
                            let is_current = search_state
                                .current_index
                                .map(|idx| search_state.matches[idx] == **match_ref)
                                .unwrap_or(false);

                            let highlight_style = if is_current {
                                Style::default()
                                    .bg(RatatuiColor::Yellow)
                                    .fg(RatatuiColor::Black)
                                    .add_modifier(Modifier::BOLD)
                            } else {
                                Style::default()
                                    .bg(RatatuiColor::DarkGray)
                                    .fg(RatatuiColor::White)
                            };

                            spans.push(Span::styled(match_text.to_string(), highlight_style));
                            last_pos = overlap_end;
                        }
                    }
                }

                // Add remaining text after all matches
                if last_pos < seg.text.len() {
                    let after_text = &seg.text[last_pos..];
                    spans.push(Span::styled(
                        after_text.to_string(),
                        segment_to_style(seg, theme),
                    ));
                }

                current_x = seg_end;
            }
        }

        let line_text = RatatuiText::from(ratatui::text::Line::from(spans));
        let para = Paragraph::new(line_text);

        let line_area = ratatui::layout::Rect {
            x,
            y,
            width: area.width.saturating_sub(x),
            height: 1,
        };

        frame.render_widget(para, line_area);
    }
}

/// Convert a text segment to a style (without the text)
fn segment_to_style(seg: &TextSegment, theme: &Theme) -> Style {
    let base = Style::default().fg(seg
        .style
        .foreground
        .map(to_ratatui_color)
        .unwrap_or(to_ratatui_color(theme.colors.foreground)));

    let base = if let Some(bg) = seg.style.background {
        base.bg(to_ratatui_color(bg))
    } else {
        base
    };

    let base = match seg.style.weight {
        FontWeight::Bold => base.add_modifier(Modifier::BOLD),
        FontWeight::Normal => base,
    };

    match seg.style.style {
        FontStyle::Italic => base.add_modifier(Modifier::ITALIC),
        FontStyle::Normal => base,
    }
}

fn render_code_block(
    frame: &mut ratatui::Frame,
    lang: &Option<String>,
    lines: &[String],
    theme: &Theme,
    x: u16,
    display_y: u16,
    width: u16,
    area: ratatui::layout::Rect,
    node_y: u16,
    scroll_y: u16,
) {
    let code_style = &theme.blocks.code_block;
    let style = Style::default()
        .fg(to_ratatui_color(code_style.foreground))
        .bg(to_ratatui_color(code_style.background));

    let border_fg = to_ratatui_color(code_style.foreground);
    let border_style = Style::default()
        .fg(border_fg)
        .bg(to_ratatui_color(code_style.background));

    // Constrain width to available area (important when sidebar is present)
    let actual_width = width.min(area.width.saturating_sub(x));

    // Calculate block boundaries in document coordinates
    let block_start = node_y;
    let block_end = node_y + lines.len() as u16 + 2; // +2 for top and bottom borders

    // Draw top border if visible
    if block_start >= scroll_y && block_start < scroll_y + area.height {
        let top_y = block_start - scroll_y;
        let top_border = format!("┌{}┐", "─".repeat(actual_width.saturating_sub(2) as usize));
        frame.render_widget(
            Paragraph::new(RatatuiText::from(Span::styled(top_border, border_style))),
            ratatui::layout::Rect {
                x,
                y: top_y,
                width: actual_width,
                height: 1,
            },
        );
    }

    // Draw side borders for visible content lines
    let content_start = block_start + 1;
    let content_end = block_end - 1;
    for doc_y in content_start..content_end {
        if doc_y >= scroll_y && doc_y < scroll_y + area.height {
            let display_line_y = doc_y - scroll_y;
            // Left border
            frame.render_widget(
                Paragraph::new(RatatuiText::from(Span::styled("│", border_style))),
                ratatui::layout::Rect {
                    x,
                    y: display_line_y,
                    width: 1,
                    height: 1,
                },
            );
            // Right border
            frame.render_widget(
                Paragraph::new(RatatuiText::from(Span::styled("│", border_style))),
                ratatui::layout::Rect {
                    x: x + actual_width - 1,
                    y: display_line_y,
                    width: 1,
                    height: 1,
                },
            );
        }
    }

    // Draw bottom border if visible
    let bottom_y = block_end - 1;
    if bottom_y >= scroll_y && bottom_y < scroll_y + area.height {
        let display_bottom_y = bottom_y - scroll_y;
        let bottom_border = format!("└{}┘", "─".repeat(actual_width.saturating_sub(2) as usize));
        frame.render_widget(
            Paragraph::new(RatatuiText::from(Span::styled(bottom_border, border_style))),
            ratatui::layout::Rect {
                x,
                y: display_bottom_y,
                width: actual_width,
                height: 1,
            },
        );
    }

    // Render language badge if present (only if top border is visible)
    if let Some(lang_name) = lang {
        if code_style.show_language_badge && node_y >= scroll_y && display_y < area.height {
            let badge = format!(" {} ", lang_name);
            let badge_span = Span::styled(
                badge,
                Style::default().fg(to_ratatui_color(theme.colors.accent)),
            );
            let badge_text = RatatuiText::from(badge_span);

            let badge_area = ratatui::layout::Rect {
                x: x + actual_width.saturating_sub(lang_name.len() as u16 + 3),
                y: display_y,
                width: lang_name.len() as u16 + 2,
                height: 1,
            };

            frame.render_widget(Paragraph::new(badge_text), badge_area);
        }
    }

    // Calculate which lines are visible
    // Code block content starts at node_y + 1 (after top border)
    let content_start_y = node_y + 1;

    // Determine starting line index based on scroll position
    let start_line = if content_start_y < scroll_y {
        (scroll_y - content_start_y) as usize
    } else {
        0
    };

    // Render code lines
    for (i, line) in lines.iter().enumerate().skip(start_line) {
        let line_y_in_doc = content_start_y + i as u16;

        // Skip if line is above viewport
        if line_y_in_doc < scroll_y {
            continue;
        }

        // Stop if line is below viewport
        if line_y_in_doc >= scroll_y + area.height {
            break;
        }

        let display_line_y = line_y_in_doc - scroll_y;

        // Pad line with spaces to fill the full width so background extends across
        let content_width = actual_width.saturating_sub(2) as usize;
        let padded_line = format!("{:width$}", line, width = content_width);

        let span = Span::styled(padded_line, style);
        let line_text = RatatuiText::from(span);
        let para = Paragraph::new(line_text);

        let line_area = ratatui::layout::Rect {
            x: x + 1,
            y: display_line_y,
            width: actual_width.saturating_sub(2),
            height: 1,
        };

        frame.render_widget(para, line_area);
    }
}

fn render_status_bar(
    frame: &mut ratatui::Frame,
    tree: &LayoutTree,
    area: ratatui::layout::Rect,
    search_state: &SearchState,
    file_jump_mode: bool,
    file_jump_buffer: &str,
) {
    // If file jump mode is active, show file jump prompt
    if file_jump_mode {
        let prompt_text = if file_jump_buffer.is_empty() {
            ":".to_string()
        } else {
            format!(":{}", file_jump_buffer)
        };

        let prompt_span = Span::styled(
            format!("{} (Enter file number)", prompt_text),
            Style::default()
                .bg(RatatuiColor::Blue)
                .fg(RatatuiColor::White),
        );

        let prompt_bar_area = ratatui::layout::Rect {
            x: 0,
            y: area.height.saturating_sub(1),
            width: area.width,
            height: 1,
        };

        frame.render_widget(Paragraph::new(prompt_span), prompt_bar_area);
        return;
    }

    // If search is active, show search bar instead
    if search_state.active {
        let search_text = if search_state.needle.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", search_state.needle)
        };

        let match_info = if search_state.matches.is_empty() && !search_state.needle.is_empty() {
            " (no matches)".to_string()
        } else if let Some(idx) = search_state.current_index {
            format!(" [{}/{}]", idx + 1, search_state.matches.len())
        } else {
            String::new()
        };

        let full_text = format!("{}{}", search_text, match_info);

        let search_span = Span::styled(
            full_text,
            Style::default()
                .bg(RatatuiColor::Yellow)
                .fg(RatatuiColor::Black),
        );

        let search_bar_area = ratatui::layout::Rect {
            x: 0,
            y: area.height.saturating_sub(1),
            width: area.width,
            height: 1,
        };

        frame.render_widget(Paragraph::new(search_span), search_bar_area);
        return;
    }

    let doc_height = tree.document_height();
    let viewport_height = tree.viewport.height;
    let scroll_y = tree.viewport.scroll_y;

    // Calculate visible line range
    let top_line = scroll_y + 1; // +1 for 1-based display
    let bottom_line = (scroll_y + viewport_height).min(doc_height);

    // Calculate percentage through document
    let max_scroll = doc_height.saturating_sub(viewport_height);
    let percentage = if max_scroll > 0 {
        (scroll_y * 100) / max_scroll
    } else {
        100 // If document fits in viewport, we're at 100%
    };

    let status = if doc_height <= viewport_height {
        " All ".to_string()
    } else if scroll_y == 0 {
        " Top ".to_string()
    } else if scroll_y >= max_scroll {
        " Bot ".to_string()
    } else {
        format!(" {}% ", percentage)
    };

    let position = format!("Lines {}-{}/{} ", top_line, bottom_line, doc_height);

    // Show search match count if we have matches
    let search_info = if !search_state.matches.is_empty() {
        if let Some(idx) = search_state.current_index {
            format!(" Search: {}/{} ", idx + 1, search_state.matches.len())
        } else {
            format!(" Search: {} matches ", search_state.matches.len())
        }
    } else {
        String::new()
    };

    let help_text = "Press 'h' for help";

    // Pad status bar to fill entire width
    let total_text_len = status.len() + position.len() + search_info.len() + help_text.len();
    let padding_len = area.width.saturating_sub(total_text_len as u16) as usize;
    let padding = " ".repeat(padding_len);

    let full_status = format!(
        "{}{}{}{}{}",
        status, position, search_info, padding, help_text
    );

    let status_span = Span::styled(
        full_status,
        Style::default()
            .bg(RatatuiColor::DarkGray)
            .fg(RatatuiColor::White),
    );
    let status_text = RatatuiText::from(status_span);

    let status_area = ratatui::layout::Rect {
        x: 0,
        y: area.height.saturating_sub(1),
        width: area.width,
        height: 1,
    };

    frame.render_widget(Paragraph::new(status_text), status_area);
}

fn render_help_menu(frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
    let help_text = vec![
        "LUMEN - Keyboard Shortcuts",
        "",
        "Navigation:",
        "  j / ↓        Scroll down one line",
        "  k / ↑        Scroll up one line",
        "  d            Scroll down half page",
        "  u            Scroll up half page",
        "  Space        Scroll down one page",
        "  PageDown     Scroll down one page",
        "  PageUp       Scroll up one page",
        "  g / Home     Go to top of document",
        "  G / End      Go to bottom of document",
        "",
        "Header Navigation:",
        "  n            Jump to next heading",
        "  p            Jump to previous heading",
        "",
        "Link Navigation:",
        "  a            Cycle through links",
        "  Enter        Follow selected link",
        "",
        "File Navigation:",
        "  Tab          Switch to next file",
        "  Shift+Tab    Switch to previous file",
        "  1-9          Jump to file 1-9",
        "  :N           Jump to file N (any number)",
        "",
        "Search:",
        "  /            Start search",
        "  n            Next match (when searching)",
        "  N            Previous match",
        "  Esc          Clear search results",
        "",
        "Other:",
        "  t            Cycle through themes",
        "  f            Toggle file sidebar",
        "  r            Reload current file",
        "  m            Toggle mouse mode",
        "  h            Toggle this help menu",
        "  q / Esc      Quit",
        "",
        "Press 'h' or Esc to close this menu",
    ];

    // Calculate centered position
    let width = 60u16.min(area.width);
    let height = (help_text.len() as u16 + 2).min(area.height);
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;

    // Create help menu area
    let help_area = ratatui::layout::Rect {
        x,
        y,
        width,
        height,
    };

    // Create bordered block
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(RatatuiColor::Cyan))
        .style(Style::default().bg(RatatuiColor::Black));

    // Create text
    let text: Vec<ratatui::text::Line> = help_text
        .iter()
        .map(|line| {
            if line.starts_with("LUMEN") {
                ratatui::text::Line::from(Span::styled(
                    *line,
                    Style::default()
                        .fg(RatatuiColor::Cyan)
                        .add_modifier(Modifier::BOLD),
                ))
            } else if line.ends_with(':') {
                ratatui::text::Line::from(Span::styled(
                    *line,
                    Style::default()
                        .fg(RatatuiColor::Yellow)
                        .add_modifier(Modifier::BOLD),
                ))
            } else {
                ratatui::text::Line::from(*line)
            }
        })
        .collect();

    let paragraph = Paragraph::new(text).block(block).style(
        Style::default()
            .fg(RatatuiColor::White)
            .bg(RatatuiColor::Black),
    );

    // Clear the area first to prevent transparency
    frame.render_widget(Clear, help_area);
    frame.render_widget(paragraph, help_area);
}

fn text_segment_to_span<'a>(
    segment: &'a TextSegment,
    _theme: &Theme,
    is_selected_link: bool,
) -> Span<'a> {
    let mut style = Style::default();

    if let Some(fg) = segment.style.foreground {
        style = style.fg(to_ratatui_color(fg));
    }

    if let Some(bg) = segment.style.background {
        style = style.bg(to_ratatui_color(bg));
    }

    if segment.style.weight == FontWeight::Bold {
        style = style.add_modifier(Modifier::BOLD)
    }

    if segment.style.style == FontStyle::Italic {
        style = style.add_modifier(Modifier::ITALIC)
    }

    // Add inverse highlighting for selected link
    if is_selected_link {
        style = style.add_modifier(Modifier::REVERSED);
    }

    // NOTE: OSC 8 clickable links and iTerm2 inline images are DISABLED by default
    // because escape sequences interfere with Ratatui's rendering and cause visual
    // artifacts (dashed lines, terminal corruption).
    //
    // These features are fundamentally incompatible with how Ratatui manages the
    // terminal screen buffer. Enable them at your own risk via environment variables:
    //
    //   LUMEN_ENABLE_LINKS=1      - Enable OSC 8 clickable links
    //   LUMEN_ENABLE_IMAGES=1     - Enable iTerm2 inline images
    //
    // Known issues when enabled:
    // - Horizontal dashed lines appear when scrolling
    // - Terminal may become corrupted requiring `reset`
    // - Width calculations break, causing layout issues

    let enable_images = std::env::var("LUMEN_ENABLE_IMAGES").is_ok();
    let enable_links = std::env::var("LUMEN_ENABLE_LINKS").is_ok();

    // iTerm2 inline images (experimental, disabled by default)
    if enable_images {
        if let (Some(image_url), Some(image_alt)) = (&segment.image_url, &segment.image_alt) {
            if let Some(iterm2_seq) = try_render_iterm2_image(image_url, image_alt) {
                return Span::styled(iterm2_seq, style);
            }
        }
    }

    // OSC 8 clickable links (experimental, disabled by default)
    if enable_links {
        if let Some(url) = &segment.link_url {
            let wrapped_text = format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", url, segment.text);
            return Span::styled(wrapped_text, style);
        }
    }

    // Default: render text without escape sequences
    Span::styled(segment.text.as_str(), style)
}

fn render_file_sidebar(
    frame: &mut ratatui::Frame,
    file_manager: &crate::FileManager,
    area: ratatui::layout::Rect,
    _theme: &Theme,
) {
    // Draw sidebar border on the right side
    let border_style = Style::default().fg(RatatuiColor::DarkGray);
    for y in 0..area.height {
        let border_span = Span::styled("│", border_style);
        let border_area = ratatui::layout::Rect {
            x: area.x + area.width - 1,
            y: area.y + y,
            width: 1,
            height: 1,
        };
        frame.render_widget(Paragraph::new(RatatuiText::from(border_span)), border_area);
    }

    // Render title
    let title_span = Span::styled(
        " Open Files ",
        Style::default()
            .fg(RatatuiColor::Cyan)
            .add_modifier(Modifier::BOLD),
    );
    let title_area = ratatui::layout::Rect {
        x: area.x,
        y: area.y,
        width: area.width - 1,
        height: 1,
    };
    frame.render_widget(Paragraph::new(RatatuiText::from(title_span)), title_area);

    // Render file list starting at y=2
    let mut current_y = 2;
    for (i, file) in file_manager.files.iter().enumerate() {
        if current_y >= area.height {
            break;
        }

        let is_current = i == file_manager.current_index;

        // Create file entry: number, indicator, filename
        let number = format!("{}.", i + 1);
        let indicator = if is_current { "▶ " } else { "  " };

        // Truncate filename if too long
        let max_name_len = (area.width as usize).saturating_sub(6);
        let display_name = if file.name.len() > max_name_len {
            format!("{}...", &file.name[..max_name_len.saturating_sub(3)])
        } else {
            file.name.clone()
        };

        let file_text = format!(" {} {}{}", number, indicator, display_name);

        let file_style = if is_current {
            Style::default()
                .fg(RatatuiColor::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(RatatuiColor::White)
        };

        let file_span = Span::styled(file_text, file_style);
        let file_area = ratatui::layout::Rect {
            x: area.x,
            y: area.y + current_y,
            width: area.width - 1,
            height: 1,
        };
        frame.render_widget(Paragraph::new(RatatuiText::from(file_span)), file_area);

        current_y += 1;
    }
}

fn render_image_sidebar(
    frame: &mut ratatui::Frame,
    images: &[ImageReference],
    scroll_y: u16,
    area: ratatui::layout::Rect,
    _theme: &Theme,
) {
    // Draw sidebar border
    let border_style = Style::default().fg(RatatuiColor::DarkGray);
    for y in 0..area.height {
        let border_span = Span::styled("│", border_style);
        let border_area = ratatui::layout::Rect {
            x: area.x,
            y: area.y + y,
            width: 1,
            height: 1,
        };
        frame.render_widget(Paragraph::new(RatatuiText::from(border_span)), border_area);
    }

    // Track used space to prevent overlaps
    let mut next_available_y = 0u16;

    // Render each image at its position
    for image in images {
        // Calculate display Y position (adjusted for scrolling)
        let display_y = image.y_position.saturating_sub(scroll_y);

        // Skip if image is off-screen
        if display_y >= area.height {
            continue;
        }

        // Ensure images don't overlap - use the later of the two positions
        let actual_y = display_y.max(next_available_y);

        // Skip if adjusted position is off-screen
        if actual_y >= area.height {
            continue;
        }

        // Try to load and render the image
        if let Ok(img) = load_image(&image.path) {
            // Available width in characters (sidebar width minus border/padding)
            let max_width_chars = area.width.saturating_sub(3);

            // Use a compact fixed height - ratatui-image will scale appropriately
            // This ensures captions appear close to images
            let image_height = 12u16.min(area.height.saturating_sub(actual_y));

            let image_area = ratatui::layout::Rect {
                x: area.x + 2, // Offset from border
                y: area.y + actual_y,
                width: max_width_chars,
                height: image_height,
            };

            // Render the image using ratatui-image
            if let Ok(mut protocol) = create_image_protocol(&img) {
                let image_widget = StatefulImage::default();
                frame.render_stateful_widget(image_widget, image_area, &mut protocol);
            } else {
                // Fallback: show alt text if image can't be rendered
                render_image_fallback(frame, &image.alt_text, image_area);
            }

            // Render caption immediately below image (no gap)
            let caption_y = actual_y + image_height;
            if caption_y < area.height {
                let caption_text = format!("[IMAGE: {}]", image.alt_text);
                let caption_span = Span::styled(
                    caption_text,
                    Style::default()
                        .fg(RatatuiColor::DarkGray)
                        .add_modifier(Modifier::ITALIC),
                );
                let caption_area = ratatui::layout::Rect {
                    x: area.x + 2,
                    y: area.y + caption_y,
                    width: max_width_chars,
                    height: 1,
                };
                frame.render_widget(
                    Paragraph::new(RatatuiText::from(caption_span)),
                    caption_area,
                );
            }

            // Update next available position (image + caption + 1 line gap)
            next_available_y = actual_y + image_height + 2;
        } else {
            // Image failed to load, show alt text
            let fallback_area = ratatui::layout::Rect {
                x: area.x + 2,
                y: area.y + actual_y,
                width: area.width.saturating_sub(3),
                height: 3,
            };
            render_image_fallback(frame, &image.alt_text, fallback_area);
            next_available_y = actual_y + 4; // 3 for text + 1 gap
        }
    }
}

fn load_image(path: &str) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
    // Security: only load local files
    if path.starts_with("http://") || path.starts_with("https://") {
        return Err("Remote images not supported".into());
    }

    let img = image::ImageReader::open(path)?.decode()?;
    Ok(img)
}

fn create_image_protocol(
    img: &image::DynamicImage,
) -> Result<StatefulProtocol, Box<dyn std::error::Error>> {
    // Query the terminal to detect capabilities (iTerm2, Kitty, Sixel, etc.)
    // This will automatically detect and use the best available protocol
    let picker = match Picker::from_query_stdio() {
        Ok(picker) => picker,
        Err(_) => {
            // Fallback to manual font size if terminal query fails
            Picker::from_fontsize((8, 12))
        }
    };

    // Resize image to fit the area
    let protocol = picker.new_resize_protocol(img.clone());

    Ok(protocol)
}

fn render_image_fallback(frame: &mut ratatui::Frame, alt_text: &str, area: ratatui::layout::Rect) {
    let fallback_text = format!("[{}]", alt_text);
    let span = Span::styled(fallback_text, Style::default().fg(RatatuiColor::DarkGray));
    frame.render_widget(Paragraph::new(RatatuiText::from(span)), area);
}

fn render_inline_image(
    frame: &mut ratatui::Frame,
    path: &str,
    alt_text: &str,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    area: ratatui::layout::Rect,
) {
    // Skip if image is off-screen
    if y >= area.height {
        return;
    }

    // Calculate actual available height
    let available_height = height.min(area.height.saturating_sub(y));

    // Try to load and render the image
    if let Ok(img) = load_image(path) {
        let image_area = ratatui::layout::Rect {
            x,
            y,
            width: width.saturating_sub(2), // Leave some margin
            height: available_height.saturating_sub(2), // Leave space for caption
        };

        // Render the image using ratatui-image
        if let Ok(mut protocol) = create_image_protocol(&img) {
            let image_widget = StatefulImage::default();
            frame.render_stateful_widget(image_widget, image_area, &mut protocol);
        } else {
            // Fallback: show alt text if image can't be rendered
            render_image_fallback(frame, alt_text, image_area);
        }

        // Render caption below image
        let caption_y = y + available_height.saturating_sub(1);
        if caption_y < area.height {
            let caption_text = format!("[IMAGE: {}]", alt_text);
            let caption_span = Span::styled(
                caption_text,
                Style::default()
                    .fg(RatatuiColor::DarkGray)
                    .add_modifier(Modifier::ITALIC),
            );
            let caption_area = ratatui::layout::Rect {
                x,
                y: caption_y,
                width,
                height: 1,
            };
            frame.render_widget(
                Paragraph::new(RatatuiText::from(caption_span)),
                caption_area,
            );
        }
    } else {
        // Image failed to load, show alt text
        let fallback_area = ratatui::layout::Rect {
            x,
            y,
            width,
            height: available_height.min(3),
        };
        render_image_fallback(frame, alt_text, fallback_area);
    }
}

/// Border characters for drawing table borders
struct BorderChars {
    horizontal: &'static str,
    vertical: &'static str,
    top_left: &'static str,
    top_right: &'static str,
    bottom_left: &'static str,
    bottom_right: &'static str,
    cross: &'static str,
    t_down: &'static str,
    t_up: &'static str,
    t_right: &'static str,
    t_left: &'static str,
}

fn get_border_chars(style: BorderStyle) -> BorderChars {
    match style {
        BorderStyle::Single => BorderChars {
            horizontal: "─",
            vertical: "│",
            top_left: "┌",
            top_right: "┐",
            bottom_left: "└",
            bottom_right: "┘",
            cross: "┼",
            t_down: "┬",
            t_up: "┴",
            t_right: "├",
            t_left: "┤",
        },
        BorderStyle::Double => BorderChars {
            horizontal: "═",
            vertical: "║",
            top_left: "╔",
            top_right: "╗",
            bottom_left: "╚",
            bottom_right: "╝",
            cross: "╬",
            t_down: "╦",
            t_up: "╩",
            t_right: "╠",
            t_left: "╣",
        },
        BorderStyle::Rounded => BorderChars {
            horizontal: "─",
            vertical: "│",
            top_left: "╭",
            top_right: "╮",
            bottom_left: "╰",
            bottom_right: "╯",
            cross: "┼",
            t_down: "┬",
            t_up: "┴",
            t_right: "├",
            t_left: "┤",
        },
        BorderStyle::Heavy => BorderChars {
            horizontal: "━",
            vertical: "┃",
            top_left: "┏",
            top_right: "┓",
            bottom_left: "┗",
            bottom_right: "┛",
            cross: "╋",
            t_down: "┳",
            t_up: "┻",
            t_right: "┣",
            t_left: "┫",
        },
        BorderStyle::Ascii => BorderChars {
            horizontal: "-",
            vertical: "|",
            top_left: "+",
            top_right: "+",
            bottom_left: "+",
            bottom_right: "+",
            cross: "+",
            t_down: "+",
            t_up: "+",
            t_right: "+",
            t_left: "+",
        },
        BorderStyle::None => BorderChars {
            horizontal: " ",
            vertical: " ",
            top_left: " ",
            top_right: " ",
            bottom_left: " ",
            bottom_right: " ",
            cross: " ",
            t_down: " ",
            t_up: " ",
            t_right: " ",
            t_left: " ",
        },
    }
}

fn to_ratatui_color(color: Color) -> RatatuiColor {
    match color {
        Color::Reset => RatatuiColor::Reset,
        Color::Rgb(r, g, b) => RatatuiColor::Rgb(r, g, b),
        Color::Ansi256(idx) => RatatuiColor::Indexed(idx),
        Color::Ansi(ansi) => {
            use crate::theme::AnsiColor;
            match ansi {
                AnsiColor::Black => RatatuiColor::Black,
                AnsiColor::Red => RatatuiColor::Red,
                AnsiColor::Green => RatatuiColor::Green,
                AnsiColor::Yellow => RatatuiColor::Yellow,
                AnsiColor::Blue => RatatuiColor::Blue,
                AnsiColor::Magenta => RatatuiColor::Magenta,
                AnsiColor::Cyan => RatatuiColor::Cyan,
                AnsiColor::White => RatatuiColor::White,
                AnsiColor::BrightBlack => RatatuiColor::DarkGray,
                AnsiColor::BrightRed => RatatuiColor::LightRed,
                AnsiColor::BrightGreen => RatatuiColor::LightGreen,
                AnsiColor::BrightYellow => RatatuiColor::LightYellow,
                AnsiColor::BrightBlue => RatatuiColor::LightBlue,
                AnsiColor::BrightMagenta => RatatuiColor::LightMagenta,
                AnsiColor::BrightCyan => RatatuiColor::LightCyan,
                AnsiColor::BrightWhite => RatatuiColor::Gray,
            }
        }
    }
}

/// Try to render an image as an iTerm2 inline image
/// Returns the iTerm2 escape sequence if successful, None otherwise
fn try_render_iterm2_image(image_url: &str, _alt: &str) -> Option<String> {
    // Only support local file paths for security
    let path = Path::new(image_url);

    // Check if it's a local file (not a URL)
    if image_url.starts_with("http://") || image_url.starts_with("https://") {
        return None;
    }

    // Try to read the file
    let image_data = fs::read(path).ok()?;

    // Base64 encode the image
    use base64::engine::general_purpose::STANDARD;
    use base64::Engine;
    let encoded = STANDARD.encode(&image_data);

    // iTerm2 inline image protocol:
    // ESC]1337;File=inline=1;width=40;preserveAspectRatio=1:[base64]^G
    // width is in character cells
    Some(format!(
        "\x1b]1337;File=inline=1;width=40;preserveAspectRatio=1:{}\x07",
        encoded
    ))
}
