//! Theme type definitions

use super::color::Color;
use serde::{Deserialize, Serialize};

/// Complete theme definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Theme name
    pub name: String,
    /// Theme version
    #[serde(default = "default_version")]
    pub version: String,
    /// Color palette
    pub colors: ColorPalette,
    /// Typography settings
    #[serde(default)]
    pub typography: Typography,
    /// Spacing settings
    #[serde(default)]
    pub spacing: Spacing,
    /// Block-level element styles
    pub blocks: BlockStyles,
    /// Inline element styles
    pub inlines: InlineStyles,
}

fn default_version() -> String {
    "1.0".to_string()
}

/// Core color palette
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    pub foreground: Color,
    pub background: Color,
    pub primary: Color,
    #[serde(default = "Color::reset")]
    pub secondary: Color,
    #[serde(default = "Color::reset")]
    pub accent: Color,
    #[serde(default = "Color::reset")]
    pub muted: Color,
    #[serde(default = "Color::reset")]
    pub error: Color,
    #[serde(default = "Color::reset")]
    pub warning: Color,
    #[serde(default = "Color::reset")]
    pub success: Color,
}

impl Color {
    fn reset() -> Self {
        Color::Reset
    }
}

/// Typography configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Typography {
    /// Emphasis rendering strategy
    #[serde(default)]
    pub emphasis: EmphasisStyle,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            emphasis: EmphasisStyle::Native,
        }
    }
}

/// How to render emphasis (italic, bold, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum EmphasisStyle {
    /// Use terminal's native bold/italic
    #[default]
    Native,
    /// Use color shifts
    ColorShift,
    /// Use background highlighting
    BackgroundBand,
}

/// Spacing configuration (in lines/columns)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spacing {
    #[serde(default = "default_paragraph_spacing")]
    pub paragraph_spacing: u16,
    #[serde(default = "default_heading_margin_top")]
    pub heading_margin_top: u16,
    #[serde(default = "default_heading_margin_bottom")]
    pub heading_margin_bottom: u16,
    #[serde(default = "default_list_indent")]
    pub list_indent: u16,
    #[serde(default = "default_blockquote_indent")]
    pub blockquote_indent: u16,
    #[serde(default = "default_code_block_padding")]
    pub code_block_padding: u16,
}

fn default_paragraph_spacing() -> u16 {
    1
}
fn default_heading_margin_top() -> u16 {
    2
}
fn default_heading_margin_bottom() -> u16 {
    1
}
fn default_list_indent() -> u16 {
    2
}
fn default_blockquote_indent() -> u16 {
    2
}
fn default_code_block_padding() -> u16 {
    1
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            paragraph_spacing: default_paragraph_spacing(),
            heading_margin_top: default_heading_margin_top(),
            heading_margin_bottom: default_heading_margin_bottom(),
            list_indent: default_list_indent(),
            blockquote_indent: default_blockquote_indent(),
            code_block_padding: default_code_block_padding(),
        }
    }
}

/// Block-level element styles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockStyles {
    pub heading: HeadingStyles,
    #[serde(default)]
    pub paragraph: ParagraphStyle,
    pub code_block: CodeBlockStyle,
    pub blockquote: BlockQuoteStyle,
    #[serde(default)]
    pub list: ListStyle,
    pub table: TableStyle,
    #[serde(default)]
    pub horizontal_rule: HorizontalRuleStyle,
    #[serde(default)]
    pub callout: CalloutStyles,
}

/// Styles for all heading levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingStyles {
    pub h1: HeadingStyle,
    pub h2: HeadingStyle,
    pub h3: HeadingStyle,
    #[serde(default)]
    pub h4: HeadingStyle,
    #[serde(default)]
    pub h5: HeadingStyle,
    #[serde(default)]
    pub h6: HeadingStyle,
}

/// Individual heading style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingStyle {
    pub color: Color,
    #[serde(default)]
    pub background: Option<Color>,
    #[serde(default)]
    pub border: Option<BorderConfig>,
    #[serde(default)]
    pub padding: (u16, u16), // (vertical, horizontal)
    #[serde(default)]
    pub margin: (u16, u16), // (top, bottom)
    #[serde(default)]
    pub prefix: Option<String>,
}

impl Default for HeadingStyle {
    fn default() -> Self {
        Self {
            color: Color::Reset,
            background: None,
            border: None,
            padding: (0, 0),
            margin: (1, 1),
            prefix: None,
        }
    }
}

/// Paragraph style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParagraphStyle {
    #[serde(default)]
    pub color: Color,
    #[serde(default)]
    pub margin: (u16, u16),
}

impl Default for ParagraphStyle {
    fn default() -> Self {
        Self {
            color: Color::Reset,
            margin: (0, 1),
        }
    }
}

/// Code block style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeBlockStyle {
    pub background: Color,
    pub foreground: Color,
    #[serde(default)]
    pub border: Option<BorderConfig>,
    #[serde(default)]
    pub padding: (u16, u16),
    #[serde(default = "default_true")]
    pub show_language_badge: bool,
}

fn default_true() -> bool {
    true
}

