//! Inline text layout and wrapping

use super::types::{ImageReference, Line, TextStyle};
use crate::ir::Inline;
use crate::theme::{FontStyle, FontWeight, Theme};

/// Context for laying out inline elements
struct InlineLayoutContext<'a> {
    current_line: &'a mut Line,
    current_width: &'a mut u16,
    max_width: u16,
    lines: &'a mut Vec<Line>,
    base_style: TextStyle,
    link_url: Option<String>,
    theme: &'a Theme,
    y_offset: u16,
    images: &'a mut Vec<ImageReference>,
    inline_images_mode: bool,
    inline_images: &'a mut Vec<(u16, String, String)>,
}

/// Layout inline elements into wrapped lines
/// Returns (lines, inline_images) where inline_images contains images to be rendered inline
pub fn layout_text(
    inlines: &[Inline],
    max_width: u16,
    theme: &Theme,
    y_offset: u16,
    images: &mut Vec<ImageReference>,
    inline_images_mode: bool,
) -> (Vec<Line>, Vec<(u16, String, String)>) {
    let mut lines = Vec::new();
    let mut current_line = Line::new();
    let mut current_width = 0u16;
    let mut inline_images = Vec::new(); // (line_index, path, alt_text)

    let mut ctx = InlineLayoutContext {
        current_line: &mut current_line,
        current_width: &mut current_width,
        max_width,
        lines: &mut lines,
        base_style: TextStyle::default(),
        link_url: None,
        theme,
        y_offset,
        images,
        inline_images_mode,
        inline_images: &mut inline_images,
    };

    for inline in inlines {
        layout_inline(inline, &mut ctx);
    }

    // Push remaining line
    if !current_line.is_empty() {
        lines.push(current_line);
    }

    // Ensure at least one line for empty content
    if lines.is_empty() {
        lines.push(Line::new());
    }

    (lines, inline_images)
}

fn layout_inline(inline: &Inline, ctx: &mut InlineLayoutContext) {
    match inline {
        Inline::Text(text) => {
            layout_text_content(
                text,
                ctx.current_line,
                ctx.current_width,
                ctx.max_width,
                ctx.lines,
                ctx.base_style,
                ctx.link_url.clone(),
            );
        }
        Inline::Strong(nested) => {
            let old_style = ctx.base_style;
            ctx.base_style = TextStyle {
                weight: FontWeight::Bold,
                ..old_style
            };
            for inner in nested {
                layout_inline(inner, ctx);
            }
            ctx.base_style = old_style;
        }
        Inline::Emphasis(nested) => {
            let old_style = ctx.base_style;
            ctx.base_style = TextStyle {
                style: FontStyle::Italic,
                ..old_style
            };
            for inner in nested {
                layout_inline(inner, ctx);
            }
            ctx.base_style = old_style;
        }
        Inline::Strikethrough(nested) => {
            // Use base style for now (strikethrough rendering in Phase 4)
            for inner in nested {
                layout_inline(inner, ctx);
            }
        }
        Inline::Code(code) => {
            let style = TextStyle {
                foreground: ctx.theme.inlines.code.foreground,
                background: ctx.theme.inlines.code.background,
                ..ctx.base_style
            };
            layout_text_content(
                code,
                ctx.current_line,
                ctx.current_width,
                ctx.max_width,
                ctx.lines,
                style,
                ctx.link_url.clone(),
            );
        }
        Inline::Link { text, url, .. } => {
            let old_style = ctx.base_style;
            let old_link = ctx.link_url.clone();
            ctx.base_style = TextStyle {
                foreground: Some(ctx.theme.inlines.link.foreground),
                ..old_style
            };
            ctx.link_url = Some(url.clone());

            // Pass the URL to nested content so it becomes clickable
            for inner in text {
                layout_inline(inner, ctx);
            }

            ctx.base_style = old_style;
            ctx.link_url = old_link;

            // Optionally show URL inline (but don't make it clickable)
            if matches!(
                ctx.theme.inlines.link.show_url,
                crate::theme::UrlDisplayMode::Inline
            ) {
                let url_text = format!(" ({})", url);
                layout_text_content(
                    &url_text,
                    ctx.current_line,
                    ctx.current_width,
                    ctx.max_width,
                    ctx.lines,
                    TextStyle {
                        foreground: Some(ctx.theme.colors.muted),
                        ..old_style
                    },
                    None, // Don't make the displayed URL itself clickable
                );
            }
        }
        Inline::Image { url, alt, .. } => {
            // Calculate which line this image appears on (within this text block)
            let line_number = ctx.lines.len() as u16; // Current line being built

            if ctx.inline_images_mode {
                // For inline images, collect them to be rendered as layout nodes
                // Don't add placeholder text since the actual image will be rendered
                ctx.inline_images
                    .push((line_number, url.clone(), alt.clone()));
            } else {
                // For sidebar images, collect image reference and add placeholder text
                ctx.images.push(ImageReference {
                    path: url.clone(),
                    alt_text: alt.clone(),
                    y_position: ctx.y_offset + line_number, // Absolute position
                });

                // Add placeholder text segment for sidebar mode
                let image_text = format!("[IMAGE: {}]", alt);
                let style = TextStyle {
                    foreground: Some(ctx.theme.colors.muted),
                    ..ctx.base_style
                };
                ctx.current_line.add_segment_full(
                    image_text,
                    style,
                    None,
                    Some(url.clone()),
                    Some(alt.clone()),
                );
            }
        }
        Inline::LineBreak => {
            // Force new line
            if !ctx.current_line.is_empty() {
                ctx.lines.push(std::mem::take(ctx.current_line));
                *ctx.current_width = 0;
            }
        }
        Inline::SoftBreak => {
            // In a terminal viewer, treat soft breaks as line breaks for better readability
            // This makes the rendered output match the source file more closely
            if !ctx.current_line.is_empty() {
                ctx.lines.push(std::mem::take(ctx.current_line));
                *ctx.current_width = 0;
            }
        }
    }
}

