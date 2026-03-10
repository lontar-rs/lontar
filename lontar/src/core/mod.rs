//! Core AST types and document model.
//!
//! This module defines the fundamental document structure (AST) that all
//! backends consume. The AST is unified across all document types and
//! supports rich semantic markup including citations, cross-references,
//! equations, and bibliographies.

pub mod ast;

pub use ast::{
    Alignment, BibAuthor, BibEntry, BibEntryKind, BibliographyStore, BibliographyStyle, Block,
    CitationMode, Color, CrossRefKind, Document, DocumentMetadata, FontStyle, FontWeight, Inline,
    ListItem, Margins, PageOrientation, PageSetup, ParagraphStyle, Resource, ResourceStore, Script,
    StyleSheet, TableCell, TableRow, TableStyle, TextStyle,
};

/// Builder for constructing documents ergonomically.
#[derive(Debug, Default)]
pub struct DocumentBuilder {
    metadata: DocumentMetadata,
    page_setup: PageSetup,
    stylesheet: StyleSheet,
    content: Vec<Block>,
    bibliography: Option<BibliographyStore>,
    resources: ResourceStore,
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

    /// Build the document.
    pub fn build(self) -> Document {
        Document {
            metadata: self.metadata,
            page_setup: self.page_setup,
            stylesheet: self.stylesheet,
            content: self.content,
            bibliography: self.bibliography,
            resources: self.resources,
        }
    }
}
