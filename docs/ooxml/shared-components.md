# Shared OOXML Components

Components and markup patterns used across multiple OOXML formats (DOCX, PPTX, etc.).

## DrawingML (Namespace: `a`)

DrawingML is the shared graphics markup language used by DOCX, PPTX, and other Office formats for shapes, images, text formatting, and effects.

### Images (Blips)

**Purpose:** Embed or reference images in documents.

**Basic structure:**
```xml
<a:blip r:embed="rId4" cstate="print"/>
```

**With effects:**
```xml
<a:blip r:embed="rId4">
  <a:alphaModFix amt="50000"/>  <!-- 50% transparency -->
  <a:grayscale/>                 <!-- Grayscale filter -->
</a:blip>
```

**Attributes:**
- `r:embed` — relationship ID pointing to image file
- `r:link` — external URL (for linked images)
- `cstate` — compression state (print, screen)

**Supported formats:**
- JPEG (image/jpeg)
- PNG (image/png)
- GIF (image/gif)
- BMP (image/bmp)
- TIFF (image/tiff)
- WMF (image/x-wmf)
- EMF (image/x-emf)

### Tables (DrawingML)

**Purpose:** Embed tables in presentations and other formats.

**Basic structure:**
```xml
<a:tbl>
  <a:tblPr>
    <a:tableStyleId>{5C22544A-7EE6-4342-B048-85BDC9FD1C3A}</a:tableStyleId>
  </a:tblPr>
  <a:tblGrid>
    <a:gridCol w="2438400"/>
    <a:gridCol w="2438400"/>
  </a:tblGrid>
  <a:tr h="304800">
    <a:tc>
      <a:tcPr/>
      <a:txBody>
        <a:bodyPr/>
        <a:lstStyle/>
        <a:p>
          <a:r>
            <a:rPr/>
            <a:t>Cell content</a:t>
          </a:r>
        </a:p>
      </a:txBody>
    </a:tc>
  </a:tr>
</a:tbl>
```

**Key elements:**
- `<a:tblPr>` — table properties (style, borders, shading)
- `<a:tblGrid>` — column widths
- `<a:tr>` — table row
- `<a:tc>` — table cell
- `<a:tcPr>` — cell properties (background, borders, margins)

### Text Formatting

**Paragraph properties:**
```xml
<a:p>
  <a:pPr lvl="0" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="0">
    <a:lnSpc>
      <a:spcPts val="1200"/>  <!-- Line spacing in points -->
    </a:lnSpc>
    <a:spcBef>
      <a:spcPts val="0"/>     <!-- Space before paragraph -->
    </a:spcBef>
    <a:spcAft>
      <a:spcPts val="0"/>     <!-- Space after paragraph -->
    </a:spcAft>
    <a:buFont typeface="Wingdings" pitchFamily="2" charset="0"/>
    <a:buChar char="•"/>
    <a:buSzPct val="100000"/>
    <a:buClr>
      <a:schemeClr val="accent1"/>
    </a:buClr>
  </a:pPr>
  <a:r>
    <a:rPr lang="en-US" sz="2400" b="0" i="0" u="none" strike="sngStrike" kern="1200" baseline="0">
      <a:solidFill>
        <a:srgbClr val="000000"/>
      </a:solidFill>
      <a:latin typeface="Calibri"/>
      <a:ea typeface="Calibri"/>
      <a:cs typeface="Calibri"/>
    </a:rPr>
    <a:t>Text content</a:t>
  </a:r>
</a:p>
```

**Run properties (`<a:rPr>`):**
- `lang` — language code (e.g., "en-US")
- `sz` — font size in hundredths of a point
- `b` — bold (0 or 1)
- `i` — italic (0 or 1)
- `u` — underline (none, sng, dbl, heavy, dotted, dash, dashDot, lgDash, lgDashDot, lgDashDotDot, solid, sysDot, sysDash, dash, dotDash, lgDashDotDot, wavy, wavyHeavy, wavyDbl)
- `strike` — strikethrough (noStrike, sngStrike, dblStrike)
- `kern` — kerning in twips
- `baseline` — baseline shift in percentage

### Colors

**Solid fill:**
```xml
<a:solidFill>
  <a:srgbClr val="FF0000"/>  <!-- RGB color -->
</a:solidFill>
```

**Scheme color (theme-based):**
```xml
<a:solidFill>
  <a:schemeClr val="accent1">
    <a:lumMod val="50000"/>  <!-- 50% darker -->
  </a:schemeClr>
</a:solidFill>
```

