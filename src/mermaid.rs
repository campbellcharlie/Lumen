//! Built-in mermaid flowchart renderer — renders mermaid code blocks as ASCII/Unicode art.
//!
//! Supports `graph` and `flowchart` diagram types with LR, RL, TD/TB, BT directions.
//! Unsupported diagram types fall back to raw code display.
//!
//! Inspired by [mermaid-ascii](https://github.com/AlexanderGrooff/mermaid-ascii)
//! by Alexander Grooff — the original terminal mermaid renderer.

use crate::ir::{Block, Document};
use std::collections::HashMap;
use unicode_width::UnicodeWidthStr;

// ── Data types ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    LR,
    RL,
    TD,
    BT,
}

#[derive(Debug, Clone)]
struct Node {
    id: String,
    label: String,
}

#[derive(Debug, Clone)]
struct Edge {
    from: String,
    to: String,
    #[allow(dead_code)]
    label: Option<String>,
}

#[derive(Debug)]
struct FlowGraph {
    direction: Direction,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

// ── Public API ──────────────────────────────────────────────────────────────

/// Render a mermaid code block to ASCII art. Returns `None` if the diagram
/// type is unsupported or the syntax can't be parsed.
pub fn render_mermaid(input: &str) -> Option<String> {
    let graph = parse_flowchart(input)?;
    if graph.nodes.is_empty() {
        return None;
    }
    Some(render_graph(&graph))
}

/// Walk a parsed document and replace mermaid code blocks with rendered ASCII.
pub fn transform_mermaid_blocks(document: &mut Document) {
    for block in &mut document.blocks {
        transform_block(block);
    }
}

fn transform_block(block: &mut Block) {
    match block {
        Block::CodeBlock { lang, code } => {
            if lang.as_deref() == Some("mermaid") {
                if let Some(rendered) = render_mermaid(code) {
                    *code = rendered;
                }
            }
        }
        Block::BlockQuote { blocks } | Block::Callout { content: blocks, .. } => {
            for b in blocks {
                transform_block(b);
            }
        }
        Block::List { items, .. } => {
            for item in items {
                for b in &mut item.content {
                    transform_block(b);
                }
            }
        }
        _ => {}
    }
}

// ── Parser ──────────────────────────────────────────────────────────────────

fn parse_flowchart(input: &str) -> Option<FlowGraph> {
    let mut direction = Direction::TD;
    let mut nodes: Vec<Node> = Vec::new();
    let mut edges: Vec<Edge> = Vec::new();
    let mut node_map: HashMap<String, usize> = HashMap::new(); // id → index in nodes

    // Flatten semicolons into line breaks, then process line by line
    let input = input.replace(';', "\n");
    let mut found_header = false;

    for raw_line in input.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with("%%") {
            continue;
        }

        // Skip subgraph/end/style/classDef for now
        let lower = line.to_lowercase();
        if lower.starts_with("subgraph")
            || lower == "end"
            || lower.starts_with("style ")
            || lower.starts_with("classdef ")
            || lower.starts_with("class ")
            || lower.starts_with("click ")
            || lower.starts_with("linkstyle ")
        {
            continue;
        }

        // Parse header: graph/flowchart + direction
        if !found_header {
            if lower.starts_with("graph") || lower.starts_with("flowchart") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    direction = match parts[1].to_uppercase().as_str() {
                        "LR" => Direction::LR,
                        "RL" => Direction::RL,
                        "TD" | "TB" => Direction::TD,
                        "BT" => Direction::BT,
                        _ => Direction::TD,
                    };
                }
                found_header = true;
                continue;
            }
            // First non-empty line must be a graph/flowchart header
            if !lower.starts_with("graph") && !lower.starts_with("flowchart") {
                return None;
            }
        }

        // Parse edges and node definitions from this line
        parse_edge_line(line, &mut nodes, &mut edges, &mut node_map);
    }

    if !found_header && nodes.is_empty() {
        return None;
    }

    Some(FlowGraph {
        direction,
        nodes,
        edges,
    })
}

