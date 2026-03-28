//! Core document types and structures.
//!
//! This module provides the fundamental types for representing documents,
//! including the main document structure, block and inline elements,
//! and metadata.

use std::collections::HashMap;

/// A complete Lontar document.
///
/// This is the root structure of the AST that represents an entire document
/// with all its content, metadata, and resources. Documents can be converted
/// to various formats (DOCX, PDF, HTML, etc.) through the `DocumentWriter` trait.
///
/// # Fields
///
/// * `metadata` - Document metadata like title, author, creation date
/// * `page_setup` - Page layout settings (size, margins, orientation)
/// * `stylesheet` - Style definitions and cascading resolution rules
/// * `content` - Ordered list of block-level elements (headings, paragraphs, etc.)
/// * `bibliography` - Optional bibliography store for citations and references
/// * `resources` - Binary resources like images and embedded files
/// * `crossrefs` - Registry for cross-reference labels and their resolutions
///
/// # Examples
///
/// ```rust
/// use lontar::core::*;
///
/// let doc = Document {
///     metadata: DocumentMetadata {
///         title: Some("My Document".to_string()),
///         ..Default::default()
///     },
///     page_setup: PageSetup::default(),
///     stylesheet: StyleSheet::default(),
///     content: vec![
///         Block::Heading {
///             level: 1,
///             content: vec![Inline::Text("Introduction".to_string())],
///             style: None,
///         },
///         Block::Paragraph {
///             content: vec![Inline::Text("Hello, world!".to_string())],
///             style: None,
///         },
///     ],
///     bibliography: None,
///     resources: ResourceStore::default(),
///     crossrefs: CrossRefRegistry::default(),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Document {
    pub metadata: DocumentMetadata,
    pub page_setup: PageSetup,
    pub stylesheet: StyleSheet,
    pub content: Vec<Block>,
    pub bibliography: Option<BibliographyStore>,
    pub resources: ResourceStore,
    pub crossrefs: CrossRefRegistry,
}

/// Document metadata (title, author, creation date, etc.).
///
/// Contains descriptive information about the document that may be used by
/// different backends for headers, footers, document properties, or other
/// metadata fields. All fields are optional.
///
/// # Fields
///
/// * `title` - Document title
/// * `author` - Document author or authors
/// * `subject` - Subject or topic of the document
/// * `keywords` - List of keywords for indexing/searching
/// * `created` - Creation date/time (ISO 8601 format recommended)
/// * `modified` - Last modification date/time
/// * `language` - Document language (BCP 47 language code)
///
/// # Examples
///
/// ```rust
/// use lontar::core::*;
///
/// let metadata = DocumentMetadata {
///     title: Some("Research Paper".to_string()),
///     author: Some("Dr. Jane Smith".to_string()),
///     subject: Some("Machine Learning".to_string()),
///     keywords: vec![
///         "machine learning".to_string(),
///         "neural networks".to_string(),
///         "deep learning".to_string(),
///     ],
///     created: Some("2024-01-15T10:30:00Z".to_string()),
///     language: Some("en-US".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default)]
pub struct DocumentMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub keywords: Vec<String>,
    pub created: Option<String>,
    pub modified: Option<String>,
    pub language: Option<String>,
}

/// Page setup (size, margins, orientation).
///
/// Defines the physical layout of pages in the document. All measurements are
/// specified in points (1/72 inch). This structure is used by backends that
/// support page-based layouts like DOCX, PDF, and LaTeX.
///
/// # Fields
///
/// * `page_width` - Page width in points
/// * `page_height` - Page height in points
/// * `margins` - Page margins (top, bottom, left, right)
/// * `orientation` - Portrait or landscape orientation
///
/// # Examples
///
/// ```rust
/// use lontar::core::*;
///
/// // A4 paper in portrait orientation with 1-inch margins
/// let page_setup = PageSetup {
///     page_width: 595.28,  // A4 width in points
///     page_height: 841.89, // A4 height in points
///     margins: Margins {
///         top: 72.0,    // 1 inch
///         bottom: 72.0, // 1 inch
///         left: 72.0,   // 1 inch
///         right: 72.0,  // 1 inch
///     },
///     orientation: PageOrientation::Portrait,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct PageSetup {
    pub page_width: f32,  // in points
    pub page_height: f32, // in points
    pub margins: Margins,
    pub orientation: PageOrientation,
}

/// Page orientation options.
///
/// Defines whether pages are laid out in portrait (taller than wide) or
/// landscape (wider than tall) orientation.
///
/// # Variants
///
/// * `Portrait` - Standard orientation, height > width
/// * `Landscape` - Rotated orientation, width > height
///
/// # Examples
///
/// ```rust
/// use lontar::core::*;
///
/// let orientation = PageOrientation::Landscape;
/// ```
#[derive(Debug, Clone, Copy)]
pub enum PageOrientation {
    Portrait,
    Landscape,
}

