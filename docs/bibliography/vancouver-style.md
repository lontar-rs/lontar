# Vancouver Citation Style

## Overview

The Vancouver style is the standard citation format for medical and biomedical journals. It uses **numeric citations** in the text and a **numbered reference list** at the end.

**Used by:**
- JAMA (Journal of the American Medical Association)
- BMJ (British Medical Journal)
- The Lancet
- New England Journal of Medicine (NEJM)
- Most medical journals

## In-Text Citations

### Numeric Format

Citations appear as **superscript numbers** or **numbers in parentheses**.

**Superscript (preferred):**
```
Recent studies have shown improved outcomes.¹
Multiple factors contribute to disease progression.²⁻⁴
```

**Parentheses (alternative):**
```
Recent studies have shown improved outcomes (1).
Multiple factors contribute to disease progression (2-4).
```

### Citation Rules

1. **Sequential numbering** — First citation is [1], second is [2], etc.
2. **Reuse numbers** — Same source cited again uses same number
3. **Multiple citations** — Separate with commas: `¹,³,⁵` or ranges: `²⁻⁴`
4. **Citation placement** — After punctuation, before period

**Examples:**
```
Correct: "...as demonstrated by Smith.¹"
Incorrect: "...as demonstrated by Smith¹."

Multiple: "Several studies¹⁻³,⁵ have confirmed..."
```

## Reference List Format

### General Rules

1. **Numbered list** — In order of first citation
2. **Author limit** — List first 6 authors, then "et al."
3. **Punctuation** — Periods separate major sections
4. **Abbreviations** — Journal names abbreviated per Index Medicus

### Journal Article

**Format:**
```
Author(s). Title. Journal abbreviation. Year;Volume(Issue):Pages.
```

**Example:**
```
1. Knuth DE. The TeXbook. Comput Typesetting. 1984;A:1-483.
```

**With DOI:**
```
2. Miller A, Chen B. Efficient graph layouts for large diagrams. 
   Proc Diagram Syst Conf. 2023;15:101-15. 
   doi:10.1234/diag.2023.15
```

**More than 6 authors:**
```
3. Smith J, Jones A, Brown C, Davis E, Wilson F, Taylor G, et al. 
   Multi-author study on typography. J Med Inform. 2024;42(3):234-45.
```

### Book

**Format:**
```
Author(s). Title. Edition. Place: Publisher; Year.
```

**Example:**
```
4. Lamport L. LaTeX: A Document Preparation System. 2nd ed. 
   Reading (MA): Addison-Wesley; 1994.
```

### Book Chapter

**Format:**
```
Author(s). Chapter title. In: Editor(s), editor(s). Book title. 
Edition. Place: Publisher; Year. p. Pages.
```

**Example:**
```
5. Jones S. Typography in the digital age. In: Brown R, editor. 
   Handbook of Modern Publishing. New York: Academic Press; 2020. 
   p. 45-78.
```

### Conference Paper

**Format:**
```
Author(s). Title. Conference name; Date; Place. Place: Publisher; Year. p. Pages.
```

**Example:**
```
6. Miller A, Chen B. Efficient graph layouts. Diagram Systems 
   Conference; 2023 Jun 15-17; Boston, MA. New York: ACM; 2023. 
   p. 101-15.
```

### Thesis/Dissertation

**Format:**
```
Author. Title [dissertation]. Place: University; Year.
```

**Example:**
```
7. Garcia M. Multi-script typesetting for scientific publishing 
   [dissertation]. Boston: University of Example; 2022.
```

### Technical Report

**Format:**
```
Author(s). Title. Place: Institution; Year. Report No.: Number.
```

**Example:**
```
8. Lee D. Unicode fonts for complex scripts. Cambridge (MA): 
   Example Research Lab; 2021. Report No.: ERL-2021-05.
```

### Website

**Format:**
```
Author(s). Title [Internet]. Place: Publisher; Year [cited Date]. 
Available from: URL
```

**Example:**
```
9. Singh P. Accessible math on the web: a practical guide [Internet]. 
   New York: Example Publishing; 2024 [cited 2024 Feb 1]. 
   Available from: https://example.com/accessible-math
```

## Journal Abbreviations

Vancouver uses **Index Medicus** abbreviations for journal names.

**Examples:**

| Full Name | Abbreviation |
|---|---|
| Journal of the American Medical Association | JAMA |
| New England Journal of Medicine | N Engl J Med |
| The Lancet | Lancet |
| British Medical Journal | BMJ |
| Nature | Nature |
| Science | Science |
| Proceedings of the National Academy of Sciences | Proc Natl Acad Sci USA |

**Rules:**
- Omit articles (the, a, an)
- Abbreviate significant words
- No periods in abbreviations
- Capitalize major words

## Implementation in Lontar

### Citation Rendering

