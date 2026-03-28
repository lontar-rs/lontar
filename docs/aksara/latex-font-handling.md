# LaTeX Font Handling

## Overview

This document details how to integrate text shaping results into LaTeX documents, focusing on fontspec package integration, script-specific font selection, and HarfBuzz shaping in XeLaTeX/LuaLaTeX.

## LaTeX Engine Selection

| Engine | Font Format | Shaping | BiDi | Script Support |
|--------|-------------|---------|------|---|
| pdfLaTeX | Type 1 | None | None | Latin only |
| XeLaTeX | TrueType/OpenType | HarfBuzz | ✅ | All scripts |
| LuaLaTeX | TrueType/OpenType | HarfBuzz | ✅ | All scripts |

**Recommendation for Lontar:** Use XeLaTeX or LuaLaTeX with fontspec package.

## Integration Pipeline

```
1. Input text: "Hello مرحبا 你好 ᬮᭀᬦ᭄ᬢᬭ᭄"
2. BiDi reordering: "Hello ابحرم 你好 ᬮᭀᬦ᭄ᬢᬭ᭄"
3. Script detection: [Latin, Arabic, Han, Balinese]
4. Font selection: Select font for each script
5. LaTeX generation: Create fontspec commands
6. Compilation: XeLaTeX/LuaLaTeX handles shaping
```

## Step 1: Detect Script Runs

```rust
use unicode_script::Script;

pub fn detect_script_runs(text: &str) -> Vec<ScriptRun> {
    // Same as DOCX/PDF
    // See unicode-script-evaluation.md
}
```

## Step 2: Select Fonts

```rust
pub fn select_font_for_script(script: Script) -> &'static str {
    match script {
        Script::Latin => "Calibri",
        Script::Arabic => "Noto Sans Arabic",
        Script::Hebrew => "Noto Sans Hebrew",
        Script::Devanagari => "Noto Sans Devanagari",
        Script::Bengali => "Noto Sans Bengali",
        Script::Tamil => "Noto Sans Tamil",
        Script::Telugu => "Noto Sans Telugu",
        Script::Kannada => "Noto Sans Kannada",
        Script::Malayalam => "Noto Sans Malayalam",
        Script::Thai => "Noto Sans Thai",
        Script::Lao => "Noto Sans Lao",
        Script::Khmer => "Noto Sans Khmer",
        Script::Myanmar => "Noto Sans Myanmar",
        Script::Han => "Noto Sans CJK SC",
        Script::Hiragana | Script::Katakana => "Noto Sans CJK JP",
        Script::Hangul => "Noto Sans CJK KR",
        Script::Balinese => "Noto Sans Balinese",
        _ => "Calibri",
    }
}
```

## Step 3: Generate fontspec Commands

### Basic Font Setup

```latex
\documentclass{article}
\usepackage{fontspec}
\usepackage{polyglossia}

% Set main font
\setmainfont{Calibri}

% Set fonts for specific scripts
\newfontfamily\arabicfont{Noto Sans Arabic}
\newfontfamily\devanagariFont{Noto Sans Devanagari}
\newfontfamily\cjkfont{Noto Sans CJK SC}
\newfontfamily\balinesesfont{Noto Sans Balinese}

\begin{document}

% Latin text (uses main font)
Hello World

% Arabic text
{\arabicfont مرحبا}

% Devanagari text
{\devanagariFont नमस्ते}

% CJK text
{\cjkfont 你好世界}

% Balinese text
{\balinesesfont ᬮᭀᬦ᭄ᬢᬭ᭄}

\end{document}
```

### Rust Code Generation

```rust
pub struct LatexFontSetup {
    pub main_font: String,
    pub font_families: HashMap<Script, String>,
}

pub fn generate_fontspec_preamble(scripts: &[Script]) -> String {
    let mut preamble = String::from(
        r#"\documentclass{article}
\usepackage{fontspec}
\usepackage{polyglossia}

"#,
    );

    // Set main font
    preamble.push_str(&format!(
        r#"\setmainfont{{{}}}"#,
        select_font_for_script(Script::Latin)
    ));
    preamble.push('\n');

    // Define font families for each script
    for script in scripts {
        let font_name = select_font_for_script(*script);
        let family_name = script_to_font_family_name(*script);

        preamble.push_str(&format!(
            r#"\newfontfamily\{}{{{}}}"#,
            family_name, font_name
        ));
        preamble.push('\n');
    }

    preamble
}

fn script_to_font_family_name(script: Script) -> String {
    match script {
        Script::Latin => "latinFont".to_string(),
        Script::Arabic => "arabicFont".to_string(),
        Script::Hebrew => "hebrewFont".to_string(),
        Script::Devanagari => "devanagariFont".to_string(),
        Script::Bengali => "bengaliFont".to_string(),
        Script::Tamil => "tamilFont".to_string(),
        Script::Telugu => "teluguFont".to_string(),
        Script::Kannada => "kannadaFont".to_string(),
        Script::Malayalam => "malayalamFont".to_string(),
        Script::Thai => "thaiFont".to_string(),
        Script::Lao => "laoFont".to_string(),
        Script::Khmer => "khmerFont".to_string(),
        Script::Myanmar => "myanmarFont".to_string(),
        Script::Han => "cjkFont".to_string(),
        Script::Hiragana | Script::Katakana => "japaneseFont".to_string(),
        Script::Hangul => "koreanFont".to_string(),
        Script::Balinese => "balineseFont".to_string(),
        _ => "defaultFont".to_string(),
    }
}
```

