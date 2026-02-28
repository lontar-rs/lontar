# Font Embedding and Font Management in OOXML (DOCX and PPTX)

## Overview

Font management in OOXML is critical for ensuring documents render correctly across different systems and scripts. This document covers font references, embedding, subsetting, and language tagging for both DOCX and PPTX, with special attention to complex scripts like Arabic, Indic scripts, and Balinese (Aksara Bali).

This is a foundational reference for implementing the lontar-aksara text shaping pipeline with OOXML backends.

---

## 1. Font References in DOCX

### The w:rFonts Element

The `w:rFonts` element in run properties specifies which fonts to use for different character ranges:

```xml
<w:r>
  <w:rPr>
    <w:rFonts w:ascii="Calibri"
              w:hAnsi="Calibri"
              w:cs="Calibri"
              w:eastAsia="SimSun"/>
  </w:rPr>
  <w:t>Hello مرحبا 你好</w:t>
</w:r>
```

### Font Slot Attributes

| Attribute | Character Range | Purpose | Notes |
|---|---|---|---|
| `w:ascii` | U+0000–U+007F | ASCII characters (Latin A-Z, 0-9, punctuation) | Fallback for unspecified ranges |
| `w:hAnsi` | U+0080–U+00FF | High-ANSI (Latin Extended, Western European) | Includes accented characters (é, ñ, ü) |
| `w:cs` | Complex scripts | Arabic, Hebrew, Indic, Southeast Asian (including Balinese) | Requires shaping engine (HarfBuzz) |
| `w:eastAsia` | CJK (Chinese, Japanese, Korean) | East Asian characters | U+4E00–U+9FFF (CJK Unified Ideographs) |

### Font Selection Algorithm

Word uses this algorithm to select a font for each character:

```
1. Determine the Unicode range of the character
2. If character is in ASCII range (U+0000–U+007F):
   → Use w:ascii font
3. Else if character is in High-ANSI range (U+0080–U+00FF):
   → Use w:hAnsi font
4. Else if character is in East Asian range (CJK, Hiragana, Katakana):
   → Use w:eastAsia font
5. Else if character is in complex script range (Arabic, Indic, etc.):
   → Use w:cs font
6. Else:
   → Use w:ascii font (fallback)
```

### The w:hint Attribute

The `w:hint` attribute provides a hint to the rendering engine about which font slot is primary:

```xml
<w:rFonts w:ascii="Calibri" w:hAnsi="Calibri" w:cs="Calibri" w:hint="default"/>
<!-- w:hint values: default, eastAsia, cs -->
```

- `w:hint="default"` — Standard behavior (use algorithm above)
- `w:hint="eastAsia"` — Prioritize East Asian font (for CJK-heavy documents)
- `w:hint="cs"` — Prioritize complex script font (for Arabic/Indic-heavy documents)

### XML Examples

#### Example 1: Latin-Only Text

```xml
<w:r>
  <w:rPr>
    <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/>
  </w:rPr>
  <w:t>Hello World</w:t>
</w:r>
```

All characters are ASCII, so `w:ascii` is used. `w:hAnsi` is specified as fallback for accented characters if present.

#### Example 2: Arabic Text

```xml
<w:r>
  <w:rPr>
    <w:rFonts w:ascii="Calibri" w:cs="Arial"/>
    <w:rtl/>  <!-- Right-to-left flag -->
  </w:rPr>
  <w:t>مرحبا</w:t>
</w:r>
```

Arabic characters (U+0600–U+06FF) are complex scripts, so `w:cs` font is used. The `w:rtl` flag indicates right-to-left text direction.

#### Example 3: Mixed Latin + Arabic in One Run

```xml
<w:r>
  <w:rPr>
    <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri" w:cs="Arial"/>
  </w:rPr>
  <w:t>Hello مرحبا World</w:t>
</w:r>
```

- "Hello" and "World" use `w:ascii`/`w:hAnsi` (Calibri)
- "مرحبا" uses `w:cs` (Arial)

**Best practice:** Split into separate runs for better control:

```xml
<w:r>
  <w:rPr>
    <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/>
  </w:rPr>
  <w:t>Hello </w:t>
</w:r>

<w:r>
  <w:rPr>
    <w:rFonts w:cs="Arial"/>
    <w:rtl/>
  </w:rPr>
  <w:t>مرحبا</w:t>
</w:r>

<w:r>
  <w:rPr>
    <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/>
  </w:rPr>
  <w:t> World</w:t>
</w:r>
```

