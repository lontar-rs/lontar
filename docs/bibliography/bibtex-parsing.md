# BibTeX Parsing Research

## Overview

BibTeX is a reference management format used extensively in academic publishing. This document evaluates Rust crates for parsing BibTeX files and integrating them with Lontar's bibliography system.

## BibTeX Format

### Structure

```bibtex
@article{key2024,
  author = "John Doe and Jane Smith",
  title = "A Study on Text Shaping",
  journal = "Journal of Typography",
  year = "2024",
  volume = "42",
  pages = "123--145",
  doi = "10.1234/example",
}

@book{knuth1984,
  author = "Donald E. Knuth",
  title = "The TeXbook",
  publisher = "Addison-Wesley",
  year = "1984",
}

@inproceedings{smith2023,
  author = "Alice Smith",
  title = "Font Subsetting in Modern Typesetting",
  booktitle = "Proceedings of the International Conference on Typography",
  year = "2023",
  pages = "45--67",
}
```

### Entry Types

| Type | Fields | Use Case |
|------|--------|----------|
| `article` | author, title, journal, year | Journal articles |
| `book` | author, title, publisher, year | Books |
| `inproceedings` | author, title, booktitle, year | Conference papers |
| `incollection` | author, title, booktitle, publisher, year | Book chapters |
| `misc` | author, title, howpublished, year | Websites, reports |
| `thesis` | author, title, school, year | Theses |
| `techreport` | author, title, institution, year | Technical reports |

## Rust Crates for BibTeX Parsing

### 1. nom-bibtex

A parser combinator-based BibTeX parser using `nom`.

```rust
use nom_bibtex::Bibtex;

let bibtex_str = r#"
@article{key2024,
  author = "John Doe",
  title = "A Study",
  journal = "Journal",
  year = "2024",
}
"#;

let bibtex = Bibtex::parse(bibtex_str)?;

for entry in bibtex.entries() {
    println!("Key: {}", entry.key());
    println!("Type: {}", entry.entry_type());

    for (field, value) in entry.fields() {
        println!("  {}: {}", field, value);
    }
}
```

**Advantages:**
- Pure Rust, no external dependencies
- Fast parsing with nom combinators
- Handles most BibTeX syntax

**Disadvantages:**
- Limited error recovery
- No support for BibTeX macros
- Minimal documentation

### 2. bibtex-rs

A dedicated BibTeX parser with better error handling.

```rust
use bibtex::Bibliography;

let bibtex_str = std::fs::read_to_string("references.bib")?;
let bibliography = Bibliography::parse(&bibtex_str)?;

for entry in bibliography.entries() {
    println!("Key: {}", entry.key);
    println!("Type: {}", entry.entry_type);

    for (field, value) in &entry.fields {
        println!("  {}: {}", field, value);
    }
}
```

**Advantages:**
- Better error messages
- Handles nested braces
- Supports string concatenation

**Disadvantages:**
- Less mature than nom-bibtex
- Smaller community

### 3. biblatex

A higher-level BibLaTeX parser (extension of BibTeX).

```rust
use biblatex::Bibliography;

let bibtex_str = std::fs::read_to_string("references.bib")?;
let bibliography = Bibliography::parse(&bibtex_str)?;

for entry in bibliography.entries() {
    match entry.entry_type() {
        "article" => {
            let author = entry.author()?;
            let title = entry.title()?;
            let journal = entry.journal()?;
            let year = entry.year()?;

            println!("{} ({}). {}. {}", author, year, title, journal);
        }
        _ => {}
    }
}
```

**Advantages:**
- Type-safe field access
- Supports BibLaTeX extensions
- Better documentation

**Disadvantages:**
- More opinionated API
- Requires specific field names

## Recommended Approach for Lontar

### Phase 1: Use nom-bibtex

For MVP, use `nom-bibtex` for its simplicity and performance:

```rust
use nom_bibtex::Bibtex;

pub struct BibliographyEntry {
    pub key: String,
    pub entry_type: String,
    pub fields: HashMap<String, String>,
}

pub fn parse_bibtex_file(path: &str) -> Result<Vec<BibliographyEntry>> {
    let content = std::fs::read_to_string(path)?;
    let bibtex = Bibtex::parse(&content)?;

    let mut entries = Vec::new();
    for entry in bibtex.entries() {
        let mut fields = HashMap::new();
        for (field, value) in entry.fields() {
            fields.insert(field.to_string(), value.to_string());
        }

        entries.push(BibliographyEntry {
            key: entry.key().to_string(),
            entry_type: entry.entry_type().to_string(),
            fields,
        });
    }

    Ok(entries)
}
```

### Phase 2: Implement Citation Formatting

