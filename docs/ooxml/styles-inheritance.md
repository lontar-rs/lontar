# OOXML Styles Inheritance in WordprocessingML (DOCX)

## Overview

The OOXML styles system in DOCX (WordprocessingML) is a multi-level inheritance hierarchy that determines the effective formatting for every piece of text in a document. Understanding this system is critical for generating correct `styles.xml` files.

This document covers the complete styles inheritance chain, from document defaults through direct formatting, with concrete XML examples and a mapping to Lontar's type system.

---

## 1. Style Inheritance Hierarchy

### The 6-Level Precedence Chain

When Word renders a text run, it resolves formatting in this order (lowest to highest precedence):

1. **Document Defaults** (`w:docDefaults`) — baseline formatting for the entire document
2. **Named Style** (`w:style`) — paragraph, character, table, or numbering style
3. **Table Style** (`w:tblStyle`) — conditional formatting for table cells
4. **Numbering Style** — formatting applied by list numbering
5. **Paragraph Properties** (`w:pPr`) — direct formatting on the paragraph
6. **Run Properties** (`w:rPr`) — direct formatting on the text run (highest precedence)

Each level **completely overrides** the previous level for any property it explicitly sets. Properties not set at a given level inherit from the level below.

### Document Defaults (Level 1)

The `w:docDefaults` element in `styles.xml` establishes baseline formatting:

```xml
<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:docDefaults>
    <w:rPrDefault>
      <w:rPr>
        <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri" w:cs="Calibri"/>
        <w:sz w:val="22"/>        <!-- 11pt (in half-points) -->
        <w:szCs w:val="22"/>
        <w:lang w:val="en-US" w:eastAsia="zh-CN" w:bidi="ar-SA"/>
        <w:color w:val="000000"/>
      </w:rPr>
    </w:rPrDefault>
    <w:pPrDefault>
      <w:pPr>
        <w:spacing w:line="240" w:lineRule="auto"/>  <!-- 1.0 line spacing -->
        <w:jc w:val="left"/>
      </w:pPr>
    </w:pPrDefault>
  </w:docDefaults>
</w:styles>
```

**Key properties:**
- `w:rFonts` — font family (ascii, hAnsi, cs for complex scripts)
- `w:sz` — font size in half-points (22 = 11pt)
- `w:lang` — language codes for spell-check and hyphenation
- `w:color` — text color
- `w:spacing` — line spacing (line height in twips, lineRule = "auto" or "exact")
- `w:jc` — paragraph alignment (left, center, right, both)

### Named Styles (Level 2)

Named styles are defined with `w:style` elements. Each style has a type and can inherit from another style via `w:basedOn`.

#### Paragraph Style Example

```xml
<w:style w:type="paragraph" w:styleId="Heading1" w:default="0">
  <w:name w:val="Heading 1"/>
  <w:basedOn w:val="Normal"/>
  <w:next w:val="Normal"/>
  <w:link w:val="Heading1Char"/>
  <w:qFormat/>
  <w:pPr>
    <w:pStyle w:val="Heading1"/>
    <w:keepNext/>
    <w:keepLines/>
    <w:spacing w:before="240" w:after="120" w:line="240" w:lineRule="auto"/>
    <w:outlineLevel w:val="0"/>
  </w:pPr>
  <w:rPr>
    <w:rFonts w:ascii="Calibri Light" w:hAnsi="Calibri Light"/>
    <w:b/>
    <w:bCs/>
    <w:sz w:val="32"/>
    <w:szCs w:val="32"/>
    <w:color w:val="2E75B6"/>
  </w:rPr>
</w:style>
```

**Key attributes and elements:**
- `w:type="paragraph"` — style applies to paragraphs
- `w:styleId="Heading1"` — unique identifier
- `w:default="1"` — if present, this is the default style for its type
- `w:basedOn w:val="Normal"` — inherits from the Normal style; if Normal sets bold, Heading1 inherits it unless explicitly overridden
- `w:next w:val="Normal"` — when you press Enter after a Heading1, the next paragraph uses Normal style
- `w:link w:val="Heading1Char"` — links this paragraph style to a character style (for applying to selected text)
- `w:qFormat/>` — marks this as a "quick style" (appears in the styles gallery in Word)
- `w:pPr` — paragraph-level properties (spacing, alignment, indentation, etc.)
- `w:rPr` — run-level properties (font, bold, color, etc.)

