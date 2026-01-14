//! Markdown to IR conversion using pulldown-cmark

use crate::ir::{Alignment, Block, CalloutKind, Document, Inline, ListItem, TableCell};
use pulldown_cmark::{
    Alignment as CMarkAlignment, Event, HeadingLevel, Options, Parser, Tag, TagEnd,
};

/// Parse a Markdown string into a Lumen Document
pub fn parse_markdown(markdown: &str) -> Document {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(markdown, options);
    let mut converter = MarkdownConverter::new();
    converter.process_events(parser);
    converter.finish()
}

/// Converter state machine
struct MarkdownConverter {
    document: Document,
    block_stack: Vec<BlockContext>,
    inline_stack: Vec<InlineContext>,
    current_inlines: Vec<Inline>,
}

/// Context for nested block elements
#[derive(Debug)]
enum BlockContext {
    Paragraph,
    Heading {
        level: u8,
    },
    BlockQuote {
        blocks: Vec<Block>,
    },
    List {
        ordered: bool,
        start: usize,
        items: Vec<ListItem>,
    },
    ListItem {
        blocks: Vec<Block>,
        task: Option<bool>,
    },
    CodeBlock {
        lang: Option<String>,
        code: String,
    },
    Table {
        headers: Vec<TableCell>,
        rows: Vec<Vec<TableCell>>,
        current_row: Vec<TableCell>,
        alignment: Vec<Alignment>,
    },
    TableHead,
    TableRow,
    TableCell,
}

/// Context for nested inline elements
#[derive(Debug)]
enum InlineContext {
    Strong {
        content: Vec<Inline>,
    },
    Emphasis {
        content: Vec<Inline>,
    },
    Strikethrough {
        content: Vec<Inline>,
    },
    Link {
        url: String,
        title: Option<String>,
        text: Vec<Inline>,
    },
}

impl MarkdownConverter {
    fn new() -> Self {
        Self {
            document: Document::new(),
            block_stack: Vec::new(),
            inline_stack: Vec::new(),
            current_inlines: Vec::new(),
        }
    }

    fn process_events(&mut self, parser: Parser) {
        for event in parser {
            match event {
                Event::Start(tag) => self.handle_start_tag(tag),
                Event::End(tag_end) => self.handle_end_tag(tag_end),
                Event::Text(text) => self.handle_text(text.as_ref()),
                Event::Code(code) => self.handle_code(code.as_ref()),
                Event::SoftBreak => self.current_inlines.push(Inline::SoftBreak),
                Event::HardBreak => self.current_inlines.push(Inline::LineBreak),
                Event::Rule => self.document.blocks.push(Block::HorizontalRule),
                Event::Html(_) | Event::InlineHtml(_) => {
                    // Skip raw HTML for now (could support in future)
                }
                Event::FootnoteReference(_)
                | Event::TaskListMarker(_)
                | Event::InlineMath(_)
                | Event::DisplayMath(_) => {
                    // Skip for now
                }
            }
        }
    }