## Step 4: Generate Text with Font Switching

```rust
pub fn generate_latex_text(script_runs: &[ScriptRun]) -> String {
    let mut latex = String::new();

    for run in script_runs {
        let font_family = script_to_font_family_name(run.script);

        // Escape special LaTeX characters
        let escaped_text = escape_latex(&run.text);

        // Wrap in font family command
        latex.push_str(&format!(
            r#"{{\{}{}}}"#,
            font_family, escaped_text
        ));
    }

    latex
}

fn escape_latex(text: &str) -> String {
    text.replace('\\', "\\textbackslash{}")
        .replace('{', "\\{")
        .replace('}', "\\}")
        .replace('$', "\\$")
        .replace('&', "\\&")
        .replace('#', "\\#")
        .replace('^', "\\^{}")
        .replace('_', "\\_")
        .replace('~', "\\textasciitilde{}")
        .replace('%', "\\%")
}
```

**Example Output:**

```latex
{\latinFont Hello }{\arabicFont مرحبا}{\latinFont  }{\cjkFont 你好}{\balineseFont ᬮᭀᬦ᭄ᬢᬭ᭄}
```

## Step 5: Handle BiDi Text

```rust
pub fn generate_latex_bidi_text(script_runs: &[ScriptRun]) -> String {
    let mut latex = String::new();

    for run in script_runs {
        let font_family = script_to_font_family_name(run.script);
        let escaped_text = escape_latex(&run.text);

        // Use \textlr or \textrl based on direction
        let direction = if is_rtl_script(run.script) {
            "rl"
        } else {
            "lr"
        };

        latex.push_str(&format!(
            r#"\text{{{}}}{{\{}{}}}"#,
            direction, font_family, escaped_text
        ));
    }

    latex
}

fn is_rtl_script(script: Script) -> bool {
    matches!(script, Script::Arabic | Script::Hebrew)
}
```

**Example Output:**

```latex
\textlr{\latinFont Hello }\textrl{\arabicFont مرحبا}\textlr{\latinFont  }\textlr{\cjkFont 你好}
```

## Step 6: Handle Complex Scripts

For complex scripts, XeLaTeX/LuaLaTeX automatically apply shaping via HarfBuzz:

```rust
pub fn generate_latex_complex_script_text(script_runs: &[ScriptRun]) -> String {
    let mut latex = String::new();

    for run in script_runs {
        let font_family = script_to_font_family_name(run.script);
        let escaped_text = escape_latex(&run.text);

        // For complex scripts, add shaping hints
        if requires_shaping(run.script) {
            // XeLaTeX/LuaLaTeX will automatically apply HarfBuzz shaping
            latex.push_str(&format!(
                r#"{{\{}{}}}"#,
                font_family, escaped_text
            ));
        } else {
            latex.push_str(&format!(
                r#"{{\{}{}}}"#,
                font_family, escaped_text
            ));
        }
    }

    latex
}

fn requires_shaping(script: Script) -> bool {
    matches!(
        script,
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
}
```

## Step 7: Language Tags

```rust
pub fn generate_latex_language_tags(script_runs: &[ScriptRun]) -> String {
    let mut latex = String::new();

    for run in script_runs {
        let lang = script_to_language_tag(run.script);
        let font_family = script_to_font_family_name(run.script);
        let escaped_text = escape_latex(&run.text);

        // Use polyglossia for language-specific rules
        latex.push_str(&format!(
            r#"\text{{{}}}{{\{}{}}}"#,
            lang, font_family, escaped_text
        ));
    }

    latex
}

fn script_to_language_tag(script: Script) -> &'static str {
    match script {
        Script::Latin => "english",
        Script::Arabic => "arabic",
        Script::Hebrew => "hebrew",
        Script::Devanagari => "hindi",
        Script::Bengali => "bengali",
        Script::Tamil => "tamil",
        Script::Telugu => "telugu",
        Script::Kannada => "kannada",
        Script::Malayalam => "malayalam",
        Script::Thai => "thai",
        Script::Lao => "lao",
        Script::Khmer => "khmer",
        Script::Myanmar => "burmese",
        Script::Han => "chinese",
        Script::Hiragana | Script::Katakana => "japanese",
        Script::Hangul => "korean",
        Script::Balinese => "english",  // No specific Balinese language in polyglossia
        _ => "english",
    }
}
```

