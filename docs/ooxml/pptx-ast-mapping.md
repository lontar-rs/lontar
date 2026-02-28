# PPTX AST Mapping

Complete mapping between Lontar's AST nodes and OOXML PresentationML elements. PPTX differs from DOCX in that it uses spatial positioning (EMU coordinates) rather than flow-based layout.

## Slide Types and Layouts

| AST Node | PPTX Layout Type | Notes |
|----------|------------------|-------|
| `Block::Slide { layout: Title }` | `slideLayout1.xml` (Title Slide) | Single title placeholder, centered |
| `Block::Slide { layout: TitleContent }` | `slideLayout2.xml` (Title and Content) | Title at top, content area below |
| `Block::Slide { layout: TitleOnly }` | `slideLayout3.xml` (Title Only) | Title placeholder only |
| `Block::Slide { layout: Blank }` | `slideLayout6.xml` (Blank) | No predefined placeholders |
| `Block::Slide { layout: TwoColumn }` | `slideLayout4.xml` (Two Content) | Title + two content areas side-by-side |

## Text Content in Slides

### Headings (Title Placeholders)

| AST Node | PPTX Element | Positioning | Notes |
|----------|--------------|-------------|-------|
| `Block::Heading { level: 1, content }` | `<p:sp>` with title placeholder | Top of slide, full width | Maps to title placeholder |
| | `<p:txBody><a:p><a:r><a:rPr sz="4400" b="1"/><a:t>text</a:t></a:r></a:p></p:txBody>` | | Large bold text |

### Body Text (Content Placeholders)

| AST Node | PPTX Element | Positioning | Notes |
|----------|--------------|-------------|-------|
| `Block::Paragraph { content, style }` | `<p:sp>` with content placeholder | Content area | Auto-flows in placeholder |
| | `<p:txBody><a:p><a:r><a:rPr/><a:t>text</a:t></a:r></a:p></p:txBody>` | | Regular body text |

### Text Formatting (Inline Styles)

| AST Property | PPTX Element | Notes |
|--------------|--------------|-------|
| `bold: true` | `<a:rPr b="1"/>` | Bold flag |
| `italic: true` | `<a:rPr i="1"/>` | Italic flag |
| `underline: true` | `<a:rPr u="sng"/>` | Underline type (sng, dbl, etc.) |
| `color: "FF0000"` | `<a:rPr><a:solidFill><a:srgbClr val="FF0000"/></a:solidFill></a:rPr>` | RGB color |
| `font_size: 12` | `<a:rPr sz="2400"/>` | Size in hundredths of a point (12pt = 2400) |
| `font_name: "Arial"` | `<a:rPr latin typeface="Arial"/>` | Font family |

### Text Run Example

```xml
<a:p>
  <a:r>
    <a:rPr lang="en-US" sz="2400" b="1">
      <a:solidFill>
        <a:srgbClr val="0563C1"/>
      </a:solidFill>
    </a:rPr>
    <a:t>Bold blue text</a:t>
  </a:r>
</a:p>
```

## Lists

| AST Node | PPTX Element | Notes |
|----------|--------------|-------|
| `Block::List { ordered: true, items }` | `<a:p><a:pPr lvl="0"><a:buFont typeface="Wingdings"/><a:buChar char="1."/></a:pPr>...</a:p>` | Numbered list |
| `Block::List { ordered: false, items }` | `<a:p><a:pPr lvl="0"><a:buFont typeface="Wingdings"/><a:buChar char="•"/></a:pPr>...</a:p>` | Bulleted list |
| Nested list (level 1) | `<a:pPr lvl="1">` | Indentation level |

### List Example

