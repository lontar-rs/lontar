# Lontar

[![Crates.io](https://img.shields.io/crates/v/lontar.svg)](https://crates.io/crates/lontar)
[![Documentation](https://docs.rs/lontar/badge.svg)](https://docs.rs/lontar)
[![License](https://img.shields.io/crates/l/lontar.svg)](https://github.com/lontar-rs/lontar#license)

**Comprehensive document generation library for Rust — write once, render everywhere.**

Lontar is a Rust library for creating documents programmatically and exporting them to multiple formats: DOCX, PPTX, PDF, XLSX, HTML, Markdown, LaTeX, and plain text.

## Features

- **Multi-format output**: Generate DOCX, PPTX, PDF, XLSX, HTML, Markdown, LaTeX, and plain text from a single document AST
- **Feature-gated backends**: Only compile the backends you need
- **Text shaping**: Full Unicode support with BiDi, complex scripts (Balinese, Devanagari, Arabic, etc.)
- **Native diagram engine**: Auto-layout flowcharts, trees, and graphs with format-specific rendering
- **Citations & bibliography**: BibTeX/BibLaTeX integration with multiple citation styles
- **Cross-references**: Automatic numbering and references for figures, tables, equations, and sections
- **Template system**: Declarative document templates via Tera

## Quick Start

Add Lontar to your `Cargo.toml`:

```toml
[dependencies]
lontar = { version = "0.1", features = ["docx", "pdf", "html"] }
```

### Example

```rust
use lontar::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let doc = DocumentBuilder::new()
        .title("My Document")
        .add_heading("Introduction", 1)
        .add_paragraph("This is a sample document.")
        .add_heading("Features", 1)
        .add_list(vec![
            "Multi-format output",
            "Unicode support",
            "Citations and cross-references",
        ])
        .build();

    // Export to DOCX
    DocxWriter::new().write(&doc, "output.docx")?;
    
    // Export to PDF
    PdfWriter::new().write(&doc, "output.pdf")?;
    
    // Export to HTML
    HtmlWriter::new().write(&doc, "output.html")?;

    Ok(())
}
```

## Feature Flags

Lontar uses feature flags to enable specific backends and functionality:

| Feature | Description |
|---------|-------------|
| `docx` | DOCX output (implies `aksara`) |
| `pptx` | PPTX output (implies `aksara`) |
| `pdf` | PDF output (implies `aksara`) |
| `xlsx` | XLSX output (wraps `rust_xlsxwriter`) |
| `md` | Markdown output |
| `html` | HTML output |
| `txt` | Plain text output |
| `aksara` | Text shaping, BiDi, font management |
| `diagram` | Native diagram engine with auto-layout |
| `template` | Declarative document templates (Tera) |
| `full` | All features enabled |

### Default Features

By default, **no features are enabled**. Enable only what you need:

```toml
# Minimal: Markdown and plain text only
lontar = { version = "0.1", features = ["md", "txt"] }

# Office formats only
lontar = { version = "0.1", features = ["docx", "pptx", "xlsx"] }

# Everything
lontar = { version = "0.1", features = ["full"] }
```

## Supported Formats

### Input
- Programmatic AST construction via builder API
- Template-based generation (with `template` feature)

### Output
- **DOCX**: Microsoft Word documents (Office Open XML)
- **PPTX**: Microsoft PowerPoint presentations
- **PDF**: Portable Document Format
- **XLSX**: Microsoft Excel spreadsheets
- **HTML**: Self-contained HTML with inline CSS
- **Markdown**: CommonMark-compatible Markdown
- **LaTeX**: XeLaTeX/LuaLaTeX source files with BibLaTeX
- **TXT**: Plain text with ASCII art tables

## Architecture

Lontar uses a single-crate architecture with feature-gated modules:

```
lontar/
├── core/       - AST types (Block, Inline, Document, styles)
├── aksara/     - Text shaping, BiDi, font management
├── diagram/    - Diagram layout engine
├── template/   - Tera template integration
└── backends/   - Format-specific writers
    ├── docx/
    ├── pptx/
    ├── pdf/
    ├── xlsx/
    ├── md/
    ├── html/
    └── txt/
```

## Multi-Script Support

Lontar supports complex text shaping for:

- **Latin, Cyrillic, Greek, Georgian**
- **Indic scripts**: Devanagari, Bengali, Tamil, Telugu, Kannada, Malayalam
- **Southeast Asian**: Balinese (ᬮᭀᬦ᭄ᬢᬭ᭄), Javanese, Thai, Lao, Khmer, Myanmar
- **RTL scripts**: Arabic, Hebrew, Syriac
- **CJK**: Chinese, Japanese, Korean
- **Tibetan** and other complex scripts

Text shaping is automatically enabled when using binary format backends (`docx`, `pptx`, `pdf`).

## Citation & Bibliography

```rust
use lontar::prelude::*;

let doc = DocumentBuilder::new()
    .load_bibliography("references.bib")?
    .add_paragraph("According to ")
    .add_citation("knuth1984", CitationMode::Narrative)
    .add_text(", TeX is a typesetting system.")
    .add_bibliography()
    .build();
```

Supports:
- BibTeX/BibLaTeX input
- CSL-JSON import (Zotero, Mendeley, EndNote)
- Multiple citation styles: numeric, author-year, Vancouver, APA, etc.

## Diagrams

```rust
use lontar::prelude::*;

let diagram = DiagramBuilder::new()
    .add_node("start", "Start")
    .add_node("process", "Process")
    .add_node("end", "End")
    .add_edge("start", "process")
    .add_edge("process", "end")
    .layout(LayoutAlgorithm::Layered)
    .build();

doc.add_diagram(diagram);
```

Renders to:
- **DOCX/PPTX**: DrawingML shapes
- **PDF**: Vector graphics
- **HTML**: SVG
- **LaTeX**: TikZ
- **Markdown**: Mermaid
- **TXT**: ASCII art

## Minimum Supported Rust Version (MSRV)

Rust **1.88** or later.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

## Links

- [Documentation](https://docs.rs/lontar)
- [Repository](https://github.com/lontar-rs/lontar)
- [Crates.io](https://crates.io/crates/lontar)
- [Issue Tracker](https://github.com/lontar-rs/lontar/issues)