    fn handle_start_tag(&mut self, tag: Tag) {
        match tag {
            Tag::Paragraph => {
                self.block_stack.push(BlockContext::Paragraph);
            }
            Tag::Heading { level, .. } => {
                let level = match level {
                    HeadingLevel::H1 => 1,
                    HeadingLevel::H2 => 2,
                    HeadingLevel::H3 => 3,
                    HeadingLevel::H4 => 4,
                    HeadingLevel::H5 => 5,
                    HeadingLevel::H6 => 6,
                };
                self.block_stack.push(BlockContext::Heading { level });
            }
            Tag::BlockQuote(_) => {
                self.block_stack
                    .push(BlockContext::BlockQuote { blocks: Vec::new() });
            }
            Tag::CodeBlock(kind) => {
                let lang = match kind {
                    pulldown_cmark::CodeBlockKind::Fenced(lang) => {
                        if lang.is_empty() {
                            None
                        } else {
                            Some(lang.to_string())
                        }
                    }
                    pulldown_cmark::CodeBlockKind::Indented => None,
                };
                self.block_stack.push(BlockContext::CodeBlock {
                    lang,
                    code: String::new(),
                });
            }
            Tag::List(start) => {
                // If we're starting a list inside a ListItem and have accumulated text,
                // wrap it in a paragraph first. This handles tight lists where pulldown-cmark
                // doesn't emit paragraph boundaries.
                if self.block_stack.iter().any(|ctx| matches!(ctx, BlockContext::ListItem { .. })) {
                    let content = std::mem::take(&mut self.current_inlines);
                    if !content.is_empty() {
                        self.push_block(Block::Paragraph { content });
                    }
                }

                let (ordered, start_num) = match start {
                    Some(n) => (true, n as usize),
                    None => (false, 1),
                };
                self.block_stack.push(BlockContext::List {
                    ordered,
                    start: start_num,
                    items: Vec::new(),
                });
            }
            Tag::Item => {
                self.block_stack.push(BlockContext::ListItem {
                    blocks: Vec::new(),
                    task: None,
                });
            }
            Tag::Table(alignments) => {
                let alignment = alignments
                    .iter()
                    .map(|a| match a {
                        CMarkAlignment::Left => Alignment::Left,
                        CMarkAlignment::Center => Alignment::Center,
                        CMarkAlignment::Right => Alignment::Right,
                        CMarkAlignment::None => Alignment::None,
                    })
                    .collect();
                self.block_stack.push(BlockContext::Table {
                    headers: Vec::new(),
                    rows: Vec::new(),
                    current_row: Vec::new(),
                    alignment,
                });
            }
            Tag::TableHead => {
                self.block_stack.push(BlockContext::TableHead);
            }
            Tag::TableRow => {
                self.block_stack.push(BlockContext::TableRow);
            }
            Tag::TableCell => {
                self.block_stack.push(BlockContext::TableCell);
            }
            Tag::Strong => {
                let saved = std::mem::take(&mut self.current_inlines);
                self.inline_stack
                    .push(InlineContext::Strong { content: saved });
            }
            Tag::Emphasis => {
                let saved = std::mem::take(&mut self.current_inlines);
                self.inline_stack
                    .push(InlineContext::Emphasis { content: saved });
            }
            Tag::Strikethrough => {
                let saved = std::mem::take(&mut self.current_inlines);
                self.inline_stack
                    .push(InlineContext::Strikethrough { content: saved });
            }
            Tag::Link {
                dest_url, title, ..
            } => {
                let url = dest_url.to_string();
                let title = if title.is_empty() {
                    None
                } else {
                    Some(title.to_string())
                };
                let saved = std::mem::take(&mut self.current_inlines);
                self.inline_stack.push(InlineContext::Link {
                    url,
                    title,
                    text: saved,
                });
            }
            Tag::Image {
                dest_url, title, ..
            } => {
                let url = dest_url.to_string();
                let title = if title.is_empty() {
                    None
                } else {
                    Some(title.to_string())
                };
                // Images are self-closing, we'll handle them in End event
                let saved = std::mem::take(&mut self.current_inlines);
                self.inline_stack.push(InlineContext::Link {
                    url,
                    title,
                    text: saved,
                }); // Temp use Link context
            }
            Tag::FootnoteDefinition(_) | Tag::HtmlBlock | Tag::MetadataBlock(_) => {
                // Skip for now
            }
            Tag::DefinitionList | Tag::DefinitionListTitle | Tag::DefinitionListDefinition => {
                // Skip definition lists for now (not in v1 scope)
            }
        }
    }

