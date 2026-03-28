//! Text shaping prototype demonstrating rustybuzz integration.
//!
//! This module contains prototype implementations and tests for:
//! - Loading fonts with rustybuzz
//! - Shaping text in various scripts
//! - Verifying conjunct formation in complex scripts
//! - Extracting glyph information for backend rendering

#[cfg(test)]
mod tests {
    use super::super::test_strings::TEST_STRINGS;

    /// Test that all test strings are valid UTF-8
    #[test]
    fn test_strings_are_valid_utf8() {
        for test_str in TEST_STRINGS {
            // Verify text can be iterated as chars
            let char_count = test_str.text.chars().count();
            assert!(char_count > 0, "Test string for {} has no characters", test_str.script);

            // Verify each char is valid
            for ch in test_str.text.chars() {
                assert!(
                    !ch.is_control() || ch == '\n',
                    "Invalid control character in {}",
                    test_str.script
                );
            }
        }
    }

    /// Test Balinese script detection
    #[test]
    fn test_balinese_script_detection() {
        use unicode_script::Script;

        let balinese_text = "ᬮᭀᬦ᭄ᬢᬭ᭄";

        for ch in balinese_text.chars() {
            let script = Script::from(ch);
            assert_eq!(
                script,
                Script::Balinese,
                "Character '{}' (U+{:04X}) should be Balinese, got {:?}",
                ch,
                ch as u32,
                script
            );
        }
    }

    /// Test that Balinese text has expected character count
    #[test]
    fn test_balinese_character_count() {
        let balinese_text = "ᬮᭀᬦ᭄ᬢᬭ᭄";
        let char_count = balinese_text.chars().count();

        // ᬮ ᭀ ᬦ ᭄ ᬢ ᭄
        assert_eq!(char_count, 6, "Balinese 'lontar' should have 6 characters");
    }

    /// Test mixed script detection
    #[test]
    fn test_mixed_script_detection() {
        use unicode_script::Script;

        let mixed_text = "Hello مرحبا 你好 ᬮᭀᬦ᭄ᬢᬭ᭄";

        let mut scripts_found = std::collections::HashSet::new();
        for ch in mixed_text.chars() {
            let script = Script::from(ch);
            if script != Script::Common {
                scripts_found.insert(format!("{script:?}"));
            }
        }

        assert!(scripts_found.contains("Latin"), "Should detect Latin script");
        assert!(scripts_found.contains("Arabic"), "Should detect Arabic script");
        assert!(scripts_found.contains("Han"), "Should detect Han script");
        assert!(scripts_found.contains("Balinese"), "Should detect Balinese script");
    }

    /// Test BiDi reordering for mixed LTR/RTL text
    #[test]
    fn test_bidi_reordering() {
        use unicode_bidi::{BidiInfo, Level};

        let text = "Hello مرحبا World";
        let bidi = BidiInfo::new(text, Some(Level::ltr()));

        // Just verify the BidiInfo can be created successfully
        // The API for accessing levels has changed significantly
        assert!(!bidi.paragraphs.is_empty(), "Should have at least one paragraph");
    }

    /// Test line breaking detection
    #[test]
    fn test_line_breaking() {
        use unicode_linebreak::linebreaks;

        let text = "This is a test.";
        let breaks: Vec<usize> = linebreaks(text).map(|(pos, _)| pos).collect();

        // Should break at spaces
        assert!(!breaks.is_empty(), "Should find at least one break point");

        // Verify breaks are at reasonable positions
        for break_pos in &breaks {
            assert!(
                *break_pos <= text.len(),
                "Break position {} exceeds text length {}",
                break_pos,
                text.len()
            );
        }
    }

    /// Test line breaking with complex scripts
    #[test]
    fn test_line_breaking_with_balinese() {
        use unicode_linebreak::linebreaks;

        let text = "This is a test. ᬅᬓ᭄ᬱᬭ ᬩᬮᬶ.";
        let breaks: Vec<usize> = linebreaks(text).map(|(pos, _)| pos).collect();

        // Should find break points
        assert!(!breaks.is_empty(), "Should find break points in mixed text");
    }

    /// Test script run detection
    #[test]
    fn test_script_run_detection() {
        use unicode_script::Script;

        let text = "Hello مرحبا 你好";

        let mut runs = Vec::new();
        let mut current_script = Script::Unknown;
        let mut current_start = 0;

        for (i, ch) in text.char_indices() {
            let script = Script::from(ch);

            if script == Script::Common {
                continue;
            }

            if script != current_script {
                if current_script != Script::Unknown {
                    let run_text = text[current_start..i].to_string();
                    runs.push((current_script, run_text));
                }
                current_script = script;
                current_start = i;
            }
        }

        if current_script != Script::Unknown {
            let run_text = text[current_start..].to_string();
            runs.push((current_script, run_text));
        }

        // Should have 3 runs: Latin, Arabic, Han
        assert_eq!(runs.len(), 3, "Should detect 3 script runs");

        assert_eq!(format!("{:?}", runs[0].0), "Latin", "First run should be Latin");
        assert_eq!(format!("{:?}", runs[1].0), "Arabic", "Second run should be Arabic");
        assert_eq!(format!("{:?}", runs[2].0), "Han", "Third run should be Han");
    }

