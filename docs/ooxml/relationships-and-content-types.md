# Relationships and Content Types System

Deep dive into the Open Packaging Conventions (ECMA-376 Part 2) that govern how OOXML documents are structured and how parts are linked together.

## Overview

OOXML documents are ZIP archives containing multiple XML files (parts) that are linked together via relationships. The content types registry tells the application how to interpret each file.

**Key concepts:**
- **Parts** — individual XML files (document.xml, styles.xml, image1.png, etc.)
- **Relationships** — directed links between parts
- **Content Types** — MIME type declarations for each part
- **Packages** — the ZIP archive containing all parts

## Relationships

### Relationship Anatomy

```xml
<Relationship Id="rId1" 
              Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" 
              Target="media/image1.png"
              TargetMode="Internal"/>
```

**Attributes:**
- **Id** — unique identifier within the relationships file (e.g., `rId1`, `rId2`)
- **Type** — semantic relationship type (URI)
- **Target** — path to the target part (relative or absolute)
- **TargetMode** — `Internal` (default) or `External`

### Relationship Types

#### Package-Level Relationships (_rels/.rels)

These relationships are in the root `_rels/.rels` file and define the entry points to the document.

| Type | Target | Purpose |
|------|--------|---------|
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument` | word/document.xml or ppt/presentation.xml | Main document part |
| `http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties` | docProps/core.xml | Core metadata |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties` | docProps/app.xml | Extended metadata |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties` | docProps/custom.xml | Custom properties |

**Example (_rels/.rels):**
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml"/>
  <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" Target="docProps/app.xml"/>
</Relationships>
```

#### Document-Level Relationships (word/_rels/document.xml.rels)

These relationships are in `word/_rels/document.xml.rels` and define resources used by the main document.

| Type | Target | Purpose |
|------|--------|---------|
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/image` | media/image1.png | Embedded image |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink` | https://example.com | External hyperlink |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart` | charts/chart1.xml | Embedded chart |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/header` | header1.xml | Header |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer` | footer1.xml | Footer |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering` | numbering.xml | List numbering definitions |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles` | styles.xml | Style definitions |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable` | fontTable.xml | Font table |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings` | settings.xml | Document settings |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme` | theme/theme1.xml | Theme |

**Example (word/_rels/document.xml.rels):**
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering" Target="numbering.xml"/>
  <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="media/image1.png"/>
  <Relationship Id="rId4" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink" Target="https://example.com" TargetMode="External"/>
  <Relationship Id="rId5" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/header" Target="header1.xml"/>
  <Relationship Id="rId6" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer" Target="footer1.xml"/>
</Relationships>
```

#### Presentation-Level Relationships (ppt/_rels/presentation.xml.rels)

| Type | Target | Purpose |
|------|--------|---------|
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide` | slides/slide1.xml | Slide |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster` | slideMasters/slideMaster1.xml | Slide master |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps` | presProps.xml | Presentation properties |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles` | tableStyles.xml | Table styles |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme` | theme/theme1.xml | Theme |

### Referencing Relationships

Relationships are referenced in XML using the `r:id` attribute (namespace `r` = `http://schemas.openxmlformats.org/officeDocument/2006/relationships`).

**In DOCX:**
```xml
<!-- Reference to image via relationship -->
<a:blip r:embed="rId3"/>

<!-- Reference to hyperlink via relationship -->
<w:hyperlink r:id="rId4">
  <w:r>
    <w:t>Click here</w:t>
  </w:r>
</w:hyperlink>

<!-- Reference to header via relationship -->
<w:headerReference w:type="default" r:id="rId5"/>
```

**In PPTX:**
```xml
<!-- Reference to slide layout via relationship -->
<p:sldId id="256" r:id="rId2"/>

<!-- Reference to image via relationship -->
<a:blip r:embed="rId2"/>

<!-- Reference to theme via relationship -->
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="../theme/theme1.xml"/>
```

### Relationship ID Management

**Best practices:**
1. **Start with rId1** — first relationship gets ID `rId1`
2. **Increment sequentially** — `rId2`, `rId3`, etc.
3. **Maintain a counter** — track the next available ID
4. **Avoid gaps** — don't skip IDs (though technically allowed)
5. **Scope per file** — each relationships file has its own ID namespace