/// Margins (top, bottom, left, right).
///
/// Defines the page margins in points. All measurements are from the edge
/// of the physical page to the content area.
///
/// # Fields
///
/// * `top` - Top margin in points
/// * `bottom` - Bottom margin in points
/// * `left` - Left margin in points
/// * `right` - Right margin in points
///
/// # Examples
///
/// ```rust
/// use lontar::core::*;
///
/// // 1-inch margins (72 points = 1 inch)
/// let margins = Margins {
///     top: 72.0,
///     bottom: 72.0,
///     left: 72.0,
///     right: 72.0,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Margins {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl Default for PageSetup {
    fn default() -> Self {
        Self {
            page_width: 612.0,  // Letter: 8.5"
            page_height: 792.0, // Letter: 11"
            margins: Margins {
                top: 72.0, // 1"
                bottom: 72.0,
                left: 72.0,
                right: 72.0,
            },
            orientation: PageOrientation::Portrait,
        }
    }
}

/// Top-level block elements (paragraphs, headings, tables, etc.).
///
/// Block elements represent the main structural components of a document.
/// They are ordered sequentially and each block may contain inline elements
/// or other nested blocks (in the case of block quotes and sections).
///
/// # Variants
///
/// * `Heading` - Section headings with levels 1-6
/// * `Paragraph` - Standard text paragraphs
/// * `Table` - Structured data with rows and columns
/// * `Chart` - Data visualizations (bar, line, pie, etc.)
/// * `List` - Ordered or unordered lists with nesting
/// * `CodeBlock` - Preformatted code with syntax highlighting
/// * `BlockQuote` - Quoted text blocks that may contain other blocks
/// * `Image` - Embedded images with sizing and alt text
/// * `HorizontalRule` - Visual separators between content
/// * `Equation` - Mathematical equations (inline or display)
/// * `Section` - Logical document groupings
/// * `Bibliography` - Generated bibliography section
///
/// # Examples
///
/// ```rust
/// use lontar::core::*;
///
/// let block = Block::Heading {
///     level: 1,
///     content: vec![Inline::Text("Introduction".to_string())],
///     style: None,
/// };
/// ```
#[derive(Debug, Clone)]
pub enum Block {
    Heading {
        level: u8,
        content: Vec<Inline>,
        style: Option<String>,
    },
    Paragraph {
        content: Vec<Inline>,
        style: Option<String>,
    },
    Table {
        rows: Vec<TableRow>,
        style: Option<String>,
    },
    Chart {
        title: Option<String>,
        kind: ChartKind,
        data: ChartData,
        style: Option<String>,
    },
    List {
        items: Vec<ListItem>,
        ordered: bool,
        style: Option<String>,
    },
    CodeBlock {
        code: String,
        language: Option<String>,
        style: Option<String>,
    },
    BlockQuote {
        content: Vec<Block>,
        style: Option<String>,
    },
    Image {
        resource_id: String,
        alt_text: Option<String>,
        width: Option<f32>,
        height: Option<f32>,
        style: Option<String>,
    },
    HorizontalRule {
        style: Option<String>,
    },
    Equation {
        content: String,
        display: bool,
        label: Option<String>,
        style: Option<String>,
    },
    Section {
        title: Option<String>,
        content: Vec<Block>,
        style: Option<String>,
    },
    Bibliography {
        style: BibliographyStyle,
        title: Option<String>,
    },
}

/// Table structure.
#[derive(Debug, Clone)]
pub struct TableRow {
    pub cells: Vec<TableCell>,
    pub is_header: bool,
}

#[derive(Debug, Clone)]
pub struct TableCell {
    pub content: Vec<Block>,
    pub colspan: u32,
    pub rowspan: u32,
}

/// List item.
#[derive(Debug, Clone)]
pub struct ListItem {
    pub content: Vec<Block>,
    pub level: u32,
}

/// Inline elements (text, bold, italic, links, citations, etc.).
///
/// Inline elements are contained within block elements and represent
/// text-level formatting and semantics. They can be nested to create
/// complex formatting combinations.
///
/// # Variants
///
/// * `Text` - Plain text content
/// * `SoftBreak` - Soft line break (word wrapping opportunity)
/// * `LineBreak` - Hard line break (forced line break)
/// * `Bold` - Bold text
/// * `Italic` - Italic text
/// * `Code` - Inline code with monospace font
/// * `Strikethrough` - Text with strikethrough decoration
/// * `Subscript` - Subscript text
/// * `Superscript` - Superscript text
/// * `Link` - Clickable hyperlinks
/// * `Image` - Inline images
/// * `Citation` - Bibliography citations with various modes
/// * `CrossRef` - Cross-references to labels
/// * `Styled` - Text with custom styling
///
/// # Examples
///
/// ```rust
/// use lontar::core::*;
///
/// let inline = Inline::Bold(vec![
///     Inline::Text("Important".to_string()),
///     Inline::Italic(vec![Inline::Text("text".to_string())]),
/// ]);
/// ```
#[derive(Debug, Clone)]
pub enum Inline {
    Text(String),
    SoftBreak,
    LineBreak,
    Bold(Vec<Inline>),
    Italic(Vec<Inline>),
    Code(String),
    Strikethrough(Vec<Inline>),
    Subscript(Vec<Inline>),
    Superscript(Vec<Inline>),
    Link { text: Vec<Inline>, url: String, title: Option<String> },
    Image { resource_id: String, alt_text: Option<String>, title: Option<String> },
    Citation { key: String, mode: CitationMode, prefix: Option<String>, suffix: Option<String> },
    CrossRef { label: String, kind: CrossRefKind },
    Styled { content: Vec<Inline>, style: String },
}

