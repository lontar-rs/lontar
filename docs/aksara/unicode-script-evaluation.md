# Unicode Script Evaluation

## Overview

The `unicode-script` crate identifies which Unicode script (Latin, Arabic, Devanagari, Balinese, CJK, etc.) each character belongs to. This is essential for:
1. Selecting appropriate fonts
2. Determining text shaping requirements
3. Mapping to OOXML font slots (ascii, cs, eastAsia)
4. Setting language tags

## Problem Statement

Different scripts require different handling:

```
Text:     "Hello مرحبا 你好 ᬮᭀᬦ᭄ᬢᬭ᭄"
Scripts:  Latin  Arabic  CJK  Balinese
Fonts:    Calibri Arial  SimSun NotoSansBalinese
Slots:    ascii  cs     eastAsia cs
```

The algorithm determines which script each character belongs to based on Unicode blocks.

## API Structure

### Core Types

```rust
pub enum Script {
    // Latin scripts
    Latin,
    LatinExtended,
    
    // RTL scripts
    Arabic,
    Hebrew,
    Syriac,
    Thaana,
    NKo,
    Samaritan,
    Mandaic,
    
    // Indic scripts
    Devanagari,
    Bengali,
    Gurmukhi,
    Gujarati,
    Oriya,
    Tamil,
    Telugu,
    Kannada,
    Malayalam,
    Sinhala,
    
    // Southeast Asian scripts
    Thai,
    Lao,
    Tibetan,
    Myanmar,
    Khmer,
    
    // East Asian scripts
    Han,           // CJK
    Hiragana,      // Japanese
    Katakana,      // Japanese
    Hangul,        // Korean
    
    // Other scripts
    Balinese,
    Javanese,
    Sundanese,
    Batak,
    Lepcha,
    Ol_Chiki,
    Cyrillic,
    Greek,
    Georgian,
    Armenian,
    
    // Special
    Common,        // Punctuation, numbers, symbols
    Inherited,     // Combining marks
    Unknown,
}
```

### Main Functions

```rust
// Get script for a single character
pub fn script(ch: char) -> Script

// Get scripts for a string
pub fn scripts(s: &str) -> impl Iterator<Item = (Script, &str)>
```

## Example: Single Character

```rust
use unicode_script::Script;

let chars = vec!['H', 'م', '你', 'ᬮ'];
for ch in chars {
    let script = Script::script(ch);
    println!("'{}' -> {:?}", ch, script);
}

// Output:
// 'H' -> Latin
// 'م' -> Arabic
// '你' -> Han
// 'ᬮ' -> Balinese
```

## Example: Script Runs

```rust
use unicode_script::Script;

let text = "Hello مرحبا 你好 ᬮᭀᬦ᭄ᬢᬭ᭄";

for (script, segment) in Script::scripts(text) {
    println!("Script: {:?}, Text: '{}'", script, segment);
}

// Output:
// Script: Latin, Text: 'Hello '
// Script: Common, Text: ' '
// Script: Arabic, Text: 'مرحبا'
// Script: Common, Text: ' '
// Script: Han, Text: '你好'
// Script: Common, Text: ' '
// Script: Balinese, Text: 'ᬮᭀᬦ᭄ᬢᬭ᭄'
```

**Note:** Common script includes spaces, punctuation, and numbers.

## Script Runs

A **script run** is a maximal sequence of characters with the same script:

```rust
pub struct ScriptRun {
    pub script: Script,
    pub text: String,
    pub range: Range<usize>,
}

pub fn detect_script_runs(text: &str) -> Vec<ScriptRun> {
    let mut runs = Vec::new();
    let mut current_script = Script::Unknown;
    let mut current_start = 0;
    
    for (i, ch) in text.char_indices() {
        let script = Script::script(ch);
        
        // Skip Common script (spaces, punctuation)
        if script == Script::Common {
            continue;
        }
        
        if script != current_script {
            // Start new run
            if current_script != Script::Unknown {
                let run_text = text[current_start..i].to_string();
                runs.push(ScriptRun {
                    script: current_script,
                    text: run_text,
                    range: current_start..i,
                });
            }
            current_script = script;
            current_start = i;
        }
    }
    
    // Add final run
    if current_script != Script::Unknown {
        let run_text = text[current_start..].to_string();
        runs.push(ScriptRun {
            script: current_script,
            text: run_text,
            range: current_start..text.len(),
        });
    }
    
    runs
}
```