#### Character Style Example

```xml
<w:style w:type="character" w:styleId="Emphasis">
  <w:name w:val="Emphasis"/>
  <w:basedOn w:val="DefaultParagraphFont"/>
  <w:link w:val="EmphasisParagraph"/>
  <w:qFormat/>
  <w:rPr>
    <w:i/>
    <w:iCs/>
  </w:rPr>
</w:style>
```

Character styles only contain `w:rPr` (run properties), not `w:pPr`.

#### Table Style Example

```xml
<w:style w:type="table" w:styleId="TableGrid">
  <w:name w:val="Table Grid"/>
  <w:basedOn w:val="TableNormal"/>
  <w:tblPr>
    <w:tblW w:w="0" w:type="auto"/>
    <w:tblInd w:w="0" w:type="dxa"/>
    <w:tblBorders>
      <w:top w:val="single" w:sz="12" w:space="0" w:color="auto"/>
      <w:left w:val="single" w:sz="12" w:space="0" w:color="auto"/>
      <w:bottom w:val="single" w:sz="12" w:space="0" w:color="auto"/>
      <w:right w:val="single" w:sz="12" w:space="0" w:color="auto"/>
      <w:insideH w:val="single" w:sz="12" w:space="0" w:color="auto"/>
      <w:insideV w:val="single" w:sz="12" w:space="0" w:color="auto"/>
    </w:tblBorders>
  </w:tblPr>
</w:style>
```

Table styles define borders, shading, and cell properties.

#### Numbering Style Example

```xml
<w:style w:type="numbering" w:styleId="ListBullet">
  <w:name w:val="List Bullet"/>
  <w:basedOn w:val="Normal"/>
  <w:pPr>
    <w:numPr>
      <w:ilvl w:val="0"/>
      <w:numId w:val="1"/>
    </w:numPr>
    <w:ind w:left="720" w:hanging="360"/>
  </w:pPr>
</w:style>
```

Numbering styles link to numbering definitions via `w:numId`.

### Style Inheritance Chain (w:basedOn)

Styles form inheritance chains. When a style doesn't explicitly set a property, it inherits from its `w:basedOn` style:

```
Heading1 (basedOn: Normal)
  ↓ inherits from
Normal (basedOn: DefaultParagraphFont)
  ↓ inherits from
DefaultParagraphFont (no basedOn)
  ↓ inherits from
Document Defaults (w:docDefaults)
```

**Example:**
- Document Defaults: font = Calibri, size = 11pt, color = black
- Normal style: (inherits font, size, color from defaults)
- Heading1 style (basedOn Normal): sets bold, size = 16pt, color = blue
  - Effective formatting: font = Calibri (from Normal), size = 16pt (overridden), bold = true (set), color = blue (overridden)

---

## 2. Document Defaults (w:docDefaults)

The `w:docDefaults` element is the foundation of the styles hierarchy. It contains two sub-elements:

### w:rPrDefault — Run Property Defaults

```xml
<w:rPrDefault>
  <w:rPr>
    <!-- Font family -->
    <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri" w:cs="Calibri"/>
    
    <!-- Font size (in half-points) -->
    <w:sz w:val="22"/>        <!-- 11pt -->
    <w:szCs w:val="22"/>      <!-- complex script size -->
    
    <!-- Language -->
    <w:lang w:val="en-US" w:eastAsia="zh-CN" w:bidi="ar-SA"/>
    
    <!-- Text color -->
    <w:color w:val="000000"/>
    
    <!-- Theme color (optional, used if w:color is not present) -->
    <w:color w:val="auto" w:themeColor="text1"/>
    
    <!-- Bold, italic (typically not set in defaults) -->
    <!-- <w:b/> -->
    <!-- <w:i/> -->
  </w:rPr>
</w:rPrDefault>
```

### w:pPrDefault — Paragraph Property Defaults

