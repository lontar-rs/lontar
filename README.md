# 🌿 Lontar

**A comprehensive document generation library for Rust.**

*Named after the palm leaf manuscripts of the Indonesian archipelago — ancient technology for preserving and sharing knowledge. Lontar brings that same purpose to modern document formats.*

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)]()
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)]()

---

## What is Lontar?

Lontar is a Rust-native library for generating documents across multiple output formats from a single, unified document model. Write your document once — in any script, any language — and render it to DOCX, PPTX, PDF, LaTeX, HTML, Markdown, or plain text.

```rust
use lontar::prelude::*;

let bib = BibliographyStore::from_bibtex_file("references.bib")?;

let doc = Document::new("Efficacy of Novel Treatment Protocol")
    .bibliography(bib)
    .bibliography_style(BibliographyStyle::Vancouver)
    .heading(1, "Introduction")
    .paragraph(|p| p
        .text("Cardiovascular disease remains the leading cause of mortality worldwide")
        .cite(&["who2024", "gbd2023"], CitationMode::Parenthetical)
        .text(". Recent advances in treatment protocols have shown promise")
        .cite(&["smith2024"], CitationMode::Parenthetical)
        .text(".")
    )
    .heading(1, "Methods")
    .paragraph("A randomized controlled trial was conducted across 12 centers...")
    .table_with_label(
        &["Outcome", "Treatment (n=240)", "Control (n=238)", "p-value"],
        &[
            &["Primary endpoint", "42.3%", "28.1%", "<0.001"],
            &["Secondary endpoint", "67.8%", "51.2%", "0.003"],
        ],
        "tab:results",
    )
    .paragraph(|p| p
        .text("Results are summarized in ")
        .crossref("tab:results", CrossRefKind::Auto)
        .text(".")
    )
    .heading(1, "References")
    .render_bibliography()
    .build();

// One document, every format a researcher needs
doc.write_latex("submission.tex")?;     // Journal submission
doc.write_docx("review_copy.docx")?;    // Collaborator review
doc.write_pdf("preprint.pdf")?;          // Preprint upload
doc.write_md("readme.md")?;             // Repository documentation
```

## Why Lontar?

| Problem | Lontar's Answer |
|---|---|
| Multi-service architecture for doc generation | Single-binary solution — no additional services or runtimes |
| Each format has its own isolated library and API | Unified document model with pluggable format backends |
| OOXML spec is massive and intimidating | Progressive coverage — common use cases first, full spec over time |
| Rust doc libraries cover individual formats | Unified, actively maintained, spec-compliant |
| Complex scripts (Arabic, Devanagari, Balinese, CJK...) are an afterthought | Universal script support built into the foundation via text shaping pipeline |
| LaTeX is powerful but inaccessible to non-technical researchers | Write once, emit LaTeX + DOCX + PDF — no markup language knowledge required |
| Citation management is fragmented across tools | Unified bibliography system: one `.bib` file, correct citations in every format |

## Crate Structure

Lontar is a workspace of focused crates. Use only what you need:

| Crate | Purpose | Status |
|---|---|---|
| `lontar-core` | Unified document AST, styles, citations, and traits | 🔴 Design |
| `lontar-aksara` | Text shaping, BiDi, line breaking, font management | 🔴 Design |
| `lontar-docx` | OOXML WordprocessingML (.docx) output | 🔴 Design |
| `lontar-pptx` | OOXML PresentationML (.pptx) output | 🔴 Design |
| `lontar-pdf` | PDF output | 🔴 Design |
| `lontar-latex` | LaTeX output (.tex + .bib) | 🔴 Design |
| `lontar-xlsx` | XLSX output (wraps rust_xlsxwriter) | 🔴 Design |
| `lontar-diagram` | Native diagramming with auto-layout | 🔴 Design |
| `lontar-md` | Markdown output | 🔴 Design |
| `lontar-html` | HTML output | 🔴 Design |
| `lontar-txt` | Plain text output | 🔴 Design |
| `lontar-template` | Declarative document templates | 🔴 Design |
| `lontar-cli` | CLI tool for document conversion | 🔴 Design |

**Status legend:** 🔴 Design · 🟡 In Progress · 🟢 Usable · ✅ Stable

## Quick Start

Add the crates you need to your `Cargo.toml`:

```toml
[dependencies]
lontar-core = "0.1"
lontar-docx = "0.1"      # if you need .docx
lontar-pptx = "0.1"      # if you need .pptx
lontar-latex = "0.1"     # if you need .tex
# ... or use the umbrella crate:
lontar = { version = "0.1", features = ["docx", "pptx", "pdf", "latex", "xlsx"] }
```

## Design Principles

1. **Unified AST, pluggable backends.** The document model is format-agnostic. Format-specific features are handled through extension traits and style hints, not separate APIs.

2. **Universal script support.** Every writing system in Unicode — Latin, Arabic, Devanagari, CJK, Balinese, and 150+ others — works correctly through an integrated text shaping pipeline. Complex scripts are a first-class concern, not an afterthought.

