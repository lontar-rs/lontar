//! Plain text backend for Lontar documents.
//!
//! Converts Lontar AST to plain ASCII text with minimal formatting.

use crate::core::{Block, Document, DocumentWriter, Inline, WriteError, WriteReport, WriteResult};
use std::io::Write;

/// Plain text writer that outputs ASCII text.
pub struct PlainTextWriter<W: Write> {
    output: W,
    line_width: usize,
    current_line_length: usize,
}

impl<W: Write> PlainTextWriter<W> {
    /// Create a new plain text writer.
    pub fn new(output: W) -> Self {
        Self {
            output,
            line_width: 80,
            current_line_length: 0,
        }
    }

    /// Set the line width for text wrapping.
    pub fn with_line_width(mut self, width: usize) -> Self {
        self.line_width = width;
        self
    }

    /// Write a block element.
    fn write_block(&mut self, block: &Block) -> WriteResult<()> {
        match block {
            Block::Heading { level, content, .. } => {
                let underline = match level {
                    1 => "=",
                    2 => "-",
                    _ => "~",
                };

                self.write_inline_sequence(content)?;
                writeln!(self.output).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;

                let heading_text = self.inline_sequence_to_string(content)?;
                writeln!(self.output, "{}", underline.repeat(heading_text.len())).map_err(|e| {
                    WriteError::IoError {
                        message: e.to_string(),
                    }
                })?;
                writeln!(self.output).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Block::Paragraph { content, .. } => {
                self.write_inline_sequence(content)?;
                writeln!(self.output).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                writeln!(self.output).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Block::Table { rows, .. } => {
                for row in rows {
                    for (i, cell) in row.cells.iter().enumerate() {
                        if i > 0 {
                            write!(self.output, " | ").map_err(|e| WriteError::IoError {
                                message: e.to_string(),
                            })?;
                        }
                        for block in &cell.content {
                            if let Block::Paragraph { content, .. } = block {
                                let text = self.inline_sequence_to_string(content)?;
                                write!(self.output, "{text}").map_err(|e| WriteError::IoError {
                                    message: e.to_string(),
                                })?;
                            }
                        }
                    }
                    writeln!(self.output).map_err(|e| WriteError::IoError {
                        message: e.to_string(),
                    })?;
                }
                writeln!(self.output).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Block::Chart { .. } => {
                writeln!(self.output, "[Chart omitted]").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                writeln!(self.output).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Block::List { items, ordered, .. } => {
                for (i, item) in items.iter().enumerate() {
                    let indent = "  ".repeat(item.level as usize);
                    let marker = if *ordered {
                        format!("{}. ", i + 1)
                    } else {
                        "* ".to_string()
                    };
                    write!(self.output, "{indent}{marker}").map_err(|e| WriteError::IoError {
                        message: e.to_string(),
                    })?;
                    for block in &item.content {
                        self.write_block(block)?;
                    }
                }
                writeln!(self.output).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Block::CodeBlock {
                code, language: _, ..
            } => {
                writeln!(self.output, "[CODE]").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                write!(self.output, "{code}").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                writeln!(self.output, "[/CODE]").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                writeln!(self.output).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Block::BlockQuote { content, .. } => {
                for block in content {
                    match block {
                        Block::Paragraph { content, .. } => {
                            write!(self.output, "  ").map_err(|e| WriteError::IoError {
                                message: e.to_string(),
                            })?;
                            self.write_inline_sequence(content)?;
                            writeln!(self.output).map_err(|e| WriteError::IoError {
                                message: e.to_string(),
                            })?;
                        }
                        _ => self.write_block(block)?,
                    }
                }
                writeln!(self.output).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Block::HorizontalRule { .. } => {
                writeln!(self.output, "---").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                writeln!(self.output).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Block::Image { alt_text, .. } => {
                let alt = alt_text.as_deref().unwrap_or("image");
                writeln!(self.output, "[Image: {alt}]").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                writeln!(self.output).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Block::Equation {
                content, display, ..
            } => {
                if *display {
                    writeln!(self.output, "[Equation]").map_err(|e| WriteError::IoError {
                        message: e.to_string(),
                    })?;
                    writeln!(self.output, "{content}").map_err(|e| WriteError::IoError {
                        message: e.to_string(),
                    })?;
                } else {
                    write!(self.output, "[{content}]").map_err(|e| WriteError::IoError {
                        message: e.to_string(),
                    })?;
                }
                writeln!(self.output).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Block::Section { title, content, .. } => {
                if let Some(t) = title {
                    writeln!(self.output, "\n{t}").map_err(|e| WriteError::IoError {
                        message: e.to_string(),
                    })?;
                    writeln!(self.output, "{}", "-".repeat(t.len())).map_err(|e| {
                        WriteError::IoError {
                            message: e.to_string(),
                        }
                    })?;
                    writeln!(self.output).map_err(|e| WriteError::IoError {
                        message: e.to_string(),
                    })?;
                }
                for block in content {
                    self.write_block(block)?;
                }
            }
            Block::Bibliography { title, .. } => {
                if let Some(t) = title {
                    writeln!(self.output, "\n{t}").map_err(|e| WriteError::IoError {
                        message: e.to_string(),
                    })?;
                    writeln!(self.output, "{}", "-".repeat(t.len())).map_err(|e| {
                        WriteError::IoError {
                            message: e.to_string(),
                        }
                    })?;
                    writeln!(self.output).map_err(|e| WriteError::IoError {
                        message: e.to_string(),
                    })?;
                }
            }
        }
        Ok(())
    }

    /// Write a sequence of inline elements.
    fn write_inline_sequence(&mut self, inlines: &[Inline]) -> WriteResult<()> {
        for inline in inlines {
            self.write_inline(inline)?;
        }
        Ok(())
    }

    /// Convert inline sequence to string.
    fn inline_sequence_to_string(&self, inlines: &[Inline]) -> WriteResult<String> {
        let mut result = String::new();
        for inline in inlines {
            result.push_str(&self.inline_to_string(inline)?);
        }
        Ok(result)
    }

    /// Convert inline element to string.
    fn inline_to_string(&self, inline: &Inline) -> WriteResult<String> {
        match inline {
            Inline::Text(text) => Ok(text.clone()),
            Inline::SoftBreak => Ok(" ".to_string()),
            Inline::LineBreak => Ok("\n".to_string()),
            Inline::Bold(content) => self.inline_sequence_to_string(content),
            Inline::Italic(content) => self.inline_sequence_to_string(content),
            Inline::Code(code) => Ok(code.clone()),
            Inline::Strikethrough(content) => self.inline_sequence_to_string(content),
            Inline::Subscript(content) => self.inline_sequence_to_string(content),
            Inline::Superscript(content) => self.inline_sequence_to_string(content),
            Inline::Link { text, .. } => self.inline_sequence_to_string(text),
            Inline::Image { alt_text, .. } => Ok(alt_text.as_ref().cloned().unwrap_or_default()),
            Inline::Citation { key, .. } => Ok(format!("[{key}]")),
            Inline::CrossRef { label, .. } => Ok(format!("[{label}]")),
            Inline::Styled { content, .. } => self.inline_sequence_to_string(content),
        }
    }

    /// Write an inline element.
    fn write_inline(&mut self, inline: &Inline) -> WriteResult<()> {
        match inline {
            Inline::Text(text) => {
                write!(self.output, "{text}").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                self.current_line_length += text.len();
            }
            Inline::SoftBreak => {
                write!(self.output, " ").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                self.current_line_length += 1;
            }
            Inline::LineBreak => {
                writeln!(self.output).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                self.current_line_length = 0;
            }
            Inline::Bold(content) => {
                self.write_inline_sequence(content)?;
            }
            Inline::Italic(content) => {
                self.write_inline_sequence(content)?;
            }
            Inline::Code(code) => {
                write!(self.output, "{code}").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                self.current_line_length += code.len();
            }
            Inline::Strikethrough(content) => {
                self.write_inline_sequence(content)?;
            }
            Inline::Subscript(content) => {
                self.write_inline_sequence(content)?;
            }
            Inline::Superscript(content) => {
                self.write_inline_sequence(content)?;
            }
            Inline::Link { text, .. } => {
                self.write_inline_sequence(text)?;
            }
            Inline::Image { alt_text, .. } => {
                if let Some(alt) = alt_text {
                    write!(self.output, "[{alt}]").map_err(|e| WriteError::IoError {
                        message: e.to_string(),
                    })?;
                    self.current_line_length += alt.len() + 2;
                }
            }
            Inline::Citation { key, .. } => {
                write!(self.output, "[{key}]").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                self.current_line_length += key.len() + 2;
            }
            Inline::CrossRef { label, .. } => {
                write!(self.output, "[{label}]").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                self.current_line_length += label.len() + 2;
            }
            Inline::Styled { content, .. } => {
                self.write_inline_sequence(content)?;
            }
        }
        Ok(())
    }
}

impl<W: Write> DocumentWriter for PlainTextWriter<W> {
    fn write(&mut self, document: &Document) -> WriteResult<WriteReport> {
        let report = WriteReport::new()
            .degrade_with_fallback(
                "formatting",
                "Plain text does not support bold, italic, or colors",
                "Text is rendered without formatting",
            )
            .degrade_with_fallback(
                "images",
                "Plain text cannot embed images",
                "Images are shown as [Image: alt text]",
            )
            .degrade_with_fallback(
                "equations",
                "Plain text has limited equation support",
                "Equations are shown as text",
            );

        // Write title if present
        if let Some(title) = &document.metadata.title {
            writeln!(self.output, "{title}").map_err(|e| WriteError::IoError {
                message: e.to_string(),
            })?;
            writeln!(self.output, "{}", "=".repeat(title.len())).map_err(|e| {
                WriteError::IoError {
                    message: e.to_string(),
                }
            })?;
            writeln!(self.output).map_err(|e| WriteError::IoError {
                message: e.to_string(),
            })?;
        }

        // Write author if present
        if let Some(author) = &document.metadata.author {
            writeln!(self.output, "By: {author}").map_err(|e| WriteError::IoError {
                message: e.to_string(),
            })?;
            writeln!(self.output).map_err(|e| WriteError::IoError {
                message: e.to_string(),
            })?;
        }

        // Write content
        for block in &document.content {
            self.write_block(block)?;
        }

        // Write bibliography if present
        if let Some(bib) = &document.bibliography {
            writeln!(self.output, "\nReferences").map_err(|e| WriteError::IoError {
                message: e.to_string(),
            })?;
            writeln!(self.output, "----------").map_err(|e| WriteError::IoError {
                message: e.to_string(),
            })?;
            writeln!(self.output).map_err(|e| WriteError::IoError {
                message: e.to_string(),
            })?;

            for (key, entry) in &bib.entries {
                writeln!(self.output, "[{}] {}", key, entry.key).map_err(|e| {
                    WriteError::IoError {
                        message: e.to_string(),
                    }
                })?;
            }
        }

        Ok(report)
    }

    fn name(&self) -> &'static str {
        "PlainText"
    }

    fn supported_bibliography_styles(&self) -> Vec<&'static str> {
        vec!["numeric"]
    }
}
