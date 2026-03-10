//! Core Abstract Syntax Tree (AST) types for Lontar documents.
//!
//! This module defines the fundamental document structure that all backends
//! consume. The AST is designed to be:
//! - **Unified**: Single representation for all document types
//! - **Extensible**: Supports custom styles and metadata
//! - **Backend-agnostic**: No assumptions about output format
//! - **Semantic**: Preserves meaning (not just formatting)

use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

/// A complete Lontar document.
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

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CrossRefError {
    #[error("duplicate cross-reference label: {0}")]
    DuplicateLabel(String),
    #[error("unknown cross-reference label: {0}")]
    UnknownLabel(String),
}

/// Registry for cross-references to enforce uniqueness and provide resolution.
#[derive(Debug, Clone, Default)]
pub struct CrossRefRegistry {
    labels: HashMap<String, usize>,
}

impl CrossRefRegistry {
    /// Register a label and assign a sequential number (1-based). Errors on duplicates.
    pub fn register(&mut self, label: impl Into<String>) -> Result<usize, CrossRefError> {
        let label = label.into();
        if self.labels.contains_key(&label) {
            return Err(CrossRefError::DuplicateLabel(label));
        }
        let number = self.labels.len() + 1;
        self.labels.insert(label, number);
        Ok(number)
    }

    /// Resolve a label to its number as string.
    pub fn resolve(&self, label: &str) -> Result<String, CrossRefError> {
        self.labels
            .get(label)
            .map(|n| n.to_string())
            .ok_or_else(|| CrossRefError::UnknownLabel(label.to_string()))
    }

    /// Number of registered labels.
    pub fn len(&self) -> usize {
        self.labels.len()
    }

    /// Check if registry is empty.
    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }
}

/// Chart kinds supported by the AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChartKind {
    Bar,
    Line,
    Pie,
    Scatter,
    Area,
}

/// Chart data representation (categories + series values).
#[derive(Debug, Clone)]
pub struct ChartData {
    pub categories: Vec<String>,
    pub series: Vec<ChartSeries>,
}

/// A single chart series.
#[derive(Debug, Clone)]
pub struct ChartSeries {
    pub name: String,
    pub values: Vec<f64>,
}

/// Document metadata (title, author, creation date, etc.).
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
#[derive(Debug, Clone)]
pub struct PageSetup {
    pub page_width: f32,  // in points
    pub page_height: f32, // in points
    pub margins: Margins,
    pub orientation: PageOrientation,
}

#[derive(Debug, Clone, Copy)]
pub enum PageOrientation {
    Portrait,
    Landscape,
}

/// Margins (top, bottom, left, right).
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
    Link {
        text: Vec<Inline>,
        url: String,
        title: Option<String>,
    },
    Image {
        resource_id: String,
        alt_text: Option<String>,
        title: Option<String>,
    },
    Citation {
        key: String,
        mode: CitationMode,
        prefix: Option<String>,
        suffix: Option<String>,
    },
    CrossRef {
        label: String,
        kind: CrossRefKind,
    },
    Styled {
        content: Vec<Inline>,
        style: String,
    },
}

/// Citation mode (how the citation is rendered).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CitationMode {
    /// (Author Year)
    Parenthetical,
    /// Author (Year)
    Narrative,
    /// (Year)
    YearOnly,
    /// (Year) without author
    SuppressAuthor,
    /// Full citation
    Full,
}

/// Cross-reference kind (what to display).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossRefKind {
    /// Auto-generated number (e.g., "1", "Figure 1")
    Auto,
    /// Page number
    Page,
    /// Section/figure title
    Title,
}

