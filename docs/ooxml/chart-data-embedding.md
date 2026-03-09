# Chart Data Embedding in OOXML (DOCX and PPTX)

## Overview

Charts in OOXML documents use a dual-file structure: a chart definition XML file plus an optional embedded spreadsheet containing source data. This document covers chart architecture, data storage, styling, and integration with Lontar's chart generation.

---

## 1. Chart Architecture Overview

### Dual-File Structure

OOXML charts consist of two components:

1. **Chart Definition** (`chart1.xml`) — Chart structure, series, axes, styling
2. **Embedded Spreadsheet** (`chart1.xlsx`) — Source data for editing in Office (optional)

**Why embed a full xlsx?**
- Allows users to edit chart data directly in Office applications
- Preserves data when opening in different Office versions
- Enables data-driven updates

**For Lontar:** Start with inline data (no xlsx) for simplicity. Add xlsx embedding in Phase 3 for Office editing support.

### File Structure for DOCX with One Chart

```
document.docx (ZIP archive)
├── [Content_Types].xml
├── _rels/.rels
├── word/
│   ├── document.xml
│   ├── charts/
│   │   ├── chart1.xml          ← Chart definition
│   │   ├── chart1.xlsx         ← Embedded spreadsheet (optional)
│   │   └── _rels/
│   │       └── chart1.xml.rels ← Relationships
│   ├── media/
│   │   └── image1.png
│   └── _rels/
│       └── document.xml.rels
└── docProps/
    ├── app.xml
    └── core.xml
```

### File Structure for PPTX with One Chart

```
presentation.pptx (ZIP archive)
├── [Content_Types].xml
├── _rels/.rels
├── ppt/
│   ├── slides/
│   │   ├── slide1.xml
│   │   └── _rels/
│   │       └── slide1.xml.rels
│   ├── charts/
│   │   ├── chart1.xml          ← Chart definition
│   │   ├── chart1.xlsx         ← Embedded spreadsheet (optional)
│   │   └── _rels/
│   │       └── chart1.xml.rels ← Relationships
│   ├── slideLayouts/
│   └── _rels/
│       └── presentation.xml.rels
└── docProps/
    ├── app.xml
    └── core.xml
```

### Relationship Entries

**In `word/_rels/document.xml.rels` (DOCX):**
```xml
<Relationship Id="rId4"
             Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart"
             Target="charts/chart1.xml"/>
```

**In `ppt/slides/_rels/slide1.xml.rels` (PPTX):**
```xml
<Relationship Id="rId2"
             Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart"
             Target="../charts/chart1.xml"/>
```

**In `word/charts/_rels/chart1.xml.rels` (if embedding xlsx):**
```xml
<Relationship Id="rId1"
             Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/package"
             Target="chart1.xlsx"/>
```

### Content Type Entries

**In `[Content_Types].xml`:**
```xml
<Override PartName="/word/charts/chart1.xml"
         ContentType="application/vnd.openxmlformats-officedocument.drawingml.chart+xml"/>

<Override PartName="/word/charts/chart1.xlsx"
         ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"/>
```

---

## 2. Chart Definition XML (chart1.xml)

### Root Element (c:chartSpace)

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"
              xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
              xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">

  <c:date1904 val="0"/>
  <c:lang val="en-US"/>
  <c:roundedCorners val="0"/>

  <c:chartContainer>
    <!-- Chart definition goes here -->
  </c:chartContainer>

</c:chartSpace>
```

### Chart Container Structure

```xml
<c:chartContainer>
  <c:title>
    <!-- Chart title -->
  </c:title>

  <c:autoTitleDeleted val="0"/>

  <c:plotArea>
    <!-- Chart type (barChart, lineChart, etc.) -->
    <!-- Axes -->
  </c:plotArea>

  <c:legend>
    <!-- Legend -->
  </c:legend>

  <c:plotVisOnly val="0"/>