```xml
<w:pPrDefault>
  <w:pPr>
    <!-- Line spacing -->
    <w:spacing w:line="240" w:lineRule="auto"/>  <!-- 1.0 line spacing -->
    
    <!-- Paragraph alignment -->
    <w:jc w:val="left"/>
    
    <!-- Indentation (optional) -->
    <w:ind w:left="0" w:right="0" w:firstLine="0"/>
    
    <!-- Paragraph borders (optional) -->
    <!-- <w:pBdr>...</w:pBdr> -->
    
    <!-- Shading (optional) -->
    <!-- <w:shd w:val="clear" w:color="auto" w:fill="FFFFFF"/> -->
  </w:pPr>
</w:pPrDefault>
```

**Common defaults:**
- `w:rFonts` — font family (separate for ASCII, complex scripts, East Asian)
- `w:sz` — font size in half-points
- `w:lang` — language codes
- `w:spacing` — line spacing (line height in twips, lineRule = "auto" or "exact")
- `w:jc` — alignment (left, center, right, both)

---

## 3. Named Styles (w:style)

### Style Types

| Type | Purpose | Contains |
|---|---|---|
| `paragraph` | Applied to paragraphs | `w:pPr` and `w:rPr` |
| `character` | Applied to text runs | `w:rPr` only |
| `table` | Applied to entire tables | `w:tblPr` |
| `numbering` | Applied to list items | `w:pPr` with `w:numPr` |

### Built-in Style IDs

Word defines standard style IDs that should be recognized:

**Paragraph styles:**
- `Normal` — default paragraph style
- `Heading1` through `Heading6` — heading levels
- `Title` — document title
- `Subtitle` — subtitle
- `Quote` — block quote
- `ListParagraph` — paragraph in a list
- `CodeBlock` — code block (monospace)
- `TableNormal` — default table style

**Character styles:**
- `DefaultParagraphFont` — default for character formatting
- `Emphasis` — italic
- `Strong` — bold
- `Hyperlink` — hyperlink formatting

### Style Attributes

```xml
<w:style w:type="paragraph" w:styleId="Normal" w:default="1">
  <!-- w:type: paragraph, character, table, numbering -->
  <!-- w:styleId: unique identifier (used in w:pStyle, w:rStyle, w:tblStyle) -->
  <!-- w:default="1": if present, this is the default style for its type -->
  
  <w:name w:val="Normal"/>
  <!-- Display name in Word UI -->
  
  <w:basedOn w:val="DefaultParagraphFont"/>
  <!-- Style this one inherits from (optional) -->
  
  <w:next w:val="Normal"/>
  <!-- Style to apply to next paragraph after pressing Enter (paragraph styles only) -->
  
  <w:link w:val="NormalChar"/>
  <!-- Links paragraph style to character style (optional) -->
  
  <w:qFormat/>
  <!-- Marks as "quick style" (appears in styles gallery) -->
  
  <w:hidden/>
  <!-- Hides from UI (optional) -->
  
  <w:unhideWhenUsed/>
  <!-- Show in UI when this style is used (optional) -->
  
  <w:pPr>
    <!-- Paragraph properties -->
  </w:pPr>
  
  <w:rPr>
    <!-- Run properties -->
  </w:rPr>
</w:style>
```

### Paragraph Properties (w:pPr)

```xml
<w:pPr>
  <!-- Style reference -->
  <w:pStyle w:val="Heading1"/>
  
  <!-- Spacing before/after paragraph (in twips, 1/20th of a point) -->
  <w:spacing w:before="240" w:after="120" w:line="240" w:lineRule="auto"/>
  <!-- line="240" = 1.0 line spacing, "360" = 1.5, "480" = 2.0 -->
  <!-- lineRule="auto" = proportional, "exact" = fixed -->
  
  <!-- Indentation (in twips) -->
  <w:ind w:left="0" w:right="0" w:firstLine="0" w:hanging="0"/>
  <!-- hanging: first line indentation (negative = hanging indent) -->
  
  <!-- Alignment -->
  <w:jc w:val="left"/>
  <!-- left, center, right, both (justified) -->
  
  <!-- Keep paragraph with next paragraph -->
  <w:keepNext/>
  
  <!-- Keep all lines of paragraph together -->
  <w:keepLines/>
  
  <!-- Paragraph page break before -->
  <w:pageBreakBefore/>
  
  <!-- Outline level (for TOC) -->
  <w:outlineLevel w:val="0"/>
  <!-- 0 = Heading1, 1 = Heading2, ..., 8 = Heading9, 9 = body text -->
  
  <!-- Paragraph borders -->
  <w:pBdr>
    <w:top w:val="single" w:sz="12" w:space="1" w:color="000000"/>
    <w:left w:val="single" w:sz="12" w:space="1" w:color="000000"/>
    <w:bottom w:val="single" w:sz="12" w:space="1" w:color="000000"/>
    <w:right w:val="single" w:sz="12" w:space="1" w:color="000000"/>
  </w:pBdr>
  
  <!-- Paragraph shading -->
  <w:shd w:val="clear" w:color="auto" w:fill="FFFF00"/>
  <!-- fill = background color in hex -->
  
  <!-- Numbering/list properties -->
  <w:numPr>
    <w:ilvl w:val="0"/>  <!-- list level (0-8) -->
    <w:numId w:val="1"/> <!-- numbering definition ID -->
  </w:numPr>
</w:pPr>
```