/// Text styling (font, size, color, etc.).
#[derive(Debug, Clone, Default)]
pub struct TextStyle {
    pub font_family: Option<String>,
    pub font_size: Option<f32>,
    pub font_weight: Option<FontWeight>,
    pub font_style: Option<FontStyle>,
    pub color: Option<Color>,
    pub background_color: Option<Color>,
    pub underline: bool,
    pub strikethrough: bool,
    pub script: Option<Script>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Script {
    Latin,
    Arabic,
    Devanagari,
    Bengali,
    Tamil,
    Telugu,
    Kannada,
    Malayalam,
    Thai,
    Lao,
    Khmer,
    Myanmar,
    Han,
    Hiragana,
    Katakana,
    Hangul,
    Balinese,
    Hebrew,
    Cyrillic,
    Greek,
}

/// Paragraph styling (alignment, spacing, indentation, etc.).
#[derive(Debug, Clone, Default)]
pub struct ParagraphStyle {
    pub alignment: Option<Alignment>,
    pub line_spacing: Option<f32>,
    pub space_before: Option<f32>,
    pub space_after: Option<f32>,
    pub indent_first: Option<f32>,
    pub indent_left: Option<f32>,
    pub indent_right: Option<f32>,
    pub keep_together: bool,
    pub keep_with_next: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
    Justify,
}

/// Table styling.
#[derive(Debug, Clone, Default)]
pub struct TableStyle {
    pub border_width: Option<f32>,
    pub border_color: Option<Color>,
    pub cell_padding: Option<f32>,
    pub header_background: Option<Color>,
}

/// Stylesheet with named styles and cascading resolution.
#[derive(Debug, Clone, Default)]
pub struct StyleSheet {
    pub text_styles: HashMap<String, TextStyle>,
    pub paragraph_styles: HashMap<String, ParagraphStyle>,
    pub table_styles: HashMap<String, TableStyle>,
    pub default_text: TextStyle,
    pub default_paragraph: ParagraphStyle,
}

impl StyleSheet {
    /// Resolve a text style by name, falling back to defaults.
    pub fn resolve_text_style(&self, name: Option<&str>) -> TextStyle {
        match name {
            Some(n) => self
                .text_styles
                .get(n)
                .cloned()
                .unwrap_or_else(|| self.default_text.clone()),
            None => self.default_text.clone(),
        }
    }

    /// Resolve a paragraph style by name, falling back to defaults.
    pub fn resolve_paragraph_style(&self, name: Option<&str>) -> ParagraphStyle {
        match name {
            Some(n) => self
                .paragraph_styles
                .get(n)
                .cloned()
                .unwrap_or_else(|| self.default_paragraph.clone()),
            None => self.default_paragraph.clone(),
        }
    }

