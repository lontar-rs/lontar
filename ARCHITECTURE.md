# Architecture

## Overview

Lontar follows a **compiler-like architecture**: parse intent → build AST → shape text → lower to format-specific IR → serialize to bytes. This separation allows the core document model to remain format-agnostic while each backend handles the specifics of its target format.

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
                              ▼
                     ┌───────────────────┐
                     │   lontar-aksara   │
                     │  Text Shaping     │
                     │  BiDi, Linebreak  │
                     │  Font Management  │
                     └────────┬──────────┘
                              │
        ┌───────┬───────┬─────┼─────┬───────┬───────┬───────┬───────┐
        ▼       ▼       ▼     ▼     ▼       ▼       ▼       ▼       ▼
     ┌─────┐ ┌─────┐ ┌─────┐ ┌───┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐
     │docx │ │pptx │ │ pdf │ │tex│ │ md  │ │html │ │ txt │ │xlsx │ │diag │
     └──┬──┘ └──┬──┘ └──┬──┘ └─┬─┘ └──┬──┘ └──┬──┘ └──┬──┘ └──┬──┘ └──┬──┘
        │       │       │      │      │       │       │       │       │
        ▼       ▼       ▼      ▼      ▼       ▼       ▼       ▼       ▼
     .docx   .pptx    .pdf   .tex   .md     .html   .txt   .xlsx   (lib)
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
    pub bibliography: BibliographyStore,
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
        label: Option<String>,
    },

    /// Chart (bar, line, pie, etc.)
    Chart {
        kind: ChartKind,
        data: ChartData,
        title: Option<String>,
        size: Option<Size>,
        label: Option<String>,
    },

    /// Embedded image
    Image {
        source: ImageSource,
        alt_text: Option<String>,
        layout: ImageLayout,
        caption: Option<Vec<Inline>>,
        label: Option<String>,
    },

    /// Fenced code block
    CodeBlock {
        language: Option<String>,
        content: String,
        caption: Option<Vec<Inline>>,
        label: Option<String>,
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

    /// Structured diagram with automatic layout
    Diagram {
        kind: DiagramKind,
        nodes: Vec<DiagramNode>,
        edges: Vec<DiagramEdge>,
        layout: DiagramLayout,
        style: DiagramStyle,
        caption: Option<Vec<Inline>>,
        label: Option<String>,
    },

    /// Math equation (display mode)
    Equation {
        expression: String,
        label: Option<String>,
    },

    /// Rendered bibliography / reference list
    Bibliography {
        style: BibliographyStyle,
    },

    /// Raw format-specific passthrough (escape hatch)
    Raw {
        format: String,
        content: String,
    },
}
```

### Diagram Types

```rust
pub enum DiagramKind {
    /// Directed flow with process/decision nodes
    Flowchart,
    /// Message passing between participants over time
    Sequence,
    /// Hierarchical parent-child layout
    Tree,
    /// Undirected or directed graph with flexible layout
    Network,
    /// Radial hierarchical layout
    MindMap,
    /// User-defined layout with explicit coordinates
    Custom,
}

pub struct DiagramNode {
    pub id: String,
    pub label: Vec<Inline>,
    pub shape: NodeShape,
    pub style: DiagramNodeStyle,
}

pub struct DiagramEdge {
    pub from: String,
    pub to: String,
    pub label: Option<Vec<Inline>>,
    pub kind: EdgeKind,
    pub style: DiagramEdgeStyle,
}

pub enum NodeShape {
    Rectangle,
    RoundedRect,
    Circle,
    Diamond,
    Parallelogram,
    Cylinder,
    Stadium,
}

pub enum EdgeKind {
    Solid,
    Dashed,
    Dotted,
    Arrow,
    BiDirectional,
}

