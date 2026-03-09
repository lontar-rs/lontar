# Expected Shaping Behaviors (Reference Notes)

These notes summarize expected shaping features per script category for verification against reference outputs.

## Simple LTR
- Latin/Cyrillic/Greek/Georgian: no contextual shaping; verify basic ligatures only if font provides (optional).

## Complex Indic
- Devanagari: conjunct formation (क्ष, त्र), split matras (ि before consonant), reph positioning, halant handling.
- Bengali: reph placement, ya-phala, split matras (ি, ী), conjuncts (ক্য, শ্র).
- Tamil: minimal shaping; verify ligatures like க்ஷ.
- Telugu/Kannada: reph analogs not used; verify matra placement, conjuncts (క్ష, ತ್ರ, ಕ್ರ).
- Malayalam: chillus, reph-like features (റ + chandrakala), ligatures like ക്ഷ.

## Southeast Asian
- Balinese/Javanese/Sundanese/Batak: stacked consonants; dependent vowel ordering; verify subjoined glyphs.
- Thai/Lao: tone mark ordering above vowels; vowel reordering (เ-า etc.); no conjunct stacking.
- Khmer: subjoined consonants; coeng; split vowels; mark collision handling.
- Myanmar: stacked consonants (kinzi), medial forms, ligatures (္ယ,္ရ,္ဝ), vowel ordering.

## RTL
- Arabic: contextual joining (init/med/final/isol), lam-alef ligatures, tatweel handling, mark positioning (tashkeel).
- Hebrew: mostly non-joining; optional presentation forms; right-to-left ordering.
- Syriac/Thaana/N'Ko: contextual forms similar to Arabic; verify bidi ordering and joining.

## CJK
- Chinese/Japanese/Korean: no shaping; verify correct glyph coverage and width; CJK punctuation forms.

## Tibetan
- Stacking consonants; subjoined forms; tsek alignment; vowel signs placement.

## African
- Ethiopic: syllabic forms; no contextual shaping beyond font-specific ligatures.
- Tifinagh: basic non-joining script.

## Mixed Scripts
- Latin + Balinese, Arabic + English, CJK + Latin: ensure script runs and bidi ordering (for Arabic) are correct; no unintended font fallback if fonts provided per script.
