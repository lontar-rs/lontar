# OOXML Namespace Cheat Sheet

Quick reference for all XML namespaces used in DOCX and PPTX generation.

---

## 1. Master Namespace Table

| Prefix | Full URI | Used In | Purpose | Common Elements |
|---|---|---|---|---|
| `w` | `http://schemas.openxmlformats.org/wordprocessingml/2006/main` | DOCX | WordprocessingML main namespace | `w:document`, `w:p`, `w:r`, `w:t`, `w:rPr`, `w:pPr` |
| `w14` | `http://schemas.microsoft.com/office/word/2010/wordml` | DOCX | Word 2010+ extensions | `w14:glow`, `w14:shadow`, `w14:reflection` |
| `w15` | `http://schemas.microsoft.com/office/word/2012/wordml` | DOCX | Word 2012+ extensions | `w15:footnoteRef`, `w15:endnoteRef` |
| `r` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships` | Both | Relationships namespace | `r:id`, `r:embed` |
| `p` | `http://schemas.openxmlformats.org/presentationml/2006/main` | PPTX | PresentationML main namespace | `p:presentation`, `p:sld`, `p:cSld`, `p:spTree` |
| `p14` | `http://schemas.microsoft.com/office/powerpoint/2010/main` | PPTX | PowerPoint 2010+ extensions | `p14:hiddenFill`, `p14:modId` |
| `p15` | `http://schemas.microsoft.com/office/powerpoint/2012/main` | PPTX | PowerPoint 2012+ extensions | `p15:presenceInfo` |
| `a` | `http://schemas.openxmlformats.org/drawingml/2006/main` | Both | DrawingML main namespace | `a:p`, `a:r`, `a:t`, `a:rPr`, `a:solidFill`, `a:latin` |
| `pic` | `http://schemas.openxmlformats.org/drawingml/2006/picture` | Both | DrawingML picture namespace | `pic:pic`, `pic:nvPicPr`, `pic:blipFill` |
| `wp` | `http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing` | DOCX | Word drawing (inline/anchor) | `wp:inline`, `wp:anchor`, `wp:extent`, `wp:docPr` |
| `wp14` | `http://schemas.microsoft.com/office/word/2010/wordprocessingDrawing` | DOCX | Word drawing 2010+ extensions | `wp14:sizeRelH`, `wp14:sizeRelV` |
| `wps` | `http://schemas.microsoft.com/office/word/2010/wordprocessingShape` | DOCX | Word drawing shape | `wps:wsp`, `wps:cNvSpPr`, `wps:txbx` |
| `wpg` | `http://schemas.microsoft.com/office/word/2010/wordprocessingGroup` | DOCX | Word drawing group | `wpg:grpSp`, `wpg:grpSpPr` |
| `c` | `http://schemas.openxmlformats.org/drawingml/2006/chart` | Both | DrawingML chart namespace | `c:chartSpace`, `c:barChart`, `c:ser`, `c:val`, `c:cat` |
| `dgm` | `http://schemas.openxmlformats.org/drawingml/2006/diagram` | Both | DrawingML diagram namespace | `dgm:relIds`, `dgm:cNvDiagramPr` |
| `m` | `http://schemas.openxmlformats.org/officeDocument/2006/math` | DOCX | Office Math namespace | `m:oMath`, `m:e`, `m:r`, `m:t` |
| `mc` | `http://schemas.openxmlformats.org/markup-compatibility/2006` | Both | Markup compatibility | `mc:Ignorable`, `mc:AlternateContent` |
| `v` | `urn:schemas-microsoft-com:vml` | Both | VML (legacy vector markup) | `v:shape`, `v:line`, `v:polyline`, `v:f` |
| `o` | `urn:schemas-microsoft-com:office:office` | Both | VML Office extensions | `o:p`, `o:idmap`, `o:shapelayout` |
| `dc` | `http://purl.org/dc/elements/1.1/` | Both | Dublin Core metadata | `dc:title`, `dc:subject`, `dc:creator` |
| `dcterms` | `http://purl.org/dc/terms/` | Both | Dublin Core terms | `dcterms:created`, `dcterms:modified` |
| `dcmitype` | `http://purl.org/dc/dcmitype/` | Both | Dublin Core type vocabulary | `dcmitype:Text` |
| `cp` | `http://schemas.openxmlformats.org/officeDocument/2006/custom-properties` | Both | Core properties | `cp:coreProperties` |
| `xsi` | `http://www.w3.org/2001/XMLSchema-instance` | Both | XML Schema instance | `xsi:type`, `xsi:schemaLocation` |
| `xml` | `http://www.w3.org/XML/1998/namespace` | Both | XML base namespace | `xml:lang`, `xml:space` |
| (none) | `http://schemas.openxmlformats.org/package/2006/content-types` | Both | Content Types | `Types`, `Default`, `Override` |
| (none) | `http://schemas.openxmlformats.org/package/2006/relationships` | Both | Package relationships | `Relationships`, `Relationship` |

