//! HTML backend for Lontar documents.
//!
//! Converts Lontar AST to semantic HTML5.

use crate::core::{
    Block, CitationMode, Document, DocumentWriter, Inline, WriteError, WriteReport, WriteResult,
};
use std::io::Write;

/// HTML writer that outputs semantic HTML5.
pub struct HtmlWriter<W: Write> {
    output: W,
    #[allow(dead_code)]
    in_paragraph: bool,
}

impl<W: Write> HtmlWriter<W> {
    /// Create a new HTML writer.
    pub fn new(output: W) -> Self {
        Self {
            output,
            in_paragraph: false,
        }
    }

    /// Write a block element.
    fn write_block(&mut self, block: &Block) -> WriteResult<()> {
        match block {
            Block::Heading { level, content, .. } => {
                let tag = format!("h{level}");
                write!(self.output, "<{tag}>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                self.write_inline_sequence(content)?;
                writeln!(self.output, "</{tag}>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Block::Paragraph { content, .. } => {
                write!(self.output, "<p>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                self.write_inline_sequence(content)?;
                writeln!(self.output, "</p>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Block::Table { rows, .. } => {
                writeln!(self.output, "<table>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                for row in rows {
                    writeln!(self.output, "  <tr>").map_err(|e| WriteError::IoError {
                        message: e.to_string(),
                    })?;
                    let tag = if row.is_header { "th" } else { "td" };
                    for cell in &row.cells {
                        write!(self.output, "    <{tag}>").map_err(|e| WriteError::IoError {
                            message: e.to_string(),
                        })?;
                        for block in &cell.content {
                            self.write_block(block)?;
                        }
                        writeln!(self.output, "</{tag}>").map_err(|e| WriteError::IoError {
                            message: e.to_string(),
                        })?;
                    }
                    writeln!(self.output, "  </tr>").map_err(|e| WriteError::IoError {
                        message: e.to_string(),
                    })?;
                }
                writeln!(self.output, "</table>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Block::List { items, ordered, .. } => {
                let tag = if *ordered { "ol" } else { "ul" };
                writeln!(self.output, "<{tag}>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                for item in items {
                    writeln!(self.output, "  <li>").map_err(|e| WriteError::IoError {
                        message: e.to_string(),
                    })?;
                    for block in &item.content {
                        self.write_block(block)?;
                    }
                    writeln!(self.output, "  </li>").map_err(|e| WriteError::IoError {
                        message: e.to_string(),
                    })?;
                }
                writeln!(self.output, "</{tag}>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Block::CodeBlock { code, language, .. } => {
                if let Some(lang) = language {
                    writeln!(self.output, "<pre><code class=\"language-{lang}\">").map_err(
                        |e| WriteError::IoError {
                            message: e.to_string(),
                        },
                    )?;
                } else {
                    writeln!(self.output, "<pre><code>").map_err(|e| WriteError::IoError {
                        message: e.to_string(),
                    })?;
                }
                write!(self.output, "{}", html_escape(code)).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                writeln!(self.output, "</code></pre>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Block::BlockQuote { content, .. } => {
                writeln!(self.output, "<blockquote>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                for block in content {
                    self.write_block(block)?;
                }
                writeln!(self.output, "</blockquote>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Block::HorizontalRule { .. } => {
                writeln!(self.output, "<hr>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Block::Chart { .. } => {
                // Charts not rendered in HTML backend yet.
                writeln!(
                    self.output,
                    "<div class=\"chart-placeholder\">[Chart omitted]</div>"
                )
                .map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Block::Image {
                resource_id,
                alt_text,
                width,
                height,
                ..
            } => {
                let alt = alt_text.as_deref().unwrap_or("");
                let mut img_tag = format!("<img src=\"{resource_id}\" alt=\"{alt}\"");
                if let Some(w) = width {
                    img_tag.push_str(&format!(" width=\"{w}\""));
                }
                if let Some(h) = height {
                    img_tag.push_str(&format!(" height=\"{h}\""));
                }
                img_tag.push('>');
                writeln!(self.output, "{img_tag}").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Block::Equation {
                content,
                display,
                label,
                ..
            } => {
                if *display {
                    writeln!(self.output, "<div class=\"equation\">").map_err(|e| {
                        WriteError::IoError {
                            message: e.to_string(),
                        }
                    })?;
                    writeln!(self.output, "$${content}$$").map_err(|e| WriteError::IoError {
                        message: e.to_string(),
                    })?;
                    if let Some(lbl) = label {
                        writeln!(self.output, "<span class=\"label\">({lbl})</span>",).map_err(
                            |e| WriteError::IoError {
                                message: e.to_string(),
                            },
                        )?;
                    }
                    writeln!(self.output, "</div>").map_err(|e| WriteError::IoError {
                        message: e.to_string(),
                    })?;
                } else {
                    write!(self.output, "<span class=\"equation\">${content}$</span>").map_err(
                        |e| WriteError::IoError {
                            message: e.to_string(),
                        },
                    )?;
                }
            }
            Block::Section { title, content, .. } => {
                writeln!(self.output, "<section>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                if let Some(t) = title {
                    writeln!(self.output, "<h2>{}</h2>", html_escape(t)).map_err(|e| {
                        WriteError::IoError {
                            message: e.to_string(),
                        }
                    })?;
                }
                for block in content {
                    self.write_block(block)?;
                }
                writeln!(self.output, "</section>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Block::Bibliography { title, .. } => {
                writeln!(self.output, "<section class=\"bibliography\">").map_err(|e| {
                    WriteError::IoError {
                        message: e.to_string(),
                    }
                })?;
                if let Some(t) = title {
                    writeln!(self.output, "<h2>{}</h2>", html_escape(t)).map_err(|e| {
                        WriteError::IoError {
                            message: e.to_string(),
                        }
                    })?;
                }
                writeln!(self.output, "</section>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
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
                write!(self.output, "{}", html_escape(text)).map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Inline::SoftBreak => {
                write!(self.output, " ").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Inline::LineBreak => {
                writeln!(self.output, "<br>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Inline::Bold(content) => {
                write!(self.output, "<strong>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                self.write_inline_sequence(content)?;
                write!(self.output, "</strong>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Inline::Italic(content) => {
                write!(self.output, "<em>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                self.write_inline_sequence(content)?;
                write!(self.output, "</em>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Inline::Code(code) => {
                write!(self.output, "<code>{}</code>", html_escape(code)).map_err(|e| {
                    WriteError::IoError {
                        message: e.to_string(),
                    }
                })?;
            }
            Inline::Strikethrough(content) => {
                write!(self.output, "<del>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                self.write_inline_sequence(content)?;
                write!(self.output, "</del>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Inline::Subscript(content) => {
                write!(self.output, "<sub>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                self.write_inline_sequence(content)?;
                write!(self.output, "</sub>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Inline::Superscript(content) => {
                write!(self.output, "<sup>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                self.write_inline_sequence(content)?;
                write!(self.output, "</sup>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Inline::Link { text, url, title } => {
                let title_attr = title
                    .as_ref()
                    .map(|t| format!(" title=\"{}\"", html_escape(t)))
                    .unwrap_or_default();
                write!(
                    self.output,
                    "<a href=\"{}\"{}> ",
                    html_escape(url),
                    title_attr
                )
                .map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
                self.write_inline_sequence(text)?;
                write!(self.output, "</a>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Inline::Image {
                resource_id,
                alt_text,
                title,
            } => {
                let alt = alt_text.as_deref().unwrap_or("");
                let title_attr = title
                    .as_ref()
                    .map(|t| format!(" title=\"{}\"", html_escape(t)))
                    .unwrap_or_default();
                writeln!(
                    self.output,
                    "<img src=\"{}\" alt=\"{}\"{}/>",
                    html_escape(resource_id),
                    html_escape(alt),
                    title_attr
                )
                .map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Inline::Citation {
                key,
                mode,
                prefix,
                suffix,
            } => {
                let prefix_text = prefix.as_deref().unwrap_or("");
                let suffix_text = suffix.as_deref().unwrap_or("");
                let class = match mode {
                    CitationMode::Parenthetical => "citation parenthetical",
                    CitationMode::Narrative => "citation narrative",
                    CitationMode::YearOnly => "citation year-only",
                    CitationMode::SuppressAuthor => "citation suppress-author",
                    CitationMode::Full => "citation full",
                };
                write!(
                    self.output,
                    "<span class=\"{}\" data-key=\"{}\">{}{}{}</span>",
                    class,
                    html_escape(key),
                    html_escape(prefix_text),
                    html_escape(key),
                    html_escape(suffix_text)
                )
                .map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Inline::CrossRef { label, kind } => {
                let kind_str = match kind {
                    crate::core::CrossRefKind::Auto => "auto",
                    crate::core::CrossRefKind::Page => "page",
                    crate::core::CrossRefKind::Title => "title",
                };
                write!(
                    self.output,
                    "<span class=\"crossref\" data-label=\"{}\" data-kind=\"{}\">{}</span>",
                    html_escape(label),
                    kind_str,
                    html_escape(label)
                )
                .map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
            Inline::Styled { content, style } => {
                write!(self.output, "<span class=\"{}\">", html_escape(style)).map_err(|e| {
                    WriteError::IoError {
                        message: e.to_string(),
                    }
                })?;
                self.write_inline_sequence(content)?;
                write!(self.output, "</span>").map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }
        }
        Ok(())
    }
}

impl<W: Write> DocumentWriter for HtmlWriter<W> {
    fn write(&mut self, document: &Document) -> WriteResult<WriteReport> {
        let report = WriteReport::new();

        // Write HTML header
        writeln!(self.output, "<!DOCTYPE html>").map_err(|e| WriteError::IoError {
            message: e.to_string(),
        })?;
        writeln!(self.output, "<html>").map_err(|e| WriteError::IoError {
            message: e.to_string(),
        })?;
        writeln!(self.output, "<head>").map_err(|e| WriteError::IoError {
            message: e.to_string(),
        })?;
        writeln!(self.output, "  <meta charset=\"UTF-8\">").map_err(|e| WriteError::IoError {
            message: e.to_string(),
        })?;
        writeln!(
            self.output,
            "  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">"
        )
        .map_err(|e| WriteError::IoError {
            message: e.to_string(),
        })?;

        if let Some(title) = &document.metadata.title {
            writeln!(self.output, "  <title>{}</title>", html_escape(title)).map_err(|e| {
                WriteError::IoError {
                    message: e.to_string(),
                }
            })?;
        }

        if let Some(lang) = &document.metadata.language {
            writeln!(
                self.output,
                "  <meta name=\"language\" content=\"{}\">",
                html_escape(lang)
            )
            .map_err(|e| WriteError::IoError {
                message: e.to_string(),
            })?;
        }

        writeln!(self.output, "</head>").map_err(|e| WriteError::IoError {
            message: e.to_string(),
        })?;
        writeln!(self.output, "<body>").map_err(|e| WriteError::IoError {
            message: e.to_string(),
        })?;

        // Write title if present
        if let Some(title) = &document.metadata.title {
            writeln!(self.output, "<h1>{}</h1>", html_escape(title)).map_err(|e| {
                WriteError::IoError {
                    message: e.to_string(),
                }
            })?;
        }

        // Write author if present
        if let Some(author) = &document.metadata.author {
            writeln!(
                self.output,
                "<p class=\"author\">By: {}</p>",
                html_escape(author)
            )
            .map_err(|e| WriteError::IoError {
                message: e.to_string(),
            })?;
        }

        // Write content
        for block in &document.content {
            self.write_block(block)?;
        }

        // Write bibliography if present
        if let Some(bib) = &document.bibliography {
            writeln!(self.output, "<section class=\"bibliography\">").map_err(|e| {
                WriteError::IoError {
                    message: e.to_string(),
                }
            })?;
            writeln!(self.output, "<h2>References</h2>").map_err(|e| WriteError::IoError {
                message: e.to_string(),
            })?;
            writeln!(self.output, "<ol>").map_err(|e| WriteError::IoError {
                message: e.to_string(),
            })?;

            for (key, entry) in &bib.entries {
                writeln!(
                    self.output,
                    "  <li id=\"ref-{}\">[{}] {}</li>",
                    html_escape(key),
                    html_escape(key),
                    html_escape(&entry.key)
                )
                .map_err(|e| WriteError::IoError {
                    message: e.to_string(),
                })?;
            }

            writeln!(self.output, "</ol>").map_err(|e| WriteError::IoError {
                message: e.to_string(),
            })?;
            writeln!(self.output, "</section>").map_err(|e| WriteError::IoError {
                message: e.to_string(),
            })?;
        }

        // Write HTML footer
        writeln!(self.output, "</body>").map_err(|e| WriteError::IoError {
            message: e.to_string(),
        })?;
        writeln!(self.output, "</html>").map_err(|e| WriteError::IoError {
            message: e.to_string(),
        })?;

        Ok(report)
    }

    fn name(&self) -> &'static str {
        "HTML"
    }

    fn supported_bibliography_styles(&self) -> Vec<&'static str> {
        vec!["numeric", "author-year"]
    }
}

/// Escape HTML special characters.
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
