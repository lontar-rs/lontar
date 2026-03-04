//! Bibliography types and citation management.
//!
//! This module provides types for managing bibliographic data,
//! parsing BibTeX and CSL-JSON files, and rendering citations.

use crate::ast::{BibliographyStyle, CitationMode};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

/// Bibliography store containing all bibliographic entries
#[derive(Debug, Clone, Default)]
pub struct BibliographyStore {
    entries: HashMap<String, BibEntry>,
    cited_keys: Vec<String>,
    citation_numbers: HashMap<String, usize>,
}

/// Single bibliographic entry
#[derive(Debug, Clone)]
pub struct BibEntry {
    pub key: String,
    pub entry_type: BibEntryKind,
    pub authors: Vec<BibAuthor>,
    pub title: String,
    pub year: Option<u32>,
    pub fields: HashMap<String, String>,
}

/// Bibliographic entry type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BibEntryKind {
    Article,
    Book,
    InProceedings,
    PhdThesis,
    MastersThesis,
    TechReport,
    Misc,
    Online,
}

/// Author information
#[derive(Debug, Clone)]
pub struct BibAuthor {
    pub given: Option<String>,
    pub family: String,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

impl BibliographyStore {
    /// Create a new empty bibliography store
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a bibliographic entry
    pub fn add_entry(&mut self, entry: BibEntry) {
        self.entries.insert(entry.key.clone(), entry);
    }

    /// Get an entry by key
    pub fn get_entry(&self, key: &str) -> Option<&BibEntry> {
        self.entries.get(key)
    }

    /// Mark a citation key as cited
    pub fn cite(&mut self, key: &str) -> Result<(), BibError> {
        if !self.entries.contains_key(key) {
            return Err(BibError::CitationNotFound(key.to_string()));
        }

        if !self.cited_keys.contains(&key.to_string()) {
            let citation_number = self.cited_keys.len() + 1;
            self.citation_numbers.insert(key.to_string(), citation_number);
            self.cited_keys.push(key.to_string());
        }

        Ok(())
    }

    /// Get citation number for a key (1-indexed)
    pub fn get_citation_number(&self, key: &str) -> Option<usize> {
        self.citation_numbers.get(key).copied()
    }

    /// Get all cited entries in citation order
    pub fn cited_entries(&self) -> Vec<&BibEntry> {
        self.cited_keys
            .iter()
            .filter_map(|key| self.entries.get(key))
            .collect()
    }

    /// Get all entries (for alphabetical bibliography)
    pub fn all_entries(&self) -> Vec<&BibEntry> {
        self.entries.values().collect()
    }