    /// Resolve a table style by name, falling back to defaults.
    pub fn resolve_table_style(&self, name: Option<&str>) -> TableStyle {
        match name {
            Some(n) => self.table_styles.get(n).cloned().unwrap_or_default(),
            None => TableStyle::default(),
        }
    }
}

/// Bibliography storage and management.
#[derive(Debug, Clone)]
pub struct BibliographyStore {
    pub entries: HashMap<String, BibEntry>,
    pub style: BibliographyStyle,
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum BibliographyError {
    #[error("invalid BibTeX entry")]
    InvalidBibtex,
    #[error("missing BibTeX key")]
    MissingBibtexKey,
    #[error("invalid CSL-JSON")]
    InvalidCslJson,
    #[error("missing CSL-JSON id")]
    MissingCslId,
    #[error("citation key not found: {0}")]
    MissingEntry(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BibliographyStyle {
    Numeric,
    AuthorYear,
    Vancouver,
    Superscript,
    APA7,
    Named,
}

/// Bibliography entry (from BibTeX or CSL-JSON).
#[derive(Debug, Clone)]
pub struct BibEntry {
    pub key: String,
    pub kind: BibEntryKind,
    pub fields: HashMap<String, String>,
    pub authors: Vec<BibAuthor>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BibEntryKind {
    Article,
    Book,
    InProceedings,
    InCollection,
    Thesis,
    Misc,
    TechReport,
    WebPage,
}

/// Bibliography author.
#[derive(Debug, Clone)]
pub struct BibAuthor {
    pub given: Option<String>,
    pub family: String,
    pub literal: Option<String>,
}

impl BibliographyStore {
    pub fn new(style: BibliographyStyle) -> Self {
        Self {
            entries: HashMap::new(),
            style,
        }
    }

    /// Add an entry to the bibliography.
    pub fn add_entry(&mut self, entry: BibEntry) {
        self.entries.insert(entry.key.clone(), entry);
    }

    /// Get an entry by key.
    pub fn get(&self, key: &str) -> Option<&BibEntry> {
        self.entries.get(key)
    }

    /// Check if an entry exists.
    pub fn contains(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }

    /// Load entries from a BibTeX string.
    pub fn load_bibtex(&mut self, input: &str) -> Result<(), BibliographyError> {
        for entry in parse_bibtex_entries(input)? {
            self.add_entry(entry);
        }
        Ok(())
    }

    /// Load entries from a CSL-JSON string.
    pub fn load_csl_json(&mut self, input: &str) -> Result<(), BibliographyError> {
        for entry in parse_csl_json(input)? {
            self.add_entry(entry);
        }
        Ok(())
    }

    /// Render a citation for the given keys using the configured bibliography style.
    pub fn render_citation(
        &self,
        keys: &[&str],
        mode: CitationMode,
    ) -> Result<String, BibliographyError> {
        match self.style {
            BibliographyStyle::Numeric
            | BibliographyStyle::Vancouver
            | BibliographyStyle::Superscript => {
                let numbers: Vec<String> = keys
                    .iter()
                    .enumerate()
                    .map(|(i, key)| {
                        if self.contains(key) {
                            // Use 1-based numbering in order of appearance.
                            (i + 1).to_string()
                        } else {
                            key.to_string()
                        }
                    })
                    .collect();
                let body = numbers.join(", ");
                let rendered = if matches!(self.style, BibliographyStyle::Superscript) {
                    format!("^{body}^")
                } else if matches!(mode, CitationMode::Narrative) {
                    body.clone()
                } else {
                    format!("[{body}]")
                };
                // validate all keys exist
                for key in keys {
                    if !self.contains(key) {
                        return Err(BibliographyError::MissingEntry((*key).to_string()));
                    }
                }
                Ok(rendered)
            }
            BibliographyStyle::AuthorYear | BibliographyStyle::APA7 | BibliographyStyle::Named => {
                let mut parts = Vec::new();
                for key in keys {
                    let entry = self
                        .get(key)
                        .ok_or_else(|| BibliographyError::MissingEntry((*key).to_string()))?;
                    parts.push(format_author_year(entry));
                }
                let body = parts.join("; ");
                let rendered = match mode {
                    CitationMode::Narrative => body.clone(),
                    CitationMode::YearOnly => {
                        let year = extract_year(&body);
                        format!("({year})")
                    }
                    CitationMode::SuppressAuthor => {
                        let year = extract_year(&body);
                        format!("({year})")
                    }
                    _ => format!("({body})"),
                };
                Ok(rendered)
            }
        }
    }
}

fn format_author_year(entry: &BibEntry) -> String {
    let author = entry
        .authors
        .first()
        .map(|a| a.family.as_str())
        .unwrap_or("Anon");
    let year = entry
        .fields
        .get("year")
        .or_else(|| entry.fields.get("issued"))
        .map(String::as_str)
        .unwrap_or("n.d.");
    format!("{author} {year}")
}

fn extract_year(text: &str) -> String {
    text.split_whitespace()
        .find(|s| s.chars().all(|c| c.is_ascii_digit()))
        .unwrap_or("n.d.")
        .to_string()
}

fn parse_bibtex_entries(input: &str) -> Result<Vec<BibEntry>, BibliographyError> {
    let mut entries = Vec::new();
    for chunk in input.split('@') {
        let trimmed = chunk.trim();
        if trimmed.is_empty() {
            continue;
        }

        let mut parts = trimmed.splitn(2, '{');
        let kind_str = parts.next().ok_or(BibliographyError::InvalidBibtex)?;
        let rest = parts.next().ok_or(BibliographyError::InvalidBibtex)?;

        let mut key_and_body = rest.splitn(2, ',');
        let key = key_and_body
            .next()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or(BibliographyError::MissingBibtexKey)?;
        let body = key_and_body.next().unwrap_or("");

        let fields_str = body.rsplitn(2, '}').last().unwrap_or("");
        let mut fields = HashMap::new();
        let mut authors = Vec::new();

        for field in fields_str.split(',') {
            let mut kv = field.splitn(2, '=');
            if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
                let key_trim = k.trim().to_lowercase();
                let value_trim = v
                    .trim()
                    .trim_matches('{')
                    .trim_matches('}')
                    .trim_matches('"');
                if key_trim == "author" {
                    for name in value_trim.split(" and ") {
                        let parts: Vec<_> = name.split_whitespace().collect();
                        if parts.is_empty() {
                            continue;
                        }
                        let family = parts.last().unwrap().to_string();
                        let given = if parts.len() > 1 {
                            Some(parts[..parts.len() - 1].join(" "))
                        } else {
                            None
                        };
                        authors.push(BibAuthor {
                            given,
                            family,
                            literal: None,
                        });
                    }
                } else {
                    fields.insert(key_trim, value_trim.to_string());
                }
            }
        }

        let entry = BibEntry {
            key: key.to_string(),
            kind: map_bibtex_kind(kind_str),
            fields,
            authors,
        };

        entries.push(entry);
    }

    Ok(entries)
}

fn map_bibtex_kind(kind: &str) -> BibEntryKind {
    match kind.to_lowercase().as_str() {
        "article" => BibEntryKind::Article,
        "book" => BibEntryKind::Book,
        "inproceedings" | "conference" => BibEntryKind::InProceedings,
        "incollection" => BibEntryKind::InCollection,
        "phdthesis" | "mastersthesis" | "thesis" => BibEntryKind::Thesis,
        "techreport" => BibEntryKind::TechReport,
        "misc" => BibEntryKind::Misc,
        "online" | "webpage" => BibEntryKind::WebPage,
        _ => BibEntryKind::Misc,
    }
}

fn parse_csl_json(input: &str) -> Result<Vec<BibEntry>, BibliographyError> {
    let value: Value =
        serde_json::from_str(input).map_err(|_| BibliographyError::InvalidCslJson)?;
    let items = match value {
        Value::Array(arr) => arr,
        Value::Object(_) => vec![value],
        _ => return Err(BibliographyError::InvalidCslJson),
    };

    let mut entries = Vec::new();
    for item in items {
        let obj = item.as_object().ok_or(BibliographyError::InvalidCslJson)?;
        let id = obj
            .get("id")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .ok_or(BibliographyError::MissingCslId)?;

        let mut fields = HashMap::new();
        let mut authors = Vec::new();

        if let Some(title) = obj.get("title").and_then(|v| v.as_str()) {
            fields.insert("title".to_string(), title.to_string());
        }
        if let Some(container) = obj.get("container-title").and_then(|v| v.as_str()) {
            fields.insert("container-title".to_string(), container.to_string());
        }
        if let Some(issued) = obj.get("issued").and_then(|v| v.as_str()) {
            fields.insert("issued".to_string(), issued.to_string());
        }

        if let Some(author_array) = obj.get("author").and_then(|v| v.as_array()) {
            for a in author_array {
                if let Some(aobj) = a.as_object() {
                    let given = aobj
                        .get("given")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    let family = aobj
                        .get("family")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let literal = aobj
                        .get("literal")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    if !family.is_empty() || literal.is_some() {
                        authors.push(BibAuthor {
                            given,
                            family,
                            literal,
                        });
                    }
                }
            }
        }

        let entry = BibEntry {
            key: id.to_string(),
            kind: BibEntryKind::Article,
            fields,
            authors,
        };

        entries.push(entry);
    }

    Ok(entries)
}

/// Resource store for images and binary assets.
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
        self.resources.insert(
            id.clone(),
            Resource {
                id,
                mime_type,
                data,
            },
        );
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
        };

        assert_eq!(doc.metadata.title, Some("Test Document".to_string()));
    }

