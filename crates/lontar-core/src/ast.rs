//! Document AST (Abstract Syntax Tree) types.
//!
//! The AST is the heart of Lontar. Every format backend consumes
//! this same tree structure.

use crate::resource::ResourceStore;
use crate::style::{ParagraphStyle, StyleSheet, TableStyle, TextStyle};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Root document node
#[derive(Debug, Clone)]
pub struct Document {
    pub metadata: DocumentMetadata,
    pub content: Vec<Block>,
    pub styles: StyleSheet,
    pub resources: ResourceStore,
}

/// Document metadata (title, author, creation date, etc.)
#[derive(Debug, Clone, Default)]
pub struct DocumentMetadata {
    pub title: String,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub created: Option<DateTime<Utc>>,
    pub modified: Option<DateTime<Utc>>,
    pub language: Option<String>,
    pub custom: HashMap<String, String>,
}

/// Block-level content (paragraphs, headings, tables, etc.)
#[derive(Debug, Clone)]
pub enum Block {
    /// Section heading (level 1-6)
    Heading {
        level: u8,
        content: Vec<Inline>,
        id: Option<String>,
    },

    /// Body paragraph
    Paragraph {
        content: Vec<Inline>,
        style: Option<ParagraphStyle>,
    },

    /// Data table
    Table {
        headers: Option<Vec<Cell>>,
        rows: Vec<Vec<Cell>>,
        style: Option<TableStyle>,
        caption: Option<Vec<Inline>>,
    },

    /// Embedded image
    Image {
        resource_id: String,
        alt_text: Option<String>,
        width: Option<f64>,
        height: Option<f64>,
    },

    /// Bulleted or numbered list
    List {
        ordered: bool,
        items: Vec<ListItem>,
    },

    /// Code block
    CodeBlock {
        code: String,
        language: Option<String>,
    },

    /// Block quote
    BlockQuote {
        content: Vec<Block>,
    },

    /// Hard page break
    PageBreak,

    /// Horizontal rule
    HorizontalRule,
}

/// List item
#[derive(Debug, Clone)]
pub struct ListItem {
    pub content: Vec<Inline>,
    pub nested_items: Vec<ListItem>,
}

/// Table cell
#[derive(Debug, Clone)]
pub struct Cell {
    pub content: Vec<Block>,
    pub colspan: Option<usize>,
    pub rowspan: Option<usize>,
}

/// Inline content (text, bold, italic, links, etc.)
#[derive(Debug, Clone)]
pub enum Inline {
    /// Plain text
    Text {
        content: String,
        style: Option<TextStyle>,
    },

    /// Hyperlink
    Link {
        text: Vec<Inline>,
        url: String,
    },

    /// Line break
    LineBreak,

    /// Soft space (non-breaking)
    SoftSpace,

    /// Tab character
    Tab,
}

impl Document {
    /// Create a new document with the given title
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            metadata: DocumentMetadata {
                title: title.into(),
                created: Some(Utc::now()),
                modified: Some(Utc::now()),
                ..Default::default()
            },
            content: Vec::new(),
            styles: StyleSheet::new(),
            resources: ResourceStore::new(),
        }
    }

    /// Add a block to the document
    pub fn add_block(&mut self, block: Block) {
        self.content.push(block);
    }

    /// Add multiple blocks
    pub fn add_blocks(&mut self, blocks: Vec<Block>) {
        self.content.extend(blocks);
    }
}

impl Block {
    /// Create a heading block
    pub fn heading(level: u8, content: Vec<Inline>) -> Self {
        assert!((1..=6).contains(&level), "Heading level must be 1-6");
        Self::Heading {
            level,
            content,
            id: None,
        }
    }

    /// Create a paragraph block
    pub fn paragraph(content: Vec<Inline>) -> Self {
        Self::Paragraph {
            content,
            style: None,
        }
    }

    /// Create a table block
    pub fn table(headers: Option<Vec<Cell>>, rows: Vec<Vec<Cell>>) -> Self {
        Self::Table {
            headers,
            rows,
            style: None,
            caption: None,
        }
    }

    /// Create an image block
    pub fn image(resource_id: String) -> Self {
        Self::Image {
            resource_id,
            alt_text: None,
            width: None,
            height: None,
        }
    }

    /// Create a list block
    pub fn list(ordered: bool, items: Vec<ListItem>) -> Self {
        Self::List { ordered, items }
    }

    /// Create a code block
    pub fn code_block(code: String, language: Option<String>) -> Self {
        Self::CodeBlock { code, language }
    }

    /// Create a block quote
    pub fn block_quote(content: Vec<Block>) -> Self {
        Self::BlockQuote { content }
    }
}

impl Inline {
    /// Create a text inline
    pub fn text(content: impl Into<String>) -> Self {
        Self::Text {
            content: content.into(),
            style: None,
        }
    }

    /// Create a text inline with style
    pub fn styled_text(content: impl Into<String>, style: TextStyle) -> Self {
        Self::Text {
            content: content.into(),
            style: Some(style),
        }
    }

    /// Create a link inline
    pub fn link(text: Vec<Inline>, url: impl Into<String>) -> Self {
        Self::Link {
            text,
            url: url.into(),
        }
    }

    /// Create a link from plain text
    pub fn text_link(text: impl Into<String>, url: impl Into<String>) -> Self {
        Self::Link {
            text: vec![Self::text(text)],
            url: url.into(),
        }
    }
}
