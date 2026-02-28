# TODO

Detailed task breakdown by phase. Check the box when complete.

---

## Phase 0 — Research & Spec Analysis

**Goal:** Understand exactly what XML we need to generate. Kill the unknown unknowns.

### OOXML Spec Analysis
- [x] Download ECMA-376 5th Edition (Office Open XML)
- [x] Document the minimum viable XML for a "hello world" .docx
- [x] Document the minimum viable XML for a "hello world" .pptx
- [x] Map docx XML elements to our AST nodes (create mapping table)
- [x] Map pptx XML elements to our AST nodes (create mapping table)
- [x] Identify shared OOXML components (DrawingML, Chart markup, relationships)
- [x] Document the relationship/content_types system (.rels files)
- [x] Catalog EMU (English Metric Units) coordinate system for pptx

### Test Corpus Generation
- [ ] Write Python script to generate reference documents using python-docx:
  - [x] Minimal doc (single paragraph)
  - [x] Headings (all 6 levels)
  - [x] Styled text (bold, italic, underline, color, font size)
  - [x] Tables (simple, styled, merged cells)
  - [x] Images (inline, floating)
  - [x] Lists (ordered, unordered, nested)
  - [x] Headers and footers
  - [x] Page breaks and section breaks
  - [x] Table of contents
  - [x] Hyperlinks
  - [x] Block quotes
- [x] Write Python script to generate reference documents using python-pptx:
  - [x] Title slide
  - [x] Content slide with bullets
  - [x] Table slide
  - [x] Chart slide (bar, line, pie)
  - [x] Image slide
  - [x] Two-column layout
  - [x] Slide with speaker notes
- [x] Unzip all generated docs, organize XML for reference
- [x] Create `tests/fixtures/reference_docs/` with all samples
- [x] Create `tests/fixtures/expected_xml/` with extracted XML fragments
- [ ] For each reference doc, manually create expected output in MD, HTML, and TXT formats
- [ ] Verify reference docs open correctly in LibreOffice and MS Office Online

### Reference Output Fixtures
- [ ] Create `tests/fixtures/expected_md/` with expected Markdown output for each test document
- [ ] Create `tests/fixtures/expected_html/` with expected HTML output for each test document
- [ ] Create `tests/fixtures/expected_txt/` with expected plain text output for each test document
- [ ] Create `tests/fixtures/expected_pdf/` with reference PDF output (via typst or LaTeX) for visual comparison
- [ ] Define expected Markdown for: headings, styled text, tables, lists, images, code blocks, block quotes, links, horizontal rules
- [ ] Define expected HTML for: headings, styled text, tables, lists, images, code blocks, block quotes, links (self-contained with inline CSS)
- [ ] Define expected TXT for: headings (underline style), tables (ASCII art), lists (indented), word-wrapped paragraphs
- [ ] Define expected PDF for: basic report, styled text, tables, images (visual reference only)

### Multi-Script Test Corpus
- [ ] Create `tests/fixtures/scripts/` directory for script-specific test data
- [ ] Create test strings file with representative text for each script category:
  - [ ] Simple LTR: Latin, Cyrillic, Greek, Georgian
  - [ ] Complex Indic: Devanagari (conjuncts, split matras), Bengali, Tamil, Telugu, Kannada, Malayalam
  - [ ] Southeast Asian: Balinese (ᬮᭀᬦ᭄ᬢᬭ᭄), Javanese, Sundanese, Batak, Thai, Lao, Khmer, Myanmar
  - [ ] RTL: Arabic (contextual joining), Hebrew, Syriac, Thaana, N'Ko
  - [ ] CJK: Simplified Chinese, Traditional Chinese, Japanese (Kanji + Hiragana + Katakana), Korean (Hangul)
  - [ ] Tibetan: stacking consonants
  - [ ] African: Ethiopic, Tifinagh
  - [ ] Mixed-script paragraphs: Latin + Balinese, Arabic + English, CJK + Latin
