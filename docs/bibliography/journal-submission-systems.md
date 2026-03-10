# Journal Submission Systems Research

## Overview

This document surveys major academic journal submission systems and their LaTeX/PDF requirements, critical for understanding which output formats Lontar must support.

## Major Journal Categories

### Medical Journals

#### JAMA (Journal of the American Medical Association)

**Submission Requirements:**
- Format: PDF or Word
- LaTeX: Not explicitly supported
- Bibliography: Vancouver style (numbered)
- Font: Times New Roman, 12pt
- Line spacing: Double-spaced

**Workflow:**
1. Submit PDF or DOCX
2. Editorial review
3. If accepted, submit source files (Word or LaTeX)

**Implication for Lontar:**
- Must support PDF export with proper font embedding
- Must support DOCX export with Vancouver citations
- LaTeX not required but beneficial for author convenience

#### NEJM (New England Journal of Medicine)

**Submission Requirements:**
- Format: PDF or Word
- LaTeX: Not supported
- Bibliography: Vancouver style
- Font: Times New Roman, 12pt

**Workflow:**
1. Submit PDF or DOCX
2. Editorial review
3. If accepted, submit source files

**Implication for Lontar:**
- PDF and DOCX exports essential
- LaTeX support optional

#### Lancet

**Submission Requirements:**
- Format: PDF or Word
- LaTeX: Not supported
- Bibliography: Vancouver style
- Font: Times New Roman, 12pt

**Implication for Lontar:**
- PDF and DOCX exports essential
- LaTeX support optional

### Computer Science Journals

#### ACM Transactions

**Submission Requirements:**
- Format: PDF
- LaTeX: Supported via ACM templates
- Bibliography: Chicago style or custom
- Template: `acmart` class

**Workflow:**
1. Use ACM LaTeX template
2. Submit PDF
3. Source files (LaTeX) required if accepted

**Implication for Lontar:**
- LaTeX support beneficial
- Must generate compatible LaTeX output
- PDF export essential

#### IEEE Transactions

**Submission Requirements:**
- Format: PDF
- LaTeX: Supported via IEEE templates
- Bibliography: IEEE style
- Template: `IEEEtran` class

**Workflow:**
1. Use IEEE LaTeX template
2. Submit PDF
3. Source files required if accepted

**Implication for Lontar:**
- LaTeX support beneficial
- Must generate compatible LaTeX output
- PDF export essential

### Physics/Mathematics Journals

#### arXiv

**Submission Requirements:**
- Format: PDF
- LaTeX: Preferred (source files required)
- Bibliography: Any style
- Compilation: pdfLaTeX, XeLaTeX, LuaLaTeX

**Workflow:**
1. Submit LaTeX source files
2. arXiv compiles and generates PDF
3. Automatic versioning

**Implication for Lontar:**
- LaTeX support essential
- Must generate compilable LaTeX source
- PDF export for preview

#### Nature

**Submission Requirements:**
- Format: PDF or Word
- LaTeX: Not supported
- Bibliography: Nature style (numbered)
- Font: Times New Roman, 12pt

**Implication for Lontar:**
- PDF and DOCX exports essential
- LaTeX support optional

### Humanities Journals

#### PMLA (Modern Language Association)

**Submission Requirements:**
- Format: Word or PDF
- LaTeX: Not supported
- Bibliography: MLA style
- Font: Times New Roman, 12pt

**Implication for Lontar:**
- DOCX export essential
- PDF export beneficial
- LaTeX support optional

#### Chicago Journals

**Submission Requirements:**
- Format: Word or PDF
- LaTeX: Not supported
- Bibliography: Chicago style (notes-bibliography or author-date)
- Font: Times New Roman, 12pt

**Implication for Lontar:**
- DOCX export essential
- PDF export beneficial
- LaTeX support optional

## Submission System Platforms

### ScholarOne Manuscripts

Used by: JAMA, Lancet, Nature, many others

**Features:**
- Web-based submission
- Accepts PDF, Word, LaTeX
- Automatic format conversion
- Plagiarism detection
- Peer review workflow

**LaTeX Support:**
- Accepts LaTeX source files
- Compiles to PDF automatically
- Requires standard document class

**Implication for Lontar:**
- Must generate compilable LaTeX
- PDF export for preview
- DOCX export as fallback

### Editorial Manager

Used by: Elsevier journals, many others

**Features:**
- Web-based submission
- Accepts PDF, Word, LaTeX
- Automatic format conversion
- Plagiarism detection
- Peer review workflow

**LaTeX Support:**
- Accepts LaTeX source files
- Compiles to PDF automatically
- Supports custom document classes

**Implication for Lontar:**
- Must generate compilable LaTeX
- PDF export for preview
- DOCX export as fallback

### Open Journal Systems (OJS)

Used by: Open access journals, university presses

**Features:**
- Open source platform
- Web-based submission
- Accepts PDF, Word, LaTeX
- Customizable workflows

**LaTeX Support:**
- Varies by journal
- Some journals accept LaTeX source
- Others require PDF only

**Implication for Lontar:**
- Must support PDF export
- LaTeX support beneficial
- DOCX support helpful

### Overleaf

Used by: Authors for collaborative writing

**Features:**
- Cloud-based LaTeX editor
- Real-time collaboration
- Template library
- Direct submission to journals

**Implication for Lontar:**
- LaTeX output must be Overleaf-compatible
- Must work with standard document classes
- PDF export for preview

## LaTeX Compatibility Requirements

### Document Classes

**Standard Classes:**
- `article` — General articles
- `report` — Reports with chapters
- `book` — Books