</c:chartContainer>
```

### Chart Types

#### Bar Chart (Vertical Columns)

```xml
<c:chartContainer>
  <c:title>
    <c:txBody>
      <a:bodyPr/>
      <a:lstStyle/>
      <a:p>
        <a:r>
          <a:rPr lang="en-US" dirty="0" smtClean="0"/>
          <a:t>Sales by Quarter</a:t>
        </a:r>
      </a:p>
    </c:txBody>
  </c:title>

  <c:plotArea>
    <c:layout/>

    <!-- Vertical bar chart (columns) -->
    <c:barChart>
      <c:barDir val="col"/>  <!-- col = vertical, bar = horizontal -->
      <c:grouping val="clustered"/>  <!-- clustered, stacked, percentStacked -->

      <!-- Series 1 -->
      <c:ser>
        <c:idx val="0"/>
        <c:order val="0"/>
        <c:title>
          <c:strRef>
            <c:strCache>
              <c:ptCount val="1"/>
              <c:pt idx="0"><c:v>Q1</c:v></c:pt>
            </c:strCache>
          </c:strRef>
        </c:title>

        <!-- Categories (X-axis labels) -->
        <c:cat>
          <c:strRef>
            <c:strCache>
              <c:ptCount val="4"/>
              <c:pt idx="0"><c:v>Jan</c:v></c:pt>
              <c:pt idx="1"><c:v>Feb</c:v></c:pt>
              <c:pt idx="2"><c:v>Mar</c:v></c:pt>
              <c:pt idx="3"><c:v>Apr</c:v></c:pt>
            </c:strCache>
          </c:strRef>
        </c:cat>

        <!-- Values (Y-axis data) -->
        <c:val>
          <c:numRef>
            <c:numCache>
              <c:formatCode>General</c:formatCode>
              <c:ptCount val="4"/>
              <c:pt idx="0"><c:v>100</c:v></c:pt>
              <c:pt idx="1"><c:v>150</c:v></c:pt>
              <c:pt idx="2"><c:v>120</c:v></c:pt>
              <c:pt idx="3"><c:v>200</c:v></c:pt>
            </c:numCache>
          </c:numRef>
        </c:val>

        <!-- Series formatting -->
        <c:dLbls>
          <c:showVal val="0"/>
          <c:showCatName val="0"/>
          <c:showSerName val="0"/>
          <c:showLegendKey val="0"/>
        </c:dLbls>

        <!-- Series color -->
        <c:spPr>
          <a:solidFill>
            <a:srgbClr val="4472C4"/>
          </a:solidFill>
        </c:spPr>
      </c:ser>

      <!-- Category axis (X-axis) -->
      <c:catAx>
        <c:axId val="1"/>
        <c:scaling>
          <c:orientation val="minMax"/>
        </c:scaling>
        <c:delete val="0"/>
        <c:axPos val="b"/>  <!-- b = bottom -->
        <c:majorTickMark val="out"/>
        <c:minorTickMark val="none"/>
        <c:tickLblPos val="low"/>
      </c:catAx>

      <!-- Value axis (Y-axis) -->
      <c:valAx>
        <c:axId val="2"/>
        <c:scaling>
          <c:orientation val="minMax"/>
        </c:scaling>
        <c:delete val="0"/>
        <c:axPos val="l"/>  <!-- l = left -->
        <c:majorTickMark val="out"/>
        <c:minorTickMark val="none"/>
        <c:tickLblPos val="low"/>
      </c:valAx>

      <c:crossAx val="1"/>
      <c:crossesAt val="0"/>
    </c:barChart>

    <c:spPr>
      <a:noFill/>
    </c:spPr>
  </c:plotArea>

  <c:legend>
    <c:legendPos val="r"/>  <!-- r = right, l = left, t = top, b = bottom -->
    <c:layout/>
  </c:legend>

