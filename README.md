# 🌿 Lontar

**A comprehensive document generation library for Rust.**

*Named after the palm leaf manuscripts of the Indonesian archipelago — ancient technology for preserving and sharing knowledge. Lontar brings that same purpose to modern document formats.*

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)]()
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)]()

---

## What is Lontar?

Lontar is a Rust-native library for generating documents across multiple output formats from a single, unified document model. Write your document once — render it to DOCX, PPTX, PDF, HTML, Markdown, or plain text.

```rust
use lontar::prelude::*;

let doc = Document::new("Quarterly Report")
    .heading(1, "Executive Summary")
    .paragraph("Revenue grew 42% year-over-year...")
    .table(
        &["Metric", "Q1", "Q2", "Q3"],
        &[
            &["Revenue", "$1.2M", "$1.5M", "$1.7M"],
            &["Users", "10K", "15K", "22K"],
        ],
    )
    .heading(1, "Recommendations")
    .paragraph("Based on current trajectory...")
    .build();

// One document, many formats
doc.write_docx("report.docx")?;
doc.write_pptx("report.pptx")?;
doc.write_pdf("report.pdf")?;
doc.write_md("report.md")?;
```

## Why Lontar?

| Problem | Lontar's Answer |
|---|---|
| Multi-service architecture for doc generation | Single-binary solution — no additional services or runtimes |
| Each format has its own isolated library and API | Unified document model with pluggable format backends |
| OOXML spec is massive and intimidating | Progressive coverage — common use cases first, full spec over time |
| Rust doc libraries cover individual formats | Unified, actively maintained, spec-compliant |

## Crate Structure

Lontar is a workspace of focused crates. Use only what you need:

| Crate | Purpose | Status |
|---|---|---|
| `lontar-core` | Unified document AST, styles, and traits | 🔴 Design |
| `lontar-docx` | OOXML WordprocessingML (.docx) output | 🔴 Design |
| `lontar-pptx` | OOXML PresentationML (.pptx) output | 🔴 Design |
| `lontar-pdf` | PDF output | 🔴 Design |
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
lontar-docx = "0.1"     # if you need .docx
lontar-pptx = "0.1"     # if you need .pptx
# ... or use the umbrella crate:
lontar = { version = "0.1", features = ["docx", "pptx", "pdf"] }
```

## Design Principles

1. **Unified AST, pluggable backends.** The document model is format-agnostic. Format-specific features are handled through extension traits and style hints, not separate APIs.

2. **Progressive fidelity.** Common operations (headings, paragraphs, tables, images, charts) work across all formats. Format-specific features (e.g., slide transitions in PPTX, tracked changes in DOCX) are available through format-specific extensions.

3. **Type-safe but ergonomic.** Rust's type system should prevent invalid documents at compile time, but the builder API should feel natural — not like wrestling with generics.

4. **Spec-compliant, not spec-complete.** We target correctness for the features we implement, following ECMA-376 (OOXML) and ISO 32000 (PDF). We don't aim to cover every obscure feature on day one.

5. **Zero unsafe, minimal dependencies.** Core document model uses no unsafe code. Format backends depend on `zip`, `quick-xml`, and format-specific crates only as needed.

## Roadmap

See [TODO.md](./TODO.md) for the detailed task breakdown.

| Phase | Target | Goal |
|---|---|---|
| 0 | Spec Research | Analyze OOXML structure, build test corpus |
| 1 | Core + MD + TXT | Prove the unified document model |
| 2 | DOCX | The primary deliverable |
| 3 | PPTX | Presentations with charts and tables |
| 4 | PDF + HTML | Complete the format matrix |
| 5 | Templates + CLI | Production tooling |

## Architecture

See [ARCHITECTURE.md](./ARCHITECTURE.md) for the full technical design.

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for development setup and guidelines.

## Comparison with Existing Tools

| Feature | lontar | python-docx | rust_xlsxwriter | docx-rs |
|---|---|---|---|---|
| Language | Rust | Python | Rust | Rust |
| Multi-format | ✅ | ❌ (docx only) | ❌ (xlsx only) | ❌ (docx only) |
| Unified model | ✅ | N/A | N/A | N/A |
| DOCX | 🔴 Planned | ✅ Mature | ❌ | 🟡 Basic |
| PPTX | 🔴 Planned | ❌ | ❌ | ❌ |
| PDF | 🔴 Planned | ❌ | ❌ | ❌ |
| Charts | 🔴 Planned | ❌ | ✅ | ❌ |
| Templates | 🔴 Planned | ❌ | ❌ | ❌ |

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Name & Heritage

**Lontar** (ลอนตาร์) — palm leaf manuscripts crafted across the Indonesian archipelago for over a millennium. In Bali, lontar manuscripts carry the *Usada* healing texts, the *Taru Pramana* botanical wisdom, and centuries of accumulated knowledge. Each leaf is carefully prepared, inscribed with a stylus, and bound together to form a complete document.

This library carries that spirit: carefully crafted components, bound together into a unified system for creating documents that preserve and share knowledge.

---

*Built with 🦀 and ☕ — documents, natively.*