---

## 2. Namespace Declarations by File

### DOCX Files

#### word/document.xml

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
            xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
            xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml"
            xmlns:w15="http://schemas.microsoft.com/office/word/2012/wordml"
            xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing"
            xmlns:wp14="http://schemas.microsoft.com/office/word/2010/wordprocessingDrawing"
            xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"
            xmlns:wpg="http://schemas.microsoft.com/office/word/2010/wordprocessingGroup"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
            xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture"
            xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"
            xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"
            xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006"
            mc:Ignorable="w14 w15 wp14">
  <!-- document content -->
</w:document>
```

#### word/styles.xml

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
          xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
          xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml"
          xmlns:w15="http://schemas.microsoft.com/office/word/2012/wordml"
          xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006"
          mc:Ignorable="w14 w15">
  <!-- styles content -->
</w:styles>
```

#### word/numbering.xml

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:numbering xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
             xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml"
             xmlns:w15="http://schemas.microsoft.com/office/word/2012/wordml"
             xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006"
             mc:Ignorable="w14 w15">
  <!-- numbering content -->
</w:numbering>
```

#### word/fontTable.xml

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:fontTable xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
             xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <!-- font definitions -->
</w:fontTable>
```

#### word/header1.xml / word/footer1.xml

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:hdr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
       xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing"
       xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture">
  <!-- header content -->
</w:hdr>

<!-- For footer, use w:ftr instead of w:hdr -->
```

#### word/settings.xml

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:settings xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
            xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
            xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml"
            xmlns:w15="http://schemas.microsoft.com/office/word/2012/wordml">
  <!-- settings content -->
</w:settings>
```

### PPTX Files

#### ppt/presentation.xml

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
                xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
                xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
                xmlns:p14="http://schemas.microsoft.com/office/powerpoint/2010/main"
                xmlns:p15="http://schemas.microsoft.com/office/powerpoint/2012/main"
                xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006"
                mc:Ignorable="p14 p15">
  <!-- presentation content -->
</p:presentation>
```

#### ppt/slides/slide1.xml

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
       xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture"
       xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"
       xmlns:p14="http://schemas.microsoft.com/office/powerpoint/2010/main"
       xmlns:p15="http://schemas.microsoft.com/office/powerpoint/2012/main"
       xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006"
       mc:Ignorable="p14 p15">
  <!-- slide content -->
</p:sld>
```

#### ppt/slideLayouts/slideLayout1.xml

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
             xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
             xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
             xmlns:p14="http://schemas.microsoft.com/office/powerpoint/2010/main"
             xmlns:p15="http://schemas.microsoft.com/office/powerpoint/2012/main"
             xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006"
             mc:Ignorable="p14 p15">
  <!-- slide layout content -->
</p:sldLayout>
```

#### ppt/slideMasters/slideMaster1.xml

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldMaster xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
             xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
             xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
             xmlns:p14="http://schemas.microsoft.com/office/powerpoint/2010/main"
             xmlns:p15="http://schemas.microsoft.com/office/powerpoint/2012/main"
             xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006"
             mc:Ignorable="p14 p15">
  <!-- slide master content -->
</p:sldMaster>
```