</c:chartContainer>
```

#### Line Chart

```xml
<c:lineChart>
  <c:grouping val="standard"/>  <!-- standard, stacked, percentStacked -->

  <c:ser>
    <c:idx val="0"/>
    <c:order val="0"/>
    <c:title>
      <c:strRef>
        <c:strCache>
          <c:ptCount val="1"/>
          <c:pt idx="0"><c:v>Series 1</c:v></c:pt>
        </c:strCache>
      </c:strRef>
    </c:title>

    <c:cat>
      <c:strRef>
        <c:strCache>
          <c:ptCount val="4"/>
          <c:pt idx="0"><c:v>Q1</c:v></c:pt>
          <c:pt idx="1"><c:v>Q2</c:v></c:pt>
          <c:pt idx="2"><c:v>Q3</c:v></c:pt>
          <c:pt idx="3"><c:v>Q4</c:v></c:pt>
        </c:strCache>
      </c:strRef>
    </c:cat>

    <c:val>
      <c:numRef>
        <c:numCache>
          <c:formatCode>General</c:formatCode>
          <c:ptCount val="4"/>
          <c:pt idx="0"><c:v>100</c:v></c:pt>
          <c:pt idx="1"><c:v>150</c:v></c:pt>
          <c:pt idx="2"><c:v>120</c:v></c:pt>
          <c:pt idx="3"><c:v>200</c:v></c:pt>
        </c:numCache>
      </c:numRef>
    </c:val>

    <c:smooth val="1"/>  <!-- 1 = smooth lines, 0 = straight lines -->

    <c:spPr>
      <a:ln>
        <a:solidFill>
          <a:srgbClr val="FF0000"/>
        </a:solidFill>
        <a:prstDash val="solid"/>
        <a:round/>
      </a:ln>
    </c:spPr>
  </c:ser>

  <c:catAx>
    <c:axId val="1"/>
    <c:scaling>
      <c:orientation val="minMax"/>
    </c:scaling>
    <c:delete val="0"/>
    <c:axPos val="b"/>
  </c:catAx>

  <c:valAx>
    <c:axId val="2"/>
    <c:scaling>
      <c:orientation val="minMax"/>
    </c:scaling>
    <c:delete val="0"/>
    <c:axPos val="l"/>
  </c:valAx>

  <c:crossAx val="1"/>
  <c:crossesAt val="0"/>
</c:lineChart>
```

#### Pie Chart

```xml
<c:pieChart>
  <c:varyColors val="1"/>  <!-- 1 = each slice different color -->

  <c:ser>
    <c:idx val="0"/>
    <c:order val="0"/>
    <c:title>
      <c:strRef>
        <c:strCache>
          <c:ptCount val="1"/>
          <c:pt idx="0"><c:v>Market Share</c:v></c:pt>
        </c:strCache>
      </c:strRef>
    </c:title>

    <!-- Categories (slice labels) -->
    <c:cat>
      <c:strRef>
        <c:strCache>
          <c:ptCount val="3"/>
          <c:pt idx="0"><c:v>Product A</c:v></c:pt>
          <c:pt idx="1"><c:v>Product B</c:v></c:pt>
          <c:pt idx="2"><c:v>Product C</c:v></c:pt>
        </c:strCache>
      </c:strRef>
    </c:cat>

    <!-- Values (slice sizes) -->
    <c:val>
      <c:numRef>
        <c:numCache>
          <c:formatCode>General</c:formatCode>
          <c:ptCount val="3"/>
          <c:pt idx="0"><c:v>30</c:v></c:pt>
          <c:pt idx="1"><c:v>50</c:v></c:pt>
          <c:pt idx="2"><c:v>20</c:v></c:pt>
        </c:numCache>
      </c:numRef>
    </c:val>

    <!-- Data labels (show percentages) -->
    <c:dLbls>
      <c:showVal val="0"/>
      <c:showCatName val="1"/>
      <c:showSerName val="0"/>
      <c:showPercent val="1"/>
      <c:showLegendKey val="0"/>
    </c:dLbls>
  </c:ser>

</c:pieChart>
```

#### Scatter Chart

```xml
<c:scatterChart>
  <c:scatterStyle val="lineMarker"/>  <!-- lineMarker, lineMarkerNoLine, lineNoMarker, smoothMarker, smoothMarkerNoLine, smoothNoMarker -->

  <c:ser>
    <c:idx val="0"/>
    <c:order val="0"/>
    <c:title>
      <c:strRef>
        <c:strCache>
          <c:ptCount val="1"/>
          <c:pt idx="0"><c:v>Data Series</c:v></c:pt>
        </c:strCache>
      </c:strRef>
    </c:title>

    <!-- X values -->
    <c:xVal>
      <c:numRef>
        <c:numCache>
          <c:formatCode>General</c:formatCode>
          <c:ptCount val="4"/>
          <c:pt idx="0"><c:v>1</c:v></c:pt>
          <c:pt idx="1"><c:v>2</c:v></c:pt>
          <c:pt idx="2"><c:v>3</c:v></c:pt>
          <c:pt idx="3"><c:v>4</c:v></c:pt>
        </c:numCache>
      </c:numRef>
    </c:xVal>

    <!-- Y values -->
    <c:yVal>
      <c:numRef>
        <c:numCache>
          <c:formatCode>General</c:formatCode>
          <c:ptCount val="4"/>
          <c:pt idx="0"><c:v>10</c:v></c:pt>
          <c:pt idx="1"><c:v>20</c:v></c:pt>
          <c:pt idx="2"><c:v>15</c:v></c:pt>
          <c:pt idx="3"><c:v>25</c:v></c:pt>
        </c:numCache>
      </c:numRef>
    </c:yVal>

    <c:spPr>
      <a:solidFill>
        <a:srgbClr val="4472C4"/>
      </a:solidFill>
    </c:spPr>
  </c:ser>

  <!-- X axis (value axis) -->
  <c:valAx>
    <c:axId val="1"/>
    <c:scaling>
      <c:orientation val="minMax"/>
    </c:scaling>
    <c:delete val="0"/>
    <c:axPos val="b"/>
  </c:valAx>

  <!-- Y axis (value axis) -->
  <c:valAx>
    <c:axId val="2"/>
    <c:scaling>
      <c:orientation val="minMax"/>
    </c:scaling>
    <c:delete val="0"/>
    <c:axPos val="l"/>
  </c:valAx>

  <c:crossAx val="1"/>
  <c:crossesAt val="0"/>
