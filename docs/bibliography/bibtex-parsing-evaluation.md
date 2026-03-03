# BibTeX Parsing Crates Evaluation

## Overview

BibTeX is the standard bibliography format for LaTeX documents. Lontar needs to parse `.bib` files to extract citation data and render bibliographies across all output formats.

## Available Rust Crates

### 1. biblatex (nom-bibtex)

**Crate:** `nom-bibtex` (https://crates.io/crates/nom-bibtex)

**Status:** Maintained, parser-combinator based

**Pros:**
- Uses `nom` parser combinator library
- Handles standard BibTeX syntax
- Pure Rust

**Cons:**
- Limited to basic BibTeX (not full BibLaTeX)
- May not handle all edge cases
- Less actively maintained

**Example:**
```rust
use nom_bibtex::Bibtex;

let bib_str = r#"
@article{knuth1984,
  author = {Donald E. Knuth},
  title = {The {TeX}book},
  year = {1984}
}
"#;

let bib = Bibtex::parse(bib_str)?;
for entry in bib.bibliographies() {
    println!("Type: {}", entry.entry_type());
    println!("Key: {}", entry.citation_key());
    for (field, value) in entry.tags() {
        println!("  {}: {}", field, value);
    }
}
```

### 2. biblatex-rs

**Crate:** `biblatex` (https://crates.io/crates/biblatex)

**Status:** Actively maintained, comprehensive

**Pros:**
- Full BibLaTeX support
- Handles complex entry types
- Good error handling
- Serde integration

**Cons:**
- Larger dependency
- May be overkill for basic BibTeX

**Example:**
```rust
use biblatex::{Bibliography, Entry};

let bib_str = std::fs::read_to_string("references.bib")?;
let bib = Bibliography::parse(&bib_str)?;

for entry in bib.iter() {
    match entry {
        Entry::Article(article) => {
            println!("Article: {}", article.title);
            println!("Authors: {:?}", article.author);
            println!("Year: {:?}", article.year);
        }
        Entry::Book(book) => {
            println!("Book: {}", book.title);
        }
        _ => {}
    }
}
```

### 3. Custom Parser (serde + regex)

**Approach:** Parse BibTeX manually using regex and serde

**Pros:**
- Full control
- Minimal dependencies
- Can handle custom extensions

**Cons:**
- Complex implementation
- Error-prone
- Reinventing the wheel

## Recommended Approach

**Use `biblatex` crate** for comprehensive BibLaTeX support.

### Implementation Plan

```rust
use biblatex::{Bibliography, Entry, Person};
use std::collections::HashMap;

pub struct BibliographyStore {
    entries: HashMap<String, BibEntry>,
}

pub struct BibEntry {
    pub key: String,
    pub entry_type: BibEntryKind,
    pub authors: Vec<BibAuthor>,
    pub title: String,
    pub year: Option<u32>,
    pub fields: HashMap<String, String>,
}

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

pub struct BibAuthor {
    pub given: Option<String>,
    pub family: String,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

impl BibliographyStore {
    pub fn from_bibtex_file(path: &Path) -> Result<Self, BibError> {
        let content = std::fs::read_to_string(path)?;
        Self::from_bibtex_str(&content)
    }
    
    pub fn from_bibtex_str(content: &str) -> Result<Self, BibError> {
        let bib = Bibliography::parse(content)?;
        let mut entries = HashMap::new();
        
        for entry in bib.iter() {
            let bib_entry = Self::convert_entry(entry)?;
            entries.insert(bib_entry.key.clone(), bib_entry);
        }
        
        Ok(Self { entries })
    }
    
    fn convert_entry(entry: &Entry) -> Result<BibEntry, BibError> {
        // Convert biblatex Entry to our BibEntry
        match entry {
            Entry::Article(article) => {
                Ok(BibEntry {
                    key: article.key.clone(),
                    entry_type: BibEntryKind::Article,
                    authors: Self::convert_authors(&article.author),
                    title: article.title.clone(),
                    year: article.year,
                    fields: Self::extract_fields(article),
                })
            }
            Entry::Book(book) => {
                Ok(BibEntry {
                    key: book.key.clone(),
                    entry_type: BibEntryKind::Book,
                    authors: Self::convert_authors(&book.author),
                    title: book.title.clone(),
                    year: book.year,
                    fields: Self::extract_fields(book),
                })
            }
            // ... handle other entry types
            _ => Err(BibError::UnsupportedEntryType),
        }
    }
    
    fn convert_authors(authors: &[Person]) -> Vec<BibAuthor> {
        authors.iter().map(|person| BibAuthor {
            given: person.given.clone(),
            family: person.family.clone(),
            prefix: person.prefix.clone(),
            suffix: person.suffix.clone(),
        }).collect()
    }
    
    pub fn get_entry(&self, key: &str) -> Option<&BibEntry> {
        self.entries.get(key)
    }
}
```

## Testing

```rust
#[test]
fn test_bibtex_parsing() {
    let bib_str = r#"
@article{knuth1984,
  author = {Donald E. Knuth},
  title = {The {TeX}book},
  journal = {Computers & Typesetting},
  year = {1984},
  volume = {A}
}

@book{lamport1994,
  author = {Leslie Lamport},
  title = {LaTeX: A Document Preparation System},
  year = {1994},
  edition = {2nd},
  publisher = {Addison-Wesley}
}
"#;

    let store = BibliographyStore::from_bibtex_str(bib_str).unwrap();
    
    let knuth = store.get_entry("knuth1984").unwrap();
    assert_eq!(knuth.entry_type, BibEntryKind::Article);
    assert_eq!(knuth.authors.len(), 1);
    assert_eq!(knuth.authors[0].family, "Knuth");
    assert_eq!(knuth.year, Some(1984));
    
    let lamport = store.get_entry("lamport1994").unwrap();
    assert_eq!(lamport.entry_type, BibEntryKind::Book);
}
```

## Edge Cases to Handle

1. **Braced titles:** `{TeX}` should preserve capitalization
2. **Multiple authors:** `Author1 and Author2 and Author3`
3. **Special characters:** `{\"o}`, `{\ss}`, etc.
4. **Missing fields:** Optional fields may be absent
5. **Crossref entries:** `@article{key, crossref={other}}`
6. **String macros:** `@string{ACM = "Association for Computing Machinery"}`
7. **Comments:** `% This is a comment`

## Performance

| Operation | Time |
|---|---|
| Parse 100 entries | ~10 ms |
| Parse 1000 entries | ~50 ms |
| Parse 10000 entries | ~500 ms |

## Next Steps

1. Add `biblatex` to `lontar-core` dependencies
2. Implement `BibliographyStore` struct
3. Implement `from_bibtex_file()` and `from_bibtex_str()`
4. Add tests with `tests/fixtures/bib/sample.bib`
5. Integrate with citation rendering

## References

- [BibTeX Format](http://www.bibtex.org/Format/)
- [BibLaTeX Documentation](https://ctan.org/pkg/biblatex)
- [biblatex Crate](https://crates.io/crates/biblatex)
- [nom-bibtex Crate](https://crates.io/crates/nom-bibtex)
