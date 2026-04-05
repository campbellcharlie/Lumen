//! Import vim/neovim colorschemes and convert them to Lumen themes.
//!
//! Supports:
//! - Local .vim colorscheme files
//! - Direct URLs to .vim files
//! - GitHub repository URLs (auto-discovers colors/*.vim)
//! - vimcolorschemes.com URLs (extracts GitHub repo)

use super::color::Color;
use super::types::*;
use std::collections::HashMap;
use std::io;
use std::path::Path;

/// Parsed highlight group from a vim colorscheme
#[derive(Debug, Clone, Default)]
struct HiGroup {
    guifg: Option<String>,
    guibg: Option<String>,
    gui: Option<String>,
}

/// All extracted highlight groups from a vim colorscheme
#[derive(Debug, Default)]
struct VimColors {
    groups: HashMap<String, HiGroup>,
    name: Option<String>,
    background: Option<String>, // "dark" or "light"
}

/// Import a vim colorscheme from a file path
pub fn import_from_file(path: &str) -> io::Result<Theme> {
    let contents = std::fs::read_to_string(path)?;
    let name = Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("imported")
        .to_string();

    let colors = parse_vim_colorscheme(&contents);
    Ok(build_theme(&colors, &name))
}

/// Import a vim colorscheme from a URL.
/// Supports:
/// - Direct .vim file URLs
/// - GitHub repo URLs (e.g., https://github.com/owner/repo)
/// - vimcolorschemes.com URLs (e.g., https://vimcolorschemes.com/owner/repo)
pub fn import_from_url(url: &str) -> Result<Theme, String> {
    // Normalize URL and determine type
    let (vim_content, name) = if url.ends_with(".vim") {
        // Direct .vim file URL
        let content = fetch_url(url)?;
        let name = url
            .rsplit('/')
            .next()
            .unwrap_or("imported")
            .trim_end_matches(".vim")
            .to_string();
        (content, name)
    } else {
        // Try to extract owner/repo from GitHub or vimcolorschemes URL
        let (owner, repo) = parse_repo_url(url)?;
        fetch_colorscheme_from_repo(&owner, &repo)?
    };

    let colors = parse_vim_colorscheme(&vim_content);
    Ok(build_theme(&colors, &name))
}

/// Fetch a URL and return its body as a string
fn fetch_url(url: &str) -> Result<String, String> {
    ureq::get(url)
        .call()
        .map_err(|e| format!("HTTP request failed: {}", e))?
        .into_string()
        .map_err(|e| format!("Failed to read response body: {}", e))
}

/// Extract owner/repo from a GitHub or vimcolorschemes URL
fn parse_repo_url(url: &str) -> Result<(String, String), String> {
    let url = url.trim_end_matches('/');

    // Strip protocol and host
    let path = if let Some(rest) = url.strip_prefix("https://vimcolorschemes.com/") {
        rest
    } else if let Some(rest) = url.strip_prefix("https://github.com/") {
        rest
    } else if let Some(rest) = url.strip_prefix("http://github.com/") {
        rest
    } else {
        return Err(
            "Unrecognized URL format. Expected a GitHub or vimcolorschemes.com URL.\n\
             Supported formats:\n  \
             https://github.com/owner/repo\n  \
             https://vimcolorschemes.com/owner/repo\n  \
             https://raw.githubusercontent.com/owner/repo/branch/colors/name.vim"
                .to_string(),
        );
    };

    // Remove tree/branch/path suffix if present (e.g., /tree/main/colors)
    let path = path.split("/tree/").next().unwrap_or(path);
    let path = path.split("/blob/").next().unwrap_or(path);

    let parts: Vec<&str> = path.splitn(3, '/').collect();
    if parts.len() < 2 {
        return Err(format!("Could not extract owner/repo from URL: {}", url));
    }

    Ok((parts[0].to_string(), parts[1].to_string()))
}