#### ppt/theme/theme1.xml

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
         name="Office Theme">
  <!-- theme content -->
</a:theme>
```

#### ppt/presProps.xml

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presProps xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
             xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <!-- presentation properties -->
</p:presProps>
```

### Shared Files

#### [Content_Types].xml

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <!-- content type definitions -->
</Types>
```

#### _rels/.rels

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <!-- package relationships -->
</Relationships>
```

#### word/_rels/document.xml.rels (DOCX)

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <!-- document relationships -->
</Relationships>
```

#### ppt/_rels/presentation.xml.rels (PPTX)

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <!-- presentation relationships -->
</Relationships>
```

#### docProps/core.xml

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/officeDocument/2006/custom-properties"
                   xmlns:dc="http://purl.org/dc/elements/1.1/"
                   xmlns:dcterms="http://purl.org/dc/terms/"
                   xmlns:dcmitype="http://purl.org/dc/dcmitype/"
                   xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <!-- core properties -->
</cp:coreProperties>
```

#### docProps/app.xml

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties"
            xmlns:vt="http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes">
  <!-- application properties -->
</Properties>
```

---

## 3. Relationship Type URIs

| Short Name | Full URI | Source File | Target File |
|---|---|---|---|
| `officeDocument` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument` | `_rels/.rels` | `word/document.xml` (DOCX) or `ppt/presentation.xml` (PPTX) |
| `styles` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles` | `word/_rels/document.xml.rels` | `word/styles.xml` |
| `numbering` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering` | `word/_rels/document.xml.rels` | `word/numbering.xml` |
| `fontTable` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable` | `word/_rels/document.xml.rels` | `word/fontTable.xml` |
| `settings` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings` | `word/_rels/document.xml.rels` | `word/settings.xml` |
| `theme` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme` | `ppt/_rels/presentation.xml.rels` or `ppt/slides/_rels/slide1.xml.rels` | `ppt/theme/theme1.xml` |
| `image` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/image` | `word/_rels/document.xml.rels` or `ppt/slides/_rels/slide1.xml.rels` | `word/media/image1.png` |
| `hyperlink` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink` | `word/_rels/document.xml.rels` | External URL |
| `header` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/header` | `word/_rels/document.xml.rels` | `word/header1.xml` |
| `footer` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer` | `word/_rels/document.xml.rels` | `word/footer1.xml` |
| `chart` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart` | `word/_rels/document.xml.rels` or `ppt/slides/_rels/slide1.xml.rels` | `word/charts/chart1.xml` |
| `slide` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide` | `ppt/_rels/presentation.xml.rels` | `ppt/slides/slide1.xml` |
| `slideLayout` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout` | `ppt/slides/_rels/slide1.xml.rels` | `ppt/slideLayouts/slideLayout1.xml` |
| `slideMaster` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster` | `ppt/slideLayouts/_rels/slideLayout1.xml.rels` | `ppt/slideMasters/slideMaster1.xml` |
| `presProps` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps` | `ppt/_rels/presentation.xml.rels` | `ppt/presProps.xml` |
| `tableStyles` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles` | `ppt/_rels/presentation.xml.rels` | `ppt/tableStyles.xml` |
| `core-properties` | `http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties` | `_rels/.rels` | `docProps/core.xml` |
| `extended-properties` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties` | `_rels/.rels` | `docProps/app.xml` |
| `custom-properties` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties` | `_rels/.rels` | `docProps/custom.xml` |
| `package` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/package` | `word/charts/_rels/chart1.xml.rels` | `word/charts/chart1.xlsx` |

---

## 4. Content Type Strings

### DOCX Content Types

| Part | Content Type String |
|---|---|
| `word/document.xml` | `application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml` |
| `word/styles.xml` | `application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml` |
| `word/numbering.xml` | `application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml` |
| `word/fontTable.xml` | `application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml` |
| `word/settings.xml` | `application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml` |
| `word/header*.xml` | `application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml` |
| `word/footer*.xml` | `application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml` |
| `word/charts/chart*.xml` | `application/vnd.openxmlformats-officedocument.drawingml.chart+xml` |
| `word/media/image*.png` | `image/png` |
| `word/media/image*.jpg` | `image/jpeg` |
| `word/media/image*.gif` | `image/gif` |
| `word/embeddings/font*.odttf` | `application/vnd.openxmlformats-officedocument.obfuscatedFont` |

### PPTX Content Types

| Part | Content Type String |
|---|---|
| `ppt/presentation.xml` | `application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml` |
| `ppt/slides/slide*.xml` | `application/vnd.openxmlformats-officedocument.presentationml.slide+xml` |
| `ppt/slideLayouts/slideLayout*.xml` | `application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml` |
| `ppt/slideMasters/slideMaster*.xml` | `application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml` |
| `ppt/theme/theme*.xml` | `application/vnd.openxmlformats-officedocument.theme+xml` |
| `ppt/presProps.xml` | `application/vnd.openxmlformats-officedocument.presentationml.presProps+xml` |
| `ppt/tableStyles.xml` | `application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml` |
| `ppt/charts/chart*.xml` | `application/vnd.openxmlformats-officedocument.drawingml.chart+xml` |
| `ppt/media/image*.png` | `image/png` |
| `ppt/media/image*.jpg` | `image/jpeg` |
| `ppt/media/image*.gif` | `image/gif` |

### Shared Content Types

| Part | Content Type String |
|---|---|
| `docProps/core.xml` | `application/vnd.openxmlformats-package.core-properties+xml` |
| `docProps/app.xml` | `application/vnd.openxmlformats-officedocument.extended-properties+xml` |
| `docProps/custom.xml` | `application/vnd.openxmlformats-officedocument.custom-properties+xml` |
| `.rels` (any) | `application/vnd.openxmlformats-package.relationships+xml` |
| `[Content_Types].xml` | `application/vnd.openxmlformats-package.content-types+xml` |

---

## 5. Lontar Implementation Guide

### Recommended Module Structure

Create the following modules in `lontar-docx` and `lontar-pptx`:

```
src/
├── namespaces.rs      # Namespace URI constants
├── relationships.rs   # Relationship type constants
├── content_types.rs   # Content type MIME strings
└── xml/
    └── builder.rs     # XML element builder with namespace helpers
