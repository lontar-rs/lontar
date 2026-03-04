//! Document AST (Abstract Syntax Tree) types.
//!
//! The AST is the heart of Lontar. Every format backend consumes
//! this same tree structure.

use crate::resource::ResourceStore;
use crate::style::{ParagraphStyle, StyleSheet, TableStyle, TextStyle};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Chart visualization type.
///
/// Represents different kinds of charts that can be embedded in documents.
/// Charts are rendered as images or tables depending on the output format.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChartKind {
    Bar,
    Line,
    Pie,
    Scatter,
}

/// Cross-reference target metadata.
///
/// Stores information about a labeled element (equation, figure, section) that can be
/// referenced from elsewhere in the document. Each target has a unique label, an auto-incremented
/// number, and optional title text.
#[derive(Debug, Clone)]
pub struct CrossRefTarget {
    pub label: String,
    pub number: usize,
    pub title: Option<String>,
}

/// Chart data for visualization.
///
/// Contains the data needed to render a chart, including axis labels and datasets.
/// This is a simplified structure for Phase 1; production versions may include
/// additional properties like colors, legends, and axis ranges.
#[derive(Debug, Clone)]
pub struct ChartData {
    pub labels: Vec<String>,
    pub datasets: Vec<Dataset>,
}

/// A single dataset within a chart.
///
/// Contains a label and numeric values for visualization.
#[derive(Debug, Clone)]
pub struct Dataset {
    pub label: String,
    pub values: Vec<f64>,
}

/// Bibliography rendering style.
///
/// Determines how citations and bibliographies are formatted in the output.
/// Different styles are appropriate for different academic disciplines:
/// - `Numeric`: Used in engineering and sciences (e.g., IEEE)
/// - `AuthorYear`: Used in social sciences and humanities (e.g., Harvard)
/// - `Vancouver`: Used in medical and life sciences
/// - `Superscript`: Numeric citations rendered as superscript
/// - `Apa7`: American Psychological Association 7th edition
/// - `Named`: Custom style from a CSL file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BibliographyStyle {
    /// Numeric citations [1], [2], etc.
    Numeric,
    /// Author-year citations (Smith, 2024)
    AuthorYear,
    /// Vancouver style (medical journals)
    Vancouver,
    /// Superscript citations ¹, ², etc.
    Superscript,
    /// APA 7th edition
    Apa7,
    /// Named style from CSL file
    Named(u32), // Index into style registry
}

/// Citation rendering mode.
///
/// Controls how a citation is formatted when rendered inline:
/// - `Parenthetical`: (Author, Year) or [1]
/// - `Narrative`: Author (Year) or Author [1]
/// - `YearOnly`: Year only, without author
/// - `SuppressAuthor`: Year only, suppressing author name
/// - `Full`: Complete citation with all details
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CitationMode {
    /// (Author, Year) or [1]
    Parenthetical,
    /// Author (Year) or Author [1]
    Narrative,
    /// (Year) only
    YearOnly,
    /// Suppress author, show year only
    SuppressAuthor,
    /// Full citation inline
    Full,
}

/// Cross-reference rendering kind.
///
/// Determines what information is displayed when rendering a cross-reference:
/// - `Auto`: Auto-detect based on target type (number for equations, title for sections)
/// - `Number`: Show only the number (e.g., "1", "Figure 2")
/// - `Page`: Show the page number where the target appears
/// - `Title`: Show the title or caption of the target
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossRefKind {
    /// Auto-detect (number for equations/figures, title for sections)
    Auto,
    /// Show number only
    Number,
    /// Show page number
    Page,
    /// Show title/caption
    Title,
}

/// Root document node.
///
/// The top-level container for all document content, metadata, styles, and resources.
/// A document is built using `DocumentBuilder` and then passed to a backend's
/// `DocumentWriter` implementation for rendering to a specific format.
///
/// # Example
///
/// ```ignore
/// let doc = DocumentBuilder::new("My Report")
///     .heading(1, "Introduction")
///     .paragraph("Hello, world!")
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Document {
    pub metadata: DocumentMetadata,
    pub content: Vec<Block>,
    pub styles: StyleSheet,
    pub resources: ResourceStore,
    pub bibliography: Option<crate::bibliography::BibliographyStore>,
    pub crossrefs: std::collections::HashMap<String, CrossRefTarget>,
}

/// Document metadata.
///
/// Contains information about the document such as title, author, subject,
/// creation/modification dates, language, and custom metadata fields.
/// This metadata is used by backends to generate document properties and headers.
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

/// Block-level content element.
///
/// Represents major structural elements of a document. Blocks are top-level
/// content containers that can include paragraphs, headings, tables, lists,
/// equations, and more. Each block may contain inline content or nested blocks.
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
    List { ordered: bool, items: Vec<ListItem> },

    /// Code block
    CodeBlock {
        code: String,
        language: Option<String>,
    },

    /// Block quote
    BlockQuote { content: Vec<Block> },

    /// Hard page break
    PageBreak,

    /// Horizontal rule
    HorizontalRule,

    /// Mathematical equation
    Equation {
        latex: String,
        label: Option<String>,
        numbered: bool,
    },

    /// Chart or diagram
    Chart {
        kind: ChartKind,
        data: ChartData,
        caption: Option<Vec<Inline>>,
    },

    /// Bibliography section
    Bibliography { style: BibliographyStyle },
}

/// A single item in an ordered or unordered list.
///
/// Contains inline content and optional nested list items for creating
/// multi-level lists.
#[derive(Debug, Clone)]
pub struct ListItem {
    pub content: Vec<Inline>,
    pub nested_items: Vec<ListItem>,
}

/// A cell within a table row.
///
/// Contains block-level content (typically paragraphs) and supports
/// colspan and rowspan for merged cells.
#[derive(Debug, Clone)]
pub struct Cell {
    pub content: Vec<Block>,
    pub colspan: Option<usize>,
    pub rowspan: Option<usize>,
}

/// Inline content element.
///
/// Represents text-level content that flows within blocks. Inline elements
/// include plain text, styled text, links, line breaks, citations, and equations.
/// Inline elements cannot contain block-level content.
#[derive(Debug, Clone)]
pub enum Inline {
    /// Plain text
    Text {
        content: String,
        style: Option<TextStyle>,
    },

    /// Hyperlink
    Link { text: Vec<Inline>, url: String },

    /// Line break
    LineBreak,

    /// Soft space (non-breaking)
    SoftSpace,

    /// Tab character
    Tab,

    /// Citation reference
    Citation {
        keys: Vec<String>,
        mode: CitationMode,
    },

    /// Cross-reference to a labeled element
    CrossRef { label: String, kind: CrossRefKind },

    /// Inline mathematical equation
    InlineEquation { latex: String },
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
            bibliography: None,
            crossrefs: std::collections::HashMap::new(),
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

    /// Resolve a cross-reference to display text based on kind
    pub fn resolve_crossref(&self, label: &str, kind: CrossRefKind) -> Option<String> {
        let target = self.crossrefs.get(label)?;
        match kind {
            CrossRefKind::Auto | CrossRefKind::Number => Some(target.number.to_string()),
            CrossRefKind::Title => target.title.clone(),
            CrossRefKind::Page => None, // Page numbers not supported in core model
        }
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
