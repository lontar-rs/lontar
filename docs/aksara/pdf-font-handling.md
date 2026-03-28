# PDF Font Handling

## Overview

This document details how to integrate text shaping results into PDF documents, focusing on font embedding, glyph positioning, and text encoding.

## PDF Font Types

PDF supports several font types:

| Type | Description | Embedding | Subsetting | Use Case |
|------|-------------|-----------|-----------|----------|
| Type 1 | PostScript outline | ✅ | ✅ | Legacy, Latin |
| TrueType | Outline format | ✅ | ✅ | Modern, all scripts |
| OpenType | Extended TrueType | ✅ | ✅ | Modern, all scripts |
| CIDFont | Composite font | ✅ | ✅ | CJK, complex scripts |
| Type 3 | Custom outline | ✅ | ❌ | Rare |

**Recommendation for Lontar:** Use TrueType/OpenType (Type 42 in PDF) for all fonts.

## Integration Pipeline

```
1. Input text: "Hello مرحبا 你好 ᬮᭀᬦ᭄ᬢᬭ᭄"
2. BiDi reordering: "Hello ابحرم 你好 ᬮᭀᬦ᭄ᬢᬭ᭄"
3. Script detection: [Latin, Arabic, Han, Balinese]
4. Text shaping: Shape each script run with rustybuzz
5. Font embedding: Embed subset fonts
6. Glyph positioning: Calculate glyph positions
7. PDF generation: Create text objects with positioned glyphs
```

## Step 1: Embed Fonts

```rust
use uuid::Uuid;

pub struct PdfFont {
    pub name: String,
    pub font_data: Vec<u8>,
    pub subset_data: Vec<u8>,
    pub font_dict_ref: String,  // PDF object reference
}

pub fn embed_font_in_pdf(
    pdf: &mut PdfDocument,
    font_name: &str,
) -> Result<PdfFont> {
    // Load font
    let font_data = load_font(font_name)?;

    // Check embedding permission
    match check_embedding_permission(&font_data)? {
        EmbeddingPermission::Restricted => {
            return Err("Font embedding restricted".into());
        }
        _ => {}
    }

    // Subset font (collect used glyphs first)
    let used_glyphs = collect_used_glyphs_from_document(pdf);
    let subset_data = subset_font(&font_data, &used_glyphs)?;

    // Create font dictionary
    let font_dict = create_font_dict(font_name, &subset_data)?;

    // Add to PDF
    let font_dict_ref = pdf.add_object(font_dict);

    Ok(PdfFont {
        name: font_name.to_string(),
        font_data,
        subset_data,
        font_dict_ref,
    })
}

fn create_font_dict(font_name: &str, font_data: &[u8]) -> Result<String> {
    // Create Type 42 font dictionary (TrueType in PDF)
    let font_stream = format!(
        r#"
<< /Type /Font
   /Subtype /TrueType
   /BaseFont /{}
   /Encoding /WinAnsiEncoding
   /FontDescriptor << /Type /FontDescriptor
                      /FontName /{}
                      /FontFile2 {} 0 R
                      /FontBBox [-100 -200 1000 900]
                      /ItalicAngle 0
                      /Ascent 800
                      /Descent -200
                      /CapHeight 700
                      /StemV 70
                      /Flags 32
                   >>
   /FirstChar 0
   /LastChar 255
   /Widths [500 500 500 500 500 500 500 500 500 500 500 500 500 500 500 500]
>>
"#,
        font_name, font_name
    );

    Ok(font_stream)
}
```

## Step 2: Create ToUnicode CMap

The ToUnicode CMap maps glyph IDs to Unicode codepoints, enabling text selection and searching:

```rust
pub fn create_to_unicode_cmap(
    font_name: &str,
    glyph_map: &HashMap<u32, u32>,  // glyph_id -> codepoint
) -> String {
    let mut cmap = String::from(
        r#"/CIDInit /ProcSet findresource begin
12 dict begin
begincmap
/CIDSystemInfo
<< /Registry (Adobe)
/Ordering (UCS)
/Supplement 0
>> def
/CMapName /Adobe-Identity-UCS def
/CMapType 2 def
1 begincodespacerange
<0000> <FFFF>
endcodespacerange
"#,
    );

    // Add glyph to Unicode mappings
    let mut count = 0;
    cmap.push_str("1 beginbfchar\n");

    for (glyph_id, codepoint) in glyph_map {
        cmap.push_str(&format!(
            "<{:04X}> <{:04X}>\n",
            glyph_id, codepoint
        ));
        count += 1;

        if count >= 100 {
            cmap.push_str("endbfchar\n");
            cmap.push_str("1 beginbfchar\n");
            count = 0;
        }
    }

    cmap.push_str("endbfchar\n");
    cmap.push_str(
        r#"endcmap
CMapName currentdict /CMap defineresource pop
end
end"#,
    );

    cmap
}
```

## Step 3: Calculate Glyph Positions

Text shaping provides glyph advances and offsets. Convert to PDF coordinates:

```rust
pub struct PdfGlyph {
    pub glyph_id: u32,
    pub x: f32,
    pub y: f32,
    pub advance: f32,
}

pub fn calculate_glyph_positions(
    shaped_runs: &[ShapedRun],
    font_size: f32,
) -> Vec<PdfGlyph> {
    let mut glyphs = Vec::new();
    let mut x_pos = 0.0;
    let mut y_pos = 0.0;

    for shaped_run in shaped_runs {
        for (glyph_info, glyph_pos) in shaped_run.glyphs.iter().zip(&shaped_run.positions) {
            // Convert from font units to PDF units
            let scale = font_size / 1000.0;  // Assuming 1000 units per em

            let glyph = PdfGlyph {
                glyph_id: glyph_info.codepoint,
                x: x_pos + (glyph_pos.x_offset as f32 * scale),
                y: y_pos + (glyph_pos.y_offset as f32 * scale),
                advance: glyph_pos.x_advance as f32 * scale,
            };

            glyphs.push(glyph);
            x_pos += glyph.advance;
        }
    }

    glyphs
}
```

## Step 4: Generate Text Objects

```rust
pub fn generate_pdf_text_object(
    text: &str,
    glyphs: &[PdfGlyph],
    font_ref: &str,
    font_size: f32,
    x: f32,
    y: f32,
) -> String {
    let mut obj = format!(
        "BT\n/F1 {} Tf\n{} {} Td\n",
        font_size, x, y
    );

    // Add glyph positioning
    obj.push_str("(");
    for glyph in glyphs {
        // Encode glyph ID as hex
        obj.push_str(&format!("<{:04X}>", glyph.glyph_id));
    }
    obj.push_str(") Tj\n");

    obj.push_str("ET\n");

    obj
}
```

**Example Output:**

```
BT
/F1 12 Tf
50 750 Td
(<0048><0065><006C><006C><006F>) Tj
ET
```

## Step 5: Handle BiDi Text

For RTL text, reverse glyph order:

```rust
pub fn reorder_glyphs_for_pdf(
    glyphs: &[PdfGlyph],
    is_rtl: bool,
) -> Vec<PdfGlyph> {
    if is_rtl {
        // Reverse glyph order for RTL
        let mut reversed = glyphs.to_vec();
        reversed.reverse();

        // Recalculate positions
        let mut x_pos = 0.0;
        for glyph in &mut reversed {
            x_pos += glyph.advance;
            glyph.x = x_pos;
        }

        reversed
    } else {
        glyphs.to_vec()
    }
}
```

## Step 6: Handle Complex Scripts

For complex scripts (Arabic, Indic, Balinese), glyphs may have offsets:

```rust
pub fn generate_pdf_text_with_offsets(
    glyphs: &[PdfGlyph],
    font_ref: &str,
    font_size: f32,
    x: f32,
    y: f32,
) -> String {
    let mut obj = format!(
        "BT\n/F1 {} Tf\n{} {} Td\n",
        font_size, x, y
    );

    // Add glyphs with positioning
    for glyph in glyphs {
        // Position glyph
        obj.push_str(&format!(
            "{} {} Td\n",
            glyph.x, glyph.y
        ));

        // Show glyph
        obj.push_str(&format!("<{:04X}> Tj\n", glyph.glyph_id));
    }

    obj.push_str("ET\n");

    obj
}
```

## Step 7: Text Wrapping