**Gradient fill:**
```xml
<a:gradFill>
  <a:gsLst>
    <a:gs pos="0">
      <a:srgbClr val="FF0000"/>
    </a:gs>
    <a:gs pos="100000">
      <a:srgbClr val="0000FF"/>
    </a:gs>
  </a:gsLst>
  <a:lin ang="2700000" scaled="1"/>  <!-- Linear gradient, angle in 60000ths of degree -->
</a:gradFill>
```

**Pattern fill:**
```xml
<a:pattFill prst="ltDnDiag">  <!-- Light downward diagonal -->
  <a:fgClr>
    <a:srgbClr val="000000"/>
  </a:fgClr>
  <a:bgClr>
    <a:srgbClr val="FFFFFF"/>
  </a:bgClr>
</a:pattFill>
```

**Color schemes:**
- `dk1` — dark 1 (text)
- `lt1` — light 1 (background)
- `dk2` — dark 2 (lines)
- `lt2` — light 2 (fill)
- `accent1` through `accent6` — theme accent colors
- `hyperlink` — hyperlink color
- `folHyperlink` — followed hyperlink color

### Shapes and Geometry

**Basic shape:**
```xml
<a:prstGeom prst="rect">  <!-- Preset geometry: rect, ellipse, triangle, etc. -->
  <a:avLst/>              <!-- Adjustment value list (empty for standard shapes) -->
</a:prstGeom>
```

**Common preset geometries:**
- `rect` — rectangle
- `ellipse` — ellipse/circle
- `triangle` — triangle
- `diamond` — diamond
- `pentagon` — pentagon
- `hexagon` — hexagon
- `octagon` — octagon
- `star5` — 5-pointed star
- `star6` — 6-pointed star
- `star8` — 8-pointed star
- `roundRect` — rounded rectangle
- `flowChartProcess` — flowchart process box
- `flowChartDecision` — flowchart decision diamond
- `flowChartTerminator` — flowchart terminator

### Effects

**Shadow:**
```xml
<a:effectLst>
  <a:outerShdw blurRad="50800" dist="38100" dir="2700000" algn="tl" rotWithShape="0">
    <a:srgbClr val="000000">
      <a:alpha val="50000"/>  <!-- 50% opacity -->
    </a:srgbClr>
  </a:outerShdw>
</a:effectLst>
```

**Reflection:**
```xml
<a:effectLst>
  <a:reflection blurRad="0" dist="38100" dir="2700000" fadeDir="5400000" rotWithShape="0" endA="38000" endPos="0"/>
</a:effectLst>
```

**Glow:**
```xml
<a:effectLst>
  <a:glow rad="50800">
    <a:srgbClr val="FF0000">
      <a:alpha val="100000"/>
    </a:srgbClr>
  </a:glow>
</a:effectLst>
```

## Chart Markup (Namespace: `c`)

Charts are embedded as separate XML files and referenced from documents/presentations.

### Chart File Structure

**Location:** `ppt/charts/chart1.xml` (PPTX) or `word/charts/chart1.xml` (DOCX)

**Basic structure:**
```xml
<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"
              xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
              xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <c:chart>
    <c:barChart>
      <!-- Chart-specific content -->
    </c:barChart>
  </c:chart>
</c:chartSpace>
```

### Chart Types

**Bar Chart:**
```xml
<c:barChart>
  <c:barDir val="col"/>  <!-- col (vertical) or bar (horizontal) -->
  <c:ser>
    <c:idx val="0"/>
    <c:order val="0"/>
    <c:val>
      <c:numRef>
        <c:f>Sheet1!$B$2:$B$5</c:f>
      </c:numRef>
    </c:val>
  </c:ser>
</c:barChart>
```

**Line Chart:**
```xml
<c:lineChart>
  <c:grouping val="standard"/>  <!-- standard, percentStacked, stacked -->
  <c:ser>
    <c:idx val="0"/>
    <c:order val="0"/>
    <c:val>
      <c:numRef>
        <c:f>Sheet1!$B$2:$B$5</c:f>
      </c:numRef>
    </c:val>
  </c:ser>
</c:lineChart>
```

**Pie Chart:**
```xml
<c:pieChart>
  <c:ser>
    <c:idx val="0"/>
    <c:order val="0"/>
    <c:val>
      <c:numRef>
        <c:f>Sheet1!$B$2:$B$5</c:f>
      </c:numRef>
    </c:val>
  </c:ser>
</c:pieChart>
```

