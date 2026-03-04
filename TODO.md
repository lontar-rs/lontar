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

### LaTeX Spec Analysis
- [x] Document LaTeX package dependencies for each AST feature
- [x] Map AST nodes to LaTeX commands/environments (create mapping table)
- [x] Document BibLaTeX citation commands and their mapping to `CitationMode`
- [x] Catalog common medical/scientific journal document classes (Lancet, BMJ, JAMA, NEJM, Elsevier, Springer Nature)
- [x] Document XeLaTeX/LuaLaTeX `fontspec` usage for multi-script text
- [x] Document TikZ primitives needed for diagram rendering
- [x] Create minimal compilable .tex for each AST feature (verify with XeLaTeX)
- [x] Document LaTeX special character escaping rules (& % $ # _ { } ~ ^ \)

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
- [x] For each reference doc, manually create expected output in MD, HTML, TXT, and LaTeX formats
- [x] Verify reference docs open correctly in LibreOffice and MS Office Online

### Reference Output Fixtures
- [x] Create `tests/fixtures/expected_md/` with expected Markdown output for each test document
- [x] Create `tests/fixtures/expected_html/` with expected HTML output for each test document
- [x] Create `tests/fixtures/expected_txt/` with expected plain text output for each test document
- [x] Create `tests/fixtures/expected_tex/` with expected LaTeX output for each test document
- [x] Create reference PDF output (via typst or LaTeX) for visual comparison
- [x] Define expected Markdown for: headings, styled text, tables, lists, images, code blocks, block quotes, links, horizontal rules
- [x] Define expected HTML for: headings, styled text, tables, lists, images, code blocks, block quotes, links (self-contained with inline CSS)
- [x] Define expected TXT for: headings (underline style), tables (ASCII art), lists (indented), word-wrapped paragraphs
- [x] Define expected LaTeX for: headings, styled text, tables (booktabs), lists, images (figure), code blocks (listings/minted), equations, citations, cross-references
- [x] Define expected PDF for: basic report, styled text, tables, images (visual reference only)

### Citation & Bibliography Test Fixtures
- [x] Create `tests/fixtures/bib/` directory
- [x] Create test `.bib` file with representative entries (article, book, inproceedings, thesis, report, webpage)
- [x] Create test CSL-JSON file with equivalent entries
- [x] Create test documents with citations in various modes (parenthetical, narrative, year-only)
- [x] Create expected citation rendering for each backend (LaTeX, DOCX, MD, HTML, TXT)
- [x] Create expected bibliography rendering for each style (numeric, author-year, Vancouver, superscript)
- [x] Create test document with cross-references (figure refs, table refs, equation refs, section refs)

### Multi-Script Test Corpus
- [x] Create `tests/fixtures/scripts/` directory for script-specific test data
- [x] Create test strings file with representative text for each script category:
  - [x] Simple LTR: Latin, Cyrillic, Greek, Georgian
  - [x] Complex Indic: Devanagari (conjuncts, split matras), Bengali, Tamil, Telugu, Kannada, Malayalam
  - [x] Southeast Asian: Balinese (ᬮᭀᬦ᭄ᬢᬭ᭄), Javanese, Sundanese, Batak, Thai, Lao, Khmer, Myanmar
  - [x] RTL: Arabic (contextual joining), Hebrew, Syriac, Thaana, N'Ko
  - [x] CJK: Simplified Chinese, Traditional Chinese, Japanese (Kanji + Hiragana + Katakana), Korean (Hangul)
  - [x] Tibetan: stacking consonants
  - [x] African: Ethiopic, Tifinagh
  - [x] Mixed-script paragraphs: Latin + Balinese, Arabic + English, CJK + Latin
- [x] Collect test fonts: Noto Sans families for each script category
- [x] Generate reference documents (via python-docx) containing multi-script text — verify they open correctly
- [x] Generate reference presentations (via python-pptx) containing multi-script text — verify they open correctly
- [x] Generate reference LaTeX documents containing multi-script text — verify they compile with XeLaTeX
- [x] Document expected shaping behavior for each script (which conjuncts should form, which reorderings should occur)

### Tooling Setup
- [x] Initialize Cargo workspace
- [x] Set up CI (GitHub Actions): clippy, fmt, test, miri
- [x] Configure code coverage (cargo-llvm-cov)
- [x] Set up benchmarks (criterion)
- [x] Create issue templates for bug reports and feature requests
- [x] Set up dependabot for dependency updates

### Text Shaping Research
- [x] Evaluate `rustybuzz` API and capabilities
- [x] Evaluate `unicode-bidi` for bidirectional text
- [x] Evaluate `unicode-linebreak` for line break opportunities
- [x] Evaluate `unicode-script` for script run detection
- [x] Research font subsetting approaches in Rust
- [x] Collect test fonts: Noto Sans Balinese, Noto Sans Devanagari, Noto Sans Arabic, Noto Sans CJK
- [x] Create test strings for each script category (simple LTR, complex Indic, SE Asian, RTL, CJK)
- [x] Document how DOCX/PPTX handle font embedding and language tagging
- [x] Document how PDF embeds fonts and stores positioned glyphs
- [x] Document how XeLaTeX/LuaLaTeX handle font selection and shaping (fontspec + HarfBuzz)
- [x] Prototype: shape "ᬮᭀᬦ᭄ᬢᬭ᭄" (lontar in Aksara Bali) with rustybuzz → verify conjunct formation

### Bibliography Research
- [x] Evaluate BibTeX parsing crates in Rust (e.g., `biblatex`, `nom-bibtex`)
- [x] Evaluate CSL-JSON parsing approach (serde deserialization)
- [x] Document BibLaTeX entry types and required/optional fields
- [x] Document Vancouver citation style rules (critical for medical journals)
- [x] Document APA 7th edition citation style rules
- [x] Survey journal submission systems: which accept XeLaTeX? which require pdfLaTeX?
- [x] Research CSL (Citation Style Language) processor options in Rust

---

## Phase 1 — Core Document Model + Simple Backends

**Goal:** Prove the unified AST works with the simplest possible outputs.

### lontar-core
- [x] Define `Block` enum with all variants (including `Equation`, `Bibliography`)
- [x] Define `Inline` enum with all variants (including `Citation`, `CrossRef`)
- [x] Define `TextStyle` struct
- [x] Define `ParagraphStyle` struct
- [x] Define `TableStyle` struct
- [x] Define `ChartKind` and `ChartData` types
- [x] Define `PageSetup` and `Margins`
- [x] Define `StyleSheet` with cascading resolution
- [x] Define `DocumentMetadata`
- [x] Define `ResourceStore` for images and binary assets
- [x] Define `BibliographyStore` with `BibEntry` and `BibAuthor`
- [x] Define `BibEntryKind` enum
- [x] Implement `BibliographyStore::load_bibtex()` — parse .bib files
- [x] Implement `BibliographyStore::load_csl_json()` — parse CSL-JSON
- [x] Implement citation resolution: given citation keys + style → rendered text
- [x] Implement cross-reference resolution: given label → computed number/text
- [x] Implement label uniqueness validation in `DocumentBuilder`
- [x] Implement `Document` struct with root node (including `bibliography` field)
- [x] Implement `DocumentBuilder` (ergonomic builder API, with `.cite()`, `.crossref()`, `.bib_entry()`)
- [x] Implement `DocumentWriter` trait
- [x] Implement `WriteReport` for feature degradation reporting
- [x] Implement `LontarError` error types (including `CitationNotFound`, `Bibliography`)
- [x] Implement style cascade resolution (default → named → paragraph → run)
- [x] Write unit tests for AST construction
- [x] Write unit tests for style resolution
- [x] Write unit tests for citation resolution (all modes × all styles)
- [x] Write unit tests for cross-reference resolution
- [x] Write unit tests for BibTeX parsing
- [x] Write documentation for all public types

### lontar-md (Markdown Backend)
- [x] Implement `MarkdownWriter` for `DocumentWriter` trait
- [x] Handle: headings, paragraphs, bold/italic/code text
- [x] Handle: tables (GitHub-flavored Markdown)
- [x] Handle: ordered and unordered lists
- [x] Handle: images (as `![alt](path)`)
- [x] Handle: links
- [x] Handle: code blocks with language
- [x] Handle: block quotes
- [x] Handle: horizontal rules
- [x] Handle: citations (render as inline text per bibliography style)
- [x] Handle: bibliography (render as formatted list)
- [x] Handle: cross-references (render as `[Type N](#label)`)
- [x] Handle: math (render as `$...$` / `$$...$$`)
- [x] Graceful degradation: charts → table fallback
- [x] Graceful degradation: page breaks → `---`
- [ ] Graceful degradation: headers/footers → skip with warning
- [ ] Write snapshot tests against expected Markdown output
- [ ] Verify round-trip: AST → MD → parse with pulldown-cmark → compare structure

### lontar-txt (Plain Text Backend)
- [ ] Implement `PlainTextWriter` for `DocumentWriter` trait
- [ ] Handle: headings (underlined with = or -)
- [ ] Handle: paragraphs with word wrapping
- [ ] Handle: tables (ASCII art)
- [ ] Handle: lists (with indentation)
- [ ] Handle: citations (render as inline text per bibliography style)
- [ ] Handle: bibliography (render as numbered/formatted list)
- [ ] Handle: cross-references (render as "(see Type N)")
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

### Citations & Cross-References (DOCX)
- [ ] Citation rendering as inline formatted text
- [ ] Citation rendering as DOCX field codes (for Zotero/Mendeley compatibility)
- [ ] Bibliography rendering with hanging indent paragraph style
- [ ] Cross-reference bookmarks on labelled blocks
- [ ] Cross-reference `REF` field codes for headings
- [ ] Cross-reference `SEQ` field codes for figures, tables, equations
- [ ] Auto-numbering for figure captions, table captions

### Advanced Features
- [ ] Table of contents (field codes)
- [ ] Footnotes (`word/footnotes.xml`)
- [ ] Block quotes (styled paragraphs)
- [ ] Code blocks (monospaced styled paragraphs)
- [ ] Horizontal rules
- [ ] Display equations (OMML or MathML)

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
- [ ] Test citation rendering: verify numbering, ordering, and bibliography formatting
- [ ] Test cross-references: verify computed numbers match actual positions
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

### Citations (PPTX)
- [ ] Citation rendering as inline text in text frames
- [ ] Reference slide generation for bibliography
- [ ] Simplified citation display (space-constrained context)

### Testing
- [ ] Open in LibreOffice Impress
- [ ] Open in MS PowerPoint Online
- [ ] Compare against python-pptx reference
- [ ] Test slide count limits
- [ ] Performance benchmarks

---

## Phase 4 — PDF + HTML + LaTeX + XLSX Backends

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
- [ ] Citations (inline rendered text with hyperlinks to bibliography)
- [ ] Bibliography (formatted list with anchor IDs)
- [ ] Cross-references (internal hyperlinks with computed numbers)
- [ ] Display equations
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
- [ ] Citations (anchor links to bibliography section)
- [ ] Bibliography (ordered list with anchor IDs and hanging indent CSS)
- [ ] Cross-references (internal `<a href="#label">` links)
- [ ] Math rendering (MathML or KaTeX integration)

### lontar-latex
- [ ] Implement `LatexWriter` for `DocumentWriter` trait
- [ ] Implement preamble generator (analyze AST features → `\usepackage` declarations)
- [ ] Implement LaTeX special character escaping (`& % $ # _ { } ~ ^ \`)
- [ ] Implement `LatexOptions` (document class, class options, engine, code package, etc.)
- [ ] Handle: headings → `\section`, `\subsection`, `\subsubsection`, `\paragraph`
- [ ] Handle: paragraphs → text blocks with `\par` separation
- [ ] Handle: bold, italic, underline → `\textbf`, `\textit`, `\underline`
- [ ] Handle: font size → `\small`, `\large`, etc. or explicit `\fontsize`
- [ ] Handle: text color → `\textcolor` (requires `xcolor` package)
- [ ] Handle: tables → `\begin{table}` + `booktabs` formatting
- [ ] Handle: table captions and labels
- [ ] Handle: images → `\begin{figure}` + `\includegraphics`
- [ ] Handle: figure captions and labels
- [ ] Handle: code blocks → `\begin{lstlisting}` or `\begin{minted}`
- [ ] Handle: ordered lists → `\begin{enumerate}`
- [ ] Handle: unordered lists → `\begin{itemize}`
- [ ] Handle: nested lists
- [ ] Handle: block quotes → `\begin{quote}`
- [ ] Handle: hyperlinks → `\href{url}{text}` (requires `hyperref`)
- [ ] Handle: inline math → `$...$`
- [ ] Handle: display math → `\begin{equation}` with optional `\label`
- [ ] Handle: citations → `\parencite`, `\textcite`, `\cite` per `CitationMode`
- [ ] Handle: bibliography → `\printbibliography` + emit `.bib` file
- [ ] Handle: cross-references → `\ref`, `\autoref`, or `\cref` per option
- [ ] Handle: table of contents → `\tableofcontents`
- [ ] Handle: page breaks → `\newpage`
- [ ] Handle: horizontal rules → `\hrulefill` or `\rule`
- [ ] Handle: footnotes → `\footnote{...}`
- [ ] Handle: headers/footers → `\fancyhf` (requires `fancyhdr`)
- [ ] Handle: page numbers → `\thepage` in header/footer
- [ ] Handle: diagrams → delegate to lontar-diagram TikZ renderer
- [ ] Emit `.bib` file alongside `.tex` when bibliography is present
- [ ] Emit image files alongside `.tex` (or embed as data URIs for single-file mode)
- [ ] Graceful degradation: slides → `\section` divisions
- [ ] Graceful degradation: charts → tables (or TikZ/pgfplots if available)
- [ ] Write snapshot tests against expected LaTeX output
- [ ] Verify generated .tex compiles with XeLaTeX without errors
- [ ] Verify generated .tex compiles with LuaLaTeX without errors
- [ ] Test with multi-script content (Latin + Balinese + Arabic + CJK)
- [ ] Test with complex citation scenarios (multiple keys, prefix/suffix, all modes)
- [ ] Test with cross-references (verify `\label`/`\ref` pairs resolve correctly)
- [ ] Test preamble minimality (unused packages not included)
- [ ] Performance benchmarks

### Journal Template Testing
- [ ] Create test template for a generic medical journal (IMRAD structure)
- [ ] Generate LaTeX with medical journal template → verify compilation
- [ ] Generate DOCX with same data → verify formatting
- [ ] Test citation rendering in Vancouver style (standard for medical journals)
- [ ] Test citation rendering in Author-Year style (standard for social sciences)
- [ ] Test with real-world .bib file (50+ entries)

### lontar-xlsx
- [ ] Implement `XlsxWriter` as thin wrapper around `rust_xlsxwriter`
- [ ] Map `Block::Table` → worksheet with header row + data rows
- [ ] Map `Block::Heading { level: 1 }` → worksheet name
- [ ] Map `Block::Chart` → rust_xlsxwriter chart object
- [ ] Map `Block::Image` → embedded worksheet image
- [ ] Map `Block::Paragraph` → cell content (merged across columns)
- [ ] Graceful degradation for unsupported blocks (citations → skip, diagrams → skip with warning)
- [ ] Style mapping: bold, italic, font size, color → cell formatting
- [ ] Write snapshot tests
- [ ] Performance benchmarks

---

## Phase 5 — Templates + CLI

### lontar-template
- [ ] Define template format (Tera-based or custom DSL)
- [ ] Template parsing and validation
- [ ] Variable substitution in text
- [ ] Conditional sections (`{% if ... %}`)
- [ ] Loop sections (`{% for ... %}`)
- [ ] Template inheritance / includes
- [ ] Bibliography integration: `render_with_bibliography()` API
- [ ] Built-in templates:
  - [ ] Generic report
  - [ ] Memo
  - [ ] Presentation
  - [ ] Invoice
  - [ ] Research article (IMRAD)
  - [ ] Systematic review (PRISMA-compliant)
  - [ ] Case report
  - [ ] Conference abstract
  - [ ] Thesis chapter
  - [ ] Grant proposal
- [ ] Journal-specific LaTeX templates:
  - [ ] Elsevier article (`elsarticle`)
  - [ ] Springer Nature (`sn-jnl`)
  - [ ] IEEE (`IEEEtran`)
  - [ ] PLOS ONE
  - [ ] BMJ / Lancet / JAMA / NEJM style guides
- [ ] Template documentation and examples

### lontar-cli
- [ ] `lontar convert input.md --to docx,pptx,pdf,tex`
- [ ] `lontar template report.lontar --data data.json --bib refs.bib --to docx,tex`
- [ ] `lontar inspect file.docx` (show document structure)
- [ ] `lontar validate file.docx` (check OOXML conformance)
- [ ] `lontar bib refs.bib --check` (validate bibliography entries)
- [ ] Shell completions (bash, zsh, fish)
- [ ] Man page generation

---

## Phase 6 — Polish & Ecosystem

### Documentation
- [ ] API documentation (rustdoc) for all public items
- [ ] User guide (mdbook)
- [ ] Tutorial: "Generate your first document"
- [ ] Tutorial: "Create a custom template"
- [ ] Tutorial: "Academic writing workflow: Zotero → Lontar → journal submission"
- [ ] Tutorial: "Multi-format manuscript: write once, submit everywhere"
- [ ] Example gallery with source code
- [ ] Migration guide from python-docx

### Quality
- [ ] Fuzz testing (cargo-fuzz) for all parsers (including BibTeX parser)
- [ ] Property-based testing (proptest) for AST invariants
- [ ] Property-based testing for citation resolution (any valid bib + citation combo produces valid output)
- [ ] MIRI testing for unsafe code (if any)
- [ ] Security audit of ZIP handling
- [ ] Accessibility: alt text warnings, heading structure validation

### Ecosystem
- [ ] Publish to crates.io
- [ ] WASM build target (lontar-core + lontar-md + lontar-html + lontar-latex)
- [ ] `wasm-pack` npm package
- [ ] Python bindings via PyO3 (lontar-py)
- [ ] C FFI bindings (lontar-ffi)
- [ ] Integration example: Actix-web document generation endpoint
- [ ] Integration example: Axum document generation endpoint
- [ ] Integration example: Academic manuscript pipeline (Zotero export → lontar-cli → multi-format output)
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
- [ ] CSL (Citation Style Language) processor — full CSL spec for precise journal formatting
- [ ] Document diffing (compare two Documents)
- [ ] Document merging (combine multiple Documents)
- [ ] Document reading/parsing (docx → AST, not just AST → docx)
- [ ] BibTeX reading from DOCX (extract Zotero/Mendeley field codes → BibliographyStore)
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