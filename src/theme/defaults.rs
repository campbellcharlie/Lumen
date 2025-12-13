//! Built-in default themes

use super::color::{AnsiColor, Color};
use super::types::*;

/// "Docs" theme - Clean, documentation-focused
pub fn docs_theme() -> Theme {
    Theme {
        name: "Docs".to_string(),
        version: "1.0".to_string(),
        colors: ColorPalette {
            foreground: Color::rgb(220, 220, 220),
            background: Color::rgb(30, 30, 30),
            primary: Color::rgb(100, 180, 255),
            secondary: Color::rgb(150, 150, 150),
            accent: Color::rgb(255, 200, 100),
            muted: Color::rgb(100, 100, 100),
            error: Color::rgb(255, 100, 100),
            warning: Color::rgb(255, 200, 100),
            success: Color::rgb(100, 255, 150),
        },
        typography: Typography {
            emphasis: EmphasisStyle::Native,
        },
        spacing: Spacing {
            paragraph_spacing: 1,
            heading_margin_top: 2,
            heading_margin_bottom: 1,
            list_indent: 2,
            blockquote_indent: 2,
            code_block_padding: 1,
        },
        blocks: BlockStyles {
            heading: HeadingStyles {
                h1: HeadingStyle {
                    color: Color::rgb(100, 180, 255),
                    background: None,
                    border: Some(BorderConfig {
                        style: BorderStyle::Double,
                        color: Some(Color::rgb(100, 180, 255)),
                        sides: vec![BorderSide::Bottom],
                    }),
                    padding: (0, 0),
                    margin: (2, 1),
                    prefix: None,
                },
                h2: HeadingStyle {
                    color: Color::rgb(100, 180, 255),
                    background: None,
                    border: Some(BorderConfig {
                        style: BorderStyle::Single,
                        color: Some(Color::rgb(100, 180, 255)),
                        sides: vec![BorderSide::Bottom],
                    }),
                    padding: (0, 0),
                    margin: (2, 1),
                    prefix: None,
                },
                h3: HeadingStyle {
                    color: Color::rgb(150, 200, 255),
                    background: None,
                    border: None,
                    padding: (0, 0),
                    margin: (1, 1),
                    prefix: None,
                },
                h4: HeadingStyle {
                    color: Color::rgb(150, 200, 255),
                    ..Default::default()
                },
                h5: HeadingStyle {
                    color: Color::rgb(180, 200, 220),
                    ..Default::default()
                },
                h6: HeadingStyle {
                    color: Color::rgb(180, 200, 220),
                    ..Default::default()
                },
            },
            paragraph: ParagraphStyle::default(),
            code_block: CodeBlockStyle {
                background: Color::rgb(40, 40, 40),
                foreground: Color::rgb(220, 220, 220),
                border: Some(BorderConfig {
                    style: BorderStyle::Rounded,
                    color: Some(Color::rgb(80, 80, 80)),
                    sides: vec![
                        BorderSide::Top,
                        BorderSide::Right,
                        BorderSide::Bottom,
                        BorderSide::Left,
                    ],
                }),
                padding: (1, 2),
                show_language_badge: true,
            },
            blockquote: BlockQuoteStyle {
                color: Color::rgb(180, 180, 200),
                background: None,
                border: Some(BorderConfig {
                    style: BorderStyle::Heavy,
                    color: Some(Color::rgb(100, 100, 150)),
                    sides: vec![BorderSide::Left],
                }),
                indent: 2,
            },
            list: ListStyle {
                marker_color: Color::rgb(100, 180, 255),
                indent: 2,
            },
            table: TableStyle {
                border_style: BorderStyle::Single,
                header_background: Some(Color::rgb(50, 50, 80)),
                header_foreground: Some(Color::rgb(255, 255, 255)),
                row_separator: true,
                padding: 1,
            },
            horizontal_rule: HorizontalRuleStyle {
                style: BorderStyle::Single,
                color: Color::rgb(100, 100, 100),
            },
            callout: Default::default(),
        },
        inlines: InlineStyles {
            strong: TextStyle {
                foreground: Some(Color::rgb(255, 255, 255)),
                background: None,
                weight: FontWeight::Bold,
                style: FontStyle::Normal,
            },
            emphasis: TextStyle {
                foreground: Some(Color::rgb(200, 200, 255)),
                background: None,
                weight: FontWeight::Normal,
                style: FontStyle::Italic,
            },
            code: TextStyle {
                foreground: Some(Color::rgb(255, 150, 100)),
                background: Some(Color::rgb(50, 50, 50)),
                weight: FontWeight::Normal,
                style: FontStyle::Normal,
            },
            link: LinkStyle {
                foreground: Color::rgb(100, 180, 255),
                underline: true,
                show_url: UrlDisplayMode::Hover,
            },
            strikethrough: TextStyle {
                foreground: Some(Color::rgb(150, 150, 150)),
                background: None,
                weight: FontWeight::Normal,
                style: FontStyle::Normal,
            },
        },
    }
}

