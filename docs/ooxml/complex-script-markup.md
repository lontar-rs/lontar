# Complex Script Markup in OOXML (DOCX and PPTX)

## Overview

Complex scripts in OOXML require special handling beyond simple left-to-right (LTR) Latin text. This document covers bidirectional text, right-to-left (RTL) languages, Indic and Southeast Asian scripts, and mixed-script paragraphs.

This is a foundational reference for integrating lontar-aksara's script detection with OOXML backends (lontar-docx and lontar-pptx).

---

## 1. What OOXML Considers "Complex Scripts"

### Unicode Ranges for Complex Scripts

OOXML classifies the following Unicode ranges as complex scripts (CS):

| Script | Unicode Range | Characteristics | Notes |
|---|---|---|---|
| Arabic | U+0600–U+06FF | RTL, contextual forms, ligatures | Includes Arabic Supplement (U+0750–U+077F) |
| Hebrew | U+0590–U+05FF | RTL, diacritics above/below | Includes Hebrew Supplement (U+FB1D–U+FB4F) |
| Syriac | U+0700–U+074F | RTL, contextual forms | Ancient Aramaic script |
| Thaana | U+0780–U+07BF | RTL, contextual forms | Maldivian script |
| Devanagari | U+0900–U+097F | LTR, conjuncts, diacritics | Hindi, Sanskrit, Marathi |
| Bengali | U+0980–U+09FF | LTR, conjuncts, diacritics | Bengali, Assamese |
| Gurmukhi | U+0A00–U+0A7F | LTR, conjuncts, diacritics | Punjabi |
| Gujarati | U+0A80–U+0AFF | LTR, conjuncts, diacritics | Gujarati |
| Oriya | U+0B00–U+0B7F | LTR, conjuncts, diacritics | Odia |
| Tamil | U+0B80–U+0BFF | LTR, diacritics, no conjuncts | Tamil |
| Telugu | U+0C00–U+0C7F | LTR, conjuncts, diacritics | Telugu |
| Kannada | U+0C80–U+0CFF | LTR, conjuncts, diacritics | Kannada |
| Malayalam | U+0D00–U+0D7F | LTR, conjuncts, diacritics | Malayalam |
| Sinhala | U+0D80–U+0DFF | LTR, conjuncts, diacritics | Sinhalese |
| Thai | U+0E00–U+0E7F | LTR, no spaces, tone marks | Thai |
| Lao | U+0E80–U+0EFF | LTR, no spaces, tone marks | Lao |
| Tibetan | U+0F00–U+0FFF | LTR, stacked consonants | Tibetan |
| Myanmar | U+1000–U+109F | LTR, conjuncts, diacritics | Burmese |
| Khmer | U+1780–U+17FF | LTR, conjuncts, diacritics | Cambodian |
| Balinese | U+1B00–U+1B7F | LTR, conjuncts, vowel reordering | Aksara Bali |
| Javanese | U+1B80–U+1BBF | LTR, conjuncts, vowel reordering | Javanese |
| Sundanese | U+1B80–U+1BBF | LTR, conjuncts, vowel reordering | Sundanese |

### Script Classification in OOXML

OOXML divides scripts into two categories:

1. **Latin Scripts** — ASCII (U+0000–U+007F), High-ANSI (U+0080–U+00FF), Latin Extended
   - Stored in `w:ascii` or `w:hAnsi` font slots
   - Use standard run properties (`w:b`, `w:i`, `w:sz`)

2. **Complex Scripts** — All others (Arabic, Indic, Southeast Asian, etc.)
   - Stored in `w:cs` (complex script) or `w:eastAsia` (CJK) font slots
   - Use CS-specific run properties (`w:bCs`, `w:iCs`, `w:szCs`)
   - May require shaping engine (HarfBuzz) for correct rendering
   - May be RTL (Arabic, Hebrew) or LTR (Indic, Southeast Asian)

### Impact on Run Properties

When OOXML detects complex script text, it applies different property handling:

```xml
<!-- Latin text: use standard properties -->
<w:r>
  <w:rPr>
    <w:rFonts w:ascii="Calibri"/>
    <w:b/>        <!-- Standard bold -->
    <w:sz w:val="22"/>  <!-- Standard size -->
  </w:rPr>
  <w:t>Hello</w:t>
</w:r>

<!-- Complex script text: use CS-specific properties -->
<w:r>
  <w:rPr>
    <w:rFonts w:cs="Arial"/>
    <w:bCs/>      <!-- Complex script bold -->
    <w:szCs w:val="24"/>  <!-- Complex script size -->
    <w:cs/>       <!-- Mark as complex script -->
  </w:rPr>
  <w:t>مرحبا</w:t>
</w:r>
```

