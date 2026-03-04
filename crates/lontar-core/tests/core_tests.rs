use lontar_core::prelude::*;

#[test]
fn ast_construction_basic() {
    let doc = DocumentBuilder::new("Title")
        .heading(1, "Intro")
        .paragraph("Hello")
        .equation("x^2", Some("eq1".into()), true)
        .build();

    assert_eq!(doc.metadata.title, "Title");
    assert_eq!(doc.content.len(), 3);
    match &doc.content[0] {
        Block::Heading { level, .. } => assert_eq!(*level, 1),
        _ => panic!("expected heading"),
    }
    match &doc.content[2] {
        Block::Equation {
            label, numbered, ..
        } => {
            assert_eq!(label.as_deref(), Some("eq1"));
            assert!(numbered);
        }
        _ => panic!("expected equation"),
    }
}

#[test]
fn style_resolution_cascade() {
    let mut sheet = StyleSheet::new();
    sheet.default_style = Some("base".to_string());
    sheet.add_style(
        "base".to_string(),
        NamedStyle {
            name: "base".to_string(),
            based_on: None,
            text_style: Some(TextStyle {
                bold: Some(true),
                ..Default::default()
            }),
            paragraph_style: None,
        },
    );
    sheet.add_style(
        "em".to_string(),
        NamedStyle {
            name: "em".to_string(),
            based_on: Some("base".to_string()),
            text_style: Some(TextStyle {
                italic: Some(true),
                ..Default::default()
            }),
            paragraph_style: None,
        },
    );

    let para_style = ParagraphStyle {
        alignment: None,
        spacing_before: None,
        spacing_after: None,
        line_spacing: None,
        indent_left: None,
        indent_right: None,
        indent_first: None,
        text_style: Some(TextStyle {
            underline: Some(true),
            ..Default::default()
        }),
    };

    let run_style = TextStyle {
        color: Some("FF0000".to_string()),
        ..Default::default()
    };

    let resolved = sheet.resolve_text_style(Some("em"), Some(&para_style), Some(&run_style));
    assert_eq!(resolved.bold, Some(true));
    assert_eq!(resolved.italic, Some(true));
    assert_eq!(resolved.underline, Some(true));
    assert_eq!(resolved.color.as_deref(), Some("FF0000"));
}

#[test]
fn citation_numeric() {
    let mut store = BibliographyStore::new();
    store.add_entry(BibEntry {
        key: "knuth1984".to_string(),
        entry_type: BibEntryKind::Article,
        authors: vec![BibAuthor {
            given: Some("Donald E.".to_string()),
            family: "Knuth".to_string(),
            prefix: None,
            suffix: None,
        }],
        title: "The TeXbook".to_string(),
        year: Some(1984),
        fields: Default::default(),
    });

    store.cite("knuth1984").unwrap();
    let rendered = store
        .render_citation(
            &["knuth1984".to_string()],
            BibliographyStyle::Numeric,
            CitationMode::Parenthetical,
        )
        .unwrap();
    assert_eq!(rendered, "[1]");
}

#[test]
fn crossref_resolution() {
    let doc = DocumentBuilder::new("Refs")
        .register_label("fig:1", Some("Figure One".to_string()))
        .build();
    let resolved_num = doc.resolve_crossref("fig:1", CrossRefKind::Number).unwrap();
    assert_eq!(resolved_num, "1");
    let resolved_title = doc.resolve_crossref("fig:1", CrossRefKind::Title).unwrap();
    assert_eq!(resolved_title, "Figure One");
}

#[test]
fn bibtex_parsing_minimal() {
    let bib = r#"
@article{knuth1984,
  author = {Donald E. Knuth},
  title = {The TeXbook},
  year = {1984}
}
"#;
    let store = BibliographyStore::from_bibtex_str(bib).unwrap();
    let entry = store.get_entry("knuth1984").unwrap();
    assert_eq!(entry.title, "The TeXbook");
    assert_eq!(entry.year, Some(1984));
    assert_eq!(entry.entry_type, BibEntryKind::Article);
    assert_eq!(entry.authors[0].family, "Knuth");
}

#[test]
fn citation_missing_key() {
    let store = BibliographyStore::new();
    let result = store.render_citation(
        &["nonexistent".to_string()],
        BibliographyStyle::Numeric,
        CitationMode::Parenthetical,
    );
    assert!(result.is_err());
}

#[test]
fn duplicate_label_skipped() {
    let doc = DocumentBuilder::new("Doc")
        .register_label("fig:1", Some("Figure One".to_string()))
        .register_label("fig:1", Some("Figure Two".to_string()))
        .build();

    // Second registration should be skipped; first title should remain
    let title = doc.resolve_crossref("fig:1", CrossRefKind::Title).unwrap();
    assert_eq!(title, "Figure One");
}

#[test]
fn bibtex_parsing_with_braces() {
    let bib = r#"
@article{test2024,
  author = {John Doe},
  title = {The {Importance} of {Braces}},
  year = {2024}
}
"#;
    let store = BibliographyStore::from_bibtex_str(bib).unwrap();
    let entry = store.get_entry("test2024").unwrap();
    // Title should preserve content (outer braces removed)
    assert!(entry.title.contains("Importance"));
}
