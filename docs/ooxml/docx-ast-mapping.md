# DOCX AST Mapping

Complete mapping between Lontar's AST nodes and OOXML WordprocessingML elements.

## Text Formatting (Inline Styles)

| AST Node | OOXML Element | Namespace | Notes |
|----------|---------------|-----------|-------|
| `Inline::Text { style: { bold: true } }` | `<w:r><w:rPr><w:b/></w:rPr><w:t>text</w:t></w:r>` | `w` | Bold run property |
| `Inline::Text { style: { italic: true } }` | `<w:r><w:rPr><w:i/></w:rPr><w:t>text</w:t></w:r>` | `w` | Italic run property |
| `Inline::Text { style: { underline: true } }` | `<w:r><w:rPr><w:u w:val="single"/></w:rPr><w:t>text</w:t></w:r>` | `w` | Underline (single, double, dotted, etc.) |
| `Inline::Text { style: { strikethrough: true } }` | `<w:r><w:rPr><w:strike/></w:rPr><w:t>text</w:t></w:r>` | `w` | Strikethrough |
| `Inline::Text { style: { color: "FF0000" } }` | `<w:r><w:rPr><w:color w:val="FF0000"/></w:rPr><w:t>text</w:t></w:r>` | `w` | RGB color (6-digit hex) |
| `Inline::Text { style: { font_size: 12 } }` | `<w:r><w:rPr><w:sz w:val="24"/></w:rPr><w:t>text</w:t></w:r>` | `w` | Font size in half-points (12pt = 24) |
| `Inline::Text { style: { font_name: "Arial" } }` | `<w:r><w:rPr><w:rFonts w:ascii="Arial" w:hAnsi="Arial"/></w:rPr><w:t>text</w:t></w:r>` | `w` | Font family |
| `Inline::Text { style: { superscript: true } }` | `<w:r><w:rPr><w:vertAlign w:val="superscript"/></w:rPr><w:t>text</w:t></w:r>` | `w` | Superscript positioning |
| `Inline::Text { style: { subscript: true } }` | `<w:r><w:rPr><w:vertAlign w:val="subscript"/></w:rPr><w:t>text</w:t></w:r>` | `w` | Subscript positioning |

## Block-Level Elements

### Paragraphs and Headings

| AST Node | OOXML Element | Notes |
|----------|---------------|-------|
| `Block::Paragraph { content, style }` | `<w:p><w:pPr>...</w:pPr><w:r><w:t>text</w:t></w:r></w:p>` | Basic paragraph with runs |
| `Block::Heading { level: 1, content }` | `<w:p><w:pPr><w:pStyle w:val="Heading1"/></w:pPr>...</w:p>` | Heading1-Heading6 styles |
| `Block::Heading { level: 2, content }` | `<w:p><w:pPr><w:pStyle w:val="Heading2"/></w:pPr>...</w:p>` | |
| `Block::Heading { level: 3, content }` | `<w:p><w:pPr><w:pStyle w:val="Heading3"/></w:pPr>...</w:p>` | |
| `Block::Heading { level: 4, content }` | `<w:p><w:pPr><w:pStyle w:val="Heading4"/></w:pPr>...</w:p>` | |
| `Block::Heading { level: 5, content }` | `<w:p><w:pPr><w:pStyle w:val="Heading5"/></w:pPr>...</w:p>` | |
| `Block::Heading { level: 6, content }` | `<w:p><w:pPr><w:pStyle w:val="Heading6"/></w:pPr>...</w:p>` | |

### Paragraph Properties