---

## 2. Bidirectional Text (BiDi) in DOCX

### Paragraph-Level BiDi (w:bidi)

The `w:bidi` attribute on paragraph properties marks the paragraph as RTL:

```xml
<w:p>
  <w:pPr>
    <w:bidi/>  <!-- Paragraph is RTL -->
  </w:pPr>
  <w:r>
    <w:rPr>
      <w:rtl/>
    </w:rPr>
    <w:t>مرحبا</w:t>
  </w:r>
</w:p>
```

**When to set:**
- Paragraph contains RTL text (Arabic, Hebrew, Persian, Urdu)
- Paragraph direction is explicitly right-to-left

### Run-Level BiDi (w:rtl)

The `w:rtl` flag on run properties marks individual runs as RTL:

```xml
<w:r>
  <w:rPr>
    <w:rtl/>  <!-- This run is RTL -->
  </w:rPr>
  <w:t>مرحبا</w:t>
</w:r>
```

**When to set:**
- Run contains RTL text
- Run direction differs from paragraph direction

### Mixed LTR+RTL Text

#### Example 1: Arabic Sentence with English Word

```xml
<w:p>
  <w:pPr>
    <w:bidi/>  <!-- Paragraph is RTL -->
  </w:pPr>

  <!-- Arabic text -->
  <w:r>
    <w:rPr>
      <w:rtl/>
      <w:rFonts w:cs="Arial"/>
    </w:rPr>
    <w:t>مرحبا </w:t>
  </w:r>

  <!-- English word (LTR) -->
  <w:r>
    <w:rPr>
      <w:rFonts w:ascii="Calibri"/>
    </w:rPr>
    <w:t>hello</w:t>
  </w:r>

  <!-- More Arabic -->
  <w:r>
    <w:rPr>
      <w:rtl/>
      <w:rFonts w:cs="Arial"/>
    </w:rPr>
    <w:t> عالم</w:t>
  </w:r>
</w:p>
```

**Visual rendering (RTL paragraph):**
```
عالم hello مرحبا
```

#### Example 2: English Sentence with Arabic Phrase

```xml
<w:p>
  <!-- No w:bidi: paragraph is LTR -->

  <!-- English text -->
  <w:r>
    <w:rPr>
      <w:rFonts w:ascii="Calibri"/>
    </w:rPr>
    <w:t>Hello </w:t>
  </w:r>

  <!-- Arabic phrase (RTL) -->
  <w:r>
    <w:rPr>
      <w:rtl/>
      <w:rFonts w:cs="Arial"/>
    </w:rPr>
    <w:t>مرحبا</w:t>
  </w:r>

  <!-- More English -->
  <w:r>
    <w:rPr>
      <w:rFonts w:ascii="Calibri"/>
    </w:rPr>
    <w:t> world</w:t>
  </w:r>
</w:p>
```

**Visual rendering (LTR paragraph):**
```
Hello مرحبا world
```

### Unicode Bidirectional Algorithm (UBA)

The Unicode Bidirectional Algorithm (UBA) determines text direction:

1. **Paragraph direction** — Set by `w:bidi` on paragraph
2. **Run direction** — Set by `w:rtl` on run
3. **Character properties** — Unicode assigns each character a directionality (L, R, AL, etc.)

**UBA rules:**
- Explicit formatting (w:bidi, w:rtl) overrides character properties
- Neutral characters (spaces, punctuation) inherit direction from surrounding text
- Isolate characters (U+2066–U+2069) create explicit directional boundaries

**Example: How Word applies UBA**
```
Input: "Hello مرحبا world"
Paragraph: LTR (no w:bidi)

Character analysis:
H e l l o   = LTR
م ر ح ب ا   = RTL
w o r l d   = LTR

Rendering:
Hello [RTL text] world
```

### w:bidiVisual (Visual vs Logical Ordering)

The `w:bidiVisual` element specifies whether text is stored in visual or logical order:

