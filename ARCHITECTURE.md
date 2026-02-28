# Architecture

## Overview

Lontar follows a **compiler-like architecture**: parse intent → build AST → lower to format-specific IR → serialize to bytes. This separation allows the core document model to remain format-agnostic while each backend handles the specifics of its target format.

```
                        ┌─────────────┐
                        │  User API   │
                        │  (Builder)  │
                        └──────┬──────┘
                               │
                               ▼
                     ┌───────────────────┐
                     │    lontar-core    │
                     │   Document AST    │
                     │  Styles, Layout   │
                     └────────┬──────────┘
                              │
              ┌───────┬───────┼───────┬───────┬───────┐
              ▼       ▼       ▼       ▼       ▼       ▼
           ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐
           │docx │ │pptx │ │ pdf │ │ md  │ │html │ │ txt │
           └──┬──┘ └──┬──┘ └──┬──┘ └──┬──┘ └──┬──┘ └──┬──┘
              │       │       │       │       │       │
              ▼       ▼       ▼       ▼       ▼       ▼
           .docx   .pptx    .pdf    .md     .html   .txt
```

## Core Document Model (`lontar-core`)

### Document AST

The AST is the heart of Lontar. Every format backend consumes this same tree.

```rust
/// Root document node
pub struct Document {
    pub metadata: DocumentMetadata,
    pub content: Vec<Block>,
    pub styles: StyleSheet,
    pub resources: ResourceStore,
}

pub struct DocumentMetadata {
    pub title: String,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub created: Option<DateTime<Utc>>,
    pub modified: Option<DateTime<Utc>>,
    pub language: Option<String>,
    pub custom: HashMap<String, String>,
}
```

### Block-Level Nodes

```rust
pub enum Block {
    /// Section heading (level 1-6)
    Heading {
        level: u8,
        content: Vec<Inline>,
        id: Option<String>,
        numbering: Option<NumberingStyle>,
    },

    /// Body paragraph
    Paragraph {
        content: Vec<Inline>,
        style: ParagraphStyle,
    },

    /// Data table
    Table {
        headers: Option<Vec<Cell>>,
        rows: Vec<Vec<Cell>>,
        style: TableStyle,
        caption: Option<Vec<Inline>>,
    },

    /// Chart (bar, line, pie, etc.)
    Chart {
        kind: ChartKind,
        data: ChartData,
        title: Option<String>,
        size: Option<Size>,
    },

    /// Embedded image
    Image {
        source: ImageSource,
        alt_text: Option<String>,
        layout: ImageLayout,
    },

    /// Fenced code block
    CodeBlock {
        language: Option<String>,
        content: String,
    },

    /// Block quote
    BlockQuote {
        content: Vec<Block>,
        attribution: Option<Vec<Inline>>,
    },

    /// Ordered or unordered list
    List {
        kind: ListKind,
        items: Vec<ListItem>,
    },

    /// Hard page break (docx/pdf) or slide break (pptx)
    PageBreak,

    /// Horizontal rule / separator
    Divider,

    /// Table of contents placeholder
    TableOfContents {
        depth: u8,
    },

    /// Header/footer definition
    HeaderFooter {
        kind: HeaderFooterKind,
        content: Vec<Block>,
    },

    /// Presentation slide (pptx-specific, but modeled in core)
    Slide {
        layout: SlideLayout,
        content: Vec<Block>,
        notes: Option<Vec<Block>>,
    },

    /// Raw format-specific passthrough (escape hatch)
    Raw {
        format: String,
        content: String,
    },
}
```

### Inline-Level Nodes

```rust
pub enum Inline {
    /// Styled text span
    Text {
        content: String,
        style: TextStyle,
    },

    /// Hyperlink
    Link {
        url: String,
        text: Vec<Inline>,
        tooltip: Option<String>,
    },

    /// Inline image
    InlineImage {
        source: ImageSource,
        alt_text: Option<String>,
    },

    /// Footnote reference
    FootnoteRef {
        id: String,
        content: Vec<Block>,
    },

    /// Line break within a paragraph
    LineBreak,

    /// Non-breaking space
    NonBreakingSpace,

    /// Tab character
    Tab,

    /// Math expression (LaTeX notation)
    Math {
        expression: String,
        display: bool, // inline vs block
    },

    /// Page/slide number placeholder
    PageNumber,

    /// Total page count placeholder
    PageCount,

    /// Current date placeholder
    CurrentDate {
        format: Option<String>,
    },
}
```

### Style System

Styles follow a cascading model similar to CSS, with specificity:
`default → named style → paragraph style → run (inline) style`