### Run Properties (w:rPr)

```xml
<w:rPr>
  <!-- Font family -->
  <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri" w:cs="Calibri"/>
  <!-- ascii = Latin, hAnsi = Western European, cs = complex scripts -->
  
  <!-- Font size (in half-points) -->
  <w:sz w:val="22"/>        <!-- 11pt -->
  <w:szCs w:val="22"/>      <!-- complex script size -->
  
  <!-- Bold -->
  <w:b/>
  <w:bCs/>  <!-- complex script bold -->
  
  <!-- Italic -->
  <w:i/>
  <w:iCs/>  <!-- complex script italic -->
  
  <!-- Underline -->
  <w:u w:val="single"/>
  <!-- single, double, thick, dotted, dash, dotDash, dotDotDash, wave, etc. -->
  
  <!-- Strikethrough -->
  <w:strike/>
  
  <!-- Double strikethrough -->
  <w:dstrike/>
  
  <!-- Superscript/subscript -->
  <w:vertAlign w:val="superscript"/>
  <!-- superscript, subscript, baseline -->
  
  <!-- Text color -->
  <w:color w:val="FF0000"/>
  <!-- hex color, or w:themeColor for theme colors -->
  
  <!-- Highlight color -->
  <w:highlight w:val="yellow"/>
  <!-- yellow, green, cyan, magenta, blue, red, darkBlue, darkCyan, darkGreen, darkMagenta, darkRed, darkYellow, lightGray, darkGray, black, none -->
  
  <!-- Language -->
  <w:lang w:val="en-US" w:eastAsia="zh-CN" w:bidi="ar-SA"/>
  
  <!-- All caps -->
  <w:caps/>
  
  <!-- Small caps -->
  <w:smallCaps/>
  
  <!-- Spacing between characters (in twips) -->
  <w:spacing w:val="24"/>
  
  <!-- Character scale (percentage) -->
  <w:w w:val="100"/>
  
  <!-- Kerning (minimum font size in half-points) -->
  <w:kern w:val="24"/>
  
  <!-- Shadow -->
  <w:shadow/>
  
  <!-- Outline -->
  <w:outline/>
  
  <!-- Emboss -->
  <w:emboss/>
  
  <!-- Imprint -->
  <w:imprint/>
</w:rPr>
```

---

## 4. Style Resolution Algorithm

### Step-by-Step Resolution

When Word renders a text run, it resolves the effective formatting as follows:

```
1. Start with Document Defaults (w:docDefaults)
   effective_formatting = docDefaults.rPr + docDefaults.pPr

2. Apply Named Style (from w:pStyle or w:rStyle)
   if paragraph has w:pStyle="Heading1":
     effective_formatting += Heading1.pPr
     effective_formatting += Heading1.rPr
     if Heading1.basedOn = "Normal":
       effective_formatting += Normal.pPr
       effective_formatting += Normal.rPr
       if Normal.basedOn = "DefaultParagraphFont":
         effective_formatting += DefaultParagraphFont.rPr

3. Apply Table Style (if in a table)
   if table has w:tblStyle="TableGrid":
     effective_formatting += TableGrid.tblPr (for table)
     effective_formatting += TableGrid conditional formatting (for cell)

4. Apply Numbering Style (if in a list)
   if paragraph has w:numPr:
     effective_formatting += ListBullet.pPr

5. Apply Direct Paragraph Formatting (w:pPr in paragraph)
   effective_formatting += paragraph.w:pPr

6. Apply Direct Run Formatting (w:rPr in run)
   effective_formatting += run.w:rPr

Result: effective_formatting is the final formatting applied to the text
```

