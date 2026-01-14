//! Theme system for Lumen
//!
//! Provides a CSS-like theming layer without full CSS complexity.
//! Themes are declarative token sets that map element types to styles.

pub mod color;
pub mod defaults;
pub mod types;

pub use color::{AnsiColor, Color};
pub use defaults::{
    catppuccin_theme, docs_theme, dracula_theme, gruvbox_theme, minimal_theme, monokai_theme,
    neon_theme, nord_theme, solarized_theme, tokyo_night_theme,
};
pub use types::*;

use std::io;

impl Theme {
    /// Load a theme from a YAML string
    pub fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }

    /// Load a theme from a YAML file
    pub fn from_file(path: &str) -> Result<Self, io::Error> {
        let contents = std::fs::read_to_string(path)?;
        Self::from_yaml(&contents).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to parse theme: {}", e),
            )
        })
    }

    /// Serialize theme to YAML string
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }

    /// Get a built-in theme by name
    pub fn builtin(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "docs" => Some(docs_theme()),
            "neon" => Some(neon_theme()),
            "minimal" => Some(minimal_theme()),
            "dracula" => Some(dracula_theme()),
            "monokai" => Some(monokai_theme()),
            "solarized" => Some(solarized_theme()),
            "gruvbox" => Some(gruvbox_theme()),
            "nord" => Some(nord_theme()),
            "tokyo-night" | "tokyonight" => Some(tokyo_night_theme()),
            "catppuccin" => Some(catppuccin_theme()),
            _ => None,
        }
    }

    /// List all built-in theme names
    pub fn builtin_names() -> Vec<&'static str> {
        vec![
            "docs",
            "neon",
            "minimal",
            "dracula",
            "monokai",
            "solarized",
            "gruvbox",
            "nord",
            "tokyo-night",
            "catppuccin",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_themes() {
        assert!(Theme::builtin("docs").is_some());
        assert!(Theme::builtin("neon").is_some());
        assert!(Theme::builtin("minimal").is_some());
        assert!(Theme::builtin("dracula").is_some());
        assert!(Theme::builtin("monokai").is_some());
        assert!(Theme::builtin("solarized").is_some());
        assert!(Theme::builtin("gruvbox").is_some());
        assert!(Theme::builtin("nord").is_some());
        assert!(Theme::builtin("tokyo-night").is_some());
        assert!(Theme::builtin("catppuccin").is_some());
        assert!(Theme::builtin("nonexistent").is_none());
    }

    #[test]
    fn test_builtin_names() {
        let names = Theme::builtin_names();
        assert_eq!(names.len(), 10);
        assert!(names.contains(&"docs"));
        assert!(names.contains(&"neon"));
        assert!(names.contains(&"minimal"));
        assert!(names.contains(&"dracula"));
        assert!(names.contains(&"monokai"));
        assert!(names.contains(&"solarized"));
        assert!(names.contains(&"gruvbox"));
        assert!(names.contains(&"nord"));
        assert!(names.contains(&"tokyo-night"));
        assert!(names.contains(&"catppuccin"));
    }
}
