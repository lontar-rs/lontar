# PDF Font Embedding and Glyph Positioning

## Overview

PDF embeds fonts differently than DOCX/PPTX. Instead of storing font files, PDF stores glyph outlines and positioning information directly in the PDF stream.

**Key differences:**
- DOCX/PPTX: Embed full font files (TTF/OTF)
- PDF: Embed font subsets + glyph metrics + positioning

## Font Types in PDF

### Type 1 (PostScript)

**Format:** PostScript-based font format

**Pros:**
- Compact
- Well-supported

**Cons:**
- Limited to 256 glyphs
- Obsolete

### TrueType (Type 42)

**Format:** TrueType font embedded in PDF

**Pros:**
- Supports full Unicode
- Preserves OpenType features

**Cons:**
- Larger file size

**PDF structure:**
```
/Type /Font
/Subtype /TrueType
/BaseFont /NotoSans
/Encoding /Identity-H  (for Unicode)
/DescendantFonts [...]
/FontDescriptor
  /FontFile2 <stream>  (TrueType data)
  /FontBBox [...]
  /Ascent 800
  /Descent -200
```

### CID Font (Type 0)

**Format:** Composite font for CJK and complex scripts

**Pros:**
- Supports thousands of glyphs
- Optimized for CJK

**Cons:**
- Complex structure
- Requires CMap

**PDF structure:**
```
/Type /Font
/Subtype /Type0
/BaseFont /NotoSansCJK
/Encoding /Identity-H
/DescendantFonts [
  /Type /Font
  /Subtype /CIDFontType2
  /CIDSystemInfo
    /Registry (Adobe)
    /Ordering (Identity)
    /Supplement 0
  /FontDescriptor
    /FontFile2 <stream>
]
/ToUnicode <stream>  (CID to Unicode mapping)
```

## Glyph Positioning in PDF

### Text Matrix

PDF positions text using a transformation matrix:

```
BT                          % Begin text
/F1 12 Tf                   % Font and size
100 700 Td                  % Position (x, y)
(Hello) Tj                  % Show text
ET                          % End text
```

### Glyph Positioning (Advanced)

For precise glyph positioning (needed for complex scripts):

```
BT
/F1 12 Tf
100 700 Td
[(H)100(e)50(l)50(l)50(o)] TJ  % Individual glyph positioning
ET
```

**Array format:** `[(glyph1) x_offset (glyph2) x_offset ...]`

### Kerning and Advances

```
BT
/F1 12 Tf
100 700 Td
[
  (A)      % Glyph A
  -50      % Kerning adjustment
  (V)      % Glyph V
  -30      % Kerning adjustment
  (e)      % Glyph e
] TJ
ET
```

## Font Subsetting in PDF

PDF requires subsetting fonts to reduce file size.

### Subsetting Process

1. **Identify used glyphs** — Scan document for all characters
2. **Extract glyphs** — Get glyph outlines from font
3. **Build CFF/glyf table** — Subset glyph table
4. **Update metrics** — Adjust hmtx, hhea tables
5. **Create CMap** — Map CIDs to Unicode
6. **Embed in PDF** — Include font stream

### Subset Naming

PDF requires subset fonts to have special names:

```
/BaseFont /ABCDEF+NotoSans  % ABCDEF = 6-char random prefix
```

The prefix prevents font reuse across documents (licensing requirement).

## Integration with Lontar

### PDF Generation Pipeline

```
Input Document (AST)
    ↓
[lontar-aksara] — Shape text
    ↓
ShapedRun { glyphs, positions, font }
    ↓
[lontar-pdf] — Generate PDF
    ├─ Identify used glyphs
    ├─ Subset fonts
    ├─ Create font descriptors
    ├─ Generate glyph positioning
    └─ Embed fonts in PDF stream
    ↓
Output PDF with embedded fonts
```

### Proposed API

