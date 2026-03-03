# Text Shaping Expectations

This document defines the expected shaping behavior for each script category in the Multi-Script Test Corpus. These expectations serve as validation criteria for the `lontar-aksara` text shaping pipeline.

## Overview

Text shaping transforms Unicode codepoints into positioned glyphs. Different scripts have different shaping requirements:

1. **Simple scripts**: One-to-one mapping (Latin, Cyrillic)
2. **Complex scripts**: Reordering, conjuncts, contextual forms (Indic, Arabic)
3. **Bidirectional**: RTL and mixed-direction text
4. **Emoji**: Multi-codepoint sequences

## Simple LTR Scripts

### Latin, Cyrillic, Greek, Georgian

**Expected Behavior:**
- One codepoint → one glyph (mostly)
- Optional ligatures (fi, fl) via `liga` feature
- Combining marks position correctly via `mark` feature
- No reordering

**Test Cases:**
```
Input:  "Hello"
Output: [H][e][l][l][o]  (5 glyphs, 5 codepoints)

Input:  "café" (U+0063 U+0061 U+0066 U+00E9)
Output: [c][a][f][é]  (4 glyphs)

Input:  "e̊" (U+0065 U+030A)  # e + combining ring above
Output: [e̊]  (1 glyph with mark positioned above)
```

## Complex Indic Scripts

### Devanagari

**Shaping Rules:**
1. **Conjuncts**: Consonant + virama + consonant → conjunct glyph
2. **Reordering**: Pre-base matras move before consonant cluster
3. **Reph**: Ra + virama at start → reph mark above
4. **Half-forms**: Some consonants have half-forms before virama

**Test Case: क्षत्रिय (kṣatriya)**
```
Input:  क् ष त् र ि य
        [ka][virama][ṣa][ta][virama][ra][i-matra][ya]

Expected Glyphs:
        [kṣa-conjunct][tri-conjunct][ya]
        
Shaping Steps:
1. क् + ष → क्ष (kṣa conjunct)
2. त् + र + ि → त्रि (tri with reordered i-matra)
3. Final: [kṣa][tri][ya]
```

**Test Case: श्रीमद् (śrīmad)**
```
Input:  श् र ी म द्
        [śa][virama][ra][ī-matra][ma][da][virama]

Expected:
1. श् + र → श with reph mark above (र् becomes reph)
2. ी positions after श
3. Final: [śa-with-reph-above][ī][ma][da-half-form]
```

### Bengali

**Shaping Rules:**
- Similar to Devanagari but different conjunct forms
- Vowel signs (matras) attach to consonants
- Ya-phala (য-ফলা) subscript form

**Test Case: আমি (āmi)**
```
Input:  আ ম ি
        [ā][ma][i-kar]

Expected: [ā][ma-with-i-kar-above]
```

### Tamil

**Shaping Rules:**
- **No conjuncts** (unique among Indic scripts)
- Pulli (்) marks dead consonant
- Vowel signs attach to consonants

**Test Case: தமிழ் (tamiḻ)**
```
Input:  த ம ி ழ ்
        [ta][ma][i-kar][ḻa][pulli]

Expected: [ta][ma-with-i-kar][ḻa-with-pulli]
```

## Southeast Asian Scripts

### Balinese (ᬮᭀᬦ᭄ᬢᬭ᭄)

**Critical Test Case: "lontar"**
```
Input:  ᬮ ᭀ ᬦ ᭄ ᬢ ᬭ ᭄
        [la][vowel-o][na][virama][ta][ra][virama]
        U+1B2E U+1B40 U+1B26 U+1B44 U+1B22 U+1B2D U+1B44

Expected Glyphs:
        [la-with-o-above][nta-conjunct][ra-half-form]

Shaping Steps:
1. ᬮ + ᭀ → ᬮᭀ (la with vowel o above)
2. ᬦ᭄ + ᬢ → ᬦ᭄ᬢ (n + virama + t = nt conjunct)
3. ᬭ᭄ → ᬭ᭄ (r + virama = half-form r)
4. Final: [lᭀ][ᬦ᭄ᬢ][ᬭ᭄]
```

**Validation:**
- ✅ Vowel sign ᭀ positions above ᬮ
- ✅ Virama ᭄ triggers conjunct formation
- ✅ ᬦ᭄ᬢ renders as single glyph (not separate n + t)
- ✅ Final ᬭ᭄ shows subscript or half-form

### Thai

**Shaping Rules:**
- Tone marks and vowels stack above/below consonants
- No conjuncts
- Complex positioning rules

**Test Case: ภาษา (phāsā)**
```
Input:  ภ า ษ า
        [pho][sara-a][so][sara-a]

Expected: [pho-with-sara-a][so-with-sara-a]
```

### Myanmar

**Shaping Rules:**
- Medial consonants stack below
- Vowel signs position around consonants
- Tone marks above

**Test Case: မြန်မာ (mranmā)**
```
Input:  မ ြ န ် မ ာ
        [ma][ya-medial][na][asat][ma][aa-vowel]

Expected: [ma-with-ya-below][na-with-asat][ma-with-aa]
```

## RTL Scripts

### Arabic

**Shaping Rules:**
- **Contextual joining**: Each letter has 4 forms (isolated, initial, medial, final)
- Ligatures (lam-alif)
- Diacritics (harakat) position above/below

