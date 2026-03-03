# APA 7th Edition Citation Style

## Overview

APA (American Psychological Association) 7th edition is the standard citation format for psychology, education, and social sciences. It uses **author-date citations** in the text and an **alphabetical reference list**.

**Used by:**
- Psychology journals
- Education journals
- Social science journals
- Business journals

## In-Text Citations

### Author-Date Format

Citations appear as **(Author, Year)** in the text.

**Single author:**
```
Recent research (Smith, 2024) shows...
Smith (2024) demonstrated that...
```

**Two authors:**
```
Multiple studies (Miller & Chen, 2023) have found...
Miller and Chen (2023) reported...
```

**Three or more authors:**
```
First citation: (Garcia, Lee, & Singh, 2022)
Subsequent: (Garcia et al., 2022)

Narrative: Garcia et al. (2022) found...
```

### Special Cases

**Multiple works by same author:**
```
(Smith, 2022, 2023, 2024)
```

**Multiple works in same year:**
```
(Smith, 2024a, 2024b)
```

**Multiple citations:**
```
Several studies (Chen, 2023; Lee, 2024; Smith, 2022) support...
```

**Direct quote:**
```
"Typography matters" (Knuth, 1984, p. 42).
```

**No author:**
```
("Title of Work," 2024)
```

**No date:**
```
(Smith, n.d.)
```

## Reference List Format

### General Rules

1. **Alphabetical order** — By first author's last name
2. **Hanging indent** — First line flush left, subsequent lines indented
3. **Author names** — Last name, Initials (up to 20 authors)
4. **Title case** — Only first word and proper nouns capitalized
5. **DOI preferred** — Include DOI when available

### Journal Article

**Format:**
```
Author, A. A., Author, B. B., & Author, C. C. (Year). Title of article. 
    Title of Periodical, Volume(Issue), Pages. https://doi.org/xx.xxxx
```

**Example:**
```
Knuth, D. E. (1984). The TeXbook. Computers & Typesetting, A, 1-483. 
    https://doi.org/10.1234/example
```

**With 3+ authors:**
```
Miller, A., Chen, B., & Davis, C. (2023). Efficient graph layouts for 
    large diagrams. Proceedings of the Diagram Systems Conference, 15, 
    101-115. https://doi.org/10.1234/diag.2023.15
```

**With 21+ authors:**
```
Smith, J., Jones, A., Brown, C., Davis, E., Wilson, F., Taylor, G., 
    Anderson, H., Thomas, I., Jackson, J., White, K., Harris, L., 
    Martin, M., Thompson, N., Garcia, O., Martinez, P., Robinson, Q., 
    Clark, R., Rodriguez, S., Lewis, T., ... Young, Z. (2024). 
    Multi-author collaboration study. Journal of Research, 42(3), 234-245.
```

### Book

**Format:**
```
Author, A. A. (Year). Title of work: Subtitle. Publisher. 
    https://doi.org/xx.xxxx
```

**Example:**
```
Lamport, L. (1994). LaTeX: A document preparation system (2nd ed.). 
    Addison-Wesley.
```

**Edited book:**
```
Brown, R. (Ed.). (2020). Handbook of modern publishing. Academic Press.
```

### Book Chapter

**Format:**
```
Author, A. A. (Year). Title of chapter. In E. E. Editor (Ed.), 
    Title of book (pp. Pages). Publisher.
```

**Example:**
```
Jones, S. (2020). Typography in the digital age. In R. Brown (Ed.), 
    Handbook of modern publishing (pp. 45-78). Academic Press.
```

### Conference Paper

**Format:**
```
Author, A. A. (Year, Month Days). Title of paper [Conference presentation]. 
    Conference Name, Location. URL
```

**Example:**
```
Miller, A., & Chen, B. (2023, June 15-17). Efficient graph layouts 
    [Conference presentation]. Diagram Systems Conference, Boston, MA, 
    United States. https://example.com/paper
```

### Thesis/Dissertation