/// "Neon" theme - Vibrant, modern, high-contrast
pub fn neon_theme() -> Theme {
    Theme {
        name: "Neon".to_string(),
        version: "1.0".to_string(),
        colors: ColorPalette {
            foreground: Color::rgb(240, 240, 255),
            background: Color::rgb(10, 10, 20),
            primary: Color::rgb(0, 255, 255),
            secondary: Color::rgb(255, 0, 255),
            accent: Color::rgb(255, 255, 0),
            muted: Color::rgb(100, 100, 120),
            error: Color::rgb(255, 50, 100),
            warning: Color::rgb(255, 200, 0),
            success: Color::rgb(0, 255, 150),
        },
        typography: Typography {
            emphasis: EmphasisStyle::Native,
        },
        spacing: Spacing::default(),
        blocks: BlockStyles {
            heading: HeadingStyles {
                h1: HeadingStyle {
                    color: Color::rgb(0, 255, 255),
                    background: None,
                    border: Some(BorderConfig {
                        style: BorderStyle::Rounded,
                        color: Some(Color::rgb(0, 255, 255)),
                        sides: vec![BorderSide::Bottom],
                    }),
                    padding: (0, 0),
                    margin: (2, 1),
                    prefix: Some("▶ ".to_string()),
                },
                h2: HeadingStyle {
                    color: Color::rgb(255, 0, 255),
                    background: None,
                    border: Some(BorderConfig {
                        style: BorderStyle::Rounded,
                        color: Some(Color::rgb(255, 0, 255)),
                        sides: vec![BorderSide::Bottom],
                    }),
                    padding: (0, 0),
                    margin: (2, 1),
                    prefix: Some("■ ".to_string()),
                },
                h3: HeadingStyle {
                    color: Color::rgb(255, 255, 0),
                    prefix: Some("● ".to_string()),
                    ..Default::default()
                },
                h4: HeadingStyle {
                    color: Color::rgb(0, 255, 200),
                    ..Default::default()
                },
                h5: HeadingStyle {
                    color: Color::rgb(255, 150, 255),
                    ..Default::default()
                },
                h6: HeadingStyle {
                    color: Color::rgb(200, 200, 255),
                    ..Default::default()
                },
            },
            paragraph: ParagraphStyle::default(),
            code_block: CodeBlockStyle {
                background: Color::rgb(20, 20, 40),
                foreground: Color::rgb(0, 255, 200),
                border: Some(BorderConfig {
                    style: BorderStyle::Rounded,
                    color: Some(Color::rgb(0, 200, 200)),
                    sides: vec![
                        BorderSide::Top,
                        BorderSide::Right,
                        BorderSide::Bottom,
                        BorderSide::Left,
                    ],
                }),
                padding: (1, 2),
                show_language_badge: true,
            },
            blockquote: BlockQuoteStyle {
                color: Color::rgb(200, 200, 255),
                background: Some(Color::rgb(20, 20, 40)),
                border: Some(BorderConfig {
                    style: BorderStyle::Heavy,
                    color: Some(Color::rgb(255, 0, 255)),
                    sides: vec![BorderSide::Left],
                }),
                indent: 2,
            },
            list: ListStyle {
                marker_color: Color::rgb(0, 255, 255),
                indent: 2,
            },
            table: TableStyle {
                border_style: BorderStyle::Rounded,
                header_background: Some(Color::rgb(50, 0, 100)),
                header_foreground: Some(Color::rgb(255, 255, 255)),
                row_separator: true,
                padding: 1,
            },
            horizontal_rule: HorizontalRuleStyle {
                style: BorderStyle::Heavy,
                color: Color::rgb(0, 255, 255),
            },
            callout: Default::default(),
        },
        inlines: InlineStyles {
            strong: TextStyle {
                foreground: Some(Color::rgb(255, 255, 255)),
                background: None,
                weight: FontWeight::Bold,
                style: FontStyle::Normal,
            },
            emphasis: TextStyle {
                foreground: Some(Color::rgb(255, 200, 255)),
                background: None,
                weight: FontWeight::Normal,
                style: FontStyle::Italic,
            },
            code: TextStyle {
                foreground: Some(Color::rgb(0, 255, 200)),
                background: Some(Color::rgb(30, 30, 50)),
                weight: FontWeight::Normal,
                style: FontStyle::Normal,
            },
            link: LinkStyle {
                foreground: Color::rgb(0, 255, 255),
                underline: true,
                show_url: UrlDisplayMode::Hover,
            },
            strikethrough: TextStyle {
                foreground: Some(Color::rgb(120, 120, 140)),
                background: None,
                weight: FontWeight::Normal,
                style: FontStyle::Normal,
            },
        },
    }
}