#### Example 4: Mixed Latin + CJK

```xml
<w:r>
  <w:rPr>
    <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri" w:eastAsia="SimSun"/>
  </w:rPr>
  <w:t>Hello 你好 World</w:t>
</w:r>
```

- "Hello" and "World" use `w:ascii` (Calibri)
- "你好" uses `w:eastAsia` (SimSun, a CJK font)

#### Example 5: Balinese Script Text (Aksara Bali)

Balinese script (U+1B00–U+1B7F) is a complex script, so it uses the `w:cs` font slot:

```xml
<w:r>
  <w:rPr>
    <w:rFonts w:ascii="Calibri" w:cs="Noto Sans Balinese"/>
    <w:lang w:val="en-US" w:bidi="ban"/>  <!-- ban = Balinese -->
  </w:rPr>
  <w:t>ᬅᬓ᭄ᬱᬭ</w:t>  <!-- Balinese text -->
</w:r>
```

The `w:cs` font must support Balinese characters. Recommended fonts:
- Noto Sans Balinese
- Aksara Bali (if available)

---

## 2. Font Table (word/fontTable.xml)

### File Structure

The `word/fontTable.xml` file lists all fonts used in the document:

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:fontTable xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
             xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  
  <w:font w:name="Calibri">
    <w:panose1 w:val="020B0604030504040204"/>
    <w:charset w:val="00"/>
    <w:family w:val="swiss"/>
    <w:pitch w:val="variable"/>
    <w:sig w:usb0="E0002AFF" w:usb1="C0000000" w:usb2="00000000" w:usb3="00000000"
           w:csb0="00000001" w:csb1="00000000"/>
  </w:font>
  
  <w:font w:name="Arial">
    <w:panose1 w:val="020B0604020202020204"/>
    <w:charset w:val="00"/>
    <w:family w:val="swiss"/>
    <w:pitch w:val="variable"/>
    <w:sig w:usb0="E0002AFF" w:usb1="C0000000" w:usb2="00000000" w:usb3="00000000"
           w:csb0="00000001" w:csb1="00000000"/>
  </w:font>
  
  <w:font w:name="Noto Sans Balinese">
    <w:panose1 w:val="020B0604030504040204"/>
    <w:charset w:val="00"/>
    <w:family w:val="swiss"/>
    <w:pitch w:val="variable"/>
    <w:sig w:usb0="00000000" w:usb1="00000000" w:usb2="00000000" w:usb3="00000010"
           w:csb0="00000000" w:csb1="00000000"/>
  </w:font>
  
</w:fontTable>
```

### w:font Element Attributes

| Attribute | Purpose | Values |
|---|---|---|
| `w:name` | Font name | Any font name (e.g., "Calibri", "Arial", "Noto Sans Balinese") |
| `w:panose1` | PANOSE-1 classification | 10-byte hex string (font metrics) |
| `w:charset` | Character set | "00" = default (ANSI) |
| `w:family` | Font family | roman, swiss, modern, script, decorative |
| `w:pitch` | Pitch type | fixed, variable |

### w:sig (Font Signature)

The `w:sig` element describes which Unicode ranges the font covers:

```xml
<w:sig w:usb0="E0002AFF" w:usb1="C0000000" w:usb2="00000000" w:usb3="00000000"
       w:csb0="00000001" w:csb1="00000000"/>
```

- `w:usb0–w:usb3` — Unicode Subrange Bitmask (4 × 32-bit values)
- `w:csb0–w:csb1` — Code Page Coverage Bitmask (2 × 32-bit values)

**Example: Font supports Latin (usb0 bit 0) and Arabic (usb1 bit 4):**

```
usb0 = 0x00000001  (bit 0 set = Latin)
usb1 = 0x00000010  (bit 4 set = Arabic)
usb2 = 0x00000000
usb3 = 0x00000000
```

**Common Unicode ranges:**
- Bit 0 (usb0): Basic Latin (U+0000–U+007F)
- Bit 1 (usb0): Latin Extended-A (U+0100–U+017F)
- Bit 4 (usb1): Arabic (U+0600–U+06FF)
- Bit 15 (usb1): Devanagari (U+0900–U+097F)
- Bit 29 (usb3): Balinese (U+1B00–U+1B7F)

### Font Table Relationship

The document must reference the font table:

**In `word/_rels/document.xml.rels`:**
```xml
<Relationship Id="rId1"
             Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable"
             Target="fontTable.xml"/>