```rust
pub fn render_vancouver_citation(
    cite_keys: &[String],
    bib_store: &BibliographyStore,
) -> String {
    let numbers: Vec<String> = cite_keys
        .iter()
        .map(|key| bib_store.get_citation_number(key).to_string())
        .collect();
    
    // Format as superscript range
    format_citation_range(&numbers)
}

fn format_citation_range(numbers: &[String]) -> String {
    if numbers.is_empty() {
        return String::new();
    }
    
    if numbers.len() == 1 {
        return numbers[0].clone();
    }
    
    // Check if consecutive
    let nums: Vec<u32> = numbers.iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    
    if is_consecutive(&nums) {
        format!("{}-{}", nums.first().unwrap(), nums.last().unwrap())
    } else {
        numbers.join(",")
    }
}
```

### Bibliography Rendering

```rust
pub fn render_vancouver_bibliography(
    bib_store: &BibliographyStore,
) -> Vec<String> {
    let mut refs = Vec::new();
    
    for (num, entry) in bib_store.entries_in_citation_order() {
        let formatted = match entry.entry_type {
            BibEntryKind::Article => format_article_vancouver(entry),
            BibEntryKind::Book => format_book_vancouver(entry),
            BibEntryKind::InProceedings => format_conference_vancouver(entry),
            BibEntryKind::PhdThesis => format_thesis_vancouver(entry),
            BibEntryKind::TechReport => format_report_vancouver(entry),
            BibEntryKind::Online => format_website_vancouver(entry),
            _ => format_misc_vancouver(entry),
        };
        
        refs.push(format!("{}. {}", num, formatted));
    }
    
    refs
}

fn format_article_vancouver(entry: &BibEntry) -> String {
    let authors = format_authors_vancouver(&entry.authors, 6);
    let title = &entry.title;
    let journal = entry.fields.get("journal").map(|s| s.as_str()).unwrap_or("");
    let year = entry.year.map(|y| y.to_string()).unwrap_or_default();
    let volume = entry.fields.get("volume").map(|s| s.as_str()).unwrap_or("");
    let pages = entry.fields.get("pages").map(|s| s.as_str()).unwrap_or("");
    
    format!("{}. {}. {}. {};{}:{}.", authors, title, journal, year, volume, pages)
}

fn format_authors_vancouver(authors: &[BibAuthor], max: usize) -> String {
    let formatted: Vec<String> = authors.iter()
        .take(max)
        .map(|author| {
            let initials = author.given
                .as_ref()
                .map(|g| g.chars()
                    .filter(|c| c.is_uppercase())
                    .collect::<String>())
                .unwrap_or_default();
            format!("{} {}", author.family, initials)
        })
        .collect();
    
    if authors.len() > max {
        format!("{}, et al", formatted.join(", "))
    } else {
        formatted.join(", ")
    }
}
```

## Output Examples

### LaTeX Output

```latex
\documentclass{article}
\usepackage[style=vancouver]{biblatex}
\addbibresource{references.bib}

\begin{document}

Recent studies have shown improved outcomes.\cite{knuth1984}
Multiple factors contribute.\cite{lamport1994,miller2023}

\printbibliography

\end{document}
```

### DOCX Output

In-text: Superscript field codes
```xml
<w:r>
  <w:rPr><w:vertAlign w:val="superscript"/></w:rPr>
  <w:t>1</w:t>
</w:r>
```

Bibliography: Numbered list with hanging indent

### Markdown Output

```markdown
Recent studies have shown improved outcomes.[1]
Multiple factors contribute.[2-4]

## References

1. Knuth DE. The TeXbook. Comput Typesetting. 1984;A:1-483.
2. Lamport L. LaTeX: A Document Preparation System. 2nd ed. 
   Reading (MA): Addison-Wesley; 1994.
```

## Testing

```rust
#[test]
fn test_vancouver_citation() {
    let mut store = BibliographyStore::new();
    store.add_entry(create_test_article("knuth1984"));
    store.add_entry(create_test_book("lamport1994"));
    
    let citation = render_vancouver_citation(
        &["knuth1984".to_string()],
        &store
    );
    assert_eq!(citation, "1");
    
    let citation = render_vancouver_citation(
        &["knuth1984".to_string(), "lamport1994".to_string()],
        &store
    );
    assert_eq!(citation, "1,2");
}

#[test]
fn test_vancouver_bibliography() {
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
            ("journal".to_string(), "Comput Typesetting".to_string()),
            ("volume".to_string(), "A".to_string()),
            ("pages".to_string(), "1-483".to_string()),
        ].iter().cloned().collect(),
    };
    
    let formatted = format_article_vancouver(&entry);
    assert_eq!(
        formatted,
        "Knuth DE. The TeXbook. Comput Typesetting. 1984;A:1-483."
    );
}
```

## References

- [ICMJE Recommendations](http://www.icmje.org/recommendations/)
- [NLM Citing Medicine](https://www.ncbi.nlm.nih.gov/books/NBK7256/)
- [Vancouver Style Guide](https://guides.library.ubc.ca/c.php?g=707463&p=5032145)
