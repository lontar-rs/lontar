# Font Subsetting Research

## Overview

Font subsetting is the process of creating a smaller font file containing only the glyphs needed for a specific document. This reduces file size significantly (often 50-90%) while maintaining correct rendering.

## Problem Statement

Full font files are large:
- Noto Sans Balinese: ~150KB
- Noto Sans Devanagari: ~200KB
- SimSun (CJK): ~10MB

Embedding full fonts in every DOCX/PDF is wasteful. Subsetting extracts only used glyphs.

```
Original font: 150KB (contains 1000+ glyphs)
Document uses: 50 glyphs (ᬮᭀᬦ᭄ᬢᬭ᭄ and diacritics)
Subset font: 15KB (contains only 50 glyphs)
Savings: 90%
```

## Subsetting Algorithm

### Step 1: Collect Used Glyphs

```rust
pub fn collect_used_glyphs(text: &str, font: &Font) -> HashSet<u32> {
    let mut glyphs = HashSet::new();
    
    for ch in text.chars() {
        // Get glyph ID for character
        if let Some(glyph_id) = font.glyph_id(ch) {
            glyphs.insert(glyph_id);
        }
    }
    
    glyphs
}
```

### Step 2: Identify Dependent Glyphs

Some glyphs depend on others (e.g., composite glyphs, ligatures):

```rust
pub fn collect_dependent_glyphs(
    glyph_ids: &HashSet<u32>,
    font: &Font,
) -> HashSet<u32> {
    let mut all_glyphs = glyph_ids.clone();
    let mut queue: Vec<u32> = glyph_ids.iter().copied().collect();
    
    while let Some(glyph_id) = queue.pop() {
        // Get composite glyph components
        if let Some(components) = font.glyph_components(glyph_id) {
            for component_id in components {
                if !all_glyphs.contains(&component_id) {
                    all_glyphs.insert(component_id);
                    queue.push(component_id);
                }
            }
        }
    }
    
    all_glyphs
}
```

### Step 3: Create Subset Font

```rust
pub fn subset_font(
    font_data: &[u8],
    glyph_ids: &HashSet<u32>,
) -> Result<Vec<u8>> {
    // Parse original font
    let font = Font::from_bytes(font_data)?;
    
    // Create new font with only subset glyphs
    let mut subset = Font::new();
    
    // Copy font metadata
    subset.copy_metadata(&font);
    
    // Copy glyph tables
    for glyph_id in glyph_ids {
        let glyph_data = font.glyph_data(*glyph_id)?;
        subset.add_glyph(*glyph_id, glyph_data);
    }
    
    // Update font tables (cmap, hmtx, loca, etc.)
    subset.rebuild_tables()?;
    
    // Serialize to bytes
    subset.to_bytes()
}
```

## Rust Libraries for Font Subsetting

### 1. ttf-parser (Read-only)

```rust
use ttf_parser::Face;

let font_data = std::fs::read("font.ttf")?;
let face = Face::parse(&font_data, 0)?;

// Get glyph ID for character
let glyph_id = face.glyph_index('ᬮ')?;

// Get glyph metrics
let bbox = face.glyph_bounding_box(glyph_id)?;
let advance = face.glyph_hor_advance(glyph_id)?;
```

**Limitations:** Read-only; cannot create subset.

### 2. owned_ttf_parser (Owned variant)

```rust
use owned_ttf_parser::OwnedFace;

let font_data = std::fs::read("font.ttf")?;
let face = OwnedFace::from_vec(font_data, 0)?;

// Same API as ttf_parser, but owns the data
let glyph_id = face.glyph_index('ᬮ')?;
```

**Limitations:** Still read-only.

### 3. fonttools (Python, via subprocess)

For Rust, call fonttools via subprocess:

```rust
use std::process::Command;

pub fn subset_font_with_fonttools(
    input_path: &str,
    output_path: &str,
    glyphs: &[u32],
) -> Result<()> {
    let glyph_list = glyphs
        .iter()
        .map(|g| format!("gid{}", g))
        .collect::<Vec<_>>()
        .join(",");
    
    Command::new("fonttools")
        .args(&["subset", input_path])
        .arg(format!("--glyphs={}", glyph_list))
        .arg(format!("--output-file={}", output_path))
        .output()?;
    
    Ok(())
}
```