```

**In `[Content_Types].xml`:**
```xml
<Override PartName="/word/fontTable.xml"
         ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml"/>
```

---

## 3. Font Embedding in DOCX

### Embedding Elements in Styles

Font embedding is declared in `word/styles.xml` or `word/document.xml`:

```xml
<w:rPr>
  <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri" w:cs="Noto Sans Balinese"/>
  
  <!-- Embedding declarations -->
  <w:embedRegular w:fontKey="00000000" w:subsetted="1"/>
  <w:embedBold w:fontKey="00000000" w:subsetted="1"/>
  <w:embedItalic w:fontKey="00000000" w:subsetted="1"/>
  <w:embedBoldItalic w:fontKey="00000000" w:subsetted="1"/>
</w:rPr>
```

### Embedded Font Storage

Embedded fonts are stored in the `word/embeddings/` directory:

```
document.docx
├── word/
│   ├── document.xml
│   ├── styles.xml
│   ├── embeddings/
│   │   ├── font1.odttf  (obfuscated TrueType)
│   │   ├── font2.odttf
│   │   └── font3.odttf
│   └── ...
└── ...
```

### Font Obfuscation

Embedded fonts are obfuscated using XOR with a GUID to protect against unauthorized use:

```
Obfuscated bytes = Original bytes XOR (GUID repeated)
```

**Example obfuscation:**
```
GUID: 12345678-1234-5678-1234-567812345678
Repeated: 12345678123456781234567812345678...

Original font bytes: [0x00, 0x01, 0x02, 0x03, ...]
GUID bytes:         [0x12, 0x34, 0x56, 0x78, ...]
Obfuscated:         [0x12, 0x35, 0x54, 0x7B, ...]
```

The first 32 bytes of the font file are obfuscated; the rest are stored as-is.

### Embedding Flags

The `w:fontKey` attribute is the GUID used for obfuscation. The `w:subsetted` attribute indicates whether only used glyphs are embedded:

```xml
<w:embedRegular w:fontKey="12345678-1234-5678-1234-567812345678" w:subsetted="1"/>
<!-- w:subsetted="1" = only used glyphs embedded (smaller file) -->
<!-- w:subsetted="0" = full font embedded (larger file) -->
```

### Embedding Permissions (fsType)

The OS/2 table in the font file contains an `fsType` field that controls embedding rights:

| fsType Value | Meaning | Can Embed? |
|---|---|---|
| 0 | Installable | ✅ Yes, full embedding allowed |
| 2 | Restricted | ❌ No embedding allowed |
| 4 | Preview & Print | ✅ Yes, but only for preview/print |
| 8 | Editable | ✅ Yes, can be edited |

**Lontar strategy:** Check `fsType` before embedding:
- If `fsType == 0`: Embed fully
- If `fsType == 4`: Embed with restrictions
- If `fsType == 2`: Do not embed; use system font fallback

### Font Subsetting Approach

**Full embedding:**
```xml
<w:embedRegular w:fontKey="..." w:subsetted="0"/>
```
Embeds the entire font file. Larger file size but guarantees all characters render.

**Subsetting (recommended):**
```xml
<w:embedRegular w:fontKey="..." w:subsetted="1"/>
```
Embeds only glyphs used in the document. Smaller file size, but requires tracking which glyphs are used.

**Lontar implementation:**
1. Scan document for all characters
2. Query font for glyph IDs for those characters
3. Subset font to include only those glyphs
4. Obfuscate and embed

---

## 4. Font References in PPTX

### DrawingML Run Properties

In PPTX, fonts are specified in DrawingML run properties (`a:rPr`):

```xml
<a:r>
  <a:rPr lang="en-US" dirty="0" smtClean="0">
    <a:latin typeface="Calibri" pitchFamily="2" charset="0"/>
    <a:ea typeface="SimSun" pitchFamily="2" charset="0"/>
    <a:cs typeface="Arial" pitchFamily="2" charset="0"/>
  </a:rPr>
  <a:t>Hello مرحبا 你好</a:t>
