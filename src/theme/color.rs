//! Color types and terminal color handling

use serde::{Deserialize, Serialize};

/// Terminal color representation with fallback support
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    /// Reset to terminal default
    #[default]
    Reset,
    /// True color (24-bit RGB)
    #[serde(rename = "rgb")]
    Rgb(u8, u8, u8),
    /// 256-color palette index
    #[serde(rename = "ansi256")]
    Ansi256(u8),
    /// 16-color ANSI palette
    #[serde(rename = "ansi")]
    Ansi(AnsiColor),
}

/// Standard ANSI 16-color palette
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AnsiColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {
    /// Create a color from RGB values
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color::Rgb(r, g, b)
    }

    /// Convert RGB to approximate 256-color palette index
    pub fn to_ansi256(&self) -> u8 {
        match self {
            Color::Reset => 0,
            Color::Rgb(r, g, b) => {
                // Convert 24-bit RGB to 256-color palette (6x6x6 color cube + grayscale)
                if r == g && g == b {
                    // Grayscale (colors 232-255)
                    let gray = ((*r as u16 * 24) / 255) as u8;
                    232 + gray
                } else {
                    // Color cube (16-231): 16 + 36*r + 6*g + b
                    let r_idx = ((*r as u16 * 5) / 255) as u8;
                    let g_idx = ((*g as u16 * 5) / 255) as u8;
                    let b_idx = ((*b as u16 * 5) / 255) as u8;
                    16 + 36 * r_idx + 6 * g_idx + b_idx
                }
            }
            Color::Ansi256(idx) => *idx,
            Color::Ansi(ansi) => ansi.to_ansi256(),
        }
    }

    /// Convert to nearest ANSI 16-color
    pub fn to_ansi(&self) -> AnsiColor {
        match self {
            Color::Reset => AnsiColor::White,
            Color::Rgb(r, g, b) => {
                // Simple brightness-based mapping
                let brightness = (*r as u16 + *g as u16 + *b as u16) / 3;
                let is_bright = brightness > 128;

                let max_component = (*r).max(*g).max(*b);
                if max_component < 64 {
                    return if is_bright {
                        AnsiColor::BrightBlack
                    } else {
                        AnsiColor::Black
                    };
                }

                // Determine dominant color
                match (r, g, b) {
                    (r, g, b) if r == &max_component && *r > *g && *r > *b => {
                        if is_bright {
                            AnsiColor::BrightRed
                        } else {
                            AnsiColor::Red
                        }
                    }
                    (r, g, b) if g == &max_component && *g > *r && *g > *b => {
                        if is_bright {
                            AnsiColor::BrightGreen
                        } else {
                            AnsiColor::Green
                        }
                    }
                    (r, g, b) if b == &max_component && *b > *r && *b > *g => {
                        if is_bright {
                            AnsiColor::BrightBlue
                        } else {
                            AnsiColor::Blue
                        }
                    }
                    (r, g, b) if r == g && *r > *b => {
                        if is_bright {
                            AnsiColor::BrightYellow
                        } else {
                            AnsiColor::Yellow
                        }
                    }
                    (r, g, b) if r == b && *r > *g => {
                        if is_bright {
                            AnsiColor::BrightMagenta
                        } else {
                            AnsiColor::Magenta
                        }
                    }
                    (r, g, b) if g == b && *g > *r => {
                        if is_bright {
                            AnsiColor::BrightCyan
                        } else {
                            AnsiColor::Cyan
                        }
                    }
                    _ => {
                        if is_bright {
                            AnsiColor::BrightWhite
                        } else {
                            AnsiColor::White
                        }
                    }
                }
            }
            Color::Ansi256(idx) => {
                // Map 256-color to 16-color (simplified)
                AnsiColor::from_ansi256(*idx).unwrap_or(AnsiColor::White)
            }
            Color::Ansi(ansi) => *ansi,
        }
    }
}

impl AnsiColor {
    /// Convert ANSI color to its palette index (0-15)
    pub fn to_ansi256(&self) -> u8 {
        match self {
            AnsiColor::Black => 0,
            AnsiColor::Red => 1,
            AnsiColor::Green => 2,
            AnsiColor::Yellow => 3,
            AnsiColor::Blue => 4,
            AnsiColor::Magenta => 5,
            AnsiColor::Cyan => 6,
            AnsiColor::White => 7,
            AnsiColor::BrightBlack => 8,
            AnsiColor::BrightRed => 9,
            AnsiColor::BrightGreen => 10,
            AnsiColor::BrightYellow => 11,
            AnsiColor::BrightBlue => 12,
            AnsiColor::BrightMagenta => 13,
            AnsiColor::BrightCyan => 14,
            AnsiColor::BrightWhite => 15,
        }
    }

    /// Convert palette index (0-15) to ANSI color (safe alternative to transmute)
    pub fn from_ansi256(idx: u8) -> Option<Self> {
        match idx {
            0 => Some(AnsiColor::Black),
            1 => Some(AnsiColor::Red),
            2 => Some(AnsiColor::Green),
            3 => Some(AnsiColor::Yellow),
            4 => Some(AnsiColor::Blue),
            5 => Some(AnsiColor::Magenta),
            6 => Some(AnsiColor::Cyan),
            7 => Some(AnsiColor::White),
            8 => Some(AnsiColor::BrightBlack),
            9 => Some(AnsiColor::BrightRed),
            10 => Some(AnsiColor::BrightGreen),
            11 => Some(AnsiColor::BrightYellow),
            12 => Some(AnsiColor::BrightBlue),
            13 => Some(AnsiColor::BrightMagenta),
            14 => Some(AnsiColor::BrightCyan),
            15 => Some(AnsiColor::BrightWhite),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_to_ansi256() {
        let red = Color::rgb(255, 0, 0);
        let ansi256 = red.to_ansi256();
        assert!(ansi256 >= 16); // Should be in color cube range
    }

    #[test]
    fn test_rgb_to_ansi() {
        let red = Color::rgb(255, 0, 0);
        assert!(matches!(
            red.to_ansi(),
            AnsiColor::Red | AnsiColor::BrightRed
        ));

        let blue = Color::rgb(0, 0, 255);
        assert!(matches!(
            blue.to_ansi(),
            AnsiColor::Blue | AnsiColor::BrightBlue
        ));
    }

    #[test]
    fn test_ansi_color_conversion() {
        let red = AnsiColor::Red;
        assert_eq!(red.to_ansi256(), 1);

        let bright_cyan = AnsiColor::BrightCyan;
        assert_eq!(bright_cyan.to_ansi256(), 14);
    }
}