/// Fetch a colorscheme from a GitHub repository.
/// Searches: colors/, extras/vim/, extras/, then tries common name patterns.
fn fetch_colorscheme_from_repo(owner: &str, repo: &str) -> Result<(String, String), String> {
    // Directories to search for .vim colorscheme files
    let search_dirs = ["colors", "extras/vim", "extras"];

    for dir in &search_dirs {
        let api_url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            owner, repo, dir
        );

        let vim_files = match fetch_url(&api_url) {
            Ok(json) => find_vim_files_in_json(&json),
            Err(_) => Vec::new(),
        };

        if let Some(vim_file) = vim_files.first() {
            let raw_url = format!(
                "https://raw.githubusercontent.com/{}/{}/HEAD/{}/{}",
                owner, repo, dir, vim_file
            );
            let content = fetch_url(&raw_url)?;
            let name = vim_file.trim_end_matches(".vim").to_string();
            return Ok((content, name));
        }
    }

    // Fallback: try common name patterns in colors/
    let base_name = repo
        .trim_end_matches(".nvim")
        .trim_end_matches(".vim");
    let common_names = [
        format!("{}.vim", base_name),
        format!("{}.vim", base_name.replace('-', "_")),
        format!("{}.vim", base_name.replace('_', "-")),
    ];

    for name in &common_names {
        let raw_url = format!(
            "https://raw.githubusercontent.com/{}/{}/HEAD/colors/{}",
            owner, repo, name
        );
        if let Ok(content) = fetch_url(&raw_url) {
            let theme_name = name.trim_end_matches(".vim").to_string();
            return Ok((content, theme_name));
        }
    }

    // Check if this is a Lua-only theme (modern neovim)
    let api_url = format!(
        "https://api.github.com/repos/{}/{}/contents/colors",
        owner, repo
    );
    let has_lua = fetch_url(&api_url)
        .map(|json| json.contains(".lua"))
        .unwrap_or(false);

    if has_lua {
        Err(format!(
            "This appears to be a Lua-only neovim theme ({}/{}).\n\
             Lumen can only import classic .vim colorscheme files.\n\
             Check if the repo has a 'extras/' directory with generated .vim files,\n\
             or provide a direct URL to a .vim file.",
            owner, repo
        ))
    } else {
        Err(format!(
            "Could not find a .vim colorscheme file in {}/{}.\n\
             Try providing a direct URL to the .vim file instead.",
            owner, repo
        ))
    }
}

/// Parse the GitHub API JSON response to find .vim files
fn find_vim_files_in_json(json: &str) -> Vec<String> {
    // Simple JSON parsing — look for "name": "something.vim" patterns
    // Avoids adding a full JSON parsing dependency
    let mut files = Vec::new();
    for line in json.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("\"name\":") {
            let rest = rest.trim().trim_start_matches('"');
            if let Some(name) = rest.split('"').next() {
                if name.ends_with(".vim") {
                    files.push(name.to_string());
                }
            }
        }
    }

    // Also try the compact JSON format
    if files.is_empty() {
        let mut idx = 0;
        let bytes = json.as_bytes();
        while idx < bytes.len() {
            if let Some(pos) = json[idx..].find("\"name\"") {
                let start = idx + pos;
                // Find the value after the colon
                if let Some(colon) = json[start..].find(':') {
                    let after_colon = &json[start + colon + 1..];
                    let after_colon = after_colon.trim();
                    if let Some(inner) = after_colon.strip_prefix('"') {
                        if let Some(end_quote) = inner.find('"') {
                            let name = &inner[..end_quote];
                            if name.ends_with(".vim") {
                                files.push(name.to_string());
                            }
                        }
                    }
                }
                idx = start + 6; // skip past "name"
            } else {
                break;
            }
        }
    }

    files
}