</a:r>
```

### DrawingML Font Elements

| Element | Purpose | Attributes |
|---|---|---|
| `a:latin` | Latin/ASCII font | typeface, pitchFamily, charset |
| `a:ea` | East Asian (CJK) font | typeface, pitchFamily, charset |
| `a:cs` | Complex script font | typeface, pitchFamily, charset |

### pitchFamily Attribute

The `pitchFamily` attribute encodes pitch and family information:

```
pitchFamily = (pitch << 4) | family

pitch: 0 = not set, 1 = fixed, 2 = variable
family: 0 = not set, 1 = roman, 2 = swiss, 3 = modern, 4 = script, 5 = decorative
```

**Examples:**
- `pitchFamily="2"` = variable pitch, not set family (0x2)
- `pitchFamily="18"` = variable pitch (2), swiss family (2) = 0x22

### Theme Fonts

PPTX supports theme fonts that can be overridden per script:

```xml
<a:rPr lang="en-US">
  <a:latin typeface="Calibri"/>
  <a:ea typeface="SimSun"/>
  <a:cs typeface="Arial"/>
  <a:latin typeface="Calibri Light"/>  <!-- Override for specific context -->
</a:rPr>
```

### PPTX Font Embedding

Fonts in PPTX are embedded in the `ppt/fonts/` directory:

```
presentation.pptx
├── ppt/
│   ├── slides/
│   ├── fonts/
│   │   ├── font1.odttf
│   │   ├── font2.odttf
│   │   └── font3.odttf
│   └── ...
└── ...
```

Embedded fonts are referenced in slide XML:

```xml
<a:rPr>
  <a:latin typeface="CustomFont"/>
  <a:fontRef idx="1"/>  <!-- Reference to embedded font -->
</a:rPr>
```

---

## 5. Language Tagging

### Language Tags in DOCX

The `w:lang` element specifies language for spell-checking and text shaping:

```xml
<w:rPr>
  <w:lang w:val="en-US" w:eastAsia="zh-CN" w:bidi="ar-SA"/>
</w:rPr>
```

| Attribute | Purpose | Example |
|---|---|---|
| `w:val` | Latin/default language | en-US, fr-FR, de-DE |
| `w:eastAsia` | East Asian language | zh-CN, ja-JP, ko-KR |
| `w:bidi` | Bidirectional (RTL) language | ar-SA, he-IL, fa-IR, ban (Balinese) |

### BCP 47 Language Tags

Language tags follow BCP 47 format: `language[-script][-region]`

**Examples relevant to Lontar:**

| Tag | Language | Script | Region | Notes |
|---|---|---|---|---|
| en-US | English | Latin | United States | Default |
| id-ID | Indonesian | Latin | Indonesia | Uses Latin script |
| ban | Balinese | Balinese | (none) | Aksara Bali script |
| ar-SA | Arabic | Arabic | Saudi Arabia | RTL script |
| ar-EG | Arabic | Arabic | Egypt | RTL script |
| hi-IN | Hindi | Devanagari | India | Complex script |
| zh-CN | Chinese | Han | China (Simplified) | CJK script |
| zh-TW | Chinese | Han | Taiwan (Traditional) | CJK script |
| ja-JP | Japanese | Hiragana/Katakana/Kanji | Japan | CJK script |
| ko-KR | Korean | Hangul | South Korea | CJK script |

### Language Tagging in PPTX

In PPTX, language is specified in the `lang` attribute of `a:rPr`:

```xml
<a:rPr lang="en-US">
  <a:latin typeface="Calibri"/>
</a:rPr>

<a:rPr lang="ar-SA">
  <a:cs typeface="Arial"/>
</a:rPr>
```

### Effects of Language Tagging

Language tags affect:

1. **Spell-checking** — Word uses the language to select the correct dictionary
2. **Hyphenation** — Different languages have different hyphenation rules
3. **Font selection** — Some fonts are optimized for specific languages
4. **Text shaping** — Complex script shaping rules vary by language

**Example: Arabic text shaping varies by language**
- `ar-SA` (Saudi Arabic) — Uses Saudi-specific letter forms
- `ar-EG` (Egyptian Arabic) — Uses Egyptian-specific letter forms
- Shaping engine (HarfBuzz) applies different rules based on language tag

---

## 6. Complex Script Properties

### The w:cs Flag

The `w:cs` flag indicates that a run contains complex script text:

```xml
<w:rPr>
  <w:cs/>  <!-- This run is complex script -->
  <w:rFonts w:cs="Arial"/>
