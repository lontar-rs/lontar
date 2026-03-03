# Test Fonts for Multi-Script Corpus

This document lists the required fonts for testing all script categories in the Lontar Multi-Script Test Corpus.

## Font Strategy

We use the **Noto font family** from Google Fonts as our primary test font collection because:
- Comprehensive Unicode coverage across all scripts
- Open source (SIL Open Font License)
- Professionally designed with proper OpenType features
- Consistent design language across scripts
- Actively maintained

## Required Noto Fonts

### Simple LTR Scripts
| Script | Font | Coverage |
|--------|------|----------|
| Latin | Noto Sans | Basic Latin, Latin Extended |
| Cyrillic | Noto Sans | Cyrillic, Cyrillic Extended |
| Greek | Noto Sans | Greek, Greek Extended |
| Georgian | Noto Sans Georgian | Georgian, Georgian Extended |

### Complex Indic Scripts
| Script | Font | Coverage | Notes |
|--------|------|----------|-------|
| Devanagari | Noto Sans Devanagari | U+0900–U+097F | Conjuncts, split matras |
| Bengali | Noto Sans Bengali | U+0980–U+09FF | Vowel signs, conjuncts |
| Tamil | Noto Sans Tamil | U+0B80–U+0BFF | No conjuncts, uses pulli |
| Telugu | Noto Sans Telugu | U+0C00–U+0C7F | Rounded characters |
| Kannada | Noto Sans Kannada | U+0C80–U+0CFF | Conjuncts |
| Malayalam | Noto Sans Malayalam | U+0D00–U+0D7F | Complex conjuncts |

### Southeast Asian Scripts
| Script | Font | Coverage | Notes |
|--------|------|----------|-------|
| Balinese | Noto Sans Balinese | U+1B00–U+1B7F | **Critical for Lontar** |
| Javanese | Noto Sans Javanese | U+A980–U+A9DF | Aksara Jawa |
| Sundanese | Noto Sans Sundanese | U+1B80–U+1BBF | West Java script |
| Batak | Noto Sans Batak | U+1BC0–U+1BFF | Sumatra script |
| Thai | Noto Sans Thai | U+0E00–U+0E7F | Tone marks, vowels |
| Lao | Noto Sans Lao | U+0E80–U+0EFF | Similar to Thai |
| Khmer | Noto Sans Khmer | U+1780–U+17FF | Subscript consonants |
| Myanmar | Noto Sans Myanmar | U+1000–U+109F | Burmese stacking |

### RTL Scripts
| Script | Font | Coverage | Notes |
|--------|------|----------|-------|
| Arabic | Noto Sans Arabic | U+0600–U+06FF | Contextual joining |
| Hebrew | Noto Sans Hebrew | U+0590–U+05FF | RTL, optional niqqud |
| Syriac | Noto Sans Syriac | U+0700–U+074F | Aramaic, joining |
| Thaana | Noto Sans Thaana | U+0780–U+07BF | Maldivian |
| N'Ko | Noto Sans NKo | U+07C0–U+07FF | West African RTL |

### CJK Scripts
| Script | Font | Coverage | Notes |
|--------|------|----------|-------|
| Simplified Chinese | Noto Sans SC | CJK Unified Ideographs | Mainland China |
| Traditional Chinese | Noto Sans TC | CJK Unified Ideographs | Taiwan, Hong Kong |
| Japanese | Noto Sans JP | Kanji + Kana | Mixed scripts |
| Korean | Noto Sans KR | Hangul Syllables | Korean |

### Other Scripts
| Script | Font | Coverage | Notes |
|--------|------|----------|-------|
| Tibetan | Noto Sans Tibetan | U+0F00–U+0FFF | Stacking consonants |
| Ethiopic | Noto Sans Ethiopic | U+1200–U+137F | Ge'ez syllabary |
| Tifinagh | Noto Sans Tifinagh | U+2D30–U+2D7F | Berber |

### Emoji & Symbols
| Category | Font | Coverage |
|----------|------|----------|
| Emoji | Noto Color Emoji | Emoji sequences, ZWJ, modifiers |

## Font Installation

### Download from Google Fonts

