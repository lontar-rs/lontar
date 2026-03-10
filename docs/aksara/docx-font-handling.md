# DOCX Font Handling

## Overview

This document details how to integrate text shaping results into DOCX documents, focusing on font selection, language tagging, and embedding.

## Integration Pipeline

```
1. Input text: "Hello مرحبا 你好 ᬮᭀᬦ᭄ᬢᬭ᭄"
2. BiDi reordering: "Hello ابحرم 你好 ᬮᭀᬦ᭄ᬢᬭ᭄"
3. Script detection: [Latin, Arabic, Han, Balinese]
4. Text shaping: Shape each script run with rustybuzz
5. Font selection: Select font for each script
6. DOCX generation: Create w:rFonts, w:lang, embed fonts
```

## Step 1: Detect Script Runs

```rust
use unicode_script::Script;

pub fn detect_script_runs(text: &str) -> Vec<ScriptRun> {
    let mut runs = Vec::new();
    let mut current_script = Script::Unknown;
    let mut current_start = 0;

    for (i, ch) in text.char_indices() {
        let script = Script::script(ch);

        // Skip Common script
        if script == Script::Common {
            continue;
        }

        if script != current_script {
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

## Step 2: Select Fonts

```rust
pub fn select_font_for_script(script: Script) -> &'static str {
    match script {
        Script::Latin | Script::LatinExtended => "Calibri",
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
        _ => "Calibri",
    }
}
```

## Step 3: Map to OOXML Font Slots

```rust
pub enum FontSlot {
    Ascii,           // w:ascii (U+0000–U+007F)
    HAnsi,           // w:hAnsi (U+0080–U+00FF)
    EastAsia,        // w:eastAsia (CJK)
    ComplexScript,   // w:cs (Arabic, Indic, etc.)
}