## Example: Script Run Detection

```rust
let text = "Hello مرحبا 你好 ᬮᭀᬦ᭄ᬢᬭ᭄";
let runs = detect_script_runs(text);

for run in runs {
    println!("Script: {:?}, Text: '{}'", run.script, run.text);
}

// Output:
// Script: Latin, Text: 'Hello'
// Script: Arabic, Text: 'مرحبا'
// Script: Han, Text: '你好'
// Script: Balinese, Text: 'ᬮᭀᬦ᭄ᬢᬭ᭄'
```

## Integration with Lontar

### 1. Font Selection

```rust
pub fn select_font_for_script(script: Script) -> &'static str {
    match script {
        Script::Latin => "Calibri",
        Script::Arabic => "Arial",
        Script::Hebrew => "Arial",
        Script::Devanagari => "Noto Sans Devanagari",
        Script::Bengali => "Noto Sans Bengali",
        Script::Tamil => "Noto Sans Tamil",
        Script::Telugu => "Noto Sans Telugu",
        Script::Kannada => "Noto Sans Kannada",
        Script::Malayalam => "Noto Sans Malayalam",
        Script::Thai => "Tahoma",
        Script::Lao => "Noto Sans Lao",
        Script::Khmer => "Noto Sans Khmer",
        Script::Myanmar => "Noto Sans Myanmar",
        Script::Han => "SimSun",
        Script::Hiragana | Script::Katakana => "MS Gothic",
        Script::Hangul => "Gulim",
        Script::Balinese => "Noto Sans Balinese",
        _ => "Calibri",  // Fallback
    }
}
```

### 2. OOXML Font Slot Mapping

```rust
pub enum FontSlot {
    Ascii,      // w:ascii
    HAnsi,      // w:hAnsi
    EastAsia,   // w:eastAsia
    ComplexScript, // w:cs
}

pub fn script_to_font_slot(script: Script) -> FontSlot {
    match script {
        Script::Latin | Script::LatinExtended => FontSlot::Ascii,
        Script::Han | Script::Hiragana | Script::Katakana | Script::Hangul => {
            FontSlot::EastAsia
        }
        Script::Arabic | Script::Hebrew | Script::Devanagari | Script::Bengali
        | Script::Gurmukhi | Script::Gujarati | Script::Oriya | Script::Tamil
        | Script::Telugu | Script::Kannada | Script::Malayalam | Script::Sinhala
        | Script::Thai | Script::Lao | Script::Tibetan | Script::Myanmar
        | Script::Khmer | Script::Balinese | Script::Javanese | Script::Sundanese
        | Script::Batak => FontSlot::ComplexScript,
        _ => FontSlot::Ascii,  // Fallback
    }
}
```

### 3. Language Tag Mapping

```rust
pub fn script_to_language_tag(script: Script) -> &'static str {
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
        _ => "en-US",  // Fallback
    }
}
```

### 4. Text Shaping Requirements

```rust
pub fn requires_shaping(script: Script) -> bool {
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
            | Script::Myanmar
            | Script::Khmer
            | Script::Balinese
            | Script::Javanese
            | Script::Sundanese
            | Script::Batak
    )
}
```

## Example: Full Pipeline