| AST Property | OOXML Element | Notes |
|--------------|---------------|-------|
| `alignment: Left` | `<w:pPr><w:jc w:val="left"/></w:pPr>` | Default |
| `alignment: Center` | `<w:pPr><w:jc w:val="center"/></w:pPr>` | |
| `alignment: Right` | `<w:pPr><w:jc w:val="right"/></w:pPr>` | |
| `alignment: Justify` | `<w:pPr><w:jc w:val="both"/></w:pPr>` | |
| `spacing_before: 12pt` | `<w:pPr><w:spacing w:before="240"/></w:pPr>` | In twips (1/20 point) |
| `spacing_after: 12pt` | `<w:pPr><w:spacing w:after="240"/></w:pPr>` | |
| `line_spacing: 1.5` | `<w:pPr><w:spacing w:line="360" w:lineRule="auto"/></w:pPr>` | In twips, lineRule: auto/exact/atLeast |
| `indent_left: 0.5in` | `<w:pPr><w:ind w:left="720"/></w:pPr>` | In twips (1/20 point) |
| `indent_right: 0.5in` | `<w:pPr><w:ind w:right="720"/></w:pPr>` | |
| `indent_first: 0.5in` | `<w:pPr><w:ind w:firstLine="720"/></w:ind>` | First line indent |

## Lists

### Ordered Lists

```xml
<!-- List definition in numbering.xml -->
<w:num w:numId="1">
  <w:abstractNumId w:val="0"/>
</w:num>

<!-- In document.xml -->
<w:p>
  <w:pPr>
    <w:numPr>
      <w:ilvl w:val="0"/>
      <w:numId w:val="1"/>
    </w:numPr>
  </w:pPr>
  <w:r>
    <w:t>First item</w:t>
  </w:r>
</w:p>
```

| AST Node | OOXML Element | Notes |
|----------|---------------|-------|
| `Block::List { ordered: true, items }` | `<w:p><w:pPr><w:numPr>...</w:numPr></w:pPr>...</w:p>` | Numbered list |
| `Block::List { ordered: false, items }` | `<w:p><w:pPr><w:numPr>...</w:numPr></w:pPr>...</w:p>` | Bulleted list |
| Nested list (level 1) | `<w:numPr><w:ilvl w:val="1"/></w:numPr>` | Indentation level |

### Unordered Lists

```xml
<!-- In numbering.xml -->
<w:abstractNum w:abstractNumId="1">
  <w:lvl w:ilvl="0">
    <w:start w:val="1"/>
    <w:numFmt w:val="bullet"/>
    <w:lvlText w:val="•"/>
    <w:lvlJc w:val="left"/>
    <w:pPr>
      <w:ind w:left="720" w:hanging="360"/>
    </w:pPr>
  </w:lvl>
</w:abstractNum>
```

## Tables

| AST Node | OOXML Element | Notes |
|----------|---------------|-------|
| `Block::Table { headers, rows }` | `<w:tbl><w:tblPr>...</w:tblPr><w:tr>...</w:tr></w:tbl>` | Table element |
| Header row | `<w:tr><w:trPr><w:tblHeader/></w:trPr><w:tc>...</w:tc></w:tr>` | Marked as header |
| Data row | `<w:tr><w:tc>...</w:tc></w:tr>` | Regular row |
| Cell | `<w:tc><w:tcPr>...</w:tcPr><w:p>...</w:p></w:tc>` | Table cell |
| Cell borders | `<w:tcPr><w:tcBorders><w:top w:val="single" w:sz="12" w:space="0" w:color="000000"/></w:tcBorders></w:tcPr>` | Border styling |
| Cell shading | `<w:tcPr><w:shd w:fill="CCCCCC"/></w:tcPr>` | Background color |
| Merged cells (horizontal) | `<w:tcPr><w:gridSpan w:val="2"/></w:tcPr>` | Span across columns |
| Merged cells (vertical) | `<w:tcPr><w:vMerge w:val="restart"/></w:tcPr>` | Start vertical merge |
| | `<w:tcPr><w:vMerge/></w:tcPr>` | Continue vertical merge |

### Table Example