**Test Case: اللغة (al-lugha)**
```
Input:  ا ل ل غ ة
        [alif][lam][lam][ghayn][tah-marbuta]

Expected Forms:
        [alif-isolated][lam-initial][lam-medial][ghayn-medial][tah-final]

Joining:
        ا + لـ + ـلـ + ـغـ + ـة
        
Ligatures:
        ا + ل → لا (lam-alif ligature)
```

**Test Case with Diacritics: بِسْمِ (bismi)**
```
Input:  ب ِ س ْ م ِ
        [ba][kasra][sin][sukun][mim][kasra]

Expected:
        [ba-initial-with-kasra-below][sin-medial-with-sukun-above][mim-final-with-kasra-below]
```

### Hebrew

**Shaping Rules:**
- RTL but **no joining** (letters don't connect)
- Optional niqqud (vowel points)
- Cantillation marks

**Test Case: שָׁלוֹם (shalom)**
```
Input:  ש ָ ׁ ל ו ֹ ם
        [shin][qamats][shin-dot][lamed][vav][holam][mem-final]

Expected: [shin-with-qamats-and-dot][lamed][vav-with-holam][mem-final]
```

## CJK Scripts

### Chinese, Japanese, Korean

**Shaping Rules:**
- Mostly one-to-one mapping
- Vertical text support (optional)
- Variation selectors for glyph variants

**Test Case: 日本語 (Japanese)**
```
Input:  日 本 語
        [nichi][hon][go]

Expected: [日][本][語]  (3 glyphs, Han characters)
```

**Test Case: Mixed Scripts (Japanese)**
```
Input:  漢字とひらがな
        [kanji][kanji][to-hiragana][hiragana][hiragana][hiragana][hiragana]

Expected: Script runs should be detected:
        Run 1 (Hani): [漢][字]
        Run 2 (Hira): [と][ひ][ら][が][な]
```

## Tibetan

**Shaping Rules:**
- Subjoined consonants stack below
- Vowel signs above/below
- Tsheg (་) word separator

**Test Case: བོད་ (bod)**
```
Input:  བ ོ ད ་
        [ba][vowel-o][da][tsheg]

Expected: [ba-with-o-above][da][tsheg]
```

**Test Case with Stacking: རྒྱ (rgya)**
```
Input:  ར ྒ ྱ
        [ra][ga-subjoined][ya-subjoined]

Expected: [ra-with-ga-and-ya-stacked-below]
```

## Mixed-Script Paragraphs

### Latin + Balinese

**Test Case:**
```
Input:  "The word lontar in Balinese is ᬮᭀᬦ᭄ᬢᬭ᭄ (palm leaf)."

Expected Script Runs:
        Run 1 (Latn): "The word lontar in Balinese is "
        Run 2 (Bali): "ᬮᭀᬦ᭄ᬢᬭ᭄"
        Run 3 (Latn): " (palm leaf)."

Font Selection:
        Run 1: Noto Sans
        Run 2: Noto Sans Balinese
        Run 3: Noto Sans
```

### Arabic + English (Bidirectional)

**Test Case:**
```
Input:  "The Arabic word for book is كتاب (kitab)."

Logical Order:  "The Arabic word for book is " + كتاب + " (kitab)."
Visual Order:   "The Arabic word for book is باتك (kitab)."
                                              ←───

Expected:
        LTR Run: "The Arabic word for book is "
        RTL Run: "باتك" (reversed)
        LTR Run: " (kitab)."
```

## Edge Cases

### Emoji with ZWJ

**Test Case: Family Emoji**
```
Input:  👨‍👩‍👧‍👦
        [man][ZWJ][woman][ZWJ][girl][ZWJ][boy]
        U+1F468 U+200D U+1F469 U+200D U+1F467 U+200D U+1F466

Expected: Single family emoji glyph (if font supports)
Fallback: Four separate emoji glyphs
```

### Skin Tone Modifiers

**Test Case:**
```
Input:  👋🏽
        [waving-hand][medium-skin-tone]
        U+1F44B U+1F3FD

Expected: Single glyph with medium skin tone
```

### Combining Marks

**Test Case:**
```
Input:  e̊ẙẘ
        [e][ring-above][y][ring-above][dot-above][w][ring-above][dot-below]

Expected: Each base letter with marks positioned correctly
```

## Validation Methodology

For each test case:

1. **Parse**: Convert input string to codepoints
2. **Segment**: Detect script runs
3. **Shape**: Apply HarfBuzz with appropriate font
4. **Verify**:
   - Glyph count matches expected
   - Glyph IDs are correct (not .notdef)
   - Positions are reasonable (no overlaps)
   - Visual rendering matches reference

## Reference Implementations

- **HarfBuzz**: Industry-standard shaping engine
- **ICU**: Unicode algorithms (bidi, line breaking)
- **Pango**: Text layout library
- **DirectWrite**: Windows text rendering
- **Core Text**: macOS text rendering

## See Also

- [Unicode Text Segmentation (UAX #29)](https://www.unicode.org/reports/tr29/)
- [Unicode Bidirectional Algorithm (UAX #9)](https://www.unicode.org/reports/tr9/)
- [Unicode Line Breaking Algorithm (UAX #14)](https://www.unicode.org/reports/tr14/)
- [HarfBuzz Documentation](https://harfbuzz.github.io/)
- [Microsoft Script Development Specs](https://docs.microsoft.com/en-us/typography/script-development/standard)
