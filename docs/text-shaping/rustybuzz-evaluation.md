# Rustybuzz Evaluation

## Overview

**rustybuzz** is a pure Rust port of HarfBuzz, the industry-standard OpenType text shaping engine. It transforms Unicode text + font into positioned glyphs with correct conjuncts, ligatures, reordering, and mark positioning.

- **Crate:** `rustybuzz` (https://crates.io/crates/rustybuzz)
- **Latest version:** 0.13+
- **License:** Apache 2.0 / MIT
- **Repository:** https://github.com/RustType/rustybuzz
- **Status:** Actively maintained, production-ready

## Why Rustybuzz?

1. **Pure Rust** — No C dependencies, no FFI overhead
2. **HarfBuzz-compatible** — Implements same algorithm as industry standard
3. **Comprehensive script support** — All 159+ Unicode scripts
4. **Complex script shaping** — Handles conjuncts, ligatures, reordering, mark positioning
5. **OpenType features** — Full support for GSUB/GPOS tables

## API Overview

### Core Types

```rust
use rustybuzz::{Face, Font, UnicodeBuffer, shape};

// Load font from bytes
let font_data = std::fs::read("NotoSansBalinese-Regular.ttf")?;
let face = Face::from_slice(&font_data, 0)?;
let font = Font::new(face);

// Create text buffer
let mut buffer = UnicodeBuffer::new();
buffer.push_str("ᬮᭀᬦ᭄ᬢᬭ᭄");  // Balinese "lontar"
buffer.set_script(rustybuzz::Script::Balinese);
buffer.set_language(rustybuzz::Language::new(b"ban"));  // Balinese language

// Shape text
let glyph_buffer = shape(&font, &buffer);

// Extract results
for glyph_info in glyph_buffer.glyph_infos() {
    println!("Glyph ID: {}, Cluster: {}", glyph_info.codepoint, glyph_info.cluster);
}

for glyph_pos in glyph_buffer.glyph_positions() {
    println!("X advance: {}, Y advance: {}", glyph_pos.x_advance, glyph_pos.y_advance);
}
```

### Key Structs

| Type | Purpose |
|------|---------|
| `Face` | Parsed font file (immutable) |
| `Font` | Font with scale/ppem settings |
| `UnicodeBuffer` | Input text with metadata (script, language, direction) |
| `GlyphBuffer` | Output glyphs with positions |
| `GlyphInfo` | Glyph ID and cluster index |
| `GlyphPosition` | Glyph advance and offset |

### Script Detection

```rust
use rustybuzz::Script;

// Explicit script assignment
buffer.set_script(Script::Balinese);
buffer.set_script(Script::Arabic);
buffer.set_script(Script::Devanagari);

// Available scripts: Latin, Arabic, Hebrew, Devanagari, Bengali, Tamil, Telugu,
// Kannada, Malayalam, Thai, Lao, Khmer, Myanmar, Balinese, CJK, Hiragana,
// Katakana, Hangul, and 140+ others
```

### Language Tagging

```rust
use rustybuzz::Language;

// BCP 47 language tags
buffer.set_language(Language::new(b"en"));      // English
buffer.set_language(Language::new(b"ar"));      // Arabic
buffer.set_language(Language::new(b"ban"));     // Balinese
buffer.set_language(Language::new(b"hi"));      // Hindi (Devanagari)
buffer.set_language(Language::new(b"zh-CN"));   // Simplified Chinese
```

### Direction (BiDi)

```rust
use rustybuzz::Direction;

// Explicit direction
buffer.set_direction(Direction::Ltr);  // Left-to-right (default)
buffer.set_direction(Direction::Rtl);  // Right-to-left (Arabic, Hebrew)
buffer.set_direction(Direction::Ttb);  // Top-to-bottom (vertical CJK)
buffer.set_direction(Direction::Btt);  // Bottom-to-top (rare)
```

## Capabilities

### ✅ Supported Features

- **Conjunct formation** — Indic scripts (Devanagari क्ष, Balinese ᬦ᭄ᬢ)
- **Ligatures** — fi, fl, ffi, ffl (Latin); contextual (Arabic)
- **Mark positioning** — Diacritics, vowel signs, tone marks
- **Contextual alternates** — Arabic letter forms (initial, medial, final, isolated)
- **Reordering** — Pre-base matras (Indic), reph (Devanagari)
- **Kerning** — Pair-based glyph adjustment
- **OpenType features** — GSUB (substitution), GPOS (positioning)
- **RTL text** — Arabic, Hebrew, Syriac, Thaana, N'Ko
- **CJK** — Simplified/Traditional Chinese, Japanese, Korean

### ⚠️ Limitations

- **No font subsetting** — Must use separate tool (fonttools, ttf-parser)
- **No glyph rendering** — Only produces glyph IDs and positions; use FreeType/fontdue for rasterization
- **No line breaking** — Use `unicode-linebreak` crate separately
- **No BiDi algorithm** — Use `unicode-bidi` crate for mixed LTR/RTL
- **No script detection** — Use `unicode-script` crate to segment mixed-script text

## Performance

**Benchmarks (approximate, on modern CPU):**

| Text Length | Time | Throughput |
|---|---|---|
| 10 chars | ~100 µs | 100 K glyphs/sec |
| 100 chars | ~500 µs | 200 K glyphs/sec |
| 1000 chars | ~4 ms | 250 K glyphs/sec |

**Factors affecting performance:**
- Font complexity (number of OpenType features)
- Script complexity (Indic scripts slower than Latin)
- Language-specific rules (Arabic contextual forms)

**Optimization tips:**
- Reuse `Font` object across multiple shape calls
- Batch shape operations when possible
- Cache font data in memory

## Integration with Lontar

### Architecture

```
Input Text
    ↓
[unicode-script] — Segment into script runs
    ↓
For each run:
  1. Detect script (Latin, Arabic, Devanagari, etc.)
  2. Select font (Noto Sans, Noto Sans Arabic, etc.)
  3. [rustybuzz] — Shape with OpenType engine
  4. Extract glyphs + positions
    ↓
Output: ShapedRun { glyphs, positions, font, script }
```

### Proposed API

```rust
pub struct ShapedRun {
    pub text: String,
    pub script: Script,
    pub language: Language,
    pub font: FontRef,
    pub glyph_infos: Vec<GlyphInfo>,
    pub glyph_positions: Vec<GlyphPosition>,
}

pub fn shape_text(
    text: &str,
    font: &Font,
    script: Script,
    language: Language,
) -> Result<ShapedRun, ShapingError> {
    let mut buffer = UnicodeBuffer::new();
    buffer.push_str(text);
    buffer.set_script(script);
    buffer.set_language(language);
    
    let glyph_buffer = rustybuzz::shape(&font, &buffer);
    
    Ok(ShapedRun {
        text: text.to_string(),
        script,
        language,
        font: font.clone(),
        glyph_infos: glyph_buffer.glyph_infos().to_vec(),
        glyph_positions: glyph_buffer.glyph_positions().to_vec(),
    })
}
```

## Example: Balinese Shaping

```rust
use rustybuzz::{Face, Font, UnicodeBuffer, Script, Language, shape};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load Noto Sans Balinese
    let font_data = std::fs::read("NotoSansBalinese-Regular.ttf")?;
    let face = Face::from_slice(&font_data, 0)?;
    let font = Font::new(face);
    
    // Shape "ᬮᭀᬦ᭄ᬢᬭ᭄" (lontar)
    let mut buffer = UnicodeBuffer::new();
    buffer.push_str("ᬮᭀᬦ᭄ᬢᬭ᭄");
    buffer.set_script(Script::Balinese);
    buffer.set_language(Language::new(b"ban"));
    
    let glyph_buffer = shape(&font, &buffer);
    
    println!("Input: ᬮᭀᬦ᭄ᬢᬭ᭄");
    println!("Output glyphs:");
    for (info, pos) in glyph_buffer.glyph_infos()
        .iter()
        .zip(glyph_buffer.glyph_positions().iter())
    {
        println!("  Glyph ID: {}, Cluster: {}, X advance: {}",
            info.codepoint, info.cluster, pos.x_advance);
    }
    
    // Expected: conjunct formation (ᬦ᭄ᬢ → single glyph)
    Ok(())
}
```

## Comparison with Alternatives

| Crate | Language | Completeness | Performance | Maintenance |
|---|---|---|---|---|
| **rustybuzz** | Rust | 100% (HarfBuzz port) | Good | Active |
| harfbuzz-sys | Rust FFI | 100% (C binding) | Excellent | Active |
| fontdue | Rust | Partial (basic shaping) | Excellent | Active |
| skribo | Rust | Partial (layout only) | Good | Inactive |

**Recommendation:** Use `rustybuzz` for comprehensive script support without C dependencies.

## Next Steps

1. Add `rustybuzz` to `lontar-aksara` dependencies
2. Implement `ShapedRun` struct and `shape_text()` function
3. Create Balinese shaping prototype (see `examples/balinese_shaping.rs`)
4. Integrate with `unicode-script` for script detection
5. Integrate with `unicode-bidi` for bidirectional text

## References

- [Rustybuzz GitHub](https://github.com/RustType/rustybuzz)
- [HarfBuzz Documentation](https://harfbuzz.github.io/)
- [OpenType Specification](https://docs.microsoft.com/en-us/typography/opentype/)
- [Unicode Script Property](https://www.unicode.org/reports/tr24/)