fn layout_text_content(
    text: &str,
    current_line: &mut Line,
    current_width: &mut u16,
    max_width: u16,
    lines: &mut Vec<Line>,
    style: TextStyle,
    link_url: Option<String>,
) {
    // Split by whitespace for word wrapping
    let words: Vec<&str> = text.split_whitespace().collect();

    for (i, word) in words.iter().enumerate() {
        let word_len = word.len() as u16;
        let need_space = i > 0 || *current_width > 0;
        let space_len = if need_space { 1 } else { 0 };

        // Check if word fits on current line
        if *current_width + space_len + word_len > max_width && *current_width > 0 {
            // Wrap to next line
            lines.push(std::mem::take(current_line));
            *current_width = 0;
        }

        // Add space before word (if not at line start)
        if *current_width > 0 {
            current_line.add_segment_with_link(" ".to_string(), style, link_url.clone());
            *current_width += 1;
        }

        // Handle very long words that don't fit even on empty line
        if word_len > max_width {
            // Break word into chunks
            let mut remaining = *word;
            while !remaining.is_empty() {
                let chunk_len = (max_width - *current_width).min(remaining.len() as u16) as usize;
                if chunk_len == 0 {
                    // Current line is full, wrap
                    lines.push(std::mem::take(current_line));
                    *current_width = 0;
                    continue;
                }

                let chunk = &remaining[..chunk_len];
                current_line.add_segment_with_link(chunk.to_string(), style, link_url.clone());
                *current_width += chunk_len as u16;
                remaining = &remaining[chunk_len..];

                if !remaining.is_empty() {
                    // More to go, wrap to next line
                    lines.push(std::mem::take(current_line));
                    *current_width = 0;
                }
            }
        } else {
            // Normal word, add to line
            current_line.add_segment_with_link(word.to_string(), style, link_url.clone());
            *current_width += word_len;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme;

    #[test]
    fn test_simple_text_layout() {
        let theme = theme::docs_theme();
        let inlines = vec![Inline::Text("Hello World".to_string())];
        let mut images = Vec::new();

        let (lines, _) = layout_text(&inlines, 80, &theme, 0, &mut images, false);
        assert_eq!(lines.len(), 1);
        // "Hello" + " " + "World" = 3 segments
        assert_eq!(lines[0].segments.len(), 3);
    }

    #[test]
    fn test_text_wrapping() {
        let theme = theme::docs_theme();
        let inlines = vec![Inline::Text(
            "This is a long line that should wrap".to_string(),
        )];
        let mut images = Vec::new();

        let (lines, _) = layout_text(&inlines, 20, &theme, 0, &mut images, false);
        assert!(lines.len() > 1, "Text should wrap into multiple lines");
    }

    #[test]
    fn test_long_word_breaking() {
        let theme = theme::docs_theme();
        let inlines = vec![Inline::Text(
            "Supercalifragilisticexpialidocious".to_string(),
        )];
        let mut images = Vec::new();

        let (lines, _) = layout_text(&inlines, 10, &theme, 0, &mut images, false);
        assert!(lines.len() > 1, "Long word should break across lines");
    }

    #[test]
    fn test_strong_emphasis() {
        let theme = theme::docs_theme();
        let inlines = vec![Inline::Strong(vec![Inline::Text("Bold".to_string())])];
        let mut images = Vec::new();

        let (lines, _) = layout_text(&inlines, 80, &theme, 0, &mut images, false);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].segments[0].style.weight, FontWeight::Bold);
    }

    #[test]
    fn test_line_break() {
        let theme = theme::docs_theme();
        let inlines = vec![
            Inline::Text("Line 1".to_string()),
            Inline::LineBreak,
            Inline::Text("Line 2".to_string()),
        ];
        let mut images = Vec::new();

        let (lines, _) = layout_text(&inlines, 80, &theme, 0, &mut images, false);
        assert_eq!(lines.len(), 2);
    }

    #[test]
    fn test_empty_content() {
        let theme = theme::docs_theme();
        let inlines = vec![];
        let mut images = Vec::new();

        let (lines, _) = layout_text(&inlines, 80, &theme, 0, &mut images, false);
        assert_eq!(lines.len(), 1); // At least one empty line
    }
}