/// Parse a vim colorscheme file and extract highlight groups
fn parse_vim_colorscheme(content: &str) -> VimColors {
    let mut colors = VimColors::default();

    for line in content.lines() {
        let line = line.trim();

        // Skip comments and empty lines
        if line.starts_with('"') || line.is_empty() {
            continue;
        }

        // Extract colorscheme name: let g:colors_name = "name"
        if line.contains("colors_name") {
            if let Some(name) = extract_quoted_value(line) {
                colors.name = Some(name);
            }
        }

        // Extract background setting: set background=dark
        if line.contains("background=") {
            if line.contains("dark") {
                colors.background = Some("dark".to_string());
            } else if line.contains("light") {
                colors.background = Some("light".to_string());
            }
        }

        // Parse hi[ghlight] commands
        // Format: hi[!] GroupName key=value ...
        // Also: highlight[!] GroupName key=value ...
        let line_lower = line.to_lowercase();
        let is_hi = line_lower.starts_with("hi ") || line_lower.starts_with("hi! ");
        let is_highlight = line_lower.starts_with("highlight ") || line_lower.starts_with("highlight! ");

        if is_hi || is_highlight {
            // Skip "hi link" and "hi clear" commands
            if line_lower.contains(" link ") || line_lower.contains(" clear") {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                continue;
            }

            // Find the group name (skip "hi" or "hi!" or "highlight" etc.)
            let group_name = parts[1];

            let mut group = HiGroup::default();

            for part in &parts[2..] {
                if let Some((key, value)) = part.split_once('=') {
                    match key.to_lowercase().as_str() {
                        "guifg" => group.guifg = Some(value.to_string()),
                        "guibg" => group.guibg = Some(value.to_string()),
                        "gui" => group.gui = Some(value.to_string()),
                        _ => {}
                    }
                }
            }

            // Only store if we got at least one useful attribute
            if group.guifg.is_some() || group.guibg.is_some() {
                colors.groups.insert(group_name.to_string(), group);
            }
        }
    }

    colors
}

/// Extract a quoted string value from a vim assignment line
fn extract_quoted_value(line: &str) -> Option<String> {
    // Match: = "value" or = 'value'
    let after_eq = line.split('=').nth(1)?.trim();
    let after_eq = after_eq.trim_start_matches(['\'', '"']);
    let end = after_eq.find(['\'', '"']).unwrap_or(after_eq.len());
    Some(after_eq[..end].to_string())
}

/// Convert a hex color string (#rrggbb) to a Lumen Color
fn hex_to_color(hex: &str) -> Option<Color> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(Color::Rgb(r, g, b))
}

/// Try to get a color from a highlight group's foreground
fn fg_color(colors: &VimColors, group: &str) -> Option<Color> {
    colors
        .groups
        .get(group)
        .and_then(|g| g.guifg.as_ref())
        .filter(|c| c.to_uppercase() != "NONE")
        .and_then(|c| hex_to_color(c))
}

/// Try to get a color from a highlight group's background
fn bg_color(colors: &VimColors, group: &str) -> Option<Color> {
    colors
        .groups
        .get(group)
        .and_then(|g| g.guibg.as_ref())
        .filter(|c| c.to_uppercase() != "NONE")
        .and_then(|c| hex_to_color(c))
}

/// Darken a color by a factor (0.0 = black, 1.0 = unchanged)
fn darken(color: Color, factor: f32) -> Color {
    match color {
        Color::Rgb(r, g, b) => Color::Rgb(
            (r as f32 * factor) as u8,
            (g as f32 * factor) as u8,
            (b as f32 * factor) as u8,
        ),
        other => other,
    }
}