/// Parse a line that may contain chained edge definitions like `A --> B --> C`
fn parse_edge_line(
    line: &str,
    nodes: &mut Vec<Node>,
    edges: &mut Vec<Edge>,
    node_map: &mut HashMap<String, usize>,
) {
    // Tokenize into alternating node-refs and arrows
    let tokens = tokenize(line);
    if tokens.is_empty() {
        return;
    }

    // Process tokens: NodeRef Arrow NodeRef Arrow NodeRef ...
    let mut prev_node_id: Option<String> = None;
    let mut pending_label: Option<String> = None;

    for token in &tokens {
        match token {
            Token::NodeRef { id, label } => {
                ensure_node(nodes, node_map, id, label.as_deref());
                if let Some(from) = prev_node_id.take() {
                    edges.push(Edge {
                        from,
                        to: id.clone(),
                        label: pending_label.take(),
                    });
                }
                prev_node_id = Some(id.clone());
            }
            Token::Arrow { label } => {
                pending_label = label.clone();
            }
        }
    }
}

fn ensure_node(
    nodes: &mut Vec<Node>,
    node_map: &mut HashMap<String, usize>,
    id: &str,
    label: Option<&str>,
) {
    if let Some(&idx) = node_map.get(id) {
        // Update label if a better one is provided
        if let Some(lbl) = label {
            if nodes[idx].label == nodes[idx].id {
                nodes[idx].label = lbl.to_string();
            }
        }
    } else {
        let label = label.unwrap_or(id).to_string();
        node_map.insert(id.to_string(), nodes.len());
        nodes.push(Node {
            id: id.to_string(),
            label,
        });
    }
}

// ── Tokenizer ───────────────────────────────────────────────────────────────

#[derive(Debug)]
enum Token {
    NodeRef { id: String, label: Option<String> },
    Arrow { label: Option<String> },
}

/// Arrow patterns to search for, ordered longest-first to avoid partial matches
const ARROW_PATTERNS: &[&str] = &[
    "-.->", "==>", "-->", "-.-", "===", "---", "~~~", "-->", "->",
];

fn tokenize(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut remaining = line.trim();

    while !remaining.is_empty() {
        remaining = remaining.trim_start();
        if remaining.is_empty() {
            break;
        }

        // Try to match an arrow
        if let Some((arrow_label, rest)) = try_match_arrow(remaining) {
            tokens.push(Token::Arrow { label: arrow_label });
            remaining = rest;
            continue;
        }

        // Try to match a node reference
        if let Some((id, label, rest)) = try_match_node_ref(remaining) {
            tokens.push(Token::NodeRef { id, label });
            remaining = rest;
            continue;
        }

        // Skip unknown character
        if let Some(rest) = remaining.get(1..) {
            remaining = rest;
        } else {
            break;
        }
    }

    tokens
}

/// Try to match an arrow at the start of `s`. Returns (optional_label, remaining).
fn try_match_arrow(s: &str) -> Option<(Option<String>, &str)> {
    for &pattern in ARROW_PATTERNS {
        if s.starts_with(pattern) {
            let rest = &s[pattern.len()..];
            // Check for pipe-delimited label: -->|label|
            if let Some(rest) = rest.strip_prefix('|') {
                if let Some(end) = rest.find('|') {
                    let label = rest[..end].trim().to_string();
                    let label = if label.is_empty() { None } else { Some(label) };
                    return Some((label, &rest[end + 1..]));
                }
            }
            return Some((None, rest));
        }
    }

    // Handle "-- label -->" syntax: match `-- <text> -->`
    if s.starts_with("--") && !s.starts_with("-->") && !s.starts_with("---") {
        // Look for --> after the label
        if let Some(end) = s[2..].find("-->") {
            let label = s[2..2 + end].trim().to_string();
            let label = if label.is_empty() { None } else { Some(label) };
            return Some((label, &s[2 + end + 3..]));
        }
    }

    None
}