## Step 8: OpenType Features

```rust
pub fn generate_latex_with_features(script_runs: &[ScriptRun]) -> String {
    let mut latex = String::new();

    for run in script_runs {
        let font_family = script_to_font_family_name(run.script);
        let escaped_text = escape_latex(&run.text);

        // Add OpenType features for specific scripts
        let features = get_opentype_features(run.script);

        if !features.is_empty() {
            latex.push_str(&format!(
                r#"{{\{}{{{}}}{}}}"#,
                font_family, features, escaped_text
            ));
        } else {
            latex.push_str(&format!(
                r#"{{\{}{}}}"#,
                font_family, escaped_text
            ));
        }
    }

    latex
}

fn get_opentype_features(script: Script) -> String {
    match script {
        Script::Arabic => "[Script=Arabic, Language=Default]".to_string(),
        Script::Devanagari => "[Script=Deva, Language=Default]".to_string(),
        Script::Bengali => "[Script=Beng, Language=Default]".to_string(),
        Script::Tamil => "[Script=Taml, Language=Default]".to_string(),
        Script::Thai => "[Script=Thai, Language=Default]".to_string(),
        Script::Han => "[Script=Han, Language=Chinese Simplified]".to_string(),
        Script::Balinese => "[Script=Bali, Language=Default]".to_string(),
        _ => String::new(),
    }
}
```

## Complete Example

```rust
pub fn generate_latex_document(
    title: &str,
    content: &str,
) -> Result<String> {
    // Step 1: BiDi reordering
    let bidi = BidiInfo::new(content, Some(Level::ltr()));
    let visual_text = bidi.reorder_visual();

    // Step 2: Detect script runs
    let script_runs = detect_script_runs(&visual_text);

    // Step 3: Collect unique scripts
    let scripts: Vec<Script> = script_runs
        .iter()
        .map(|r| r.script)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    // Step 4: Generate preamble
    let mut latex = generate_fontspec_preamble(&scripts);
    latex.push_str("\n\\begin{document}\n\n");

    // Step 5: Add title
    latex.push_str(&format!(r#"\title{{{}}}"#, escape_latex(title)));
    latex.push_str("\n\\maketitle\n\n");

    // Step 6: Add content
    latex.push_str(&generate_latex_bidi_text(&script_runs));

    // Step 7: Close document
    latex.push_str("\n\n\\end{document}\n");

    Ok(latex)
}
```

**Example Output:**

```latex
\documentclass{article}
\usepackage{fontspec}
\usepackage{polyglossia}

\setmainfont{Calibri}
\newfontfamily\latinFont{Calibri}
\newfontfamily\arabicFont{Noto Sans Arabic}
\newfontfamily\cjkFont{Noto Sans CJK SC}
\newfontfamily\balineseFont{Noto Sans Balinese}

\begin{document}

\title{Multi-Script Document}
\maketitle

\textlr{\latinFont Hello }\textrl{\arabicFont مرحبا}\textlr{\latinFont  }\textlr{\cjkFont 你好}\textlr{\balineseFont ᬮᭀᬦ᭄ᬢᬭ᭄}

\end{document}
```

## Compilation

```bash
# Compile with XeLaTeX
xelatex document.tex

# Or with LuaLaTeX
lualatex document.tex

# With bibliography support
xelatex document.tex
biber document
xelatex document.tex
```

## Key Considerations

### 1. Font Installation

Fonts must be installed on the system or specified with full paths:

```latex
\setmainfont[Path=/path/to/fonts/]{Calibri.ttf}
```

### 2. HarfBuzz Shaping

XeLaTeX/LuaLaTeX automatically use HarfBuzz for text shaping. No manual shaping needed.

### 3. BiDi Support

Use `\textlr{}` and `\textrl{}` for explicit direction control:

```latex
\textlr{English text}
\textrl{النص العربي}
```

### 4. Language-Specific Rules

Use `polyglossia` for language-specific hyphenation and typography:

```latex
\usepackage{polyglossia}
\setdefaultlanguage{english}
\setotherlanguage{arabic}
```

### 5. Bibliography with Citations

```latex
\usepackage{biblatex}
\addbibresource{references.bib}

% In document
\cite{key}

% At end
\printbibliography
```

## Performance Characteristics

| Operation | Time | Notes |
|-----------|------|-------|
| Font loading | ~100-500ms | First compilation |
| Shaping | ~50-200ms | Per page |
| Compilation | ~1-3s | Total time |

## References

- [fontspec Documentation](https://ctan.org/pkg/fontspec)
- [polyglossia Documentation](https://ctan.org/pkg/polyglossia)
- [XeLaTeX Documentation](https://tug.org/xetex/)
- [LuaLaTeX Documentation](https://www.luatex.org/)
- [HarfBuzz in XeLaTeX](https://tug.org/xetex/xetex-faq.html)
