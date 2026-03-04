# Lontar CLI

[![Crates.io](https://img.shields.io/crates/v/lontar-cli.svg)](https://crates.io/crates/lontar-cli)
[![License](https://img.shields.io/crates/l/lontar-cli.svg)](https://github.com/lontar-rs/lontar#license)

**Command-line interface for the Lontar document generation library.**

Convert documents between multiple formats using the Lontar library from the command line.

## Installation

```bash
cargo install lontar-cli
```

## Usage

```bash
# Convert Markdown to DOCX
lontar input.md --format docx -o output.docx

# Convert template to PDF
lontar template.tera --format pdf -o output.pdf

# Convert to multiple formats
lontar input.md --format docx,pdf,html -o output
```

## Supported Formats

### Input
- Markdown
- Tera templates
- JSON (document AST)

### Output
- **DOCX**: Microsoft Word documents
- **PPTX**: Microsoft PowerPoint presentations
- **PDF**: Portable Document Format
- **XLSX**: Microsoft Excel spreadsheets
- **HTML**: Self-contained HTML with inline CSS
- **Markdown**: CommonMark-compatible Markdown
- **LaTeX**: XeLaTeX/LuaLaTeX source files
- **TXT**: Plain text with ASCII art tables

## Features

All features from the `lontar` library are available:

- Multi-format output
- Unicode and complex script support
- Citations and bibliography
- Cross-references
- Diagrams with auto-layout
- Template-based generation

## Examples

### Basic Conversion

```bash
lontar document.md --format docx -o output.docx
```

### With Bibliography

```bash
lontar paper.md --format pdf --bibliography refs.bib -o paper.pdf
```

### Template Rendering

```bash
lontar report.tera --data data.json --format docx -o report.docx
```

### Multiple Outputs

```bash
lontar input.md --format docx,pdf,html -o output
# Generates: output.docx, output.pdf, output.html
```

## Command-Line Options

```
lontar [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Input file

Options:
  -f, --format <FORMAT>        Output format(s): docx, pptx, pdf, xlsx, md, html, txt, tex
  -o, --output <OUTPUT>        Output file or directory
  -b, --bibliography <FILE>    Bibliography file (.bib or .json)
  -d, --data <FILE>            Template data file (JSON)
  -h, --help                   Print help
  -V, --version                Print version
```

## Library

This CLI is built on the [lontar](https://crates.io/crates/lontar) library. For programmatic document generation, use the library directly:

```toml
[dependencies]
lontar = { version = "0.1", features = ["docx", "pdf"] }
```

## Minimum Supported Rust Version (MSRV)

Rust **1.88** or later.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Links

- [Lontar Library](https://crates.io/crates/lontar)
- [Documentation](https://docs.rs/lontar)
- [Repository](https://github.com/lontar-rs/lontar)
- [Issue Tracker](https://github.com/lontar-rs/lontar/issues)