</c:scatterChart>
```

#### Area Chart

```xml
<c:areaChart>
  <c:grouping val="standard"/>  <!-- standard, stacked, percentStacked -->

  <c:ser>
    <c:idx val="0"/>
    <c:order val="0"/>
    <c:title>
      <c:strRef>
        <c:strCache>
          <c:ptCount val="1"/>
          <c:pt idx="0"><c:v>Series 1</c:v></c:pt>
        </c:strCache>
      </c:strRef>
    </c:title>

    <c:cat>
      <c:strRef>
        <c:strCache>
          <c:ptCount val="4"/>
          <c:pt idx="0"><c:v>Q1</c:v></c:pt>
          <c:pt idx="1"><c:v>Q2</c:v></c:pt>
          <c:pt idx="2"><c:v>Q3</c:v></c:pt>
          <c:pt idx="3"><c:v>Q4</c:v></c:pt>
        </c:strCache>
      </c:strRef>
    </c:cat>

    <c:val>
      <c:numRef>
        <c:numCache>
          <c:formatCode>General</c:formatCode>
          <c:ptCount val="4"/>
          <c:pt idx="0"><c:v>100</c:v></c:pt>
          <c:pt idx="1"><c:v>150</c:v></c:pt>
          <c:pt idx="2"><c:v>120</c:v></c:pt>
          <c:pt idx="3"><c:v>200</c:v></c:pt>
        </c:numCache>
      </c:numRef>
    </c:val>

    <c:spPr>
      <a:solidFill>
        <a:srgbClr val="4472C4"/>
      </a:solidFill>
    </c:spPr>
  </c:ser>

  <c:catAx>
    <c:axId val="1"/>
    <c:scaling>
      <c:orientation val="minMax"/>
    </c:scaling>
    <c:delete val="0"/>
    <c:axPos val="b"/>
  </c:catAx>

  <c:valAx>
    <c:axId val="2"/>
    <c:scaling>
      <c:orientation val="minMax"/>
    </c:scaling>
    <c:delete val="0"/>
    <c:axPos val="l"/>
  </c:valAx>

  <c:crossAx val="1"/>
  <c:crossesAt val="0"/>
</c:areaChart>
```

---

## 3. Data References

### Inline Data (No Spreadsheet)

Inline data is stored directly in the chart XML using `c:numCache` and `c:strCache`:

**Advantages:**
- Simpler implementation
- No embedded xlsx file
- Smaller document size

**Disadvantages:**
- Data not editable in Office applications
- No data-driven updates

**Example: Inline numeric data**
```xml
<c:val>
  <c:numRef>
    <c:numCache>
      <c:formatCode>General</c:formatCode>
      <c:ptCount val="4"/>
      <c:pt idx="0"><c:v>100</c:v></c:pt>
      <c:pt idx="1"><c:v>150</c:v></c:pt>
      <c:pt idx="2"><c:v>120</c:v></c:pt>
      <c:pt idx="3"><c:v>200</c:v></c:pt>
    </c:numCache>
  </c:numRef>
</c:val>
```

**Example: Inline string data (categories)**
```xml
<c:cat>
  <c:strRef>
    <c:strCache>
      <c:ptCount val="4"/>
      <c:pt idx="0"><c:v>Jan</c:v></c:pt>
      <c:pt idx="1"><c:v>Feb</c:v></c:pt>
      <c:pt idx="2"><c:v>Mar</c:v></c:pt>
      <c:pt idx="3"><c:v>Apr</c:v></c:pt>
    </c:strCache>
  </c:strRef>