pub enum DiagramLayout {
    /// Automatic layout using graph algorithms
    Auto,
    /// Left-to-right hierarchical
    LeftToRight,
    /// Top-to-bottom hierarchical
    TopToBottom,
    /// Force-directed (spring model)
    ForceDirected,
    /// Manual coordinates (user-specified positions)
    Manual,
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

    /// Citation reference (resolves from BibliographyStore)
    Citation {
        keys: Vec<String>,
        prefix: Option<String>,
        suffix: Option<String>,
        mode: CitationMode,
    },

    /// Cross-reference to a labelled element (figure, table, equation, section)
    CrossRef {
        target: String,
        kind: CrossRefKind,
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

/// How the citation is rendered in text
pub enum CitationMode {
    /// Parenthetical: (Author, Year) or [1]
    Parenthetical,
    /// Narrative: Author (Year) or Author [1]
    Narrative,
    /// Year only: (Year) — when author is already in prose
    YearOnly,
    /// Suppress author: (Year) — explicit variant for styles that distinguish
    SuppressAuthor,
    /// Full: render all authors even if previously abbreviated
    Full,
}

/// What aspect of the target to display in a cross-reference
pub enum CrossRefKind {
    /// "Figure 3", "Table 2", etc. — auto-detect from target type
    Auto,
    /// Just the number: "3"
    Number,
    /// The page: "page 12"
    Page,
    /// The target's title or caption text
    Title,
}
```

### Bibliography System

The bibliography system stores reference metadata centrally, parallel to how
`ResourceStore` handles binary assets. Backends resolve citations against
this store and render according to their format's conventions.

```rust
pub struct BibliographyStore {
    entries: HashMap<String, BibEntry>,
}

pub struct BibEntry {
    pub key: String,
    pub kind: BibEntryKind,
    pub title: String,
    pub authors: Vec<BibAuthor>,
    pub year: Option<String>,
    pub journal: Option<String>,
    pub volume: Option<String>,
    pub issue: Option<String>,
    pub pages: Option<String>,
    pub publisher: Option<String>,
    pub doi: Option<String>,
    pub url: Option<String>,
    pub isbn: Option<String>,
    pub abstract_text: Option<String>,
    pub custom: HashMap<String, String>,
}

pub struct BibAuthor {
    pub given: String,
    pub family: String,
    pub suffix: Option<String>,
}

pub enum BibEntryKind {
    Article,
    Book,
    InProceedings,
    InCollection,
    Thesis,
    Report,
    Patent,
    WebPage,
    Software,
    Dataset,
    Other(String),
}

pub enum BibliographyStyle {
    /// Numeric: [1], [2], [3]
    Numeric,
    /// Author-Year: (Smith, 2024)
    AuthorYear,
    /// Superscript numeric: ¹, ², ³ — common in medical journals
    Superscript,
    /// Vancouver style — numbered, ordered by first citation (common in medicine)
    Vancouver,
    /// APA 7th edition
    Apa7,
    /// Named style (resolved from a CSL file or built-in)
    Named(String),
}
```

**Backend rendering:**

| Backend | Citation Rendering | Bibliography Rendering |
|---|---|---|
| **LaTeX** | `\cite{key}`, `\parencite{key}`, `\textcite{key}` + `.bib` file | `\printbibliography` (BibLaTeX) or `\bibliography{}` (BibTeX) |
| **DOCX** | Field codes or inline text | Formatted reference list as paragraphs with hanging indent |
| **PPTX** | Inline text (simplified) | Reference slide |
| **PDF** | Rendered inline text with hyperlinks | Formatted reference list |
| **HTML** | `<a>` with tooltip, linked to `#ref-key` anchor | `<ol>` or `<dl>` with anchor IDs |
| **Markdown** | `[Author, Year]` or `[1]` with link | Formatted list at end of document |
| **TXT** | `(Author, Year)` or `[1]` | Numbered/formatted list at end |

**BibTeX/BibLaTeX import:**

```rust
impl BibliographyStore {
    /// Parse a .bib file and add all entries
    pub fn load_bibtex(&mut self, bibtex_source: &str) -> Result<(), BibParseError>;

    /// Parse CSL-JSON (e.g., from Zotero export)
    pub fn load_csl_json(&mut self, json: &str) -> Result<(), BibParseError>;
}
```

This means a medical researcher can export from Zotero/Mendeley/EndNote as `.bib` or CSL-JSON, load it into the document, and get correct citations in every output format.

### Cross-Reference System

The `label` field on blocks (`Table`, `Image`, `Equation`, `Diagram`, `CodeBlock`)
and the `id` field on `Heading` form the cross-reference namespace. Backends resolve
`Inline::CrossRef { target }` by looking up matching labels/IDs and rendering
format-appropriate references:

| Backend | Cross-Reference Rendering |
|---|---|
| **LaTeX** | `\ref{target}`, `\autoref{target}`, `\cref{target}` |
| **DOCX** | `SEQ` field codes for figure/table numbers, `REF` bookmarks for headings |
| **PDF** | Internal hyperlinks with computed numbers |
| **HTML** | `<a href="#target">` with computed text |
| **Markdown** | `[Figure N](#target)` |
| **TXT** | `(see Figure N)` |

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

## Text Shaping Pipeline (`lontar-aksara`)

The text shaping pipeline is what makes Lontar's universal script support possible. Rather than handling individual scripts with custom code, Lontar integrates proven, standards-compliant components that handle all 159+ Unicode scripts correctly.

### Why This Matters

Document generation libraries typically store text as plain Unicode strings and rely on the rendering application (Word, LibreOffice, a PDF viewer) to handle shaping. This works for DOCX and PPTX where the application does the heavy lifting, but breaks for PDF where the generator is responsible for glyph selection and positioning. By building a shaping pipeline into the foundation, Lontar produces correct output across all formats and all scripts.

### Pipeline Components

```
Unicode Text + Font
        │
        ▼
┌──────────────────┐
│   Script Detection │  ← Identify script runs (Latin, Arabic, Balinese, etc.)
│   (unicode-script) │
└────────┬───────────┘
         │
         ▼
┌──────────────────┐
│  BiDi Resolution  │  ← Resolve bidirectional text (LTR/RTL mixing)
│  (unicode-bidi)   │
└────────┬──────────┘
         │
         ▼
┌──────────────────┐
│   Text Shaping    │  ← Map characters → glyphs with positioning
│   (rustybuzz)     │     Handles: conjuncts, ligatures, reordering,
└────────┬──────────┘     contextual forms, mark positioning
         │
         ▼
┌──────────────────┐
│  Line Breaking    │  ← Script-aware line break opportunities
│ (unicode-linebreak)│
└────────┬──────────┘
         │
         ▼
┌──────────────────┐
│  Font Management  │  ← Font loading, fallback chains, subsetting
│  (subsetting TBD) │
└────────┬──────────┘
         │
         ▼
   Shaped Glyphs + Positions
   (consumed by format backends)
```

### Shaping Detail

`rustybuzz` (Rust port of HarfBuzz) is the core of the pipeline. It handles:

- **Conjunct formation** — e.g., Aksara Bali consonant clusters where ᬦ + ᭄ + ᬢ forms a stacked conjunct
- **Vowel sign reordering** — e.g., Balinese TALING (ᬾ) visually precedes the consonant it logically follows
- **Contextual shaping** — e.g., Arabic letters taking initial/medial/final/isolated forms
- **Ligatures** — e.g., Latin fi/fl ligatures, Arabic lam-alef
- **Mark positioning** — e.g., combining diacritics, vowel signs above/below consonants

### Backend Integration

Each backend uses the text pipeline differently:

| Backend | Shaping Role |
|---|---|
| **DOCX/PPTX** | Font embedding and run-level language tagging. The application handles shaping, but correct font embedding ensures glyphs are available. |
| **PDF** | Full shaping required. The pipeline produces positioned glyphs that are written directly into the PDF content stream. |
| **HTML** | Font-face declarations with proper unicode-range. Browser handles shaping. |
| **LaTeX** | Pass-through for text; XeLaTeX/LuaLaTeX handle shaping via HarfBuzz. Font declarations emitted in preamble. |
| **MD/TXT** | Pass-through. Plain Unicode text; the viewer handles rendering. |

### Font Management

```rust
pub struct FontManager {
    /// Registered font families with style variants
    families: HashMap<String, FontFamily>,
    /// Fallback chain for script coverage
    fallback_chain: Vec<FontFamily>,
    /// System font discovery (optional)
    system_fonts: Option<SystemFontIndex>,
}

pub struct FontFamily {
    pub name: String,
    pub regular: Option<FontData>,
    pub bold: Option<FontData>,
    pub italic: Option<FontData>,
    pub bold_italic: Option<FontData>,
    /// Scripts this font covers (auto-detected from cmap table)
    pub script_coverage: HashSet<Script>,
}

pub struct FontData {
    pub data: Vec<u8>,
    /// Parsed face for shaping queries
    pub face: rustybuzz::Face,
}
```

**Font fallback strategy:** When a text run contains characters not covered by the specified font, the font manager walks the fallback chain to find a font that covers the required script. This ensures mixed-script documents (e.g., Latin + Balinese + Arabic in the same paragraph) render correctly.

**Font subsetting:** For PDF and DOCX embedding, only the glyphs actually used in the document are included, reducing file size. This is particularly important for CJK fonts which can be 10MB+ for a full set.

### Supported Script Categories

With `rustybuzz` + `unicode-bidi` + `unicode-linebreak`, Lontar supports:

| Category | Scripts | Key Challenges |
|---|---|---|
| **Simple LTR** | Latin, Cyrillic, Greek, Georgian, Armenian... | Ligatures, kerning |
| **Complex Indic** | Devanagari, Bengali, Tamil, Telugu, Kannada, Malayalam, Gujarati, Gurmukhi, Oriya, Sinhala... | Conjuncts, vowel reordering, split matras |
| **Southeast Asian** | Thai, Lao, Khmer, Myanmar, Balinese, Javanese, Sundanese, Batak, Bugis, Cham... | Conjuncts, above/below marks, no word spaces |
| **RTL** | Arabic, Hebrew, Syriac, Thaana, N'Ko... | Bidirectional layout, contextual joining |
| **CJK** | Han, Hiragana, Katakana, Hangul, Bopomofo | Large glyph sets, vertical text (future) |
| **Tibetan** | Tibetan, Phags-Pa | Complex stacking |
| **African** | Ethiopic, Tifinagh, Vai, Bamum... | Syllabic systems |
| **Historic** | Kawi, Old Uyghur, Mongolian, Brahmi... | Rare fonts, limited shaping data |

### Module Structure

```
lontar-aksara/
├── src/
│   ├── lib.rs
│   ├── shaping.rs       # rustybuzz integration, shaped run output
│   ├── bidi.rs           # Bidirectional text resolution
│   ├── linebreak.rs      # Script-aware line breaking
│   ├── script.rs         # Script detection and run segmentation
│   ├── font/
│   │   ├── mod.rs
│   │   ├── manager.rs    # Font loading, fallback chains
│   │   ├── subset.rs     # Glyph subsetting for embedding
│   │   └── system.rs     # System font discovery (optional)
│   └── lang.rs           # Language tagging (BCP 47)
```

## Diagram Engine (`lontar-diagram`)

Lontar includes a native diagramming engine that defines diagrams as structured data (nodes + edges), computes layout automatically, and renders using each format's native primitives.

### Pipeline

```
Diagram DSL (nodes + edges + kind)
        │
        ▼
┌──────────────────┐
│  Layout Engine   │  ← Graph layout algorithms
│  (petgraph +     │     Computes positions for nodes
│   layered/force) │     and path points for edges
└────────┬─────────┘
         │
         ▼
   PositionedDiagram
   (nodes with x, y, width, height;
    edges with path control points)
         │
    ┌────┼────┬────┬────┬────┬────┐
    ▼    ▼    ▼    ▼    ▼    ▼    ▼
  DOCX  PPTX  PDF  LaTeX HTML  MD  TXT
```

### Format-Native Rendering

| Backend | Renderer | Output |
|---|---|---|
| **DOCX** | DrawingML shapes | `<wsp:wsp>` with flowchart presets, connectors. Native vector, editable in Word. |
| **PPTX** | DrawingML shapes | Same primitives, positioned in EMU. Editable in PowerPoint. |
| **PDF** | Vector paths | Bezier curves, fills, strokes. Scalable, resolution-independent. |
| **LaTeX** | TikZ code | `\begin{tikzpicture}` with nodes and edges. Native vector, editable. |
| **HTML** | Inline SVG | `<svg>` with `<rect>`, `<path>`, `<text>`. Scalable, potentially interactive. |
| **SVG** | Direct SVG | Standalone `.svg` file output (future backend). |
| **MD** | Mermaid code block | ` ```mermaid ` fenced block. Renders on GitHub, GitLab, and Mermaid-aware viewers. |
| **TXT** | ASCII art | Box-drawing characters and arrows via layout algorithm. |

### Layout Algorithms

| Algorithm | Best For | Approach |
|---|---|---|
| **Layered/Sugiyama** | Flowcharts, DAGs | Assigns nodes to layers, minimizes edge crossings |
| **Force-directed** | Network graphs | Simulates physical forces (spring model) until equilibrium |
| **Tree layout** | Hierarchies, org charts | Tidier tree drawing (Reingold-Tilford) |
| **Manual** | Precise control | User provides explicit coordinates |

Dependencies (to be evaluated in Phase 0): `petgraph` for graph data structures, layout algorithms TBD (custom implementation, `layout-rs`, or port of Graphviz subset).

### Module Structure

```
lontar-diagram/
├── src/
│   ├── lib.rs
│   ├── model.rs          # DiagramKind, Node, Edge, Style types
│   ├── layout/
│   │   ├── mod.rs
│   │   ├── layered.rs    # Sugiyama algorithm for flowcharts/DAGs
│   │   ├── force.rs      # Force-directed layout
│   │   └── tree.rs       # Tree layout (Reingold-Tilford)
│   └── render/
│       ├── mod.rs
│       ├── drawingml.rs  # DOCX/PPTX DrawingML shapes
│       ├── svg.rs        # PDF/HTML/standalone SVG
│       ├── tikz.rs       # LaTeX TikZ diagrams
│       ├── mermaid.rs    # Markdown Mermaid code blocks
│       └── ascii.rs      # Plain text ASCII art
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
| `Citation` | Field code `ADDIN ZOTERO_ITEM` or inline text with bookmark |
| `Bibliography` | Formatted paragraphs with hanging indent style |
| `CrossRef` | `REF` bookmark field or `SEQ` field |

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

### LaTeX Backend (`lontar-latex`)

Emits compilable LaTeX source files. Unlike most backends, LaTeX output is a *source format* — it requires a TeX engine (XeLaTeX or LuaLaTeX) to produce the final PDF. This is by design: the LaTeX ecosystem's typographic quality, citation management, and journal template system are the point.

**Output structure:**

For a self-contained document:
```
output.tex              ← main LaTeX source
output.bib              ← BibTeX/BibLaTeX bibliography (if citations present)
media/
├── image1.png          ← referenced images (copied alongside)
└── image2.pdf
```

**Key design decisions:**

1. **XeLaTeX/LuaLaTeX assumed.** These engines support Unicode natively and use system fonts via `fontspec`. We don't target pdfLaTeX — its 8-bit encoding model is incompatible with universal script support.

2. **Preamble is computed, not templated.** The backend analyzes which AST features are used and emits only the required `\usepackage` declarations. A document with no math doesn't load `amsmath`. A document with no tables doesn't load `booktabs`. This produces clean, minimal source.

3. **BibLaTeX for citations.** BibLaTeX + Biber is the modern standard, supporting all `BibliographyStyle` variants. The backend emits `\addbibresource{output.bib}` and appropriate `\cite` commands.

4. **Journal templates via document class.** The `LatexOptions` struct accepts a custom document class and preamble overrides, allowing journal-specific templates.

**AST mapping:**

| AST Node | LaTeX Output |
|---|---|
| `Heading { level: 1 }` | `\section{...}` |
| `Heading { level: 2 }` | `\subsection{...}` |
| `Heading { level: 3 }` | `\subsubsection{...}` |
| `Heading { level: 4+ }` | `\paragraph{...}`, `\subparagraph{...}` |
| `Paragraph` | Text block with `\par` separation |
| `Table` | `\begin{table}` + `booktabs` (`\toprule`, `\midrule`, `\bottomrule`) |
| `Image` | `\begin{figure}` + `\includegraphics` |
| `CodeBlock` | `\begin{lstlisting}` or `\begin{minted}` |
| `Equation` | `\begin{equation}` with optional `\label` |
| `Citation` | `\parencite{key}`, `\textcite{key}`, `\cite{key}` per `CitationMode` |
| `CrossRef` | `\ref{target}`, `\autoref{target}`, or `\cref{target}` |
| `Bibliography` | `\printbibliography` |
| `List (unordered)` | `\begin{itemize}` |
| `List (ordered)` | `\begin{enumerate}` |
| `BlockQuote` | `\begin{quote}` |
| `Diagram` | `\begin{tikzpicture}` (via lontar-diagram TikZ renderer) |
| `TextStyle.bold` | `\textbf{...}` |
| `TextStyle.italic` | `\textit{...}` |
| `PageBreak` | `\newpage` |
| `Divider` | `\hrulefill` or `\rule{\linewidth}{0.4pt}` |
| `TableOfContents` | `\tableofcontents` |
| `Math { display: false }` | `$...$` |
| `Math { display: true }` | `\[...\]` |

**Preamble generation example:**

```latex
\documentclass[a4paper,12pt]{article}  % or journal-specific class
\usepackage{fontspec}                   % always (XeLaTeX/LuaLaTeX)
\usepackage[english]{babel}             % from DocumentMetadata.language
\usepackage{graphicx}                   % if any Image blocks present
\usepackage{booktabs}                   % if any Table blocks present
\usepackage{amsmath,amssymb}            % if any Math/Equation nodes present
\usepackage{listings}                   % if any CodeBlock nodes present
\usepackage{hyperref}                   % if any Link/CrossRef nodes present
\usepackage{tikz}                       % if any Diagram blocks present
\usepackage[backend=biber,
            style=numeric]{biblatex}    % style from BibliographyStyle
\addbibresource{output.bib}

\title{...}
\author{...}
\date{...}

\begin{document}
\maketitle
% ... body ...
\printbibliography
\end{document}
```

**Options:**

```rust
pub struct LatexOptions {
    /// Document class (default: "article")
    pub document_class: String,
    /// Additional class options (e.g., "twocolumn", "11pt")
    pub class_options: Vec<String>,
    /// Additional preamble lines (for journal-specific packages)
    pub extra_preamble: Vec<String>,
    /// TeX engine hint (for comments/documentation in output)
    pub engine: TexEngine,
    /// Whether to emit \maketitle
    pub emit_maketitle: bool,
    /// Code listing package preference
    pub code_package: CodePackage,
    /// Cross-reference package preference
    pub crossref_package: CrossRefPackage,
}

pub enum TexEngine {
    XeLaTeX,
    LuaLaTeX,
}

pub enum CodePackage {
    Listings,
    Minted,
}

pub enum CrossRefPackage {
    /// Standard \ref
    Standard,
    /// \autoref (hyperref)
    AutoRef,
    /// \cref (cleveref) — most flexible
    CleverRef,
}
```

**Dependencies:** None beyond `std`. LaTeX output is pure text generation. The complexity is in correct escaping and preamble management, not in binary format handling.

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

**Citation rendering:** Inline text following the selected `BibliographyStyle` (e.g., `(Smith et al., 2024)` or `[1]`), with full reference list appended at the `Bibliography` block position.

### HTML Backend (`lontar-html`)

Emits self-contained HTML with inline CSS. Can optionally emit separate CSS.

**Use cases:**
- Email-ready document output
- Web preview of documents
- Intermediate format for PDF generation

**Citation rendering:** `<a>` tags linking to `#ref-{key}` anchors in the bibliography section, with tooltip showing the full reference.

### XLSX Backend (`lontar-xlsx`)

Thin integration wrapper around the `rust_xlsxwriter` crate. Rather than reimplementing Excel generation, this backend maps Lontar's AST to rust_xlsxwriter API calls.

**AST mapping strategy:**

| AST Node | XLSX Mapping |
|---|---|
| `Block::Heading { level: 1 }` | Worksheet name |
| `Block::Table { headers, rows }` | Worksheet with header row + data rows |
| `Block::Paragraph` | Cell content (merged across columns) |
| `Block::Chart` | rust_xlsxwriter chart object |
| `Block::Image` | Embedded worksheet image |
| Other blocks | Gracefully degraded or skipped |

**Design rationale:** Spreadsheets are fundamentally a different data model (typed cell grid) from documents (block/inline tree). A full AST-to-XLSX mapping will always be lossy. The wrapper provides convenience for the common case — exporting tables and charts to Excel from the same document pipeline — while users needing advanced spreadsheet features should use `rust_xlsxwriter` directly.

**Dependencies:** `rust_xlsxwriter`

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

### Academic Templates

The template system explicitly supports academic workflows:

```rust
// Medical journal submission
let template = Template::from_file("templates/lancet-article.lontar")?;
let bib = BibliographyStore::from_bibtex_file("references.bib")?;

let data = json!({
    "title": "Efficacy of Novel Treatment in Randomized Controlled Trial",
    "authors": ["Dr. A", "Dr. B"],
    "abstract": "Background: ...",
    "sections": { ... },
});

let doc = template.render_with_bibliography(&data, &bib)?;
doc.write_latex("submission.tex")?;   // For journal submission
doc.write_docx("review_copy.docx")?;  // For collaborator review
doc.write_pdf("preprint.pdf")?;        // For preprint upload
```

**Built-in academic templates (planned):**
- Generic research article (IMRAD structure)
- Systematic review (PRISMA-compliant)
- Case report
- Conference abstract
- Thesis/dissertation chapter
- Grant proposal
- Clinical trial report

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

    #[error("Bibliography error: {0}")]
    Bibliography(String),

    #[error("Citation key not found: {0}")]
    CitationNotFound(String),
}
```

### Feature Degradation

Not all AST features map to all formats. The strategy:

1. **Lossless:** Feature is fully supported (e.g., bold text in all formats)
2. **Degraded:** Feature is approximated (e.g., chart → table in Markdown, diagram → Mermaid in Markdown, diagram → ASCII in plain text)
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
│   ├── reference_docs/     ← Documents generated by python-docx/pptx
│   ├── expected_xml/       ← Extracted XML fragments from OOXML
│   ├── expected_md/        ← Expected Markdown output
│   ├── expected_html/      ← Expected HTML output
│   ├── expected_txt/       ← Expected plain text output
│   ├── expected_tex/       ← Expected LaTeX output
│   ├── expected_pdf/       ← Reference PDF output for visual comparison
│   ├── bib/                ← Test .bib files and CSL-JSON
│   └── scripts/            ← Multi-script test strings and fonts
├── integration/            ← Cross-crate integration tests
└── conformance/            ← Office compatibility tests
```

**Key testing approaches:**

- **Multi-format snapshot testing:** Generate all output formats from same AST, compare each against expected fixtures
- **XML snapshot testing:** Serialize AST to OOXML XML, compare against known-good snapshots
- **Round-trip testing:** AST → Markdown → parse → AST (verify structural equivalence)
- **LaTeX compilation testing:** Generate .tex → compile with XeLaTeX → verify PDF output
- **Citation resolution testing:** AST with citations → each format → verify citation numbers/keys and bibliography entries match
- **Reference comparison:** Generate equivalent docs via Python libs, compare XML structure
- **Conformance testing:** Open in LibreOffice/MS Office, verify rendering (manual + screenshot diffing)
- **Script rendering verification:** Shape test strings with rustybuzz, verify glyph output matches expected shaping behavior

## Dependency Graph

```
lontar-core          (zero external deps beyond std)
    │
    ├── lontar-aksara (rustybuzz, unicode-bidi, unicode-linebreak, unicode-script)
    │       │
    │       ├── lontar-docx  (zip, quick-xml)
    │       ├── lontar-pptx  (zip, quick-xml)
    │       └── lontar-pdf   (typst or printpdf — TBD)
    │
    ├── lontar-diagram (petgraph — optional, consumed by all backends)
    ├── lontar-xlsx  (rust_xlsxwriter)
    ├── lontar-latex (zero external deps — pure text generation)
    ├── lontar-md    (zero external deps)
    ├── lontar-txt   (zero external deps)
    ├── lontar-html  (zero external deps)
    └── lontar-template (tera or custom)

lontar (umbrella)    re-exports all backends via feature flags
lontar-cli           (clap, lontar)
```

Note: `lontar-latex` has zero external dependencies — it's pure text generation, like MD and TXT. The complexity is in correct LaTeX escaping and preamble management, not binary format handling. However, it *optionally* depends on `lontar-aksara` for font declarations when the document uses non-Latin scripts and the user wants explicit `\setmainfont` / `\newfontfamily` declarations.

## Performance Targets

For a typical 20-page report with tables and images:

| Format | Target | Notes |
|---|---|---|
| DOCX | < 50ms | XML templating + zip compression |
| PPTX | < 100ms | More complex XML, chart rendering |
| PDF | < 200ms | Layout computation is the bottleneck |
| LaTeX | < 10ms | String concatenation + preamble computation |
| XLSX | < 50ms | Delegates to rust_xlsxwriter |
| MD | < 5ms | String concatenation |
| HTML | < 10ms | String concatenation + CSS |
| TXT | < 5ms | Simplest possible output |

These targets reflect native compilation and zero-copy where possible. Text shaping adds minimal overhead — rustybuzz shapes thousands of glyphs per millisecond.

## Future Architecture Considerations

- **WASM target:** Core + text shaping + MD/HTML/LaTeX backends should compile to WASM for browser-side document generation with full script support
- **Vertical text layout:** CJK vertical text for PDF and PPTX backends
- **Streaming writes:** For very large documents, write pages/slides incrementally instead of buffering the entire ZIP
- **Async support:** Optional async file I/O for server use cases
- **Plugin system:** Allow third-party format backends (e.g., ODT, RTF, EPUB)
- **Font auto-discovery:** Detect and use system-installed fonts with script coverage analysis
- **BibTeX parser:** Native `.bib` file parsing (evaluate `biblatex-rs` or write custom parser)
- **CSL processor:** Citation Style Language support for precise journal-specific citation formatting
