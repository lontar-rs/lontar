//! Test strings for text shaping evaluation across different scripts.
//!
//! These strings are used to verify that rustybuzz correctly handles:
//! - Simple LTR (Latin)
//! - Complex Indic scripts (Devanagari, Bengali, Tamil)
//! - Southeast Asian scripts (Thai, Lao, Balinese)
//! - RTL scripts (Arabic, Hebrew)
//! - CJK scripts (Chinese, Japanese, Korean)

pub struct TestString {
    pub text: &'static str,
    pub script: &'static str,
    pub description: &'static str,
    pub expected_features: &'static [&'static str],
    pub unicode_range: &'static str,
}

/// Test strings covering all major script categories
pub const TEST_STRINGS: &[TestString] = &[
    // ============================================================================
    // SIMPLE LTR (Latin)
    // ============================================================================
    TestString {
        text: "Hello World",
        script: "Latin",
        description: "Simple English text, no ligatures",
        expected_features: &[],
        unicode_range: "U+0041-U+007A (Basic Latin)",
    },
    TestString {
        text: "The quick brown fox jumps over the lazy dog.",
        script: "Latin",
        description: "Pangram with common ligatures (fi, fl)",
        expected_features: &["liga"],
        unicode_range: "U+0041-U+007A (Basic Latin)",
    },
    TestString {
        text: "Café résumé naïve",
        script: "Latin Extended",
        description: "Latin with diacritics (accents)",
        expected_features: &[],
        unicode_range: "U+00C0-U+00FF (Latin Extended-A)",
    },
    // ============================================================================
    // COMPLEX INDIC SCRIPTS
    // ============================================================================
    TestString {
        text: "नमस्ते",
        script: "Devanagari",
        description: "Hindi greeting with conjuncts (न्म, स्त)",
        expected_features: &["blwf", "pstf", "ccmp"],
        unicode_range: "U+0900-U+097F (Devanagari)",
    },
    TestString {
        text: "आप कैसे हैं?",
        script: "Devanagari",
        description: "Hindi question with virama-based conjuncts",
        expected_features: &["blwf", "pstf", "ccmp"],
        unicode_range: "U+0900-U+097F (Devanagari)",
    },
    TestString {
        text: "স্বাগতম",
        script: "Bengali",
        description: "Bengali greeting with conjuncts (স্ব)",
        expected_features: &["blwf", "pstf", "ccmp"],
        unicode_range: "U+0980-U+09FF (Bengali)",
    },
    TestString {
        text: "தமிழ்",
        script: "Tamil",
        description: "Tamil word with virama (்)",
        expected_features: &["blwf", "pstf"],
        unicode_range: "U+0B80-U+0BFF (Tamil)",
    },
    TestString {
        text: "తెలుగు",
        script: "Telugu",
        description: "Telugu word with complex vowel marks",
        expected_features: &["blwf", "pstf", "ccmp"],
        unicode_range: "U+0C60-U+0C7F (Telugu)",
    },
    TestString {
        text: "ಕನ್ನಡ",
        script: "Kannada",
        description: "Kannada word with conjuncts (ನ್ನ)",
        expected_features: &["blwf", "pstf", "ccmp"],
        unicode_range: "U+0C80-U+0CFF (Kannada)",
    },
    TestString {
        text: "മലയാളം",
        script: "Malayalam",
        description: "Malayalam word with complex forms",
        expected_features: &["blwf", "pstf", "ccmp"],
        unicode_range: "U+0D00-U+0D7F (Malayalam)",
    },
    // ============================================================================
    // SOUTHEAST ASIAN SCRIPTS
    // ============================================================================
    TestString {
        text: "สวัสดี",
        script: "Thai",
        description: "Thai greeting with tone marks and vowels",
        expected_features: &["ccmp"],
        unicode_range: "U+0E00-U+0E7F (Thai)",
    },
    TestString {
        text: "ສະບາຍດີ",
        script: "Lao",
        description: "Lao greeting with tone marks",
        expected_features: &["ccmp"],
        unicode_range: "U+0E80-U+0EFF (Lao)",
    },
    TestString {
        text: "ᬮᭀᬦ᭄ᬢᬭ᭄",
        script: "Balinese",
        description: "Balinese word 'lontar' with virama and adeg",
        expected_features: &["blwf", "pstf", "ccmp"],
        unicode_range: "U+1B00-U+1B7F (Balinese)",
    },
    TestString {
        text: "ᬅᬓ᭄ᬱᬭ ᬩᬮᬶ",
        script: "Balinese",
        description: "Balinese phrase with multiple conjuncts",
        expected_features: &["blwf", "pstf", "ccmp"],
        unicode_range: "U+1B00-U+1B7F (Balinese)",
    },
    TestString {
        text: "ខ្មែរ",
        script: "Khmer",
        description: "Khmer word with subscript consonants",
        expected_features: &["blwf", "pstf"],
        unicode_range: "U+1780-U+17FF (Khmer)",
    },
    TestString {
        text: "မြန်မာ",
        script: "Myanmar",
        description: "Myanmar word with medial consonants",
        expected_features: &["blwf", "pstf"],
        unicode_range: "U+1000-U+109F (Myanmar)",
    },
    // ============================================================================
    // RTL SCRIPTS
    // ============================================================================
    TestString {
        text: "مرحبا",
        script: "Arabic",
        description: "Arabic greeting with contextual forms",
        expected_features: &["init", "medi", "fina"],
        unicode_range: "U+0600-U+06FF (Arabic)",
    },
    TestString {
        text: "السلام عليكم",
        script: "Arabic",
        description: "Arabic phrase with multiple words",
        expected_features: &["init", "medi", "fina"],
        unicode_range: "U+0600-U+06FF (Arabic)",
    },
    TestString {
        text: "שלום",
        script: "Hebrew",
        description: "Hebrew greeting",
        expected_features: &[],
        unicode_range: "U+0590-U+05FF (Hebrew)",
    },
    // ============================================================================
    // CJK SCRIPTS
    // ============================================================================
    TestString {
        text: "你好世界",
        script: "Han (Chinese)",
        description: "Chinese 'Hello World' with simplified characters",
        expected_features: &[],
        unicode_range: "U+4E00-U+9FFF (CJK Unified Ideographs)",
    },
    TestString {
        text: "日本語",
        script: "Japanese",
        description: "Japanese with Kanji (日本) and Hiragana (語)",
        expected_features: &[],
        unicode_range: "U+4E00-U+9FFF (Kanji), U+3040-U+309F (Hiragana)",
    },
    TestString {
        text: "ひらがなカタカナ",
        script: "Japanese",
        description: "Japanese Hiragana and Katakana",
        expected_features: &[],
        unicode_range: "U+3040-U+309F (Hiragana), U+30A0-U+30FF (Katakana)",
    },
    TestString {
        text: "한글",
        script: "Korean",
        description: "Korean Hangul",
        expected_features: &[],
        unicode_range: "U+AC00-U+D7AF (Hangul Syllables)",
    },
    // ============================================================================
    // MIXED SCRIPTS (for BiDi testing)
    // ============================================================================
    TestString {
        text: "Hello مرحبا World",
        script: "Mixed (Latin + Arabic + Latin)",
        description: "Mixed LTR and RTL text",
        expected_features: &["init", "medi", "fina"],
        unicode_range: "U+0041-U+007A (Latin), U+0600-U+06FF (Arabic)",
    },
    TestString {
        text: "The word नमस्ते means hello",
        script: "Mixed (Latin + Devanagari + Latin)",
        description: "English with embedded Devanagari",
        expected_features: &["blwf", "pstf", "ccmp"],
        unicode_range: "U+0041-U+007A (Latin), U+0900-U+097F (Devanagari)",
    },
    TestString {
        text: "สวัสดี Hello 你好",
        script: "Mixed (Thai + Latin + Han)",
        description: "Thai, English, and Chinese",
        expected_features: &["ccmp"],
        unicode_range: "U+0E00-U+0E7F (Thai), U+0041-U+007A (Latin), U+4E00-U+9FFF (Han)",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strings_are_valid() {
        for test_str in TEST_STRINGS {
            // Verify text is not empty
            assert!(
                !test_str.text.is_empty(),
                "Text is empty for {}",
                test_str.script
            );

            // Verify script name is provided
            assert!(!test_str.script.is_empty(), "Script name is empty");

            // Verify description is provided
            assert!(!test_str.description.is_empty(), "Description is empty");

            // Verify unicode range is provided
            assert!(!test_str.unicode_range.is_empty(), "Unicode range is empty");
        }
    }

    #[test]
    fn test_strings_by_script() {
        let scripts = vec![
            "Latin",
            "Latin Extended",
            "Devanagari",
            "Bengali",
            "Tamil",
            "Telugu",
            "Kannada",
            "Malayalam",
            "Thai",
            "Lao",
            "Balinese",
            "Khmer",
            "Myanmar",
            "Arabic",
            "Hebrew",
            "Han (Chinese)",
            "Japanese",
            "Korean",
        ];

        for script in scripts {
            let count = TEST_STRINGS.iter().filter(|s| s.script == script).count();
            println!("{}: {} test strings", script, count);
        }
    }

    #[test]
    fn test_balinese_strings() {
        let balinese_strings: Vec<_> = TEST_STRINGS
            .iter()
            .filter(|s| s.script == "Balinese")
            .collect();

        assert!(
            !balinese_strings.is_empty(),
            "No Balinese test strings found"
        );

        for test_str in balinese_strings {
            println!("Balinese: {} ({})", test_str.text, test_str.description);
        }
    }

    #[test]
    fn test_mixed_script_strings() {
        let mixed_strings: Vec<_> = TEST_STRINGS
            .iter()
            .filter(|s| s.script.contains("Mixed"))
            .collect();

        assert!(
            !mixed_strings.is_empty(),
            "No mixed script test strings found"
        );

        for test_str in mixed_strings {
            println!("Mixed: {} ({})", test_str.text, test_str.description);
        }
    }
}