```rust
pub struct PdfFont {
    pub name: String,
    pub subset_prefix: String,
    pub font_data: Vec<u8>,
    pub glyph_metrics: HashMap<u16, GlyphMetric>,
    pub cmap: HashMap<char, u16>,  // Character to glyph ID
}

pub struct GlyphMetric {
    pub glyph_id: u16,
    pub width: i32,
    pub height: i32,
    pub x_min: i32,
    pub y_min: i32,
}

pub fn embed_font_in_pdf(
    pdf: &mut PdfBuilder,
    font_path: &Path,
    used_chars: &str,
) -> Result<PdfFont, Box<dyn std::error::Error>> {
    // Step 1: Load font
    let font_data = std::fs::read(font_path)?;
    
    // Step 2: Identify used glyphs
    let used_glyphs = identify_used_glyphs(&font_data, used_chars)?;
    
    // Step 3: Subset font
    let subset_data = subset_font(&font_data, &used_glyphs)?;
    
    // Step 4: Create font descriptor
    let font_name = format!("{}+NotoSans", generate_subset_prefix());
    let pdf_font = PdfFont {
        name: font_name,
        subset_prefix: generate_subset_prefix(),
        font_data: subset_data,
        glyph_metrics: extract_metrics(&font_data, &used_glyphs)?,
        cmap: build_cmap(&font_data, used_chars)?,
    };
    
    // Step 5: Embed in PDF
    pdf.add_font(&pdf_font)?;
    
    Ok(pdf_font)
}

pub fn position_glyphs_in_pdf(
    pdf: &mut PdfBuilder,
    shaped_runs: &[ShapedRun],
) -> Result<(), Box<dyn std::error::Error>> {
    // Generate text positioning commands
    for run in shaped_runs {
        let mut positioning = Vec::new();
        
        for (glyph_info, glyph_pos) in run.glyph_infos.iter().zip(run.glyph_positions.iter()) {
            positioning.push((
                glyph_info.codepoint,
                glyph_pos.x_advance,
                glyph_pos.y_advance,
            ));
        }
        
        pdf.add_positioned_text(&positioning)?;
    }
    
    Ok(())
}

fn generate_subset_prefix() -> String {
    // Generate 6-character random prefix
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..6)
        .map(|_| (b'A' + rng.gen_range(0..26)) as char)
        .collect()
}
```

## Complex Script Handling in PDF

### Arabic Text Positioning

```
Input: "مرحبا" (RTL)
    ↓
[rustybuzz] — Shape with Arabic features
    ↓
Glyphs: [meem_initial, reh_medial, hah_medial, bah_final]
Positions: [x1, x2, x3, x4]
    ↓
PDF positioning:
BT
/F1 12 Tf
100 700 Td
[(glyph1)x1(glyph2)x2(glyph3)x3(glyph4)x4] TJ
ET
```

### Devanagari Text Positioning

```
Input: "क्षत्रिय" (Devanagari with conjuncts)
    ↓
[rustybuzz] — Shape with Indic features
    ↓
Glyphs: [ksha_conjunct, tri_conjunct, ya]
Positions: [x1, x2, x3]
    ↓
PDF positioning:
BT
/F1 12 Tf
100 700 Td
[(glyph1)x1(glyph2)x2(glyph3)x3] TJ
ET
```

## Performance Considerations

| Operation | Time | Size |
|---|---|---|
| Font subsetting | 50-200 ms | 50-200 KB (subset) |
| PDF generation | 100-500 ms | 100-500 KB (full doc) |
| Glyph positioning | 1-10 ms | Minimal |

**Optimization tips:**
- Subset fonts once, reuse across document
- Cache glyph metrics
- Use efficient positioning commands

## Testing

```rust
#[test]
fn test_pdf_font_embedding() {
    let font_path = Path::new("tests/fixtures/scripts/fonts/NotoSans-Regular.ttf");
    let text = "Hello مرحبا";
    
    let pdf_font = embed_font_in_pdf(&mut pdf, font_path, text).unwrap();
    
    assert!(pdf_font.font_data.len() > 0);
    assert!(pdf_font.cmap.len() > 0);
    assert!(pdf_font.name.contains("+NotoSans"));
}

#[test]
fn test_glyph_positioning() {
    let shaped_runs = vec![
        ShapedRun { /* ... */ },
        ShapedRun { /* ... */ },
    ];
    
    let mut pdf = PdfBuilder::new();
    position_glyphs_in_pdf(&mut pdf, &shaped_runs).unwrap();
    
    // Verify PDF contains positioning commands
}
```

## References

- [PDF Specification (ISO 32000)](https://www.adobe.io/content/dam/udp/assets/open/pdf/spec/PDF32000_2008.pdf)
- [Font Embedding in PDF](https://www.adobe.io/content/dam/udp/assets/open/pdf/spec/PDF32000_2008.pdf) (Section 9)
- [CID Fonts](https://www.adobe.io/content/dam/udp/assets/open/pdf/spec/PDF32000_2008.pdf) (Section 9.7)
- [Text Positioning](https://www.adobe.io/content/dam/udp/assets/open/pdf/spec/PDF32000_2008.pdf) (Section 5.3)