```

### namespaces.rs

```rust
// DOCX/PPTX shared namespaces
pub const NS_RELATIONSHIPS: &str = 
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
pub const NS_MARKUP_COMPAT: &str = 
    "http://schemas.openxmlformats.org/markup-compatibility/2006";
pub const NS_XML: &str = 
    "http://www.w3.org/XML/1998/namespace";
pub const NS_XSI: &str = 
    "http://www.w3.org/2001/XMLSchema-instance";

// DrawingML namespaces (shared)
pub const NS_DRAWINGML: &str = 
    "http://schemas.openxmlformats.org/drawingml/2006/main";
pub const NS_DRAWINGML_PICTURE: &str = 
    "http://schemas.openxmlformats.org/drawingml/2006/picture";
pub const NS_DRAWINGML_CHART: &str = 
    "http://schemas.openxmlformats.org/drawingml/2006/chart";
pub const NS_DRAWINGML_DIAGRAM: &str = 
    "http://schemas.openxmlformats.org/drawingml/2006/diagram";

// VML namespaces (legacy)
pub const NS_VML: &str = 
    "urn:schemas-microsoft-com:vml";
pub const NS_VML_OFFICE: &str = 
    "urn:schemas-microsoft-com:office:office";

// Metadata namespaces
pub const NS_DUBLIN_CORE: &str = 
    "http://purl.org/dc/elements/1.1/";
pub const NS_DUBLIN_CORE_TERMS: &str = 
    "http://purl.org/dc/terms/";
pub const NS_DUBLIN_CORE_TYPE: &str = 
    "http://purl.org/dc/dcmitype/";
pub const NS_CORE_PROPERTIES: &str = 
    "http://schemas.openxmlformats.org/officeDocument/2006/custom-properties";

// DOCX-specific namespaces
pub const NS_WML: &str = 
    "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
pub const NS_WML_2010: &str = 
    "http://schemas.microsoft.com/office/word/2010/wordml";
