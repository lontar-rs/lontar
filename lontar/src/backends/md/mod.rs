//! Markdown backend for Lontar documents.
//!
//! Converts Lontar AST to GitHub-flavored Markdown (GFM).

use crate::core::{
    Block, Citation, CitationMode, Document, DocumentWriter, Inline, WriteError, WriteReport,
    WriteResult,
};
use std::io::Write;

/// Markdown writer that outputs GitHub-flavored Markdown.
pub struct MarkdownWriter<W: Write> {
    output: W,
    in_list: bool,
    list_level: usize,
}

impl<W: Write> MarkdownWriter<W> {
    /// Create a new Markdown writer.
    pub fn new(output: W) -> Self {
        Self {
            output,
            in_list: false,
            list_level: 0,
        }
    }

    /// Write a block element.
    fn write_block(&mut self, block: &Block) -> WriteResult<()> {
        match block {
            Block::Heading { level, content, .. } => {
                let hashes = "#".repeat(*level as usize);
                write!(self.output, "{} ", hashes).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                self.write_inline_sequence(content)?;
                writeln!(self.output).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
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
                for (i, row) in rows.iter().enumerate() {
                    write!(self.output, "|").map_err(|e| WriteError::IoError {
                        message: e.to_string(),
                    })?;
                    for cell in &row.cells {
                        for block in &cell.content {
                            if let Block::Paragraph { content, .. } = block {
                                self.write_inline_sequence(content)?;
                            }
                        }
                        write!(self.output, "|").map_err(|e| WriteError::IoError {
                            message: e.to_string(),
                        })?;
                    }
                    writeln!(self.output).map_err(|e| WriteError::IoError {
                        message: e.to_string(),
                    })?;

                    if i == 0 && row.is_header {
                        write!(self.output, "|").map_err(|e| WriteError::IoError {
                            message: e.to_string(),
                        })?;
                        for _ in &row.cells {
                            write!(self.output, "---|").map_err(|e| WriteError::IoError {
                                message: e.to_string(),
                            })?;
                        }
                        writeln!(self.output).map_err(|e| WriteError::IoError {
                            message: e.to_string(),
                        })?;
                    }
                }
                writeln!(self.output).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Block::List { items, ordered, .. } => {
                for item in items {
                    let indent = "  ".repeat(item.level as usize);
                    let marker = if *ordered { "1. " } else { "- " };
                    write!(self.output, "{}{}", indent, marker).map_err(|e| WriteError::IoError {
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
                code,
                language,
                ..
            } => {
                let lang = language.as_deref().unwrap_or("");
                writeln!(self.output, "```{}", lang).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                write!(self.output, "{}", code).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                writeln!(self.output, "\n```").map_err(|e| WriteError::IoError {
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
                            write!(self.output, "> ").map_err(|e| WriteError::IoError {
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
            Block::Image {
                resource_id,
                alt_text,
                ..
            } => {
                let alt = alt_text.as_deref().unwrap_or("");
                writeln!(self.output, "![{}]({})", alt, resource_id).map_err(|e| {
                    WriteError::IoError {
                        message: e.to_string(),
                    }
                })?;
                writeln!(self.output).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Block::Equation {
                content,
                display,
                ..
            } => {
                if *display {
                    writeln!(self.output, "$$\n{}\n$$", content).map_err(|e| WriteError::IoError {
                        message: e.to_string(),
                    })?;
                } else {
                    write!(self.output, "${}$", content).map_err(|e| WriteError::IoError {
                        message: e.to_string(),
                    })?;
                }
                writeln!(self.output).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Block::Section { title, content, .. } => {
                if let Some(t) = title {
                    writeln!(self.output, "## {}", t).map_err(|e| WriteError::IoError {
                        message: e.to_string(),
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
                    writeln!(self.output, "## {}", t).map_err(|e| WriteError::IoError {
                        message: e.to_string(),
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

    /// Write an inline element.
    fn write_inline(&mut self, inline: &Inline) -> WriteResult<()> {
        match inline {
            Inline::Text(text) => {
                write!(self.output, "{}", text).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Inline::SoftBreak => {
                write!(self.output, " ").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Inline::LineBreak => {
                writeln!(self.output).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Inline::Bold(content) => {
                write!(self.output, "**").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                self.write_inline_sequence(content)?;
                write!(self.output, "**").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Inline::Italic(content) => {
                write!(self.output, "*").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                self.write_inline_sequence(content)?;
                write!(self.output, "*").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Inline::Code(code) => {
                write!(self.output, "`{}`", code).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Inline::Strikethrough(content) => {
                write!(self.output, "~~").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                self.write_inline_sequence(content)?;
                write!(self.output, "~~").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Inline::Link { text, url, .. } => {
                write!(self.output, "[").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                self.write_inline_sequence(text)?;
                write!(self.output, "]({})", url).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Inline::Image {
                resource_id,
                alt_text,
                ..
            } => {
                let alt = alt_text.as_deref().unwrap_or("");
                write!(self.output, "![{}]({})", alt, resource_id).map_err(|e| {
                    WriteError::IoError {
                        message: e.to_string(),
                    }
                })?;
            }
            Inline::Citation { key, mode, .. } => {
                match mode {
                    CitationMode::Parenthetical => {
                        write!(self.output, "[{}]", key).map_err(|e| WriteError::IoError {
                            message: e.to_string(),
                        })?;
                    }
                    _ => {
                        write!(self.output, "[{}]", key).map_err(|e| WriteError::IoError {
                            message: e.to_string(),
                        })?;
                    }
                }
            }
            Inline::CrossRef { label, .. } => {
                write!(self.output, "[{}]", label).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Inline::Styled { content, .. } => {
                self.write_inline_sequence(content)?;
            }
            _ => {}
        }
        Ok(())
    }
}

impl<W: Write> DocumentWriter for MarkdownWriter<W> {
    fn write(&mut self, document: &Document) -> WriteResult<WriteReport> {
        let mut report = WriteReport::new();

        // Write title if present
        if let Some(title) = &document.metadata.title {
            writeln!(self.output, "# {}", title).map_err(|e| WriteError::IoError {
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
            writeln!(self.output, "## References").map_err(|e| WriteError::IoError {
                message: e.to_string(),
            })?;
            writeln!(self.output).map_err(|e| WriteError::IoError {
                message: e.to_string(),
            })?;

            for (key, entry) in &bib.entries {
                writeln!(self.output, "- [{}] {}", key, entry.key).map_err(|e| {
                    WriteError::IoError {
                        message: e.to_string(),
                    }
                })?;
            }
        }

        Ok(report)
    }

    fn name(&self) -> &'static str {
        "Markdown"
    }

    fn supported_bibliography_styles(&self) -> Vec<&'static str> {
        vec!["numeric", "author-year"]
    }
}