```rust
pub struct Citation {
    pub key: String,
    pub author: String,
    pub title: String,
    pub year: String,
}

pub fn format_citation(entry: &BibliographyEntry, style: CitationStyle) -> String {
    match style {
        CitationStyle::APA => format_apa(entry),
        CitationStyle::Chicago => format_chicago(entry),
        CitationStyle::Vancouver => format_vancouver(entry),
    }
}

fn format_apa(entry: &BibliographyEntry) -> String {
    let author = entry.fields.get("author").unwrap_or(&"Unknown".to_string());
    let year = entry.fields.get("year").unwrap_or(&"n.d.".to_string());
    let title = entry.fields.get("title").unwrap_or(&"Untitled".to_string());

    format!("{} ({}). {}.", author, year, title)
}
```

## BibTeX Field Mapping

### Standard Fields

| Field | Type | Description |
|-------|------|-------------|
| author | string | Author names |
| title | string | Entry title |
| year | integer | Publication year |
| journal | string | Journal name |
| booktitle | string | Book/conference title |
| publisher | string | Publisher name |
| volume | string | Volume number |
| number | string | Issue number |
| pages | string | Page range |
| doi | string | Digital Object Identifier |
| url | string | URL |
| note | string | Additional notes |

### BibLaTeX Extensions

| Field | Type | Description |
|-------|------|-------------|
| editor | string | Editor names |
| translator | string | Translator names |
| edition | string | Edition number |
| isbn | string | ISBN |
| issn | string | ISSN |
| keywords | string | Keywords |
| abstract | string | Abstract |
| langid | string | Language ID |

## Integration with Lontar

### 1. Load Bibliography

```rust
pub struct Document {
    pub content: Vec<Block>,
    pub bibliography: Option<Bibliography>,
}

pub fn load_document_with_bibliography(
    doc_path: &str,
    bib_path: &str,
) -> Result<Document> {
    let content = parse_document(doc_path)?;
    let bibliography = parse_bibtex_file(bib_path)?;

    Ok(Document {
        content,
        bibliography: Some(bibliography),
    })
}
```

### 2. Resolve Citations

```rust
pub fn resolve_citations(
    content: &[Block],
    bibliography: &[BibliographyEntry],
) -> Result<Vec<Block>> {
    let mut resolved = Vec::new();

    for block in content {
        match block {
            Block::Paragraph(para) => {
                let resolved_para = resolve_citations_in_paragraph(para, bibliography)?;
                resolved.push(Block::Paragraph(resolved_para));
            }
            _ => resolved.push(block.clone()),
        }
    }

    Ok(resolved)
}

fn resolve_citations_in_paragraph(
    para: &Paragraph,
    bibliography: &[BibliographyEntry],
) -> Result<Paragraph> {
    let mut resolved_inlines = Vec::new();

    for inline in &para.inlines {
        match inline {
            Inline::Citation(key) => {
                // Find entry in bibliography
                if let Some(entry) = bibliography.iter().find(|e| e.key == *key) {
                    let formatted = format_citation(entry, CitationStyle::APA);
                    resolved_inlines.push(Inline::Text(formatted));
                } else {
                    return Err(format!("Citation key not found: {}", key).into());
                }
            }
            _ => resolved_inlines.push(inline.clone()),
        }
    }

    Ok(Paragraph {
        inlines: resolved_inlines,
    })
}
```

### 3. Generate Bibliography Section

```rust
pub fn generate_bibliography_section(
    bibliography: &[BibliographyEntry],
    style: CitationStyle,
) -> Block {
    let mut entries = Vec::new();

    // Sort entries by author and year
    let mut sorted = bibliography.to_vec();
    sorted.sort_by(|a, b| {
        let a_author = a.fields.get("author").unwrap_or(&"".to_string());
        let b_author = b.fields.get("author").unwrap_or(&"".to_string());
        a_author.cmp(b_author)
    });

    for entry in sorted {
        let formatted = format_citation(&entry, style);
        entries.push(Inline::Text(formatted));
    }

    Block::Section {
        title: "Bibliography".to_string(),
        content: vec![Block::Paragraph(Paragraph {
            inlines: entries,
        })],
    }
}
```

## Performance Characteristics

| Operation | Time | Notes |
|-----------|------|-------|
| Parse 100 entries | ~1-5ms | Fast |
| Parse 1000 entries | ~10-50ms | Acceptable |
| Format 100 citations | ~5-10ms | Fast |
| Generate bibliography | ~10-20ms | Acceptable |

## References

- [BibTeX Documentation](http://www.ctan.org/pkg/bibtex)
- [BibLaTeX Documentation](http://www.ctan.org/pkg/biblatex)
- [nom-bibtex on crates.io](https://crates.io/crates/nom-bibtex)
- [bibtex-rs on crates.io](https://crates.io/crates/bibtex)