- [ ] Collect test fonts: Noto Sans families for each script category
- [ ] Generate reference documents (via python-docx) containing multi-script text — verify they open correctly
- [ ] Generate reference presentations (via python-pptx) containing multi-script text — verify they open correctly
- [ ] Document expected shaping behavior for each script (which conjuncts should form, which reorderings should occur)

### Tooling Setup
- [ ] Initialize Cargo workspace
- [ ] Set up CI (GitHub Actions): clippy, fmt, test, miri
- [ ] Configure code coverage (cargo-llvm-cov)
- [ ] Set up benchmarks (criterion)
- [ ] Create issue templates for bug reports and feature requests
- [ ] Set up dependabot for dependency updates

### Text Shaping Research
- [ ] Evaluate `rustybuzz` API and capabilities
- [ ] Evaluate `unicode-bidi` for bidirectional text
- [ ] Evaluate `unicode-linebreak` for line break opportunities
- [ ] Evaluate `unicode-script` for script run detection
- [ ] Research font subsetting approaches in Rust
- [ ] Collect test fonts: Noto Sans Balinese, Noto Sans Devanagari, Noto Sans Arabic, Noto Sans CJK
- [ ] Create test strings for each script category (simple LTR, complex Indic, SE Asian, RTL, CJK)
- [ ] Document how DOCX/PPTX handle font embedding and language tagging
- [ ] Document how PDF embeds fonts and stores positioned glyphs
- [ ] Prototype: shape "ᬮᭀᬦ᭄ᬢᬭ᭄" (lontar in Aksara Bali) with rustybuzz → verify conjunct formation

---

## Phase 1 — Core Document Model + Simple Backends

**Goal:** Prove the unified AST works with the simplest possible outputs.

### lontar-core
- [ ] Define `Block` enum with all variants
- [ ] Define `Inline` enum with all variants
- [ ] Define `TextStyle` struct
- [ ] Define `ParagraphStyle` struct
- [ ] Define `TableStyle` struct
- [ ] Define `ChartKind` and `ChartData` types
- [ ] Define `PageSetup` and `Margins`
- [ ] Define `StyleSheet` with cascading resolution
- [ ] Define `DocumentMetadata`
- [ ] Define `ResourceStore` for images and binary assets
- [ ] Implement `Document` struct with root node
- [ ] Implement `DocumentBuilder` (ergonomic builder API)
- [ ] Implement `DocumentWriter` trait
- [ ] Implement `WriteReport` for feature degradation reporting
- [ ] Implement `LontarError` error types
- [ ] Implement style cascade resolution (default → named → paragraph → run)
- [ ] Write unit tests for AST construction
- [ ] Write unit tests for style resolution
- [ ] Write documentation for all public types

### lontar-md (Markdown Backend)
- [ ] Implement `MarkdownWriter` for `DocumentWriter` trait
- [ ] Handle: headings, paragraphs, bold/italic/code text
- [ ] Handle: tables (GitHub-flavored Markdown)
- [ ] Handle: ordered and unordered lists
- [ ] Handle: images (as `![alt](path)`)
- [ ] Handle: links
- [ ] Handle: code blocks with language
- [ ] Handle: block quotes
- [ ] Handle: horizontal rules
- [ ] Graceful degradation: charts → table fallback
- [ ] Graceful degradation: page breaks → `---`
- [ ] Graceful degradation: headers/footers → skip with warning
- [ ] Write snapshot tests against expected Markdown output
- [ ] Verify round-trip: AST → MD → parse with pulldown-cmark → compare structure

### lontar-txt (Plain Text Backend)
- [ ] Implement `PlainTextWriter` for `DocumentWriter` trait
- [ ] Handle: headings (underlined with = or -)
- [ ] Handle: paragraphs with word wrapping
- [ ] Handle: tables (ASCII art)
- [ ] Handle: lists (with indentation)
- [ ] Write snapshot tests

---

## Phase 1.5 — Text Shaping Pipeline