</w:rPr>
```

**When to set:**
- Text contains Arabic, Hebrew, Indic, or Southeast Asian scripts
- Text requires bidirectional (RTL) processing
- Text requires glyph shaping (ligatures, contextual forms)

### The w:rtl Flag

The `w:rtl` flag indicates right-to-left text direction:

```xml
<w:rPr>
  <w:rtl/>  <!-- Right-to-left text -->
  <w:rFonts w:cs="Arial"/>
</w:rPr>
<w:t>مرحبا</w:t>
```

**When to set:**
- Text is in an RTL language (Arabic, Hebrew, Persian, Urdu)
- Text direction is explicitly right-to-left

### Complex Script Font Size (w:szCs)

The `w:szCs` element specifies font size for complex script text (separate from `w:sz`):

```xml
<w:rPr>
  <w:sz w:val="22"/>      <!-- 11pt for Latin -->
  <w:szCs w:val="24"/>    <!-- 12pt for complex scripts -->
</w:rPr>
```

This allows complex script text to be slightly larger for readability.

### Complex Script Bold/Italic (w:bCs, w:iCs)

Separate bold and italic flags for complex scripts:

```xml
<w:rPr>
  <w:b/>      <!-- Bold for Latin -->
  <w:bCs/>    <!-- Bold for complex scripts -->
  <w:i/>      <!-- Italic for Latin -->
  <w:iCs/>    <!-- Italic for complex scripts -->
</w:rPr>
```

### Why Separate Properties?

OOXML has separate properties for complex scripts because:

1. **Different fonts** — Complex script fonts are often different from Latin fonts
2. **Different sizing** — Complex scripts may need different sizes for readability
3. **Different shaping** — Complex scripts require glyph shaping; Latin doesn't
4. **Different rendering** — RTL text requires different layout than LTR

---

## 7. Mapping to Lontar

### Lontar-Aksara Font Management

The `lontar-aksara` text shaping pipeline detects scripts and manages fonts:

```rust
pub struct ShapedRun {
    pub text: String,
    pub script: Script,  // Latin, Arabic, Devanagari, Balinese, CJK, etc.
    pub language: Language,
    pub font: FontRef,
    pub glyphs: Vec<Glyph>,
    pub positions: Vec<Position>,
}

pub enum Script {
    Latin,
    Arabic,
    Hebrew,
    Devanagari,
    Bengali,
    Gujarati,
    Gurmukhi,
    Kannada,
    Malayalam,
    Oriya,
    Tamil,
    Telugu,
    Thai,
    Lao,
    Khmer,
    Myanmar,
    Balinese,
    CJK,
    Hiragana,
    Katakana,
    Hangul,
}
```

### Mapping to w:rFonts

Based on script detection, populate `w:rFonts` correctly:

| Script | Font Slot | Example |
|---|---|---|
| Latin | `w:ascii` | Calibri |
| Latin Extended | `w:hAnsi` | Calibri |
| Arabic | `w:cs` | Arial |
| Hebrew | `w:cs` | Arial |
| Devanagari | `w:cs` | Noto Sans Devanagari |
| Balinese | `w:cs` | Noto Sans Balinese |
| CJK | `w:eastAsia` | SimSun (Chinese), MS Gothic (Japanese) |

### Font Embedding Strategy

**Decision tree:**

```
1. Is the font a system font?
   → No: Embed it (subsetting recommended)
   → Yes: Don't embed (use system fallback)

2. Check fsType in font's OS/2 table:
   → fsType == 0 (Installable): Embed fully
   → fsType == 4 (Preview & Print): Embed with restrictions
   → fsType == 2 (Restricted): Don't embed; use fallback

3. For complex script fonts:
   → Always embed if not a system font
   → Use subsetting to reduce file size
   → Ensure font supports required Unicode ranges