```xml
<!-- Bulleted list -->
<a:p>
  <a:pPr lvl="0">
    <a:buFont typeface="Wingdings"/>
    <a:buChar char="•"/>
    <a:buSzPct val="100000"/>
    <a:buClr>
      <a:schemeClr val="accent1"/>
    </a:buClr>
  </a:pPr>
  <a:r>
    <a:rPr lang="en-US" dirty="0" smtClean="0"/>
    <a:t>First item</a:t>
  </a:r>
</a:p>

<!-- Numbered list -->
<a:p>
  <a:pPr lvl="0">
    <a:buFont typeface="Calibri"/>
    <a:buChar char="1."/>
    <a:buSzPct val="100000"/>
    <a:buClr>
      <a:schemeClr val="accent1"/>
    </a:buClr>
  </a:pPr>
  <a:r>
    <a:rPr lang="en-US" dirty="0" smtClean="0"/>
    <a:t>First item</a:t>
  </a:r>
</a:p>
```

## Tables

| AST Node | PPTX Element | Notes |
|----------|--------------|-------|
| `Block::Table { headers, rows }` | `<p:graphicFrame><a:graphic><a:graphicData><a:tbl>...</a:tbl></a:graphicData></a:graphic></p:graphicFrame>` | DrawingML table |
| Header row | `<a:tr><a:tc><a:tcPr><a:solidFill><a:schemeClr val="accent1"/></a:solidFill></a:tcPr>...</a:tc></a:tr>` | Styled header |
| Data row | `<a:tr><a:tc>...</a:tc></a:tr>` | Regular row |
| Cell | `<a:tc><a:tcPr>...</a:tcPr><a:txBody><a:p>...</a:p></a:txBody></a:tc>` | Table cell |

### Table Example

```xml
<p:graphicFrame>
  <p:nvGraphicFramePr>
    <p:cNvPr id="2" name="Table 1"/>
    <p:cNvGraphicFramePr/>
    <p:nvPr/>
  </p:nvGraphicFramePr>
  <p:xfrm>
    <a:off x="914400" y="1828800"/>
    <a:ext cx="7315200" cy="2743200"/>
  </p:xfrm>
  <a:graphic>
    <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/table">
      <a:tbl>
        <a:tblPr>
          <a:tableStyleId>{5C22544A-7EE6-4342-B048-85BDC9FD1C3A}</a:tableStyleId>
        </a:tblPr>
        <a:tblGrid>
          <a:gridCol w="2438400"/>
          <a:gridCol w="2438400"/>
          <a:gridCol w="2438400"/>
        </a:tblGrid>
        <!-- Header row -->
        <a:tr h="304800">
          <a:tc>
            <a:tcPr>
              <a:solidFill>
                <a:schemeClr val="accent1"/>
              </a:solidFill>
            </a:tcPr>
            <a:txBody>
              <a:bodyPr/>
              <a:lstStyle/>
              <a:p>
                <a:r>
                  <a:rPr lang="en-US" b="1" sz="2400"/>
                  <a:t>Header 1</a:t>
                </a:r>
              </a:p>
            </a:txBody>
          </a:tc>
          <a:tc>
            <a:tcPr>
              <a:solidFill>
                <a:schemeClr val="accent1"/>
              </a:solidFill>
            </a:tcPr>
            <a:txBody>
              <a:bodyPr/>
              <a:lstStyle/>
              <a:p>
                <a:r>
                  <a:rPr lang="en-US" b="1" sz="2400"/>
                  <a:t>Header 2</a:t>
                </a:r>
              </a:p>
            </a:txBody>
          </a:tc>
          <a:tc>
            <a:tcPr>
              <a:solidFill>
                <a:schemeClr val="accent1"/>
              </a:solidFill>
            </a:tcPr>
            <a:txBody>
              <a:bodyPr/>
              <a:lstStyle/>
              <a:p>
                <a:r>
                  <a:rPr lang="en-US" b="1" sz="2400"/>
                  <a:t>Header 3</a:t>
                </a:r>
              </a:p>
            </a:txBody>
          </a:tc>
        </a:tr>
        <!-- Data row -->
        <a:tr h="304800">
          <a:tc>
            <a:tcPr/>
            <a:txBody>
              <a:bodyPr/>
              <a:lstStyle/>
              <a:p>
                <a:r>
                  <a:rPr lang="en-US" sz="2400"/>
                  <a:t>Data 1</a:t>
                </a:r>
              </a:p>
            </a:txBody>
          </a:tc>
          <a:tc>
            <a:tcPr/>
            <a:txBody>
              <a:bodyPr/>
              <a:lstStyle/>
              <a:p>
                <a:r>
                  <a:rPr lang="en-US" sz="2400"/>
                  <a:t>Data 2</a:t>
                </a:r>
              </a:p>
            </a:txBody>
          </a:tc>
          <a:tc>
            <a:tcPr/>
            <a:txBody>
              <a:bodyPr/>
              <a:lstStyle/>
              <a:p>
                <a:r>
                  <a:rPr lang="en-US" sz="2400"/>
                  <a:t>Data 3</a:t>
                </a:r>
              </a:p>
            </a:txBody>
          </a:tc>
        </a:tr>
      </a:tbl>
    </a:graphicData>
  </a:graphic>
</p:graphicFrame>
```