/// Try to match a node reference at the start of `s`.
/// Returns (id, optional_label, remaining).
fn try_match_node_ref(s: &str) -> Option<(String, Option<String>, &str)> {
    let s = s.trim_start();
    if s.is_empty() {
        return None;
    }

    // Collect the node ID (alphanumeric + underscore)
    let id_end = s
        .find(|c: char| !c.is_alphanumeric() && c != '_')
        .unwrap_or(s.len());

    if id_end == 0 {
        return None;
    }

    let id = s[..id_end].to_string();
    let rest = &s[id_end..];

    // Check for shape brackets after the ID
    let (label, rest) = if let Some(r) = rest.strip_prefix("([") {
        parse_until(r, "])")
    } else if let Some(r) = rest.strip_prefix("((") {
        parse_until(r, "))")
    } else if let Some(r) = rest.strip_prefix("[[") {
        parse_until(r, "]]")
    } else if let Some(r) = rest.strip_prefix("[/") {
        parse_until(r, "/]")
    } else if let Some(r) = rest.strip_prefix("[\\") {
        parse_until(r, "\\]")
    } else if let Some(r) = rest.strip_prefix('[') {
        parse_until_char(r, ']')
    } else if let Some(r) = rest.strip_prefix('(') {
        parse_until_char(r, ')')
    } else if let Some(r) = rest.strip_prefix('{') {
        parse_until_char(r, '}')
    } else if let Some(r) = rest.strip_prefix('>') {
        parse_until_char(r, ']')
    } else {
        (None, rest)
    };

    Some((id, label, rest))
}

fn parse_until<'a>(s: &'a str, end: &str) -> (Option<String>, &'a str) {
    if let Some(pos) = s.find(end) {
        let label = s[..pos].trim().to_string();
        (Some(label), &s[pos + end.len()..])
    } else {
        (None, s)
    }
}

fn parse_until_char(s: &str, end: char) -> (Option<String>, &str) {
    if let Some(pos) = s.find(end) {
        let label = s[..pos].trim().to_string();
        (Some(label), &s[pos + 1..])
    } else {
        (None, s)
    }
}

// ── Layout ──────────────────────────────────────────────────────────────────

struct NodeLayout {
    #[allow(dead_code)]
    col: usize, // layer (column for LR, row for TD)
    #[allow(dead_code)]
    row: usize, // position within layer
    x: usize,   // canvas x
    y: usize,   // canvas y
    width: usize,
    label: String,
}

struct GraphLayout {
    nodes: Vec<NodeLayout>,
    node_index: HashMap<String, usize>, // node id → index in nodes
    canvas_width: usize,
    canvas_height: usize,
    direction: Direction,
}

const BOX_HEIGHT: usize = 3;
const BOX_PAD: usize = 2; // padding inside box on each side
const ARROW_LEN: usize = 5; // length of arrow between nodes
const LAYER_GAP_H: usize = 3; // vertical gap between rows for TD (room for L-routing)
const LAYER_GAP_V: usize = 1; // horizontal gap between nodes in same layer