**Example workflow:**
```
1. Create word/document.xml
2. Add relationship to styles.xml → rId1
3. Add relationship to numbering.xml → rId2
4. Add relationship to image1.png → rId3
5. Add relationship to hyperlink → rId4
6. Write word/_rels/document.xml.rels with all relationships
```

### Internal vs. External Relationships

**Internal (default):**
```xml
<Relationship Id="rId1" 
              Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" 
              Target="media/image1.png"/>
```
- Target is relative path within the package
- No `TargetMode` attribute (defaults to `Internal`)

**External:**
```xml
<Relationship Id="rId2" 
              Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink" 
              Target="https://example.com" 
              TargetMode="External"/>
```
- Target is absolute URL
- Must include `TargetMode="External"`

## Content Types Registry

### [Content_Types].xml Structure

The `[Content_Types].xml` file (note the square brackets) is located at the root of the ZIP archive and declares MIME types for all parts.

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <!-- Default types by file extension -->
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Default Extension="jpeg" ContentType="image/jpeg"/>
  <Default Extension="png" ContentType="image/png"/>
  <Default Extension="gif" ContentType="image/gif"/>
  <Default Extension="bmp" ContentType="image/bmp"/>
  <Default Extension="tiff" ContentType="image/tiff"/>
  <Default Extension="wmf" ContentType="image/x-wmf"/>
  <Default Extension="emf" ContentType="image/x-emf"/>
  
  <!-- Override types for specific parts -->
  <Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
  <Override PartName="/word/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"/>
  <Override PartName="/word/numbering.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml"/>
  <Override PartName="/word/header1.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml"/>
  <Override PartName="/word/footer1.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml"/>
  <Override PartName="/word/charts/chart1.xml" ContentType="application/vnd.openxmlformats-officedocument.drawingml.chart+xml"/>
  <Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>
  <Override PartName="/ppt/slides/slide1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
  <Override PartName="/ppt/slideLayouts/slideLayout1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"/>
  <Override PartName="/ppt/slideMasters/slideMaster1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"/>
  <Override PartName="/ppt/theme/theme1.xml" ContentType="application/vnd.openxmlformats-officedocument.theme+xml"/>
  <Override PartName="/ppt/presProps.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presProps+xml"/>
  <Override PartName="/ppt/tableStyles.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml"/>
  <Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>
  <Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.custom-properties+xml"/>
</Types>
```

### Default vs. Override

**Default elements:**
- Apply to all files matching the extension
- Used for common types (images, generic XML)
- Processed first

**Override elements:**
- Apply to a specific part path
- Override default types
- Processed after defaults

**Resolution:**
1. Check if part matches an `<Override>` — use that content type
2. Otherwise, check if extension matches a `<Default>` — use that content type
3. If neither, the part is untyped (error)

### Content Type Naming

**Pattern:** `application/vnd.openxmlformats-[package|officedocument].[format].[part]+xml`

**Components:**
- `application/vnd.openxmlformats` — vendor prefix
- `package` or `officedocument` — scope
- `wordprocessingml`, `presentationml`, `spreadsheetml` — format
- `document`, `styles`, `slide`, etc. — part type
- `main` — main part (for documents with multiple parts)
- `+xml` — suffix indicating XML

**Examples:**
- `application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml` — DOCX main document
- `application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml` — DOCX styles
- `application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml` — PPTX main presentation
- `application/vnd.openxmlformats-officedocument.presentationml.slide+xml` — PPTX slide
- `application/vnd.openxmlformats-officedocument.drawingml.chart+xml` — Chart
- `application/vnd.openxmlformats-package.core-properties+xml` — Core metadata

## Building a Document: Step-by-Step

### DOCX Example

**Step 1: Create main document**
```
word/document.xml
```

**Step 2: Create styles**
```
word/styles.xml
```

**Step 3: Create relationships file**
```
word/_rels/document.xml.rels
```
Contains:
- Relationship to styles.xml (rId1)
- Relationship to numbering.xml (rId2)
- Relationship to image1.png (rId3)
- Relationship to hyperlink (rId4)

**Step 4: Create metadata**
```
docProps/core.xml
docProps/app.xml
```

**Step 5: Create package relationships**
```
_rels/.rels
```
Contains:
- Relationship to word/document.xml (rId1)
- Relationship to docProps/core.xml (rId2)
- Relationship to docProps/app.xml (rId3)

**Step 6: Create content types registry**
```
[Content_Types].xml
```
Declares types for:
- word/document.xml
- word/styles.xml
- word/numbering.xml
- media/image1.png
- docProps/core.xml
- docProps/app.xml

**Step 7: ZIP everything**
```
docx.zip
├── [Content_Types].xml
├── _rels/.rels
├── word/
│   ├── document.xml
│   ├── styles.xml
│   ├── numbering.xml
│   ├── media/
│   │   └── image1.png
│   └── _rels/
│       └── document.xml.rels
└── docProps/
    ├── core.xml
    └── app.xml