```xml
<w:tbl>
  <w:tblPr>
    <w:tblW w:w="5000" w:type="auto"/>
    <w:tblBorders>
      <w:top w:val="single" w:sz="12" w:space="0" w:color="000000"/>
      <w:left w:val="single" w:sz="12" w:space="0" w:color="000000"/>
      <w:bottom w:val="single" w:sz="12" w:space="0" w:color="000000"/>
      <w:right w:val="single" w:sz="12" w:space="0" w:color="000000"/>
      <w:insideH w:val="single" w:sz="12" w:space="0" w:color="000000"/>
      <w:insideV w:val="single" w:sz="12" w:space="0" w:color="000000"/>
    </w:tblBorders>
  </w:tblPr>
  <w:tr>
    <w:trPr>
      <w:tblHeader/>
    </w:trPr>
    <w:tc>
      <w:tcPr>
        <w:shd w:fill="D3D3D3"/>
      </w:tcPr>
      <w:p>
        <w:r>
          <w:t>Header 1</w:t>
        </w:r>
      </w:p>
    </w:tc>
    <w:tc>
      <w:tcPr>
        <w:shd w:fill="D3D3D3"/>
      </w:tcPr>
      <w:p>
        <w:r>
          <w:t>Header 2</w:t>
        </w:r>
      </w:p>
    </w:tc>
  </w:tr>
  <w:tr>
    <w:tc>
      <w:p>
        <w:r>
          <w:t>Data 1</w:t>
        </w:r>
      </w:p>
    </w:tc>
    <w:tc>
      <w:p>
        <w:r>
          <w:t>Data 2</w:t>
        </w:r>
      </w:p>
    </w:tc>
  </w:tr>
</w:tbl>
```

## Images

| AST Node | OOXML Element | Notes |
|----------|---------------|-------|
| `Block::Image { source, width, height }` | `<w:p><w:r><w:drawing>...</w:drawing></w:r></w:p>` | Inline image |
| | `<wp:inline><a:graphic><a:graphicData><pic:pic>...</pic:pic></a:graphicData></a:graphic></wp:inline>` | DrawingML structure |

### Image Example

```xml
<w:p>
  <w:r>
    <w:drawing>
      <wp:inline distT="0" distB="0" distL="114300" distR="114300">
        <wp:extent cx="2286000" cy="1524000"/>
        <wp:effectExtent l="0" t="0" r="0" b="0"/>
        <wp:docPr id="1" name="Picture 1"/>
        <wp:cNvGraphicFramePr/>
        <a:graphic>
          <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/picture">
            <pic:pic xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture">
              <pic:nvPicPr>
                <pic:cNvPr id="0" name="image1.png"/>
                <pic:cNvPicPr/>
              </pic:nvPicPr>
              <pic:blipFill>
                <a:blip r:embed="rId4"/>
                <a:stretch>
                  <a:fillRect/>
                </a:stretch>
              </pic:blipFill>
              <pic:spPr>
                <a:xfrm>
                  <a:off x="0" y="0"/>
                  <a:ext cx="2286000" cy="1524000"/>
                </a:xfrm>
                <a:prstGeom prst="rect">
                  <a:avLst/>
                </a:prstGeom>
              </pic:spPr>
            </pic:pic>
          </a:graphicData>
        </a:graphic>
      </wp:inline>
    </w:drawing>
  </w:r>
</w:p>
```

**Steps:**
1. Store image file in `word/media/image1.png`
2. Add relationship in `word/_rels/document.xml.rels`: `<Relationship Id="rId4" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="media/image1.png"/>`
3. Reference in document with `r:embed="rId4"`

## Hyperlinks

| AST Node | OOXML Element | Notes |
|----------|---------------|-------|
| `Inline::Link { text, url }` | `<w:hyperlink r:id="rId5"><w:r><w:rPr><w:rStyle w:val="Hyperlink"/></w:rPr><w:t>text</w:t></w:r></w:hyperlink>` | External link |

### Hyperlink Example

```xml
<!-- In word/_rels/document.xml.rels -->
<Relationship Id="rId5" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink" Target="https://example.com" TargetMode="External"/>

<!-- In word/document.xml -->
<w:p>
  <w:hyperlink r:id="rId5">
    <w:r>
      <w:rPr>
        <w:rStyle w:val="Hyperlink"/>
        <w:color w:val="0563C1"/>
        <w:u w:val="single"/>
      </w:rPr>
      <w:t>Click here</w:t>
    </w:r>
  </w:hyperlink>
</w:p>
```

