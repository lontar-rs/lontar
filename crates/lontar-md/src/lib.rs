//! Markdown document format support for Lontar.
//!
//! This crate implements the `DocumentWriter` trait to render Lontar documents as Markdown.
//! It handles all block and inline types, including citations, cross-references, and equations.

use lontar_core::prelude::*;
use lontar_core::writer::WriteOptions;
use std::fmt::Write;

/// Options for Markdown output
#[derive(Debug, Clone, Default)]
pub struct MarkdownOptions {
    /// Use GitHub-flavored Markdown (GFM) for tables
    pub gfm_tables: bool,
    /// Include table of contents
    pub include_toc: bool,
}

impl WriteOptions for MarkdownOptions {}

/// Markdown document writer
pub struct MarkdownWriter;

impl DocumentWriter for MarkdownWriter {
    type Options = MarkdownOptions;

    fn write(doc: &Document, _options: &Self::Options) -> Result<Vec<u8>, LontarError> {
        let mut output = String::new();

        // Write metadata as YAML front matter
        writeln!(&mut output, "---").map_err(|_| LontarError::InternalError("write error".into()))?;
        writeln!(&mut output, "title: {}", doc.metadata.title).map_err(|_| LontarError::InternalError("write error".into()))?;
        if let Some(author) = &doc.metadata.author {
            writeln!(&mut output, "author: {}", author).map_err(|_| LontarError::InternalError("write error".into()))?;
        }
        if let Some(subject) = &doc.metadata.subject {
            writeln!(&mut output, "subject: {}", subject).map_err(|_| LontarError::InternalError("write error".into()))?;
        }
        writeln!(&mut output, "---").map_err(|_| LontarError::InternalError("write error".into()))?;
        writeln!(&mut output).map_err(|_| LontarError::InternalError("write error".into()))?;

        // Write content blocks
        for block in &doc.content {
            write_block(&mut output, block, doc)?;
            writeln!(&mut output).map_err(|_| LontarError::InternalError("write error".into()))?;
        }

        Ok(output.into_bytes())
    }
}

fn write_block(output: &mut String, block: &Block, doc: &Document) -> Result<(), LontarError> {
    match block {
        Block::Heading { level, content, id } => {
            let hashes = "#".repeat(*level as usize);
            write!(output, "{} ", hashes).map_err(|_| LontarError::InternalError("write error".into()))?;
            for inline in content {
                write_inline(output, inline, doc)?;
            }
            if let Some(id) = id {
                write!(output, " {{#{}}}", id).map_err(|_| LontarError::InternalError("write error".into()))?;
            }
            writeln!(output).map_err(|_| LontarError::InternalError("write error".into()))?;
        }

        Block::Paragraph { content, .. } => {
            for inline in content {
                write_inline(output, inline, doc)?;
            }
            writeln!(output).map_err(|_| LontarError::InternalError("write error".into()))?;
        }

        Block::Table { headers, rows, .. } => {
            if let Some(header_cells) = headers {
                write!(output, "|").map_err(|_| LontarError::InternalError("write error".into()))?;
                for cell in header_cells {
                    for block in &cell.content {
                        if let Block::Paragraph { content, .. } = block {
                            for inline in content {
                                write_inline(output, inline, doc)?;
                            }
                        }
                    }
                    write!(output, "|").map_err(|_| LontarError::InternalError("write error".into()))?;
                }
                writeln!(output).map_err(|_| LontarError::InternalError("write error".into()))?;

                write!(output, "|").map_err(|_| LontarError::InternalError("write error".into()))?;
                for _ in header_cells {
                    write!(output, "----|").map_err(|_| LontarError::InternalError("write error".into()))?;
                }
                writeln!(output).map_err(|_| LontarError::InternalError("write error".into()))?;
            }

            for row in rows {
                write!(output, "|").map_err(|_| LontarError::InternalError("write error".into()))?;
                for cell in row {
                    for block in &cell.content {
                        if let Block::Paragraph { content, .. } = block {
                            for inline in content {
                                write_inline(output, inline, doc)?;
                            }
                        }
                    }
                    write!(output, "|").map_err(|_| LontarError::InternalError("write error".into()))?;
                }
                writeln!(output).map_err(|_| LontarError::InternalError("write error".into()))?;
            }
        }

        Block::List { ordered, items } => {
            write_list_items(output, items, *ordered, 0, doc)?;
        }

        Block::CodeBlock { code, language } => {
            let lang = language.as_deref().unwrap_or("");
            writeln!(output, "```{}", lang).map_err(|_| LontarError::InternalError("write error".into()))?;
            write!(output, "{}", code).map_err(|_| LontarError::InternalError("write error".into()))?;
            writeln!(output, "\n```").map_err(|_| LontarError::InternalError("write error".into()))?;
        }

        Block::BlockQuote { content } => {
            for block in content {
                match block {
                    Block::Paragraph { content, .. } => {
                        write!(output, "> ").map_err(|_| LontarError::InternalError("write error".into()))?;
                        for inline in content {
                            write_inline(output, inline, doc)?;
                        }
                        writeln!(output).map_err(|_| LontarError::InternalError("write error".into()))?;
                    }
                    _ => write_block(output, block, doc)?,
                }
            }
        }

        Block::Image { resource_id, alt_text, .. } => {
            let alt = alt_text.as_deref().unwrap_or("");
            writeln!(output, "![{}]({})", alt, resource_id).map_err(|_| LontarError::InternalError("write error".into()))?;
        }

        Block::HorizontalRule => {
            writeln!(output, "---").map_err(|_| LontarError::InternalError("write error".into()))?;
        }

        Block::PageBreak => {
            writeln!(output, "\n---\n").map_err(|_| LontarError::InternalError("write error".into()))?;
        }

        Block::Equation { latex, label, numbered } => {
            if *numbered {
                if let Some(label) = label {
                    writeln!(output, "$$\n{}\n$$ ({})", latex, label).map_err(|_| LontarError::InternalError("write error".into()))?;
                } else {
                    writeln!(output, "$$\n{}\n$$", latex).map_err(|_| LontarError::InternalError("write error".into()))?;
                }
            } else {
                writeln!(output, "$$\n{}\n$$", latex).map_err(|_| LontarError::InternalError("write error".into()))?;
            }
        }

        Block::Chart { .. } => {
            writeln!(output, "*[Chart: not supported in Markdown]*").map_err(|_| LontarError::InternalError("write error".into()))?;
        }

        Block::Bibliography { style } => {
            if let Some(bib) = &doc.bibliography {
                writeln!(output, "## References\n").map_err(|_| LontarError::InternalError("write error".into()))?;
                match bib.render_bibliography(*style) {
                    Ok(entries) => {
                        for entry in entries {
                            writeln!(output, "- {}", entry).map_err(|_| LontarError::InternalError("write error".into()))?;
                        }
                    }
                    Err(_) => {
                        writeln!(output, "*[Bibliography rendering failed]*").map_err(|_| LontarError::InternalError("write error".into()))?;
                    }
                }
            }
        }
    }

    Ok(())
}

