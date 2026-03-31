//! Markdown backend for Lontar documents.
//!
//! Converts Lontar AST to GitHub-flavored Markdown (GFM).

use crate::core::{
    BibliographyStore, Block, CitationMode, CrossRefKind, Document, DocumentWriter, Inline,
    WriteError, WriteReport, WriteResult,
};
use std::io::Write;

/// Markdown writer that outputs GitHub-flavored Markdown.
pub struct MarkdownWriter<W: Write> {
    output: W,
}

impl<W: Write> MarkdownWriter<W> {
    /// Create a new Markdown writer.
    pub fn new(output: W) -> Self {
        Self { output }
    }

    /// Write a block element.
    fn write_block(&mut self, block: &Block) -> WriteResult<()> {
        match block {
            Block::Heading { level, content, .. } => {
                // Clamp heading level to valid Markdown range (1-6)
                let clamped_level = level.min(&6);
                let hashes = "#".repeat(*clamped_level as usize);
                write!(self.output, "{hashes} ")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                self.write_inline_sequence(content)?;
                writeln!(self.output)
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                writeln!(self.output)
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
            }
            Block::Paragraph { content, .. } => {
                self.write_inline_sequence(content)?;
                writeln!(self.output)
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                writeln!(self.output)
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
            }
            Block::Table { rows, .. } => {
                for (i, row) in rows.iter().enumerate() {
                    write!(self.output, "|")
                        .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                    for cell in &row.cells {
                        for block in &cell.content {
                            match block {
                                Block::Paragraph { content, .. } => {
                                    self.write_inline_sequence(content)?;
                                }
                                Block::Heading { content, .. } => {
                                    self.write_inline_sequence(content)?;
                                }
                                Block::List { items, .. } => {
                                    // Simple list representation in table cells
                                    for item in items {
                                        if let Some(Block::Paragraph { content, .. }) =
                                            item.content.first()
                                        {
                                            self.write_inline_sequence(content)?;
                                        }
                                        if item.content.len() > 1 {
                                            write!(self.output, "...").map_err(|e| {
                                                WriteError::IoError { message: e.to_string() }
                                            })?;
                                        }
                                    }
                                }
                                Block::CodeBlock { code, .. } => {
                                    write!(self.output, "`{}`", code.trim()).map_err(|e| {
                                        WriteError::IoError { message: e.to_string() }
                                    })?;
                                }
                                _ => {
                                    // For other block types, try to extract text content
                                    write!(self.output, "...").map_err(|e| {
                                        WriteError::IoError { message: e.to_string() }
                                    })?;
                                }
                            }
                        }
                        write!(self.output, "|")
                            .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                    }
                    writeln!(self.output)
                        .map_err(|e| WriteError::IoError { message: e.to_string() })?;

                    if i == 0 && row.is_header {
                        write!(self.output, "|")
                            .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                        for _ in &row.cells {
                            write!(self.output, "---|")
                                .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                        }
                        writeln!(self.output)
                            .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                    }
                }
                writeln!(self.output)
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
            }
            Block::Chart { title, kind, data, .. } => {
                // Graceful degradation: convert chart to table
                if let Some(t) = title {
                    writeln!(self.output, "## {t}")
                        .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                    writeln!(self.output)
                        .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                }

                writeln!(self.output, "*Chart: {kind:?} (rendered as table)*")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                writeln!(self.output)
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;

                // Create table header
                write!(self.output, "| Category")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                for series in &data.series {
                    write!(self.output, "| {}", series.name)
                        .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                }
                writeln!(self.output, "|")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;

                // Create table separator
                write!(self.output, "|---")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                for _ in &data.series {
                    write!(self.output, "|---:")
                        .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                }
                writeln!(self.output, "|")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;

                // Create table rows
                for (i, category) in data.categories.iter().enumerate() {
                    write!(self.output, "| {category}")
                        .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                    for series in &data.series {
                        if i < series.values.len() {
                            write!(self.output, "| {}", series.values[i])
                                .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                        } else {
                            write!(self.output, "| -")
                                .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                        }
                    }
                    writeln!(self.output, "|")
                        .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                }

                writeln!(self.output)
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
            }
            Block::List { items, ordered, .. } => {
                for item in items {
                    let indent = "  ".repeat(item.level as usize);
                    let marker = if *ordered { "1. " } else { "- " };
                    write!(self.output, "{indent}{marker}")
                        .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                    for block in &item.content {
                        // Don't add extra newlines for list items - let the blocks handle their own spacing
                        match block {
                            Block::Paragraph { content, .. } => {
                                self.write_inline_sequence(content)?;
                                writeln!(self.output)
                                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                            }
                            _ => {
                                self.write_block(block)?;
                            }
                        }
                    }
                }
                writeln!(self.output)
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
            }
            Block::CodeBlock { code, language, .. } => {
                let lang = language.as_deref().unwrap_or("");
                writeln!(self.output, "```{lang}")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                write!(self.output, "{code}")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                writeln!(self.output, "\n```")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                writeln!(self.output)
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
            }
            Block::BlockQuote { content, .. } => {
                for block in content {
                    match block {
                        Block::Paragraph { content, .. } => {
                            write!(self.output, "> ")
                                .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                            self.write_inline_sequence(content)?;
                            writeln!(self.output)
                                .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                        }
                        _ => {
                            // For non-paragraph blocks, we need to prefix each line with "> "
                            // This is a simplified approach - a more complete solution would
                            // track the output line by line, but this handles common cases
                            write!(self.output, "> ")
                                .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                            self.write_block(block)?;
                        }
                    }
                }
                writeln!(self.output)
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
            }
            Block::HorizontalRule { .. } => {
                writeln!(self.output, "---")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                writeln!(self.output)
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
            }
            Block::Image { resource_id, alt_text, .. } => {
                let alt = alt_text.as_deref().unwrap_or("");
                writeln!(self.output, "![{alt}]({resource_id})")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                writeln!(self.output)
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
            }
            Block::Equation { content, display, .. } => {
                if *display {
                    writeln!(self.output, "$$\n{content}\n$$")
                        .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                } else {
                    write!(self.output, "${content}$")
                        .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                }
                writeln!(self.output)
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
            }
            Block::Section { title, content, .. } => {
                if let Some(t) = title {
                    writeln!(self.output, "## {t}")
                        .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                    writeln!(self.output)
                        .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                }
                for block in content {
                    self.write_block(block)?;
                }
            }
            Block::Bibliography { title: Some(t), .. } => {
                writeln!(self.output, "## {t}")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                writeln!(self.output)
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
            }
            Block::Bibliography { title: None, .. } => {}
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
                write!(self.output, "{text}")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
            }
            Inline::SoftBreak => {
                write!(self.output, " ")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
            }
            Inline::LineBreak => {
                writeln!(self.output)
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
            }
            Inline::Bold(content) => {
                write!(self.output, "**")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                self.write_inline_sequence(content)?;
                write!(self.output, "**")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
            }
            Inline::Italic(content) => {
                write!(self.output, "*")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                self.write_inline_sequence(content)?;
                write!(self.output, "*")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
            }
            Inline::Code(code) => {
                write!(self.output, "`{code}`")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
            }
            Inline::Strikethrough(content) => {
                write!(self.output, "~~")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                self.write_inline_sequence(content)?;
                write!(self.output, "~~")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
            }
            Inline::Link { text, url, .. } => {
                write!(self.output, "[")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                self.write_inline_sequence(text)?;
                write!(self.output, "]({url})")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
            }
            Inline::Image { resource_id, alt_text, .. } => {
                let alt = alt_text.as_deref().unwrap_or("");
                write!(self.output, "![{alt}]({resource_id})")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
            }
            Inline::Citation { key, mode, prefix, suffix } => match mode {
                CitationMode::Parenthetical => {
                    if let Some(pre) = prefix {
                        write!(self.output, "{pre} ")
                            .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                    }
                    write!(self.output, "({key})")
                        .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                    if let Some(suf) = suffix {
                        write!(self.output, " {suf}")
                            .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                    }
                }
                CitationMode::Narrative => {
                    if let Some(pre) = prefix {
                        write!(self.output, "{pre} ")
                            .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                    }
                    write!(self.output, "{key}")
                        .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                    if let Some(suf) = suffix {
                        write!(self.output, " {suf}")
                            .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                    }
                }
                CitationMode::YearOnly => {
                    if let Some(pre) = prefix {
                        write!(self.output, "{pre} ")
                            .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                    }
                    write!(self.output, "({key})")
                        .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                    if let Some(suf) = suffix {
                        write!(self.output, " {suf}")
                            .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                    }
                }
                CitationMode::SuppressAuthor => {
                    if let Some(pre) = prefix {
                        write!(self.output, "{pre} ")
                            .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                    }
                    write!(self.output, "({key})")
                        .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                    if let Some(suf) = suffix {
                        write!(self.output, " {suf}")
                            .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                    }
                }
                CitationMode::Full => {
                    if let Some(pre) = prefix {
                        write!(self.output, "{pre} ")
                            .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                    }
                    write!(self.output, "{key}")
                        .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                    if let Some(suf) = suffix {
                        write!(self.output, " {suf}")
                            .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                    }
                }
            },
            Inline::CrossRef { label, kind } => {
                let type_name = match kind {
                    CrossRefKind::Auto => "Reference",
                    CrossRefKind::Page => "Page",
                    CrossRefKind::Title => "Title",
                };
                write!(self.output, "[{type_name}](#{label})")
                    .map_err(|e| WriteError::IoError { message: e.to_string() })?;
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
        let report = WriteReport::new();

        // Write title if present
        if let Some(title) = &document.metadata.title {
            writeln!(self.output, "# {title}")
                .map_err(|e| WriteError::IoError { message: e.to_string() })?;
            writeln!(self.output).map_err(|e| WriteError::IoError { message: e.to_string() })?;
        }

        // Write content
        for block in &document.content {
            self.write_block(block)?;
        }

        // Write bibliography if present
        if let Some(bib) = &document.bibliography {
            writeln!(self.output, "## References")
                .map_err(|e| WriteError::IoError { message: e.to_string() })?;
            writeln!(self.output).map_err(|e| WriteError::IoError { message: e.to_string() })?;

            // Create a temporary bibliography store to render entries properly
            let mut bib_store = BibliographyStore::new(bib.style);
            for entry in bib.entries.values() {
                bib_store.add_entry((*entry).clone());
            }

            // Render each bibliography entry
            let mut entry_number = 1;
            for key in bib.entries.keys() {
                // Try to render the citation, fallback to key if it fails
                match bib_store.render_citation(&[key], CitationMode::Parenthetical) {
                    Ok(rendered) => {
                        // Only remove outermost brackets, not all brackets in the string
                        let trimmed = if rendered.starts_with('[') && rendered.ends_with(']') {
                            &rendered[1..rendered.len() - 1]
                        } else {
                            &rendered
                        };
                        writeln!(self.output, "{entry_number}. {trimmed}")
                            .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                    }
                    Err(_) => {
                        writeln!(self.output, "{entry_number}. {key}")
                            .map_err(|e| WriteError::IoError { message: e.to_string() })?;
                    }
                }
                entry_number += 1;
            }

            writeln!(self.output).map_err(|e| WriteError::IoError { message: e.to_string() })?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        ChartData, ChartKind, ChartSeries, CrossRefRegistry, DocumentMetadata, PageSetup,
        ResourceStore, StyleSheet,
    };
    use std::io::Cursor;

    #[test]
    fn test_markdown_writer_basic() {
        let document = Document {
            metadata: DocumentMetadata {
                title: Some("Test Document".to_string()),
                author: Some("Test Author".to_string()),
                ..Default::default()
            },
            page_setup: PageSetup::default(),
            stylesheet: StyleSheet::default(),
            content: vec![
                Block::Heading {
                    level: 1,
                    content: vec![Inline::Text("Introduction".to_string())],
                    style: None,
                },
                Block::Paragraph {
                    content: vec![
                        Inline::Text("This is a test paragraph with ".to_string()),
                        Inline::Bold(vec![Inline::Text("bold".to_string())]),
                        Inline::Text(" and ".to_string()),
                        Inline::Italic(vec![Inline::Text("italic".to_string())]),
                        Inline::Text(" text.".to_string()),
                    ],
                    style: None,
                },
            ],
            bibliography: None,
            resources: ResourceStore::default(),
            crossrefs: CrossRefRegistry::default(),
        };

        let mut output = Cursor::new(Vec::new());
        let mut writer = MarkdownWriter::new(&mut output);

        let result = writer.write(&document);
        assert!(result.is_ok());

        let markdown = String::from_utf8(output.into_inner()).unwrap();
        assert!(markdown.contains("# Test Document"));
        assert!(markdown.contains("# Introduction"));
        assert!(markdown.contains("**bold**"));
        assert!(markdown.contains("*italic*"));
    }

    #[test]
    fn test_markdown_writer_chart_to_table() {
        let document = Document {
            metadata: DocumentMetadata::default(),
            page_setup: PageSetup::default(),
            stylesheet: StyleSheet::default(),
            content: vec![Block::Chart {
                title: Some("Sales Data".to_string()),
                kind: ChartKind::Bar,
                data: ChartData {
                    categories: vec!["Q1".to_string(), "Q2".to_string()],
                    series: vec![ChartSeries {
                        name: "Revenue".to_string(),
                        values: vec![1000.0, 1500.0],
                    }],
                },
                style: None,
            }],
            bibliography: None,
            resources: ResourceStore::default(),
            crossrefs: CrossRefRegistry::default(),
        };

        let mut output = Cursor::new(Vec::new());
        let mut writer = MarkdownWriter::new(&mut output);

        let result = writer.write(&document);
        assert!(result.is_ok());

        let markdown = String::from_utf8(output.into_inner()).unwrap();
        assert!(markdown.contains("## Sales Data"));
        assert!(markdown.contains("Chart: Bar"));
        assert!(markdown.contains("| Category| Revenue|"));
        assert!(markdown.contains("| Q1| 1000|"));
        assert!(markdown.contains("| Q2| 1500|"));
    }

    #[test]
    fn test_markdown_writer_citations() {
        let document = Document {
            metadata: DocumentMetadata::default(),
            page_setup: PageSetup::default(),
            stylesheet: StyleSheet::default(),
            content: vec![Block::Paragraph {
                content: vec![
                    Inline::Text("According to ".to_string()),
                    Inline::Citation {
                        key: "smith2024".to_string(),
                        mode: CitationMode::Parenthetical,
                        prefix: Some("see".to_string()),
                        suffix: None,
                    },
                    Inline::Text(", the results show...".to_string()),
                ],
                style: None,
            }],
            bibliography: None,
            resources: ResourceStore::default(),
            crossrefs: CrossRefRegistry::default(),
        };

        let mut output = Cursor::new(Vec::new());
        let mut writer = MarkdownWriter::new(&mut output);

        let result = writer.write(&document);
        assert!(result.is_ok());

        let markdown = String::from_utf8(output.into_inner()).unwrap();
        assert!(markdown.contains("see (smith2024)"));
    }

    #[test]
    fn test_markdown_writer_cross_references() {
        let document = Document {
            metadata: DocumentMetadata::default(),
            page_setup: PageSetup::default(),
            stylesheet: StyleSheet::default(),
            content: vec![Block::Paragraph {
                content: vec![
                    Inline::Text("See ".to_string()),
                    Inline::CrossRef { label: "figure1".to_string(), kind: CrossRefKind::Auto },
                    Inline::Text(" for details.".to_string()),
                ],
                style: None,
            }],
            bibliography: None,
            resources: ResourceStore::default(),
            crossrefs: CrossRefRegistry::default(),
        };

        let mut output = Cursor::new(Vec::new());
        let mut writer = MarkdownWriter::new(&mut output);

        let result = writer.write(&document);
        assert!(result.is_ok());

        let markdown = String::from_utf8(output.into_inner()).unwrap();
        assert!(markdown.contains("[Reference](#figure1)"));
    }

    #[test]
    fn test_markdown_writer_heading_level_clamping() {
        let document = Document {
            metadata: DocumentMetadata::default(),
            page_setup: PageSetup::default(),
            stylesheet: StyleSheet::default(),
            content: vec![Block::Heading {
                level: 10, // Invalid level > 6
                content: vec![Inline::Text("Over-level heading".to_string())],
                style: None,
            }],
            bibliography: None,
            resources: ResourceStore::default(),
            crossrefs: CrossRefRegistry::default(),
        };

        let mut output = Cursor::new(Vec::new());
        let mut writer = MarkdownWriter::new(&mut output);

        let result = writer.write(&document);
        assert!(result.is_ok());

        let markdown = String::from_utf8(output.into_inner()).unwrap();
        // Should clamp to level 6 (######)
        assert!(markdown.contains("###### Over-level heading"));
        // Should NOT contain level 10 (##########)
        assert!(!markdown.contains("########## Over-level heading"));
    }

    #[cfg(feature = "md")]
    #[test]
    fn test_round_trip_verification() {
        use pulldown_cmark::{Event, Parser, Tag, TagEnd};

        // Create a test document with various elements
        let document = Document {
            metadata: DocumentMetadata {
                title: Some("Round Trip Test".to_string()),
                author: Some("Test Author".to_string()),
                ..Default::default()
            },
            page_setup: PageSetup::default(),
            stylesheet: StyleSheet::default(),
            content: vec![
                Block::Heading {
                    level: 1,
                    content: vec![Inline::Text("Main Title".to_string())],
                    style: None,
                },
                Block::Paragraph {
                    content: vec![
                        Inline::Text("This paragraph has ".to_string()),
                        Inline::Bold(vec![Inline::Text("bold".to_string())]),
                        Inline::Text(" and ".to_string()),
                        Inline::Italic(vec![Inline::Text("italic".to_string())]),
                        Inline::Text(" text with a ".to_string()),
                        Inline::Code("code".to_string()),
                        Inline::Text(" snippet.".to_string()),
                    ],
                    style: None,
                },
                Block::List {
                    items: vec![
                        crate::core::ListItem {
                            level: 0,
                            content: vec![Block::Paragraph {
                                content: vec![Inline::Text("First item".to_string())],
                                style: None,
                            }],
                        },
                        crate::core::ListItem {
                            level: 0,
                            content: vec![Block::Paragraph {
                                content: vec![Inline::Text("Second item".to_string())],
                                style: None,
                            }],
                        },
                    ],
                    ordered: false,
                    style: None,
                },
                Block::CodeBlock {
                    code: "println!(\"Hello, world!\");".to_string(),
                    language: Some("rust".to_string()),
                    style: None,
                },
            ],
            bibliography: None,
            resources: ResourceStore::default(),
            crossrefs: CrossRefRegistry::default(),
        };

        // Convert to Markdown
        let mut output = Cursor::new(Vec::new());
        let mut writer = MarkdownWriter::new(&mut output);
        let result = writer.write(&document);
        assert!(result.is_ok());

        let markdown = String::from_utf8(output.into_inner()).unwrap();

        // Parse the Markdown back to events
        let parser = Parser::new(&markdown);
        let events: Vec<Event> = parser.collect();

        // Verify key elements are preserved
        let mut found_title = false;
        let mut found_heading = false;
        let mut found_bold = false;
        let mut found_italic = false;
        let mut found_code = false;
        let mut found_list = false;
        let mut found_code_block = false;

        for event in &events {
            match event {
                Event::Text(text) => {
                    if text.contains("Round Trip Test") {
                        found_title = true;
                    }
                    if text.contains("Main Title") {
                        found_heading = true;
                    }
                    if text.contains("bold") {
                        found_bold = true;
                    }
                    if text.contains("italic") {
                        found_italic = true;
                    }
                    if text.contains("code") {
                        found_code = true;
                    }
                    if text.contains("First item") || text.contains("Second item") {
                        found_list = true;
                    }
                }
                Event::Code(code) => {
                    if code.contains("code") {
                        found_code = true;
                    }
                }
                Event::Start(Tag::Heading { .. }) => {
                    found_heading = true;
                }
                Event::Start(Tag::List { .. }) => {
                    found_list = true;
                }
                Event::Start(Tag::CodeBlock { .. }) => {
                    found_code_block = true;
                }
                Event::End(TagEnd::CodeBlock) => {
                    found_code_block = true;
                }
                _ => {}
            }
        }

        // Assert all key elements are preserved in the round trip
        assert!(found_title, "Document title should be preserved");
        assert!(found_heading, "Heading should be preserved");
        assert!(found_bold, "Bold text should be preserved");
        assert!(found_italic, "Italic text should be preserved");
        assert!(found_code, "Code should be preserved");
        assert!(found_list, "List should be preserved");
        assert!(found_code_block, "Code block should be preserved");

        // Additional verification: check that the markdown structure is valid
        assert!(markdown.starts_with("# Round Trip Test"), "Should start with document title");
        assert!(markdown.contains("# Main Title"), "Should contain heading");
        assert!(markdown.contains("**bold**"), "Should contain bold formatting");
        assert!(markdown.contains("*italic*"), "Should contain italic formatting");
        assert!(markdown.contains("`code`"), "Should contain inline code");
        assert!(markdown.contains("- First item"), "Should contain list items");
        assert!(markdown.contains("```rust"), "Should contain code block with language");
    }
}
