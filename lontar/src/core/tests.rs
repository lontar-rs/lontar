//! Comprehensive unit tests for citation resolution.
//!
//! This module contains tests for all citation modes and bibliography styles
//! to ensure correct rendering behavior across different academic formats.

#![allow(unused_imports)]

use super::types::{
    BibAuthor, BibEntry, BibEntryKind, BibliographyError, BibliographyStore, BibliographyStyle,
    CitationMode,
};
use std::collections::HashMap;

#[test]
fn test_citation_resolution_numeric_style() {
    let mut bib = BibliographyStore::new(BibliographyStyle::Numeric);

    // Add test entries
    bib.add_entry(BibEntry {
        key: "smith2024".to_string(),
        kind: BibEntryKind::Article,
        fields: {
            let mut m = HashMap::new();
            m.insert("title".to_string(), "Test Article".to_string());
            m.insert("year".to_string(), "2024".to_string());
            m
        },
        authors: vec![BibAuthor {
            given: Some("John".to_string()),
            family: "Smith".to_string(),
            literal: None,
        }],
    });

    bib.add_entry(BibEntry {
        key: "doe2023".to_string(),
        kind: BibEntryKind::Book,
        fields: {
            let mut m = HashMap::new();
            m.insert("title".to_string(), "Test Book".to_string());
            m.insert("year".to_string(), "2023".to_string());
            m
        },
        authors: vec![BibAuthor {
            given: Some("Jane".to_string()),
            family: "Doe".to_string(),
            literal: None,
        }],
    });

    // Test parenthetical mode
    let result = bib.render_citation(&["smith2024"], CitationMode::Parenthetical).unwrap();
    assert_eq!(result, "[1]");

    // Test narrative mode
    let result = bib.render_citation(&["smith2024"], CitationMode::Narrative).unwrap();
    assert_eq!(result, "1");

    // Test multiple citations
    let result = bib
        .render_citation(&["smith2024", "doe2023"], CitationMode::Parenthetical)
        .unwrap();
    assert_eq!(result, "[1, 2]");

    // Test year-only mode (should behave like parenthetical for numeric styles)
    let result = bib.render_citation(&["smith2024"], CitationMode::YearOnly).unwrap();
    assert_eq!(result, "[1]");
}

#[test]
fn test_citation_resolution_vancouver_style() {
    let mut bib = BibliographyStore::new(BibliographyStyle::Vancouver);

    bib.add_entry(BibEntry {
        key: "johnson2024".to_string(),
        kind: BibEntryKind::Article,
        fields: {
            let mut m = HashMap::new();
            m.insert("title".to_string(), "Medical Study".to_string());
            m.insert("year".to_string(), "2024".to_string());
            m
        },
        authors: vec![BibAuthor {
            given: Some("Robert".to_string()),
            family: "Johnson".to_string(),
            literal: None,
        }],
    });

    let result = bib.render_citation(&["johnson2024"], CitationMode::Parenthetical).unwrap();
    assert_eq!(result, "[1]");

    let result = bib.render_citation(&["johnson2024"], CitationMode::Narrative).unwrap();
    assert_eq!(result, "1");
}

#[test]
fn test_citation_resolution_superscript_style() {
    let mut bib = BibliographyStore::new(BibliographyStyle::Superscript);

    bib.add_entry(BibEntry {
        key: "wilson2024".to_string(),
        kind: BibEntryKind::Article,
        fields: {
            let mut m = HashMap::new();
            m.insert("title".to_string(), "Physics Paper".to_string());
            m.insert("year".to_string(), "2024".to_string());
            m
        },
        authors: vec![BibAuthor {
            given: Some("Alice".to_string()),
            family: "Wilson".to_string(),
            literal: None,
        }],
    });

    let result = bib.render_citation(&["wilson2024"], CitationMode::Parenthetical).unwrap();
    assert_eq!(result, "^1^");

    let result = bib.render_citation(&["wilson2024"], CitationMode::Narrative).unwrap();
    assert_eq!(result, "^1^"); // Narrative mode still uses superscript formatting

    let result = bib.render_citation(&["wilson2024"], CitationMode::YearOnly).unwrap();
    assert_eq!(result, "^1^");
}

#[test]
fn test_citation_resolution_author_year_style() {
    let mut bib = BibliographyStore::new(BibliographyStyle::AuthorYear);

    bib.add_entry(BibEntry {
        key: "brown2024".to_string(),
        kind: BibEntryKind::Article,
        fields: {
            let mut m = HashMap::new();
            m.insert("title".to_string(), "Social Science Study".to_string());
            m.insert("year".to_string(), "2024".to_string());
            m
        },
        authors: vec![BibAuthor {
            given: Some("Charlie".to_string()),
            family: "Brown".to_string(),
            literal: None,
        }],
    });

    // Test parenthetical mode
    let result = bib.render_citation(&["brown2024"], CitationMode::Parenthetical).unwrap();
    assert_eq!(result, "(Brown 2024)");

    // Test narrative mode
    let result = bib.render_citation(&["brown2024"], CitationMode::Narrative).unwrap();
    assert_eq!(result, "Brown 2024");

    // Test year-only mode
    let result = bib.render_citation(&["brown2024"], CitationMode::YearOnly).unwrap();
    assert_eq!(result, "(2024)");

    // Test suppress author mode
    let result = bib.render_citation(&["brown2024"], CitationMode::SuppressAuthor).unwrap();
    assert_eq!(result, "(2024)");

    // Test full mode (should behave like parenthetical)
    let result = bib.render_citation(&["brown2024"], CitationMode::Full).unwrap();
    assert_eq!(result, "(Brown 2024)");
}