fn write_list_items(
    output: &mut String,
    items: &[ListItem],
    ordered: bool,
    depth: usize,
    doc: &Document,
) -> Result<(), LontarError> {
    for (i, item) in items.iter().enumerate() {
        let indent = "  ".repeat(depth);
        let marker = if ordered {
            format!("{}. ", i + 1)
        } else {
            "- ".to_string()
        };

        write!(output, "{}{}", indent, marker).map_err(|_| LontarError::InternalError("write error".into()))?;
        for inline in &item.content {
            write_inline(output, inline, doc)?;
        }
        writeln!(output).map_err(|_| LontarError::InternalError("write error".into()))?;

        if !item.nested_items.is_empty() {
            write_list_items(output, &item.nested_items, ordered, depth + 1, doc)?;
        }
    }

    Ok(())
}

fn write_inline(output: &mut String, inline: &Inline, doc: &Document) -> Result<(), LontarError> {
    match inline {
        Inline::Text { content, style } => {
            if let Some(style) = style {
                let mut formatted = content.clone();
                if style.bold == Some(true) {
                    formatted = format!("**{}**", formatted);
                }
                if style.italic == Some(true) {
                    formatted = format!("*{}*", formatted);
                }
                if style.strikethrough == Some(true) {
                    formatted = format!("~~{}~~", formatted);
                }
                write!(output, "{}", formatted).map_err(|_| LontarError::InternalError("write error".into()))?;
            } else {
                write!(output, "{}", content).map_err(|_| LontarError::InternalError("write error".into()))?;
            }
        }

        Inline::Link { text, url } => {
            write!(output, "[").map_err(|_| LontarError::InternalError("write error".into()))?;
            for t in text {
                write_inline(output, t, doc)?;
            }
            write!(output, "]({})", url).map_err(|_| LontarError::InternalError("write error".into()))?;
        }

        Inline::LineBreak => {
            writeln!(output).map_err(|_| LontarError::InternalError("write error".into()))?;
        }

        Inline::SoftSpace => {
            write!(output, " ").map_err(|_| LontarError::InternalError("write error".into()))?;
        }

        Inline::Tab => {
            write!(output, "\t").map_err(|_| LontarError::InternalError("write error".into()))?;
        }

        Inline::Citation { keys, mode } => {
            if let Some(bib) = &doc.bibliography {
                match bib.render_citation(keys, BibliographyStyle::AuthorYear, *mode) {
                    Ok(citation) => write!(output, "{}", citation).map_err(|_| LontarError::InternalError("write error".into()))?,
                    Err(_) => write!(output, "[?]").map_err(|_| LontarError::InternalError("write error".into()))?,
                }
            } else {
                write!(output, "[?]").map_err(|_| LontarError::InternalError("write error".into()))?;
            }
        }

        Inline::CrossRef { label, kind } => {
            match doc.resolve_crossref(label, *kind) {
                Some(text) => write!(output, "[{}](#{})", text, label).map_err(|_| LontarError::InternalError("write error".into()))?,
                None => write!(output, "[?](#{})", label).map_err(|_| LontarError::InternalError("write error".into()))?,
            }
        }

        Inline::InlineEquation { latex } => {
            write!(output, "${}$", latex).map_err(|_| LontarError::InternalError("write error".into()))?;
        }
    }

    Ok(())
}
