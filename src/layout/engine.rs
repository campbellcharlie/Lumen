//! Main layout engine

use super::text::layout_text;
use super::types::*;
use crate::ir::{Block, Document, Inline, ListItem};
use crate::theme::Theme;

/// Layout a document into a positioned tree
pub fn layout_document(document: &Document, theme: &Theme, viewport: Viewport) -> LayoutTree {
    let mut node_counter = 0;
    let mut hit_regions = Vec::new();

    let root = layout_blocks(
        &document.blocks,
        0,
        0,
        viewport.width,
        theme,
        &mut node_counter,
        &mut hit_regions,
    );

    let document_node = LayoutNode {
        id: node_counter,
        rect: Rectangle::new(0, 0, viewport.width, root.iter().map(|n| n.rect.height).sum()),
        element: LayoutElement::Document,
        children: root,
        style: ComputedStyle::default(),
    };

    LayoutTree {
        root: document_node,
        viewport,
        hit_regions,
    }
}

fn layout_blocks(
    blocks: &[Block],
    x: u16,
    mut y: u16,
    width: u16,
    theme: &Theme,
    node_counter: &mut NodeId,
    hit_regions: &mut Vec<HitRegion>,
) -> Vec<LayoutNode> {
    let mut nodes = Vec::new();

    for block in blocks {
        // Add top margin
        let margin_top = block_margin_top(block, theme);
        y += margin_top;

        let node = layout_block(block, x, y, width, theme, node_counter, hit_regions);
        y += node.rect.height;

        // Add bottom margin
        let margin_bottom = block_margin_bottom(block, theme);
        y += margin_bottom;

        nodes.push(node);
    }

    nodes
}

fn layout_block(
    block: &Block,
    x: u16,
    y: u16,
    width: u16,
    theme: &Theme,
    node_counter: &mut NodeId,
    hit_regions: &mut Vec<HitRegion>,
) -> LayoutNode {
    *node_counter += 1;
    let id = *node_counter;

    match block {
        Block::Heading { level, content } => {
            layout_heading(*level, content, x, y, width, theme, id, hit_regions)
        }
        Block::Paragraph { content } => {
            layout_paragraph(content, x, y, width, theme, id)
        }
        Block::CodeBlock { lang, code } => {
            layout_code_block(lang.as_deref(), code, x, y, width, theme, id, hit_regions)
        }
        Block::BlockQuote { blocks } => {
            layout_blockquote(blocks, x, y, width, theme, id, node_counter, hit_regions)
        }
        Block::List { ordered, start, items } => {
            layout_list(*ordered, *start, items, x, y, width, theme, id, node_counter, hit_regions)
        }
        Block::Table { headers, rows, alignment } => {
            layout_table(headers, rows, alignment, x, y, width, theme, id, node_counter)
        }
        Block::HorizontalRule => layout_horizontal_rule(x, y, width, theme, id),
        Block::Callout { .. } => {
            // Treat callouts as blockquotes for now
            layout_blockquote(&[], x, y, width, theme, id, node_counter, hit_regions)
        }
    }
}

fn layout_heading(
    level: u8,
    content: &[Inline],
    x: u16,
    y: u16,
    width: u16,
    theme: &Theme,
    id: NodeId,
    hit_regions: &mut Vec<HitRegion>,
) -> LayoutNode {
    let lines = layout_text(content, width, theme);
    let text = content.iter().map(|i| i.to_plain_text()).collect::<String>();
    let height = lines.len() as u16;

    // Add hit region for heading
    let heading_id = format!("h{}-{}", level, text.to_lowercase().replace(' ', "-"));
    hit_regions.push(HitRegion {
        rect: Rectangle::new(x, y, width, height),
        element: HitElement::Heading {
            level,
            id: heading_id,
        },
    });

    LayoutNode {
        id,
        rect: Rectangle::new(x, y, width, height),
        element: LayoutElement::Heading { level, text },
        children: Vec::new(),
        style: ComputedStyle::default(),
    }
}

fn layout_paragraph(
    content: &[Inline],
    x: u16,
    y: u16,
    width: u16,
    theme: &Theme,
    id: NodeId,
) -> LayoutNode {
    let lines = layout_text(content, width, theme);
    let height = lines.len() as u16;

    LayoutNode {
        id,
        rect: Rectangle::new(x, y, width, height),
        element: LayoutElement::Paragraph { lines },
        children: Vec::new(),
        style: ComputedStyle::default(),
    }
}