### Chart Data

**Inline data:**
```xml
<c:val>
  <c:numRef>
    <c:numCache>
      <c:formatCode>General</c:formatCode>
      <c:pt idx="0">
        <c:v>10</c:v>
      </c:pt>
      <c:pt idx="1">
        <c:v>20</c:v>
      </c:pt>
      <c:pt idx="2">
        <c:v>15</c:v>
      </c:pt>
    </c:numCache>
  </c:numRef>
</c:val>
```

**Categories:**
```xml
<c:cat>
  <c:strRef>
    <c:strCache>
      <c:formatCode>General</c:formatCode>
      <c:pt idx="0">
        <c:v>Category A</c:v>
      </c:pt>
      <c:pt idx="1">
        <c:v>Category B</c:v>
      </c:pt>
      <c:pt idx="2">
        <c:v>Category C</c:v>
      </c:pt>
    </c:strCache>
  </c:strRef>
</c:cat>
```

## Relationships System

### Relationship Types

**Common relationship types:**

| Type | Purpose | Target |
|------|---------|--------|
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument` | Main document | word/document.xml or ppt/presentation.xml |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/image` | Image | media/image1.png |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink` | Hyperlink | External URL |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart` | Chart | charts/chart1.xml |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/header` | Header | word/header1.xml |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer` | Footer | word/footer1.xml |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide` | Slide | slides/slide1.xml |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout` | Slide layout | slideLayouts/slideLayout1.xml |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster` | Slide master | slideMasters/slideMaster1.xml |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme` | Theme | theme/theme1.xml |
| `http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties` | Core metadata | docProps/core.xml |
| `http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties` | Extended metadata | docProps/app.xml |

### Relationship ID Generation

Relationship IDs are arbitrary strings, typically `rId1`, `rId2`, etc.

**Best practice:**
- Start with `rId1` for the first relationship
- Increment for each new relationship
- Keep a counter to avoid duplicates
- Maintain a map of ID → target for reference

### External vs. Internal Relationships

**External (hyperlink):**
```xml
<Relationship Id="rId5"
              Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink"
              Target="https://example.com"
              TargetMode="External"/>
```

**Internal (image):**
```xml
<Relationship Id="rId4"
              Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image"
              Target="media/image1.png"/>
```

## Content Types Registry

### [Content_Types].xml Structure

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <!-- Default types by extension -->
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Default Extension="jpeg" ContentType="image/jpeg"/>
  <Default Extension="png" ContentType="image/png"/>
  <Default Extension="gif" ContentType="image/gif"/>
  <Default Extension="bmp" ContentType="image/bmp"/>

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
  <Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>
  <Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.custom-properties+xml"/>
</Types>
```

### Content Type Naming Convention

**Pattern:** `application/vnd.openxmlformats-[package|officedocument].[format].[part]+xml`

**Examples:**
- `application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml` — DOCX main document
- `application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml` — PPTX main presentation
- `application/vnd.openxmlformats-officedocument.drawingml.chart+xml` — Chart
- `application/vnd.openxmlformats-package.core-properties+xml` — Core metadata

## Theme System

**Location:** `ppt/theme/theme1.xml` (PPTX) or `word/theme/theme1.xml` (DOCX)

**Structure:**
```xml
<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" name="Office Theme">
  <a:themeElements>
    <a:clrScheme name="Office">
      <!-- Color scheme -->
    </a:clrScheme>
    <a:fontScheme name="Office">
      <!-- Font scheme -->
    </a:fontScheme>
    <a:fmtScheme name="Office">
      <!-- Format scheme (fills, lines, effects) -->
    </a:fmtScheme>
  </a:themeElements>
</a:theme>
```

**Color scheme colors:**
- `dk1`, `lt1` — dark/light 1 (text/background)
- `dk2`, `lt2` — dark/light 2 (lines/fill)
- `accent1` through `accent6` — accent colors
- `hyperlink`, `folHyperlink` — link colors

**Font scheme:**
- `majorFont` — used for headings
- `minorFont` — used for body text

## Summary

Shared components across OOXML formats:
- **DrawingML** — graphics, shapes, images, text formatting
- **Chart markup** — bar, line, pie, scatter charts
- **Relationships** — link parts together
- **Content types** — MIME type registry
- **Themes** — color and font schemes
- **Metadata** — core properties, extended properties

These components enable consistent handling of complex content across different Office formats.