### Pseudocode

```python
def resolve_formatting(run, paragraph, table=None):
    """Resolve effective formatting for a text run."""
    
    # Start with document defaults
    effective = copy(doc_defaults.rPr)
    effective.update(doc_defaults.pPr)
    
    # Apply paragraph style and its inheritance chain
    if paragraph.pStyle:
        style = get_style(paragraph.pStyle)
        effective.update(style.rPr)
        effective.update(style.pPr)
        
        # Walk the basedOn chain
        while style.basedOn:
            style = get_style(style.basedOn)
            effective.update(style.rPr)
            effective.update(style.pPr)
    
    # Apply table style (if in table)
    if table and table.tblStyle:
        table_style = get_style(table.tblStyle)
        effective.update(table_style.tblPr)
        # Apply conditional formatting based on cell position
        if is_first_row(cell):
            effective.update(table_style.firstRow)
        if is_last_row(cell):
            effective.update(table_style.lastRow)
        # ... etc for other conditional types
    
    # Apply numbering style (if in list)
    if paragraph.numPr:
        numbering_style = get_style(paragraph.numPr.numId)
        effective.update(numbering_style.pPr)
    
    # Apply direct paragraph formatting
    effective.update(paragraph.pPr)
    
    # Apply direct run formatting (highest precedence)
    effective.update(run.rPr)
    
    return effective
```

### Paragraph Mark Formatting (w:pPr inside w:rPr)

The paragraph mark itself can have formatting via `w:rPr` inside `w:pPr`:

```xml
<w:p>
  <w:pPr>
    <w:pStyle w:val="Normal"/>
    <w:rPr>
      <!-- Formatting applied to the paragraph mark (¶) -->
      <w:b/>
      <w:color w:val="FF0000"/>
    </w:rPr>
  </w:pPr>
  <w:r>
    <w:rPr>
      <w:b/>
    </w:rPr>
    <w:t>Hello</w:t>
  </w:r>
</w:p>
```

In this example:
- The text "Hello" is bold (from `w:r/w:rPr`)
- The paragraph mark is bold and red (from `w:pPr/w:rPr`)

### Direct Formatting vs Style-Based Formatting

**Style-based formatting:**
```xml
<w:p>
  <w:pPr>
    <w:pStyle w:val="Heading1"/>
  </w:pPr>
  <w:r>
    <w:t>My Heading</w:t>
  </w:r>
</w:p>
```
The text inherits all formatting from the Heading1 style. If you change the Heading1 style, this text updates automatically.

**Direct formatting:**
```xml
<w:p>
  <w:pPr>
    <w:pStyle w:val="Heading1"/>
  </w:pPr>
  <w:r>
    <w:rPr>
      <w:color w:val="FF0000"/>  <!-- Direct formatting overrides style -->
    </w:rPr>
    <w:t>My Heading</w:t>
  </w:r>
</w:p>
```
The text uses Heading1 style but overrides the color with red. If you change the Heading1 color, this text stays red.

---

## 5. Table Styles

Table styles define formatting for entire tables and conditional formatting for specific cells.

### Table Style Structure