**Goal:** Build the universal text shaping infrastructure that enables correct rendering of all Unicode scripts.

### lontar-aksara — Core Pipeline
- [ ] Create `lontar-aksara` crate with module structure
- [ ] Integrate `rustybuzz` for OpenType text shaping
- [ ] Integrate `unicode-bidi` for bidirectional text resolution
- [ ] Integrate `unicode-linebreak` for script-aware line breaking
- [ ] Integrate `unicode-script` for script run segmentation
- [ ] Define `ShapedRun` output type (glyph IDs, positions, advances)
- [ ] Define `TextPipeline` API: text + font → shaped runs

### Font Management
- [ ] Implement `FontManager` with family registration
- [ ] Implement font fallback chain (per-script coverage detection)
- [ ] Parse font cmap tables to detect script coverage
- [ ] Implement font subsetting (extract only used glyphs)
- [ ] Support loading fonts from file paths and embedded bytes
- [ ] Optional: system font discovery

### Language & Script Support
- [ ] Implement script run segmentation (split mixed-script text into runs)
- [ ] Implement BCP 47 language tagging
- [ ] Handle mixed-direction paragraphs (e.g., Latin + Arabic in one line)

### Testing — Script Categories
- [ ] Simple LTR: Latin, Cyrillic, Greek
- [ ] Complex Indic: Devanagari conjuncts, split matras
- [ ] Southeast Asian: Balinese (ᬮᭀᬦ᭄ᬢᬭ᭄), Javanese, Thai
- [ ] RTL: Arabic contextual joining, Hebrew
- [ ] RTL + LTR mixed: Arabic paragraph with English words
- [ ] CJK: Han characters, Hiragana, Katakana, Hangul
- [ ] Tibetan: stacking behavior
- [ ] African: Ethiopic syllabary
- [ ] Edge cases: zero-width joiners, combining marks, emoji with skin tones

### Benchmarks
- [ ] Shaping throughput: glyphs/second for each script category
- [ ] Font subsetting time for various font sizes
- [ ] Memory usage for loaded fonts

---

## Phase 2 — DOCX Backend

**Goal:** Generate Word documents that open correctly in MS Office and LibreOffice.

### Core OOXML Infrastructure
- [ ] Implement ZIP archive builder (using `zip` crate)
- [ ] Implement `[Content_Types].xml` generator
- [ ] Implement `.rels` relationship manager
- [ ] Implement `docProps/core.xml` (metadata) generator
- [ ] Implement `docProps/app.xml` generator
- [ ] Implement XML escaping utilities
- [ ] Implement EMU/twip/point unit conversion utilities

### Document Content (`word/document.xml`)
- [ ] Paragraph generation (`<w:p>`)
- [ ] Text runs with formatting (`<w:r>`, `<w:rPr>`)
- [ ] Headings with built-in styles
- [ ] Bold, italic, underline, strikethrough
- [ ] Font family and size
- [ ] Text color and highlighting
- [ ] Superscript and subscript
- [ ] Hyperlinks (`<w:hyperlink>`)
- [ ] Line breaks within paragraphs
- [ ] Page breaks (`<w:br w:type="page"/>`)

### Tables
- [ ] Basic table structure (`<w:tbl>`, `<w:tr>`, `<w:tc>`)
- [ ] Table column widths
- [ ] Header row designation
- [ ] Cell borders and shading
- [ ] Cell vertical alignment
- [ ] Merged cells (horizontal span)
- [ ] Merged cells (vertical span)

### Lists
- [ ] `word/numbering.xml` generator
- [ ] Unordered lists (bullets)
- [ ] Ordered lists (numbers)
- [ ] Nested lists with correct indentation
- [ ] Custom bullet/number styles

### Images
- [ ] Image embedding in `word/media/`
- [ ] Inline image insertion (`<w:drawing>` → `<wp:inline>`)
- [ ] Image sizing (maintain aspect ratio)
- [ ] Alt text support
- [ ] PNG, JPEG, GIF support
- [ ] Image relationship management

