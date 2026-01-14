//! Main layout engine

use super::text::layout_text;
use super::types::*;
use crate::ir::{Block, CalloutKind, Document, Inline, ListItem};
use crate::theme::Theme;

/// Layout a document into a positioned tree
pub fn layout_document(document: &Document, theme: &Theme, viewport: Viewport, inline_images: bool) -> LayoutTree {
    let mut node_counter = 0;
    let mut hit_regions = Vec::new();
    let mut images = Vec::new();

    let root = layout_blocks(
        &document.blocks,
        0,
        0,
        viewport.width,
        theme,
        &mut node_counter,
        &mut hit_regions,
        &mut images,
        inline_images,
    );

    // Calculate actual document height as the maximum (y + height) of all children
    let doc_height = root.iter()
        .map(|n| n.rect.y + n.rect.height)
        .max()
        .unwrap_or(0);

    let document_node = LayoutNode {
        id: node_counter,
        rect: Rectangle::new(0, 0, viewport.width, doc_height),
        element: LayoutElement::Document,
        children: root,
        style: ComputedStyle::default(),
    };

    LayoutTree {
        root: document_node,
        viewport,
        hit_regions,
        images,
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
    images: &mut Vec<ImageReference>,
    inline_images: bool,
) -> Vec<LayoutNode> {
    let mut nodes = Vec::new();

    for block in blocks {
        // Add top margin
        let margin_top = block_margin_top(block, theme);
        y += margin_top;

        let node = layout_block(block, x, y, width, theme, node_counter, hit_regions, images, inline_images);
        y += node.rect.height;

        // Add bottom margin
        let margin_bottom = block_margin_bottom(block, theme);
        y += margin_bottom;

        nodes.push(node);
    }

    nodes
}

/// Layout blocks for list item content with tight spacing (no paragraph margins)
fn layout_list_item_blocks(
    blocks: &[Block],
    x: u16,
    mut y: u16,
    width: u16,
    theme: &Theme,
    node_counter: &mut NodeId,
    hit_regions: &mut Vec<HitRegion>,
    images: &mut Vec<ImageReference>,
    inline_images: bool,
) -> Vec<LayoutNode> {
    let mut nodes = Vec::new();

    for block in blocks {
        // Add top margin (only for headings, not paragraphs)
        let margin_top = match block {
            Block::Heading { .. } => theme.spacing.heading_margin_top,
            _ => 0,
        };
        y += margin_top;

        let node = layout_block(block, x, y, width, theme, node_counter, hit_regions, images, inline_images);
        y += node.rect.height;

        // Add bottom margin (no paragraph spacing in tight lists)
        let margin_bottom = match block {
            Block::Heading { .. } => theme.spacing.heading_margin_bottom,
            Block::CodeBlock { .. } => 1,
            Block::BlockQuote { .. } => 1,
            Block::Table { .. } => 1,
            _ => 0,  // No spacing for paragraphs and lists in tight list items
        };
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
    images: &mut Vec<ImageReference>,
    inline_images: bool,
) -> LayoutNode {
    *node_counter += 1;
    let id = *node_counter;

    match block {
        Block::Heading { level, content } => {
            layout_heading(*level, content, x, y, width, theme, id, hit_regions, images, inline_images)
        }
        Block::Paragraph { content } => {
            layout_paragraph(content, x, y, width, theme, id, images, inline_images, node_counter)
        }
        Block::CodeBlock { lang, code } => {
            layout_code_block(lang.as_deref(), code, x, y, width, theme, id, hit_regions, images)
        }
        Block::BlockQuote { blocks } => {
            layout_blockquote(blocks, x, y, width, theme, id, node_counter, hit_regions, images, inline_images)
        }
        Block::List { ordered, start, items } => {
            layout_list(*ordered, *start, items, x, y, width, theme, id, node_counter, hit_regions, images, inline_images)
        }
        Block::Table { headers, rows, alignment } => {
            layout_table(headers, rows, alignment, x, y, width, theme, id, node_counter, images, inline_images)
        }
        Block::HorizontalRule => layout_horizontal_rule(x, y, width, theme, id),
        Block::Callout { kind, content, .. } => {
            layout_callout(*kind, content, x, y, width, theme, id, node_counter, hit_regions, images, inline_images)
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
    images: &mut Vec<ImageReference>,
    inline_images: bool,
) -> LayoutNode {
    let (lines, _inline_imgs) = layout_text(content, width, theme, y, images, inline_images);
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
    images: &mut Vec<ImageReference>,
    inline_images: bool,
    node_counter: &mut NodeId,
) -> LayoutNode {
    let (lines, inline_imgs) = layout_text(content, width, theme, y, images, inline_images);
    // Ensure minimum height of 1 to prevent nested elements from overlapping
    let mut height = lines.len().max(1) as u16;
    let mut children = Vec::new();

    // If inline images mode is enabled, create Image child nodes
    if inline_images && !inline_imgs.is_empty() {
        // Default image height in terminal rows (can be adjusted)
        const IMAGE_HEIGHT: u16 = 12;

        for (line_idx, path, alt_text) in inline_imgs {
            *node_counter += 1;
            let img_y = y + line_idx + 1; // Position image after the line it appears in

            // Create an Image layout node
            let image_node = LayoutNode {
                id: *node_counter,
                rect: Rectangle::new(x, img_y, width, IMAGE_HEIGHT),
                element: LayoutElement::Image {
                    path,
                    alt_text,
                },
                children: Vec::new(),
                style: ComputedStyle::default(),
            };

            children.push(image_node);
            // Add image height to total paragraph height
            height += IMAGE_HEIGHT;
        }
    }

    LayoutNode {
        id,
        rect: Rectangle::new(x, y, width, height),
        element: LayoutElement::Paragraph { lines },
        children,
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
    _images: &mut Vec<ImageReference>,
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
    images: &mut Vec<ImageReference>,
    inline_images: bool,
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
        images,
        inline_images,
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

fn layout_callout(
    kind: CalloutKind,
    blocks: &[Block],
    x: u16,
    y: u16,
    width: u16,
    theme: &Theme,
    id: NodeId,
    node_counter: &mut NodeId,
    hit_regions: &mut Vec<HitRegion>,
    images: &mut Vec<ImageReference>,
    inline_images: bool,
) -> LayoutNode {
    // Callouts have a 2-character indent for the icon/border
    let indent = 2u16;
    let content_width = width.saturating_sub(indent);

    let children = layout_blocks(
        blocks,
        x + indent,
        y,
        content_width,
        theme,
        node_counter,
        hit_regions,
        images,
        inline_images,
    );

    let height = children.iter().map(|n| n.rect.height).sum::<u16>().max(1);

    LayoutNode {
        id,
        rect: Rectangle::new(x, y, width, height),
        element: LayoutElement::Callout { kind },
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
    images: &mut Vec<ImageReference>,
    inline_images: bool,
) -> LayoutNode {
    let mut children = Vec::new();
    let mut current_y = y;

    for (i, item) in items.iter().enumerate() {
        *node_counter += 1;
        let item_id = *node_counter;

        let marker = if ordered {
            format!("{}.", start + i)
        } else {
            "•".to_string()
        };

        // Calculate display width (not byte length) for proper positioning
        // The bullet "•" is 3 bytes but only 1 char wide in the terminal
        let marker_width = marker.chars().count() as u16;
        // Add minimal 1-space gap between marker and content
        let content_start = x + marker_width + 1;
        let content_width = width.saturating_sub(marker_width + 1);

        // Layout list item children with tight spacing (no paragraph margins)
        let item_children = layout_list_item_blocks(
            &item.content,
            content_start,
            current_y,
            content_width,
            theme,
            node_counter,
            hit_regions,
            images,
            inline_images,
        );

        // Calculate actual height by finding the Y span of children
        // (not just sum of heights, because layout_blocks adds margins between children)
        let item_height = if item_children.is_empty() {
            1
        } else {
            let first_y = item_children.first().unwrap().rect.y;
            let last_child = item_children.last().unwrap();
            let last_y = last_child.rect.y + last_child.rect.height;
            last_y - first_y
        };

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
    images: &mut Vec<ImageReference>,
    inline_images: bool,
) -> LayoutNode {
    let column_widths = compute_column_widths(headers, rows, theme, width);

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
            images,
            inline_images,
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
            images,
            inline_images,
        );
        current_y += row_node.rect.height;
        children.push(row_node);
    }

    let total_height = current_y - y;

    // Calculate actual table width from column widths
    let actual_width: u16 = column_widths.iter().sum();

    LayoutNode {
        id,
        rect: Rectangle::new(x, y, actual_width, total_height),
        element: LayoutElement::Table { column_widths: column_widths.clone() },
        children,
        style: ComputedStyle::default(),
    }
}

fn layout_table_row(
    cells: &[crate::ir::TableCell],
    column_widths: &[u16],
    x: u16,
    y: u16,
    _width: u16,
    theme: &Theme,
    id: NodeId,
    node_counter: &mut NodeId,
    is_header: bool,
    images: &mut Vec<ImageReference>,
    inline_images: bool,
) -> LayoutNode {
    let mut children = Vec::new();
    let mut current_x = x;
    let mut max_height = 1u16;

    for (i, cell) in cells.iter().enumerate() {
        let cell_width = column_widths.get(i).copied().unwrap_or(10);
        let padding = theme.blocks.table.padding;
        let content_width = cell_width.saturating_sub(padding * 2);

        *node_counter += 1;
        let cell_id = *node_counter;

        let (lines, _inline_imgs) = layout_text(&cell.content, content_width, theme, y, images, inline_images);
        let cell_height = lines.len() as u16 + padding * 2;
        max_height = max_height.max(cell_height);

        // Create a paragraph node to hold the cell content
        *node_counter += 1;
        let content_node = LayoutNode {
            id: *node_counter,
            rect: Rectangle::new(current_x + padding, y + padding, content_width, lines.len() as u16),
            element: LayoutElement::Paragraph { lines },
            children: Vec::new(),
            style: ComputedStyle::default(),
        };

        let cell_node = LayoutNode {
            id: cell_id,
            rect: Rectangle::new(current_x, y, cell_width, cell_height),
            element: LayoutElement::TableCell,
            children: vec![content_node],
            style: ComputedStyle::default(),
        };

        current_x += cell_width;
        children.push(cell_node);
    }

    // Calculate actual row width from column widths
    let actual_width: u16 = column_widths.iter().sum();

    LayoutNode {
        id,
        rect: Rectangle::new(x, y, actual_width, max_height),
        element: LayoutElement::TableRow { is_header },
        children,
        style: ComputedStyle::default(),
    }
}

fn inline_text_length(inline: &crate::ir::Inline) -> usize {
    match inline {
        crate::ir::Inline::Text(s) | crate::ir::Inline::Code(s) => s.len(),
        crate::ir::Inline::Strong(inlines)
        | crate::ir::Inline::Emphasis(inlines)
        | crate::ir::Inline::Strikethrough(inlines) => {
            inlines.iter().map(inline_text_length).sum()
        }
        crate::ir::Inline::Link { text, .. } => {
            text.iter().map(inline_text_length).sum()
        }
        crate::ir::Inline::SoftBreak | crate::ir::Inline::LineBreak => 0,
        crate::ir::Inline::Image { .. } => 8, // Placeholder width for images
    }
}

fn compute_column_widths(
    headers: &[crate::ir::TableCell],
    rows: &[Vec<crate::ir::TableCell>],
    theme: &Theme,
    max_width: u16,
) -> Vec<u16> {
    let num_columns = headers.len().max(rows.first().map(|r| r.len()).unwrap_or(0));
    if num_columns == 0 {
        return Vec::new();
    }

    let padding = theme.blocks.table.padding;
    let mut column_widths = vec![0u16; num_columns];

    // Check header cells
    for (i, cell) in headers.iter().enumerate() {
        if i >= num_columns {
            break;
        }
        // Get the total text width in this cell's content
        let content_width = cell.content.iter()
            .map(inline_text_length)
            .sum::<usize>() as u16;
        column_widths[i] = column_widths[i].max(content_width + padding * 2);
    }

    // Check all data rows
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if i >= num_columns {
                break;
            }
            let content_width = cell.content.iter()
                .map(inline_text_length)
                .sum::<usize>() as u16;
            column_widths[i] = column_widths[i].max(content_width + padding * 2);
        }
    }

    // Ensure minimum width of 3 per column (for borders and at least 1 char)
    for width in &mut column_widths {
        *width = (*width).max(3);
    }

    // If total exceeds max_width, scale down proportionally
    let total_width: u16 = column_widths.iter().sum();
    if total_width > max_width {
        let scale = max_width as f64 / total_width as f64;
        for width in &mut column_widths {
            *width = ((*width as f64 * scale) as u16).max(3);
        }
    }

    column_widths
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
        Block::List { .. } => 0,  // Lists should not add top margin (spacing comes from previous block)
        _ => 0,
    }
}

fn block_margin_bottom(block: &Block, theme: &Theme) -> u16 {
    match block {
        Block::Paragraph { .. } => theme.spacing.paragraph_spacing,
        Block::Heading { .. } => theme.spacing.heading_margin_bottom,
        Block::CodeBlock { .. } => 1,
        Block::List { .. } => 0,  // Lists handle their own spacing
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

        let tree = layout_document(&doc, &theme, viewport, false);

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

        let tree = layout_document(&doc, &theme, viewport, false);

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

        let tree = layout_document(&doc, &theme, viewport, false);

        assert_eq!(tree.hit_regions.len(), 1);
        assert!(matches!(
            tree.hit_regions[0].element,
            HitElement::CodeBlock { .. }
        ));
    }
}