## Images

| AST Node | PPTX Element | Notes |
|----------|--------------|-------|
| `Block::Image { source, width, height }` | `<p:graphicFrame><a:graphic><a:graphicData><pic:pic>...</pic:pic></a:graphicData></a:graphic></p:graphicFrame>` | Positioned image |

### Image Example

```xml
<p:graphicFrame>
  <p:nvGraphicFramePr>
    <p:cNvPr id="3" name="Picture 1"/>
    <p:cNvGraphicFramePr/>
    <p:nvPr/>
  </p:nvGraphicFramePr>
  <p:xfrm>
    <a:off x="914400" y="1828800"/>
    <a:ext cx="3657600" cy="2743200"/>
  </p:xfrm>
  <a:graphic>
    <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/picture">
      <pic:pic xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture">
        <pic:nvPicPr>
          <pic:cNvPr id="0" name="image1.png"/>
          <pic:cNvPicPr/>
        </pic:nvPicPr>
        <pic:blipFill>
          <a:blip r:embed="rId2"/>
          <a:stretch>
            <a:fillRect/>
          </a:stretch>
        </pic:blipFill>
        <pic:spPr>
          <a:xfrm>
            <a:off x="0" y="0"/>
            <a:ext cx="3657600" cy="2743200"/>
          </a:xfrm>
          <a:prstGeom prst="rect">
            <a:avLst/>
          </a:prstGeom>
        </pic:spPr>
      </pic:pic>
    </a:graphicData>
  </a:graphic>
</p:graphicFrame>
```

## Charts

| AST Node | PPTX Element | Notes |
|----------|--------------|-------|
| `Block::Chart { kind: Bar, data }` | `<p:graphicFrame><a:graphic><a:graphicData><c:chartSpace>...</c:chartSpace></a:graphicData></a:graphic></p:graphicFrame>` | Bar chart |
| `Block::Chart { kind: Line, data }` | Chart with `<c:lineChart>` | Line chart |
| `Block::Chart { kind: Pie, data }` | Chart with `<c:pieChart>` | Pie chart |

### Chart Example (Bar Chart)

```xml
<p:graphicFrame>
  <p:nvGraphicFramePr>
    <p:cNvPr id="4" name="Chart 1"/>
    <p:cNvGraphicFramePr/>
    <p:nvPr/>
  </p:nvGraphicFramePr>
  <p:xfrm>
    <a:off x="914400" y="1828800"/>
    <a:ext cx="5486400" cy="3657600"/>
  </p:xfrm>
  <a:graphic>
    <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/chart">
      <c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart">
        <c:chart>
          <c:barChart>
            <c:barDir val="col"/>
            <c:ser>
              <c:idx val="0"/>
              <c:order val="0"/>
              <c:dLbls>
                <c:showVal val="1"/>
              </c:dLbls>
              <c:val>
                <c:numRef>
                  <c:f>Sheet1!$B$2:$B$5</c:f>
                </c:numRef>
              </c:val>
            </c:ser>
            <c:catAx>
              <c:axId val="1"/>
              <c:scaling>
                <c:orientation val="minMax"/>
              </c:scaling>
              <c:delete val="0"/>
              <c:axPos val="b"/>
              <c:tickLblPos val="low"/>
              <c:crossAx val="2"/>
              <c:crosses val="autoZero"/>
              <c:auto val="1"/>
              <c:lblAlgn val="ctr"/>
              <c:lblOffset val="100"/>
            </c:catAx>
            <c:valAx>
              <c:axId val="2"/>
              <c:scaling>
                <c:orientation val="minMax"/>
              </c:scaling>
              <c:delete val="0"/>
              <c:axPos val="l"/>
              <c:majorGridlines/>
              <c:tickLblPos val="low"/>
              <c:crossAx val="1"/>
              <c:crosses val="autoZero"/>
              <c:crossesAt val="0"/>
              <c:auto val="1"/>
              <c:autoZero val="1"/>
            </c:valAx>
          </c:barChart>
        </c:chart>
      </c:chartSpace>
    </a:graphicData>
  </a:graphic>
</p:graphicFrame>
```