    fn handle_end_tag(&mut self, tag_end: TagEnd) {
        match tag_end {
            TagEnd::Paragraph => {
                if let Some(BlockContext::Paragraph) = self.block_stack.pop() {
                    let content = std::mem::take(&mut self.current_inlines);
                    if !content.is_empty() {
                        self.push_block(Block::Paragraph { content });
                    }
                }
            }
            TagEnd::Heading(_) => {
                if let Some(BlockContext::Heading { level }) = self.block_stack.pop() {
                    let content = std::mem::take(&mut self.current_inlines);
                    self.push_block(Block::Heading { level, content });
                }
            }
            TagEnd::BlockQuote(_) => {
                if let Some(BlockContext::BlockQuote { blocks }) = self.block_stack.pop() {
                    // Check if this is a GitHub-style callout
                    if let Some(callout) = Self::try_parse_callout(&blocks) {
                        self.push_block(callout);
                    } else {
                        self.push_block(Block::BlockQuote { blocks });
                    }
                }
            }
            TagEnd::CodeBlock => {
                if let Some(BlockContext::CodeBlock { lang, code }) = self.block_stack.pop() {
                    self.push_block(Block::CodeBlock { lang, code });
                }
            }
            TagEnd::List(_) => {
                if let Some(BlockContext::List {
                    ordered,
                    start,
                    mut items,
                }) = self.block_stack.pop()
                {
                    // Fix pulldown-cmark quirk: when markdown has "- Label:\n  - nested",
                    // it merges "Label:" into the first nested item instead of keeping
                    // it in the parent. Detect and fix this pattern.
                    fix_merged_list_labels(&mut items);

                    self.push_block(Block::List {
                        ordered,
                        start,
                        items,
                    });
                }
            }
            TagEnd::Item => {
                // For tight lists, wrap any accumulated inlines in a paragraph
                let content = std::mem::take(&mut self.current_inlines);
                if !content.is_empty() {
                    self.push_block(Block::Paragraph { content });
                }

                if let Some(BlockContext::ListItem { blocks, task }) = self.block_stack.pop() {
                    if let Some(BlockContext::List { items, .. }) = self.block_stack.last_mut() {
                        items.push(ListItem {
                            content: blocks,
                            task,
                        });
                    }
                }
            }
            TagEnd::Table => {
                if let Some(BlockContext::Table {
                    headers,
                    rows,
                    alignment,
                    ..
                }) = self.block_stack.pop()
                {
                    self.push_block(Block::Table {
                        headers,
                        rows,
                        alignment,
                    });
                }
            }
            TagEnd::TableHead => {
                if let Some(BlockContext::TableHead) = self.block_stack.pop() {
                    if let Some(BlockContext::Table {
                        current_row,
                        headers,
                        ..
                    }) = self.block_stack.last_mut()
                    {
                        *headers = std::mem::take(current_row);
                    }
                }
            }
            TagEnd::TableRow => {
                if let Some(BlockContext::TableRow) = self.block_stack.pop() {
                    // Check if we're inside TableHead - if so, don't add to rows yet
                    // TableHead will handle moving current_row to headers
                    let in_table_head = self
                        .block_stack
                        .iter()
                        .any(|ctx| matches!(ctx, BlockContext::TableHead));
                    if !in_table_head {
                        if let Some(BlockContext::Table {
                            current_row, rows, ..
                        }) = self.block_stack.last_mut()
                        {
                            rows.push(std::mem::take(current_row));
                        }
                    }
                }
            }
            TagEnd::TableCell => {
                if let Some(BlockContext::TableCell) = self.block_stack.pop() {
                    let content = std::mem::take(&mut self.current_inlines);
                    // Find the Table context in the stack
                    for ctx in self.block_stack.iter_mut().rev() {
                        if let BlockContext::Table { current_row, .. } = ctx {
                            current_row.push(TableCell { content });
                            break;
                        }
                    }
                }
            }
            TagEnd::Strong => {
                if let Some(InlineContext::Strong { mut content }) = self.inline_stack.pop() {
                    let nested = std::mem::take(&mut self.current_inlines);
                    content.push(Inline::Strong(nested));
                    self.current_inlines = content;
                }
            }
            TagEnd::Emphasis => {
                if let Some(InlineContext::Emphasis { mut content }) = self.inline_stack.pop() {
                    let nested = std::mem::take(&mut self.current_inlines);
                    content.push(Inline::Emphasis(nested));
                    self.current_inlines = content;
                }
            }
            TagEnd::Strikethrough => {
                if let Some(InlineContext::Strikethrough { mut content }) = self.inline_stack.pop()
                {
                    let nested = std::mem::take(&mut self.current_inlines);
                    content.push(Inline::Strikethrough(nested));
                    self.current_inlines = content;
                }
            }
            TagEnd::Link => {
                if let Some(InlineContext::Link {
                    url,
                    title,
                    mut text,
                }) = self.inline_stack.pop()
                {
                    let nested = std::mem::take(&mut self.current_inlines);
                    text.push(Inline::Link {
                        url,
                        title,
                        text: nested,
                    });
                    self.current_inlines = text;
                }
            }
            TagEnd::Image => {
                if let Some(InlineContext::Link {
                    url,
                    title,
                    mut text,
                }) = self.inline_stack.pop()
                {
                    let nested = std::mem::take(&mut self.current_inlines);
                    let alt = nested.iter().map(|i| i.to_plain_text()).collect();
                    text.push(Inline::Image { url, alt, title });
                    self.current_inlines = text;
                }
            }
            TagEnd::FootnoteDefinition | TagEnd::HtmlBlock | TagEnd::MetadataBlock(_) => {
                // Skip for now
            }
            TagEnd::DefinitionList
            | TagEnd::DefinitionListTitle
            | TagEnd::DefinitionListDefinition => {
                // Skip definition lists for now
            }
        }
    }

