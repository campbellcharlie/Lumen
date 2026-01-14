//! User preferences management

use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

/// User preferences stored in ~/.lumen/config.yaml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preferences {
    /// Last used theme name
    pub theme: String,

    /// Whether mouse mode was enabled
    #[serde(default)]
    pub mouse_enabled: bool,

    /// Whether file sidebar was visible
    #[serde(default = "default_true")]
    pub file_sidebar_visible: bool,
}

fn default_true() -> bool {
    true
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            theme: "docs".to_string(),
            mouse_enabled: false,
            file_sidebar_visible: true,
        }
    }
}

impl Preferences {
    /// Get the path to the preferences directory (~/.lumen)
    pub fn config_dir() -> Option<PathBuf> {
        dirs::home_dir().map(|home| home.join(".lumen"))
    }

    /// Get the path to the config file (~/.lumen/config.yaml)
    pub fn config_file_path() -> Option<PathBuf> {
        Self::config_dir().map(|dir| dir.join("config.yaml"))
    }

    /// Load preferences from disk, or return defaults if not found
    pub fn load() -> Self {
        match Self::config_file_path() {
            Some(path) => {
                if path.exists() {
                    match fs::read_to_string(&path) {
                        Ok(contents) => match serde_yaml::from_str(&contents) {
                            Ok(prefs) => return prefs,
                            Err(e) => {
                                eprintln!("Warning: Failed to parse preferences: {}", e);
                            }
                        },
                        Err(e) => {
                            eprintln!("Warning: Failed to read preferences: {}", e);
                        }
                    }
                }
            }
            None => {
                eprintln!("Warning: Could not determine home directory");
            }
        }

        // Return defaults if loading failed
        Self::default()
    }

    /// Save preferences to disk
    pub fn save(&self) -> io::Result<()> {
        let config_dir = Self::config_dir().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "Could not determine home directory",
            )
        })?;

        // Create .lumen directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }

        let config_path = config_dir.join("config.yaml");
        let yaml = serde_yaml::to_string(self).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to serialize preferences: {}", e),
            )
        })?;

        fs::write(&config_path, yaml)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_preferences() {
        let prefs = Preferences::default();
        assert_eq!(prefs.theme, "docs");
        assert!(!prefs.mouse_enabled);
        assert!(prefs.file_sidebar_visible);
    }

    #[test]
    fn test_serialization() {
        let prefs = Preferences {
            theme: "dracula".to_string(),
            mouse_enabled: true,
            file_sidebar_visible: false,
        };

        let yaml = serde_yaml::to_string(&prefs).unwrap();
        let deserialized: Preferences = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(deserialized.theme, "dracula");
        assert!(deserialized.mouse_enabled);
        assert!(!deserialized.file_sidebar_visible);
    }
}