pub fn script_to_font_slot(script: Script) -> FontSlot {
    match script {
        Script::Latin => FontSlot::Ascii,
        Script::LatinExtended => FontSlot::HAnsi,
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
```

## Step 4: Generate w:rFonts Element

```rust
pub struct DocxRunFonts {
    pub ascii: Option<String>,
    pub h_ansi: Option<String>,
    pub east_asia: Option<String>,
    pub cs: Option<String>,
}

pub fn generate_rfonts(script_runs: &[ScriptRun]) -> DocxRunFonts {
    let mut fonts = DocxRunFonts {
        ascii: None,
        h_ansi: None,
        east_asia: None,
        cs: None,
    };

    for run in script_runs {
        let font_name = select_font_for_script(run.script);
        let slot = script_to_font_slot(run.script);

        match slot {
            FontSlot::Ascii => fonts.ascii = Some(font_name.to_string()),
            FontSlot::HAnsi => fonts.h_ansi = Some(font_name.to_string()),
            FontSlot::EastAsia => fonts.east_asia = Some(font_name.to_string()),
            FontSlot::ComplexScript => fonts.cs = Some(font_name.to_string()),
        }
    }

    fonts
}

pub fn rfonts_to_xml(fonts: &DocxRunFonts) -> String {
    let mut xml = String::from("<w:rFonts");

    if let Some(ascii) = &fonts.ascii {
        xml.push_str(&format!(r#" w:ascii="{}""#, ascii));
    }
    if let Some(h_ansi) = &fonts.h_ansi {
        xml.push_str(&format!(r#" w:hAnsi="{}""#, h_ansi));
    }
    if let Some(east_asia) = &fonts.east_asia {
        xml.push_str(&format!(r#" w:eastAsia="{}""#, east_asia));
    }
    if let Some(cs) = &fonts.cs {
        xml.push_str(&format!(r#" w:cs="{}""#, cs));
    }

    xml.push_str("/>");
    xml
}
```

**Example Output:**

```xml
<w:rFonts w:ascii="Calibri" w:hAnsi="Calibri" w:cs="Arial" w:eastAsia="SimSun"/>
```

## Step 5: Generate w:lang Element

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
        _ => "en-US",
    }
}

pub struct DocxRunLanguage {
    pub default: String,
    pub east_asia: Option<String>,
    pub bidi: Option<String>,
}

pub fn generate_lang(script_runs: &[ScriptRun]) -> DocxRunLanguage {
    let mut default = "en-US".to_string();
    let mut east_asia = None;
    let mut bidi = None;

    for run in script_runs {
        let lang = script_to_language_tag(run.script);

        match run.script {
            Script::Han | Script::Hiragana | Script::Katakana | Script::Hangul => {
                east_asia = Some(lang.to_string());
            }
            Script::Arabic | Script::Hebrew => {
                bidi = Some(lang.to_string());
            }
            _ => {
                default = lang.to_string();
            }
        }
    }

    DocxRunLanguage {
        default,
        east_asia,
        bidi,
    }
}

pub fn lang_to_xml(lang: &DocxRunLanguage) -> String {
    let mut xml = String::from("<w:lang");

    xml.push_str(&format!(r#" w:val="{}""#, lang.default));

    if let Some(east_asia) = &lang.east_asia {
        xml.push_str(&format!(r#" w:eastAsia="{}""#, east_asia));
    }
    if let Some(bidi) = &lang.bidi {
        xml.push_str(&format!(r#" w:bidi="{}""#, bidi));
    }

    xml.push_str("/>");
    xml
}
```

**Example Output:**

```xml
<w:lang w:val="en-US" w:eastAsia="zh-CN" w:bidi="ar-SA"/>
```

## Step 6: Set Complex Script Flags

```rust
pub struct DocxRunProperties {
    pub fonts: DocxRunFonts,
    pub language: DocxRunLanguage,
    pub is_complex_script: bool,
    pub is_rtl: bool,
}

pub fn generate_run_properties(script_runs: &[ScriptRun]) -> DocxRunProperties {
    let fonts = generate_rfonts(script_runs);
    let language = generate_lang(script_runs);

    // Check if any script is complex
    let is_complex_script = script_runs.iter().any(|run| {
        matches!(
            run.script,
            Script::Arabic
                | Script::Hebrew
                | Script::Devanagari
                | Script::Bengali
                | Script::Tamil
                | Script::Telugu
                | Script::Kannada
                | Script::Malayalam
                | Script::Thai
                | Script::Lao
                | Script::Myanmar
                | Script::Khmer
                | Script::Balinese
        )
    });

    // Check if any script is RTL
    let is_rtl = script_runs.iter().any(|run| {
        matches!(run.script, Script::Arabic | Script::Hebrew)
    });

    DocxRunProperties {
        fonts,
        language,
        is_complex_script,
        is_rtl,
    }
}

pub fn rpr_to_xml(rpr: &DocxRunProperties) -> String {
    let mut xml = String::from("<w:rPr>");

    // Add fonts
    xml.push_str(&rfonts_to_xml(&rpr.fonts));

    // Add language
    xml.push_str(&lang_to_xml(&rpr.language));

    // Add complex script flag
    if rpr.is_complex_script {
        xml.push_str("<w:cs/>");
    }

    // Add RTL flag
    if rpr.is_rtl {
        xml.push_str("<w:rtl/>");
    }

    xml.push_str("</w:rPr>");
    xml
}
```

**Example Output:**

```xml
<w:rPr>
  <w:rFonts w:ascii="Calibri" w:cs="Arial" w:eastAsia="SimSun"/>
  <w:lang w:val="en-US" w:eastAsia="zh-CN" w:bidi="ar-SA"/>
  <w:cs/>
  <w:rtl/>
</w:rPr>
```

## Step 7: Create Font Table

```rust
pub struct FontTableEntry {
    pub name: String,
    pub panose: String,
    pub charset: String,
    pub family: String,
    pub pitch: String,
    pub usb0: String,
    pub usb1: String,
    pub usb2: String,
    pub usb3: String,
}

pub fn generate_font_table(fonts: &[&str]) -> String {
    let mut xml = String::from(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:fontTable xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">"#,
    );

    for font_name in fonts {
        let entry = get_font_entry(font_name);
        xml.push_str(&format!(
            r#"
  <w:font w:name="{}">
    <w:panose1 w:val="{}"/>
    <w:charset w:val="{}"/>
    <w:family w:val="{}"/>
    <w:pitch w:val="{}"/>
    <w:sig w:usb0="{}" w:usb1="{}" w:usb2="{}" w:usb3="{}"
           w:csb0="00000001" w:csb1="00000000"/>
  </w:font>"#,
            font_name, entry.panose, entry.charset, entry.family, entry.pitch,
            entry.usb0, entry.usb1, entry.usb2, entry.usb3
        ));
    }

    xml.push_str("\n</w:fontTable>");
    xml
}

fn get_font_entry(font_name: &str) -> FontTableEntry {
    match font_name {
        "Calibri" => FontTableEntry {
            name: "Calibri".to_string(),
            panose: "020B0604030504040204".to_string(),
            charset: "00".to_string(),
            family: "swiss".to_string(),
            pitch: "variable".to_string(),
            usb0: "E0002AFF".to_string(),
            usb1: "C0000000".to_string(),
            usb2: "00000000".to_string(),
            usb3: "00000000".to_string(),
        },
        "Arial" => FontTableEntry {
            name: "Arial".to_string(),
            panose: "020B0604020202020204".to_string(),
            charset: "00".to_string(),
            family: "swiss".to_string(),
            pitch: "variable".to_string(),
            usb0: "E0002AFF".to_string(),
            usb1: "C0000000".to_string(),
            usb2: "00000000".to_string(),
            usb3: "00000000".to_string(),
        },
        "Noto Sans Balinese" => FontTableEntry {
            name: "Noto Sans Balinese".to_string(),
            panose: "020B0604030504040204".to_string(),
            charset: "00".to_string(),
            family: "swiss".to_string(),
            pitch: "variable".to_string(),
            usb0: "00000000".to_string(),
            usb1: "00000000".to_string(),
            usb2: "00000000".to_string(),
            usb3: "00000010".to_string(),  // Balinese block
        },
        "SimSun" => FontTableEntry {
            name: "SimSun".to_string(),
            panose: "020B0600000000000000".to_string(),
            charset: "86".to_string(),
            family: "roman".to_string(),
            pitch: "fixed".to_string(),
            usb0: "00000001".to_string(),
            usb1: "28000000".to_string(),
            usb2: "00000000".to_string(),
            usb3: "00000000".to_string(),
        },
        _ => FontTableEntry {
            name: font_name.to_string(),
            panose: "020B0604030504040204".to_string(),
            charset: "00".to_string(),
            family: "swiss".to_string(),
            pitch: "variable".to_string(),
            usb0: "E0002AFF".to_string(),
            usb1: "C0000000".to_string(),
            usb2: "00000000".to_string(),
            usb3: "00000000".to_string(),
        },
    }
}
```

## Step 8: Embed Fonts

```rust
use uuid::Uuid;

pub fn embed_fonts_in_docx(
    docx: &mut DocxDocument,
    fonts: &[&str],
) -> Result<()> {
    for font_name in fonts {
        // Load font
        let font_data = load_font(font_name)?;

        // Check embedding permission
        match check_embedding_permission(&font_data)? {
            EmbeddingPermission::Restricted => {
                // Skip embedding
                continue;
            }
            _ => {}
        }

        // Subset font (collect used glyphs first)
        let used_glyphs = collect_used_glyphs_from_document(docx);
        let subset_data = subset_font(&font_data, &used_glyphs)?;

        // Obfuscate
        let guid = Uuid::new_v4();
        let obfuscated = obfuscate_font(&subset_data, &guid);

        // Add to DOCX
        docx.add_embedded_font(font_name, &obfuscated, &guid)?;
    }

    Ok(())
}

fn obfuscate_font(font_data: &[u8], guid: &Uuid) -> Vec<u8> {
    let mut obfuscated = font_data.to_vec();
    let guid_bytes = guid.as_bytes();

    // XOR first 32 bytes
    for i in 0..32.min(obfuscated.len()) {
        obfuscated[i] ^= guid_bytes[i % 16];
    }

    obfuscated
}
```

## Complete Example

```rust
pub fn generate_docx_paragraph(text: &str) -> String {
    // Step 1: BiDi reordering
    let bidi = BidiInfo::new(text, Some(Level::ltr()));
    let visual_text = bidi.reorder_visual();

    // Step 2: Detect script runs
    let script_runs = detect_script_runs(&visual_text);

    // Step 3: Generate run properties
    let rpr = generate_run_properties(&script_runs);

    // Step 4: Create DOCX paragraph
    let mut xml = String::from("<w:p>");
    xml.push_str(&rpr_to_xml(&rpr));
    xml.push_str(&format!("<w:t>{}</w:t>", escape_xml(text)));
    xml.push_str("</w:p>");

    xml
}
```

## References

- `docs/ooxml/font-embedding.md` — Detailed OOXML font specifications
- ECMA-376-1:2016 — Office Open XML File Formats (Part 1)
