# OOXML Spec Analysis Plan

Complete Phase 0 OOXML research tasks to understand the XML structure needed for DOCX and PPTX generation, creating reference documentation and mapping tables for implementation.

## Overview

This plan addresses the 8 tasks in the OOXML Spec Analysis section of Phase 0. The goal is to eliminate unknowns before implementation by thoroughly documenting the OOXML specification requirements, creating minimal working examples, and establishing clear mappings between Lontar's AST and OOXML elements.

## Prerequisites

- **ECMA-376 5th Edition specification** (downloadable from Ecma International)
- **Python environment** with `python-docx` and `python-pptx` for generating reference documents
- **Unzip utility** for extracting OOXML archives
- **Text editor/XML viewer** for examining extracted XML

## Task Breakdown

### 1. Download ECMA-376 5th Edition (Office Open XML)

**Location:** https://ecma-international.org/publications-and-standards/standards/ecma-376/

**Actions:**
- Download the complete specification (PDF format, ~5000 pages)
- Store in `docs/specs/ECMA-376-5th-edition.pdf`
- Optionally download individual parts:
  - Part 1: Fundamentals and Markup Language Reference (WordprocessingML, SpreadsheetML, PresentationML)
  - Part 2: Open Packaging Conventions
  - Part 3: Markup Compatibility and Extensibility
  - Part 4: Transitional Migration Features

**Deliverable:** Spec files in `docs/specs/`

---

### 2. Document Minimum Viable XML for "Hello World" .docx

**Approach:**
- Create the absolute simplest DOCX file using `python-docx`
- Unzip and examine the XML structure
- Document each required file and its purpose
- Identify which elements are mandatory vs. optional

**Python script** (`tools/gen_reference/minimal_docx.py`):
```python
from docx import Document

doc = Document()
doc.add_paragraph("Hello World")
doc.save("hello_world.docx")
```

**Expected structure:**
```
hello_world.docx/
├── [Content_Types].xml       # MIME type declarations
├── _rels/.rels                # Package relationships
├── word/
│   ├── document.xml           # Main content (the paragraph)
│   ├── _rels/document.xml.rels
│   └── styles.xml             # Default styles
└── docProps/
    ├── core.xml               # Metadata
    └── app.xml                # Application properties
```

**Documentation output:** `docs/ooxml/minimal-docx-structure.md`

**Contents:**
- File-by-file breakdown with XML snippets
- Explanation of each required element
- Namespace declarations and their purposes
- Minimum required attributes

---

### 3. Document Minimum Viable XML for "Hello World" .pptx

**Approach:**
- Create the simplest PPTX file with one slide and one text box
- Unzip and examine the XML structure
- Document the spatial positioning system (EMU coordinates)
- Identify slide master/layout requirements

**Python script** (`tools/gen_reference/minimal_pptx.py`):
```python
from pptx import Presentation

prs = Presentation()
slide = prs.slides.add_slide(prs.slide_layouts[0])  # Title slide
title = slide.shapes.title
title.text = "Hello World"
prs.save("hello_world.pptx")
```

**Expected structure:**
```
hello_world.pptx/
├── [Content_Types].xml
├── _rels/.rels
├── ppt/
│   ├── presentation.xml       # Presentation root
│   ├── slides/
│   │   └── slide1.xml         # The slide with text
│   ├── slideLayouts/
│   │   └── slideLayout1.xml   # Layout definition
│   ├── slideMasters/
│   │   └── slideMaster1.xml   # Master template
│   └── theme/
│       └── theme1.xml         # Color/font theme
└── docProps/
    ├── core.xml
    └── app.xml
```

**Documentation output:** `docs/ooxml/minimal-pptx-structure.md`

**Contents:**
- File-by-file breakdown
- EMU coordinate system explanation (1 EMU = 1/914,400 inch)
- Placeholder types and positioning
- Relationship between masters, layouts, and slides

---

### 4. Map DOCX XML Elements to AST Nodes

**Approach:**
- Create a comprehensive mapping table
- For each `Block` and `Inline` variant in the AST, document the corresponding OOXML element(s)
- Include namespace prefixes and required attributes
- Note any transformations or calculations needed

**Documentation output:** `docs/ooxml/docx-ast-mapping.md`