    /// Load bibliography from BibTeX file
    pub fn load_bibtex(path: &Path) -> Result<Self, BibError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| BibError::IoError(e.to_string()))?;
        Self::from_bibtex_str(&content)
    }

    /// Load bibliography from BibTeX string (minimal parser for Phase 1)
    pub fn from_bibtex_str(content: &str) -> Result<Self, BibError> {
        let mut store = Self::new();

        for entry_str in split_bib_entries(content) {
            let parsed = parse_bibtex_entry(entry_str)?;
            store.add_entry(parsed);
        }

        Ok(store)
    }

    /// Load bibliography from CSL-JSON file
    pub fn load_csl_json(path: &Path) -> Result<Self, BibError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| BibError::IoError(e.to_string()))?;
        Self::from_csl_json_str(&content)
    }

    /// Load bibliography from CSL-JSON string
    pub fn from_csl_json_str(content: &str) -> Result<Self, BibError> {
        let csl_entries: Vec<CslEntry> = serde_json::from_str(content)
            .map_err(|e| BibError::ParseError(e.to_string()))?;

        let mut store = Self::new();
        for csl in csl_entries {
            let entry = convert_csl_entry(csl);
            store.add_entry(entry);
        }

        Ok(store)
    }

    /// Render citation for given keys and style
    pub fn render_citation(
        &self,
        keys: &[String],
        style: BibliographyStyle,
        mode: CitationMode,
    ) -> Result<String, BibError> {
        match style {
            BibliographyStyle::Numeric => self.render_numeric_citation(keys),
            BibliographyStyle::Vancouver => self.render_vancouver_citation(keys),
            BibliographyStyle::Superscript => self.render_superscript_citation(keys),
            BibliographyStyle::AuthorYear => self.render_author_year_citation(keys, mode),
            BibliographyStyle::Apa7 => self.render_apa_citation(keys, mode),
            BibliographyStyle::Named(_) => {
                // TODO: Implement CSL processor
                Err(BibError::UnsupportedStyle)
            }
        }
    }

    /// Render numeric citation [1], [2], [1,2], [1-3]
    fn render_numeric_citation(&self, keys: &[String]) -> Result<String, BibError> {
        let numbers: Vec<usize> = keys
            .iter()
            .map(|key| {
                self.get_citation_number(key)
                    .ok_or_else(|| BibError::CitationNotFound(key.clone()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(format!("[{}]", format_number_range(&numbers)))
    }

    /// Render Vancouver citation (superscript or numeric)
    fn render_vancouver_citation(&self, keys: &[String]) -> Result<String, BibError> {
        let numbers: Vec<usize> = keys
            .iter()
            .map(|key| {
                self.get_citation_number(key)
                    .ok_or_else(|| BibError::CitationNotFound(key.clone()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(format_number_range(&numbers))
    }

    /// Render superscript citation ¹, ², ¹⁻³
    fn render_superscript_citation(&self, keys: &[String]) -> Result<String, BibError> {
        let numbers: Vec<usize> = keys
            .iter()
            .map(|key| {
                self.get_citation_number(key)
                    .ok_or_else(|| BibError::CitationNotFound(key.clone()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(format_superscript_range(&numbers))
    }

    /// Render author-year citation (Smith, 2024) or (Smith & Jones, 2024)
    fn render_author_year_citation(
        &self,
        keys: &[String],
        mode: CitationMode,
    ) -> Result<String, BibError> {
        let citations: Vec<String> = keys
            .iter()
            .map(|key| {
                let entry = self
                    .get_entry(key)
                    .ok_or_else(|| BibError::CitationNotFound(key.clone()))?;

                let author = if entry.authors.is_empty() {
                    "Unknown".to_string()
                } else if entry.authors.len() == 1 {
                    entry.authors[0].family.clone()
                } else if entry.authors.len() == 2 {
                    format!("{} & {}", entry.authors[0].family, entry.authors[1].family)
                } else {
                    format!("{} et al.", entry.authors[0].family)
                };

                let year = entry
                    .year
                    .map(|y| y.to_string())
                    .unwrap_or_else(|| "n.d.".to_string());

                Ok(match mode {
                    CitationMode::Parenthetical => format!("{}, {}", author, year),
                    CitationMode::Narrative => format!("{} ({})", author, year),
                    CitationMode::YearOnly => year,
                    CitationMode::SuppressAuthor => year,
                    CitationMode::Full => format!("{}, {}", author, year),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(match mode {
            CitationMode::Parenthetical => format!("({})", citations.join("; ")),
            CitationMode::Narrative => citations.join(" and "),
            _ => citations.join("; "),
        })
    }

    /// Render APA 7th edition citation
    fn render_apa_citation(
        &self,
        keys: &[String],
        mode: CitationMode,
    ) -> Result<String, BibError> {
        // APA is similar to author-year
        self.render_author_year_citation(keys, mode)
    }

    /// Render bibliography entries
    pub fn render_bibliography(&self, style: BibliographyStyle) -> Result<Vec<String>, BibError> {
        match style {
            BibliographyStyle::Numeric | BibliographyStyle::Vancouver | BibliographyStyle::Superscript => {
                // Numeric styles: order by citation
                Ok(self
                    .cited_entries()
                    .iter()
                    .enumerate()
                    .map(|(i, entry)| format!("{}. {}", i + 1, format_entry_vancouver(entry)))
                    .collect())
            }
            BibliographyStyle::AuthorYear | BibliographyStyle::Apa7 => {
                // Author-year styles: alphabetical order
                let mut entries = self.all_entries();
                entries.sort_by(|a, b| {
                    let a_author = a.authors.first().map(|au| &au.family).unwrap_or(&a.key);
                    let b_author = b.authors.first().map(|au| &au.family).unwrap_or(&b.key);
                    a_author.cmp(b_author)
                });
                Ok(entries.iter().map(|entry| format_entry_apa(entry)).collect())
            }
            BibliographyStyle::Named(_) => {
                // TODO: Implement CSL processor
                Err(BibError::UnsupportedStyle)
            }
        }
    }
}

/// Format citation number range: [1,2,3] -> "1-3", [1,3,5] -> "1,3,5"
fn format_number_range(numbers: &[usize]) -> String {
    if numbers.is_empty() {
        return String::new();
    }

    if numbers.len() == 1 {
        return numbers[0].to_string();
    }

    // Check if consecutive
    let mut sorted = numbers.to_vec();
    sorted.sort_unstable();

    let is_consecutive = sorted.windows(2).all(|w| w[1] == w[0] + 1);

    if is_consecutive {
        format!("{}-{}", sorted[0], sorted[sorted.len() - 1])
    } else {
        sorted
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

/// Format superscript number range
fn format_superscript_range(numbers: &[usize]) -> String {
    let range = format_number_range(numbers);
    // Convert to superscript Unicode characters
    range
        .chars()
        .map(|c| match c {
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            '-' => '⁻',
            ',' => ',',
            _ => c,
        })
        .collect()
}

/// Format entry in Vancouver style
fn format_entry_vancouver(entry: &BibEntry) -> String {
    let authors = format_authors_vancouver(&entry.authors);
    let title = &entry.title;
    let year = entry
        .year
        .map(|y| y.to_string())
        .unwrap_or_else(|| "n.d.".to_string());

    match entry.entry_type {
        BibEntryKind::Article => {
            let journal = entry.fields.get("journal").map(|s| s.as_str()).unwrap_or("");
            let volume = entry.fields.get("volume").map(|s| s.as_str()).unwrap_or("");
            let pages = entry.fields.get("pages").map(|s| s.as_str()).unwrap_or("");
            format!("{}. {}. {}. {};{}:{}.", authors, title, journal, year, volume, pages)
        }
        BibEntryKind::Book => {
            let publisher = entry.fields.get("publisher").map(|s| s.as_str()).unwrap_or("");
            format!("{}. {}. {}; {}.", authors, title, publisher, year)
        }
        _ => format!("{}. {}. {}.", authors, title, year),
    }
}

/// Format authors in Vancouver style (Last Initials)
fn format_authors_vancouver(authors: &[BibAuthor]) -> String {
    if authors.is_empty() {
        return "Unknown".to_string();
    }

    let formatted: Vec<String> = authors
        .iter()
        .take(6)
        .map(|author| {
            let initials = author
                .given
                .as_ref()
                .map(|g| {
                    g.split_whitespace()
                        .map(|part| format!("{}", part.chars().next().unwrap()))
                        .collect::<String>()
                })
                .unwrap_or_default();
            format!("{} {}", author.family, initials)
        })
        .collect();

    if authors.len() > 6 {
        format!("{}, et al", formatted.join(", "))
    } else {
        formatted.join(", ")
    }
}

/// Format entry in APA style
fn format_entry_apa(entry: &BibEntry) -> String {
    let authors = format_authors_apa(&entry.authors);
    let year = entry
        .year
        .map(|y| y.to_string())
        .unwrap_or_else(|| "n.d.".to_string());
    let title = &entry.title;

    match entry.entry_type {
        BibEntryKind::Article => {
            let journal = entry.fields.get("journal").map(|s| s.as_str()).unwrap_or("");
            let volume = entry.fields.get("volume").map(|s| s.as_str()).unwrap_or("");
            let pages = entry.fields.get("pages").map(|s| s.as_str()).unwrap_or("");
            format!("{}. ({}). {}. {}, {}, {}.", authors, year, title, journal, volume, pages)
        }
        BibEntryKind::Book => {
            let publisher = entry.fields.get("publisher").map(|s| s.as_str()).unwrap_or("");
            format!("{}. ({}). {}. {}.", authors, year, title, publisher)
        }
        _ => format!("{}. ({}). {}.", authors, year, title),
    }
}

/// Format authors in APA style (Last, F. M.)
fn format_authors_apa(authors: &[BibAuthor]) -> String {
    if authors.is_empty() {
        return "Unknown".to_string();
    }

    let formatted: Vec<String> = authors
        .iter()
        .take(20)
        .map(|author| {
            let initials = author
                .given
                .as_ref()
                .map(|g| {
                    g.split_whitespace()
                        .map(|part| format!("{}.", part.chars().next().unwrap()))
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .unwrap_or_default();
            format!("{}, {}", author.family, initials)
        })
        .collect();

    if authors.len() == 1 {
        formatted[0].clone()
    } else if authors.len() <= 20 {
        let (init, last) = formatted.split_at(formatted.len() - 1);
        format!("{}, & {}", init.join(", "), last[0])
    } else {
        let last = &authors[authors.len() - 1];
        let last_initials = last
            .given
            .as_ref()
            .map(|g| format!("{}.", g.chars().next().unwrap()))
            .unwrap_or_default();
        format!("{}, ... {}, {}", formatted.join(", "), last.family, last_initials)
    }
}

/// Bibliography-related errors
#[derive(Debug, Clone)]
pub enum BibError {
    CitationNotFound(String),
    ParseError(String),
    UnsupportedStyle,
    IoError(String),
}

impl std::fmt::Display for BibError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BibError::CitationNotFound(key) => write!(f, "Citation not found: {}", key),
            BibError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            BibError::UnsupportedStyle => write!(f, "Unsupported bibliography style"),
            BibError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for BibError {}

/// --- BibTeX parsing helpers ---

fn split_bib_entries(content: &str) -> Vec<String> {
    content
        .split('@')
        .filter(|s| !s.trim().is_empty())
        .map(|s| format!("@{}", s))
        .collect()
}

fn parse_bibtex_entry(entry: &str) -> Result<BibEntry, BibError> {
    // Very small, tolerant parser: assumes format @type{key, field = {value}, ...}
    let entry = entry.trim();
    let at_pos = entry.find('@').ok_or_else(|| BibError::ParseError("Missing @".into()))?;
    let brace_pos = entry[at_pos + 1..]
        .find('{')
        .ok_or_else(|| BibError::ParseError("Missing {".into()))?
        + at_pos
        + 1;

    let entry_type = entry[at_pos + 1..brace_pos].trim();
    let rest = entry[brace_pos + 1..].trim();
    let closing = rest.rfind('}').ok_or_else(|| BibError::ParseError("Missing closing }".into()))?;
    let body = &rest[..closing];

    let mut parts = body.splitn(2, ',');
    let key = parts
        .next()
        .ok_or_else(|| BibError::ParseError("Missing citation key".into()))?
        .trim()
        .to_string();
    let fields_str = parts.next().unwrap_or("");

    let mut fields = HashMap::new();
    for field in fields_str.split(',') {
        if let Some((name, value)) = field.split_once('=') {
            let name = name.trim().to_lowercase();
            let value = value.trim().trim_matches('{').trim_matches('}').trim_matches('"').to_string();
            fields.insert(name, value);
        }
    }

    let authors = fields
        .get("author")
        .map(|s| parse_authors(s))
        .unwrap_or_default();
    let title = fields.get("title").cloned().unwrap_or_default();
    let year = fields
        .get("year")
        .and_then(|y| y.parse::<u32>().ok());

    let entry_type = match entry_type.to_lowercase().as_str() {
        "article" => BibEntryKind::Article,
        "book" => BibEntryKind::Book,
        "inproceedings" | "conference" => BibEntryKind::InProceedings,
        "phdthesis" => BibEntryKind::PhdThesis,
        "mastersthesis" => BibEntryKind::MastersThesis,
        "techreport" | "report" => BibEntryKind::TechReport,
        "online" | "misc" => BibEntryKind::Online,
        _ => BibEntryKind::Misc,
    };

    // Preserve fields (except author/title/year already split)
    let mut extra_fields = fields.clone();
    extra_fields.remove("author");
    extra_fields.remove("title");
    extra_fields.remove("year");

    Ok(BibEntry {
        key,
        entry_type,
        authors,
        title,
        year,
        fields: extra_fields,
    })
}

fn parse_authors(s: &str) -> Vec<BibAuthor> {
    s.split(" and ")
        .filter(|part| !part.trim().is_empty())
        .map(|part| {
            let part = part.trim();
            // Support "Last, First" or "First Last"
            if let Some((family, given)) = part.split_once(',') {
                BibAuthor {
                    given: Some(given.trim().to_string()),
                    family: family.trim().to_string(),
                    prefix: None,
                    suffix: None,
                }
            } else {
                let mut pieces = part.split_whitespace().collect::<Vec<_>>();
                if pieces.len() >= 2 {
                    let family = pieces.pop().unwrap().to_string();
                    let given = pieces.join(" ");
                    BibAuthor {
                        given: Some(given),
                        family,
                        prefix: None,
                        suffix: None,
                    }
                } else {
                    BibAuthor {
                        given: None,
                        family: part.to_string(),
                        prefix: None,
                        suffix: None,
                    }
                }
            }
        })
        .collect()
}

/// --- CSL-JSON parsing helpers ---

#[derive(Debug, Deserialize)]
struct CslEntry {
    id: String,
    #[serde(rename = "type")]
    entry_type: String,
    #[serde(default)]
    author: Vec<CslPerson>,
    title: Option<String>,
    #[serde(rename = "container-title")]
    container_title: Option<String>,
    issued: Option<CslDate>,
    volume: Option<String>,
    issue: Option<String>,
    page: Option<String>,
    #[serde(rename = "DOI")]
    doi: Option<String>,
    #[serde(rename = "URL")]
    url: Option<String>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct CslPerson {
    family: Option<String>,
    given: Option<String>,
    #[serde(rename = "non-dropping-particle")]
    non_dropping_particle: Option<String>,
    suffix: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CslDate {
    #[serde(rename = "date-parts")]
    date_parts: Option<Vec<Vec<u32>>>,
}

fn convert_csl_entry(csl: CslEntry) -> BibEntry {
    let entry_type = match csl.entry_type.as_str() {
        "article-journal" => BibEntryKind::Article,
        "book" => BibEntryKind::Book,
        "paper-conference" => BibEntryKind::InProceedings,
        "thesis" => BibEntryKind::PhdThesis,
        "report" => BibEntryKind::TechReport,
        "webpage" => BibEntryKind::Online,
        _ => BibEntryKind::Misc,
    };

    let authors = csl
        .author
        .into_iter()
        .map(|p| BibAuthor {
            given: p.given,
            family: p.family.unwrap_or_default(),
            prefix: p.non_dropping_particle,
            suffix: p.suffix,
        })
        .collect();

    let year = csl
        .issued
        .and_then(|d| d.date_parts)
        .and_then(|parts| parts.first().cloned())
        .and_then(|part| part.first().copied());

    let mut fields = HashMap::new();
    if let Some(container) = csl.container_title {
        fields.insert("journal".to_string(), container);
    }
    if let Some(volume) = csl.volume {
        fields.insert("volume".to_string(), volume);
    }
    if let Some(issue) = csl.issue {
        fields.insert("issue".to_string(), issue);
    }
    if let Some(page) = csl.page {
        fields.insert("pages".to_string(), page);
    }
    if let Some(doi) = csl.doi {
        fields.insert("doi".to_string(), doi);
    }
    if let Some(url) = csl.url {
        fields.insert("url".to_string(), url);
    }

    BibEntry {
        key: csl.id,
        entry_type,
        authors,
        title: csl.title.unwrap_or_default(),
        year,
        fields,
    }
}