**Specialized Classes:**
- `acmart` — ACM publications
- `IEEEtran` — IEEE publications
- `elsarticle` — Elsevier journals
- `achemso` — American Chemical Society
- `aip` — American Institute of Physics

**Implication for Lontar:**
- Must generate compatible LaTeX
- Should support multiple document classes
- Must handle class-specific requirements

### Required Packages

**Common Packages:**
- `babel` — Language support
- `fontspec` — Font selection (XeLaTeX/LuaLaTeX)
- `polyglossia` — Language-specific rules
- `biblatex` — Bibliography management
- `hyperref` — Hyperlinks and PDF metadata
- `graphicx` — Graphics inclusion
- `amsmath` — Mathematical typesetting

**Implication for Lontar:**
- Must generate compatible package declarations
- Should not conflict with class-provided packages
- Must handle package options correctly

### Compilation Requirements

**pdfLaTeX:**
- Most compatible
- Limited font support
- No native Unicode support
- Required by some journals

**XeLaTeX:**
- Modern font support
- Full Unicode support
- Slower compilation
- Preferred for multi-script documents

**LuaLaTeX:**
- Modern font support
- Full Unicode support
- Lua scripting support
- Preferred for complex documents

**Implication for Lontar:**
- Must generate pdfLaTeX-compatible output for maximum compatibility
- Should support XeLaTeX/LuaLaTeX for advanced features
- Must handle font selection appropriately

## Recommended Lontar Output Strategy

### Phase 1: PDF Export

**Priority:** High
**Rationale:** Universal acceptance, no compilation required

**Implementation:**
- Use PDF library (e.g., `printpdf`, `genpdf`)
- Embed fonts
- Support all scripts
- Optimize file size

### Phase 2: DOCX Export

**Priority:** High
**Rationale:** Widely accepted, easy editing

**Implementation:**
- Use DOCX library (e.g., `docx-rs`)
- Embed fonts
- Support all scripts
- Preserve formatting

### Phase 3: LaTeX Export

**Priority:** Medium
**Rationale:** Required for arXiv, beneficial for academic authors

**Implementation:**
- Generate pdfLaTeX-compatible output
- Support XeLaTeX/LuaLaTeX for advanced features
- Use fontspec for font selection
- Generate compilable source

### Phase 4: Journal-Specific Templates

**Priority:** Low
**Rationale:** Convenience for authors

**Implementation:**
- Support ACM template (acmart)
- Support IEEE template (IEEEtran)
- Support Elsevier template (elsarticle)
- Provide template selection UI

## Integration with Lontar

### 1. Output Format Selection

```rust
pub enum OutputFormat {
    PDF,
    DOCX,
    PPTX,
    LaTeX,
    HTML,
    Markdown,
    PlainText,
}

pub fn export_document(
    document: &Document,
    format: OutputFormat,
    output_path: &str,
) -> Result<()> {
    match format {
        OutputFormat::PDF => export_pdf(document, output_path),
        OutputFormat::DOCX => export_docx(document, output_path),
        OutputFormat::LaTeX => export_latex(document, output_path),
        OutputFormat::HTML => export_html(document, output_path),
        OutputFormat::Markdown => export_markdown(document, output_path),
        OutputFormat::PlainText => export_plaintext(document, output_path),
        _ => Err("Unsupported format".into()),
    }
}
```

### 2. Journal-Specific Export

```rust
pub enum JournalTemplate {
    Generic,
    ACM,
    IEEE,
    Elsevier,
    JAMA,
    NEJM,
    Lancet,
}

pub fn export_for_journal(
    document: &Document,
    journal: JournalTemplate,
    output_path: &str,
) -> Result<()> {
    match journal {
        JournalTemplate::ACM => export_acm_latex(document, output_path),
        JournalTemplate::IEEE => export_ieee_latex(document, output_path),
        JournalTemplate::Elsevier => export_elsevier_docx(document, output_path),
        JournalTemplate::JAMA => export_jama_docx(document, output_path),
        JournalTemplate::NEJM => export_nejm_docx(document, output_path),
        JournalTemplate::Lancet => export_lancet_docx(document, output_path),
        _ => export_pdf(document, output_path),
    }
}
```

### 3. Citation Style Configuration

```rust
pub struct ExportConfig {
    pub format: OutputFormat,
    pub citation_style: CitationStyle,
    pub bibliography_title: String,
    pub include_toc: bool,
    pub include_page_numbers: bool,
    pub font_embedding: bool,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            format: OutputFormat::PDF,
            citation_style: CitationStyle::APA7,
            bibliography_title: "References".to_string(),
            include_toc: true,
            include_page_numbers: true,
            font_embedding: true,
        }
    }
}
```

## Performance Characteristics

| Operation | Time | Notes |
|-----------|------|-------|
| Export to PDF | ~500-2000ms | Depends on document size |
| Export to DOCX | ~300-1000ms | Depends on document size |
| Export to LaTeX | ~100-500ms | Fast |
| Export to HTML | ~100-500ms | Fast |

## References

- [ScholarOne Manuscripts](https://clarivate.com/products/scholarone/)
- [Editorial Manager](https://www.editorialmanager.com/)
- [Open Journal Systems](https://pkp.sfu.ca/ojs/)
- [Overleaf](https://www.overleaf.com/)
- [JAMA Instructions for Authors](https://jamanetwork.com/journals/jama/pages/instructions-for-authors)
- [NEJM Instructions for Authors](https://www.nejm.org/authors/manuscript-submission)
- [ACM Publications](https://www.acm.org/publications)
- [IEEE Publications](https://www.ieee.org/publications/rights/index.html)