### Styles
- [ ] `word/styles.xml` generator
- [ ] Default document style
- [ ] Heading styles (Heading 1-6)
- [ ] Custom named styles
- [ ] Style inheritance chain

### Page Layout
- [ ] Page size (A4, Letter, custom)
- [ ] Page orientation (portrait, landscape)
- [ ] Margins
- [ ] Headers (`word/header1.xml`)
- [ ] Footers (`word/footer1.xml`)
- [ ] Page numbers in header/footer
- [ ] Section breaks

### Advanced Features
- [ ] Table of contents (field codes)
- [ ] Footnotes (`word/footnotes.xml`)
- [ ] Block quotes (styled paragraphs)
- [ ] Code blocks (monospaced styled paragraphs)
- [ ] Horizontal rules

### Font & Script Integration
- [ ] Font embedding in docx ZIP (`word/fonts/`)
- [ ] Font subsetting for embedded fonts
- [ ] Language tagging on text runs (`w:lang` attribute)
- [ ] Complex script run properties (`w:cs`, `w:rtl`)
- [ ] Bidirectional paragraph handling

### Testing & Validation
- [ ] Open generated docs in LibreOffice — visual verification
- [ ] Open generated docs in MS Office Online — visual verification
- [ ] Compare XML structure against python-docx reference docs
- [ ] Test with complex documents (20+ pages, mixed content)
- [ ] Test with Aksara Bali text (ᬮᭀᬦ᭄ᬢᬭ᭄) — verify conjuncts render
- [ ] Test with Arabic text — verify RTL and contextual joining
- [ ] Test with Devanagari text — verify conjuncts and matras
- [ ] Test with CJK characters
- [ ] Test with mixed-script paragraphs (Latin + Balinese + Arabic)
- [ ] Test with large images (>5MB)
- [ ] Performance benchmarks (criterion)

---

## Phase 3 — PPTX Backend

**Goal:** Generate PowerPoint presentations with slides, tables, and charts.

### Core PPTX Infrastructure
- [ ] Slide master template (minimal `slideMaster1.xml`)
- [ ] Slide layout templates (title, content, two-column, blank)
- [ ] Theme definition (`theme1.xml`)
- [ ] Presentation properties (`presProps.xml`)
- [ ] Slide relationship management
- [ ] Coordinate system utilities (EMU positioning)

### Slide Types
- [ ] Title slide (centered title + subtitle)
- [ ] Content slide (title + bullet list body)
- [ ] Two-column slide
- [ ] Blank slide with positioned elements
- [ ] Section divider slide

### Content Elements
- [ ] Text frames with auto-fit
- [ ] Styled text (bold, italic, color, size)
- [ ] Bullet lists within text frames
- [ ] Tables on slides
- [ ] Images on slides (positioned and sized)

### Charts (DrawingML)
- [ ] Bar chart (vertical)
- [ ] Bar chart (horizontal)
- [ ] Line chart
- [ ] Pie chart
- [ ] Chart title and legend
- [ ] Chart data labels
- [ ] Axis labels and formatting

### Speaker Notes
- [ ] Notes slide generation
- [ ] Plain text notes
- [ ] Formatted notes

### Testing
- [ ] Open in LibreOffice Impress
- [ ] Open in MS PowerPoint Online
- [ ] Compare against python-pptx reference
- [ ] Test slide count limits
- [ ] Performance benchmarks

---

## Phase 4 — PDF + HTML Backends

### lontar-pdf
- [ ] Evaluate: typst vs printpdf vs genpdf (build prototypes)
- [ ] Select approach and implement `PdfWriter`
- [ ] Page layout (margins, headers, footers, page numbers)
- [ ] Text rendering with fonts
- [ ] Tables
- [ ] Images
- [ ] Charts (render to image, embed)
- [ ] Hyperlinks
- [ ] Table of contents with page numbers
- [ ] Font embedding (subset)
- [ ] PDF/A compliance (optional)