**Table format:**
```markdown
| AST Node | OOXML Element | Namespace | Required Attributes | Notes |
|----------|---------------|-----------|---------------------|-------|
| `Block::Heading { level: 1 }` | `<w:p><w:pPr><w:pStyle w:val="Heading1"/></w:pPr>...</w:p>` | `w` (WordprocessingML) | `w:val` | Levels 1-6 map to Heading1-Heading6 |
| `Block::Paragraph` | `<w:p><w:r><w:t>text</w:t></w:r></w:p>` | `w` | None | Text runs wrapped in `<w:r>` |
| `Inline::Text { style: { bold: true } }` | `<w:r><w:rPr><w:b/></w:rPr><w:t>text</w:t></w:r>` | `w` | None | Run properties in `<w:rPr>` |
| `Block::Table` | `<w:tbl><w:tr><w:tc>...</w:tc></w:tr></w:tbl>` | `w` | None | Rows (`w:tr`), cells (`w:tc`) |
| `Block::Image` | `<w:drawing><wp:inline>...</wp:inline></w:drawing>` | `w`, `wp`, `a` | Multiple | Complex: requires DrawingML |
| `Block::PageBreak` | `<w:p><w:r><w:br w:type="page"/></w:r></w:p>` | `w` | `w:type="page"` | Hard page break |
```

**Include sections for:**
- Text formatting (bold, italic, underline, color, font)
- Paragraph properties (alignment, spacing, indentation)
- Lists (ordered/unordered, numbering.xml)
- Tables (borders, shading, merged cells)
- Images and media
- Headers/footers
- Hyperlinks

---

### 5. Map PPTX XML Elements to AST Nodes

**Approach:**
- Similar to DOCX mapping, but account for spatial layout
- Document how flow-based AST translates to positioned elements
- Include slide layout types and placeholder mappings

**Documentation output:** `docs/ooxml/pptx-ast-mapping.md`

**Table format:**
```markdown
| AST Node | PPTX Element | Positioning Strategy | Notes |
|----------|--------------|----------------------|-------|
| `Block::Slide { layout: Title }` | `<p:sld>` with layout reference | Use title slide layout | Maps to `slideLayout1.xml` |
| `Block::Heading { level: 1 }` | `<p:sp>` (shape) with text | Title placeholder | Position at top, full width |
| `Block::Paragraph` | `<p:sp>` with `<a:p>` (text paragraph) | Body placeholder | Auto-flow in content region |
| `Block::Table` | `<p:graphicFrame>` with `<a:tbl>` | Centered in content area | Uses DrawingML table |
| `Block::Chart` | `<p:graphicFrame>` with chart reference | Centered or positioned | Separate chart XML file |
| `Inline::Text { style: { bold: true } }` | `<a:r><a:rPr><a:b/></a:rPr><a:t>text</a:t></a:r>` | N/A | DrawingML text run |
```

**Include sections for:**
- Slide layouts (title, content, two-column, blank)
- Text frames and auto-fit behavior
- Shape positioning (EMU coordinates)
- Charts (bar, line, pie)
- Speaker notes

---

### 6. Identify Shared OOXML Components

**Approach:**
- Document components used by both DOCX and PPTX
- Understand DrawingML (shared graphics markup)
- Understand Chart markup (shared between formats)
- Document the relationship system

**Documentation output:** `docs/ooxml/shared-components.md`

**Sections:**

**DrawingML (namespace `a`):**
- Used for: Images, shapes, charts, text formatting in PPTX
- Key elements: `<a:blip>` (image), `<a:tbl>` (table), `<a:p>` (paragraph)
- Color system: `<a:srgbClr>`, `<a:schemeClr>`
- Effects: shadows, reflections, 3D

**Chart Markup:**
- Shared between DOCX and PPTX
- Stored in separate XML files (`chart1.xml`)
- Chart types: bar, line, pie, scatter, area
- Data embedding vs. external links

**Relationships (.rels files):**
- Package relationships (`_rels/.rels`)
- Document relationships (`word/_rels/document.xml.rels`, `ppt/slides/_rels/slide1.xml.rels`)
- Relationship types: images, hyperlinks, charts, headers/footers
- ID generation and referencing

**Content Types ([Content_Types].xml):**
- MIME type declarations for each file in the package
- Default types (by extension)
- Override types (by part name)

---

### 7. Document the Relationship/Content Types System

**Approach:**
- Deep dive into the Open Packaging Conventions (ECMA-376 Part 2)
- Explain how relationships connect parts of the document
- Document the content type registration system

**Documentation output:** `docs/ooxml/relationships-and-content-types.md`

**Contents:**

**Relationships:**
- Relationship types (e.g., `http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument`)
- Target paths (relative vs. absolute)
- Relationship IDs (`rId1`, `rId2`, etc.)
- How to generate and track relationships during document construction

**Example workflow:**
1. Add an image to the document
2. Store image as `word/media/image1.png`
3. Add relationship in `word/_rels/document.xml.rels`:
   ```xml
   <Relationship Id="rId5" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="media/image1.png"/>
   ```