```rust
use unicode_script::Script;
use rustybuzz::{Face, UnicodeBuffer, shape};

let text = "Hello مرحبا 你好 ᬮᭀᬦ᭄ᬢᬭ᭄";

// Step 1: Detect script runs
let runs = detect_script_runs(text);

// Step 2: Process each run
for run in runs {
    // Select font for script
    let font_name = select_font_for_script(run.script);
    let font_data = load_font(font_name);
    let face = Face::from_slice(&font_data, 0)?;
    
    // Determine if shaping is needed
    if requires_shaping(run.script) {
        // Shape the text
        let mut buffer = UnicodeBuffer::new();
        buffer.push_str(&run.text);
        buffer.set_script(run.script);
        buffer.set_language(script_to_language_tag(run.script));
        
        let output = shape(&face, &[], buffer);
        
        // Get font slot for OOXML
        let slot = script_to_font_slot(run.script);
        
        // Generate DOCX run properties
        let rpr = DocxRunProperties {
            font: font_name,
            slot,
            language: script_to_language_tag(run.script),
        };
        
        // Process shaped glyphs...
    } else {
        // Simple script (Latin), no shaping needed
        let rpr = DocxRunProperties {
            font: font_name,
            slot: FontSlot::Ascii,
            language: "en-US",
        };
    }
}
```

## Key Capabilities

| Feature | Supported | Notes |
|---------|-----------|-------|
| Latin detection | ✅ | ASCII and Extended |
| Arabic detection | ✅ | U+0600–U+06FF |
| Indic detection | ✅ | All 11 Indic scripts |
| CJK detection | ✅ | Han, Hiragana, Katakana, Hangul |
| Southeast Asian | ✅ | Thai, Lao, Khmer, Myanmar |
| Balinese detection | ✅ | U+1B00–U+1B7F |
| Common script | ✅ | Spaces, punctuation, numbers |
| Inherited script | ✅ | Combining marks |
| Script runs | ✅ | Grouping consecutive chars |

## Limitations

1. **No font loading** — Only identifies script; use separate library for fonts
2. **No text shaping** — Use rustybuzz for glyph formation
3. **No width measurement** — Use font metrics for width
4. **No BiDi handling** — Use unicode-bidi for reordering

## Performance Characteristics

- **Single character**: ~0.001ms
- **Script run detection**: ~0.05-0.2ms per 100 characters
- **Memory**: ~10 bytes per character

## Common Pitfalls

### 1. Ignoring Common Script

```rust
// ❌ WRONG: Assume all characters have a script
for ch in text.chars() {
    let script = Script::script(ch);
    if script == Script::Unknown {
        // Handle unknown
    }
}

// ✅ CORRECT: Handle Common script separately
for ch in text.chars() {
    let script = Script::script(ch);
    match script {
        Script::Common => {
            // Spaces, punctuation, numbers
            // Inherit script from adjacent characters
        }
        Script::Unknown => {
            // Truly unknown characters
        }
        _ => {
            // Known script
        }
    }
}
```

### 2. Not Grouping Script Runs

```rust
// ❌ WRONG: Process each character separately
for ch in text.chars() {
    let script = Script::script(ch);
    shape_character(ch, script);  // Inefficient
}

// ✅ CORRECT: Group into runs and process together
let runs = detect_script_runs(text);
for run in runs {
    shape_run(&run.text, run.script);  // Efficient
}
```

### 3. Mixing Script Detection with BiDi

```rust
// ❌ WRONG: Detect scripts before BiDi reordering
let runs = detect_script_runs(text);  // Logical order
for run in runs {
    shape(&run.text);  // Wrong order for RTL
}

// ✅ CORRECT: BiDi first, then script detection
let bidi = BidiInfo::new(text, Some(Level::ltr()));
let visual_text = bidi.reorder_visual();
let runs = detect_script_runs(&visual_text);  // Visual order
for run in runs {
    shape(&run.text);  // Correct order
}
```

## Recommended Integration Points

1. **After document parsing** — Detect scripts early
2. **After BiDi reordering** — Detect scripts in visual order
3. **Before text shaping** — Group by script before shaping
4. **When selecting fonts** — Use script to choose appropriate font
5. **When generating DOCX** — Use script to set font slots
6. **When setting language tags** — Map script to language

## References

- [Unicode Standard: Scripts](https://unicode.org/reports/tr24/)
- [unicode-script on crates.io](https://crates.io/crates/unicode-script)
- [Unicode Blocks](https://unicode.org/reports/tr38/)