/// Block quote style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockQuoteStyle {
    pub color: Color,
    #[serde(default)]
    pub background: Option<Color>,
    #[serde(default)]
    pub border: Option<BorderConfig>,
    #[serde(default = "default_blockquote_indent")]
    pub indent: u16,
}

/// Callout/admonition styles for different types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalloutStyles {
    pub note: CalloutStyle,
    pub warning: CalloutStyle,
    pub important: CalloutStyle,
    pub tip: CalloutStyle,
    pub caution: CalloutStyle,
}

/// Individual callout style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalloutStyle {
    pub icon: String,
    pub color: Color,
    #[serde(default)]
    pub background: Option<Color>,
    #[serde(default)]
    pub border_color: Color,
}

impl Default for CalloutStyles {
    fn default() -> Self {
        Self {
            note: CalloutStyle {
                icon: "ℹ".to_string(),
                color: Color::Ansi256(39),            // Blue
                background: Some(Color::Ansi256(17)), // Dark blue bg
                border_color: Color::Ansi256(39),
            },
            warning: CalloutStyle {
                icon: "⚠".to_string(),
                color: Color::Ansi256(220),           // Yellow
                background: Some(Color::Ansi256(58)), // Dark yellow bg
                border_color: Color::Ansi256(220),
            },
            important: CalloutStyle {
                icon: "❗".to_string(),
                color: Color::Ansi256(201),           // Magenta
                background: Some(Color::Ansi256(53)), // Dark magenta bg
                border_color: Color::Ansi256(201),
            },
            tip: CalloutStyle {
                icon: "💡".to_string(),
                color: Color::Ansi256(42),            // Green
                background: Some(Color::Ansi256(22)), // Dark green bg
                border_color: Color::Ansi256(42),
            },
            caution: CalloutStyle {
                icon: "🔥".to_string(),
                color: Color::Ansi256(202),           // Orange/red
                background: Some(Color::Ansi256(52)), // Dark red bg
                border_color: Color::Ansi256(202),
            },
        }
    }
}

/// List style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListStyle {
    #[serde(default)]
    pub marker_color: Color,
    #[serde(default = "default_list_indent")]
    pub indent: u16,
}

impl Default for ListStyle {
    fn default() -> Self {
        Self {
            marker_color: Color::Reset,
            indent: default_list_indent(),
        }
    }
}

/// Table style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableStyle {
    #[serde(default)]
    pub border_style: BorderStyle,
    #[serde(default)]
    pub header_background: Option<Color>,
    #[serde(default)]
    pub header_foreground: Option<Color>,
    #[serde(default)]
    pub row_separator: bool,
    #[serde(default)]
    pub padding: u16,
}

/// Horizontal rule style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HorizontalRuleStyle {
    #[serde(default)]
    pub style: BorderStyle,
    #[serde(default)]
    pub color: Color,
}

impl Default for HorizontalRuleStyle {
    fn default() -> Self {
        Self {
            style: BorderStyle::Single,
            color: Color::Reset,
        }
    }
}

/// Border configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderConfig {
    pub style: BorderStyle,
    #[serde(default)]
    pub color: Option<Color>,
    #[serde(default = "default_all_sides")]
    pub sides: Vec<BorderSide>,
}

fn default_all_sides() -> Vec<BorderSide> {
    vec![
        BorderSide::Top,
        BorderSide::Right,
        BorderSide::Bottom,
        BorderSide::Left,
    ]
}

/// Border drawing style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BorderStyle {
    None,
    #[default]
    Single, // ┌─┐ │ └─┘
    Double,  // ╔═╗ ║ ╚═╝
    Rounded, // ╭─╮ │ ╰─╯
    Heavy,   // ┏━┓ ┃ ┗━┛
    Ascii,   // +--+ | +--+
}

/// Which sides of a border to draw
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BorderSide {
    Top,
    Right,
    Bottom,
    Left,
}

/// Inline element styles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineStyles {
    pub strong: TextStyle,
    pub emphasis: TextStyle,
    pub code: TextStyle,
    pub link: LinkStyle,
    #[serde(default)]
    pub strikethrough: TextStyle,
}

/// Text styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStyle {
    #[serde(default)]
    pub foreground: Option<Color>,
    #[serde(default)]
    pub background: Option<Color>,
    #[serde(default)]
    pub weight: FontWeight,
    #[serde(default)]
    pub style: FontStyle,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            foreground: None,
            background: None,
            weight: FontWeight::Normal,
            style: FontStyle::Normal,
        }
    }
}

/// Font weight
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FontWeight {
    #[default]
    Normal,
    Bold,
}

/// Font style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FontStyle {
    #[default]
    Normal,
    Italic,
}

/// Link styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkStyle {
    pub foreground: Color,
    #[serde(default)]
    pub underline: bool,
    #[serde(default)]
    pub show_url: UrlDisplayMode,
}

/// How to display link URLs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum UrlDisplayMode {
    Inline, // Show URL after text
    #[default]
    Hover, // Show on hover (if terminal supports)
    Hidden, // Don't show URL
}
