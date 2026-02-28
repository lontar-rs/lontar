# Minimal PPTX Structure

## Overview

A minimal PPTX file contains the absolute minimum XML structure required to be recognized as a valid Office Open XML presentation. This document deconstructs a "Hello World" PPTX with a single title slide to understand the required files and their purposes.

## File Structure

```
hello_world.pptx (ZIP archive)
├── [Content_Types].xml
├── _rels/.rels
├── ppt/
│   ├── presentation.xml
│   ├── presProps.xml
│   ├── tableStyles.xml
│   ├── slides/
│   │   ├── slide1.xml
│   │   └── _rels/
│   │       └── slide1.xml.rels
│   ├── slideLayouts/
│   │   ├── slideLayout1.xml
│   │   └── _rels/
│   │       └── slideLayout1.xml.rels
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

## File Breakdown

### [Content_Types].xml

**Purpose:** MIME type declarations for all files in the package.

**Minimal content:**
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Default Extension="jpeg" ContentType="image/jpeg"/>
  <Default Extension="png" ContentType="image/png"/>
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

### _rels/.rels

**Purpose:** Package-level relationships.

**Minimal content:**
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="ppt/presentation.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml"/>
  <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" Target="docProps/app.xml"/>
</Relationships>
```

### ppt/presentation.xml

**Purpose:** Root presentation element. Links to slides and slide masters.

**Minimal content:**
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
                xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <p:sldIdLst>
    <p:sldId id="256" r:id="rId2"/>
  </p:sldIdLst>
  <p:sldSz cx="9144000" cy="6858000"/>
  <p:notesSz cx="6858000" cy="9144000"/>
</p:presentation>
```

**Key elements:**
- `<p:sldIdLst>` — list of slides in the presentation
- `<p:sldId>` — reference to a slide (id is arbitrary, r:id points to relationship)
- `<p:sldSz>` — slide dimensions in EMU (9144000 × 6858000 = 10" × 7.5" standard)
- `<p:notesSz>` — notes page dimensions

### ppt/presProps.xml

**Purpose:** Presentation properties (show settings, etc.).

**Minimal content:**
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presProps xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
             xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"/>
```

### ppt/tableStyles.xml

**Purpose:** Table style definitions.

**Minimal content:**
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:tblStyleLst xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
               def="0"/>
```

### ppt/slides/slide1.xml

**Purpose:** Content of the first slide.

**Minimal content (title slide with text):**
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
       xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
  <p:cSld>
    <p:bg>
      <p:bgPr>
        <a:solidFill>
          <a:srgbClr val="FFFFFF"/>
        </a:solidFill>
        <a:effectLst/>
      </p:bgPr>
    </p:bg>
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name="Title 1"/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr>
        <a:xfrm>
          <a:off x="0" y="0"/>
          <a:ext cx="9144000" cy="6858000"/>
          <a:chOff x="0" y="0"/>
          <a:chExt cx="9144000" cy="6858000"/>
        </a:xfrm>
      </p:grpSpPr>
      
      <!-- Title shape -->
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="Title 1"/>
          <p:cNvSpPr txBody="1"/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm>
            <a:off x="457200" y="457200"/>
            <a:ext cx="8229600" cy="1371600"/>
          </a:xfrm>
        </p:spPr>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p>
            <a:r>
              <a:rPr lang="en-US" sz="4400" b="1"/>
              <a:t>Hello World</a:t>
            </a:r>
          </a:p>
        </p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr>
    <a:masterClrMapping/>
  </p:clrMapOvr>
</p:sld>
```

**Key elements:**
- `<p:cSld>` — slide content
- `<p:bg>` — background (white fill)
- `<p:spTree>` — shape tree (all shapes on the slide)
- `<p:sp>` — shape (text box, image, etc.)
- `<p:txBody>` — text body (contains paragraphs and runs)
- `<a:xfrm>` — transform (position and size in EMU)
- `<a:off>` — offset (x, y position)
- `<a:ext>` — extent (width, height)

### ppt/slides/_rels/slide1.xml.rels

**Purpose:** Slide-level relationships (images, hyperlinks, etc.).