fn layout_code_block(
    lang: Option<&str>,
    code: &str,
    x: u16,
    y: u16,
    width: u16,
    theme: &Theme,
    id: NodeId,
    hit_regions: &mut Vec<HitRegion>,
) -> LayoutNode {
    let padding = theme.spacing.code_block_padding;
    let _content_width = width.saturating_sub(padding * 2);

    let lines: Vec<String> = code.lines().map(|line| line.to_string()).collect();
    let height = lines.len() as u16 + padding * 2;

    // Add hit region for code block
    hit_regions.push(HitRegion {
        rect: Rectangle::new(x, y, width, height),
        element: HitElement::CodeBlock {
            lang: lang.map(|s| s.to_string()),
        },
    });

    LayoutNode {
        id,
        rect: Rectangle::new(x, y, width, height),
        element: LayoutElement::CodeBlock {
            lang: lang.map(|s| s.to_string()),
            lines,
        },
        children: Vec::new(),
        style: ComputedStyle::default(),
    }
}

fn layout_blockquote(
    blocks: &[Block],
    x: u16,
    y: u16,
    width: u16,
    theme: &Theme,
    id: NodeId,
    node_counter: &mut NodeId,
    hit_regions: &mut Vec<HitRegion>,
) -> LayoutNode {
    let indent = theme.spacing.blockquote_indent;
    let content_width = width.saturating_sub(indent);

    let children = layout_blocks(
        blocks,
        x + indent,
        y,
        content_width,
        theme,
        node_counter,
        hit_regions,
    );

    let height = children.iter().map(|n| n.rect.height).sum::<u16>();

    LayoutNode {
        id,
        rect: Rectangle::new(x, y, width, height),
        element: LayoutElement::BlockQuote,
        children,
        style: ComputedStyle::default(),
    }
}

fn layout_list(
    ordered: bool,
    start: usize,
    items: &[ListItem],
    x: u16,
    y: u16,
    width: u16,
    theme: &Theme,
    id: NodeId,
    node_counter: &mut NodeId,
    hit_regions: &mut Vec<HitRegion>,
) -> LayoutNode {
    let indent = theme.spacing.list_indent;
    let mut children = Vec::new();
    let mut current_y = y;

    for (i, item) in items.iter().enumerate() {
        *node_counter += 1;
        let item_id = *node_counter;

        let marker = if ordered {
            format!("{}. ", start + i)
        } else {
            "• ".to_string()
        };

        let _marker_width = marker.len() as u16;
        let content_width = width.saturating_sub(indent);

        let item_children = layout_blocks(
            &item.content,
            x + indent,
            current_y,
            content_width,
            theme,
            node_counter,
            hit_regions,
        );

        let item_height = item_children.iter().map(|n| n.rect.height).sum::<u16>().max(1);

        let item_node = LayoutNode {
            id: item_id,
            rect: Rectangle::new(x, current_y, width, item_height),
            element: LayoutElement::ListItem {
                marker,
                task: item.task,
            },
            children: item_children,
            style: ComputedStyle::default(),
        };

        current_y += item_height;
        children.push(item_node);
    }

    let total_height = current_y - y;

    LayoutNode {
        id,
        rect: Rectangle::new(x, y, width, total_height),
        element: LayoutElement::List { ordered, start },
        children,
        style: ComputedStyle::default(),
    }
}

fn layout_table(
    headers: &[crate::ir::TableCell],
    rows: &[Vec<crate::ir::TableCell>],
    _alignment: &[crate::ir::Alignment],
    x: u16,
    y: u16,
    width: u16,
    theme: &Theme,
    id: NodeId,
    node_counter: &mut NodeId,
) -> LayoutNode {
    let num_columns = headers.len().max(rows.first().map(|r| r.len()).unwrap_or(0));
    let column_widths = compute_column_widths(num_columns, width);

    let mut children = Vec::new();
    let mut current_y = y;

    // Layout header row
    if !headers.is_empty() {
        *node_counter += 1;
        let row_node = layout_table_row(
            headers,
            &column_widths,
            x,
            current_y,
            width,
            theme,
            *node_counter,
            node_counter,
            true,
        );
        current_y += row_node.rect.height;
        children.push(row_node);
    }

    // Layout data rows
    for row in rows {
        *node_counter += 1;
        let row_node = layout_table_row(
            row,
            &column_widths,
            x,
            current_y,
            width,
            theme,
            *node_counter,
            node_counter,
            false,
        );
        current_y += row_node.rect.height;
        children.push(row_node);
    }

    let total_height = current_y - y;

    LayoutNode {
        id,
        rect: Rectangle::new(x, y, width, total_height),
        element: LayoutElement::Table { column_widths },
        children,
        style: ComputedStyle::default(),
    }
}