/// "Minimal" theme - Low visual noise, maximum compatibility
pub fn minimal_theme() -> Theme {
    Theme {
        name: "Minimal".to_string(),
        version: "1.0".to_string(),
        colors: ColorPalette {
            foreground: Color::Ansi(AnsiColor::White),
            background: Color::Ansi(AnsiColor::Black),
            primary: Color::Ansi(AnsiColor::BrightWhite),
            secondary: Color::Ansi(AnsiColor::White),
            accent: Color::Ansi(AnsiColor::BrightWhite),
            muted: Color::Ansi(AnsiColor::BrightBlack),
            error: Color::Ansi(AnsiColor::Red),
            warning: Color::Ansi(AnsiColor::Yellow),
            success: Color::Ansi(AnsiColor::Green),
        },
        typography: Typography {
            emphasis: EmphasisStyle::Native,
        },
        spacing: Spacing::default(),
        blocks: BlockStyles {
            heading: HeadingStyles {
                h1: HeadingStyle {
                    color: Color::Ansi(AnsiColor::BrightWhite),
                    background: None,
                    border: Some(BorderConfig {
                        style: BorderStyle::Ascii,
                        color: None,
                        sides: vec![BorderSide::Bottom],
                    }),
                    padding: (0, 0),
                    margin: (2, 1),
                    prefix: Some("# ".to_string()),
                },
                h2: HeadingStyle {
                    color: Color::Ansi(AnsiColor::BrightWhite),
                    prefix: Some("## ".to_string()),
                    ..Default::default()
                },
                h3: HeadingStyle {
                    color: Color::Ansi(AnsiColor::White),
                    prefix: Some("### ".to_string()),
                    ..Default::default()
                },
                h4: HeadingStyle {
                    prefix: Some("#### ".to_string()),
                    ..Default::default()
                },
                h5: HeadingStyle {
                    prefix: Some("##### ".to_string()),
                    ..Default::default()
                },
                h6: HeadingStyle {
                    prefix: Some("###### ".to_string()),
                    ..Default::default()
                },
            },
            paragraph: ParagraphStyle::default(),
            code_block: CodeBlockStyle {
                background: Color::Reset,
                foreground: Color::Ansi(AnsiColor::White),
                border: Some(BorderConfig {
                    style: BorderStyle::Ascii,
                    color: None,
                    sides: vec![
                        BorderSide::Top,
                        BorderSide::Right,
                        BorderSide::Bottom,
                        BorderSide::Left,
                    ],
                }),
                padding: (0, 1),
                show_language_badge: false,
            },
            blockquote: BlockQuoteStyle {
                color: Color::Ansi(AnsiColor::BrightBlack),
                background: None,
                border: Some(BorderConfig {
                    style: BorderStyle::Ascii,
                    color: None,
                    sides: vec![BorderSide::Left],
                }),
                indent: 2,
            },
            list: ListStyle {
                marker_color: Color::Reset,
                indent: 2,
            },
            table: TableStyle {
                border_style: BorderStyle::Ascii,
                header_background: None,
                header_foreground: Some(Color::Ansi(AnsiColor::BrightWhite)),
                row_separator: true,
                padding: 1,
            },
            horizontal_rule: HorizontalRuleStyle {
                style: BorderStyle::Ascii,
                color: Color::Ansi(AnsiColor::BrightBlack),
            },
            callout: Default::default(),
        },
        inlines: InlineStyles {
            strong: TextStyle {
                foreground: None,
                background: None,
                weight: FontWeight::Bold,
                style: FontStyle::Normal,
            },
            emphasis: TextStyle {
                foreground: None,
                background: None,
                weight: FontWeight::Normal,
                style: FontStyle::Italic,
            },
            code: TextStyle {
                foreground: Some(Color::Ansi(AnsiColor::BrightWhite)),
                background: None,
                weight: FontWeight::Normal,
                style: FontStyle::Normal,
            },
            link: LinkStyle {
                foreground: Color::Ansi(AnsiColor::White),
                underline: false,
                show_url: UrlDisplayMode::Inline,
            },
            strikethrough: TextStyle::default(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docs_theme() {
        let theme = docs_theme();
        assert_eq!(theme.name, "Docs");
        assert_eq!(theme.version, "1.0");
    }

    #[test]
    fn test_neon_theme() {
        let theme = neon_theme();
        assert_eq!(theme.name, "Neon");
        assert!(matches!(
            theme.blocks.heading.h1.color,
            Color::Rgb(0, 255, 255)
        ));
    }

    #[test]
    fn test_minimal_theme() {
        let theme = minimal_theme();
        assert_eq!(theme.name, "Minimal");
        // Minimal theme should use ASCII borders
        assert_eq!(theme.blocks.table.border_style, BorderStyle::Ascii);
    }
}
