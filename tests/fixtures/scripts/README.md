# Multi-Script Test Corpus

This directory contains test data for validating text shaping, bidirectional text handling, and multi-script support across all Unicode script categories relevant to Lontar.

## Purpose

The Multi-Script Test Corpus ensures that Lontar correctly handles:
- Complex text shaping (conjuncts, ligatures, reordering)
- Bidirectional text (RTL, LTR, mixed)
- Script run segmentation
- Font fallback chains
- Language-specific typography rules

## Directory Structure

```
scripts/
├── README.md                    # This file
├── test_strings.json            # Comprehensive test strings for all script categories
├── fonts.md                     # Documentation of required test fonts
├── shaping_expectations.md      # Expected shaping behavior for each script
└── (future: reference outputs)
```

## Test String Categories

### Simple LTR Scripts
- **Latin**: Basic alphabet, most common script
- **Cyrillic**: Russian, Ukrainian, etc.
- **Greek**: Ancient and modern Greek
- **Georgian**: Mkhedruli script

### Complex Indic Scripts
Scripts with complex shaping rules (conjuncts, split matras, reordering):
- **Devanagari**: Hindi, Sanskrit (क्ष, त्र conjuncts)
- **Bengali**: Bengali language
- **Tamil**: No conjuncts, uses pulli
- **Telugu**: Rounded characters
- **Kannada**: South Indian script
- **Malayalam**: Complex conjuncts

### Southeast Asian Scripts
Scripts with stacking and complex vowel placement:
- **Balinese** (ᬮᭀᬦ᭄ᬢᬭ᭄): Aksara Bali, the namesake of this project
- **Javanese**: Aksara Jawa
- **Sundanese**: West Java script
- **Batak**: Sumatra script
- **Thai**: Tone marks and vowels
- **Lao**: Similar to Thai
- **Khmer**: Subscript consonants
- **Myanmar**: Burmese script with stacking

### RTL Scripts
Right-to-left scripts with contextual joining:
- **Arabic**: Contextual letter forms (initial, medial, final, isolated)
- **Hebrew**: RTL without joining, optional niqqud
- **Syriac**: Aramaic script with joining
- **Thaana**: Maldivian script
- **N'Ko**: West African RTL script

### CJK Scripts
East Asian ideographic and syllabic scripts:
- **Simplified Chinese**: Mainland China
- **Traditional Chinese**: Taiwan, Hong Kong
- **Japanese**: Mixed Kanji + Hiragana + Katakana
- **Korean**: Hangul syllables

### Other Scripts
- **Tibetan**: Stacking consonants
- **Ethiopic**: Ge'ez syllabary
- **Tifinagh**: Berber script

### Mixed-Script Paragraphs
Real-world scenarios with multiple scripts in one text run:
- Latin + Balinese
- Arabic + English (bidirectional)
- CJK + Latin
- Multiple Indic scripts

### Edge Cases
- Zero-width joiners (emoji families)
- Skin tone modifiers
- Combining diacritical marks
- Variation selectors

## Usage

### In Rust Tests

```rust
use serde_json::Value;
use std::fs;

let test_strings: Value = serde_json::from_str(
    &fs::read_to_string("tests/fixtures/scripts/test_strings.json")?
)?;

let balinese = &test_strings["southeast_asian"]["balinese"]["text"];
// ᬮᭀᬦ᭄ᬢᬭ᭄
```

### Validation Checklist

For each script category, verify:
1. ✅ Text renders without tofu (missing glyph boxes)
2. ✅ Conjuncts form correctly (where applicable)
3. ✅ Vowel marks position correctly
4. ✅ Contextual joining works (Arabic, Syriac)
5. ✅ Bidirectional algorithm handles RTL correctly
6. ✅ Script runs segment properly in mixed text
7. ✅ Font fallback selects appropriate fonts

## Reference Documents

The test corpus will include reference documents generated with:
- `python-docx` (DOCX format)
- `python-pptx` (PPTX format)
- XeLaTeX (PDF format)

These serve as ground truth for visual comparison.

## Shaping Pipeline Integration

This corpus is designed to test the `lontar-aksara` text shaping pipeline:

```
Input Text → Script Detection → Font Selection → HarfBuzz Shaping → Positioned Glyphs
```

Each test string exercises specific features of this pipeline.

## Contributing

When adding new scripts:
1. Add entry to `test_strings.json` with:
   - `text`: Representative sample
   - `language`: BCP 47 language tag
   - `script`: ISO 15924 script code
   - `description`: What the text demonstrates
   - `shaping_notes`: Expected behavior (optional)
2. Document required fonts in `fonts.md`
3. Document expected shaping in `shaping_expectations.md`
4. Generate reference documents for visual verification

## See Also

- [Unicode Script Property](https://www.unicode.org/reports/tr24/)
- [HarfBuzz Shaping](https://harfbuzz.github.io/)
- [Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/)
- [Noto Fonts](https://fonts.google.com/noto) - Comprehensive Unicode coverage