    #[test]
    fn test_style_resolution() {
        let mut stylesheet = StyleSheet::default();
        stylesheet.text_styles.insert(
            "emphasis".to_string(),
            TextStyle {
                font_style: Some(FontStyle::Italic),
                ..Default::default()
            },
        );

        let resolved = stylesheet.resolve_text_style(Some("emphasis"));
        assert_eq!(resolved.font_style, Some(FontStyle::Italic));
    }

    #[test]
    fn test_bibliography_store() {
        let mut bib = BibliographyStore::new(BibliographyStyle::APA7);

        let entry = BibEntry {
            key: "key2024".to_string(),
            kind: BibEntryKind::Article,
            fields: {
                let mut m = HashMap::new();
                m.insert("title".to_string(), "Test Article".to_string());
                m
            },
            authors: vec![BibAuthor {
                given: Some("John".to_string()),
                family: "Doe".to_string(),
                literal: None,
            }],
        };

        bib.add_entry(entry);
        assert!(bib.contains("key2024"));
    }

    #[test]
    fn test_resource_store() {
        let mut store = ResourceStore::default();
        store.add_resource(
            "image1".to_string(),
            "image/png".to_string(),
            vec![1, 2, 3, 4],
        );

        assert!(store.contains("image1"));
        assert_eq!(store.get("image1").unwrap().mime_type, "image/png");
    }
}