**Minimal content (no external resources):**
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout1.xml"/>
</Relationships>
```

### ppt/slideLayouts/slideLayout1.xml

**Purpose:** Layout definition for slides (defines placeholder regions).

**Minimal content (title slide layout):**
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
             xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
             xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
             type="title">
  <p:cSld>
    <p:bg>
      <p:bgPr>
        <a:solidFill>
          <a:srgbClr val="FFFFFF"/>
        </a:solidFill>
        <a:effectLst/>
      </p:bgPr>
    </p:bg>
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name="Title 1"/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr>
        <a:xfrm>
          <a:off x="0" y="0"/>
          <a:ext cx="9144000" cy="6858000"/>
          <a:chOff x="0" y="0"/>
          <a:chExt cx="9144000" cy="6858000"/>
        </a:xfrm>
      </p:grpSpPr>
      
      <!-- Title placeholder -->
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="Title 1"/>
          <p:cNvSpPr txBody="1"/>
          <p:nvPr>
            <p:ph type="ctrTitle"/>
          </p:nvPr>
        </p:nvSpPr>
        <p:spPr/>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
        </p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr>
    <a:masterClrMapping/>
  </p:clrMapOvr>
</p:sldLayout>
```

### ppt/slideLayouts/_rels/slideLayout1.xml.rels

**Purpose:** Layout relationships (points to slide master).

**Minimal content:**
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="../slideMasters/slideMaster1.xml"/>
</Relationships>
```

### ppt/slideMasters/slideMaster1.xml

**Purpose:** Master slide template (defines default formatting for all slides).

**Minimal content:**
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldMaster xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
             xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
             xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
  <p:cSld>
    <p:bg>
      <p:bgPr>
        <a:solidFill>
          <a:srgbClr val="FFFFFF"/>
        </a:solidFill>
        <a:effectLst/>
      </p:bgPr>
    </p:bg>
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name="Title 1"/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr>
        <a:xfrm>
          <a:off x="0" y="0"/>
          <a:ext cx="9144000" cy="6858000"/>
          <a:chOff x="0" y="0"/>
          <a:chExt cx="9144000" cy="6858000"/>
        </a:xfrm>
      </p:grpSpPr>
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr>
    <a:masterClrMapping/>
  </p:clrMapOvr>
</p:sldMaster>
```

### ppt/slideMasters/_rels/slideMaster1.xml.rels

**Purpose:** Master relationships (points to theme).

**Minimal content:**
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="../theme/theme1.xml"/>
</Relationships>
```

### ppt/theme/theme1.xml

**Purpose:** Color and font theme.

**Minimal content:**
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" name="Office Theme">
  <a:themeElements>
    <a:clrScheme name="Office">
      <a:dk1>
        <a:srgbClr val="000000"/>
      </a:dk1>
      <a:lt1>
        <a:srgbClr val="FFFFFF"/>
      </a:lt1>
      <a:dk2>
        <a:srgbClr val="1F497D"/>
      </a:dk2>
      <a:lt2>
        <a:srgbClr val="EBEBEB"/>
      </a:lt2>
      <a:accent1>
        <a:srgbClr val="4472C4"/>
      </a:accent1>
      <a:accent2>
        <a:srgbClr val="ED7D31"/>
      </a:accent2>
      <a:accent3>
        <a:srgbClr val="A5A5A5"/>
      </a:accent3>
      <a:accent4>
        <a:srgbClr val="FFC000"/>
      </a:accent4>
      <a:accent5>
        <a:srgbClr val="5B9BD5"/>
      </a:accent5>
      <a:accent6>
        <a:srgbClr val="70AD47"/>
      </a:accent6>
      <a:hyperlink>
        <a:srgbClr val="0563C1"/>
      </a:hyperlink>
      <a:folHyperlink>
        <a:srgbClr val="954F72"/>
      </a:folHyperlink>
    </a:clrScheme>
    <a:fontScheme name="Office">
      <a:majorFont>
        <a:latin typeface="Calibri Light"/>
        <a:ea typeface=""/>
        <a:cs typeface=""/>
      </a:majorFont>
      <a:minorFont>
        <a:latin typeface="Calibri"/>
        <a:ea typeface=""/>
        <a:cs typeface=""/>
      </a:minorFont>
    </a:fontScheme>
    <a:fmtScheme name="Office">
      <a:fillStyleLst>
        <a:solidFill>
          <a:srgbClr val="FFFFFF"/>
        </a:solidFill>
        <a:gradFill>
          <a:gsLst>
            <a:gs pos="0">
              <a:srgbClr val="FFFFFF"/>
            </a:gs>
            <a:gs pos="100000">
              <a:srgbClr val="FFFFFF"/>
            </a:gs>
          </a:gsLst>
          <a:lin ang="2700000" scaled="1"/>
        </a:gradFill>
        <a:pattFill prst="ltDnDiag">
          <a:fgClr>
            <a:srgbClr val="FFFFFF"/>
          </a:fgClr>
          <a:bgClr>
            <a:srgbClr val="FFFFFF"/>
          </a:bgClr>
        </a:pattFill>
      </a:fillStyleLst>
      <a:lnStyleLst>
        <a:ln w="9525">
          <a:solidFill>
            <a:srgbClr val="FFFFFF"/>
          </a:solidFill>
          <a:prstDash val="solid"/>
          <a:round/>
        </a:ln>
        <a:ln w="25400">
          <a:solidFill>
            <a:srgbClr val="FFFFFF"/>
          </a:solidFill>
          <a:prstDash val="solid"/>
          <a:round/>
        </a:ln>
        <a:ln w="38100">
          <a:solidFill>
            <a:srgbClr val="FFFFFF"/>
          </a:solidFill>
          <a:prstDash val="solid"/>
          <a:round/>
        </a:ln>
      </a:lnStyleLst>
      <a:effectStyleLst>
        <a:effectLst/>
        <a:effectLst/>
        <a:effectLst/>
      </a:effectStyleLst>
      <a:bgFillStyleLst>
        <a:solidFill>
          <a:srgbClr val="FFFFFF"/>
        </a:solidFill>
        <a:solidFill>
          <a:srgbClr val="FFFFFF"/>
        </a:solidFill>
        <a:solidFill>
          <a:srgbClr val="FFFFFF"/>
        </a:solidFill>
      </a:bgFillStyleLst>
    </a:fmtScheme>
  </a:themeElements>
</a:theme>
```

