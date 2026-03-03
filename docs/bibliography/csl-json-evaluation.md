# CSL-JSON Parsing Evaluation

## Overview

CSL-JSON (Citation Style Language JSON) is a JSON-based format for bibliographic data. It's used by Zotero, Mendeley, and other reference managers as an interchange format.

**Advantages over BibTeX:**
- Structured JSON (easy to parse with serde)
- No ambiguity in field names
- Better support for modern entry types
- Native support for URLs, DOIs, ISBNs

## CSL-JSON Format

### Example Entry

```json
{
  "id": "knuth1984",
  "type": "article-journal",
  "author": [
    {
      "family": "Knuth",
      "given": "Donald E."
    }
  ],
  "title": "The TeXbook",
  "container-title": "Computers & Typesetting",
  "issued": {
    "date-parts": [[1984]]
  },
  "volume": "A",
  "publisher": "Addison-Wesley"
}
```

### Entry Types

CSL-JSON uses different type names than BibTeX:

| BibTeX | CSL-JSON | Description |
|---|---|---|
| `@article` | `article-journal` | Journal article |
| `@book` | `book` | Book |
| `@inproceedings` | `paper-conference` | Conference paper |
| `@phdthesis` | `thesis` | PhD thesis |
| `@techreport` | `report` | Technical report |
| `@misc` | `article` | Generic |
| `@online` | `webpage` | Web page |

### Common Fields

| Field | Type | Description |
|---|---|---|
| `id` | String | Citation key |
| `type` | String | Entry type |
| `author` | Array | List of authors |
| `title` | String | Title |
| `container-title` | String | Journal/book title |
| `issued` | Object | Publication date |
| `volume` | String/Number | Volume |
| `issue` | String/Number | Issue number |
| `page` | String | Page range |
| `DOI` | String | Digital Object Identifier |
| `URL` | String | Web address |
| `ISBN` | String | Book identifier |
| `ISSN` | String | Journal identifier |

## Parsing with Serde