    fn handle_text(&mut self, text: &str) {
        if let Some(BlockContext::CodeBlock { code, .. }) = self.block_stack.last_mut() {
            code.push_str(text);
        } else {
            self.current_inlines.push(Inline::Text(text.to_string()));
        }
    }

    fn handle_code(&mut self, code: &str) {
        self.current_inlines.push(Inline::Code(code.to_string()));
    }

    fn push_block(&mut self, block: Block) {
        // Check if we're inside a nested block context
        if let Some(ctx) = self.block_stack.last_mut() {
            match ctx {
                BlockContext::BlockQuote { blocks } => blocks.push(block),
                BlockContext::ListItem { blocks, .. } => blocks.push(block),
                _ => self.document.blocks.push(block),
            }
        } else {
            self.document.blocks.push(block);
        }
    }

    /// Try to parse a blockquote as a GitHub-style callout
    /// Syntax: > [!NOTE] or > [!WARNING] etc.
    fn try_parse_callout(blocks: &[Block]) -> Option<Block> {
        if blocks.is_empty() {
            return None;
        }

        // Check if first block is a paragraph starting with [!TYPE]
        if let Block::Paragraph { content } = &blocks[0] {
            if let Some(Inline::Text(text)) = content.first() {
                let trimmed = text.trim_start();
                if trimmed.starts_with("[!") {
                    // Extract the callout type
                    if let Some(end_idx) = trimmed.find(']') {
                        let type_str = &trimmed[2..end_idx];
                        let kind = match type_str.to_uppercase().as_str() {
                            "NOTE" => CalloutKind::Note,
                            "WARNING" => CalloutKind::Warning,
                            "IMPORTANT" => CalloutKind::Important,
                            "TIP" => CalloutKind::Tip,
                            "CAUTION" => CalloutKind::Caution,
                            _ => return None, // Unknown callout type
                        };

                        // Remove the [!TYPE] marker from the first paragraph
                        let mut remaining_blocks = blocks.to_vec();
                        if let Block::Paragraph { content } = &mut remaining_blocks[0] {
                            if let Some(Inline::Text(text)) = content.first_mut() {
                                // Remove [!TYPE] and any following whitespace
                                let after_marker = &trimmed[end_idx + 1..].trim_start();
                                *text = after_marker.to_string();

                                // If the text is now empty, remove it
                                if text.is_empty() {
                                    content.remove(0);
                                }

                                // If paragraph is now empty, remove it
                                if content.is_empty() {
                                    remaining_blocks.remove(0);
                                }
                            }
                        }

                        return Some(Block::Callout {
                            kind,
                            title: None, // Could extract from after [!TYPE] if desired
                            content: remaining_blocks,
                        });
                    }
                }
            }
        }

        None
    }