**Advantages:** Robust, handles all font formats.
**Disadvantages:** Requires Python and fonttools installed.

### 4. subsetter (Rust crate, if available)

Check crates.io for dedicated subsetting libraries:

```bash
cargo search font subsetter
```

As of 2026, no pure-Rust subsetting library is mature. Recommend:
- **For MVP:** Use fonttools via subprocess
- **For production:** Implement custom subsetter using ttf-parser

## OOXML Obfuscation

Embedded fonts in DOCX/PPTX are obfuscated using XOR with a GUID:

```rust
use uuid::Uuid;

pub fn obfuscate_font(font_data: &[u8], guid: &Uuid) -> Vec<u8> {
    let mut obfuscated = font_data.to_vec();
    let guid_bytes = guid.as_bytes();
    
    // XOR first 32 bytes with GUID (repeated)
    for i in 0..32.min(obfuscated.len()) {
        obfuscated[i] ^= guid_bytes[i % 16];
    }
    
    obfuscated
}

pub fn deobfuscate_font(font_data: &[u8], guid: &Uuid) -> Vec<u8> {
    // Same operation (XOR is symmetric)
    obfuscate_font(font_data, guid)
}
```

**Example:**

```rust
let font_data = std::fs::read("NotoSansBalinese-Regular.ttf")?;
let guid = Uuid::new_v4();
let obfuscated = obfuscate_font(&font_data, &guid);

// Store in DOCX
docx.add_embedded_font("NotoSansBalinese", &obfuscated, &guid)?;
```

## Font Embedding Permissions (fsType)

The OS/2 table in the font file contains an `fsType` field controlling embedding rights:

```rust
use ttf_parser::Face;

pub fn check_embedding_permission(font_data: &[u8]) -> Result<EmbeddingPermission> {
    let face = Face::parse(font_data, 0)?;
    
    // Get OS/2 table
    let os2 = face.os2()?;
    let fs_type = os2.fs_type();
    
    match fs_type {
        0 => Ok(EmbeddingPermission::Installable),      // Full embedding allowed
        2 => Ok(EmbeddingPermission::Restricted),       // No embedding
        4 => Ok(EmbeddingPermission::PreviewAndPrint),  // Preview/print only
        8 => Ok(EmbeddingPermission::Editable),         // Can be edited
        _ => Ok(EmbeddingPermission::Unknown),
    }
}

pub enum EmbeddingPermission {
    Installable,      // ✅ Embed fully
    Restricted,       // ❌ Do not embed
    PreviewAndPrint,  // ⚠️ Embed with restrictions
    Editable,         // ✅ Embed fully
    Unknown,          // ⚠️ Assume restricted
}
```

## Integration with Lontar

### 1. Collect Glyphs During Shaping

```rust
pub struct ShapedDocument {
    pub text: String,
    pub shaped_runs: Vec<ShapedRun>,
    pub used_glyphs: HashMap<String, HashSet<u32>>,  // Font name -> glyph IDs
}

pub fn shape_document(doc: &Document) -> Result<ShapedDocument> {
    let mut used_glyphs: HashMap<String, HashSet<u32>> = HashMap::new();
    let mut shaped_runs = Vec::new();
    
    for block in &doc.content {
        for inline in block.inlines() {
            let shaped = shape_inline(inline)?;
            
            // Collect glyphs
            let entry = used_glyphs
                .entry(shaped.font.name.clone())
                .or_insert_with(HashSet::new);
            
            for glyph_info in &shaped.glyphs {
                entry.insert(glyph_info.codepoint);
            }
            
            shaped_runs.push(shaped);
        }
    }
    
    Ok(ShapedDocument {
        text: doc.to_string(),
        shaped_runs,
        used_glyphs,
    })
}
```

### 2. Subset Fonts Before Embedding