4. Reference in document: `<a:blip r:embed="rId5"/>`

**Content Types:**
- Default types: `.xml` → `application/xml`, `.png` → `image/png`
- Override types for specific parts
- Required content types for DOCX/PPTX

---

### 8. Catalog EMU (English Metric Units) Coordinate System for PPTX

**Approach:**
- Document the EMU unit system used for positioning in PPTX
- Provide conversion formulas
- Create examples of common slide dimensions and element positions

**Documentation output:** `docs/ooxml/emu-coordinate-system.md`

**Contents:**

**EMU Definition:**
- 1 EMU = 1/914,400 inch
- 1 inch = 914,400 EMU
- 1 cm = 360,000 EMU
- 1 point (typography) = 12,700 EMU

**Standard Slide Dimensions:**
- Standard (4:3): 9,144,000 × 6,858,000 EMU (10" × 7.5")
- Widescreen (16:9): 9,144,000 × 5,143,500 EMU (10" × 5.625")
- Custom sizes possible

**Positioning System:**
- Origin (0, 0) is top-left corner of slide
- X increases to the right
- Y increases downward
- Shapes defined by: `<p:xfrm>` (transform) with `<a:off>` (offset) and `<a:ext>` (extent)

**Example:**
```xml
<p:xfrm>
  <a:off x="914400" y="914400"/>    <!-- 1" from left, 1" from top -->
  <a:ext cx="4572000" cy="1828800"/> <!-- 5" wide, 2" tall -->
</p:xfrm>
```

**Conversion utilities to implement:**
```rust
fn inches_to_emu(inches: f64) -> i64 { (inches * 914_400.0) as i64 }
fn cm_to_emu(cm: f64) -> i64 { (cm * 360_000.0) as i64 }
fn pt_to_emu(pt: f64) -> i64 { (pt * 12_700.0) as i64 }
```

---

## Deliverables Summary

**Directory structure:**
```
docs/
├── specs/
│   └── ECMA-376-5th-edition.pdf
└── ooxml/
    ├── minimal-docx-structure.md
    ├── minimal-pptx-structure.md
    ├── docx-ast-mapping.md
    ├── pptx-ast-mapping.md
    ├── shared-components.md
    ├── relationships-and-content-types.md
    └── emu-coordinate-system.md

tests/fixtures/
├── reference_docs/
│   ├── hello_world.docx
│   ├── hello_world.pptx
│   └── (extracted XML directories)
└── expected_xml/
    ├── docx/
    │   ├── minimal-paragraph.xml
    │   ├── heading-level-1.xml
    │   └── ...
    └── pptx/
        ├── minimal-slide.xml
        ├── title-slide.xml
        └── ...

tools/gen_reference/
├── minimal_docx.py
├── minimal_pptx.py
└── extract_xml.py
```

## Execution Order

1. **Download spec** (Task 1) — prerequisite for all others
2. **Create minimal examples** (Tasks 2-3) — hands-on exploration
3. **Extract and organize XML** — supporting work for mapping
4. **Create mapping tables** (Tasks 4-5) — core deliverables
5. **Document shared components** (Task 6) — cross-cutting concerns
6. **Document relationships/content types** (Task 7) — packaging system
7. **Document EMU system** (Task 8) — PPTX-specific positioning

## Time Estimate

- **Task 1:** 30 minutes (download and organize)
- **Tasks 2-3:** 2-3 hours (create examples, extract, document structure)
- **Tasks 4-5:** 4-6 hours (comprehensive mapping tables with examples)
- **Task 6:** 2-3 hours (shared components research)
- **Task 7:** 2-3 hours (relationships and content types deep dive)
- **Task 8:** 1-2 hours (EMU system documentation and examples)

**Total:** 12-18 hours of focused work

## Success Criteria

- [ ] All 7 documentation files created in `docs/ooxml/`
- [ ] ECMA-376 spec downloaded and accessible
- [ ] Minimal DOCX and PPTX examples generated and extracted
- [ ] Mapping tables cover all AST node types defined in `ARCHITECTURE.md`
- [ ] Documentation is clear enough for implementation without re-reading the spec
- [ ] Examples include actual XML snippets, not just descriptions
- [ ] EMU conversion utilities are documented with formulas

## Notes

- This is **research and documentation work**, not implementation
- The goal is to create a reference that makes Phase 1 and Phase 2 implementation straightforward
- When in doubt, generate examples with python-docx/pptx and examine the XML
- Keep documentation concise but complete — include code snippets and examples
- Cross-reference between documents where appropriate