**Format:**
```
Author, A. A. (Year). Title of dissertation [Doctoral dissertation, 
    University Name]. Database Name. URL
```

**Example:**
```
Garcia, M. (2022). Multi-script typesetting for scientific publishing 
    [Doctoral dissertation, University of Example]. ProQuest Dissertations 
    and Theses. https://example.edu/theses/garcia2022
```

### Technical Report

**Format:**
```
Author, A. A. (Year). Title of report (Report No. XXX). Publisher. URL
```

**Example:**
```
Lee, D. (2021). Unicode fonts for complex scripts (Report No. ERL-2021-05). 
    Example Research Lab. https://example.org/reports/erl-2021-05
```

### Website

**Format:**
```
Author, A. A. (Year, Month Day). Title of page. Site Name. URL
```

**Example:**
```
Singh, P. (2024, February 1). Accessible math on the web: A practical guide. 
    Example Publishing. https://example.com/accessible-math
```

**No date:**
```
Author, A. A. (n.d.). Title of page. Site Name. Retrieved Month Day, Year, 
    from URL
```

## Special Formatting

### Titles

**Article/chapter titles:**
- Sentence case (only first word and proper nouns capitalized)
- No italics, no quotes

**Journal/book titles:**
- Title case (major words capitalized)
- Italicized

**Examples:**
```
Article: The effects of typography on readability
Journal: Journal of Typographic Research
Book: Handbook of Modern Publishing
```

### Author Names

**Format:** Last name, First initial. Middle initial.

**Examples:**
```
Smith, J. A.
García, M. L.
van der Berg, P.
```

**Organizations as authors:**
```
American Psychological Association. (2020). Publication manual...
```

### Dates

**Full date:**
```
(2024, March 15)
```

**Year and month:**
```
(2024, March)
```

**Year only:**
```
(2024)
```

**No date:**
```
(n.d.)
```

## Implementation in Lontar

### Citation Rendering

```rust
pub fn render_apa_citation(
    cite_keys: &[String],
    bib_store: &BibliographyStore,
    mode: CitationMode,
) -> String {
    let citations: Vec<String> = cite_keys
        .iter()
        .map(|key| {
            let entry = bib_store.get_entry(key).unwrap();
            format_apa_cite(entry)
        })
        .collect();
    
    match mode {
        CitationMode::Parenthetical => format!("({})", citations.join("; ")),
        CitationMode::Narrative => citations.join(" and "),
    }
}

fn format_apa_cite(entry: &BibEntry) -> String {
    let author = if entry.authors.is_empty() {
        "Unknown".to_string()
    } else if entry.authors.len() == 1 {
        entry.authors[0].family.clone()
    } else if entry.authors.len() == 2 {
        format!("{} & {}", entry.authors[0].family, entry.authors[1].family)
    } else {
        format!("{} et al.", entry.authors[0].family)
    };
    
    let year = entry.year
        .map(|y| y.to_string())
        .unwrap_or_else(|| "n.d.".to_string());
    
    format!("{}, {}", author, year)
}
```

### Bibliography Rendering