```

**Step 8: Rename to .docx**
```
mv docx.zip document.docx
```

### PPTX Example

**Step 1: Create presentation**
```
ppt/presentation.xml
```

**Step 2: Create slides**
```
ppt/slides/slide1.xml
ppt/slides/slide2.xml
```

**Step 3: Create slide layouts**
```
ppt/slideLayouts/slideLayout1.xml
ppt/slideLayouts/slideLayout2.xml
```

**Step 4: Create slide masters**
```
ppt/slideMasters/slideMaster1.xml
```

**Step 5: Create theme**
```
ppt/theme/theme1.xml
```

**Step 6: Create presentation properties**
```
ppt/presProps.xml
ppt/tableStyles.xml
```

**Step 7: Create relationships files**
```
ppt/_rels/presentation.xml.rels
ppt/slides/_rels/slide1.xml.rels
ppt/slides/_rels/slide2.xml.rels
ppt/slideLayouts/_rels/slideLayout1.xml.rels
ppt/slideMasters/_rels/slideMaster1.xml.rels
```

**Step 8: Create metadata**
```
docProps/core.xml
docProps/app.xml
```

**Step 9: Create package relationships**
```
_rels/.rels
```

**Step 10: Create content types registry**
```
[Content_Types].xml
```

**Step 11: ZIP everything**
```
pptx.zip
├── [Content_Types].xml
├── _rels/.rels
├── ppt/
│   ├── presentation.xml
│   ├── presProps.xml
│   ├── tableStyles.xml
│   ├── slides/
│   │   ├── slide1.xml
│   │   ├── slide2.xml
│   │   └── _rels/
│   │       ├── slide1.xml.rels
│   │       └── slide2.xml.rels
│   ├── slideLayouts/
│   │   ├── slideLayout1.xml
│   │   ├── slideLayout2.xml
│   │   └── _rels/
│   │       ├── slideLayout1.xml.rels
│   │       └── slideLayout2.xml.rels
│   ├── slideMasters/
│   │   ├── slideMaster1.xml
│   │   └── _rels/
│   │       └── slideMaster1.xml.rels
│   ├── theme/
│   │   └── theme1.xml
│   └── _rels/
│       └── presentation.xml.rels
└── docProps/
    ├── core.xml
    └── app.xml
```

**Step 12: Rename to .pptx**
```
mv pptx.zip presentation.pptx
```

## Validation Checklist

When building a document, verify:

- [ ] All referenced relationships exist in the relationships file
- [ ] All relationship IDs are unique within their scope
- [ ] All relationship targets exist in the package
- [ ] All parts have a content type declaration
- [ ] Package relationships include the main document
- [ ] Package relationships include core metadata
- [ ] All external relationships have `TargetMode="External"`
- [ ] All internal relationships have relative paths
- [ ] No circular relationships
- [ ] ZIP archive structure is correct (no leading directory)
- [ ] [Content_Types].xml is at the root

## Common Pitfalls

1. **Missing relationship** — XML references `r:id="rId5"` but relationship doesn't exist
2. **Wrong target path** — relationship points to `media/image.png` but file is at `word/media/image.png`
3. **Missing content type** — part exists but not declared in [Content_Types].xml
4. **Duplicate IDs** — two relationships with same `Id` in same file
5. **Wrong TargetMode** — external URL without `TargetMode="External"`
6. **Circular references** — A → B → A (technically allowed but problematic)
7. **ZIP structure** — files at root instead of in directories

## Summary

The relationships and content types system is the glue that holds OOXML documents together:
- **Relationships** link parts together and to external resources
- **Content types** declare the MIME type of each part
- **Proper structure** ensures Office applications can read the document
- **Validation** prevents common errors during document generation
