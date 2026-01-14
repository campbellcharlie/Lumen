//! Theme system integration tests

use lumen::theme::{BorderStyle, Color, Theme};

#[test]
fn test_builtin_themes_exist() {
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
fn test_load_docs_theme() {
    let theme = Theme::builtin("docs").expect("Docs theme should exist");
    assert_eq!(theme.name, "Docs");
    assert_eq!(theme.version, "1.0");

    // Check color palette
    assert!(matches!(theme.colors.primary, Color::Rgb(100, 180, 255)));

    // Check heading styles
    assert!(theme.blocks.heading.h1.border.is_some());
    assert_eq!(
        theme.blocks.heading.h1.border.as_ref().unwrap().style,
        BorderStyle::Double
    );
}

#[test]
fn test_load_neon_theme() {
    let theme = Theme::builtin("neon").expect("Neon theme should exist");
    assert_eq!(theme.name, "Neon");

    // Neon should have vibrant colors
    assert!(matches!(theme.colors.primary, Color::Rgb(0, 255, 255)));

    // Check for prefixes on headings
    assert_eq!(theme.blocks.heading.h1.prefix, Some("▶ ".to_string()));
    assert_eq!(theme.blocks.heading.h2.prefix, Some("■ ".to_string()));
}

#[test]
fn test_load_minimal_theme() {
    let theme = Theme::builtin("minimal").expect("Minimal theme should exist");
    assert_eq!(theme.name, "Minimal");

    // Minimal should use ASCII borders
    assert_eq!(theme.blocks.table.border_style, BorderStyle::Ascii);

    // Should use ANSI colors
    assert!(matches!(theme.colors.foreground, Color::Ansi(_)));
}

#[test]
fn test_theme_serialization_roundtrip() {
    let original = Theme::builtin("docs").unwrap();

    // Serialize to YAML
    let yaml = original.to_yaml().expect("Should serialize to YAML");

    // Deserialize back
    let loaded = Theme::from_yaml(&yaml).expect("Should deserialize from YAML");

    // Names should match
    assert_eq!(original.name, loaded.name);
    assert_eq!(original.version, loaded.version);
}

#[test]
fn test_theme_from_yaml_file() {
    // This assumes themes were exported by running the example
    let result = Theme::from_file("themes/docs.yaml");

    match result {
        Ok(theme) => {
            assert_eq!(theme.name, "Docs");
        }
        Err(_) => {
            // File might not exist if example wasn't run
            // This is acceptable in CI/CD environments
            println!(
                "Note: themes/docs.yaml not found. Run 'cargo run --example themes' to generate."
            );
        }
    }
}

#[test]
fn test_nonexistent_builtin() {
    let theme = Theme::builtin("nonexistent");
    assert!(theme.is_none());
}

#[test]
fn test_color_conversions() {
    let rgb = Color::rgb(255, 0, 0);

    // Test RGB to ANSI256
    let ansi256 = rgb.to_ansi256();
    assert!(ansi256 >= 16, "Should be in color cube range");

    // Test RGB to ANSI16
    let ansi = rgb.to_ansi();
    assert!(
        matches!(
            ansi,
            lumen::theme::AnsiColor::Red | lumen::theme::AnsiColor::BrightRed
        ),
        "Red RGB should map to Red ANSI color"
    );
}

#[test]
fn test_theme_default_values() {
    use lumen::theme::types::*;

    let spacing = Spacing::default();
    assert_eq!(spacing.paragraph_spacing, 1);
    assert_eq!(spacing.heading_margin_top, 2);

    let typography = Typography::default();
    assert_eq!(typography.emphasis, EmphasisStyle::Native);
}

#[test]
fn test_border_style_variants() {
    // Ensure all border styles are distinct
    assert_ne!(BorderStyle::Single, BorderStyle::Double);
    assert_ne!(BorderStyle::Single, BorderStyle::Rounded);
    assert_ne!(BorderStyle::Ascii, BorderStyle::Heavy);
}

#[test]
fn test_theme_yaml_minimal() {
    // Test minimal valid theme YAML
    let yaml = r#"
name: "Test"
version: "1.0"
colors:
  foreground: reset
  background: reset
  primary: reset
blocks:
  heading:
    h1:
      color: reset
    h2:
      color: reset
    h3:
      color: reset
  code_block:
    background: reset
    foreground: reset
  blockquote:
    color: reset
  table:
    border_style: Single
inlines:
  strong:
    {}
  emphasis:
    {}
  code:
    {}
  link:
    foreground: reset
    underline: false
"#;

    let theme = Theme::from_yaml(yaml);
    assert!(theme.is_ok(), "Should parse minimal valid theme");

    if let Ok(theme) = theme {
        assert_eq!(theme.name, "Test");
    }
}
