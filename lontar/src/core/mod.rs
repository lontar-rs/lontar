//! Core AST types and document model.
//!
//! This module defines the fundamental document structure (AST) that all
//! backends consume. The AST is unified across all document types and
//! supports rich semantic markup including citations, cross-references,
//! equations, and bibliographies.

pub mod ast;
pub mod error;
pub mod writer;

pub use ast::{
    Alignment, BibAuthor, BibEntry, BibEntryKind, BibliographyError, BibliographyStore,
    BibliographyStyle, Block, ChartData, ChartKind, ChartSeries, CitationMode, Color,
    CrossRefError, CrossRefKind, CrossRefRegistry, Document, DocumentMetadata, FontStyle,
    FontWeight, Inline, ListItem, Margins, PageOrientation, PageSetup, ParagraphStyle, Resource,
    ResourceStore, Script, StyleSheet, TableCell, TableRow, TableStyle, TextStyle,
};
pub use error::LontarError;
pub use writer::{DocumentWriter, WriteError, WriteReport, WriteResult};

/// Builder for constructing documents ergonomically.
#[derive(Debug, Default)]
pub struct DocumentBuilder {
    metadata: DocumentMetadata,
    page_setup: PageSetup,
    stylesheet: StyleSheet,
    content: Vec<Block>,
    bibliography: Option<BibliographyStore>,
    resources: ResourceStore,
    crossrefs: CrossRefRegistry,
}

impl DocumentBuilder {
    /// Create a new document builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the document title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.metadata.title = Some(title.into());
        self
    }

    /// Set the document author.
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.metadata.author = Some(author.into());
        self
    }

    /// Add a block to the document.
    pub fn block(mut self, block: Block) -> Self {
        self.content.push(block);
        self
    }

    /// Set the bibliography.
    pub fn bibliography(mut self, bib: BibliographyStore) -> Self {
        self.bibliography = Some(bib);
        self
    }

    /// Register a cross-reference label, returning its assigned number.
    pub fn register_label(mut self, label: impl Into<String>) -> Result<Self, CrossRefError> {
        self.crossrefs.register(label)?;
        Ok(self)
    }

    /// Resolve a cross-reference label to its assigned number.
    pub fn resolve_label(&self, label: &str) -> Result<String, CrossRefError> {
        self.crossrefs.resolve(label)
    }

    /// Build the document.
    pub fn build(self) -> Document {
        Document {
            metadata: self.metadata,
            page_setup: self.page_setup,
            stylesheet: self.stylesheet,
            content: self.content,
            bibliography: self.bibliography,
            resources: self.resources,
            crossrefs: self.crossrefs,
        }
    }
}