    fn finish(self) -> Document {
        self.document
    }
}

/// Fix pulldown-cmark's incorrect parsing of nested lists with labels.
///
/// When markdown has:
/// ```markdown
/// - **Label:**
///   - Nested item
/// ```
///
/// pulldown-cmark merges "Label:" into the first nested item's paragraph,
/// creating a structure where the parent ListItem contains only a nested List,
/// and the first nested ListItem contains both "Label:" and "Nested item" text.
///
/// This function detects that pattern and restructures it so "Label:" becomes
/// a paragraph in the parent ListItem.
fn fix_merged_list_labels(items: &mut Vec<ListItem>) {
    // First pass: fix merged labels and recursively process nested lists
    for item in items.iter_mut() {
        // Check if this item has only one child block, and it's a List
        if item.content.len() == 1 {
            // First, check if we need to extract a label and get the inline content
            let extracted_label = if let Block::List { items: nested_items, .. } = &mut item.content[0] {
                if let Some(first_nested) = nested_items.first_mut() {
                    if let Some(Block::Paragraph { content }) = first_nested.content.first_mut() {
                        // Check if paragraph starts with multiple inline elements (indicating merge)
                        // When pulldown-cmark merges parent text into first child, the paragraph
                        // will have 2+ inline elements (parent + child content)
                        if content.len() >= 2 {
                            // Extract the first inline element as the parent's content
                            // This handles:
                            // - Plain text: "Parent item"
                            // - Strong text: "**Label:**"
                            // - Links: "[Getting Started](#link)"
                            // - Any other inline content
                            Some(content.remove(0))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };

            // If we extracted a label, add it as a paragraph to the parent
            if let Some(prefix) = extracted_label {
                item.content.insert(
                    0,
                    Block::Paragraph {
                        content: vec![prefix],
                    },
                );

                // Clean up empty paragraphs in the first nested item
                if let Block::List { items: nested_items, .. } = &mut item.content[1] {
                    if let Some(first_nested) = nested_items.first_mut() {
                        if let Some(Block::Paragraph { content }) = first_nested.content.first() {
                            if content.is_empty() {
                                first_nested.content.remove(0);
                            }
                        }
                    }
                }
            }
        }

        // Recursively fix any nested lists in this item's content
        for block in &mut item.content {
            if let Block::List { items: nested_items, .. } = block {
                fix_merged_list_labels(nested_items);
            }
        }
    }

    // Second pass: flatten empty parent items (items that contain ONLY a nested list)
    // This removes unnecessary nesting levels created by pulldown-cmark
    let mut i = 0;
    while i < items.len() {
        let should_flatten = items[i].content.len() == 1
            && matches!(items[i].content[0], Block::List { .. });

        if should_flatten {
            // Extract the nested list
            if let Block::List { items: mut nested_items, .. } = items.remove(i).content.into_iter().next().unwrap() {
                // Recursively flatten the nested items before inserting
                fix_merged_list_labels(&mut nested_items);

                // Insert all nested items at the current position
                for (offset, nested_item) in nested_items.into_iter().enumerate() {
                    items.insert(i + offset, nested_item);
                }
            }
        } else {
            i += 1;
        }
    }
}