## Positioning and Layout

### EMU Coordinate System

All positioning uses EMU (English Metric Units):
- **1 EMU = 1/914,400 inch**
- **1 inch = 914,400 EMU**
- **1 cm = 360,000 EMU**

### Shape Positioning

```xml
<p:xfrm>
  <a:off x="914400" y="914400"/>      <!-- 1" from left, 1" from top -->
  <a:ext cx="4572000" cy="1828800"/>  <!-- 5" wide, 2" tall -->
</p:xfrm>
```

### Standard Slide Dimensions

| Format | Width | Height | EMU Width | EMU Height |
|--------|-------|--------|-----------|------------|
| Standard (4:3) | 10" | 7.5" | 9,144,000 | 6,858,000 |
| Widescreen (16:9) | 10" | 5.625" | 9,144,000 | 5,143,500 |
| Custom | varies | varies | varies | varies |

## Placeholder Types

| Type | Purpose | Typical Position |
|------|---------|------------------|
| `ctrTitle` | Center title | Center of slide |
| `body` | Body text | Below title |
| `obj` | Object (table, chart, image) | Content area |
| `pic` | Picture | Content area |
| `tbl` | Table | Content area |
| `chart` | Chart | Content area |

## Speaker Notes

```xml
<!-- In notesSlide1.xml -->
<p:notes xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
         xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name="Notes 1"/>
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
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="Notes Text"/>
          <p:cNvSpPr txBody="1"/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p>
            <a:r>
              <a:rPr lang="en-US" sz="2400"/>
              <a:t>Speaker notes go here</a:t>
            </a:r>
          </a:p>
        </p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
</p:notes>
```

## Summary Table

| AST Category | PPTX Namespace | Key Elements |
|--------------|----------------|--------------|
| Slides | `p` (PresentationML) | `<p:sld>`, `<p:cSld>`, `<p:spTree>` |
| Shapes/Text | `p`, `a` (DrawingML) | `<p:sp>`, `<p:txBody>`, `<a:p>`, `<a:r>` |
| Positioning | `a` | `<p:xfrm>`, `<a:off>`, `<a:ext>` |
| Tables | `a` | `<a:tbl>`, `<a:tr>`, `<a:tc>` |
| Images | `p`, `a`, `pic` | `<p:graphicFrame>`, `<pic:pic>`, `<a:blip>` |
| Charts | `p`, `a`, `c` | `<p:graphicFrame>`, `<c:chartSpace>` |
| Text formatting | `a` | `<a:rPr>`, `<a:solidFill>`, `<a:srgbClr>` |

## Implementation Strategy

1. **Determine slide layout** — based on content types
2. **Create slide XML** — with shape tree and placeholders
3. **Position shapes** — using EMU coordinates
4. **Add text** — with proper formatting runs
5. **Handle media** — images and charts as graphic frames
6. **Create relationships** — link to layouts, masters, media
7. **ZIP package** — collect all files into PPTX archive

## Key Differences from DOCX

- **Spatial layout** — explicit x, y, width, height in EMU
- **Placeholders** — predefined regions in layouts
- **Master/Layout hierarchy** — slides inherit from layouts which inherit from masters
- **DrawingML** — shared graphics markup for shapes, images, charts
- **No flow** — content doesn't automatically reflow; must be positioned explicitly
