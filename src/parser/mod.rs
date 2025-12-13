//! Markdown parser that converts Markdown to Lumen IR

mod markdown;

pub use markdown::parse_markdown;

#[cfg(test)]
mod tests;