</c:cat>
```

### Spreadsheet Reference (With Embedded xlsx)

Data references point to cells in the embedded spreadsheet:

**Advantages:**
- Full editing support in Office
- Data-driven updates
- Professional appearance

**Disadvantages:**
- More complex implementation
- Larger document size
- Requires embedded xlsx file

**Example: Reference to spreadsheet**
```xml
<c:val>
  <c:numRef>
    <c:f>Sheet1!$B$2:$B$5</c:f>
    <c:numCache>
      <!-- Cached values for offline display -->
      <c:formatCode>General</c:formatCode>
      <c:ptCount val="4"/>
      <c:pt idx="0"><c:v>100</c:v></c:pt>
      <c:pt idx="1"><c:v>150</c:v></c:pt>
      <c:pt idx="2"><c:v>120</c:v></c:pt>
      <c:pt idx="3"><c:v>200</c:v></c:pt>
    </c:numCache>
  </c:numRef>
</c:val>
```

### Lontar Recommendation

**Phase 2:** Use inline data (no xlsx) for simplicity.

```rust
pub fn generate_chart_data_inline(data: &ChartData) -> XmlElement {
    // Generate c:numCache or c:strCache directly
    // No spreadsheet reference needed
}
```

**Phase 3+:** Add optional xlsx embedding for Office editing support.

```rust
pub fn generate_chart_with_xlsx(data: &ChartData) -> (XmlElement, Vec<u8>) {
    // Generate chart XML with c:f references
    // Generate embedded xlsx file
    // Return both
}
```

---

## 4. Inline Data Format

### Numeric Data (c:numCache)

```xml
<c:numRef>
  <c:numCache>
    <c:formatCode>General</c:formatCode>
    <c:ptCount val="4"/>
    <c:pt idx="0"><c:v>100</c:v></c:pt>
    <c:pt idx="1"><c:v>150.5</c:v></c:pt>
    <c:pt idx="2"><c:v>120</c:v></c:pt>
    <c:pt idx="3"><c:v>200</c:v></c:pt>
  </c:numCache>
</c:numRef>
```

**Format codes:**
- `General` — Default number format
- `0` — Integer
- `0.00` — Two decimal places
- `#,##0` — Thousands separator
- `0%` — Percentage
- `0.00E+00` — Scientific notation

### String Data (c:strCache)

```xml
<c:strRef>
  <c:strCache>
    <c:ptCount val="4"/>
    <c:pt idx="0"><c:v>Jan</c:v></c:pt>
    <c:pt idx="1"><c:v>Feb</c:v></c:pt>
    <c:pt idx="2"><c:v>Mar</c:v></c:pt>
    <c:pt idx="3"><c:v>Apr</c:v></c:pt>
  </c:strCache>
</c:strRef>
```

### Multi-Series Chart Data

**Example: Bar chart with 3 series**