## Page Breaks

| AST Node | OOXML Element | Notes |
|----------|---------------|-------|
| `Block::PageBreak` | `<w:p><w:r><w:br w:type="page"/></w:r></w:p>` | Hard page break |

## Section Breaks

```xml
<w:p>
  <w:pPr>
    <w:sectPr>
      <w:pgSz w:w="12240" w:h="15840"/>
      <w:pgMar w:top="1440" w:right="1440" w:bottom="1440" w:left="1440"/>
      <w:cols w:num="1"/>
      <w:footerReference w:type="default" r:id="rId7"/>
      <w:headerReference w:type="default" r:id="rId6"/>
    </w:sectPr>
  </w:pPr>
</w:p>
```

## Headers and Footers

```xml
<!-- In word/_rels/document.xml.rels -->
<Relationship Id="rId6" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/header" Target="header1.xml"/>
<Relationship Id="rId7" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer" Target="footer1.xml"/>

<!-- In word/header1.xml -->
<w:hdr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:p>
    <w:r>
      <w:t>Header text</w:t>
    </w:r>
  </w:p>
</w:hdr>

<!-- In word/footer1.xml -->
<w:ftr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:p>
    <w:r>
      <w:t>Footer text</w:t>
    </w:r>
  </w:p>
</w:ftr>
```

## Code Blocks

```xml
<w:p>
  <w:pPr>
    <w:pStyle w:val="CodeBlock"/>
    <w:ind w:left="720"/>
    <w:spacing w:line="240" w:lineRule="auto"/>
  </w:pPr>
  <w:r>
    <w:rPr>
      <w:rStyle w:val="CodeChar"/>
      <w:rFonts w:ascii="Courier New" w:hAnsi="Courier New"/>
      <w:sz w:val="20"/>
    </w:rPr>
    <w:t>fn main() { println!("Hello"); }</w:t>
  </w:r>
</w:p>
```

## Block Quotes

```xml
<w:p>
  <w:pPr>
    <w:pStyle w:val="Quote"/>
    <w:ind w:left="720" w:right="720"/>
    <w:spacing w:before="240" w:after="240"/>
    <w:bdr>
      <w:left w:val="single" w:sz="24" w:space="24" w:color="CCCCCC"/>
    </w:bdr>
  </w:pPr>
  <w:r>
    <w:rPr>
      <w:i/>
    </w:rPr>
    <w:t>Quoted text</w:t>
  </w:r>
</w:p>
```

## Summary Table

| AST Category | OOXML Namespace | Key Elements |
|--------------|-----------------|--------------|
| Text formatting | `w` (WordprocessingML) | `<w:rPr>`, `<w:b>`, `<w:i>`, `<w:u>`, `<w:color>`, `<w:sz>` |
| Paragraphs | `w` | `<w:p>`, `<w:pPr>`, `<w:jc>`, `<w:ind>`, `<w:spacing>` |
| Lists | `w` | `<w:numPr>`, `<w:ilvl>`, `<w:numId>` |
| Tables | `w` | `<w:tbl>`, `<w:tr>`, `<w:tc>`, `<w:tblPr>` |
| Images | `w`, `wp`, `a`, `pic` | `<w:drawing>`, `<wp:inline>`, `<a:blip>`, `<pic:pic>` |
| Hyperlinks | `w`, `r` | `<w:hyperlink>`, `r:id` |
| Metadata | `cp`, `dc`, `dcterms` | `<dc:title>`, `<dc:creator>`, `<dcterms:created>` |

## Implementation Strategy

1. **Traverse AST** — walk the document tree
2. **Generate XML** — for each node, emit corresponding OOXML elements
3. **Track relationships** — maintain a list of external resources (images, hyperlinks)
4. **Manage IDs** — generate unique IDs for relationships, styles, numbering
5. **ZIP package** — collect all XML files and create a ZIP archive
6. **Update [Content_Types].xml** — add entries for all generated parts