```rust
pub fn render_apa_bibliography(
    bib_store: &BibliographyStore,
) -> Vec<String> {
    let mut entries: Vec<_> = bib_store.entries().collect();
    
    // Sort alphabetically by first author's last name
    entries.sort_by(|a, b| {
        let a_author = a.authors.first().map(|au| &au.family).unwrap_or(&a.key);
        let b_author = b.authors.first().map(|au| &au.family).unwrap_or(&b.key);
        a_author.cmp(b_author)
    });
    
    entries.iter()
        .map(|entry| match entry.entry_type {
            BibEntryKind::Article => format_article_apa(entry),
            BibEntryKind::Book => format_book_apa(entry),
            BibEntryKind::InProceedings => format_conference_apa(entry),
            BibEntryKind::PhdThesis => format_thesis_apa(entry),
            BibEntryKind::TechReport => format_report_apa(entry),
            BibEntryKind::Online => format_website_apa(entry),
            _ => format_misc_apa(entry),
        })
        .collect()
}

fn format_article_apa(entry: &BibEntry) -> String {
    let authors = format_authors_apa(&entry.authors);
    let year = entry.year.map(|y| y.to_string()).unwrap_or_else(|| "n.d.".to_string());
    let title = &entry.title;
    let journal = entry.fields.get("journal").map(|s| s.as_str()).unwrap_or("");
    let volume = entry.fields.get("volume").map(|s| s.as_str()).unwrap_or("");
    let issue = entry.fields.get("issue").map(|s| format!("({})", s)).unwrap_or_default();
    let pages = entry.fields.get("pages").map(|s| s.as_str()).unwrap_or("");
    let doi = entry.fields.get("doi").map(|s| format!("https://doi.org/{}", s));
    
    let mut ref_str = format!("{}. ({}). {}. {}, {}{}, {}.",
        authors, year, title, journal, volume, issue, pages);
    
    if let Some(doi_url) = doi {
        ref_str.push_str(&format!(" {}", doi_url));
    }
    
    ref_str
}

fn format_authors_apa(authors: &[BibAuthor]) -> String {
    if authors.is_empty() {
        return "Unknown".to_string();
    }
    
    let formatted: Vec<String> = authors.iter()
        .take(20)
        .map(|author| {
            let initials = author.given
                .as_ref()
                .map(|g| g.split_whitespace()
                    .map(|part| format!("{}.", part.chars().next().unwrap()))
                    .collect::<Vec<_>>()
                    .join(" "))
                .unwrap_or_default();
            format!("{}, {}", author.family, initials)
        })
        .collect();
    
    if authors.len() > 20 {
        let last = &authors[authors.len() - 1];
        let last_initials = last.given
            .as_ref()
            .map(|g| format!("{}.", g.chars().next().unwrap()))
            .unwrap_or_default();
        format!("{}, ... {}, {}", formatted.join(", "), last.family, last_initials)
    } else if authors.len() == 1 {
        formatted[0].clone()
    } else {
        let (init, last) = formatted.split_at(formatted.len() - 1);
        format!("{}, & {}", init.join(", "), last[0])
    }
}
```

## Output Examples

### LaTeX Output

```latex
\documentclass{article}
\usepackage[style=apa]{biblatex}
\addbibresource{references.bib}

\begin{document}

Recent research \parencite{smith2024} shows...
\textcite{miller2023} demonstrated that...

\printbibliography

\end{document}
```

### DOCX Output

In-text: `(Smith, 2024)` as plain text

Bibliography: Alphabetical list with hanging indent

### Markdown Output

```markdown
Recent research (Smith, 2024) shows...
Miller and Chen (2023) demonstrated that...

## References

Knuth, D. E. (1984). The TeXbook. *Computers & Typesetting*, *A*, 1-483.

Lamport, L. (1994). *LaTeX: A document preparation system* (2nd ed.). 
    Addison-Wesley.
```

## Testing

```rust
#[test]
fn test_apa_citation() {
    let entry = create_test_article("smith2024");
    let citation = format_apa_cite(&entry);
    assert_eq!(citation, "Smith, 2024");
}

#[test]
fn test_apa_bibliography() {
    let entry = BibEntry {
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
        fields: [
            ("journal".to_string(), "Computers & Typesetting".to_string()),
            ("volume".to_string(), "A".to_string()),
            ("pages".to_string(), "1-483".to_string()),
            ("doi".to_string(), "10.1234/example".to_string()),
        ].iter().cloned().collect(),
    };
    
    let formatted = format_article_apa(&entry);
    assert!(formatted.contains("Knuth, D. E."));
    assert!(formatted.contains("(1984)"));
    assert!(formatted.contains("https://doi.org/10.1234/example"));
}
```

## References

- [APA Style 7th Edition](https://apastyle.apa.org/)
- [APA Citation Guide](https://owl.purdue.edu/owl/research_and_citation/apa_style/apa_formatting_and_style_guide/general_format.html)
- [APA Reference Examples](https://apastyle.apa.org/style-grammar-guidelines/references/examples)