```rust
pub struct StyleSheet {
    pub default_text: TextStyle,
    pub default_paragraph: ParagraphStyle,
    pub named_styles: HashMap<String, NamedStyle>,
    pub page_setup: PageSetup,
}

pub struct TextStyle {
    pub font_family: Option<String>,
    pub font_size: Option<Pt>,           // in points
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<UnderlineStyle>,
    pub strikethrough: Option<bool>,
    pub color: Option<Color>,
    pub highlight: Option<Color>,
    pub superscript: Option<bool>,
    pub subscript: Option<bool>,
    pub small_caps: Option<bool>,
}

pub struct ParagraphStyle {
    pub alignment: Option<Alignment>,
    pub line_spacing: Option<Spacing>,
    pub space_before: Option<Pt>,
    pub space_after: Option<Pt>,
    pub indent_left: Option<Pt>,
    pub indent_right: Option<Pt>,
    pub indent_first_line: Option<Pt>,
    pub keep_together: Option<bool>,
    pub keep_with_next: Option<bool>,
    pub borders: Option<Borders>,
    pub background: Option<Color>,
}

pub struct PageSetup {
    pub size: PageSize,                  // A4, Letter, etc.
    pub orientation: Orientation,
    pub margins: Margins,
}
```

### Resource Management

Images and other binary resources are stored centrally and referenced by ID:

```rust
pub struct ResourceStore {
    resources: HashMap<ResourceId, Resource>,
}

pub struct Resource {
    pub id: ResourceId,
    pub data: Vec<u8>,
    pub content_type: String,
    pub filename: Option<String>,
}

pub enum ImageSource {
    /// Reference to a resource in the ResourceStore
    Resource(ResourceId),
    /// File path (resolved at write time)
    Path(PathBuf),
    /// URL (resolved at write time if network enabled)
    Url(String),
}
```

## Format Backends

### Backend Trait

Every format backend implements:

```rust
pub trait DocumentWriter {
    type Error: std::error::Error;
    type Options: Default;

    /// Write the document to a byte buffer
    fn write(
        doc: &Document,
        options: &Self::Options,
    ) -> Result<Vec<u8>, Self::Error>;

    /// Write the document to a file
    fn write_to_file(
        doc: &Document,
        path: impl AsRef<Path>,
        options: &Self::Options,
    ) -> Result<(), Self::Error> {
        let bytes = Self::write(doc, options)?;
        std::fs::write(path, bytes)?;
        Ok(())
    }
}
```

### DOCX Backend (`lontar-docx`)

Maps the AST to OOXML WordprocessingML (ECMA-376 Part 1).

**File structure emitted:**

```
output.docx (ZIP archive)
├── [Content_Types].xml
├── _rels/.rels
├── word/
│   ├── document.xml          ← main content
│   ├── styles.xml            ← style definitions
│   ├── numbering.xml         ← list numbering
│   ├── header1.xml           ← header content
│   ├── footer1.xml           ← footer content
│   ├── _rels/
│   │   └── document.xml.rels ← relationships
│   └── media/
│       └── image1.png        ← embedded images
└── docProps/
    ├── app.xml
    └── core.xml              ← metadata
```

**Key mapping decisions:**

| AST Node | OOXML Element |
|---|---|
| `Heading { level }` | `<w:p>` with `<w:pStyle w:val="HeadingN"/>` |
| `Paragraph` | `<w:p>` with `<w:r>` runs |
| `Table` | `<w:tbl>` with `<w:tr>` / `<w:tc>` |
| `Image` | `<w:drawing>` → `<wp:inline>` → `<a:blip>` |
| `TextStyle.bold` | `<w:rPr><w:b/></w:rPr>` |
| `PageBreak` | `<w:br w:type="page"/>` |

**Dependencies:** `zip`, `quick-xml`

### PPTX Backend (`lontar-pptx`)

Maps the AST to OOXML PresentationML (ECMA-376 Part 1).

**Core challenge:** PPTX is spatially positioned (EMU coordinates), not flow-based like DOCX. The backend must compute layout from the AST's logical structure.

**Layout strategy:**

```
Document with Slides
    │
    ├── Slide has a SlideLayout (title, content, two-column, etc.)
    │
    ├── Each layout defines placeholder regions (title, body, etc.)
    │
    └── Blocks are flowed into placeholders with auto-sizing
```

**File structure emitted:**

```
output.pptx (ZIP archive)
├── [Content_Types].xml
├── _rels/.rels
├── ppt/
│   ├── presentation.xml
│   ├── presProps.xml
│   ├── slideLayouts/
│   │   └── slideLayout1.xml
│   ├── slideMasters/
│   │   └── slideMaster1.xml
│   ├── slides/
│   │   ├── slide1.xml
│   │   └── slide2.xml
│   ├── theme/
│   │   └── theme1.xml
│   └── media/
│       └── chart1.xml
└── docProps/
    ├── app.xml
    └── core.xml
```

**Dependencies:** `zip`, `quick-xml`

### PDF Backend (`lontar-pdf`)

**Strategy decision (to be finalized in Phase 0):**

