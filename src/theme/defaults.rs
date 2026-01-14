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
                background: Some(Color::rgb(40, 40, 60)),  // Subtle blue-tinted band
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
                background: Some(Color::rgb(60, 40, 60)),  // Subtle magenta-tinted band
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
                    prefix: None,
                },
                h2: HeadingStyle {
                    color: Color::Ansi(AnsiColor::BrightWhite),
                    prefix: None,
                    ..Default::default()
                },
                h3: HeadingStyle {
                    color: Color::Ansi(AnsiColor::White),
                    prefix: None,
                    ..Default::default()
                },
                h4: HeadingStyle {
                    prefix: None,
                    ..Default::default()
                },
                h5: HeadingStyle {
                    prefix: None,
                    ..Default::default()
                },
                h6: HeadingStyle {
                    prefix: None,
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
                background: Some(Color::Ansi256(236)),  // Subtle gray band
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

/// "Dracula" theme - Dark purple with vibrant accents
pub fn dracula_theme() -> Theme {
    Theme {
        name: "Dracula".to_string(),
        version: "1.0".to_string(),
        colors: ColorPalette {
            foreground: Color::rgb(248, 248, 242),
            background: Color::rgb(40, 42, 54),
            primary: Color::rgb(189, 147, 249),
            secondary: Color::rgb(139, 233, 253),
            accent: Color::rgb(255, 121, 198),
            muted: Color::rgb(98, 114, 164),
            error: Color::rgb(255, 85, 85),
            warning: Color::rgb(241, 250, 140),
            success: Color::rgb(80, 250, 123),
        },
        typography: Typography {
            emphasis: EmphasisStyle::Native,
        },
        spacing: Spacing::default(),
        blocks: BlockStyles {
            heading: HeadingStyles {
                h1: HeadingStyle {
                    color: Color::rgb(189, 147, 249),
                    background: None,
                    border: Some(BorderConfig {
                        style: BorderStyle::Heavy,
                        color: Some(Color::rgb(189, 147, 249)),
                        sides: vec![BorderSide::Bottom],
                    }),
                    padding: (0, 0),
                    margin: (2, 1),
                    prefix: None,
                },
                h2: HeadingStyle {
                    color: Color::rgb(139, 233, 253),
                    background: None,
                    border: Some(BorderConfig {
                        style: BorderStyle::Single,
                        color: Some(Color::rgb(139, 233, 253)),
                        sides: vec![BorderSide::Bottom],
                    }),
                    padding: (0, 0),
                    margin: (2, 1),
                    prefix: None,
                },
                h3: HeadingStyle {
                    color: Color::rgb(255, 121, 198),
                    ..Default::default()
                },
                h4: HeadingStyle {
                    color: Color::rgb(80, 250, 123),
                    ..Default::default()
                },
                h5: HeadingStyle {
                    color: Color::rgb(241, 250, 140),
                    ..Default::default()
                },
                h6: HeadingStyle {
                    color: Color::rgb(248, 248, 242),
                    ..Default::default()
                },
            },
            paragraph: ParagraphStyle::default(),
            code_block: CodeBlockStyle {
                background: Color::rgb(68, 71, 90),
                foreground: Color::rgb(248, 248, 242),
                border: Some(BorderConfig {
                    style: BorderStyle::Rounded,
                    color: Some(Color::rgb(98, 114, 164)),
                    sides: vec![BorderSide::Top, BorderSide::Right, BorderSide::Bottom, BorderSide::Left],
                }),
                padding: (1, 2),
                show_language_badge: true,
            },
            blockquote: BlockQuoteStyle {
                color: Color::rgb(189, 147, 249),
                background: None,
                border: Some(BorderConfig {
                    style: BorderStyle::Heavy,
                    color: Some(Color::rgb(255, 121, 198)),
                    sides: vec![BorderSide::Left],
                }),
                indent: 2,
            },
            list: ListStyle {
                marker_color: Color::rgb(255, 121, 198),
                indent: 2,
            },
            table: TableStyle {
                border_style: BorderStyle::Single,
                header_background: Some(Color::rgb(68, 71, 90)),
                header_foreground: Some(Color::rgb(189, 147, 249)),
                row_separator: true,
                padding: 1,
            },
            horizontal_rule: HorizontalRuleStyle {
                style: BorderStyle::Single,
                color: Color::rgb(98, 114, 164),
            },
            callout: Default::default(),
        },
        inlines: InlineStyles {
            strong: TextStyle {
                foreground: Some(Color::rgb(255, 184, 108)),
                background: None,
                weight: FontWeight::Bold,
                style: FontStyle::Normal,
            },
            emphasis: TextStyle {
                foreground: Some(Color::rgb(189, 147, 249)),
                background: Some(Color::rgb(55, 58, 70)),
                weight: FontWeight::Normal,
                style: FontStyle::Italic,
            },
            code: TextStyle {
                foreground: Some(Color::rgb(241, 250, 140)),
                background: Some(Color::rgb(68, 71, 90)),
                weight: FontWeight::Normal,
                style: FontStyle::Normal,
            },
            link: LinkStyle {
                foreground: Color::rgb(139, 233, 253),
                underline: true,
                show_url: UrlDisplayMode::Hover,
            },
            strikethrough: TextStyle {
                foreground: Some(Color::rgb(98, 114, 164)),
                background: None,
                weight: FontWeight::Normal,
                style: FontStyle::Normal,
            },
        },
    }
}

/// "Monokai" theme - Classic sublime text inspired
pub fn monokai_theme() -> Theme {
    Theme {
        name: "Monokai".to_string(),
        version: "1.0".to_string(),
        colors: ColorPalette {
            foreground: Color::rgb(248, 248, 242),
            background: Color::rgb(39, 40, 34),
            primary: Color::rgb(102, 217, 239),
            secondary: Color::rgb(166, 226, 46),
            accent: Color::rgb(249, 38, 114),
            muted: Color::rgb(117, 113, 94),
            error: Color::rgb(249, 38, 114),
            warning: Color::rgb(253, 151, 31),
            success: Color::rgb(166, 226, 46),
        },
        typography: Typography {
            emphasis: EmphasisStyle::Native,
        },
        spacing: Spacing::default(),
        blocks: BlockStyles {
            heading: HeadingStyles {
                h1: HeadingStyle {
                    color: Color::rgb(249, 38, 114),
                    background: None,
                    border: Some(BorderConfig {
                        style: BorderStyle::Double,
                        color: Some(Color::rgb(249, 38, 114)),
                        sides: vec![BorderSide::Bottom],
                    }),
                    padding: (0, 0),
                    margin: (2, 1),
                    prefix: None,
                },
                h2: HeadingStyle {
                    color: Color::rgb(102, 217, 239),
                    background: None,
                    border: Some(BorderConfig {
                        style: BorderStyle::Single,
                        color: Some(Color::rgb(102, 217, 239)),
                        sides: vec![BorderSide::Bottom],
                    }),
                    padding: (0, 0),
                    margin: (2, 1),
                    prefix: None,
                },
                h3: HeadingStyle {
                    color: Color::rgb(166, 226, 46),
                    ..Default::default()
                },
                h4: HeadingStyle {
                    color: Color::rgb(253, 151, 31),
                    ..Default::default()
                },
                h5: HeadingStyle {
                    color: Color::rgb(174, 129, 255),
                    ..Default::default()
                },
                h6: HeadingStyle {
                    color: Color::rgb(248, 248, 242),
                    ..Default::default()
                },
            },
            paragraph: ParagraphStyle::default(),
            code_block: CodeBlockStyle {
                background: Color::rgb(60, 61, 54),
                foreground: Color::rgb(248, 248, 242),
                border: Some(BorderConfig {
                    style: BorderStyle::Single,
                    color: Some(Color::rgb(117, 113, 94)),
                    sides: vec![BorderSide::Top, BorderSide::Right, BorderSide::Bottom, BorderSide::Left],
                }),
                padding: (1, 2),
                show_language_badge: true,
            },
            blockquote: BlockQuoteStyle {
                color: Color::rgb(230, 219, 116),
                background: None,
                border: Some(BorderConfig {
                    style: BorderStyle::Heavy,
                    color: Some(Color::rgb(166, 226, 46)),
                    sides: vec![BorderSide::Left],
                }),
                indent: 2,
            },
            list: ListStyle {
                marker_color: Color::rgb(249, 38, 114),
                indent: 2,
            },
            table: TableStyle {
                border_style: BorderStyle::Single,
                header_background: Some(Color::rgb(60, 61, 54)),
                header_foreground: Some(Color::rgb(249, 38, 114)),
                row_separator: true,
                padding: 1,
            },
            horizontal_rule: HorizontalRuleStyle {
                style: BorderStyle::Single,
                color: Color::rgb(117, 113, 94),
            },
            callout: Default::default(),
        },
        inlines: InlineStyles {
            strong: TextStyle {
                foreground: Some(Color::rgb(253, 151, 31)),
                background: None,
                weight: FontWeight::Bold,
                style: FontStyle::Normal,
            },
            emphasis: TextStyle {
                foreground: Some(Color::rgb(230, 219, 116)),
                background: Some(Color::rgb(50, 51, 44)),
                weight: FontWeight::Normal,
                style: FontStyle::Italic,
            },
            code: TextStyle {
                foreground: Some(Color::rgb(230, 219, 116)),
                background: Some(Color::rgb(60, 61, 54)),
                weight: FontWeight::Normal,
                style: FontStyle::Normal,
            },
            link: LinkStyle {
                foreground: Color::rgb(102, 217, 239),
                underline: true,
                show_url: UrlDisplayMode::Hover,
            },
            strikethrough: TextStyle {
                foreground: Some(Color::rgb(117, 113, 94)),
                background: None,
                weight: FontWeight::Normal,
                style: FontStyle::Normal,
            },
        },
    }
}

/// "Solarized" theme - Low-contrast, easy on the eyes
pub fn solarized_theme() -> Theme {
    Theme {
        name: "Solarized".to_string(),
        version: "1.0".to_string(),
        colors: ColorPalette {
            foreground: Color::rgb(131, 148, 150),
            background: Color::rgb(0, 43, 54),
            primary: Color::rgb(38, 139, 210),
            secondary: Color::rgb(42, 161, 152),
            accent: Color::rgb(211, 54, 130),
            muted: Color::rgb(88, 110, 117),
            error: Color::rgb(220, 50, 47),
            warning: Color::rgb(181, 137, 0),
            success: Color::rgb(133, 153, 0),
        },
        typography: Typography {
            emphasis: EmphasisStyle::Native,
        },
        spacing: Spacing::default(),
        blocks: BlockStyles {
            heading: HeadingStyles {
                h1: HeadingStyle {
                    color: Color::rgb(38, 139, 210),
                    background: None,
                    border: Some(BorderConfig {
                        style: BorderStyle::Single,
                        color: Some(Color::rgb(38, 139, 210)),
                        sides: vec![BorderSide::Bottom],
                    }),
                    padding: (0, 0),
                    margin: (2, 1),
                    prefix: None,
                },
                h2: HeadingStyle {
                    color: Color::rgb(42, 161, 152),
                    background: None,
                    border: Some(BorderConfig {
                        style: BorderStyle::Single,
                        color: Some(Color::rgb(42, 161, 152)),
                        sides: vec![BorderSide::Bottom],
                    }),
                    padding: (0, 0),
                    margin: (2, 1),
                    prefix: None,
                },
                h3: HeadingStyle {
                    color: Color::rgb(211, 54, 130),
                    ..Default::default()
                },
                h4: HeadingStyle {
                    color: Color::rgb(181, 137, 0),
                    ..Default::default()
                },
                h5: HeadingStyle {
                    color: Color::rgb(133, 153, 0),
                    ..Default::default()
                },
                h6: HeadingStyle {
                    color: Color::rgb(147, 161, 161),
                    ..Default::default()
                },
            },
            paragraph: ParagraphStyle::default(),
            code_block: CodeBlockStyle {
                background: Color::rgb(7, 54, 66),
                foreground: Color::rgb(131, 148, 150),
                border: Some(BorderConfig {
                    style: BorderStyle::Single,
                    color: Some(Color::rgb(88, 110, 117)),
                    sides: vec![BorderSide::Top, BorderSide::Right, BorderSide::Bottom, BorderSide::Left],
                }),
                padding: (1, 2),
                show_language_badge: true,
            },
            blockquote: BlockQuoteStyle {
                color: Color::rgb(147, 161, 161),
                background: None,
                border: Some(BorderConfig {
                    style: BorderStyle::Heavy,
                    color: Some(Color::rgb(42, 161, 152)),
                    sides: vec![BorderSide::Left],
                }),
                indent: 2,
            },
            list: ListStyle {
                marker_color: Color::rgb(38, 139, 210),
                indent: 2,
            },
            table: TableStyle {
                border_style: BorderStyle::Single,
                header_background: Some(Color::rgb(7, 54, 66)),
                header_foreground: Some(Color::rgb(38, 139, 210)),
                row_separator: true,
                padding: 1,
            },
            horizontal_rule: HorizontalRuleStyle {
                style: BorderStyle::Single,
                color: Color::rgb(88, 110, 117),
            },
            callout: Default::default(),
        },
        inlines: InlineStyles {
            strong: TextStyle {
                foreground: Some(Color::rgb(203, 75, 22)),
                background: None,
                weight: FontWeight::Bold,
                style: FontStyle::Normal,
            },
            emphasis: TextStyle {
                foreground: Some(Color::rgb(108, 113, 196)),
                background: Some(Color::rgb(7, 54, 66)),
                weight: FontWeight::Normal,
                style: FontStyle::Italic,
            },
            code: TextStyle {
                foreground: Some(Color::rgb(133, 153, 0)),
                background: Some(Color::rgb(7, 54, 66)),
                weight: FontWeight::Normal,
                style: FontStyle::Normal,
            },
            link: LinkStyle {
                foreground: Color::rgb(38, 139, 210),
                underline: true,
                show_url: UrlDisplayMode::Hover,
            },
            strikethrough: TextStyle {
                foreground: Some(Color::rgb(88, 110, 117)),
                background: None,
                weight: FontWeight::Normal,
                style: FontStyle::Normal,
            },
        },
    }
}

/// "Gruvbox" theme - Retro groove with warm colors
pub fn gruvbox_theme() -> Theme {
    Theme {
        name: "Gruvbox".to_string(),
        version: "1.0".to_string(),
        colors: ColorPalette {
            foreground: Color::rgb(235, 219, 178),
            background: Color::rgb(40, 40, 40),
            primary: Color::rgb(251, 184, 108),
            secondary: Color::rgb(184, 187, 38),
            accent: Color::rgb(254, 128, 25),
            muted: Color::rgb(146, 131, 116),
            error: Color::rgb(251, 73, 52),
            warning: Color::rgb(250, 189, 47),
            success: Color::rgb(184, 187, 38),
        },
        typography: Typography {
            emphasis: EmphasisStyle::Native,
        },
        spacing: Spacing::default(),
        blocks: BlockStyles {
            heading: HeadingStyles {
                h1: HeadingStyle {
                    color: Color::rgb(251, 184, 108),
                    background: None,
                    border: Some(BorderConfig {
                        style: BorderStyle::Heavy,
                        color: Some(Color::rgb(251, 184, 108)),
                        sides: vec![BorderSide::Bottom],
                    }),
                    padding: (0, 0),
                    margin: (2, 1),
                    prefix: None,
                },
                h2: HeadingStyle {
                    color: Color::rgb(184, 187, 38),
                    background: None,
                    border: Some(BorderConfig {
                        style: BorderStyle::Single,
                        color: Some(Color::rgb(184, 187, 38)),
                        sides: vec![BorderSide::Bottom],
                    }),
                    padding: (0, 0),
                    margin: (2, 1),
                    prefix: None,
                },
                h3: HeadingStyle {
                    color: Color::rgb(142, 192, 124),
                    ..Default::default()
                },
                h4: HeadingStyle {
                    color: Color::rgb(254, 128, 25),
                    ..Default::default()
                },
                h5: HeadingStyle {
                    color: Color::rgb(211, 134, 155),
                    ..Default::default()
                },
                h6: HeadingStyle {
                    color: Color::rgb(235, 219, 178),
                    ..Default::default()
                },
            },
            paragraph: ParagraphStyle::default(),
            code_block: CodeBlockStyle {
                background: Color::rgb(60, 56, 54),
                foreground: Color::rgb(235, 219, 178),
                border: Some(BorderConfig {
                    style: BorderStyle::Rounded,
                    color: Some(Color::rgb(146, 131, 116)),
                    sides: vec![BorderSide::Top, BorderSide::Right, BorderSide::Bottom, BorderSide::Left],
                }),
                padding: (1, 2),
                show_language_badge: true,
            },
            blockquote: BlockQuoteStyle {
                color: Color::rgb(184, 187, 38),
                background: None,
                border: Some(BorderConfig {
                    style: BorderStyle::Heavy,
                    color: Some(Color::rgb(254, 128, 25)),
                    sides: vec![BorderSide::Left],
                }),
                indent: 2,
            },
            list: ListStyle {
                marker_color: Color::rgb(251, 184, 108),
                indent: 2,
            },
            table: TableStyle {
                border_style: BorderStyle::Single,
                header_background: Some(Color::rgb(60, 56, 54)),
                header_foreground: Some(Color::rgb(251, 184, 108)),
                row_separator: true,
                padding: 1,
            },
            horizontal_rule: HorizontalRuleStyle {
                style: BorderStyle::Single,
                color: Color::rgb(146, 131, 116),
            },
            callout: Default::default(),
        },
        inlines: InlineStyles {
            strong: TextStyle {
                foreground: Some(Color::rgb(250, 189, 47)),
                background: None,
                weight: FontWeight::Bold,
                style: FontStyle::Normal,
            },
            emphasis: TextStyle {
                foreground: Some(Color::rgb(211, 134, 155)),
                background: Some(Color::rgb(50, 48, 47)),
                weight: FontWeight::Normal,
                style: FontStyle::Italic,
            },
            code: TextStyle {
                foreground: Some(Color::rgb(142, 192, 124)),
                background: Some(Color::rgb(60, 56, 54)),
                weight: FontWeight::Normal,
                style: FontStyle::Normal,
            },
            link: LinkStyle {
                foreground: Color::rgb(131, 165, 152),
                underline: true,
                show_url: UrlDisplayMode::Hover,
            },
            strikethrough: TextStyle {
                foreground: Some(Color::rgb(146, 131, 116)),
                background: None,
                weight: FontWeight::Normal,
                style: FontStyle::Normal,
            },
        },
    }
}

/// "Nord" theme - Arctic, blue-ish coldness
pub fn nord_theme() -> Theme {
    Theme {
        name: "Nord".to_string(),
        version: "1.0".to_string(),
        colors: ColorPalette {
            foreground: Color::rgb(216, 222, 233),
            background: Color::rgb(46, 52, 64),
            primary: Color::rgb(136, 192, 208),
            secondary: Color::rgb(129, 161, 193),
            accent: Color::rgb(163, 190, 140),
            muted: Color::rgb(76, 86, 106),
            error: Color::rgb(191, 97, 106),
            warning: Color::rgb(235, 203, 139),
            success: Color::rgb(163, 190, 140),
        },
        typography: Typography {
            emphasis: EmphasisStyle::Native,
        },
        spacing: Spacing::default(),
        blocks: BlockStyles {
            heading: HeadingStyles {
                h1: HeadingStyle {
                    color: Color::rgb(136, 192, 208),
                    background: None,
                    border: Some(BorderConfig {
                        style: BorderStyle::Double,
                        color: Some(Color::rgb(136, 192, 208)),
                        sides: vec![BorderSide::Bottom],
                    }),
                    padding: (0, 0),
                    margin: (2, 1),
                    prefix: None,
                },
                h2: HeadingStyle {
                    color: Color::rgb(129, 161, 193),
                    background: None,
                    border: Some(BorderConfig {
                        style: BorderStyle::Single,
                        color: Some(Color::rgb(129, 161, 193)),
                        sides: vec![BorderSide::Bottom],
                    }),
                    padding: (0, 0),
                    margin: (2, 1),
                    prefix: None,
                },
                h3: HeadingStyle {
                    color: Color::rgb(143, 188, 187),
                    ..Default::default()
                },
                h4: HeadingStyle {
                    color: Color::rgb(163, 190, 140),
                    ..Default::default()
                },
                h5: HeadingStyle {
                    color: Color::rgb(180, 142, 173),
                    ..Default::default()
                },
                h6: HeadingStyle {
                    color: Color::rgb(216, 222, 233),
                    ..Default::default()
                },
            },
            paragraph: ParagraphStyle::default(),
            code_block: CodeBlockStyle {
                background: Color::rgb(59, 66, 82),
                foreground: Color::rgb(216, 222, 233),
                border: Some(BorderConfig {
                    style: BorderStyle::Rounded,
                    color: Some(Color::rgb(76, 86, 106)),
                    sides: vec![BorderSide::Top, BorderSide::Right, BorderSide::Bottom, BorderSide::Left],
                }),
                padding: (1, 2),
                show_language_badge: true,
            },
            blockquote: BlockQuoteStyle {
                color: Color::rgb(229, 233, 240),
                background: None,
                border: Some(BorderConfig {
                    style: BorderStyle::Heavy,
                    color: Some(Color::rgb(136, 192, 208)),
                    sides: vec![BorderSide::Left],
                }),
                indent: 2,
            },
            list: ListStyle {
                marker_color: Color::rgb(136, 192, 208),
                indent: 2,
            },
            table: TableStyle {
                border_style: BorderStyle::Single,
                header_background: Some(Color::rgb(59, 66, 82)),
                header_foreground: Some(Color::rgb(136, 192, 208)),
                row_separator: true,
                padding: 1,
            },
            horizontal_rule: HorizontalRuleStyle {
                style: BorderStyle::Single,
                color: Color::rgb(76, 86, 106),
            },
            callout: Default::default(),
        },
        inlines: InlineStyles {
            strong: TextStyle {
                foreground: Some(Color::rgb(235, 203, 139)),
                background: None,
                weight: FontWeight::Bold,
                style: FontStyle::Normal,
            },
            emphasis: TextStyle {
                foreground: Some(Color::rgb(180, 142, 173)),
                background: Some(Color::rgb(59, 66, 82)),
                weight: FontWeight::Normal,
                style: FontStyle::Italic,
            },
            code: TextStyle {
                foreground: Some(Color::rgb(163, 190, 140)),
                background: Some(Color::rgb(59, 66, 82)),
                weight: FontWeight::Normal,
                style: FontStyle::Normal,
            },
            link: LinkStyle {
                foreground: Color::rgb(136, 192, 208),
                underline: true,
                show_url: UrlDisplayMode::Hover,
            },
            strikethrough: TextStyle {
                foreground: Some(Color::rgb(76, 86, 106)),
                background: None,
                weight: FontWeight::Normal,
                style: FontStyle::Normal,
            },
        },
    }
}

/// "Tokyo Night" theme - Modern dark Tokyo-inspired
pub fn tokyo_night_theme() -> Theme {
    Theme {
        name: "Tokyo Night".to_string(),
        version: "1.0".to_string(),
        colors: ColorPalette {
            foreground: Color::rgb(169, 177, 214),
            background: Color::rgb(26, 27, 38),
            primary: Color::rgb(125, 207, 255),
            secondary: Color::rgb(187, 154, 247),
            accent: Color::rgb(255, 117, 127),
            muted: Color::rgb(86, 95, 137),
            error: Color::rgb(247, 118, 142),
            warning: Color::rgb(224, 175, 104),
            success: Color::rgb(158, 206, 106),
        },
        typography: Typography {
            emphasis: EmphasisStyle::Native,
        },
        spacing: Spacing::default(),
        blocks: BlockStyles {
            heading: HeadingStyles {
                h1: HeadingStyle {
                    color: Color::rgb(125, 207, 255),
                    background: None,
                    border: Some(BorderConfig {
                        style: BorderStyle::Heavy,
                        color: Some(Color::rgb(125, 207, 255)),
                        sides: vec![BorderSide::Bottom],
                    }),
                    padding: (0, 0),
                    margin: (2, 1),
                    prefix: None,
                },
                h2: HeadingStyle {
                    color: Color::rgb(187, 154, 247),
                    background: None,
                    border: Some(BorderConfig {
                        style: BorderStyle::Single,
                        color: Some(Color::rgb(187, 154, 247)),
                        sides: vec![BorderSide::Bottom],
                    }),
                    padding: (0, 0),
                    margin: (2, 1),
                    prefix: None,
                },
                h3: HeadingStyle {
                    color: Color::rgb(122, 162, 247),
                    ..Default::default()
                },
                h4: HeadingStyle {
                    color: Color::rgb(158, 206, 106),
                    ..Default::default()
                },
                h5: HeadingStyle {
                    color: Color::rgb(224, 175, 104),
                    ..Default::default()
                },
                h6: HeadingStyle {
                    color: Color::rgb(169, 177, 214),
                    ..Default::default()
                },
            },
            paragraph: ParagraphStyle::default(),
            code_block: CodeBlockStyle {
                background: Color::rgb(30, 32, 48),
                foreground: Color::rgb(169, 177, 214),
                border: Some(BorderConfig {
                    style: BorderStyle::Rounded,
                    color: Some(Color::rgb(86, 95, 137)),
                    sides: vec![BorderSide::Top, BorderSide::Right, BorderSide::Bottom, BorderSide::Left],
                }),
                padding: (1, 2),
                show_language_badge: true,
            },
            blockquote: BlockQuoteStyle {
                color: Color::rgb(169, 177, 214),
                background: None,
                border: Some(BorderConfig {
                    style: BorderStyle::Heavy,
                    color: Some(Color::rgb(187, 154, 247)),
                    sides: vec![BorderSide::Left],
                }),
                indent: 2,
            },
            list: ListStyle {
                marker_color: Color::rgb(125, 207, 255),
                indent: 2,
            },
            table: TableStyle {
                border_style: BorderStyle::Single,
                header_background: Some(Color::rgb(30, 32, 48)),
                header_foreground: Some(Color::rgb(125, 207, 255)),
                row_separator: true,
                padding: 1,
            },
            horizontal_rule: HorizontalRuleStyle {
                style: BorderStyle::Single,
                color: Color::rgb(86, 95, 137),
            },
            callout: Default::default(),
        },
        inlines: InlineStyles {
            strong: TextStyle {
                foreground: Some(Color::rgb(224, 175, 104)),
                background: None,
                weight: FontWeight::Bold,
                style: FontStyle::Normal,
            },
            emphasis: TextStyle {
                foreground: Some(Color::rgb(187, 154, 247)),
                background: Some(Color::rgb(35, 38, 52)),
                weight: FontWeight::Normal,
                style: FontStyle::Italic,
            },
            code: TextStyle {
                foreground: Some(Color::rgb(158, 206, 106)),
                background: Some(Color::rgb(30, 32, 48)),
                weight: FontWeight::Normal,
                style: FontStyle::Normal,
            },
            link: LinkStyle {
                foreground: Color::rgb(125, 207, 255),
                underline: true,
                show_url: UrlDisplayMode::Hover,
            },
            strikethrough: TextStyle {
                foreground: Some(Color::rgb(86, 95, 137)),
                background: None,
                weight: FontWeight::Normal,
                style: FontStyle::Normal,
            },
        },
    }
}

/// "Catppuccin" theme - Soothing pastel colors
pub fn catppuccin_theme() -> Theme {
    Theme {
        name: "Catppuccin".to_string(),
        version: "1.0".to_string(),
        colors: ColorPalette {
            foreground: Color::rgb(205, 214, 244),
            background: Color::rgb(30, 30, 46),
            primary: Color::rgb(137, 180, 250),
            secondary: Color::rgb(245, 194, 231),
            accent: Color::rgb(250, 179, 135),
            muted: Color::rgb(108, 112, 134),
            error: Color::rgb(243, 139, 168),
            warning: Color::rgb(249, 226, 175),
            success: Color::rgb(166, 227, 161),
        },
        typography: Typography {
            emphasis: EmphasisStyle::Native,
        },
        spacing: Spacing::default(),
        blocks: BlockStyles {
            heading: HeadingStyles {
                h1: HeadingStyle {
                    color: Color::rgb(137, 180, 250),
                    background: None,
                    border: Some(BorderConfig {
                        style: BorderStyle::Rounded,
                        color: Some(Color::rgb(137, 180, 250)),
                        sides: vec![BorderSide::Bottom],
                    }),
                    padding: (0, 0),
                    margin: (2, 1),
                    prefix: None,
                },
                h2: HeadingStyle {
                    color: Color::rgb(203, 166, 247),
                    background: None,
                    border: Some(BorderConfig {
                        style: BorderStyle::Rounded,
                        color: Some(Color::rgb(203, 166, 247)),
                        sides: vec![BorderSide::Bottom],
                    }),
                    padding: (0, 0),
                    margin: (2, 1),
                    prefix: None,
                },
                h3: HeadingStyle {
                    color: Color::rgb(245, 194, 231),
                    ..Default::default()
                },
                h4: HeadingStyle {
                    color: Color::rgb(166, 227, 161),
                    ..Default::default()
                },
                h5: HeadingStyle {
                    color: Color::rgb(250, 179, 135),
                    ..Default::default()
                },
                h6: HeadingStyle {
                    color: Color::rgb(205, 214, 244),
                    ..Default::default()
                },
            },
            paragraph: ParagraphStyle::default(),
            code_block: CodeBlockStyle {
                background: Color::rgb(49, 50, 68),
                foreground: Color::rgb(205, 214, 244),
                border: Some(BorderConfig {
                    style: BorderStyle::Rounded,
                    color: Some(Color::rgb(108, 112, 134)),
                    sides: vec![BorderSide::Top, BorderSide::Right, BorderSide::Bottom, BorderSide::Left],
                }),
                padding: (1, 2),
                show_language_badge: true,
            },
            blockquote: BlockQuoteStyle {
                color: Color::rgb(186, 194, 222),
                background: None,
                border: Some(BorderConfig {
                    style: BorderStyle::Heavy,
                    color: Some(Color::rgb(203, 166, 247)),
                    sides: vec![BorderSide::Left],
                }),
                indent: 2,
            },
            list: ListStyle {
                marker_color: Color::rgb(245, 194, 231),
                indent: 2,
            },
            table: TableStyle {
                border_style: BorderStyle::Rounded,
                header_background: Some(Color::rgb(49, 50, 68)),
                header_foreground: Some(Color::rgb(137, 180, 250)),
                row_separator: true,
                padding: 1,
            },
            horizontal_rule: HorizontalRuleStyle {
                style: BorderStyle::Single,
                color: Color::rgb(108, 112, 134),
            },
            callout: Default::default(),
        },
        inlines: InlineStyles {
            strong: TextStyle {
                foreground: Some(Color::rgb(250, 179, 135)),
                background: None,
                weight: FontWeight::Bold,
                style: FontStyle::Normal,
            },
            emphasis: TextStyle {
                foreground: Some(Color::rgb(245, 194, 231)),
                background: Some(Color::rgb(40, 41, 58)),
                weight: FontWeight::Normal,
                style: FontStyle::Italic,
            },
            code: TextStyle {
                foreground: Some(Color::rgb(166, 227, 161)),
                background: Some(Color::rgb(49, 50, 68)),
                weight: FontWeight::Normal,
                style: FontStyle::Normal,
            },
            link: LinkStyle {
                foreground: Color::rgb(137, 180, 250),
                underline: true,
                show_url: UrlDisplayMode::Hover,
            },
            strikethrough: TextStyle {
                foreground: Some(Color::rgb(108, 112, 134)),
                background: None,
                weight: FontWeight::Normal,
                style: FontStyle::Normal,
            },
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