pub const NS_WML_2012: &str = 
    "http://schemas.microsoft.com/office/word/2012/wordml";
pub const NS_WML_DRAWING: &str = 
    "http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing";
pub const NS_WML_DRAWING_2010: &str = 
    "http://schemas.microsoft.com/office/word/2010/wordprocessingDrawing";
pub const NS_WML_SHAPE: &str = 
    "http://schemas.microsoft.com/office/word/2010/wordprocessingShape";
pub const NS_WML_GROUP: &str = 
    "http://schemas.microsoft.com/office/word/2010/wordprocessingGroup";
pub const NS_MATH: &str = 
    "http://schemas.openxmlformats.org/officeDocument/2006/math";

// PPTX-specific namespaces
pub const NS_PML: &str = 
    "http://schemas.openxmlformats.org/presentationml/2006/main";
pub const NS_PML_2010: &str = 
    "http://schemas.microsoft.com/office/powerpoint/2010/main";
pub const NS_PML_2012: &str = 
    "http://schemas.microsoft.com/office/powerpoint/2012/main";

// Package namespaces
pub const NS_CONTENT_TYPES: &str = 
    "http://schemas.openxmlformats.org/package/2006/content-types";
pub const NS_PACKAGE_RELATIONSHIPS: &str = 
    "http://schemas.openxmlformats.org/package/2006/relationships";
```

### relationships.rs

```rust
// DOCX relationships
pub const REL_STYLES: &str = 
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles";
pub const REL_NUMBERING: &str = 
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering";
pub const REL_FONT_TABLE: &str = 
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable";
pub const REL_SETTINGS: &str = 
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings";
pub const REL_HEADER: &str = 
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header";
pub const REL_FOOTER: &str = 
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer";

// PPTX relationships
pub const REL_SLIDE: &str = 
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide";
pub const REL_SLIDE_LAYOUT: &str = 
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout";
pub const REL_SLIDE_MASTER: &str = 
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster";
pub const REL_PRES_PROPS: &str = 
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps";
pub const REL_TABLE_STYLES: &str = 
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles";

// Shared relationships
pub const REL_OFFICE_DOCUMENT: &str = 
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
pub const REL_THEME: &str = 
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme";
pub const REL_IMAGE: &str = 
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
pub const REL_HYPERLINK: &str = 
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink";
pub const REL_CHART: &str = 
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart";
pub const REL_PACKAGE: &str = 
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package";

// Core properties
pub const REL_CORE_PROPERTIES: &str = 
    "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties";
pub const REL_EXTENDED_PROPERTIES: &str = 
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties";
pub const REL_CUSTOM_PROPERTIES: &str = 
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties";
```

### content_types.rs

```rust
// DOCX content types
pub const CT_DOCUMENT: &str = 
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
pub const CT_STYLES: &str = 
    "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml";
pub const CT_NUMBERING: &str = 
    "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml";
pub const CT_FONT_TABLE: &str = 
    "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml";
pub const CT_SETTINGS: &str = 
    "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml";
pub const CT_HEADER: &str = 
    "application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml";
pub const CT_FOOTER: &str = 
    "application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml";

// PPTX content types
pub const CT_PRESENTATION: &str = 
    "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml";
pub const CT_SLIDE: &str = 
    "application/vnd.openxmlformats-officedocument.presentationml.slide+xml";
pub const CT_SLIDE_LAYOUT: &str = 
    "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml";
pub const CT_SLIDE_MASTER: &str = 
    "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml";
pub const CT_THEME: &str = 
    "application/vnd.openxmlformats-officedocument.theme+xml";
pub const CT_PRES_PROPS: &str = 
    "application/vnd.openxmlformats-officedocument.presentationml.presProps+xml";
pub const CT_TABLE_STYLES: &str = 
    "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml";

// Shared content types
pub const CT_CHART: &str = 
    "application/vnd.openxmlformats-officedocument.drawingml.chart+xml";
pub const CT_CORE_PROPERTIES: &str = 
    "application/vnd.openxmlformats-package.core-properties+xml";
