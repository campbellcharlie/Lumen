// Quick test to render list and inspect buffer
use ratatui::backend::TestBackend;
use ratatui::Terminal;
use std::io;

fn main() -> io::Result<()> {
    // Create test backend
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend)?;

    // Read and parse the markdown
    let content = std::fs::read_to_string("/tmp/test_list.md")?;

    // Parse markdown
    let parser = pulldown_cmark::Parser::new(&content);
    let ir = lumen::ir::parse_markdown(parser);

    // Load theme
    let theme = lumen::theme::defaults::docs_theme();

    // Layout
    let viewport = lumen::layout::types::Viewport {
        width: 80,
        height: 24,
    };
    let layout = lumen::layout::engine::layout_document(&ir, &theme, viewport);

    // Render
    terminal.draw(|frame| {
        let area = frame.area();
        lumen::render::render_document(frame, &layout, &theme, 0, area, None);
    })?;

    // Get buffer and print it
    let backend = terminal.backend();
    let buffer = backend.buffer();

    // Print first 10 lines showing char and style
    for y in 0..10.min(buffer.area.height) {
        println!("Line {}: ", y);
        for x in 0..buffer.area.width {
            let cell = buffer.get(x, y);
            let fg = if let Some(color) = cell.fg {
                format!("{:?}", color)
            } else {
                "None".to_string()
            };

            if cell.symbol != " " {
                println!("  [{},{}] '{}' fg={}", x, y, cell.symbol, fg);
            }
        }
    }

    Ok(())
}