```xml
<w:style w:type="table" w:styleId="TableGrid">
  <w:name w:val="Table Grid"/>
  <w:basedOn w:val="TableNormal"/>
  
  <!-- Table-level properties -->
  <w:tblPr>
    <w:tblW w:w="0" w:type="auto"/>
    <w:tblInd w:w="0" w:type="dxa"/>
    <w:tblBorders>
      <w:top w:val="single" w:sz="12" w:space="0" w:color="auto"/>
      <w:left w:val="single" w:sz="12" w:space="0" w:color="auto"/>
      <w:bottom w:val="single" w:sz="12" w:space="0" w:color="auto"/>
      <w:right w:val="single" w:sz="12" w:space="0" w:color="auto"/>
      <w:insideH w:val="single" w:sz="12" w:space="0" w:color="auto"/>
      <w:insideV w:val="single" w:sz="12" w:space="0" w:color="auto"/>
    </w:tblBorders>
  </w:tblPr>
  
  <!-- Conditional formatting for specific cell types -->
  <w:tblStylePr w:type="firstRow">
    <!-- Formatting for first row -->
    <w:rPr>
      <w:b/>
      <w:color w:val="FFFFFF"/>
    </w:rPr>
    <w:tcPr>
      <w:shd w:val="clear" w:color="auto" w:fill="4472C4"/>
    </w:tcPr>
  </w:tblStylePr>
  
  <w:tblStylePr w:type="lastRow">
    <!-- Formatting for last row -->
    <w:rPr>
      <w:b/>
    </w:rPr>
  </w:tblStylePr>
  
  <w:tblStylePr w:type="firstCol">
    <!-- Formatting for first column -->
    <w:rPr>
      <w:b/>
    </w:rPr>
  </w:tblStylePr>
  
  <w:tblStylePr w:type="lastCol">
    <!-- Formatting for last column -->
    <w:rPr>
      <w:b/>
    </w:rPr>
  </w:tblStylePr>
  
  <w:tblStylePr w:type="band1Vert">
    <!-- Formatting for odd columns (vertical banding) -->
    <w:tcPr>
      <w:shd w:val="clear" w:color="auto" w:fill="F2F2F2"/>
    </w:tcPr>
  </w:tblStylePr>
  
  <w:tblStylePr w:type="band2Vert">
    <!-- Formatting for even columns -->
  </w:tblStylePr>
  
  <w:tblStylePr w:type="band1Horz">
    <!-- Formatting for odd rows (horizontal banding) -->
    <w:tcPr>
      <w:shd w:val="clear" w:color="auto" w:fill="F2F2F2"/>
    </w:tcPr>
  </w:tblStylePr>
  
  <w:tblStylePr w:type="band2Horz">
    <!-- Formatting for even rows -->
  </w:tblStylePr>
  
  <w:tblStylePr w:type="neCell">
    <!-- Formatting for northeast cell (top-right corner) -->
  </w:tblStylePr>
  
  <w:tblStylePr w:type="nwCell">
    <!-- Formatting for northwest cell (top-left corner) -->
  </w:tblStylePr>
  
  <w:tblStylePr w:type="seCell">
    <!-- Formatting for southeast cell (bottom-right corner) -->
  </w:tblStylePr>
  
  <w:tblStylePr w:type="swCell">
    <!-- Formatting for southwest cell (bottom-left corner) -->
  </w:tblStylePr>
</w:style>
```

### Conditional Type Values

| Type | Applies To |
|---|---|
| `firstRow` | First row of table |
| `lastRow` | Last row of table |
| `firstCol` | First column of table |
| `lastCol` | Last column of table |
| `band1Vert` | Odd-numbered columns (vertical banding) |
| `band2Vert` | Even-numbered columns |
| `band1Horz` | Odd-numbered rows (horizontal banding) |
| `band2Horz` | Even-numbered rows |
| `neCell` | Northeast cell (top-right corner) |
| `nwCell` | Northwest cell (top-left corner) |
| `seCell` | Southeast cell (bottom-right corner) |
| `swCell` | Southwest cell (bottom-left corner) |

### Table Cell Properties (w:tcPr)

```xml
<w:tcPr>
  <!-- Cell width -->
  <w:tcW w:w="2880" w:type="dxa"/>
  
  <!-- Vertical alignment -->
  <w:vAlign w:val="center"/>
  <!-- top, center, bottom -->
  
  <!-- Cell margins -->
  <w:tcMar>
    <w:top w:w="100" w:type="dxa"/>
    <w:left w:w="100" w:type="dxa"/>
    <w:bottom w:w="100" w:type="dxa"/>
    <w:right w:w="100" w:type="dxa"/>
  </w:tcMar>
  
  <!-- Cell borders -->
  <w:tcBorders>
    <w:top w:val="single" w:sz="12" w:space="0" w:color="000000"/>
    <w:left w:val="single" w:sz="12" w:space="0" w:color="000000"/>
    <w:bottom w:val="single" w:sz="12" w:space="0" w:color="000000"/>
    <w:right w:val="single" w:sz="12" w:space="0" w:color="000000"/>
  </w:tcBorders>
  
  <!-- Cell shading -->
  <w:shd w:val="clear" w:color="auto" w:fill="FFFF00"/>
  
  <!-- No wrap -->
  <w:noWrap/>
</w:tcPr>
```