pub const CT_EXTENDED_PROPERTIES: &str = 
    "application/vnd.openxmlformats-officedocument.extended-properties+xml";
pub const CT_CUSTOM_PROPERTIES: &str = 
    "application/vnd.openxmlformats-officedocument.custom-properties+xml";
pub const CT_RELATIONSHIPS: &str = 
    "application/vnd.openxmlformats-package.relationships+xml";
pub const CT_CONTENT_TYPES: &str = 
    "application/vnd.openxmlformats-package.content-types+xml";

// Image content types
pub const CT_IMAGE_PNG: &str = "image/png";
pub const CT_IMAGE_JPEG: &str = "image/jpeg";
pub const CT_IMAGE_GIF: &str = "image/gif";
pub const CT_IMAGE_BMP: &str = "image/bmp";
pub const CT_IMAGE_TIFF: &str = "image/tiff";
pub const CT_IMAGE_SVG: &str = "image/svg+xml";

// Font content type
pub const CT_FONT_OBFUSCATED: &str = 
    "application/vnd.openxmlformats-officedocument.obfuscatedFont";

// Spreadsheet (for embedded xlsx in charts)
pub const CT_SPREADSHEET: &str = 
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";
```

### Usage Example

```rust
use crate::namespaces::*;
use crate::relationships::*;
use crate::content_types::*;

fn create_document_element() -> XmlElement {
    let mut doc = XmlElement::new("w:document");
    doc.set_namespace("w", NS_WML);
    doc.set_namespace("r", NS_RELATIONSHIPS);
    doc.set_namespace("w14", NS_WML_2010);
    doc.set_namespace("w15", NS_WML_2012);
    doc.set_namespace("wp", NS_WML_DRAWING);
    doc.set_namespace("a", NS_DRAWINGML);
    doc.set_namespace("pic", NS_DRAWINGML_PICTURE);
    doc.set_namespace("c", NS_DRAWINGML_CHART);
    doc.set_namespace("mc", NS_MARKUP_COMPAT);
    doc.set_attribute("mc:Ignorable", "w14 w15 wp14");
    doc
}

fn add_relationship(rel_id: &str, rel_type: &str, target: &str) -> XmlElement {
    let mut rel = XmlElement::new("Relationship");
    rel.set_attribute("Id", rel_id);
    rel.set_attribute("Type", rel_type);
    rel.set_attribute("Target", target);
    rel
}

fn add_content_type(part_name: &str, content_type: &str) -> XmlElement {
    let mut override_elem = XmlElement::new("Override");
    override_elem.set_attribute("PartName", part_name);
    override_elem.set_attribute("ContentType", content_type);
    override_elem
}
```

---

## Quick Reference Lookup

### By File Type

**DOCX files need:**
- `NS_WML`, `NS_RELATIONSHIPS`, `NS_DRAWINGML`, `NS_WML_DRAWING`

**PPTX files need:**
- `NS_PML`, `NS_RELATIONSHIPS`, `NS_DRAWINGML`

**Both need:**
- `NS_MARKUP_COMPAT` (for extensions)
- `NS_DRAWINGML_PICTURE` (for images)
- `NS_DRAWINGML_CHART` (for charts)

### By Feature

**Styles:**
- Namespace: `NS_WML`
- Relationship: `REL_STYLES`
- Content Type: `CT_STYLES`

**Charts:**
- Namespace: `NS_DRAWINGML_CHART`
- Relationship: `REL_CHART`
- Content Type: `CT_CHART`

**Images:**
- Namespace: `NS_DRAWINGML_PICTURE`
- Relationship: `REL_IMAGE`
- Content Type: `CT_IMAGE_PNG` (or JPEG, GIF, etc.)

**Metadata:**
- Namespaces: `NS_DUBLIN_CORE`, `NS_DUBLIN_CORE_TERMS`, `NS_CORE_PROPERTIES`
- Relationships: `REL_CORE_PROPERTIES`, `REL_EXTENDED_PROPERTIES`
- Content Types: `CT_CORE_PROPERTIES`, `CT_EXTENDED_PROPERTIES`
