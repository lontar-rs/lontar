# Multi-Script Test Fonts

This directory contains Noto Sans fonts for comprehensive multi-script testing.

## Font Collection Summary

Total fonts: 28 (24 TTF + 4 OTF)
Total size: ~66 MB

### Simple LTR Scripts
- `NotoSans-Regular.ttf` (556 KB) - Latin, Cyrillic, Greek
- `NotoSansGeorgian-Regular.ttf` (52 KB) - Georgian

### Complex Indic Scripts
- `NotoSansDevanagari-Regular.ttf` (239 KB) - Devanagari
- `NotoSansBengali-Regular.ttf` (140 KB) - Bengali
- `NotoSansTamil-Regular.ttf` (73 KB) - Tamil
- `NotoSansTelugu-Regular.ttf` (230 KB) - Telugu
- `NotoSansKannada-Regular.ttf` (179 KB) - Kannada
- `NotoSansMalayalam-Regular.ttf` (111 KB) - Malayalam

### Southeast Asian Scripts
- `NotoSansBalinese-Regular.ttf` (91 KB) - Balinese
- `NotoSansJavanese-Regular.ttf` (118 KB) - Javanese
- `NotoSansSundanese-Regular.ttf` (23 KB) - Sundanese
- `NotoSansBatak-Regular.ttf` (22 KB) - Batak
- `NotoSansThai-Regular.ttf` (37 KB) - Thai
- `NotoSansLao-Regular.ttf` (36 KB) - Lao
- `NotoSansKhmer-Regular.ttf` (112 KB) - Khmer
- `NotoSansMyanmar-Regular.ttf` (191 KB) - Myanmar

### RTL Scripts
- `NotoSansArabic-Regular.ttf` (230 KB) - Arabic
- `NotoSansHebrew-Regular.ttf` (27 KB) - Hebrew
- `NotoSansSyriac-Regular.ttf` (73 KB) - Syriac
- `NotoSansThaana-Regular.ttf` (27 KB) - Thaana
- `NotoSansNKo-Regular.ttf` (39 KB) - N'Ko

### CJK Scripts
- `NotoSansSC-Regular.otf` (16 MB) - Simplified Chinese
- `NotoSansTC-Regular.otf` (16 MB) - Traditional Chinese
- `NotoSansJP-Regular.otf` (16 MB) - Japanese
- `NotoSansKR-Regular.otf` (16 MB) - Korean

### Tibetan Script
- `NotoSansTibetan-Regular.ttf` (296 KB) - Tibetan

### African Scripts
- `NotoSansEthiopic-Regular.ttf` (368 KB) - Ethiopic
- `NotoSansTifinagh-Regular.ttf` (78 KB) - Tifinagh

## Source

All fonts downloaded from:
- https://github.com/notofonts/notofonts.github.io (TTF fonts)
- https://github.com/notofonts/noto-cjk (CJK OTF fonts)

## License

All Noto fonts are licensed under the SIL Open Font License 1.1.
See: https://scripts.sil.org/OFL

## Usage

These fonts are used by:
1. `generate_refs.py` - Generate reference DOCX/PPTX documents
2. `reference_template.tex` - Generate LaTeX reference PDFs
3. Test suites for validating multi-script text shaping

## Verification

To verify all fonts are present:
```bash
ls -1 *.ttf *.otf | wc -l  # Should output: 28
```

To check total size:
```bash
du -sh .  # Should be ~66M
```