```xml
<c:barChart>
  <c:barDir val="col"/>
  <c:grouping val="clustered"/>

  <!-- Series 1: Q1 Sales -->
  <c:ser>
    <c:idx val="0"/>
    <c:order val="0"/>
    <c:title>
      <c:strRef>
        <c:strCache>
          <c:ptCount val="1"/>
          <c:pt idx="0"><c:v>Q1 Sales</c:v></c:pt>
        </c:strCache>
      </c:strRef>
    </c:title>

    <c:cat>
      <c:strRef>
        <c:strCache>
          <c:ptCount val="3"/>
          <c:pt idx="0"><c:v>Product A</c:v></c:pt>
          <c:pt idx="1"><c:v>Product B</c:v></c:pt>
          <c:pt idx="2"><c:v>Product C</c:v></c:pt>
        </c:strCache>
      </c:strRef>
    </c:cat>

    <c:val>
      <c:numRef>
        <c:numCache>
          <c:formatCode>General</c:formatCode>
          <c:ptCount val="3"/>
          <c:pt idx="0"><c:v>100</c:v></c:pt>
          <c:pt idx="1"><c:v>150</c:v></c:pt>
          <c:pt idx="2"><c:v>120</c:v></c:pt>
        </c:numCache>
      </c:numRef>
    </c:val>

    <c:spPr>
      <a:solidFill>
        <a:srgbClr val="4472C4"/>  <!-- Blue -->
      </a:solidFill>
    </c:spPr>
  </c:ser>

  <!-- Series 2: Q2 Sales -->
  <c:ser>
    <c:idx val="1"/>
    <c:order val="1"/>
    <c:title>
      <c:strRef>
        <c:strCache>
          <c:ptCount val="1"/>
          <c:pt idx="0"><c:v>Q2 Sales</c:v></c:pt>
        </c:strCache>
      </c:strRef>
    </c:title>

    <c:cat>
      <c:strRef>
        <c:strCache>
          <c:ptCount val="3"/>
          <c:pt idx="0"><c:v>Product A</c:v></c:pt>
          <c:pt idx="1"><c:v>Product B</c:v></c:pt>
          <c:pt idx="2"><c:v>Product C</c:v></c:pt>
        </c:strCache>
      </c:strRef>
    </c:cat>

    <c:val>
      <c:numRef>
        <c:numCache>
          <c:formatCode>General</c:formatCode>
          <c:ptCount val="3"/>
          <c:pt idx="0"><c:v>110</c:v></c:pt>
          <c:pt idx="1"><c:v>160</c:v></c:pt>
          <c:pt idx="2"><c:v>130</c:v></c:pt>
        </c:numCache>
      </c:numRef>
    </c:val>

    <c:spPr>
      <a:solidFill>
        <a:srgbClr val="ED7D31"/>  <!-- Orange -->
      </a:solidFill>
    </c:spPr>
  </c:ser>

  <!-- Series 3: Q3 Sales -->
  <c:ser>
    <c:idx val="2"/>
    <c:order val="2"/>
    <c:title>
      <c:strRef>
        <c:strCache>
          <c:ptCount val="1"/>
          <c:pt idx="0"><c:v>Q3 Sales</c:v></c:pt>
        </c:strCache>
      </c:strRef>
    </c:title>

    <c:cat>
      <c:strRef>
        <c:strCache>
          <c:ptCount val="3"/>
          <c:pt idx="0"><c:v>Product A</c:v></c:pt>
          <c:pt idx="1"><c:v>Product B</c:v></c:pt>
          <c:pt idx="2"><c:v>Product C</c:v></c:pt>
        </c:strCache>
      </c:strRef>
    </c:cat>

    <c:val>
      <c:numRef>
        <c:numCache>
          <c:formatCode>General</c:formatCode>
          <c:ptCount val="3"/>
          <c:pt idx="0"><c:v>120</c:v></c:pt>
          <c:pt idx="1"><c:v>170</c:v></c:pt>
          <c:pt idx="2"><c:v>140</c:v></c:pt>
        </c:numCache>
      </c:numRef>
    </c:val>

    <c:spPr>
      <a:solidFill>
        <a:srgbClr val="A5A5A5"/>  <!-- Gray -->
      </a:solidFill>
    </c:spPr>
  </c:ser>

  <!-- Axes... -->
</c:barChart>
```

---

## 5. Embedded Spreadsheet (chart1.xlsx)

### Structure

The embedded spreadsheet is a complete XLSX file stored inside the DOCX/PPTX ZIP:

```
chart1.xlsx (ZIP archive)
├── [Content_Types].xml
├── _rels/.rels
├── xl/
│   ├── workbook.xml
│   ├── worksheets/
│   │   └── sheet1.xml
│   ├── styles.xml
│   ├── sharedStrings.xml
│   └── _rels/
│       └── workbook.xml.rels
└── docProps/
    └── core.xml
```

### Sheet Content

**In `xl/worksheets/sheet1.xml`:**
```xml
<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
  <sheetData>
    <row r="1">
      <c r="A1"><v>Product</v></c>
      <c r="B1"><v>Q1</v></c>
      <c r="C1"><v>Q2</v></c>
      <c r="D1"><v>Q3</v></c>
    </row>
    <row r="2">
      <c r="A2"><v>Product A</v></c>
      <c r="B2"><v>100</v></c>
      <c r="C2"><v>110</v></c>
      <c r="D2"><v>120</v></c>
    </row>
    <row r="3">
      <c r="A3"><v>Product B</v></c>
      <c r="B3"><v>150</v></c>
      <c r="C3"><v>160</v></c>
      <c r="D3"><v>170</v></c>
    </row>
    <row r="4">
      <c r="A4"><v>Product C</v></c>
      <c r="B4"><v>120</v></c>
      <c r="C4"><v>130</v></c>
      <c r="D4"><v>140</v></c>
    </row>
  </sheetData>
</worksheet>
```