| Approach | Pros | Cons |
|---|---|---|
| **typst** integration | Beautiful output, Rust-native, actively developed | Heavy dependency, layout differences from docx |
| **printpdf** | Lightweight, direct PDF primitives | Very low-level, must build our own layout engine |
| **HTML → PDF** via headless browser | Leverage CSS layout | Requires browser runtime, adds external dependency |
| **genpdf** | Higher-level than printpdf | Less maintained, limited features |

**Recommended:** Start with typst integration. It's Rust-native, produces excellent output, and its markup language could even serve as an intermediate representation.

### Markdown Backend (`lontar-md`)

The simplest backend — primarily useful for:
- Validating the AST (if it can't round-trip to MD, the model is wrong)
- Generating documentation
- Lightweight text output when Office formats aren't needed

**Challenge:** Markdown has no standard for many features (charts, page breaks, complex tables). These emit HTML fallbacks or are skipped with warnings.

### HTML Backend (`lontar-html`)

Emits self-contained HTML with inline CSS. Can optionally emit separate CSS.

**Use cases:**
- Email-ready document output
- Web preview of documents
- Intermediate format for PDF generation

## Template System (`lontar-template`)

Templates allow declarative document creation from structured data:

```rust
let template = Template::from_file("quarterly_report.lontar")?;

let data = json!({
    "title": "Q3 Report",
    "author": "Dr. Sucandra",
    "findings": ["Finding 1", "Finding 2"],
    "metrics": [
        {"name": "Revenue", "value": "$1.7M"},
        {"name": "Users", "value": "22K"},
    ]
});

let doc = template.render(&data)?;
doc.write_docx("q3_report.docx")?;
```

**Template language (TBD):** Likely a subset of Tera or a custom DSL that maps naturally to the document AST.

## Cross-Cutting Concerns

### Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum LontarError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("XML serialization error: {0}")]
    Xml(#[from] quick_xml::Error),

    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("Invalid document: {0}")]
    Validation(String),

    #[error("Unsupported feature '{feature}' for format '{format}'")]
    UnsupportedFeature {
        feature: String,
        format: String,
    },

    #[error("Resource not found: {0}")]
    ResourceNotFound(ResourceId),

    #[error("Image processing error: {0}")]
    Image(String),
}
```

### Feature Degradation

Not all AST features map to all formats. The strategy:

1. **Lossless:** Feature is fully supported (e.g., bold text in all formats)
2. **Degraded:** Feature is approximated (e.g., chart → table in Markdown)
3. **Skipped:** Feature is omitted with a warning (e.g., slide transitions in PDF)

Backends report degradation through a `WriteReport`:

```rust
pub struct WriteReport {
    pub warnings: Vec<WriteWarning>,
    pub bytes_written: usize,
    pub features_degraded: Vec<String>,
    pub features_skipped: Vec<String>,
}
```

### Testing Strategy

```
tests/
├── fixtures/
│   ├── input/           ← sample Documents (built programmatically)
│   ├── expected_xml/    ← known-good XML fragments for OOXML
│   ├── expected_md/     ← known-good Markdown output
│   └── reference_docs/  ← docs generated by python-docx/pptx for comparison
├── unit/                ← per-crate unit tests
├── integration/         ← cross-crate integration tests
└── conformance/         ← open generated files in LibreOffice, validate
```

**Key testing approaches:**

- **XML snapshot testing:** Serialize AST to OOXML XML, compare against known-good snapshots
- **Round-trip testing:** AST → Markdown → parse → AST (verify structural equivalence)
- **Reference comparison:** Generate equivalent docs via Python libs, compare XML structure
- **Conformance testing:** Open in LibreOffice/MS Office, verify rendering (manual + screenshot diffing)

## Dependency Graph

```
lontar-core          (zero external deps beyond std)
    │
    ├── lontar-md    (zero external deps)
    ├── lontar-txt   (zero external deps)
    ├── lontar-html  (zero external deps)
    ├── lontar-docx  (zip, quick-xml)
    ├── lontar-pptx  (zip, quick-xml)
    ├── lontar-pdf   (typst or printpdf — TBD)
    └── lontar-template (tera or custom)

lontar (umbrella)    re-exports all backends via feature flags
lontar-cli           (clap, lontar)
```

## Performance Targets

For a typical 20-page report with tables and images:

| Format | Target | Notes |
|---|---|---|
| DOCX | < 50ms | XML templating + zip compression |
| PPTX | < 100ms | More complex XML, chart rendering |
| PDF | < 200ms | Layout computation is the bottleneck |
| MD | < 5ms | String concatenation |
| HTML | < 10ms | String concatenation + CSS |
| TXT | < 5ms | Simplest possible output |

These targets reflect native compilation and zero-copy where possible.

## Future Architecture Considerations

- **WASM target:** Core + MD/HTML backends should compile to WASM for browser-side document generation
- **Streaming writes:** For very large documents, write pages/slides incrementally instead of buffering the entire ZIP
- **Async support:** Optional async file I/O for server use cases
- **Plugin system:** Allow third-party format backends (e.g., ODT, RTF, EPUB)
