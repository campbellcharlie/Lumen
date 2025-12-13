use lumen::theme::Theme;
use std::fs;

fn main() {
    println!("=== Lumen Built-in Themes ===\n");

    // List all built-in themes
    let theme_names = Theme::builtin_names();
    println!("Available themes: {:?}\n", theme_names);

    // Export each theme to YAML
    for name in theme_names {
        if let Some(theme) = Theme::builtin(name) {
            println!("--- {} Theme ---", theme.name);
            println!("Version: {}", theme.version);

            // Serialize to YAML
            match theme.to_yaml() {
                Ok(yaml) => {
                    // Save to file
                    let filename = format!("themes/{}.yaml", name.to_lowercase());
                    if let Err(e) = fs::write(&filename, &yaml) {
                        eprintln!("Failed to write {}: {}", filename, e);
                    } else {
                        println!("Exported to: {}", filename);
                    }

                    // Show a snippet
                    let lines: Vec<_> = yaml.lines().take(15).collect();
                    println!("\nPreview:");
                    for line in lines {
                        println!("  {}", line);
                    }
                    if yaml.lines().count() > 15 {
                        println!("  ...");
                    }
                }
                Err(e) => {
                    eprintln!("Failed to serialize theme: {}", e);
                }
            }
            println!();
        }
    }

    println!("\n=== Testing Theme Loading ===\n");

    // Test loading from YAML
    for name in &["docs", "neon", "minimal"] {
        let filename = format!("themes/{}.yaml", name);
        match Theme::from_file(&filename) {
            Ok(theme) => {
                println!("✓ Successfully loaded {} from {}", theme.name, filename);
            }
            Err(e) => {
                eprintln!("✗ Failed to load {}: {}", filename, e);
            }
        }
    }
}