### Chart Formula References

Chart formulas reference cells in the embedded spreadsheet:

```xml
<!-- Categories from A2:A4 -->
<c:cat>
  <c:strRef>
    <c:f>Sheet1!$A$2:$A$4</c:f>
    <c:strCache>
      <!-- Cached values -->
    </c:strCache>
  </c:strRef>
</c:cat>

<!-- Values from B2:B4 -->
<c:val>
  <c:numRef>
    <c:f>Sheet1!$B$2:$B$4</c:f>
    <c:numCache>
      <!-- Cached values -->
    </c:numCache>
  </c:numRef>
</c:val>
```

### Relationship from Chart to Spreadsheet

**In `word/charts/_rels/chart1.xml.rels`:**
```xml
<Relationship Id="rId1"
             Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/package"
             Target="chart1.xlsx"/>
```

---

## 6. Chart Styling

### Series Colors (Solid Fill)

```xml
<c:spPr>
  <a:solidFill>
    <a:srgbClr val="FF0000"/>  <!-- Red: RGB(255, 0, 0) -->
  </a:solidFill>
</c:spPr>
```

**Common colors:**
- `4472C4` — Blue
- `ED7D31` — Orange
- `A5A5A5` — Gray
- `FFC000` — Gold
- `5B9BD5` — Light Blue
- `70AD47` — Green
- `FF0000` — Red

### Theme Colors

```xml
<c:spPr>
  <a:solidFill>
    <a:schemeClr val="accent1"/>
  </a:solidFill>
</c:spPr>
```

**Theme color values:**
- `accent1` through `accent6` — Theme accent colors
- `dk1`, `lt1` — Dark/light theme colors

### Axis Formatting

```xml
<c:valAx>
  <c:axId val="2"/>
  <c:scaling>
    <c:orientation val="minMax"/>
  </c:scaling>
  <c:delete val="0"/>
  <c:axPos val="l"/>

  <!-- Number format -->
  <c:numFmt formatCode="General" sourceLinked="0"/>

  <!-- Font -->
  <c:txPr>
    <a:bodyPr/>
    <a:lstStyle/>
    <a:p>
      <a:pPr>
        <a:defRPr sz="1000" b="0" i="0"/>
      </a:pPr>
    </a:p>
  </c:txPr>

  <!-- Grid lines -->
  <c:majorGridlines>
    <c:spPr>
      <a:ln>
        <a:solidFill>
          <a:srgbClr val="D0D0D0"/>
        </a:solidFill>
      </a:ln>
    </c:spPr>
  </c:majorGridlines>

  <c:minorGridlines>
    <c:spPr>
      <a:ln>
        <a:solidFill>
          <a:srgbClr val="E0E0E0"/>
        </a:solidFill>
      </a:ln>
    </c:spPr>
  </c:minorGridlines>
</c:valAx>
```

### Chart Area and Plot Area

```xml
<!-- Chart area background -->
<c:chartSpace>
  <c:spPr>
    <a:solidFill>
      <a:srgbClr val="FFFFFF"/>  <!-- White background -->
    </a:solidFill>
    <a:ln>
      <a:solidFill>
        <a:srgbClr val="000000"/>  <!-- Black border -->
      </a:solidFill>
    </a:ln>
  </c:spPr>
</c:chartSpace>

<!-- Plot area background -->
<c:plotArea>
  <c:spPr>
    <a:noFill/>  <!-- No background -->
  </c:spPr>
</c:plotArea>
```

---

## 7. Chart in DOCX vs PPTX

### Chart in DOCX

Charts in DOCX are embedded in the document body using `w:drawing`:

```xml
<w:p>
  <w:pPr>
    <w:jc w:val="center"/>
  </w:pPr>
  <w:r>
    <w:drawing>
      <wp:inline distT="0" distB="0" distL="114300" distR="114300">
        <wp:extent cx="5486400" cy="3200400"/>
        <wp:effectExtent l="0" t="0" r="0" b="0"/>
        <wp:docPr id="1" name="Chart 1"/>
        <wp:cNvGraphicFramePr/>
        <a:graphic>
          <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/chart">
            <c:chart xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"
                     xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
                     r:id="rId4"/>
          </a:graphicData>
        </a:graphic>
      </wp:inline>
    </w:drawing>
  </w:r>
</w:p>
```