```bash
# Create fonts directory
mkdir -p tests/fixtures/scripts/fonts

# Download script (example for key fonts)
cd tests/fixtures/scripts/fonts

# Simple LTR
wget https://github.com/notofonts/noto-fonts/raw/main/hinted/ttf/NotoSans/NotoSans-Regular.ttf

# Indic
wget https://github.com/notofonts/devanagari/raw/main/fonts/NotoSansDevanagari/hinted/ttf/NotoSansDevanagari-Regular.ttf
wget https://github.com/notofonts/bengali/raw/main/fonts/NotoSansBengali/hinted/ttf/NotoSansBengali-Regular.ttf
wget https://github.com/notofonts/tamil/raw/main/fonts/NotoSansTamil/hinted/ttf/NotoSansTamil-Regular.ttf
wget https://github.com/notofonts/telugu/raw/main/fonts/NotoSansTelugu/hinted/ttf/NotoSansTelugu-Regular.ttf
wget https://github.com/notofonts/kannada/raw/main/fonts/NotoSansKannada/hinted/ttf/NotoSansKannada-Regular.ttf
wget https://github.com/notofonts/malayalam/raw/main/fonts/NotoSansMalayalam/hinted/ttf/NotoSansMalayalam-Regular.ttf

# Southeast Asian (CRITICAL)
wget https://github.com/notofonts/balinese/raw/main/fonts/NotoSansBalinese/hinted/ttf/NotoSansBalinese-Regular.ttf
wget https://github.com/notofonts/javanese/raw/main/fonts/NotoSansJavanese/hinted/ttf/NotoSansJavanese-Regular.ttf
wget https://github.com/notofonts/sundanese/raw/main/fonts/NotoSansSundanese/hinted/ttf/NotoSansSundanese-Regular.ttf
wget https://github.com/notofonts/batak/raw/main/fonts/NotoSansBatak/hinted/ttf/NotoSansBatak-Regular.ttf
wget https://github.com/notofonts/thai/raw/main/fonts/NotoSansThai/hinted/ttf/NotoSansThai-Regular.ttf
wget https://github.com/notofonts/lao/raw/main/fonts/NotoSansLao/hinted/ttf/NotoSansLao-Regular.ttf
wget https://github.com/notofonts/khmer/raw/main/fonts/NotoSansKhmer/hinted/ttf/NotoSansKhmer-Regular.ttf
wget https://github.com/notofonts/myanmar/raw/main/fonts/NotoSansMyanmar/hinted/ttf/NotoSansMyanmar-Regular.ttf

# RTL
wget https://github.com/notofonts/arabic/raw/main/fonts/NotoSansArabic/hinted/ttf/NotoSansArabic-Regular.ttf
wget https://github.com/notofonts/hebrew/raw/main/fonts/NotoSansHebrew/hinted/ttf/NotoSansHebrew-Regular.ttf
wget https://github.com/notofonts/syriac/raw/main/fonts/NotoSansSyriac/hinted/ttf/NotoSansSyriac-Regular.ttf
wget https://github.com/notofonts/thaana/raw/main/fonts/NotoSansThaana/hinted/ttf/NotoSansThaana-Regular.ttf
wget https://github.com/notofonts/nko/raw/main/fonts/NotoSansNKo/hinted/ttf/NotoSansNKo-Regular.ttf

# CJK (large files)
wget https://github.com/notofonts/noto-cjk/raw/main/Sans/OTF/SimplifiedChinese/NotoSansSC-Regular.otf
wget https://github.com/notofonts/noto-cjk/raw/main/Sans/OTF/TraditionalChinese/NotoSansTC-Regular.otf
wget https://github.com/notofonts/noto-cjk/raw/main/Sans/OTF/Japanese/NotoSansJP-Regular.otf
wget https://github.com/notofonts/noto-cjk/raw/main/Sans/OTF/Korean/NotoSansKR-Regular.otf

# Other
wget https://github.com/notofonts/tibetan/raw/main/fonts/NotoSansTibetan/hinted/ttf/NotoSansTibetan-Regular.ttf
wget https://github.com/notofonts/ethiopic/raw/main/fonts/NotoSansEthiopic/hinted/ttf/NotoSansEthiopic-Regular.ttf
wget https://github.com/notofonts/tifinagh/raw/main/fonts/NotoSansTifinagh/hinted/ttf/NotoSansTifinagh-Regular.ttf

# Emoji
wget https://github.com/googlefonts/noto-emoji/raw/main/fonts/NotoColorEmoji.ttf
```

### Alternative: System Fonts

On Linux systems with Noto fonts installed:
```bash
fc-list | grep "Noto Sans"
```

On macOS:
```bash
# Install via Homebrew
brew tap homebrew/cask-fonts
brew install --cask font-noto-sans
brew install --cask font-noto-sans-cjk
```

## Font Fallback Chain

For comprehensive script support, configure font fallback in this order:

1. **Script-specific font** (e.g., Noto Sans Balinese for Balinese text)
2. **Regional font** (e.g., Noto Sans for Latin)
3. **CJK font** (for Han characters)
4. **Symbol font** (Noto Color Emoji for emoji)

## OpenType Features

Required OpenType features for proper shaping:

| Feature | Tag | Purpose | Scripts |
|---------|-----|---------|---------|
| Localized Forms | `locl` | Language-specific glyphs | All |
| Required Ligatures | `rlig` | Essential ligatures | Arabic, Indic |
| Contextual Alternates | `calt` | Context-dependent forms | Arabic, Indic |
| Conjunct Forms | `pres`, `blws`, `abvs` | Indic conjuncts | Indic, SE Asian |
| Mark Positioning | `mark`, `mkmk` | Diacritics | All complex scripts |
| Cursive Attachment | `curs` | Joining | Arabic, Syriac |

## Font Subsetting

For production use, subset fonts to include only required glyphs:

```bash
# Example: subset Noto Sans Balinese to only include Balinese block
pyftsubset NotoSansBalinese-Regular.ttf \
  --unicodes=U+1B00-1B7F \
  --output-file=NotoSansBalinese-Subset.ttf
```

## License

All Noto fonts are licensed under the **SIL Open Font License 1.1**, which allows:
- Free use in commercial and non-commercial projects
- Modification and redistribution
- Embedding in documents and applications

## See Also

- [Noto Fonts Official Site](https://fonts.google.com/noto)
- [Noto GitHub Repository](https://github.com/notofonts)
- [OpenType Feature Tags](https://docs.microsoft.com/en-us/typography/opentype/spec/featuretags)