---

## 6. Mapping to Lontar

### Type Mapping

| Lontar Type | OOXML Element | Notes |
|---|---|---|
| `StyleSheet` | `w:styles` root | Container for all styles |
| `NamedStyle` | `w:style` | Represents a single named style |
| `TextStyle` | `w:rPr` | Run-level formatting (font, bold, color, etc.) |
| `ParagraphStyle` | `w:pPr` | Paragraph-level formatting (spacing, alignment, indentation) |
| `TableStyle` | `w:tblStyle` + `w:tblStylePr` | Table and conditional cell formatting |
| `CellStyle` | `w:tcPr` | Cell-specific properties |

### Field Mapping

#### TextStyle → w:rPr

```rust
pub struct TextStyle {
    pub font_family: Option<String>,        // → w:rFonts w:ascii
    pub font_size: Option<f32>,             // → w:sz (in half-points)
    pub bold: Option<bool>,                 // → w:b (presence = true)
    pub italic: Option<bool>,               // → w:i
    pub underline: Option<UnderlineStyle>,  // → w:u w:val
    pub color: Option<Color>,               // → w:color w:val
    pub highlight: Option<Color>,           // → w:highlight w:val
    pub strikethrough: Option<bool>,        // → w:strike
    pub superscript: Option<bool>,          // → w:vertAlign w:val="superscript"
    pub subscript: Option<bool>,            // → w:vertAlign w:val="subscript"
    pub language: Option<String>,           // → w:lang w:val
    pub all_caps: Option<bool>,             // → w:caps
    pub small_caps: Option<bool>,           // → w:smallCaps
    pub letter_spacing: Option<f32>,        // → w:spacing w:val
    pub scale: Option<f32>,                 // → w:w w:val (percentage)
}
```

**Inheritance note:** `Option<T>` fields represent "inherit from parent if not set". If a field is `None`, the style inherits from its `w:basedOn` style or document defaults.

#### ParagraphStyle → w:pPr

```rust
pub struct ParagraphStyle {
    pub alignment: Option<Alignment>,       // → w:jc w:val
    pub spacing_before: Option<f32>,        // → w:spacing w:before (in twips)
    pub spacing_after: Option<f32>,         // → w:spacing w:after
    pub line_spacing: Option<f32>,          // → w:spacing w:line (in twips)
    pub line_spacing_rule: Option<String>,  // → w:spacing w:lineRule ("auto", "exact")
    pub indent_left: Option<f32>,           // → w:ind w:left (in twips)
    pub indent_right: Option<f32>,          // → w:ind w:right
    pub indent_first_line: Option<f32>,     // → w:ind w:firstLine
    pub indent_hanging: Option<f32>,        // → w:ind w:hanging
    pub keep_with_next: Option<bool>,       // → w:keepNext
    pub keep_lines_together: Option<bool>,  // → w:keepLines
    pub page_break_before: Option<bool>,    // → w:pageBreakBefore
    pub outline_level: Option<u8>,          // → w:outlineLevel w:val
    pub style_id: Option<String>,           // → w:pStyle w:val
}
```

#### NamedStyle → w:style

```rust
pub struct NamedStyle {
    pub id: String,                         // → w:styleId
    pub name: String,                       // → w:name w:val
    pub style_type: StyleType,              // → w:type (paragraph, character, table, numbering)
    pub based_on: Option<String>,           // → w:basedOn w:val
    pub next_style: Option<String>,         // → w:next w:val
    pub link: Option<String>,               // → w:link w:val
    pub is_default: bool,                   // → w:default="1"
    pub is_quick_format: bool,              // → w:qFormat (presence = true)
    pub text_style: Option<TextStyle>,      // → w:rPr
    pub paragraph_style: Option<ParagraphStyle>, // → w:pPr
}
```

### Inheritance Handling in Lontar

In Lontar, `Option<T>` fields indicate "not explicitly set, inherit from parent":