fn layout_graph(graph: &FlowGraph) -> GraphLayout {
    let node_ids: Vec<&str> = graph.nodes.iter().map(|n| n.id.as_str()).collect();
    let id_to_idx: HashMap<&str, usize> = node_ids
        .iter()
        .enumerate()
        .map(|(i, id)| (*id, i))
        .collect();

    // Build adjacency lists
    let n = graph.nodes.len();
    let mut children: Vec<Vec<usize>> = vec![vec![]; n];
    let mut has_parent = vec![false; n];

    for edge in &graph.edges {
        if let (Some(&from), Some(&to)) = (id_to_idx.get(edge.from.as_str()), id_to_idx.get(edge.to.as_str())) {
            children[from].push(to);
            has_parent[to] = true;
        }
    }

    // Assign layers via BFS from roots
    let mut layer_of = vec![0usize; n];
    let roots: Vec<usize> = (0..n).filter(|&i| !has_parent[i]).collect();

    // If no roots (cycle), pick node 0
    let start_nodes = if roots.is_empty() { vec![0] } else { roots };

    let mut visited = vec![false; n];
    let mut queue = std::collections::VecDeque::new();
    for &root in &start_nodes {
        queue.push_back(root);
        visited[root] = true;
    }

    while let Some(node) = queue.pop_front() {
        for &child in &children[node] {
            let new_layer = layer_of[node] + 1;
            if new_layer > layer_of[child] {
                layer_of[child] = new_layer;
            }
            if !visited[child] {
                visited[child] = true;
                queue.push_back(child);
            }
        }
    }

    // Handle unvisited nodes (disconnected)
    for i in 0..n {
        if !visited[i] {
            layer_of[i] = 0;
        }
    }

    // Group nodes by layer, preserving parse order within each layer
    let num_layers = layer_of.iter().copied().max().unwrap_or(0) + 1;
    let mut layers: Vec<Vec<usize>> = vec![vec![]; num_layers];
    for (i, &layer) in layer_of.iter().enumerate() {
        layers[layer].push(i);
    }

    // Compute display widths for each node
    let display_widths: Vec<usize> = graph
        .nodes
        .iter()
        .map(|node| UnicodeWidthStr::width(node.label.as_str()) + BOX_PAD * 2 + 2) // +2 for borders
        .collect();

    // Calculate positions based on direction
    let is_horizontal = matches!(graph.direction, Direction::LR | Direction::RL);

    let mut node_layouts = Vec::with_capacity(n);
    let mut node_index = HashMap::new();

    if is_horizontal {
        // LR/RL: layers are columns, nodes within layer are rows
        let mut col_widths: Vec<usize> = vec![0; num_layers];
        let mut rows_per_layer: Vec<usize> = vec![0; num_layers];

        for (layer_idx, layer_nodes) in layers.iter().enumerate() {
            rows_per_layer[layer_idx] = layer_nodes.len();
            for &node_idx in layer_nodes {
                col_widths[layer_idx] = col_widths[layer_idx].max(display_widths[node_idx]);
            }
        }

        let max_rows = rows_per_layer.iter().copied().max().unwrap_or(1);

        // Calculate x positions for each layer (column)
        let mut layer_x = vec![0usize; num_layers];
        let mut x = 0;
        let layer_order: Vec<usize> = if graph.direction == Direction::RL {
            (0..num_layers).rev().collect()
        } else {
            (0..num_layers).collect()
        };

        for (draw_idx, &layer_idx) in layer_order.iter().enumerate() {
            layer_x[layer_idx] = x;
            x += col_widths[layer_idx];
            if draw_idx < num_layers - 1 {
                x += ARROW_LEN;
            }
        }

        let canvas_width = x;
        let canvas_height = max_rows * BOX_HEIGHT + (max_rows.saturating_sub(1)) * LAYER_GAP_V;

        for (layer_idx, layer_nodes) in layers.iter().enumerate() {
            // Center nodes vertically within the column
            let total_h =
                layer_nodes.len() * BOX_HEIGHT + layer_nodes.len().saturating_sub(1) * LAYER_GAP_V;
            let y_offset = (canvas_height.saturating_sub(total_h)) / 2;

            for (row_idx, &node_idx) in layer_nodes.iter().enumerate() {
                let node = &graph.nodes[node_idx];
                let w = display_widths[node_idx];
                // Center node within column width
                let x = layer_x[layer_idx] + (col_widths[layer_idx].saturating_sub(w)) / 2;
                let y = y_offset + row_idx * (BOX_HEIGHT + LAYER_GAP_V);

                node_index.insert(node.id.clone(), node_layouts.len());
                node_layouts.push(NodeLayout {
                    col: layer_idx,
                    row: row_idx,
                    x,
                    y,
                    width: w,
                    label: node.label.clone(),
                });
            }
        }

        GraphLayout {
            nodes: node_layouts,
            node_index,
            canvas_width,
            canvas_height,
            direction: graph.direction,
        }
    } else {
        // TD/BT: layers are rows, nodes within layer are columns
        let mut max_width_per_layer: Vec<usize> = vec![0; num_layers];
        let mut total_width_per_layer: Vec<usize> = vec![0; num_layers];

        for (layer_idx, layer_nodes) in layers.iter().enumerate() {
            let total: usize = layer_nodes.iter().map(|&i| display_widths[i]).sum::<usize>()
                + layer_nodes.len().saturating_sub(1) * (LAYER_GAP_V + 2);
            total_width_per_layer[layer_idx] = total;
            for &node_idx in layer_nodes {
                max_width_per_layer[layer_idx] =
                    max_width_per_layer[layer_idx].max(display_widths[node_idx]);
            }
        }

        let canvas_width = total_width_per_layer.iter().copied().max().unwrap_or(10);

        // Calculate y positions for each layer (row)
        let mut layer_y = vec![0usize; num_layers];
        let mut y = 0;
        let layer_order: Vec<usize> = if graph.direction == Direction::BT {
            (0..num_layers).rev().collect()
        } else {
            (0..num_layers).collect()
        };

        for (draw_idx, &layer_idx) in layer_order.iter().enumerate() {
            layer_y[layer_idx] = y;
            y += BOX_HEIGHT;
            if draw_idx < num_layers - 1 {
                y += LAYER_GAP_H;
            }
        }

        let canvas_height = y;

        for (layer_idx, layer_nodes) in layers.iter().enumerate() {
            let total_w = total_width_per_layer[layer_idx];
            let x_offset = canvas_width.saturating_sub(total_w) / 2;

            let mut x = x_offset;
            for &node_idx in layer_nodes {
                let node = &graph.nodes[node_idx];
                let w = display_widths[node_idx];

                node_index.insert(node.id.clone(), node_layouts.len());
                node_layouts.push(NodeLayout {
                    col: layer_idx,
                    row: 0,
                    x,
                    y: layer_y[layer_idx],
                    width: w,
                    label: node.label.clone(),
                });
                x += w + LAYER_GAP_V + 2;
            }
        }

        GraphLayout {
            nodes: node_layouts,
            node_index,
            canvas_width,
            canvas_height,
            direction: graph.direction,
        }
    }
}

