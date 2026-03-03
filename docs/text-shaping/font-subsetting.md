# Font Subsetting Approaches in Rust

## Overview

Font subsetting extracts only the glyphs used in a document, reducing file size significantly. This is critical for embedding fonts in DOCX, PPTX, and PDF formats.

**Problem:** A full font file can be 1-5 MB; a subset for a specific document might be 50-200 KB.

**Solution:** Identify used glyphs, extract them, and rebuild the font file.

## Approaches

### 1. Python fonttools (via FFI)

**Pros:**
- Industry standard, battle-tested
- Comprehensive font manipulation
- Handles all font formats

**Cons:**
- Requires Python installation
- FFI overhead
- Not ideal for Rust-only projects

**Usage:**
```bash
pyftsubset input.ttf --unicodes=U+0041-U+005A,U+0061-U+007A
```

### 2. Rust Crates

#### ttf-parser

**Crate:** `ttf-parser` (https://crates.io/crates/ttf-parser)

**Pros:**
- Pure Rust, no dependencies
- Fast parsing
- Supports most font tables

**Cons:**
- Read-only (no font rebuilding)
- Limited to TrueType/OpenType
- No subsetting built-in

**Usage:**
```rust
use ttf_parser::Face;

let font_data = std::fs::read("font.ttf")?;
let face = Face::parse(&font_data, 0)?;

// Query glyph IDs for characters
for ch in "Hello".chars() {
    if let Some(glyph_id) = face.glyph_index(ch) {
        println!("'{}' → Glyph ID {}", ch, glyph_id.0);
    }
}
```

#### fontdue

**Crate:** `fontdue` (https://crates.io/crates/fontdue)

**Pros:**
- Pure Rust
- Includes rasterization
- Fast

**Cons:**
- Primarily for rendering
- Limited subsetting support
- No font rebuilding

#### owned_ttf_parser

**Crate:** `owned_ttf_parser` (https://crates.io/crates/owned_ttf_parser)

**Pros:**
- Owned font data (no lifetime issues)
- Pure Rust

**Cons:**
- Similar limitations to ttf-parser
- No subsetting

### 3. Custom Implementation

**Approach:** Parse font tables, identify used glyphs, rebuild font file.

**Steps:**
1. Parse font header and table directory
2. Identify used glyphs (from character set)
3. Build glyph remapping table
4. Copy required tables (cmap, glyf, loca, hmtx, etc.)
5. Update table offsets and checksums
6. Write new font file

**Pros:**
- Full control
- No external dependencies
- Optimized for specific use case

**Cons:**
- Complex implementation
- Requires deep font format knowledge
- Error-prone

## Recommended Strategy for Lontar

### Phase 1 (MVP)

**Use:** Python fonttools via subprocess

```rust
use std::process::Command;

pub fn subset_font(
    input_path: &Path,
    output_path: &Path,
    used_glyphs: &[char],
) -> Result<(), Box<dyn std::error::Error>> {
    // Build Unicode ranges from used glyphs
    let unicode_ranges = build_unicode_ranges(used_glyphs);
    
    // Call fonttools
    let output = Command::new("pyftsubset")
        .arg(input_path)
        .arg(format!("--unicodes={}", unicode_ranges))
        .arg(format!("--output-file={}", output_path.display()))
        .output()?;
    
    if !output.status.success() {
        return Err("fonttools subsetting failed".into());
    }
    
    Ok(())
}

fn build_unicode_ranges(glyphs: &[char]) -> String {
    // Convert character set to Unicode ranges
    // E.g., [a-z, A-Z, 0-9] → "U+0041-U+005A,U+0061-U+007A,U+0030-U+0039"
    let mut ranges = Vec::new();
    let mut sorted: Vec<u32> = glyphs.iter().map(|&c| c as u32).collect();
    sorted.sort();
    sorted.dedup();
    
    let mut start = sorted[0];
    let mut end = sorted[0];
    
    for &code in &sorted[1..] {
        if code == end + 1 {
            end = code;
        } else {
            ranges.push(format!("U+{:04X}-U+{:04X}", start, end));
            start = code;
            end = code;
        }
    }
    ranges.push(format!("U+{:04X}-U+{:04X}", start, end));
    
    ranges.join(",")
}
```

**Requirements:**
```bash
pip install fonttools
```

### Phase 2 (Optimization)

**Use:** ttf-parser + custom rebuilding

```rust
use ttf_parser::Face;

pub fn identify_used_glyphs(
    font_data: &[u8],
    text: &str,
) -> Result<Vec<u16>, Box<dyn std::error::Error>> {
    let face = Face::parse(font_data, 0)?;
    let mut glyph_ids = Vec::new();
    
    for ch in text.chars() {
        if let Some(glyph_id) = face.glyph_index(ch) {
            glyph_ids.push(glyph_id.0);
        }
    }
    
    glyph_ids.sort();
    glyph_ids.dedup();
    Ok(glyph_ids)
}
```

### Phase 3+ (Full Rust Implementation)

Implement custom font subsetting in Rust (if needed for performance or to eliminate Python dependency).

## Integration with Lontar

### DOCX Backend

```rust
pub fn embed_font_in_docx(
    docx: &mut DocxBuilder,
    font_path: &Path,
    text: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Identify used glyphs
    let font_data = std::fs::read(font_path)?;
    let used_glyphs = identify_used_glyphs(&font_data, text)?;
    
    // Step 2: Subset font
    let subset_path = temp_dir().join("subset.ttf");
    subset_font(font_path, &subset_path, &used_glyphs)?;
    
    // Step 3: Obfuscate and embed
    let subset_data = std::fs::read(&subset_path)?;
    let obfuscated = obfuscate_font(&subset_data, &uuid::Uuid::new_v4());
    
    // Step 4: Add to DOCX
    docx.add_embedded_font(&obfuscated)?;
    
    Ok(())
}
```

### PPTX Backend

Similar approach to DOCX.

### PDF Backend

PDF has different embedding requirements (font subsetting + CID fonts).

## Performance Considerations

| Approach | Time | File Size | Dependencies |
|---|---|---|---|
| Python fonttools | 100-500 ms | Minimal | Python, fonttools |
| ttf-parser (identify only) | 1-10 ms | N/A | Rust crate |
| Custom Rust impl | 10-100 ms | Minimal | None |

**Recommendation:** Start with Python fonttools (Phase 1), optimize to Rust later if needed.

## Testing

```rust
#[test]
fn test_font_subsetting() {
    let font_path = Path::new("tests/fixtures/scripts/fonts/NotoSans-Regular.ttf");
    let text = "Hello";
    
    let used_glyphs = identify_used_glyphs(
        &std::fs::read(font_path).unwrap(),
        text,
    ).unwrap();
    
    assert!(used_glyphs.len() > 0);
    assert!(used_glyphs.len() <= 5);  // At most 5 glyphs for "Hello"
}
```

## References

- [fonttools Documentation](https://fonttools.readthedocs.io/)
- [ttf-parser Crate](https://crates.io/crates/ttf-parser)
- [OpenType Specification](https://docs.microsoft.com/en-us/typography/opentype/)
- [Font Subsetting Best Practices](https://web.dev/reduce-webfont-size/)
