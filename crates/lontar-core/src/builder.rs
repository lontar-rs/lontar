//! Ergonomic builder API for constructing documents.

use crate::ast::{Block, Document, Inline};
use crate::style::ParagraphStyle;

/// Ergonomic builder for constructing documents
pub struct DocumentBuilder {
    doc: Document,
}

impl DocumentBuilder {
    /// Create a new document builder with the given title
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            doc: Document::new(title),
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

    /// Build the document
    pub fn build(self) -> Document {
        self.doc
    }
}