```

**Lontar implementation:**

```rust
pub fn should_embed_font(font: &Font) -> bool {
    // Don't embed system fonts
    if is_system_font(&font.name) {
        return false;
    }
    
    // Check fsType
    match font.os2_table.fs_type {
        0 => true,      // Installable
        4 => true,      // Preview & Print (with restrictions)
        2 => false,     // Restricted
        _ => true,      // Default: embed
    }
}

pub fn subset_font(font: &Font, used_glyphs: &[GlyphId]) -> Vec<u8> {
    // Create a new font with only used glyphs
    // Reduces file size significantly
    // Implementation: use fonttools or similar
}

pub fn obfuscate_font(font_data: &[u8], guid: &Uuid) -> Vec<u8> {
    // XOR first 32 bytes with GUID
    let mut obfuscated = font_data.to_vec();
    let guid_bytes = guid.as_bytes();
    for i in 0..32.min(obfuscated.len()) {
        obfuscated[i] ^= guid_bytes[i % 16];
    }
    obfuscated
}
```

### Language Tagging Strategy

Map script detection to language tags:

```rust
pub fn script_to_language_tag(script: Script, region: &str) -> String {
    match script {
        Script::Latin => format!("en-{}", region),  // Default to English
        Script::Arabic => format!("ar-{}", region),  // ar-SA, ar-EG, etc.
        Script::Hebrew => "he-IL".to_string(),
        Script::Devanagari => "hi-IN".to_string(),
        Script::Bengali => "bn-IN".to_string(),
        Script::Gujarati => "gu-IN".to_string(),
        Script::Gurmukhi => "pa-IN".to_string(),
        Script::Kannada => "kn-IN".to_string(),
        Script::Malayalam => "ml-IN".to_string(),
        Script::Oriya => "or-IN".to_string(),
        Script::Tamil => "ta-IN".to_string(),
        Script::Telugu => "te-IN".to_string(),
        Script::Thai => "th-TH".to_string(),
        Script::Lao => "lo-LA".to_string(),
        Script::Khmer => "km-KH".to_string(),
        Script::Myanmar => "my-MM".to_string(),
        Script::Balinese => "ban".to_string(),  // Balinese (no region)
        Script::CJK => format!("zh-{}", region),  // zh-CN, zh-TW, etc.
        Script::Hiragana | Script::Katakana => "ja-JP".to_string(),
        Script::Hangul => "ko-KR".to_string(),
    }
}
```

### Generating w:rFonts for DOCX

```rust
pub fn generate_rfonts(shaped_runs: &[ShapedRun]) -> XmlElement {
    let mut fonts = HashMap::new();
    
    for run in shaped_runs {
        match run.script {
            Script::Latin | Script::LatinExtended => {
                fonts.insert("ascii", &run.font.name);
                fonts.insert("hAnsi", &run.font.name);
            }
            Script::Arabic | Script::Hebrew | Script::Devanagari | ... => {
                fonts.insert("cs", &run.font.name);
            }
            Script::CJK | Script::Hiragana | Script::Katakana | Script::Hangul => {
                fonts.insert("eastAsia", &run.font.name);
            }
        }
    }
    
    // Generate XML
    let mut rfonts = XmlElement::new("w:rFonts");
    for (slot, font_name) in fonts {
        rfonts.set_attribute(format!("w:{}", slot), font_name);
    }
    rfonts
}
```

### Generating w:lang for DOCX

```rust
pub fn generate_lang(shaped_runs: &[ShapedRun]) -> XmlElement {
    let mut lang_elem = XmlElement::new("w:lang");
    
    // Determine primary language
    let primary_lang = determine_primary_language(shaped_runs);
    lang_elem.set_attribute("w:val", &primary_lang);
    
    // Set East Asian language if present
    if has_script(shaped_runs, Script::CJK) {
        lang_elem.set_attribute("w:eastAsia", "zh-CN");
    }
    
    // Set bidirectional language if present
    if has_script(shaped_runs, Script::Arabic) {
        lang_elem.set_attribute("w:bidi", "ar-SA");
    }
    
    lang_elem
}
```

---

## 8. Font Fallback Strategy

### System Font Fallback

When a font is not available, Word uses a fallback:

```
Requested font: "Noto Sans Balinese"
Not found on system
Fallback: Use w:ascii font (e.g., "Calibri")
Result: Text renders in Calibri (may look incorrect for Balinese)
```

**Lontar strategy:**
1. Embed all non-system fonts (especially complex script fonts)
2. Provide fallback fonts in font table
3. Document which fonts are required for each script

### Recommended Fonts by Script

| Script | Recommended Fonts | Notes |
|---|---|---|
| Latin | Calibri, Arial, Times New Roman | System fonts, no embedding needed |
| Arabic | Arial, Segoe UI, Simplified Arabic | Segoe UI has good Arabic support |
| Hebrew | Arial, Calibri, Segoe UI | Segoe UI recommended |
| Devanagari | Noto Sans Devanagari, Mangal | Noto Sans is comprehensive |
| Bengali | Noto Sans Bengali, Vrinda | Noto Sans recommended |
| Gujarati | Noto Sans Gujarati, Shruti | Noto Sans recommended |
| Gurmukhi | Noto Sans Gurmukhi, Raavi | Noto Sans recommended |
| Kannada | Noto Sans Kannada, Tunga | Noto Sans recommended |
| Malayalam | Noto Sans Malayalam, Kartik | Noto Sans recommended |
| Oriya | Noto Sans Oriya, Kalimati | Noto Sans recommended |
| Tamil | Noto Sans Tamil, Latha | Noto Sans recommended |
| Telugu | Noto Sans Telugu, Gautami | Noto Sans recommended |
| Thai | Tahoma, Cordia New, Segoe UI | Segoe UI recommended |
| Lao | Noto Sans Lao, Saysettha OT | Noto Sans recommended |
| Khmer | Noto Sans Khmer, Khmer OS | Noto Sans recommended |
| Myanmar | Noto Sans Myanmar, Myanmar Text | Noto Sans recommended |
| Balinese | Noto Sans Balinese | Only comprehensive option |
| CJK (Simplified) | SimSun, Microsoft YaHei | SimSun is default |
| CJK (Traditional) | PMingLiU, Microsoft JhengHei | PMingLiU is default |
| Japanese | MS Gothic, Meiryo, Yu Gothic | MS Gothic is default |
| Korean | Gulim, Malgun Gothic, Segoe UI | Gulim is default |

---

## Implementation Checklist for lontar-docx and lontar-pptx

### Phase 1 (MVP)

- [ ] Implement script detection (lontar-aksara)
- [ ] Generate `w:rFonts` with correct font slots (ascii, hAnsi, cs, eastAsia)
- [ ] Generate `w:lang` with correct language tags
- [ ] Create `word/fontTable.xml` with all fonts used
- [ ] Add font table relationship and content type
- [ ] Support Latin, Arabic, CJK scripts
- [ ] Implement basic font fallback

### Phase 2

- [ ] Implement font embedding (non-system fonts)
- [ ] Implement font obfuscation (XOR with GUID)
- [ ] Implement font subsetting (embed only used glyphs)
- [ ] Check fsType before embedding
- [ ] Support all Indic scripts (Devanagari, Bengali, etc.)
- [ ] Support Southeast Asian scripts (Thai, Lao, Khmer, Myanmar)
- [ ] Support Balinese (Aksara Bali)

### Phase 3+

- [ ] Implement w:szCs (complex script font size)
- [ ] Implement w:bCs, w:iCs (complex script bold/italic)
- [ ] Implement theme fonts (PPTX)
- [ ] Implement font fallback chains
- [ ] Optimize font subsetting (remove unused glyphs)
- [ ] Support OpenType features (ligatures, contextual forms)

---

## References

- ECMA-376-1:2016 — Office Open XML File Formats (Part 1)
  - Section 17.8.3 — Run Properties (w:rPr)
  - Section 17.8.3.1 — Font Reference (w:rFonts)
  - Section 17.8.3.2 — Language (w:lang)
  - Section 17.8.3.3 — Complex Script (w:cs)
  - Section 17.8.3.4 — Right-to-Left (w:rtl)
- ECMA-376-2:2016 — Office Open XML File Formats (Part 2)
  - Font Embedding specifications
- Unicode Standard — https://unicode.org/
  - Unicode blocks and script ranges
- BCP 47 — Tags for Identifying Languages — https://tools.ietf.org/html/bcp47
- HarfBuzz — Text shaping engine — https://harfbuzz.github.io/
- Noto Fonts — Google's font family for all scripts — https://fonts.google.com/noto