```xml
<w:p>
  <w:pPr>
    <w:bidi/>
    <w:bidiVisual/>  <!-- Text is in visual order -->
  </w:pPr>
  <!-- ... -->
</w:p>
```

- **Logical order** (default) — Text stored as typed (RTL text stored right-to-left)
- **Visual order** — Text stored as displayed (RTL text stored left-to-right)

**Lontar recommendation:** Use logical order (don't set `w:bidiVisual`). Modern Office applications expect logical order.

---

## 3. Complex Script Run Properties

### w:cs — Complex Script Flag

Marks a run as containing complex script text:

```xml
<w:rPr>
  <w:cs/>  <!-- This run is complex script -->
</w:rPr>
```

**When to set:**
- Run contains Arabic, Hebrew, Indic, Southeast Asian, or other complex script
- Run requires glyph shaping (ligatures, contextual forms, conjuncts)

### w:rFonts w:cs — Complex Script Font

Specifies the font for complex script characters:

```xml
<w:rPr>
  <w:rFonts w:cs="Arial"/>  <!-- Use Arial for complex scripts -->
</w:rPr>
```

**Font selection:**
- Arabic: Arial, Segoe UI, Simplified Arabic
- Indic: Noto Sans [Script] (Devanagari, Bengali, Tamil, etc.)
- Southeast Asian: Noto Sans [Script] (Thai, Khmer, Myanmar, Balinese, etc.)

### w:szCs — Complex Script Font Size

Specifies font size for complex script text (separate from `w:sz`):

```xml
<w:rPr>
  <w:sz w:val="22"/>      <!-- 11pt for Latin -->
  <w:szCs w:val="24"/>    <!-- 12pt for complex scripts -->
</w:rPr>
```

**Rationale:** Complex script fonts often need larger size for readability.

### w:bCs — Complex Script Bold

Marks complex script text as bold (separate from `w:b`):

```xml
<w:rPr>
  <w:b/>      <!-- Bold for Latin -->
  <w:bCs/>    <!-- Bold for complex scripts -->
</w:rPr>
```

### w:iCs — Complex Script Italic

Marks complex script text as italic (separate from `w:i`):

```xml
<w:rPr>
  <w:i/>      <!-- Italic for Latin -->
  <w:iCs/>    <!-- Italic for complex scripts -->
</w:rPr>
```

### w:rtl — Right-to-Left Flag

Marks run as right-to-left:

```xml
<w:rPr>
  <w:rtl/>  <!-- This run is RTL -->
</w:rPr>
```

**When to set:**
- Run contains Arabic, Hebrew, or other RTL script
- Run direction differs from paragraph direction

### w:lang w:bidi — BiDi Language Tag

Specifies the language for RTL text:

```xml
<w:rPr>
  <w:lang w:val="en-US" w:bidi="ar-SA"/>  <!-- BiDi language is Arabic (Saudi) -->
</w:rPr>
```

**Common BiDi language tags:**
- `ar-SA` — Arabic (Saudi Arabia)
- `ar-EG` — Arabic (Egypt)
- `he-IL` — Hebrew (Israel)
- `fa-IR` — Persian (Iran)
- `ur-PK` — Urdu (Pakistan)

### Why Separate Properties?

OOXML has separate properties for complex scripts because:

1. **Different fonts** — Complex script fonts differ from Latin fonts
2. **Different metrics** — Complex script fonts have different x-height, ascender height
3. **Different sizing** — Complex scripts often need larger size for readability
4. **Different shaping** — Complex scripts require glyph shaping; Latin doesn't
5. **Different direction** — RTL scripts require different layout

---

## 4. Mixed-Script Paragraphs

### Paragraph with Latin + Arabic + CJK

```xml
<w:p>
  <w:pPr>
    <!-- Paragraph is LTR (majority direction) -->
  </w:pPr>

  <!-- Latin text -->
  <w:r>
    <w:rPr>
      <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/>
    </w:rPr>
    <w:t>Hello </w:t>
  </w:r>

  <!-- Arabic text (RTL) -->
  <w:r>
    <w:rPr>
      <w:rtl/>
      <w:rFonts w:cs="Arial"/>
      <w:lang w:val="en-US" w:bidi="ar-SA"/>
    </w:rPr>
    <w:t>مرحبا</w:t>
  </w:r>

  <!-- CJK text -->
  <w:r>
    <w:rPr>
      <w:rFonts w:eastAsia="SimSun"/>
    </w:rPr>
    <w:t> 你好</w:t>
  </w:r>
</w:p>
```

### Run Splitting Rules

Word creates separate `w:r` (run) elements when:

1. **Script changes** — Latin → Arabic → CJK
2. **Direction changes** — LTR → RTL → LTR
3. **Font changes** — Calibri → Arial → SimSun
4. **Style changes** — Regular → Bold → Italic
5. **Language changes** — en-US → ar-SA → zh-CN

**Lontar strategy:** Split runs at script boundaries (detected by lontar-aksara).

### Font Selection in Mixed-Script Paragraph

For a paragraph containing all four font slots:

```xml
<w:rPr>
  <w:rFonts w:ascii="Calibri"
            w:hAnsi="Calibri"
            w:cs="Arial"
            w:eastAsia="SimSun"/>
</w:rPr>
```

Word selects the appropriate font based on character:
- ASCII (U+0000–U+007F) → `w:ascii` (Calibri)
- High-ANSI (U+0080–U+00FF) → `w:hAnsi` (Calibri)
- Complex script (Arabic, etc.) → `w:cs` (Arial)
- CJK → `w:eastAsia` (SimSun)

---

## 5. Southeast Asian Scripts in OOXML

### Thai Script

**Characteristics:**
- LTR, no word spaces
- Tone marks above/below base character
- Vowels can appear above, below, or after consonant

**OOXML handling:**
```xml
<w:r>
  <w:rPr>
    <w:rFonts w:cs="Tahoma"/>  <!-- Thai uses w:cs slot -->
    <w:lang w:val="en-US" w:bidi="th-TH"/>
  </w:rPr>
  <w:t>สวัสดี</w:t>  <!-- Thai text -->
</w:r>
```

**Known limitations:**
- Word may not correctly break lines at word boundaries (no spaces)
- Some tone mark positioning may be incorrect

### Balinese (Aksara Bali)

**Characteristics:**
- LTR, complex conjuncts
- Vowel reordering (vowels appear before consonant in Unicode but after in display)
- Requires HarfBuzz for correct shaping

**OOXML handling:**
```xml
<w:r>
  <w:rPr>
    <w:rFonts w:cs="Noto Sans Balinese"/>  <!-- Balinese uses w:cs slot -->
    <w:lang w:val="en-US" w:bidi="ban"/>   <!-- ban = Balinese -->
    <w:cs/>  <!-- Mark as complex script -->
  </w:rPr>
  <w:t>ᬮᭀᬦ᭄ᬢᬭ᭄</w:t>  <!-- Balinese text -->
</w:r>
```

**Unicode codepoints stored (logical order):**
```
ᬮ (U+1B2E) = LA
ᭀ (U+1B40) = VOWEL A
ᬦ (U+1B06) = NA
᭄ (U+1B44) = ADEG (virama)
ᬢ (U+1B22) = TA
ᬭ (U+1B2D) = RA
᭄ (U+1B44) = ADEG (virama)
```

**Rendering (visual order, after shaping):**
```
ᬮᭀ ᬦ᭄ᬢ ᬭ᭄
```

**Known limitations:**
- Office applications may not render Balinese correctly without Noto Sans Balinese
- Vowel reordering may be incorrect in some cases
- Conjunct formation depends on font support

### Javanese and Sundanese

Similar to Balinese:
```xml
<w:r>
  <w:rPr>
    <w:rFonts w:cs="Noto Sans Javanese"/>
    <w:lang w:val="en-US" w:bidi="jv"/>  <!-- jv = Javanese -->
    <w:cs/>
  </w:rPr>
  <w:t>ꦏꦺꦴꦩ</w:t>  <!-- Javanese text -->
</w:r>
```

### Khmer

**Characteristics:**
- LTR, complex conjuncts
- Diacritics above/below base character
- Requires HarfBuzz for correct shaping

**OOXML handling:**
```xml
<w:r>
  <w:rPr>
    <w:rFonts w:cs="Noto Sans Khmer"/>
    <w:lang w:val="en-US" w:bidi="km"/>  <!-- km = Khmer -->
    <w:cs/>
  </w:rPr>
  <w:t>ក្ដ</w:t>  <!-- Khmer text -->
</w:r>
```

### Myanmar

**Characteristics:**
- LTR, complex conjuncts
- Medial consonants and vowels
- Requires HarfBuzz for correct shaping

**OOXML handling:**
```xml
<w:r>
  <w:rPr>
    <w:rFonts w:cs="Noto Sans Myanmar"/>
    <w:lang w:val="en-US" w:bidi="my"/>  <!-- my = Myanmar -->
    <w:cs/>
  </w:rPr>
  <w:t>မြန်မာ</w:t>  <!-- Myanmar text -->
</w:r>
```

---

## 6. Indic Script Handling

### General Characteristics

All Indic scripts share common features:
- LTR direction
- Conjuncts (consonant clusters)
- Diacritics (vowel marks) above/below base
- Require HarfBuzz for correct shaping

### Devanagari Example

```xml
<w:r>
  <w:rPr>
    <w:rFonts w:cs="Noto Sans Devanagari"/>
    <w:lang w:val="en-US" w:bidi="hi-IN"/>  <!-- hi-IN = Hindi -->
    <w:cs/>
  </w:rPr>
  <w:t>नमस्ते</w:t>  <!-- Devanagari text -->
</w:r>
```

**Unicode codepoints (logical order):**
```
न (U+0928) = NA
म (U+0915) = MA
स (U+0938) = SA
् (U+094D) = VIRAMA (conjunct marker)
त (U+0924) = TA
े (U+0947) = VOWEL E
```

**Rendering (visual order, after shaping):**
```
नमस्ते
```

The virama (U+094D) indicates a conjunct, so the shaping engine combines स् into a single glyph.

### Other Indic Scripts

| Script | Language | Font | Language Tag |
|---|---|---|---|
| Bengali | Bengali, Assamese | Noto Sans Bengali | bn-IN, as-IN |
| Gurmukhi | Punjabi | Noto Sans Gurmukhi | pa-IN |
| Gujarati | Gujarati | Noto Sans Gujarati | gu-IN |
| Oriya | Odia | Noto Sans Oriya | or-IN |
| Tamil | Tamil | Noto Sans Tamil | ta-IN |
| Telugu | Telugu | Noto Sans Telugu | te-IN |
| Kannada | Kannada | Noto Sans Kannada | kn-IN |
| Malayalam | Malayalam | Noto Sans Malayalam | ml-IN |
| Sinhala | Sinhalese | Noto Sans Sinhala | si-LK |

### OOXML Storage

**Important:** OOXML stores Unicode codepoints, not pre-shaped glyphs.

The rendering engine (Word, LibreOffice) applies shaping when displaying the document. This means:
- Conjuncts are stored as separate codepoints with virama
- Diacritics are stored as separate codepoints
- Vowel reordering is handled by the rendering engine

**Example: Devanagari conjunct**
```
Stored in XML: न्म (NA + VIRAMA + MA)
Rendered as: ण्म (single glyph for NA-VIRAMA-MA)
```

---

## 7. Bidirectional Text in PPTX

### DrawingML Run Properties (a:rPr)

In PPTX, RTL text is marked with the `rtl` attribute:

```xml
<a:r>
  <a:rPr lang="ar-SA" rtl="1">
    <a:latin typeface="Calibri"/>
    <a:cs typeface="Arial"/>
  </a:rPr>
  <a:t>مرحبا</a:t>
</a:r>
```

**Attributes:**
- `lang="ar-SA"` — Language tag (BCP 47)
- `rtl="1"` — Right-to-left flag (1 = RTL, 0 or absent = LTR)

### DrawingML Paragraph Properties (a:pPr)

Paragraph-level RTL:

```xml
<a:p>
  <a:pPr algn="rgt" rtl="1">  <!-- Right-aligned, RTL -->
  </a:pPr>
  <a:r>
    <a:rPr lang="ar-SA" rtl="1">
      <a:cs typeface="Arial"/>
    </a:rPr>
    <a:t>مرحبا</a:t>
  </a:r>
</a:p>
```

**Attributes:**
- `algn="rgt"` — Right alignment (rgt, ctr, lft)
- `rtl="1"` — Right-to-left flag

### Mixed-Direction Text in PPTX

```xml
<a:p>
  <a:pPr algn="lft">  <!-- LTR paragraph -->
  </a:pPr>

  <!-- English text -->
  <a:r>
    <a:rPr lang="en-US">
      <a:latin typeface="Calibri"/>
    </a:rPr>
    <a:t>Hello </a:t>
  </a:r>

  <!-- Arabic text (RTL) -->
  <a:r>
    <a:rPr lang="ar-SA" rtl="1">
      <a:cs typeface="Arial"/>
    </a:rPr>
    <a:t>مرحبا</a:t>
  </a:r>

  <!-- More English -->
  <a:r>
    <a:rPr lang="en-US">
      <a:latin typeface="Calibri"/>
    </a:rPr>
    <a:t> world</a:t>
  </a:r>
</a:p>
```

---

## 8. Mapping to Lontar

### Decision Table: Unicode Range → OOXML Properties

| Unicode Range | Script | w:rFonts Slot | w:cs Flag | w:rtl Flag | w:lang | Font Example |
|---|---|---|---|---|---|---|
| U+0000–U+007F | ASCII | w:ascii | ❌ | ❌ | en-US | Calibri |
| U+0080–U+00FF | Latin Extended | w:hAnsi | ❌ | ❌ | en-US | Calibri |
| U+0100–U+017F | Latin Extended-A | w:hAnsi | ❌ | ❌ | en-US | Calibri |
| U+0600–U+06FF | Arabic | w:cs | ✅ | ✅ | ar-SA | Arial |
| U+0590–U+05FF | Hebrew | w:cs | ✅ | ✅ | he-IL | Arial |
| U+0900–U+097F | Devanagari | w:cs | ✅ | ❌ | hi-IN | Noto Sans Devanagari |
| U+0980–U+09FF | Bengali | w:cs | ✅ | ❌ | bn-IN | Noto Sans Bengali |
| U+0B80–U+0BFF | Tamil | w:cs | ✅ | ❌ | ta-IN | Noto Sans Tamil |
| U+0C00–U+0C7F | Telugu | w:cs | ✅ | ❌ | te-IN | Noto Sans Telugu |
| U+0C80–U+0CFF | Kannada | w:cs | ✅ | ❌ | kn-IN | Noto Sans Kannada |
| U+0D00–U+0D7F | Malayalam | w:cs | ✅ | ❌ | ml-IN | Noto Sans Malayalam |
| U+0E00–U+0E7F | Thai | w:cs | ✅ | ❌ | th-TH | Noto Sans Thai |
| U+0E80–U+0EFF | Lao | w:cs | ✅ | ❌ | lo-LA | Noto Sans Lao |
| U+1000–U+109F | Myanmar | w:cs | ✅ | ❌ | my-MM | Noto Sans Myanmar |
| U+1780–U+17FF | Khmer | w:cs | ✅ | ❌ | km-KH | Noto Sans Khmer |
| U+1B00–U+1B7F | Balinese | w:cs | ✅ | ❌ | ban | Noto Sans Balinese |
| U+1B80–U+1BBF | Javanese | w:cs | ✅ | ❌ | jv | Noto Sans Javanese |
| U+4E00–U+9FFF | CJK | w:eastAsia | ❌ | ❌ | zh-CN | SimSun |
| U+3040–U+309F | Hiragana | w:eastAsia | ❌ | ❌ | ja-JP | MS Gothic |
| U+30A0–U+30FF | Katakana | w:eastAsia | ❌ | ❌ | ja-JP | MS Gothic |
| U+AC00–U+D7AF | Hangul | w:eastAsia | ❌ | ❌ | ko-KR | Gulim |

### Lontar-Aksara Integration

**lontar-aksara output:**
```rust
pub struct ShapedRun {
    pub text: String,
    pub script: Script,
    pub language: Language,
    pub font: FontRef,
    pub glyphs: Vec<Glyph>,
}
```

**Mapping to OOXML:**

```rust
pub fn shaped_run_to_ooxml_run(run: &ShapedRun) -> XmlElement {
    let mut rpr = XmlElement::new("w:rPr");

    // Determine font slot and properties based on script
    let (font_slot, is_cs, is_rtl) = match run.script {
        Script::Latin => ("w:ascii", false, false),
        Script::LatinExtended => ("w:hAnsi", false, false),
        Script::Arabic => ("w:cs", true, true),
        Script::Hebrew => ("w:cs", true, true),
        Script::Devanagari | Script::Bengali | Script::Tamil | ... => ("w:cs", true, false),
        Script::Thai | Script::Lao | Script::Khmer | Script::Myanmar | Script::Balinese => ("w:cs", true, false),
        Script::CJK | Script::Hiragana | Script::Katakana | Script::Hangul => ("w:eastAsia", false, false),
    };

    // Set font
    let mut rfonts = XmlElement::new("w:rFonts");
    rfonts.set_attribute(font_slot, &run.font.name);
    rpr.append_child(rfonts);

    // Set CS flag if needed
    if is_cs {
        rpr.append_child(XmlElement::new("w:cs"));
    }

    // Set RTL flag if needed
    if is_rtl {
        rpr.append_child(XmlElement::new("w:rtl"));
    }

    // Set language
    let lang_elem = generate_lang_element(&run.language);
    rpr.append_child(lang_elem);

    // Create run element
    let mut r = XmlElement::new("w:r");
    r.append_child(rpr);

    let mut t = XmlElement::new("w:t");
    t.set_text(&run.text);
    r.append_child(t);

    r
}
```

### Run Splitting Strategy

lontar-aksara detects script runs. Each run becomes a separate `w:r` element:

```rust
pub fn split_by_script(text: &str) -> Vec<ScriptRun> {
    // Detect script boundaries
    // Return vector of (text, script) tuples
}

pub fn generate_paragraph(text: &str) -> XmlElement {
    let script_runs = split_by_script(text);

    let mut p = XmlElement::new("w:p");
    let mut ppr = XmlElement::new("w:pPr");

    // Determine paragraph direction
    let para_direction = determine_paragraph_direction(&script_runs);
    if para_direction == Direction::RTL {
        ppr.append_child(XmlElement::new("w:bidi"));
    }
    p.append_child(ppr);

    // Generate runs
    for script_run in script_runs {
        let run = shaped_run_to_ooxml_run(&script_run);
        p.append_child(run);
    }

    p
}
```

### Paragraph Direction Detection

**Strategy:**
1. Count characters in each direction (LTR vs RTL)
2. Set paragraph direction to majority direction
3. Mark individual runs with opposite direction

```rust
pub fn determine_paragraph_direction(runs: &[ScriptRun]) -> Direction {
    let mut ltr_count = 0;
    let mut rtl_count = 0;

    for run in runs {
        match run.script {
            Script::Arabic | Script::Hebrew | ... => rtl_count += run.text.len(),
            _ => ltr_count += run.text.len(),
        }
    }

    if rtl_count > ltr_count {
        Direction::RTL
    } else {
        Direction::LTR
    }
}
```

---

## Implementation Checklist for lontar-docx and lontar-pptx

### Phase 1 (MVP)

- [ ] Implement script detection (lontar-aksara)
- [ ] Generate w:cs flag for complex scripts
- [ ] Generate w:rFonts with correct font slots
- [ ] Generate w:lang with correct language tags
- [ ] Support Latin, Arabic, CJK scripts
- [ ] Generate w:rtl for RTL scripts
- [ ] Support basic mixed-script paragraphs

### Phase 2

- [ ] Support all Indic scripts (Devanagari, Bengali, Tamil, Telugu, Kannada, Malayalam)
- [ ] Support Southeast Asian scripts (Thai, Lao, Khmer, Myanmar)
- [ ] Support Balinese (Aksara Bali)
- [ ] Implement paragraph direction detection
- [ ] Support mixed LTR+RTL paragraphs
- [ ] Generate w:szCs for complex script font sizing

### Phase 3+

- [ ] Implement w:bidiVisual for visual ordering
- [ ] Support Javanese, Sundanese, Syriac, Thaana
- [ ] Implement w:bCs, w:iCs (complex script bold/italic)
- [ ] Optimize run splitting (minimize number of runs)
- [ ] Support PPTX complex script markup

---

## References

- ECMA-376-1:2016 — Office Open XML File Formats (Part 1)
  - Section 17.3.1 — Bidirectional Text
  - Section 17.8.3 — Run Properties
  - Section 17.8.3.3 — Complex Script
  - Section 17.8.3.4 — Right-to-Left
- Unicode Standard — https://unicode.org/
  - Unicode Bidirectional Algorithm (UBA) — https://unicode.org/reports/tr9/
  - Unicode blocks and script ranges
- BCP 47 — Tags for Identifying Languages — https://tools.ietf.org/html/bcp47
- HarfBuzz — Text shaping engine — https://harfbuzz.github.io/
- Noto Fonts — Google's font family for all scripts — https://fonts.google.com/noto
