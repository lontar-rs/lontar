# Rustybuzz Evaluation

## Overview

`rustybuzz` is a Rust port of HarfBuzz, the industry-standard text shaping engine. It converts logical character sequences into positioned glyphs, handling complex script features like ligatures, contextual forms, and conjuncts.

## API Structure

### Core Types

```rust
// Font representation
pub struct Face<'a> {
    // Loaded from font file bytes
}

pub struct Font<'a> {
    // Wraps Face with additional metrics
}

// Text buffer
pub struct UnicodeBuffer {
    // Holds input text and properties
}

// Shaped output
pub struct GlyphBuffer {
    // Contains positioned glyphs
}

pub struct GlyphInfo {
    pub codepoint: u32,      // Original Unicode codepoint
    pub cluster: u32,        // Cluster index (groups chars that form one glyph)
}

pub struct GlyphPosition {
    pub x_advance: i32,      // Horizontal advance width
    pub y_advance: i32,      // Vertical advance width
    pub x_offset: i32,       // Horizontal offset from origin
    pub y_offset: i32,       // Vertical offset from origin
}
```

### Main Functions

```rust
// Load font from bytes
pub fn Face::from_slice(data: &[u8], index: u32) -> Result<Face>

// Create text buffer
pub fn UnicodeBuffer::new() -> UnicodeBuffer
pub fn buffer.push_str(text: &str)
pub fn buffer.set_language(lang: Language)
pub fn buffer.set_script(script: Script)

// Perform shaping
pub fn shape(face: &Face, features: &[Feature], buffer: UnicodeBuffer) -> GlyphBuffer

// Access results
pub fn glyph_buffer.glyph_infos() -> &[GlyphInfo]
pub fn glyph_buffer.glyph_positions() -> &[GlyphPosition]
```

## Example: Latin Text

```rust
use rustybuzz::{Face, UnicodeBuffer, shape};

// Load font
let font_data = include_bytes!("path/to/Calibri.ttf");
let face = Face::from_slice(font_data, 0)?;

// Create buffer
let mut buffer = UnicodeBuffer::new();
buffer.push_str("Hello");
buffer.set_language(rustybuzz::Language::English);
buffer.set_script(rustybuzz::Script::Latin);

// Shape
let output = shape(&face, &[], buffer);

// Results
for (info, pos) in output.glyph_infos().iter().zip(output.glyph_positions()) {
    println!("Glyph: {}, Advance: {}", info.codepoint, pos.x_advance);
}
```

**Output:**
- 5 glyphs (H, e, l, l, o) with no ligatures
- Each glyph has x_advance (width) and y_advance (0 for horizontal text)
- No offsets (baseline-aligned)

## Example: Balinese Text (Complex Script)

```rust
use rustybuzz::{Face, UnicodeBuffer, shape};

// Load Noto Sans Balinese
let font_data = include_bytes!("path/to/NotoSansBalinese-Regular.ttf");
let face = Face::from_slice(font_data, 0)?;

// Create buffer with Balinese text
let mut buffer = UnicodeBuffer::new();
buffer.push_str("ᬮᭀᬦ᭄ᬢᬭ᭄");  // "lontar" in Aksara Bali
buffer.set_language(rustybuzz::Language::new("ban"));  // Balinese
buffer.set_script(rustybuzz::Script::Balinese);

// Shape
let output = shape(&face, &[], buffer);

// Results
let infos = output.glyph_infos();
let positions = output.glyph_positions();

println!("Input chars: 6");
println!("Output glyphs: {}", infos.len());  // Likely < 6 due to conjuncts

for (i, (info, pos)) in infos.iter().zip(positions).enumerate() {
    println!("Glyph {}: cluster={}, advance={}", i, info.cluster, pos.x_advance);
}
```

**Expected Output:**
- Input: 6 characters (ᬮ ᭀ ᬦ ᭄ ᬢ ᭄)
- Output: ~4-5 glyphs (some characters combine into conjuncts)
- Cluster indices show which input characters map to each glyph
- Example: `ᬦ᭄ᬢ` (na + virama + ta) forms a single conjunct glyph

## OpenType Features

Rustybuzz supports OpenType features for fine-grained shaping control:

```rust
use rustybuzz::{Feature, FeatureFlags};

// Enable specific features
let features = vec![
    Feature {
        tag: rustybuzz::Tag::new(b"liga"),  // Ligatures
        value: 1,
        start: 0,
        end: usize::MAX,
    },
    Feature {
        tag: rustybuzz::Tag::new(b"dlig"),  // Discretionary ligatures
        value: 0,  // Disabled
        start: 0,
        end: usize::MAX,
    },
];

let output = shape(&face, &features, buffer);
```

**Common Features:**
- `liga` — Standard ligatures (fi, fl)
- `dlig` — Discretionary ligatures
- `calt` — Contextual alternates
- `ccmp` — Character composition
- `blwf` — Below-base forms (Indic scripts)
- `pstf` — Post-base forms (Indic scripts)

## Integration with Lontar

### 1. Script Detection → Shaping

```rust
pub struct ShapedRun {
    pub text: String,
    pub script: Script,
    pub language: Language,
    pub font: FontRef,
    pub glyphs: Vec<GlyphInfo>,
    pub positions: Vec<GlyphPosition>,
}

pub fn shape_run(text: &str, script: Script, language: Language, font: &Font) -> ShapedRun {
    let mut buffer = UnicodeBuffer::new();
    buffer.push_str(text);
    buffer.set_script(script);
    buffer.set_language(language);
    
    let output = rustybuzz::shape(font, &[], buffer);
    
    ShapedRun {
        text: text.to_string(),
        script,
        language,
        font: font.clone(),
        glyphs: output.glyph_infos().to_vec(),
        positions: output.glyph_positions().to_vec(),
    }
}
```

### 2. Glyph Positioning for DOCX

```rust
pub fn shaped_run_to_docx_runs(shaped: &ShapedRun) -> Vec<DocxRun> {
    let mut runs = Vec::new();
    
    for (glyph_info, glyph_pos) in shaped.glyphs.iter().zip(&shaped.positions) {
        let char_index = glyph_info.cluster as usize;
        let char = shaped.text.chars().nth(char_index).unwrap();
        
        let run = DocxRun {
            text: char.to_string(),
            font: shaped.font.name.clone(),
            advance: glyph_pos.x_advance,
            offset: (glyph_pos.x_offset, glyph_pos.y_offset),
        };
        runs.push(run);
    }
    
    runs
}
```

### 3. Glyph Positioning for PDF

```rust
pub fn shaped_run_to_pdf_glyphs(shaped: &ShapedRun) -> Vec<PdfGlyph> {
    let mut glyphs = Vec::new();
    let mut x_pos = 0;
    
    for (glyph_info, glyph_pos) in shaped.glyphs.iter().zip(&shaped.positions) {
        let glyph = PdfGlyph {
            glyph_id: glyph_info.codepoint,
            x: x_pos + glyph_pos.x_offset,
            y: glyph_pos.y_offset,
            advance: glyph_pos.x_advance,
        };
        glyphs.push(glyph);
        x_pos += glyph_pos.x_advance;
    }
    
    glyphs
}
```

## Key Capabilities

| Feature | Supported | Notes |
|---------|-----------|-------|
| Latin ligatures | ✅ | fi, fl, ff, ffi, ffl |
| Arabic shaping | ✅ | Initial, medial, final forms |
| Indic conjuncts | ✅ | Virama-based stacking |
| Balinese script | ✅ | Adeg, ulu, cecak combining |
| CJK positioning | ✅ | Vertical/horizontal variants |
| BiDi reordering | ❌ | Use unicode-bidi separately |
| Line breaking | ❌ | Use unicode-linebreak separately |

## Limitations

1. **BiDi not included** — Rustybuzz only shapes; use `unicode-bidi` for reordering
2. **No line breaking** — Use `unicode-linebreak` for break opportunities
3. **No font subsetting** — Use separate library (ttf-parser + custom logic)
4. **Single font per shape call** — Must call separately for each font

## Performance Characteristics

- **Font loading**: ~1-5ms per font (cached)
- **Shaping**: ~0.1-0.5ms per 100 characters
- **Memory**: ~1-2MB per font in memory

## Recommended Integration Points

1. **After script detection** — Shape each script run separately
2. **Before backend serialization** — Convert shaped glyphs to format-specific output
3. **With font fallback** — If glyph not found, try fallback font
4. **With caching** — Cache shaped runs to avoid reshaping identical text

## References

- [HarfBuzz Documentation](https://harfbuzz.github.io/)
- [Rustybuzz on crates.io](https://crates.io/crates/rustybuzz)
- [OpenType Feature Tags](https://docs.microsoft.com/en-us/typography/opentype/spec/featurelist)