```rust
pub fn embed_fonts_in_docx(
    docx: &mut DocxDocument,
    shaped_doc: &ShapedDocument,
) -> Result<()> {
    for (font_name, glyph_ids) in &shaped_doc.used_glyphs {
        // Load original font
        let font_data = load_font(font_name)?;
        
        // Check embedding permission
        match check_embedding_permission(&font_data)? {
            EmbeddingPermission::Restricted => {
                // Skip embedding; use system font fallback
                continue;
            }
            _ => {}
        }
        
        // Subset font
        let subset_data = subset_font(&font_data, glyph_ids)?;
        
        // Obfuscate
        let guid = Uuid::new_v4();
        let obfuscated = obfuscate_font(&subset_data, &guid);
        
        // Embed in DOCX
        docx.add_embedded_font(font_name, &obfuscated, &guid)?;
    }
    
    Ok(())
}
```

### 3. Track Subsetting Metrics

```rust
pub struct SubsettingMetrics {
    pub font_name: String,
    pub original_size: usize,
    pub subset_size: usize,
    pub glyph_count_original: u32,
    pub glyph_count_subset: u32,
    pub compression_ratio: f32,
}

pub fn compute_metrics(
    font_name: &str,
    original_data: &[u8],
    subset_data: &[u8],
    glyph_count_original: u32,
    glyph_count_subset: u32,
) -> SubsettingMetrics {
    let compression_ratio =
        (subset_data.len() as f32) / (original_data.len() as f32);
    
    SubsettingMetrics {
        font_name: font_name.to_string(),
        original_size: original_data.len(),
        subset_size: subset_data.len(),
        glyph_count_original,
        glyph_count_subset,
        compression_ratio,
    }
}
```

## Recommended Approach for Lontar

### Phase 1 (MVP): Fonttools Integration

Use fonttools via subprocess for robust subsetting:

```rust
pub fn subset_font_mvp(
    font_path: &str,
    output_path: &str,
    text: &str,
) -> Result<()> {
    // Collect characters
    let chars = text.chars().collect::<Vec<_>>();
    let char_list = chars
        .iter()
        .map(|c| format!("U+{:04X}", *c as u32))
        .collect::<Vec<_>>()
        .join(",");
    
    // Call fonttools
    Command::new("fonttools")
        .args(&["subset", font_path])
        .arg(format!("--unicodes={}", char_list))
        .arg(format!("--output-file={}", output_path))
        .output()?;
    
    Ok(())
}
```

### Phase 2: Pure Rust Implementation

Implement custom subsetter using ttf-parser:

```rust
pub fn subset_font_rust(
    font_data: &[u8],
    text: &str,
) -> Result<Vec<u8>> {
    let face = Face::parse(font_data, 0)?;
    
    // Collect glyphs
    let mut glyph_ids = HashSet::new();
    for ch in text.chars() {
        if let Some(glyph_id) = face.glyph_index(ch) {
            glyph_ids.insert(glyph_id);
        }
    }
    
    // Collect dependent glyphs
    let all_glyphs = collect_dependent_glyphs(&glyph_ids, &face);
    
    // Create subset (custom implementation)
    subset_font_impl(font_data, &all_glyphs)
}
```

## Performance Characteristics

| Operation | Time | Notes |
|-----------|------|-------|
| Collect glyphs | ~1ms per 1000 chars | Fast |
| Fonttools subset | ~100-500ms | Depends on font size |
| Obfuscation | ~1ms per 100KB | Fast |
| Total embedding | ~200-600ms | Acceptable for document generation |

## File Size Savings

| Font | Original | Subset (Latin) | Subset (Balinese) | Savings |
|------|----------|---|---|---|
| Noto Sans Balinese | 150KB | 30KB | 45KB | 70-70% |
| Noto Sans Devanagari | 200KB | 40KB | 80KB | 60-80% |
| SimSun (CJK) | 10MB | 500KB | 2MB | 95-80% |

## References

- [fonttools Documentation](https://fonttools.readthedocs.io/)
- [ttf-parser on crates.io](https://crates.io/crates/ttf-parser)
- [ECMA-376: Font Embedding](https://www.ecma-international.org/publications-and-standards/standards/ecma-376/)
- [OpenType Specification](https://docs.microsoft.com/en-us/typography/opentype/spec/)