3. **Citations as a first-class concept.** The bibliography system is part of the core AST, not a format-specific extension. Write `doc.cite(&["smith2024"])` and every backend renders it correctly — `\parencite{smith2024}` in LaTeX, a field code in DOCX, `[1]` in Markdown, a hyperlinked reference in HTML.

4. **Progressive fidelity.** Common operations (headings, paragraphs, tables, images, charts, citations) work across all formats. Format-specific features (e.g., slide transitions in PPTX, tracked changes in DOCX) are available through format-specific extensions.

5. **Type-safe but ergonomic.** Rust's type system should prevent invalid documents at compile time, but the builder API should feel natural — not like wrestling with generics.

6. **Spec-compliant, not spec-complete.** We target correctness for the features we implement, following ECMA-376 (OOXML) and ISO 32000 (PDF). We don't aim to cover every obscure feature on day one.

7. **Zero unsafe, minimal dependencies.** Core document model uses no unsafe code. Format backends depend on `zip`, `quick-xml`, and format-specific crates only as needed. Text-based backends (Markdown, LaTeX, plain text) have zero external dependencies.

## Academic Workflow

Lontar is designed to solve the multi-format problem that researchers face daily:

```
  Zotero / Mendeley / EndNote
          │
          │  export .bib
          ▼
  ┌───────────────┐
  │  lontar-core   │  ← unified document + bibliography
  │  + your data   │
  └───────┬───────┘
          │
    ┌─────┼─────┬──────┬──────┐
    ▼     ▼     ▼      ▼      ▼
  .tex  .docx  .pdf   .html  .md
    │     │     │      │      │
    ▼     ▼     ▼      ▼      ▼
 Journal  Co-   Pre-   Web   Repo
 submit  author print  view  docs
```

**Supported bibliography styles:** Vancouver (medicine), APA 7th (psychology), numeric, author-year, superscript, and extensible via named styles.

## Roadmap

See [TODO.md](./TODO.md) for the detailed task breakdown.

| Phase | Target | Goal |
|---|---|---|
| 0 | Spec Research | Analyze OOXML + LaTeX structure, build test corpus |
| 1 | Core + MD + TXT | Prove the unified document model (including citations) |
| 1.5 | Text Shaping | Universal script support pipeline |
| 2 | DOCX | Word document generation |
| 3 | PPTX | Presentations with charts and tables |
| 4 | PDF + HTML + LaTeX + XLSX | Complete the format matrix |
| 5 | Templates + CLI | Production tooling (including journal templates) |
| 6 | Polish & Ecosystem | Stabilization and ecosystem integration |

## Architecture

See [ARCHITECTURE.md](./ARCHITECTURE.md) for the full technical design.

## Design Decisions

See [DESIGN_DECISIONS.md](./DESIGN_DECISIONS.md) for architectural choices and their rationale.

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for development setup and guidelines.

## Comparison with Existing Tools

| Feature | lontar | python-docx | rust_xlsxwriter | docx-rs | pandoc |
|---|---|---|---|---|---|
| Language | Rust | Python | Rust | Rust | Haskell |
| Multi-format | ✅ | ❌ (docx only) | ❌ (xlsx only) | ❌ (docx only) | ✅ |
| Unified model | ✅ | N/A | N/A | N/A | ✅ (internal) |
| Library (not CLI) | ✅ | ✅ | ✅ | ✅ | ❌ (primarily CLI) |
| Universal scripts | 🔴 Planned | Partial | Partial | ❌ | ✅ |
| Citations | 🔴 Planned | ❌ | ❌ | ❌ | ✅ (via citeproc) |
| DOCX | 🔴 Planned | ✅ Mature | ❌ | 🟡 Basic | ✅ |
| PPTX | 🔴 Planned | ❌ | ❌ | ❌ | ✅ (basic) |
| LaTeX | 🔴 Planned | ❌ | ❌ | ❌ | ✅ |
| PDF | 🔴 Planned | ❌ | ❌ | ❌ | ✅ (via LaTeX) |
| XLSX | 🔴 Planned | ❌ | ✅ Mature | ❌ | ❌ |
| Charts | 🔴 Planned | ❌ | ✅ | ❌ | ❌ |
| Diagrams | 🔴 Planned | ❌ | ❌ | ❌ | ❌ |
| Templates | 🔴 Planned | ❌ | ❌ | ❌ | ✅ |
| Embeddable | ✅ (library) | ✅ | ✅ | ✅ | ❌ (subprocess) |

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Name & Heritage

**Lontar** (ᬮᭀᬦ᭄ᬢᬭ᭄ in Aksara Bali) — palm leaf manuscripts crafted across the Indonesian archipelago for over a millennium. In Bali, lontar manuscripts carry the *Usada* healing texts, the *Taru Pramana* botanical wisdom, and centuries of accumulated knowledge. Each leaf is carefully prepared, inscribed with a stylus, and bound together to form a complete document.

This library carries that spirit: carefully crafted components, bound together into a unified system for creating documents that preserve and share knowledge — in every script those documents may need.

---

*Built with 🦀 and ☕ — documents, natively.*