```rust
// Example: Heading1 style
let heading1 = NamedStyle {
    id: "Heading1".to_string(),
    name: "Heading 1".to_string(),
    style_type: StyleType::Paragraph,
    based_on: Some("Normal".to_string()),
    text_style: Some(TextStyle {
        bold: Some(true),
        font_size: Some(32.0),  // 16pt in half-points
        color: Some(Color::Rgb(46, 117, 182)),
        ..Default::default()
    }),
    paragraph_style: Some(ParagraphStyle {
        spacing_before: Some(240.0),  // 12pt
        spacing_after: Some(120.0),   // 6pt
        ..Default::default()
    }),
    ..Default::default()
};
```

When generating `styles.xml`, Lontar should:
1. Emit `w:basedOn w:val="Normal"` because `based_on` is `Some("Normal")`
2. Emit `w:rPr` with only the fields that are `Some(...)` (bold, font_size, color)
3. Emit `w:pPr` with only the fields that are `Some(...)` (spacing_before, spacing_after)
4. Fields that are `None` are omitted, allowing inheritance from the parent style

### Features to Implement vs Skip

#### Must-Have for Correctness

- [ ] Document defaults (w:docDefaults)
- [ ] Named styles with w:basedOn inheritance
- [ ] Paragraph styles (w:pStyle)
- [ ] Character styles (w:rStyle)
- [ ] Basic run properties: font, size, bold, italic, color
- [ ] Basic paragraph properties: alignment, spacing, indentation
- [ ] Style resolution algorithm (for backends to use)

#### Phase 2 (Nice-to-Have)

- [ ] Table styles with conditional formatting
- [ ] Numbering styles
- [ ] Advanced run properties: underline, strikethrough, superscript, subscript
- [ ] Advanced paragraph properties: borders, shading, outline level
- [ ] Quick styles (w:qFormat)
- [ ] Style linking (w:link)

#### Stretch Goals (Phase 3+)

- [ ] Theme colors (w:themeColor)
- [ ] Paragraph marks with formatting (w:pPr/w:rPr)
- [ ] Complex script properties (w:cs variants)
- [ ] Style UI customization (w:uiPriority, w:semiHidden, etc.)
- [ ] Automatic style generation from templates

---

## Summary Table

| Concept | OOXML Element | Lontar Type | Notes |
|---|---|---|---|
| Document baseline | `w:docDefaults` | StyleSheet defaults | Lowest precedence |
| Named style | `w:style` | NamedStyle | Paragraph, character, table, numbering |
| Style inheritance | `w:basedOn` | `based_on: Option<String>` | Forms inheritance chain |
| Paragraph formatting | `w:pPr` | ParagraphStyle | Alignment, spacing, indentation |
| Run formatting | `w:rPr` | TextStyle | Font, bold, color, etc. |
| Table formatting | `w:tblStyle` + `w:tblStylePr` | TableStyle | Conditional formatting for cells |
| Direct formatting | `w:pPr` in `w:p`, `w:rPr` in `w:r` | Direct properties | Highest precedence |
| Style resolution | Algorithm | DocumentWriter trait | Backends use to compute effective formatting |

---

## Implementation Checklist for lontar-docx

When implementing `lontar-docx`, follow this order:

1. **Phase 1 (MVP):**
   - [ ] Generate `w:docDefaults` from StyleSheet defaults
   - [ ] Generate `w:style` elements for NamedStyle
   - [ ] Implement `w:basedOn` inheritance chain
   - [ ] Generate `w:rPr` for TextStyle (font, size, bold, italic, color)
   - [ ] Generate `w:pPr` for ParagraphStyle (alignment, spacing, indentation)
   - [ ] Implement style resolution algorithm for backends

2. **Phase 2:**
   - [ ] Add table styles with conditional formatting
   - [ ] Add numbering styles
   - [ ] Add advanced run properties (underline, strikethrough, etc.)
   - [ ] Add advanced paragraph properties (borders, shading)

3. **Phase 3+:**
   - [ ] Theme colors
   - [ ] Complex script properties
   - [ ] Style UI customization

---

## References

- ECMA-376-1:2016 — Office Open XML File Formats (Part 1: Fundamentals and Markup Language Reference)
  - Section 17.7 — Styles
  - Section 17.7.1 — Document Defaults
  - Section 17.7.2 — Named Styles
  - Section 17.7.3 — Style Inheritance
- Microsoft Office Open XML Standard — https://www.ecma-international.org/publications-and-standards/standards/ecma-376/