// ── Canvas ──────────────────────────────────────────────────────────────────

struct Canvas {
    cells: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![vec![' '; width]; height],
            width,
            height,
        }
    }

    fn set(&mut self, x: usize, y: usize, ch: char) {
        if x < self.width && y < self.height {
            self.cells[y][x] = ch;
        }
    }

    fn draw_text(&mut self, x: usize, y: usize, text: &str) {
        let mut cx = x;
        for ch in text.chars() {
            self.set(cx, y, ch);
            cx += unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1);
        }
    }

    fn draw_box(&mut self, x: usize, y: usize, width: usize, label: &str) {
        if width < 2 || y + 2 >= self.height + 1 {
            return;
        }
        // Top border
        self.set(x, y, '┌');
        for i in 1..width - 1 {
            self.set(x + i, y, '─');
        }
        self.set(x + width - 1, y, '┐');

        // Content row — center the label
        self.set(x, y + 1, '│');
        let label_w = UnicodeWidthStr::width(label);
        let inner = width.saturating_sub(2);
        let pad_left = inner.saturating_sub(label_w) / 2;
        self.draw_text(x + 1 + pad_left, y + 1, label);
        self.set(x + width - 1, y + 1, '│');

        // Bottom border
        self.set(x, y + 2, '└');
        for i in 1..width - 1 {
            self.set(x + i, y + 2, '─');
        }
        self.set(x + width - 1, y + 2, '┘');
    }

    fn draw_harrow(&mut self, x1: usize, x2: usize, y: usize) {
        if x2 <= x1 + 1 {
            return;
        }
        for x in x1..x2 {
            self.set(x, y, '─');
        }
        self.set(x2, y, '▶');
    }

    fn draw_harrow_rev(&mut self, x1: usize, x2: usize, y: usize) {
        if x2 <= x1 + 1 {
            return;
        }
        self.set(x1, y, '◀');
        for x in x1 + 1..=x2 {
            self.set(x, y, '─');
        }
    }

    fn draw_varrow(&mut self, x: usize, y1: usize, y2: usize) {
        if y2 <= y1 {
            return;
        }
        for y in y1..y2 {
            self.set(x, y, '│');
        }
        self.set(x, y2, '▼');
    }

    fn draw_varrow_up(&mut self, x: usize, y1: usize, y2: usize) {
        if y2 <= y1 {
            return;
        }
        self.set(x, y1, '▲');
        for y in y1 + 1..=y2 {
            self.set(x, y, '│');
        }
    }

    fn to_string(&self) -> String {
        self.cells
            .iter()
            .map(|row| {
                let s: String = row.iter().collect();
                s.trim_end().to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
            .trim_end_matches('\n')
            .to_string()
    }
}

// ── Render ───────────────────────────────────────────────────────────────────

fn render_graph(graph: &FlowGraph) -> String {
    let layout = layout_graph(graph);

    let mut canvas = Canvas::new(
        layout.canvas_width.max(1),
        layout.canvas_height.max(1),
    );

    // Draw all nodes
    for nl in &layout.nodes {
        canvas.draw_box(nl.x, nl.y, nl.width, &nl.label);
    }

    // Draw edges
    let is_horizontal = matches!(layout.direction, Direction::LR | Direction::RL);

    for edge in &graph.edges {
        let from_idx = layout.node_index.get(&edge.from);
        let to_idx = layout.node_index.get(&edge.to);

        if let (Some(&fi), Some(&ti)) = (from_idx, to_idx) {
            let from = &layout.nodes[fi];
            let to = &layout.nodes[ti];

            if is_horizontal {
                // Horizontal arrow: from right edge of source to left edge of target
                let from_right = from.x + from.width;
                let from_mid_y = from.y + 1; // middle row of box
                let to_left = to.x;
                let to_mid_y = to.y + 1;

                if from_mid_y == to_mid_y {
                    // Same row — straight horizontal arrow
                    if layout.direction == Direction::RL {
                        if to.x + to.width < from.x {
                            canvas.draw_harrow_rev(to.x + to.width, from.x.saturating_sub(1), from_mid_y);
                        }
                    } else if from_right < to_left {
                        canvas.draw_harrow(from_right, to_left.saturating_sub(1), from_mid_y);
                    }
                } else {
                    // Different rows — L-shaped route
                    let mid_x = if layout.direction == Direction::RL {
                        to.x + to.width + (from.x.saturating_sub(to.x + to.width)) / 2
                    } else {
                        from_right + (to_left.saturating_sub(from_right)) / 2
                    };

                    // Horizontal from source to mid
                    for x in from_right..=mid_x {
                        canvas.set(x, from_mid_y, '─');
                    }
                    // Vertical from source_y to target_y
                    let (y_start, y_end) = if from_mid_y < to_mid_y {
                        (from_mid_y, to_mid_y)
                    } else {
                        (to_mid_y, from_mid_y)
                    };
                    for y in y_start..=y_end {
                        canvas.set(mid_x, y, '│');
                    }
                    // Corner at source
                    if from_mid_y < to_mid_y {
                        canvas.set(mid_x, from_mid_y, '┐');
                        canvas.set(mid_x, to_mid_y, '└');
                    } else {
                        canvas.set(mid_x, from_mid_y, '┘');
                        canvas.set(mid_x, to_mid_y, '┌');
                    }
                    // Horizontal from mid to target
                    for x in mid_x + 1..to_left {
                        canvas.set(x, to_mid_y, '─');
                    }
                    if to_left > 0 {
                        canvas.set(to_left.saturating_sub(1), to_mid_y, '▶');
                    }
                }
            } else {
                // Vertical arrow: from bottom of source to top of target
                let from_bottom = from.y + BOX_HEIGHT;
                let from_mid_x = from.x + from.width / 2;
                let to_top = to.y;
                let to_mid_x = to.x + to.width / 2;

                if from_mid_x == to_mid_x {
                    // Same column — straight vertical arrow
                    if layout.direction == Direction::BT {
                        if to.y + BOX_HEIGHT < from.y {
                            canvas.draw_varrow_up(from_mid_x, to.y + BOX_HEIGHT, from.y.saturating_sub(1));
                        }
                    } else if from_bottom < to_top {
                        canvas.draw_varrow(from_mid_x, from_bottom, to_top.saturating_sub(1));
                    }
                } else {
                    // Different columns — L-shaped route (down, across, down)
                    let mid_y = from_bottom + (to_top.saturating_sub(from_bottom)) / 2;

                    // Vertical from source bottom to mid
                    for y in from_bottom..mid_y {
                        canvas.set(from_mid_x, y, '│');
                    }
                    // Horizontal from source_x to target_x at mid_y
                    let (x_start, x_end) = if from_mid_x < to_mid_x {
                        (from_mid_x, to_mid_x)
                    } else {
                        (to_mid_x, from_mid_x)
                    };
                    for x in x_start..=x_end {
                        canvas.set(x, mid_y, '─');
                    }
                    // Corners
                    if from_mid_x < to_mid_x {
                        canvas.set(from_mid_x, mid_y, '╰');
                        canvas.set(to_mid_x, mid_y, '╮');
                    } else {
                        canvas.set(from_mid_x, mid_y, '╯');
                        canvas.set(to_mid_x, mid_y, '╭');
                    }
                    // Vertical from mid to target
                    for y in mid_y + 1..to_top.saturating_sub(1) {
                        canvas.set(to_mid_x, y, '│');
                    }
                    if to_top > 0 {
                        canvas.set(to_mid_x, to_top.saturating_sub(1), '▼');
                    }
                    if to_top > 0 {
                        canvas.set(to_mid_x, to_top.saturating_sub(1), '▼');
                    }
                }
            }
        }
    }

    canvas.to_string()
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_graph() {
        let input = "graph LR\n    A --> B --> C";
        let graph = parse_flowchart(input).unwrap();
        assert_eq!(graph.direction, Direction::LR);
        assert_eq!(graph.nodes.len(), 3);
        assert_eq!(graph.edges.len(), 2);
    }

    #[test]
    fn test_parse_labeled_nodes() {
        let input = "graph TD\n    A[Start] --> B{Decision}\n    B --> C[End]";
        let graph = parse_flowchart(input).unwrap();
        assert_eq!(graph.nodes.len(), 3);
        assert_eq!(graph.nodes[0].label, "Start");
        assert_eq!(graph.nodes[1].label, "Decision");
        assert_eq!(graph.nodes[2].label, "End");
    }

    #[test]
    fn test_parse_edge_labels() {
        let input = "graph TD\n    A -->|Yes| B\n    A -->|No| C";
        let graph = parse_flowchart(input).unwrap();
        assert_eq!(graph.edges.len(), 2);
        assert_eq!(graph.edges[0].label, Some("Yes".to_string()));
        assert_eq!(graph.edges[1].label, Some("No".to_string()));
    }

    #[test]
    fn test_render_simple_lr() {
        let input = "graph LR\n    A --> B --> C";
        let output = render_mermaid(input).unwrap();
        assert!(output.contains("A"));
        assert!(output.contains("B"));
        assert!(output.contains("C"));
        assert!(output.contains("▶")); // arrow
        assert!(output.contains("┌")); // box border
    }

    #[test]
    fn test_render_simple_td() {
        let input = "graph TD\n    A --> B --> C";
        let output = render_mermaid(input).unwrap();
        assert!(output.contains("A"));
        assert!(output.contains("B"));
        assert!(output.contains("C"));
        assert!(output.contains("▼")); // down arrow
    }

    #[test]
    fn test_render_branching_td() {
        let input = "graph TD\n    A --> B\n    A --> C";
        let output = render_mermaid(input).unwrap();
        assert!(output.contains("A"));
        assert!(output.contains("B"));
        assert!(output.contains("C"));
    }

    #[test]
    fn test_semicolons_as_separators() {
        let input = "graph LR; A --> B; B --> C";
        let graph = parse_flowchart(input).unwrap();
        assert_eq!(graph.nodes.len(), 3);
        assert_eq!(graph.edges.len(), 2);
    }

    #[test]
    fn test_comments_ignored() {
        let input = "graph TD\n    %% This is a comment\n    A --> B";
        let graph = parse_flowchart(input).unwrap();
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 1);
    }

    #[test]
    fn test_unsupported_returns_none() {
        let input = "sequenceDiagram\n    Alice->>Bob: Hello";
        assert!(parse_flowchart(input).is_none());
    }

    #[test]
    fn test_flowchart_keyword() {
        let input = "flowchart LR\n    A --> B";
        let graph = parse_flowchart(input).unwrap();
        assert_eq!(graph.direction, Direction::LR);
        assert_eq!(graph.nodes.len(), 2);
    }

    #[test]
    fn test_render_output_not_empty() {
        let input = "graph TD\n    Start[Start] --> Process[Process Data]\n    Process --> End[Done]";
        let output = render_mermaid(input).unwrap();
        assert!(!output.is_empty());
        // Should contain the labels
        assert!(output.contains("Start"));
        assert!(output.contains("Process Data"));
        assert!(output.contains("Done"));
    }
}