fn layout_table_row(
    cells: &[crate::ir::TableCell],
    column_widths: &[u16],
    x: u16,
    y: u16,
    width: u16,
    theme: &Theme,
    id: NodeId,
    node_counter: &mut NodeId,
    is_header: bool,
) -> LayoutNode {
    let mut children = Vec::new();
    let mut current_x = x;
    let mut max_height = 1u16;

    for (i, cell) in cells.iter().enumerate() {
        let cell_width = column_widths.get(i).copied().unwrap_or(10);
        let padding = theme.blocks.table.padding;
        let content_width = cell_width.saturating_sub(padding * 2);

        *node_counter += 1;
        let lines = layout_text(&cell.content, content_width, theme);
        let cell_height = lines.len() as u16 + padding * 2;
        max_height = max_height.max(cell_height);

        let cell_node = LayoutNode {
            id: *node_counter,
            rect: Rectangle::new(current_x, y, cell_width, cell_height),
            element: LayoutElement::TableCell,
            children: Vec::new(),
            style: ComputedStyle::default(),
        };

        current_x += cell_width;
        children.push(cell_node);
    }

    LayoutNode {
        id,
        rect: Rectangle::new(x, y, width, max_height),
        element: LayoutElement::TableRow { is_header },
        children,
        style: ComputedStyle::default(),
    }
}

fn compute_column_widths(num_columns: usize, total_width: u16) -> Vec<u16> {
    if num_columns == 0 {
        return Vec::new();
    }

    let width_per_column = total_width / num_columns as u16;
    vec![width_per_column; num_columns]
}

fn layout_horizontal_rule(x: u16, y: u16, width: u16, _theme: &Theme, id: NodeId) -> LayoutNode {
    LayoutNode {
        id,
        rect: Rectangle::new(x, y, width, 1),
        element: LayoutElement::HorizontalRule,
        children: Vec::new(),
        style: ComputedStyle::default(),
    }
}

fn block_margin_top(block: &Block, theme: &Theme) -> u16 {
    match block {
        Block::Heading { .. } => theme.spacing.heading_margin_top,
        _ => 0,
    }
}

fn block_margin_bottom(block: &Block, theme: &Theme) -> u16 {
    match block {
        Block::Paragraph { .. } => theme.spacing.paragraph_spacing,
        Block::Heading { .. } => theme.spacing.heading_margin_bottom,
        Block::CodeBlock { .. } => 1,
        Block::List { .. } => 1,  // Add spacing after lists
        Block::BlockQuote { .. } => 1,  // Add spacing after blockquotes
        Block::Table { .. } => 1,  // Add spacing after tables
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Block, Document, Inline};
    use crate::theme;

    #[test]
    fn test_layout_simple_document() {
        let theme = theme::docs_theme();
        let viewport = Viewport::new(80, 24);

        let doc = Document::with_blocks(vec![
            Block::Heading {
                level: 1,
                content: vec![Inline::Text("Title".to_string())],
            },
            Block::Paragraph {
                content: vec![Inline::Text("Content".to_string())],
            },
        ]);

        let tree = layout_document(&doc, &theme, viewport);

        assert_eq!(tree.root.children.len(), 2);
        assert!(matches!(
            tree.root.children[0].element,
            LayoutElement::Heading { level: 1, .. }
        ));
        assert!(matches!(
            tree.root.children[1].element,
            LayoutElement::Paragraph { .. }
        ));
    }

    #[test]
    fn test_layout_heading_has_hit_region() {
        let theme = theme::docs_theme();
        let viewport = Viewport::new(80, 24);

        let doc = Document::with_blocks(vec![Block::Heading {
            level: 1,
            content: vec![Inline::Text("Test Heading".to_string())],
        }]);

        let tree = layout_document(&doc, &theme, viewport);

        assert_eq!(tree.hit_regions.len(), 1);
        assert!(matches!(
            tree.hit_regions[0].element,
            HitElement::Heading { level: 1, .. }
        ));
    }

    #[test]
    fn test_layout_code_block_has_hit_region() {
        let theme = theme::docs_theme();
        let viewport = Viewport::new(80, 24);

        let doc = Document::with_blocks(vec![Block::CodeBlock {
            lang: Some("rust".to_string()),
            code: "fn main() {}".to_string(),
        }]);

        let tree = layout_document(&doc, &theme, viewport);

        assert_eq!(tree.hit_regions.len(), 1);
        assert!(matches!(
            tree.hit_regions[0].element,
            HitElement::CodeBlock { .. }
        ));
    }
}