```rust
pub fn wrap_text_for_pdf(
    text: &str,
    max_width: f32,
    font: &PdfFont,
    font_size: f32,
) -> Vec<String> {
    let breaker = LineBreaker::new(text);
    let breaks: Vec<usize> = breaker.collect();

    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0.0;
    let mut prev_break = 0;

    for break_pos in breaks {
        let segment = &text[prev_break..break_pos];
        let segment_width = measure_text_width(segment, font, font_size);

        if (current_width + segment_width) > max_width {
            // Start new line
            lines.push(current_line.trim_end().to_string());
            current_line = segment.to_string();
            current_width = segment_width;
        } else {
            current_line.push_str(segment);
            current_width += segment_width;
        }

        prev_break = break_pos;
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

fn measure_text_width(text: &str, font: &PdfFont, font_size: f32) -> f32 {
    // Measure text width using font metrics
    let mut width = 0.0;
    for ch in text.chars() {
        if let Some(glyph_id) = font.glyph_id(ch) {
            let advance = font.glyph_advance(glyph_id);
            width += (advance as f32 * font_size) / 1000.0;
        }
    }
    width
}
```

## Complete Example

```rust
pub fn generate_pdf_paragraph(
    text: &str,
    font_name: &str,
    font_size: f32,
    x: f32,
    y: f32,
    max_width: f32,
) -> Result<String> {
    // Step 1: BiDi reordering
    let bidi = BidiInfo::new(text, Some(Level::ltr()));
    let visual_text = bidi.reorder_visual();

    // Step 2: Line wrapping
    let lines = wrap_text_for_pdf(&visual_text, max_width, &font, font_size);

    // Step 3: Generate PDF objects
    let mut pdf_content = String::new();
    let mut current_y = y;

    for line in lines {
        // Detect script runs
        let script_runs = detect_script_runs(&line);

        // Shape text
        let shaped_runs = shape_script_runs(&script_runs)?;

        // Calculate glyph positions
        let glyphs = calculate_glyph_positions(&shaped_runs, font_size);

        // Generate PDF text object
        let text_obj = generate_pdf_text_with_offsets(
            &glyphs,
            &font_ref,
            font_size,
            x,
            current_y,
        );

        pdf_content.push_str(&text_obj);
        current_y -= (font_size * 1.2);  // Line spacing
    }

    Ok(pdf_content)
}
```

## Key Considerations

### 1. Font Subsetting

Always subset fonts to reduce file size:

```rust
// Collect all glyphs used in document
let used_glyphs = collect_all_glyphs(pdf);

// Subset each font
for font_name in fonts {
    let subset_data = subset_font(&font_data, &used_glyphs)?;
    embed_font(pdf, font_name, &subset_data)?;
}
```

### 2. ToUnicode CMap

Always include ToUnicode CMap for searchability:

```rust
let cmap = create_to_unicode_cmap(font_name, &glyph_map);
pdf.add_to_unicode_cmap(font_ref, &cmap)?;
```

### 3. Text Encoding

Use CID encoding for complex scripts:

```rust
// For complex scripts, use CID encoding
let encoding = if requires_cid_encoding(script) {
    "Identity-H"  // Horizontal CID
} else {
    "WinAnsiEncoding"  // Simple encoding
};
```

### 4. Glyph Positioning

Account for glyph offsets from text shaping:

```rust
// Apply glyph offsets from shaping
for (glyph, pos) in glyphs.iter().zip(positions) {
    let x = base_x + pos.x_offset;
    let y = base_y + pos.y_offset;
    // Position glyph at (x, y)
}
```

## Performance Characteristics

| Operation | Time | Notes |
|-----------|------|-------|
| Font embedding | ~50-200ms | Depends on font size |
| Glyph positioning | ~0.1-0.5ms per 100 chars | Fast |
| ToUnicode CMap | ~1-5ms | Depends on glyph count |
| Text wrapping | ~0.5-2ms per 100 chars | Fast |
| Total per page | ~100-300ms | Acceptable |

## References

- [PDF Specification](https://www.adobe.io/content/dam/udp/assets/open/pdf/spec/PDF32000_2008.pdf)
- [TrueType Font Specification](https://docs.microsoft.com/en-us/typography/opentype/spec/)
- [Unicode in PDF](https://www.adobe.com/content/dam/udp/assets/open/pdf/spec/PDFReference.pdf)