/// Blend a color towards another by a factor (0.0 = original, 1.0 = target)
fn blend(color: Color, target: Color, factor: f32) -> Color {
    match (color, target) {
        (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => Color::Rgb(
            (r1 as f32 + (r2 as f32 - r1 as f32) * factor) as u8,
            (g1 as f32 + (g2 as f32 - g1 as f32) * factor) as u8,
            (b1 as f32 + (b2 as f32 - b1 as f32) * factor) as u8,
        ),
        (original, _) => original,
    }
}

/// Build a Lumen theme from parsed vim colorscheme data
fn build_theme(colors: &VimColors, fallback_name: &str) -> Theme {
    let name = colors
        .name
        .as_deref()
        .unwrap_or(fallback_name)
        .to_string();

    // Extract base colors with sensible defaults
    let fg = fg_color(colors, "Normal").unwrap_or(Color::Rgb(200, 200, 200));
    let bg = bg_color(colors, "Normal").unwrap_or(Color::Rgb(30, 30, 30));

    let primary = fg_color(colors, "Title")
        .or_else(|| fg_color(colors, "Function"))
        .or_else(|| fg_color(colors, "Keyword"))
        .unwrap_or(Color::Rgb(100, 150, 255));

    let secondary = fg_color(colors, "Type")
        .or_else(|| fg_color(colors, "Identifier"))
        .unwrap_or(Color::Rgb(150, 200, 255));

    let accent = fg_color(colors, "Special")
        .or_else(|| fg_color(colors, "PreProc"))
        .unwrap_or(Color::Rgb(200, 150, 255));

    let muted = fg_color(colors, "Comment")
        .or_else(|| fg_color(colors, "LineNr"))
        .unwrap_or(Color::Rgb(100, 100, 100));

    let string_color = fg_color(colors, "String")
        .or_else(|| fg_color(colors, "Constant"))
        .unwrap_or(Color::Rgb(150, 200, 100));

    let keyword_color = fg_color(colors, "Keyword")
        .or_else(|| fg_color(colors, "Statement"))
        .unwrap_or(primary);

    let function_color = fg_color(colors, "Function")
        .or_else(|| fg_color(colors, "Identifier"))
        .unwrap_or(primary);

    let error_color = fg_color(colors, "Error")
        .or_else(|| fg_color(colors, "ErrorMsg"))
        .unwrap_or(Color::Rgb(220, 50, 50));

    let warning_color = fg_color(colors, "WarningMsg")
        .or_else(|| fg_color(colors, "Todo"))
        .unwrap_or(Color::Rgb(220, 180, 50));

    let success_color = fg_color(colors, "DiffAdd")
        .or_else(|| fg_color(colors, "String"))
        .unwrap_or(Color::Rgb(50, 200, 100));

    let link_color = fg_color(colors, "Underlined")
        .or_else(|| fg_color(colors, "Function"))
        .unwrap_or(function_color);

    let code_bg = bg_color(colors, "CursorLine")
        .or_else(|| bg_color(colors, "ColorColumn"))
        .unwrap_or_else(|| darken(bg, 0.8));

    let _visual_bg = bg_color(colors, "Visual").unwrap_or(Color::Rgb(60, 60, 100));

    // Build heading styles using a hierarchy of vim groups
    let h1_color = fg_color(colors, "Title").unwrap_or(primary);
    let h2_color = keyword_color;
    let h3_color = function_color;
    let h4_color = fg_color(colors, "Type").unwrap_or(secondary);

    // Build callout border colors (subtle versions of the accent colors)
    let note_border = blend(primary, bg, 0.3);
    let warning_border = blend(warning_color, bg, 0.3);
    let error_border = blend(error_color, bg, 0.3);
    let success_border = blend(success_color, bg, 0.3);
    let important_border = blend(accent, bg, 0.3);

    Theme {
        name,
        version: "1.0".to_string(),
        colors: ColorPalette {
            foreground: fg,
            background: bg,
            primary,
            secondary,
            accent,
            muted,
            error: error_color,
            warning: warning_color,
            success: success_color,
        },
        typography: Typography::default(),
        spacing: Spacing::default(),
        blocks: BlockStyles {
            heading: HeadingStyles {
                h1: HeadingStyle {
                    color: h1_color,
                    background: Some(blend(h1_color, bg, 0.85)),
                    border: Some(BorderConfig {
                        style: BorderStyle::Heavy,
                        color: Some(h1_color),
                        sides: vec![BorderSide::Bottom],
                    }),
                    padding: (0, 1),
                    margin: (2, 1),
                    prefix: Some("# ".to_string()),
                },
                h2: HeadingStyle {
                    color: h2_color,
                    background: None,
                    border: Some(BorderConfig {
                        style: BorderStyle::Single,
                        color: Some(h2_color),
                        sides: vec![BorderSide::Bottom],
                    }),
                    padding: (0, 0),
                    margin: (2, 1),
                    prefix: Some("## ".to_string()),
                },
                h3: HeadingStyle {
                    color: h3_color,
                    background: None,
                    border: None,
                    padding: (0, 0),
                    margin: (1, 1),
                    prefix: Some("### ".to_string()),
                },
                h4: HeadingStyle {
                    color: h4_color,
                    background: None,
                    border: None,
                    padding: (0, 0),
                    margin: (1, 1),
                    prefix: None,
                },
                h5: HeadingStyle::default(),
                h6: HeadingStyle::default(),
            },
            paragraph: ParagraphStyle {
                color: fg,
                margin: (0, 1),
            },
            code_block: CodeBlockStyle {
                background: code_bg,
                foreground: fg,
                border: Some(BorderConfig {
                    style: BorderStyle::Rounded,
                    color: Some(muted),
                    sides: vec![
                        BorderSide::Top,
                        BorderSide::Right,
                        BorderSide::Bottom,
                        BorderSide::Left,
                    ],
                }),
                padding: (1, 1),
                show_language_badge: true,
            },
            blockquote: BlockQuoteStyle {
                color: muted,
                background: Some(blend(muted, bg, 0.9)),
                border: Some(BorderConfig {
                    style: BorderStyle::Heavy,
                    color: Some(muted),
                    sides: vec![BorderSide::Left],
                }),
                indent: 2,
            },
            list: ListStyle {
                marker_color: accent,
                indent: 2,
            },
            table: TableStyle {
                border_style: BorderStyle::Single,
                header_background: Some(blend(primary, bg, 0.85)),
                header_foreground: Some(primary),
                row_separator: false,
                padding: 1,
            },
            horizontal_rule: HorizontalRuleStyle {
                style: BorderStyle::Single,
                color: muted,
            },
            callout: CalloutStyles {
                note: CalloutStyle {
                    icon: "\u{2139}".to_string(), // ℹ
                    color: primary,
                    background: Some(blend(primary, bg, 0.9)),
                    border_color: note_border,
                },
                warning: CalloutStyle {
                    icon: "\u{26a0}".to_string(), // ⚠
                    color: warning_color,
                    background: Some(blend(warning_color, bg, 0.9)),
                    border_color: warning_border,
                },
                important: CalloutStyle {
                    icon: "\u{2757}".to_string(), // ❗
                    color: accent,
                    background: Some(blend(accent, bg, 0.9)),
                    border_color: important_border,
                },
                tip: CalloutStyle {
                    icon: "\u{1f4a1}".to_string(), // 💡
                    color: success_color,
                    background: Some(blend(success_color, bg, 0.9)),
                    border_color: success_border,
                },
                caution: CalloutStyle {
                    icon: "\u{1f525}".to_string(), // 🔥
                    color: error_color,
                    background: Some(blend(error_color, bg, 0.9)),
                    border_color: error_border,
                },
            },
        },
        inlines: InlineStyles {
            strong: TextStyle {
                foreground: Some(fg),
                background: None,
                weight: FontWeight::Bold,
                style: FontStyle::Normal,
            },
            emphasis: TextStyle {
                foreground: Some(accent),
                background: None,
                weight: FontWeight::Normal,
                style: FontStyle::Italic,
            },
            code: TextStyle {
                foreground: Some(string_color),
                background: Some(code_bg),
                weight: FontWeight::Normal,
                style: FontStyle::Normal,
            },
            link: LinkStyle {
                foreground: link_color,
                underline: true,
                show_url: UrlDisplayMode::Hidden,
            },
            strikethrough: TextStyle {
                foreground: Some(muted),
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
    fn test_parse_basic_vim_colorscheme() {
        let vim = r#"
" Test colorscheme
let g:colors_name = "testscheme"
set background=dark

hi Normal       guifg=#c0caf5 guibg=#1a1b26
hi Comment      guifg=#565f89 gui=italic
hi String       guifg=#9ece6a
hi Function     guifg=#7aa2f7
hi Keyword      guifg=#bb9af7
hi Error        guifg=#db4b4b
"#;
        let colors = parse_vim_colorscheme(vim);
        assert_eq!(colors.name, Some("testscheme".to_string()));
        assert_eq!(colors.background, Some("dark".to_string()));
        assert!(colors.groups.contains_key("Normal"));
        assert!(colors.groups.contains_key("Comment"));

        let normal = &colors.groups["Normal"];
        assert_eq!(normal.guifg, Some("#c0caf5".to_string()));
        assert_eq!(normal.guibg, Some("#1a1b26".to_string()));
    }

    #[test]
    fn test_build_theme_from_vim() {
        let vim = r#"
let g:colors_name = "mytest"
set background=dark
hi Normal guifg=#d4d4d4 guibg=#1e1e1e
hi Comment guifg=#6a9955
hi String guifg=#ce9178
hi Function guifg=#dcdcaa
hi Keyword guifg=#569cd6
hi Title guifg=#4ec9b0 gui=bold
hi Error guifg=#f44747
hi Underlined guifg=#569cd6
hi CursorLine guibg=#2a2a2a
hi Visual guibg=#264f78
"#;
        let colors = parse_vim_colorscheme(vim);
        let theme = build_theme(&colors, "mytest");

        assert_eq!(theme.name, "mytest");
        // Verify colors were mapped
        assert!(matches!(theme.colors.foreground, Color::Rgb(0xd4, 0xd4, 0xd4)));
        assert!(matches!(theme.colors.background, Color::Rgb(0x1e, 0x1e, 0x1e)));
    }

    #[test]
    fn test_hex_to_color() {
        assert!(matches!(hex_to_color("#ff0000"), Some(Color::Rgb(255, 0, 0))));
        assert!(matches!(hex_to_color("#00ff00"), Some(Color::Rgb(0, 255, 0))));
        assert_eq!(hex_to_color("invalid"), None);
        assert_eq!(hex_to_color("#fff"), None); // Too short
    }

    #[test]
    fn test_parse_repo_url() {
        let (owner, repo) = parse_repo_url("https://github.com/folke/tokyonight.nvim").unwrap();
        assert_eq!(owner, "folke");
        assert_eq!(repo, "tokyonight.nvim");

        let (owner, repo) =
            parse_repo_url("https://vimcolorschemes.com/catppuccin/nvim").unwrap();
        assert_eq!(owner, "catppuccin");
        assert_eq!(repo, "nvim");

        assert!(parse_repo_url("https://example.com/whatever").is_err());
    }

    #[test]
    fn test_parse_highlight_bang() {
        let vim = "hi! Normal guifg=#ffffff guibg=#000000\n";
        let colors = parse_vim_colorscheme(vim);
        assert!(colors.groups.contains_key("Normal"));
    }

    #[test]
    fn test_skip_hi_link_and_clear() {
        let vim = r#"
hi clear
hi link Special Function
hi Normal guifg=#ffffff guibg=#000000
"#;
        let colors = parse_vim_colorscheme(vim);
        // Should only have Normal, not "clear" or "Special"
        assert_eq!(colors.groups.len(), 1);
        assert!(colors.groups.contains_key("Normal"));
    }

    #[test]
    fn test_find_vim_files_in_json() {
        let json = r#"[{"name":"tokyonight.vim","path":"colors/tokyonight.vim"},{"name":"README.md"}]"#;
        let files = find_vim_files_in_json(json);
        assert_eq!(files, vec!["tokyonight.vim"]);
    }
}
