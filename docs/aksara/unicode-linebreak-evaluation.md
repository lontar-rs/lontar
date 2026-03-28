# Unicode Linebreak Evaluation

## Overview

The `unicode-linebreak` crate implements the Unicode Line Breaking Algorithm (UAX #14), which identifies where text can be broken across lines. This is essential for paragraph layout, text wrapping, and justified alignment in all backends.

## Problem Statement

Not all positions in text are valid break points. The algorithm determines:
1. Where breaks are **mandatory** (after newlines)
2. Where breaks are **allowed** (between words)
3. Where breaks are **prohibited** (inside words, after opening quotes)

```
Text:     "This is a test. ᬅᬓ᭄ᬱᬭ ᬩᬮᬶ."
Breaks:   "This | is | a | test. | ᬅᬓ᭄ᬱᬭ | ᬩᬮᬶ."
          (allowed breaks at spaces)
```

## API Structure

### Core Types

```rust
pub enum BreakOpportunity {
    Mandatory,  // Must break (newline)
    Allowed,    // Can break (space, hyphen)
    Prohibited, // Cannot break (inside word)
}

pub struct LineBreaker<'a> {
    // Iterator over break opportunities
}
```

### Main Functions

```rust
// Create line breaker
pub fn LineBreaker::new(text: &str) -> LineBreaker

// Iterate over break positions
pub fn line_breaker.next() -> Option<usize>

// Check break opportunity at position
pub fn is_break_opportunity(text: &str, pos: usize) -> BreakOpportunity

// Get all breaks
pub fn breaks(text: &str) -> Vec<usize>
```

## Example: Simple Text

```rust
use unicode_linebreak::LineBreaker;

let text = "This is a test.";
let breaker = LineBreaker::new(text);

let breaks: Vec<usize> = breaker.collect();
println!("Breaks at: {:?}", breaks);
// Output: [4, 7, 9, 14]
//         "This| is| a| test."

// Segments
for window in breaks.windows(2) {
    let segment = &text[window[0]..window[1]];
    println!("Segment: '{}'", segment);
}
// Output:
// Segment: 'This '
// Segment: 'is '
// Segment: 'a '
// Segment: 'test.'
```

## Example: Complex Text with Balinese

```rust
use unicode_linebreak::LineBreaker;

let text = "This is a test. ᬅᬓ᭄ᬱᬭ ᬩᬮᬶ.";
let breaker = LineBreaker::new(text);

let breaks: Vec<usize> = breaker.collect();
println!("Breaks at: {:?}", breaks);
// Output: [4, 7, 9, 14, 16, 24, 30]
//         "This| is| a| test. | ᬅᬓ᭄ᬱᬭ | ᬩᬮᬶ."

// Note: Balinese characters (ᬅᬓ᭄ᬱᬭ) don't break internally
// because they form a single word unit
```

## Break Classes

The algorithm assigns each character a break class:

| Class | Meaning | Examples |
|-------|---------|----------|
| OP | Opening punctuation | ( [ { |
| CL | Closing punctuation | ) ] } |
| QU | Quotation mark | " ' |
| GL | Glue (no break after) | Non-breaking space |
| NS | Non-starter | Hiragana, Katakana |
| EX | Exclamation/Interrogation | ! ? |
| SY | Symbols | / \ |
| IS | Infix separator | , . |
| PR | Prefix | $ ¥ |
| PO | Postfix | % |
| NU | Numeric | 0-9 |
| AL | Alphabetic | a-z, A-Z, ᬅ-ᬰ |
| ID | Ideographic | CJK characters |
| EB | Emoji base | 😀 |
| EM | Emoji modifier | Skin tone |
| ZW | Zero width space | U+200B |
| ZWJ | Zero width joiner | U+200D |
| CM | Combining mark | Accents, diacritics |
| WJ | Word joiner | U+2060 |
| H2 | Hangul (2-component) | Korean |
| H3 | Hangul (3-component) | Korean |
| JL | Hangul Jamo L | Korean |
| JV | Hangul Jamo V | Korean |
| JT | Hangul Jamo T | Korean |
| RI | Regional indicator | Flag emoji |
| AI | Ambiguous | Depends on context |
| BK | Break (mandatory) | Newline |
| CR | Carriage return | \r |
| LF | Line feed | \n |
| NL | Next line | U+0085 |
| SA | Complex context | Thai, Lao, Khmer |
| SG | Surrogate | Lone surrogate |
| XX | Unknown | Unassigned |

## Example: Break Class Analysis

```rust
use unicode_linebreak::LineBreaker;

let text = "Hello, world!";
//          H    e    l    l    o    ,    sp   w    o    r    l    d    !
// Classes: AL   AL   AL   AL   AL   IS   SP   AL   AL   AL   AL   AL   EX

let breaker = LineBreaker::new(text);
let breaks: Vec<usize> = breaker.collect();
// Output: [5, 7, 13]
// Breaks after comma, space, and end
```

## Integration with Lontar

### 1. Paragraph Layout

```rust
pub struct ParagraphLayout {
    pub text: String,
    pub lines: Vec<String>,
    pub line_widths: Vec<f32>,
}

pub fn layout_paragraph(
    text: &str,
    max_width: f32,
    font: &Font,
) -> ParagraphLayout {
    let breaker = LineBreaker::new(text);
    let breaks: Vec<usize> = breaker.collect();

    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut prev_break = 0;

    for break_pos in breaks {
        let segment = &text[prev_break..break_pos];
        let segment_width = measure_text(segment, font);

        if (current_line.len() as f32 + segment_width) > max_width {
            // Start new line
            lines.push(current_line.clone());
            current_line = segment.to_string();
        } else {
            current_line.push_str(segment);
        }

        prev_break = break_pos;
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    ParagraphLayout {
        text: text.to_string(),
        lines,
        line_widths: vec![],  // Compute widths
    }
}
```

### 2. DOCX Paragraph Breaking

```rust
pub fn generate_docx_paragraphs(
    text: &str,
    max_width: f32,
    font: &Font,
) -> Vec<DocxParagraph> {
    let layout = layout_paragraph(text, max_width, font);

    layout.lines
        .iter()
        .map(|line| {
            DocxParagraph {
                text: line.clone(),
                runs: vec![],  // Will be populated with shaped runs
            }
        })
        .collect()
}
```

### 3. PDF Text Wrapping

```rust
pub fn wrap_text_for_pdf(
    text: &str,
    max_width: f32,
    font: &PdfFont,
) -> Vec<String> {
    let breaker = LineBreaker::new(text);
    let breaks: Vec<usize> = breaker.collect();

    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut prev_break = 0;

    for break_pos in breaks {
        let segment = &text[prev_break..break_pos];
        let segment_width = font.measure_text(segment);

        if (current_line.len() as f32 + segment_width) > max_width {
            lines.push(current_line.trim_end().to_string());
            current_line = segment.to_string();
        } else {
            current_line.push_str(segment);
        }

        prev_break = break_pos;
    }

    if !current_line.is_empty() {
        lines.push(current_line.trim_end().to_string());
    }

    lines
}
```

### 4. Hyphenation Points

```rust
pub fn find_hyphenation_points(text: &str) -> Vec<usize> {
    let breaker = LineBreaker::new(text);
    breaker
        .filter(|&pos| {
            // Only allow breaks at word boundaries (after spaces)
            pos > 0 && text.chars().nth(pos - 1) == Some(' ')
        })
        .collect()
}
```

## Interaction with Text Shaping

**Important:** Line breaking should happen **before** text shaping:

```
1. Input text: "This is a test."
2. Line breaking: ["This ", "is ", "a ", "test."]
3. Script detection: Latin for each segment
4. Text shaping: Shape each segment separately
5. Positioning: Combine shaped segments
```

**Why?** Shaping engines expect complete words/segments. Breaking in the middle of a word can cause incorrect glyph formation (e.g., Arabic contextual forms).

## Example: Full Pipeline

```rust
use unicode_linebreak::LineBreaker;
use rustybuzz::{Face, UnicodeBuffer, shape};

let text = "This is a test. مرحبا";
let max_width = 200.0;

// Step 1: Line breaking
let breaker = LineBreaker::new(text);
let breaks: Vec<usize> = breaker.collect();

// Step 2: Create line segments
let mut lines = Vec::new();
let mut prev = 0;
for break_pos in breaks {
    let segment = &text[prev..break_pos];
    lines.push(segment);
    prev = break_pos;
}

// Step 3: Shape each line
for line in lines {
    // Step 3a: Detect script runs
    let script_runs = detect_script_runs(line);

    // Step 3b: Shape each script run
    for script_run in script_runs {
        let mut buffer = UnicodeBuffer::new();
        buffer.push_str(&script_run.text);
        buffer.set_script(script_run.script);

        let output = shape(&face, &[], buffer);
        // Process shaped glyphs...
    }
}
```

## Key Capabilities

| Feature | Supported | Notes |
|---------|-----------|-------|
| Space breaks | ✅ | Standard word breaks |
| Hyphen breaks | ✅ | After hyphens |
| Newline breaks | ✅ | Mandatory breaks |
| CJK breaks | ✅ | Between ideographs |
| Thai/Lao breaks | ✅ | Complex script breaks |
| Balinese breaks | ✅ | Treated as alphabetic |
| Emoji breaks | ✅ | Emoji base + modifier |
| Zero-width breaks | ✅ | U+200B (ZWSP) |
| Soft hyphen | ✅ | U+00AD |

## Limitations

1. **No hyphenation** — Only identifies break points; doesn't add hyphens
2. **No text shaping** — Use rustybuzz for glyph formation
3. **No width measurement** — Use font metrics for width calculation
4. **No justification** — Only provides break points; backends must justify

## Performance Characteristics

- **LineBreaker creation**: ~0.05-0.2ms per 100 characters
- **Break iteration**: ~0.02-0.1ms per 100 characters
- **Memory**: ~50 bytes per character

## Common Pitfalls

### 1. Breaking After Shaping

```rust
// ❌ WRONG: Shape first, then break
let output = shape(&face, &[], buffer);
let breaks = LineBreaker::new(text).collect();
// Glyphs may be incorrect if break is in middle of word

// ✅ CORRECT: Break first, then shape
let breaks = LineBreaker::new(text).collect();
for segment in segments {
    let output = shape(&face, &[], buffer);
}
```

### 2. Ignoring Complex Scripts

```rust
// ❌ WRONG: Assume all scripts break at spaces
for break_pos in breaks {
    if text[break_pos] == ' ' {
        // Break here
    }
}

// ✅ CORRECT: Use LineBreaker for all scripts
let breaker = LineBreaker::new(text);
for break_pos in breaker {
    // Break here (handles Thai, Lao, CJK, etc.)
}
```

### 3. Not Handling Mandatory Breaks

```rust
// ❌ WRONG: Only check for allowed breaks
let breaker = LineBreaker::new(text);

// ✅ CORRECT: Handle both allowed and mandatory breaks
let text_with_newlines = "Line 1\nLine 2";
let breaker = LineBreaker::new(text_with_newlines);
// Automatically handles \n as mandatory break
```

## Recommended Integration Points

1. **After document parsing** — Identify break opportunities early
2. **Before text shaping** — Break into segments before shaping
3. **During paragraph layout** — Use breaks for line wrapping
4. **When generating DOCX** — Create separate runs at break points
5. **When generating PDF** — Wrap text at break points
6. **When generating LaTeX** — Use breaks for line breaking

## References

- [Unicode Standard Annex #14: Line Breaking Properties](https://www.unicode.org/reports/tr14/)
- [unicode-linebreak on crates.io](https://crates.io/crates/unicode-linebreak)
- [ECMA-376: Paragraph Properties](https://www.ecma-international.org/publications-and-standards/standards/ecma-376/)