### lontar-html
- [ ] Implement `HtmlWriter`
- [ ] Self-contained HTML (inline CSS)
- [ ] Optional external CSS mode
- [ ] Responsive table rendering
- [ ] Image embedding (base64 data URIs)
- [ ] Chart rendering (SVG or Canvas via JS)
- [ ] Print-friendly CSS

---

## Phase 5 — Templates + CLI

### lontar-template
- [ ] Define template format (Tera-based or custom DSL)
- [ ] Template parsing and validation
- [ ] Variable substitution in text
- [ ] Conditional sections (`{% if ... %}`)
- [ ] Loop sections (`{% for ... %}`)
- [ ] Template inheritance / includes
- [ ] Built-in templates (report, memo, presentation, invoice)
- [ ] Template documentation and examples

### lontar-cli
- [ ] `lontar convert input.md --to docx,pptx,pdf`
- [ ] `lontar template report.lontar --data data.json --to docx`
- [ ] `lontar inspect file.docx` (show document structure)
- [ ] `lontar validate file.docx` (check OOXML conformance)
- [ ] Shell completions (bash, zsh, fish)
- [ ] Man page generation

---

## Phase 6 — Polish & Ecosystem

### Documentation
- [ ] API documentation (rustdoc) for all public items
- [ ] User guide (mdbook)
- [ ] Tutorial: "Generate your first document"
- [ ] Tutorial: "Create a custom template"
- [ ] Example gallery with source code
- [ ] Migration guide from python-docx

### Quality
- [ ] Fuzz testing (cargo-fuzz) for all parsers
- [ ] Property-based testing (proptest) for AST invariants
- [ ] MIRI testing for unsafe code (if any)
- [ ] Security audit of ZIP handling
- [ ] Accessibility: alt text warnings, heading structure validation

### Ecosystem
- [ ] Publish to crates.io
- [ ] WASM build target (lontar-core + lontar-md + lontar-html)
- [ ] `wasm-pack` npm package
- [ ] Python bindings via PyO3 (lontar-py)
- [ ] C FFI bindings (lontar-ffi)
- [ ] Integration example: Actix-web document generation endpoint
- [ ] Integration example: Axum document generation endpoint
- [ ] Integration example: Native doc-generator in CONSILAR

---

## Stretch Goals

- [ ] Force-directed layout for arbitrary network graphs
- [ ] Sequence diagram support (message passing between participants)
- [ ] Diagram reading/parsing (Mermaid → AST, Graphviz DOT → AST)
- [ ] Interactive diagrams in HTML (hover tooltips, click navigation)
- [ ] SVG standalone backend (lontar-svg)
- [ ] ODT (OpenDocument Text) backend
- [ ] EPUB backend
- [ ] RTF backend
- [ ] LaTeX backend
- [ ] Document diffing (compare two Documents)
- [ ] Document merging (combine multiple Documents)
- [ ] Document reading/parsing (docx → AST, not just AST → docx)
- [ ] Streaming write for very large documents
- [ ] Async I/O support
- [ ] Plugin system for custom backends
- [ ] Collaborative editing support (OT/CRDT)
- [ ] AI-assisted template generation (integrate with LLM APIs)

---

## Tracking

| Phase | Status | Started | Completed |
|---|---|---|---|
| Phase 0 | 🔴 Not Started | — | — |
| Phase 1 | 🔴 Not Started | — | — |
| Phase 1.5 | 🔴 Not Started | — | — |
| Phase 2 | 🔴 Not Started | — | — |
| Phase 3 | 🔴 Not Started | — | — |
| Phase 4 | 🔴 Not Started | — | — |
| Phase 5 | 🔴 Not Started | — | — |
| Phase 6 | 🔴 Not Started | — | — |
| Phase 7 | 🔴 Not Started | — | — |
| Phase 8 | 🔴 Not Started | — | — |
