//! Ergonomic builder API for constructing documents.

use crate::ast::{BibliographyStyle, Block, CitationMode, CrossRefKind, Document, Inline, CrossRefTarget};
use crate::bibliography::BibliographyStore;
use crate::style::ParagraphStyle;

/// Ergonomic builder for constructing documents
pub struct DocumentBuilder {
    doc: Document,
    next_label_number: usize,
}

impl DocumentBuilder {
    /// Create a new document builder with the given title
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            doc: Document::new(title),
            next_label_number: 1,
        }
    }

    /// Add a heading to the document
    pub fn heading(mut self, level: u8, text: impl Into<String>) -> Self {
        let block = Block::heading(level, vec![Inline::text(text)]);
        self.doc.add_block(block);
        self
    }

    /// Add a paragraph to the document
    pub fn paragraph(mut self, text: impl Into<String>) -> Self {
        let block = Block::paragraph(vec![Inline::text(text)]);
        self.doc.add_block(block);
        self
    }

    /// Add a paragraph with styled content
    pub fn styled_paragraph(mut self, content: Vec<Inline>) -> Self {
        let block = Block::paragraph(content);
        self.doc.add_block(block);
        self
    }

    /// Add a paragraph with custom style
    pub fn paragraph_with_style(
        mut self,
        text: impl Into<String>,
        style: ParagraphStyle,
    ) -> Self {
        let block = Block::Paragraph {
            content: vec![Inline::text(text)],
            style: Some(style),
        };
        self.doc.add_block(block);
        self
    }

    /// Add a code block
    pub fn code_block(mut self, code: impl Into<String>, language: Option<String>) -> Self {
        let block = Block::code_block(code.into(), language);
        self.doc.add_block(block);
        self
    }

    /// Add a block quote
    pub fn block_quote(mut self, blocks: Vec<Block>) -> Self {
        let block = Block::block_quote(blocks);
        self.doc.add_block(block);
        self
    }

    /// Add a page break
    pub fn page_break(mut self) -> Self {
        self.doc.add_block(Block::PageBreak);
        self
    }

    /// Add a horizontal rule
    pub fn horizontal_rule(mut self) -> Self {
        self.doc.add_block(Block::HorizontalRule);
        self
    }

    /// Set the document author
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.doc.metadata.author = Some(author.into());
        self
    }

    /// Set the document subject
    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.doc.metadata.subject = Some(subject.into());
        self
    }

    /// Set the document language
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.doc.metadata.language = Some(language.into());
        self
    }

    /// Set the bibliography store
    pub fn bibliography(mut self, bib: BibliographyStore) -> Self {
        self.doc.bibliography = Some(bib);
        self
    }

    /// Add a bibliography section with the given style
    pub fn render_bibliography(mut self, style: BibliographyStyle) -> Self {
        let block = Block::Bibliography { style };
        self.doc.add_block(block);
        self
    }

    /// Add an equation block
    pub fn equation(mut self, latex: impl Into<String>, label: Option<String>, numbered: bool) -> Self {
        let block = Block::Equation {
            latex: latex.into(),
            label,
            numbered,
        };
        self.doc.add_block(block);
        self
    }

    /// Register a cross-reference label with an auto-incremented number
    pub fn register_label(mut self, label: impl Into<String>, title: Option<String>) -> Self {
        let label_str = label.into();
        // If duplicate, record degraded feature and skip
        if self.doc.crossrefs.contains_key(&label_str) {
            // In a richer API we could return a Result; for now, skip adding duplicate
            return self;
        }

        let number = self.next_label_number;
        self.next_label_number += 1;
        self.doc.crossrefs.insert(
            label_str.clone(),
            CrossRefTarget {
                label: label_str,
                number,
                title,
            },
        );
        self
    }

    /// Add a bibliography entry directly
    pub fn bib_entry(mut self, entry: crate::bibliography::BibEntry) -> Self {
        let bib = self.doc.bibliography.get_or_insert_with(crate::bibliography::BibliographyStore::new);
        bib.add_entry(entry);
        self
    }

    /// Create an inline citation (helper for ergonomics)
    pub fn cite(&self, keys: Vec<String>, mode: CitationMode) -> Inline {
        Inline::citation(keys, mode)
    }

    /// Create an inline cross-reference (helper for ergonomics)
    pub fn crossref(&self, label: impl Into<String>, kind: CrossRefKind) -> Inline {
        Inline::crossref(label, kind)
    }

    /// Build the document
    pub fn build(self) -> Document {
        self.doc
    }
}

/// Helper functions for creating inline elements
impl Inline {
    /// Create a citation inline
    pub fn citation(keys: Vec<String>, mode: CitationMode) -> Self {
        Self::Citation { keys, mode }
    }

    /// Create a cross-reference inline
    pub fn crossref(label: impl Into<String>, kind: CrossRefKind) -> Self {
        Self::CrossRef {
            label: label.into(),
            kind,
        }
    }

    /// Create an inline equation
    pub fn inline_equation(latex: impl Into<String>) -> Self {
        Self::InlineEquation {
            latex: latex.into(),
        }
    }
}
