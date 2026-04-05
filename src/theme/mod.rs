//! Theme system for Lumen
//!
//! Provides a CSS-like theming layer without full CSS complexity.
//! Themes are declarative token sets that map element types to styles.

pub mod color;
pub mod defaults;
pub mod types;
pub mod vim_import;

pub use color::{AnsiColor, Color};
pub use defaults::{
    catppuccin_theme, docs_theme, dracula_theme, gruvbox_theme, minimal_theme, monokai_theme,
    neon_theme, nord_theme, solarized_theme, tokyo_night_theme,
};
pub use types::*;

use std::io;
use std::path::PathBuf;

impl Theme {
    /// Load a theme from a YAML string
    pub fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }

    /// Load a theme from a YAML file, clamping spacing to safe values
    pub fn from_file(path: &str) -> Result<Self, io::Error> {
        let contents = std::fs::read_to_string(path)?;
        let mut theme = Self::from_yaml(&contents).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to parse theme: {}", e),
            )
        })?;
        theme.clamp_spacing();
        Ok(theme)
    }

    /// Serialize theme to YAML string
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }

    /// Get the user themes directory (~/.lumen/themes/)
    pub fn user_themes_dir() -> Option<PathBuf> {
        dirs::home_dir().map(|home| home.join(".lumen").join("themes"))
    }

    /// Load a user theme by name from ~/.lumen/themes/{name}.yaml
    pub fn user_theme(name: &str) -> Option<Self> {
        let dir = Self::user_themes_dir()?;
        let path = dir.join(format!("{}.yaml", name));
        if path.exists() {
            match Self::from_file(path.to_str()?) {
                Ok(theme) => Some(theme),
                Err(e) => {
                    eprintln!("Warning: Failed to load user theme '{}': {}", name, e);
                    None
                }
            }
        } else {
            None
        }
    }

    /// List user theme names from ~/.lumen/themes/
    pub fn user_theme_names() -> Vec<String> {
        let dir = match Self::user_themes_dir() {
            Some(d) => d,
            None => return Vec::new(),
        };
        if !dir.exists() {
            return Vec::new();
        }

        let mut names = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|ext| ext == "yaml" || ext == "yml") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        names.push(stem.to_string());
                    }
                }
            }
        }
        names.sort();
        names
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

    /// Load a theme by name — checks user themes first, then built-in
    pub fn load(name: &str) -> Option<Self> {
        // User themes take precedence (allows overriding built-in names)
        if let Some(theme) = Self::user_theme(name) {
            return Some(theme);
        }
        Self::builtin(name)
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

    /// List all available theme names (built-in + user)
    pub fn all_theme_names() -> Vec<String> {
        let mut names: Vec<String> = Self::builtin_names()
            .iter()
            .map(|s| s.to_string())
            .collect();
        for user_name in Self::user_theme_names() {
            if !names.contains(&user_name) {
                names.push(user_name);
            }
        }
        names
    }

    /// Save this theme to the user themes directory
    pub fn save_to_user_themes(&self, name: &str) -> io::Result<PathBuf> {
        let dir = Self::user_themes_dir().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "Could not determine home directory",
            )
        })?;
        std::fs::create_dir_all(&dir)?;

        let path = dir.join(format!("{}.yaml", name));
        let yaml = self.to_yaml().map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to serialize theme: {}", e),
            )
        })?;
        std::fs::write(&path, yaml)?;
        Ok(path)
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

    #[test]
    fn test_load_falls_back_to_builtin() {
        assert!(Theme::load("docs").is_some());
        assert!(Theme::load("nonexistent").is_none());
    }

    #[test]
    fn test_all_theme_names_includes_builtins() {
        let all = Theme::all_theme_names();
        assert!(all.contains(&"docs".to_string()));
        assert!(all.contains(&"neon".to_string()));
    }

    #[test]
    fn test_theme_validation() {
        let mut theme = docs_theme();
        assert!(theme.validate().is_empty());

        theme.spacing.paragraph_spacing = 100;
        let errors = theme.validate();
        assert!(!errors.is_empty());

        theme.clamp_spacing();
        assert!(theme.validate().is_empty());
        assert_eq!(theme.spacing.paragraph_spacing, 20);
    }
}
