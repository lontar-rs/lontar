# Minimal DOCX Structure

## Overview

A minimal DOCX file contains the absolute minimum XML structure required to be recognized as a valid Office Open XML document. This document deconstructs a "Hello World" DOCX to understand the required files and their purposes.

## File Structure

```
hello_world.docx (ZIP archive)
├── [Content_Types].xml
├── _rels/.rels
├── word/
│   ├── document.xml
│   ├── styles.xml
│   └── _rels/
│       └── document.xml.rels
└── docProps/
    ├── core.xml
    └── app.xml
```

## File Breakdown

### [Content_Types].xml

**Purpose:** MIME type declarations for all files in the package.

**Minimal content:**
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
  <Override PartName="/word/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"/>
  <Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>
  <Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.custom-properties+xml"/>
</Types>
```

**Key elements:**
- `<Default>` — applies to all files matching the extension
- `<Override>` — applies to a specific file path, overriding defaults
- All content types are MIME-like strings with `+xml` suffix for XML parts

### _rels/.rels

**Purpose:** Package-level relationships. Defines which part is the main document.

**Minimal content:**
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml"/>
  <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" Target="docProps/app.xml"/>
</Relationships>
```

**Key elements:**
- `Id` — unique identifier within this relationships file
- `Type` — semantic relationship type (URI)
- `Target` — relative path to the target part

### word/document.xml

**Purpose:** Main document content. Contains all paragraphs, tables, images, etc.

**Minimal content:**
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
            xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <w:body>
    <w:p>
      <w:r>
        <w:t>Hello World</w:t>
      </w:r>
    </w:p>
  </w:body>
</w:document>
```

**Key elements:**
- `<w:document>` — root element
- `<w:body>` — container for all block-level content
- `<w:p>` — paragraph
- `<w:r>` — run (sequence of characters with same formatting)
- `<w:t>` — text content

**Namespaces:**
- `w` — WordprocessingML (main content)
- `r` — Relationships (for referencing external parts like images)

### word/styles.xml

**Purpose:** Style definitions. Even if no styles are used, this file must exist.

**Minimal content:**
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:docDefaults>
    <w:rPrDefault>
      <w:rPr>
        <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/>
        <w:sz w:val="22"/>
      </w:rPr>
    </w:rPrDefault>
    <w:pPrDefault/>
  </w:docDefaults>
</w:styles>
```

**Key elements:**
- `<w:docDefaults>` — default formatting for the entire document
- `<w:rPrDefault>` — default run properties (font, size)
- `<w:pPrDefault>` — default paragraph properties

### word/_rels/document.xml.rels

**Purpose:** Document-level relationships. Links to images, hyperlinks, headers, footers, etc.

**Minimal content (empty document):**
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
</Relationships>
```

**When adding images:**
```xml
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="media/image1.png"/>
```

### docProps/core.xml

**Purpose:** Document metadata (title, author, creation date, etc.).

**Minimal content:**
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/officeDocument/2006/custom-properties"
                   xmlns:dc="http://purl.org/dc/elements/1.1/"
                   xmlns:dcterms="http://purl.org/dc/terms/"
                   xmlns:dcmitype="http://purl.org/dc/dcmitype/"
                   xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <dc:title>Hello World</dc:title>
  <dc:creator>Lontar</dc:creator>
  <cp:lastModifiedBy>Lontar</cp:lastModifiedBy>
  <dcterms:created xsi:type="dcterms:W3CDTF">2024-01-01T00:00:00Z</dcterms:created>
  <dcterms:modified xsi:type="dcterms:W3CDTF">2024-01-01T00:00:00Z</dcterms:modified>
</cp:coreProperties>
```

**Key elements:**
- `<dc:title>` — document title
- `<dc:creator>` — author
- `<dcterms:created>` — creation timestamp (ISO 8601)
- `<dcterms:modified>` — last modified timestamp

### docProps/app.xml

**Purpose:** Application-specific metadata (page count, word count, etc.).

**Minimal content:**
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties"
            xmlns:vt="http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes">
  <TotalTime>0</TotalTime>
  <Pages>1</Pages>
  <Words>2</Words>
  <Characters>11</Characters>
  <Application>Lontar</Application>
  <AppVersion>0.1.0</AppVersion>
</Properties>
```

**Key elements:**
- `<Pages>` — page count
- `<Words>` — word count
- `<Characters>` — character count
- `<Application>` — generating application name

## Summary

A minimal DOCX requires:
1. **[Content_Types].xml** — MIME type registry
2. **_rels/.rels** — package relationships (points to main document)
3. **word/document.xml** — main content
4. **word/styles.xml** — style definitions (can be minimal)
5. **word/_rels/document.xml.rels** — document relationships
6. **docProps/core.xml** — metadata
7. **docProps/app.xml** — application properties

All files are XML with specific namespaces. The entire package is a ZIP archive with no compression for XML files (stored as-is).

## Implementation Notes

When generating DOCX from Lontar's AST:
- Create `[Content_Types].xml` first with all required overrides
- Generate `_rels/.rels` with single relationship to main document
- Build `word/document.xml` by traversing the AST
- Generate `word/styles.xml` with used styles
- Create `word/_rels/document.xml.rels` as you add images/hyperlinks
- Populate `docProps/core.xml` from `DocumentMetadata`
- Generate `docProps/app.xml` with computed statistics

ZIP the entire structure with the correct archive structure (no leading directory).