/// Resource store for images and binary assets.
///
/// Manages binary resources like images, fonts, and other embedded files
/// that can be referenced by ID from other parts of the document.
#[derive(Debug, Clone, Default)]
pub struct ResourceStore {
    pub resources: HashMap<String, Resource>,
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub id: String,
    pub mime_type: String,
    pub data: Vec<u8>,
}

impl ResourceStore {
    /// Add a resource (image, etc.).
    pub fn add_resource(&mut self, id: String, mime_type: String, data: Vec<u8>) {
        self.resources.insert(id.clone(), Resource { id, mime_type, data });
    }

    /// Get a resource by ID.
    pub fn get(&self, id: &str) -> Option<&Resource> {
        self.resources.get(id)
    }

    /// Check if a resource exists.
    pub fn contains(&self, id: &str) -> bool {
        self.resources.contains_key(id)
    }
}

// Re-export types from other modules
pub use super::bibliography::*;
pub use super::chart::*;
pub use super::crossref::*;
pub use super::style::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = Document {
            metadata: DocumentMetadata {
                title: Some("Test Document".to_string()),
                ..Default::default()
            },
            page_setup: PageSetup::default(),
            stylesheet: StyleSheet::default(),
            content: vec![],
            bibliography: None,
            resources: ResourceStore::default(),
            crossrefs: CrossRefRegistry::default(),
        };

        assert_eq!(doc.metadata.title, Some("Test Document".to_string()));
    }

    #[test]
    fn test_resource_store() {
        let mut store = ResourceStore::default();
        store.add_resource("image1".to_string(), "image/png".to_string(), vec![1, 2, 3, 4]);

        assert!(store.contains("image1"));
        assert_eq!(store.get("image1").unwrap().mime_type, "image/png");
    }

    #[test]
    fn test_page_setup_default() {
        let setup = PageSetup::default();
        assert_eq!(setup.page_width, 612.0);
        assert_eq!(setup.page_height, 792.0);
        assert_eq!(setup.margins.top, 72.0);
        assert_eq!(setup.margins.bottom, 72.0);
        assert_eq!(setup.margins.left, 72.0);
        assert_eq!(setup.margins.right, 72.0);
        assert!(matches!(setup.orientation, PageOrientation::Portrait));
    }

    #[test]
    fn test_document_metadata_default() {
        let metadata = DocumentMetadata::default();
        assert_eq!(metadata.title, None);
        assert_eq!(metadata.author, None);
        assert!(metadata.keywords.is_empty());
    }

    #[test]
    fn test_margins_creation() {
        let margins = Margins { top: 36.0, bottom: 36.0, left: 54.0, right: 54.0 };

        assert_eq!(margins.top, 36.0);
        assert_eq!(margins.bottom, 36.0);
        assert_eq!(margins.left, 54.0);
        assert_eq!(margins.right, 54.0);
    }

    #[test]
    fn test_table_row_creation() {
        let row = TableRow {
            cells: vec![TableCell { content: vec![], colspan: 1, rowspan: 1 }],
            is_header: true,
        };

        assert_eq!(row.cells.len(), 1);
        assert!(row.is_header);
        assert_eq!(row.cells[0].colspan, 1);
        assert_eq!(row.cells[0].rowspan, 1);
    }

    #[test]
    fn test_list_item_creation() {
        let item = ListItem { content: vec![], level: 2 };

        assert_eq!(item.level, 2);
        assert!(item.content.is_empty());
    }

    #[test]
    fn test_inline_text_creation() {
        let text = Inline::Text("Hello, world!".to_string());
        if let Inline::Text(content) = text {
            assert_eq!(content, "Hello, world!");
        } else {
            panic!("Expected Text variant");
        }
    }

    #[test]
    fn test_inline_bold_creation() {
        let bold = Inline::Bold(vec![
            Inline::Text("Bold".to_string()),
            Inline::Italic(vec![Inline::Text("text".to_string())]),
        ]);

        if let Inline::Bold(content) = bold {
            assert_eq!(content.len(), 2);
            assert!(matches!(content[0], Inline::Text(_)));
            assert!(matches!(content[1], Inline::Italic(_)));
        } else {
            panic!("Expected Bold variant");
        }
    }

    #[test]
    fn test_resource_creation() {
        let resource = Resource {
            id: "test".to_string(),
            mime_type: "text/plain".to_string(),
            data: vec![72, 101, 108, 108, 111], // "Hello" in ASCII
        };

        assert_eq!(resource.id, "test");
        assert_eq!(resource.mime_type, "text/plain");
        assert_eq!(resource.data, vec![72, 101, 108, 108, 111]);
    }
}