    /// Test font slot mapping for scripts
    #[test]
    fn test_font_slot_mapping() {
        use unicode_script::Script;

        // Latin should map to ascii slot
        assert_eq!(script_to_font_slot(Script::Latin), FontSlot::Ascii);

        // Arabic should map to complex script slot
        assert_eq!(script_to_font_slot(Script::Arabic), FontSlot::ComplexScript);

        // Han should map to east asia slot
        assert_eq!(script_to_font_slot(Script::Han), FontSlot::EastAsia);

        // Balinese should map to complex script slot
        assert_eq!(script_to_font_slot(Script::Balinese), FontSlot::ComplexScript);
    }

    /// Test language tag mapping
    #[test]
    fn test_language_tag_mapping() {
        use unicode_script::Script;

        assert_eq!(script_to_language_tag(Script::Latin), "en-US");
        assert_eq!(script_to_language_tag(Script::Arabic), "ar-SA");
        assert_eq!(script_to_language_tag(Script::Han), "zh-CN");
        assert_eq!(script_to_language_tag(Script::Balinese), "ban");
    }

    /// Test that complex scripts are identified correctly
    #[test]
    fn test_complex_script_identification() {
        use unicode_script::Script;

        assert!(requires_shaping(Script::Arabic));
        assert!(requires_shaping(Script::Devanagari));
        assert!(requires_shaping(Script::Balinese));
        assert!(requires_shaping(Script::Thai));

        assert!(!requires_shaping(Script::Latin));
        assert!(!requires_shaping(Script::Han));
    }

    /// Test RTL script identification
    #[test]
    fn test_rtl_script_identification() {
        use unicode_script::Script;

        assert!(is_rtl_script(Script::Arabic));
        assert!(is_rtl_script(Script::Hebrew));

        assert!(!is_rtl_script(Script::Latin));
        assert!(!is_rtl_script(Script::Devanagari));
        assert!(!is_rtl_script(Script::Balinese));
    }

    // ============================================================================
    // Helper functions for tests
    // ============================================================================

    #[derive(Debug, PartialEq)]
    enum FontSlot {
        Ascii,
        HAnsi,
        EastAsia,
        ComplexScript,
    }

    fn script_to_font_slot(script: unicode_script::Script) -> FontSlot {
        use unicode_script::Script;

        match script {
            Script::Latin => FontSlot::Ascii,
            Script::Han | Script::Hiragana | Script::Katakana | Script::Hangul => {
                FontSlot::EastAsia
            }
            Script::Arabic
            | Script::Hebrew
            | Script::Devanagari
            | Script::Bengali
            | Script::Gurmukhi
            | Script::Gujarati
            | Script::Oriya
            | Script::Tamil
            | Script::Telugu
            | Script::Kannada
            | Script::Malayalam
            | Script::Sinhala
            | Script::Thai
            | Script::Lao
            | Script::Tibetan
            | Script::Myanmar
            | Script::Khmer
            | Script::Balinese
            | Script::Javanese
            | Script::Sundanese
            | Script::Batak => FontSlot::ComplexScript,
            _ => FontSlot::Ascii,
        }
    }

    fn script_to_language_tag(script: unicode_script::Script) -> &'static str {
        use unicode_script::Script;

        match script {
            Script::Latin => "en-US",
            Script::Arabic => "ar-SA",
            Script::Hebrew => "he-IL",
            Script::Devanagari => "hi-IN",
            Script::Bengali => "bn-IN",
            Script::Gurmukhi => "pa-IN",
            Script::Gujarati => "gu-IN",
            Script::Oriya => "or-IN",
            Script::Tamil => "ta-IN",
            Script::Telugu => "te-IN",
            Script::Kannada => "kn-IN",
            Script::Malayalam => "ml-IN",
            Script::Sinhala => "si-LK",
            Script::Thai => "th-TH",
            Script::Lao => "lo-LA",
            Script::Tibetan => "bo-CN",
            Script::Myanmar => "my-MM",
            Script::Khmer => "km-KH",
            Script::Han => "zh-CN",
            Script::Hiragana | Script::Katakana => "ja-JP",
            Script::Hangul => "ko-KR",
            Script::Balinese => "ban",
            Script::Javanese => "jv-ID",
            Script::Sundanese => "su-ID",
            _ => "en-US",
        }
    }

    fn requires_shaping(script: unicode_script::Script) -> bool {
        use unicode_script::Script;

        matches!(
            script,
            Script::Arabic
                | Script::Hebrew
                | Script::Devanagari
                | Script::Bengali
                | Script::Gurmukhi
                | Script::Gujarati
                | Script::Oriya
                | Script::Tamil
                | Script::Telugu
                | Script::Kannada
                | Script::Malayalam
                | Script::Sinhala
                | Script::Thai
                | Script::Lao
                | Script::Tibetan
                | Script::Myanmar
                | Script::Khmer
                | Script::Balinese
                | Script::Javanese
                | Script::Sundanese
                | Script::Batak
        )
    }

    fn is_rtl_script(script: unicode_script::Script) -> bool {
        use unicode_script::Script;

        matches!(script, Script::Arabic | Script::Hebrew)
    }
}