### Data Structures

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CslEntry {
    pub id: String,
    #[serde(rename = "type")]
    pub entry_type: String,
    #[serde(default)]
    pub author: Vec<CslPerson>,
    #[serde(default)]
    pub editor: Vec<CslPerson>,
    pub title: Option<String>,
    #[serde(rename = "container-title")]
    pub container_title: Option<String>,
    pub issued: Option<CslDate>,
    pub volume: Option<String>,
    pub issue: Option<String>,
    pub page: Option<String>,
    #[serde(rename = "DOI")]
    pub doi: Option<String>,
    #[serde(rename = "URL")]
    pub url: Option<String>,
    #[serde(rename = "ISBN")]
    pub isbn: Option<String>,
    #[serde(rename = "ISSN")]
    pub issn: Option<String>,
    pub publisher: Option<String>,
    #[serde(rename = "publisher-place")]
    pub publisher_place: Option<String>,
    pub edition: Option<String>,
    
    // Catch-all for other fields
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CslPerson {
    pub family: Option<String>,
    pub given: Option<String>,
    pub suffix: Option<String>,
    #[serde(rename = "dropping-particle")]
    pub dropping_particle: Option<String>,
    #[serde(rename = "non-dropping-particle")]
    pub non_dropping_particle: Option<String>,
    pub literal: Option<String>,  // For organizations
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CslDate {
    #[serde(rename = "date-parts")]
    pub date_parts: Option<Vec<Vec<u32>>>,
    pub raw: Option<String>,
}
```

### Parsing Implementation

```rust
use serde_json;
use std::path::Path;

impl BibliographyStore {
    pub fn from_csl_json_file(path: &Path) -> Result<Self, BibError> {
        let content = std::fs::read_to_string(path)?;
        Self::from_csl_json_str(&content)
    }
    
    pub fn from_csl_json_str(content: &str) -> Result<Self, BibError> {
        let csl_entries: Vec<CslEntry> = serde_json::from_str(content)?;
        let mut entries = HashMap::new();
        
        for csl_entry in csl_entries {
            let bib_entry = Self::convert_csl_entry(csl_entry)?;
            entries.insert(bib_entry.key.clone(), bib_entry);
        }
        
        Ok(Self { entries })
    }
    
    fn convert_csl_entry(csl: CslEntry) -> Result<BibEntry, BibError> {
        let entry_type = match csl.entry_type.as_str() {
            "article-journal" => BibEntryKind::Article,
            "book" => BibEntryKind::Book,
            "paper-conference" => BibEntryKind::InProceedings,
            "thesis" => BibEntryKind::PhdThesis,
            "report" => BibEntryKind::TechReport,
            "webpage" => BibEntryKind::Online,
            _ => BibEntryKind::Misc,
        };
        
        let authors = csl.author.iter().map(|person| BibAuthor {
            given: person.given.clone(),
            family: person.family.clone().unwrap_or_default(),
            prefix: person.non_dropping_particle.clone(),
            suffix: person.suffix.clone(),
        }).collect();
        
        let year = csl.issued
            .and_then(|date| date.date_parts)
            .and_then(|parts| parts.first())
            .and_then(|year_part| year_part.first())
            .copied();
        
        let mut fields = HashMap::new();
        if let Some(journal) = csl.container_title {
            fields.insert("journal".to_string(), journal);
        }
        if let Some(volume) = csl.volume {
            fields.insert("volume".to_string(), volume);
        }
        if let Some(doi) = csl.doi {
            fields.insert("doi".to_string(), doi);
        }
        if let Some(url) = csl.url {
            fields.insert("url".to_string(), url);
        }
        
        Ok(BibEntry {
            key: csl.id,
            entry_type,
            authors,
            title: csl.title.unwrap_or_default(),
            year,
            fields,
        })
    }
}
```

## Example CSL-JSON File

```json
[
  {
    "id": "knuth1984",
    "type": "article-journal",
    "author": [
      {
        "family": "Knuth",
        "given": "Donald E."
      }
    ],
    "title": "The TeXbook",
    "container-title": "Computers & Typesetting",
    "issued": {
      "date-parts": [[1984]]
    },
    "volume": "A",
    "publisher": "Addison-Wesley"
  },
  {
    "id": "lamport1994",
    "type": "book",
    "author": [
      {
        "family": "Lamport",
        "given": "Leslie"
      }
    ],
    "title": "LaTeX: A Document Preparation System",
    "issued": {
      "date-parts": [[1994]]
    },
    "edition": "2nd",
    "publisher": "Addison-Wesley",
    "publisher-place": "Reading, MA"
  }
]
```

## Testing

```rust
#[test]
fn test_csl_json_parsing() {
    let csl_str = r#"
[
  {
    "id": "knuth1984",
    "type": "article-journal",
    "author": [{"family": "Knuth", "given": "Donald E."}],
    "title": "The TeXbook",
    "issued": {"date-parts": [[1984]]}
  }
]
"#;

    let store = BibliographyStore::from_csl_json_str(csl_str).unwrap();
    let entry = store.get_entry("knuth1984").unwrap();
    
    assert_eq!(entry.entry_type, BibEntryKind::Article);
    assert_eq!(entry.authors[0].family, "Knuth");
    assert_eq!(entry.year, Some(1984));
}
```

## Comparison: BibTeX vs CSL-JSON

| Aspect | BibTeX | CSL-JSON |
|---|---|---|
| **Format** | Custom DSL | JSON |
| **Parsing** | Complex (needs parser) | Simple (serde) |
| **Field names** | Inconsistent | Standardized |
| **Modern fields** | Limited | Comprehensive |
| **Tool support** | LaTeX ecosystem | Zotero, Mendeley |
| **File size** | Smaller | Larger |

## Recommendation

**Support both formats:**
- BibTeX for LaTeX users
- CSL-JSON for Zotero/Mendeley users

```rust
impl BibliographyStore {
    pub fn from_file(path: &Path) -> Result<Self, BibError> {
        let ext = path.extension()
            .and_then(|s| s.to_str())
            .ok_or(BibError::InvalidFormat)?;
        
        match ext {
            "bib" => Self::from_bibtex_file(path),
            "json" => Self::from_csl_json_file(path),
            _ => Err(BibError::UnsupportedFormat(ext.to_string())),
        }
    }
}
```

## Next Steps

1. Add `serde_json` to dependencies
2. Implement CSL-JSON data structures
3. Implement `from_csl_json_file()` and `from_csl_json_str()`
4. Create test CSL-JSON file in `tests/fixtures/bib/`
5. Add format auto-detection

## References

- [CSL-JSON Schema](https://citeproc-js.readthedocs.io/en/latest/csl-json/markup.html)
- [CSL Specification](https://docs.citationstyles.org/en/stable/specification.html)
- [Zotero CSL-JSON Export](https://www.zotero.org/support/dev/web_api/v3/basics)