**Key attributes:**
- `cx`, `cy` — Chart width and height in EMU (English Metric Units)
- `r:id="rId4"` — Relationship ID pointing to chart1.xml
- `distT`, `distB`, `distL`, `distR` — Margins around chart

### Chart in PPTX

Charts in PPTX are embedded in slides using `p:graphicFrame`:

```xml
<p:cSld>
  <p:spTree>
    <!-- Other shapes... -->

    <p:graphicFrame>
      <p:nvGraphicFramePr>
        <p:cNvPr id="2" name="Chart 1"/>
        <p:cNvGraphicFramePr/>
        <p:nvPr/>
      </p:nvGraphicFramePr>
      <p:xfrm>
        <a:off x="1524000" y="1524000"/>
        <a:ext cx="5486400" cy="3200400"/>
      </p:xfrm>
      <a:graphic>
        <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/chart">
          <c:chart xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"
                   xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
                   r:id="rId2"/>
        </a:graphicData>
      </a:graphic>
    </p:graphicFrame>
  </p:spTree>
</p:cSld>
```

**Key attributes:**
- `x`, `y` — Position in EMU
- `cx`, `cy` — Width and height in EMU
- `r:id="rId2"` — Relationship ID pointing to chart1.xml

---

## 8. Mapping to Lontar

### Decision Table: ChartKind → OOXML Element

| Lontar ChartKind | OOXML Element | Notes |
|---|---|---|
| `Bar` | `c:barChart` with `c:barDir="col"` | Vertical columns |
| `BarHorizontal` | `c:barChart` with `c:barDir="bar"` | Horizontal bars |
| `Line` | `c:lineChart` | Line chart |
| `Pie` | `c:pieChart` | Pie chart |
| `Scatter` | `c:scatterChart` | Scatter/XY chart |
| `Area` | `c:areaChart` | Area chart |

### Lontar Chart Type

```rust
pub enum ChartKind {
    Bar,
    BarHorizontal,
    Line,
    Pie,
    Scatter,
    Area,
}

pub struct ChartData {
    pub title: String,
    pub categories: Vec<String>,
    pub series: Vec<ChartSeries>,
}

pub struct ChartSeries {
    pub name: String,
    pub values: Vec<f64>,
    pub color: Option<Color>,
}
```

### Generation Strategy

**Phase 2: Inline data (no xlsx)**

```rust
pub fn generate_chart_xml(chart: &Block) -> XmlElement {
    // Extract chart data from Block::Chart
    // Generate c:chartSpace with inline c:numCache/c:strCache
    // Return chart1.xml content
}

pub fn generate_chart_relationship(chart_id: u32) -> XmlElement {
    // Generate relationship entry in document.xml.rels
    // Target: charts/chart1.xml
}
```

**Phase 3+: With embedded xlsx**

```rust
pub fn generate_chart_with_data(chart: &Block) -> (XmlElement, Vec<u8>) {
    // Generate chart1.xml with c:f references
    // Generate chart1.xlsx with source data
    // Return both
}
```

### Implementation Checklist

**Phase 2 (MVP):**
- [ ] Implement chart XML generation (c:chartSpace)
- [ ] Support Bar, Line, Pie chart types
- [ ] Generate inline numeric data (c:numCache)
- [ ] Generate inline string data (c:strCache)
- [ ] Embed chart in DOCX (w:drawing)
- [ ] Embed chart in PPTX (p:graphicFrame)
- [ ] Add chart relationship entries
- [ ] Add chart content type entries

**Phase 3:**
- [ ] Add Scatter and Area chart types
- [ ] Implement chart styling (colors, fonts, grid lines)
- [ ] Generate embedded xlsx files
- [ ] Support formula references (c:f)
- [ ] Support multi-series charts
- [ ] Support stacked and percentage-stacked variants

**Phase 4+:**
- [ ] Advanced styling (gradients, patterns)
- [ ] Data labels with custom formatting
- [ ] Trend lines and error bars
- [ ] 3D charts
- [ ] Bubble charts
- [ ] Stock charts

---

## References

- ECMA-376-1:2016 — Office Open XML File Formats (Part 1)
  - Section 21 — DrawingML
  - Section 21.2.2 — Charts
- ECMA-376-2:2016 — Office Open XML File Formats (Part 2)
  - Chart XML schemas and specifications
- Microsoft Office Open XML Standard — https://www.ecma-international.org/publications-and-standards/standards/ecma-376/