### ppt/_rels/presentation.xml.rels

**Purpose:** Presentation relationships.

**Minimal content:**
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide1.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="slideMasters/slideMaster1.xml"/>
  <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps" Target="presProps.xml"/>
  <Relationship Id="rId4" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles" Target="tableStyles.xml"/>
  <Relationship Id="rId5" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="theme/theme1.xml"/>
</Relationships>
```

### docProps/core.xml

**Purpose:** Document metadata.

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

### docProps/app.xml

**Purpose:** Application metadata.

**Minimal content:**
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties"
            xmlns:vt="http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes">
  <TotalTime>0</TotalTime>
  <Application>Lontar</Application>
  <AppVersion>0.1.0</AppVersion>
</Properties>
```

## EMU Coordinate System

All positioning in PPTX uses EMU (English Metric Units):
- **1 EMU = 1/914,400 inch**
- **1 inch = 914,400 EMU**
- **1 cm = 360,000 EMU**

Standard slide dimensions:
- **Standard (4:3):** 9,144,000 × 6,858,000 EMU (10" × 7.5")
- **Widescreen (16:9):** 9,144,000 × 5,143,500 EMU (10" × 5.625")

## Hierarchy

PPTX follows a strict hierarchy:
```
Presentation
  ├── Slide Master
  │   └── Theme
  ├── Slide Layout (references Slide Master)
  └── Slide (references Slide Layout)
```

Each slide inherits formatting from its layout, which inherits from the master.

## Summary

A minimal PPTX requires:
1. **[Content_Types].xml** — MIME type registry
2. **_rels/.rels** — package relationships
3. **ppt/presentation.xml** — root presentation
4. **ppt/presProps.xml** — presentation properties
5. **ppt/tableStyles.xml** — table styles
6. **ppt/slides/slide1.xml** — slide content
7. **ppt/slides/_rels/slide1.xml.rels** — slide relationships
8. **ppt/slideLayouts/slideLayout1.xml** — layout definition
9. **ppt/slideLayouts/_rels/slideLayout1.xml.rels** — layout relationships
10. **ppt/slideMasters/slideMaster1.xml** — master template
11. **ppt/slideMasters/_rels/slideMaster1.xml.rels** — master relationships
12. **ppt/theme/theme1.xml** — color and font theme
13. **ppt/_rels/presentation.xml.rels** — presentation relationships
14. **docProps/core.xml** — metadata
15. **docProps/app.xml** — application properties

All files are XML with specific namespaces. The entire package is a ZIP archive.
