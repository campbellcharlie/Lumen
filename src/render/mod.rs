//! Terminal rendering

use crate::layout::{LayoutElement, LayoutNode, LayoutTree, TextSegment, Line};
use crate::theme::{Color, FontStyle, FontWeight, Theme};
use ratatui::{
    backend::CrosstermBackend,
    style::{Color as RatatuiColor, Modifier, Style},
    text::{Span, Text as RatatuiText},
    widgets::{Block, Borders, Paragraph},
    Terminal as RatatuiTerminal,
};
use std::io;

pub type Terminal = RatatuiTerminal<CrosstermBackend<io::Stdout>>;

/// Initialize terminal for rendering
pub fn init_terminal() -> io::Result<Terminal> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        // Don't capture mouse - allow terminal text selection
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
        crossterm::cursor::Show
    )?;
    Ok(())
}

/// Render layout tree to terminal
pub fn render(terminal: &mut Terminal, tree: &LayoutTree, theme: &Theme, show_help: bool) -> io::Result<()> {
    terminal.draw(|frame| {
        let area = frame.area();

        // Render document starting from scroll position
        let scroll_y = tree.viewport.scroll_y;

        for node in &tree.root.children {
            render_node(frame, node, theme, scroll_y, area);
        }

        // Render status bar
        render_status_bar(frame, tree, area);

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
) {
    // Adjust for scroll
    if node.rect.y < scroll_y {
        return; // Above viewport
    }

    let display_y = node.rect.y.saturating_sub(scroll_y);

    if display_y >= area.height {
        return; // Below viewport
    }

    match &node.element {
        LayoutElement::Heading { level, text } => {
            render_heading(frame, node, *level, text, theme, display_y, area);
        }
        LayoutElement::Paragraph { lines } => {
            render_paragraph(frame, lines, theme, node.rect.x, display_y, area);
        }
        LayoutElement::CodeBlock { lang, lines } => {
            render_code_block(frame, lang, lines, theme, node.rect.x, display_y, node.rect.width, area);
        }
        LayoutElement::List { .. } => {
            for child in &node.children {
                render_node(frame, child, theme, scroll_y, area);
            }
        }
        LayoutElement::ListItem { marker, .. } => {
            // Render marker - only style the marker character, not trailing space
            let marker_char = marker.trim_end();
            let marker_style = Style::default().fg(to_ratatui_color(theme.blocks.list.marker_color));

            // Create spans: styled marker + unstyled space
            let spans = vec![
                Span::styled(marker_char.to_string(), marker_style),
                Span::raw(" "),
            ];
            let marker_text = RatatuiText::from(ratatui::text::Line::from(spans));

            let marker_area = ratatui::layout::Rect {
                x: node.rect.x,
                y: display_y,
                width: marker.chars().count() as u16,
                height: 1,
            };

            frame.render_widget(Paragraph::new(marker_text), marker_area);

            // Render children
            for child in &node.children {
                render_node(frame, child, theme, scroll_y, area);
            }
        }
        LayoutElement::BlockQuote => {
            // Render border on left
            let block = Block::default()
                .borders(Borders::LEFT)
                .border_style(Style::default().fg(to_ratatui_color(theme.blocks.blockquote.color)));

            let block_area = ratatui::layout::Rect {
                x: node.rect.x,
                y: display_y,
                width: node.rect.width,
                height: node.rect.height.min(area.height.saturating_sub(display_y)),
            };

            frame.render_widget(block, block_area);

            // Render children
            for child in &node.children {
                render_node(frame, child, theme, scroll_y, area);
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

            // Render border with background
            let block = Block::default()
                .borders(Borders::LEFT)
                .border_style(Style::default().fg(to_ratatui_color(callout_style.border_color)));

            let block_area = ratatui::layout::Rect {
                x: node.rect.x,
                y: display_y,
                width: node.rect.width,
                height: node.rect.height.min(area.height.saturating_sub(display_y)),
            };

            // Render background if specified
            if let Some(bg) = callout_style.background {
                let bg_block = Block::default()
                    .style(Style::default().bg(to_ratatui_color(bg)));
                frame.render_widget(bg_block, block_area);
            }

            frame.render_widget(block, block_area);

            // Render icon at the top left
            let icon_span = Span::styled(
                &callout_style.icon,
                Style::default().fg(to_ratatui_color(callout_style.color))
            );
            let icon_area = ratatui::layout::Rect {
                x: node.rect.x,
                y: display_y,
                width: 2,
                height: 1,
            };
            frame.render_widget(Paragraph::new(RatatuiText::from(icon_span)), icon_area);

            // Render children
            for child in &node.children {
                render_node(frame, child, theme, scroll_y, area);
            }
        }
        LayoutElement::Table { .. } => {
            for child in &node.children {
                render_node(frame, child, theme, scroll_y, area);
            }
        }
        LayoutElement::TableRow { is_header: _ } => {
            for child in &node.children {
                render_node(frame, child, theme, scroll_y, area);
            }
        }
        LayoutElement::TableCell => {
            // Simple cell rendering - could be improved
            for child in &node.children {
                render_node(frame, child, theme, scroll_y, area);
            }
        }
        LayoutElement::HorizontalRule => {
            let hr = "─".repeat(node.rect.width as usize);
            let hr_text = RatatuiText::from(hr);

            let hr_area = ratatui::layout::Rect {
                x: node.rect.x,
                y: display_y,
                width: node.rect.width,
                height: 1,
            };

            frame.render_widget(Paragraph::new(hr_text), hr_area);
        }
        _ => {
            // Render children for other types
            for child in &node.children {
                render_node(frame, child, theme, scroll_y, area);
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
        x: node.rect.x,
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
) {
    for (i, line) in lines.iter().enumerate() {
        let y = display_y + i as u16;
        if y >= area.height {
            break;
        }

        let spans: Vec<Span> = line.segments.iter().map(|seg| {
            text_segment_to_span(seg, theme)
        }).collect();

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

fn render_code_block(
    frame: &mut ratatui::Frame,
    lang: &Option<String>,
    lines: &[String],
    theme: &Theme,
    x: u16,
    display_y: u16,
    width: u16,
    area: ratatui::layout::Rect,
) {
    let code_style = &theme.blocks.code_block;
    let style = Style::default()
        .fg(to_ratatui_color(code_style.foreground))
        .bg(to_ratatui_color(code_style.background));

    // Render border
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(style);

    let block_area = ratatui::layout::Rect {
        x,
        y: display_y,
        width,
        height: (lines.len() as u16 + 2).min(area.height.saturating_sub(display_y)),
    };

    frame.render_widget(block, block_area);

    // Render language badge if present
    if let Some(lang_name) = lang {
        if code_style.show_language_badge {
            let badge = format!(" {} ", lang_name);
            let badge_span = Span::styled(
                badge,
                Style::default().fg(to_ratatui_color(theme.colors.accent))
            );
            let badge_text = RatatuiText::from(badge_span);

            let badge_area = ratatui::layout::Rect {
                x: x + width.saturating_sub(lang_name.len() as u16 + 3),
                y: display_y,
                width: lang_name.len() as u16 + 2,
                height: 1,
            };

            frame.render_widget(Paragraph::new(badge_text), badge_area);
        }
    }

    // Render code lines
    for (i, line) in lines.iter().enumerate() {
        let y = display_y + i as u16 + 1;
        if y >= area.height {
            break;
        }

        let span = Span::styled(line.clone(), style);
        let line_text = RatatuiText::from(span);
        let para = Paragraph::new(line_text);

        let line_area = ratatui::layout::Rect {
            x: x + 1,
            y,
            width: width.saturating_sub(2),
            height: 1,
        };

        frame.render_widget(para, line_area);
    }
}

fn render_status_bar(
    frame: &mut ratatui::Frame,
    tree: &LayoutTree,
    area: ratatui::layout::Rect,
) {
    let doc_height = tree.document_height();
    let viewport_height = tree.viewport.height;
    let scroll_y = tree.viewport.scroll_y;

    // Calculate visible line range
    let top_line = scroll_y + 1;  // +1 for 1-based display
    let bottom_line = (scroll_y + viewport_height).min(doc_height);

    // Calculate percentage through document
    let max_scroll = doc_height.saturating_sub(viewport_height);
    let percentage = if max_scroll > 0 {
        (scroll_y * 100) / max_scroll
    } else {
        100  // If document fits in viewport, we're at 100%
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

    let help_text = "Press 'h' for help";

    // Pad status bar to fill entire width
    let total_text_len = status.len() + position.len() + help_text.len();
    let padding_len = area.width.saturating_sub(total_text_len as u16) as usize;
    let padding = " ".repeat(padding_len);

    let full_status = format!("{}{}{}{}", status, position, padding, help_text);

    let status_span = Span::styled(
        full_status,
        Style::default()
            .bg(RatatuiColor::DarkGray)
            .fg(RatatuiColor::White)
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
        "Other:",
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
                    Style::default().fg(RatatuiColor::Cyan).add_modifier(Modifier::BOLD),
                ))
            } else if line.ends_with(':') {
                ratatui::text::Line::from(Span::styled(
                    *line,
                    Style::default().fg(RatatuiColor::Yellow).add_modifier(Modifier::BOLD),
                ))
            } else {
                ratatui::text::Line::from(*line)
            }
        })
        .collect();

    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::default().fg(RatatuiColor::White));

    frame.render_widget(paragraph, help_area);
}

fn text_segment_to_span<'a>(segment: &'a TextSegment, _theme: &Theme) -> Span<'a> {
    let mut style = Style::default();

    if let Some(fg) = segment.style.foreground {
        style = style.fg(to_ratatui_color(fg));
    }

    if let Some(bg) = segment.style.background {
        style = style.bg(to_ratatui_color(bg));
    }

    match segment.style.weight {
        FontWeight::Bold => style = style.add_modifier(Modifier::BOLD),
        _ => {}
    }

    match segment.style.style {
        FontStyle::Italic => style = style.add_modifier(Modifier::ITALIC),
        _ => {}
    }

    Span::styled(segment.text.as_str(), style)
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