#[test]
fn test_citation_resolution_apa7_style() {
    let mut bib = BibliographyStore::new(BibliographyStyle::APA7);

    bib.add_entry(BibEntry {
        key: "davis2024".to_string(),
        kind: BibEntryKind::Article,
        fields: {
            let mut m = HashMap::new();
            m.insert("title".to_string(), "Psychology Research".to_string());
            m.insert("year".to_string(), "2024".to_string());
            m
        },
        authors: vec![BibAuthor {
            given: Some("Emma".to_string()),
            family: "Davis".to_string(),
            literal: None,
        }],
    });

    let result = bib.render_citation(&["davis2024"], CitationMode::Parenthetical).unwrap();
    assert_eq!(result, "(Davis 2024)");

    let result = bib.render_citation(&["davis2024"], CitationMode::Narrative).unwrap();
    assert_eq!(result, "Davis 2024");

    let result = bib.render_citation(&["davis2024"], CitationMode::YearOnly).unwrap();
    assert_eq!(result, "(2024)");
}

#[test]
fn test_citation_resolution_named_style() {
    let mut bib = BibliographyStore::new(BibliographyStyle::Named);

    bib.add_entry(BibEntry {
        key: "miller2024".to_string(),
        kind: BibEntryKind::Article,
        fields: {
            let mut m = HashMap::new();
            m.insert("title".to_string(), "History Paper".to_string());
            m.insert("year".to_string(), "2024".to_string());
            m
        },
        authors: vec![BibAuthor {
            given: Some("Frank".to_string()),
            family: "Miller".to_string(),
            literal: None,
        }],
    });

    let result = bib.render_citation(&["miller2024"], CitationMode::Parenthetical).unwrap();
    assert_eq!(result, "(Miller 2024)");

    let result = bib.render_citation(&["miller2024"], CitationMode::Narrative).unwrap();
    assert_eq!(result, "Miller 2024");
}

#[test]
fn test_citation_resolution_multiple_keys_author_year() {
    let mut bib = BibliographyStore::new(BibliographyStyle::AuthorYear);

    bib.add_entry(BibEntry {
        key: "author1".to_string(),
        kind: BibEntryKind::Article,
        fields: {
            let mut m = HashMap::new();
            m.insert("year".to_string(), "2023".to_string());
            m
        },
        authors: vec![BibAuthor {
            given: Some("First".to_string()),
            family: "Author".to_string(),
            literal: None,
        }],
    });

    bib.add_entry(BibEntry {
        key: "author2".to_string(),
        kind: BibEntryKind::Book,
        fields: {
            let mut m = HashMap::new();
            m.insert("year".to_string(), "2024".to_string());
            m
        },
        authors: vec![BibAuthor {
            given: Some("Second".to_string()),
            family: "Author".to_string(),
            literal: None,
        }],
    });

    let result = bib
        .render_citation(&["author1", "author2"], CitationMode::Parenthetical)
        .unwrap();
    assert_eq!(result, "(Author 2023; Author 2024)");

    let result = bib.render_citation(&["author1", "author2"], CitationMode::Narrative).unwrap();
    assert_eq!(result, "Author 2023; Author 2024");
}

#[test]
fn test_citation_resolution_missing_entry() {
    let bib = BibliographyStore::new(BibliographyStyle::Numeric);

    let result = bib.render_citation(&["nonexistent"], CitationMode::Parenthetical);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), BibliographyError::MissingEntry("nonexistent".to_string()));
}

#[test]
fn test_citation_resolution_no_authors() {
    let mut bib = BibliographyStore::new(BibliographyStyle::AuthorYear);

    bib.add_entry(BibEntry {
        key: "anon2024".to_string(),
        kind: BibEntryKind::Misc,
        fields: {
            let mut m = HashMap::new();
            m.insert("title".to_string(), "Anonymous Work".to_string());
            m.insert("year".to_string(), "2024".to_string());
            m
        },
        authors: vec![], // No authors
    });

    let result = bib.render_citation(&["anon2024"], CitationMode::Parenthetical).unwrap();
    assert_eq!(result, "(Anon 2024)");
}

#[test]
fn test_citation_resolution_no_year() {
    let mut bib = BibliographyStore::new(BibliographyStyle::AuthorYear);

    bib.add_entry(BibEntry {
        key: "noyear".to_string(),
        kind: BibEntryKind::Misc,
        fields: {
            let mut m = HashMap::new();
            m.insert("title".to_string(), "Undated Work".to_string());
            m
        },
        authors: vec![BibAuthor {
            given: Some("Someone".to_string()),
            family: "Author".to_string(),
            literal: None,
        }],
    });

    let result = bib.render_citation(&["noyear"], CitationMode::Parenthetical).unwrap();
    assert_eq!(result, "(Author n.d.)");

    let result = bib.render_citation(&["noyear"], CitationMode::YearOnly).unwrap();
    assert_eq!(result, "(n.d.)");
}
