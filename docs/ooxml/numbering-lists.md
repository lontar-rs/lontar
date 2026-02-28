# OOXML List Numbering System in WordprocessingML (DOCX)

## Overview

The OOXML numbering system in DOCX uses a two-tier architecture: abstract numbering definitions that define the structure and formatting of lists, and numbering instances that reference those definitions. This separation allows multiple lists in a document to share the same formatting without duplication.

This document covers the complete numbering system, from abstract definitions through document references, with copy-paste-ready XML examples for common list types.

---

## 1. Architecture Overview

### Two-Tier System

The OOXML numbering system consists of two components:

1. **Abstract Numbering Definitions** (`word/numbering.xml`, `w:abstractNum`) — Define the structure, formatting, and appearance of list levels (0-8, nine levels total)
2. **Numbering Instances** (`word/numbering.xml`, `w:num`) — Reference abstract definitions and can override specific levels

**Why this separation?**
- Multiple lists can share the same formatting (e.g., three numbered lists all using the same "1. 2. 3." format)
- Changing an abstract definition updates all lists that reference it
- Reduces file size by avoiding duplication

### File Structure of word/numbering.xml

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:numbering xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
             xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml"
             xmlns:w15="http://schemas.microsoft.com/office/word/2012/wordml">
  
  <!-- Abstract numbering definitions -->
  <w:abstractNum w:abstractNumId="0">
    <!-- Level definitions (w:lvl) for levels 0-8 -->
  </w:abstractNum>
  
  <w:abstractNum w:abstractNumId="1">
    <!-- Another abstract definition -->
  </w:abstractNum>
  
  <!-- Numbering instances -->
  <w:num w:numId="1">
    <w:abstractNumId w:val="0"/>
  </w:num>
  
  <w:num w:numId="2">
    <w:abstractNumId w:val="1"/>
  </w:num>
  
</w:numbering>
```

**Key points:**
- `w:abstractNumId` — unique identifier for abstract definitions (0, 1, 2, ...)
- `w:numId` — unique identifier for numbering instances (1, 2, 3, ...) — note: starts at 1, not 0
- Multiple `w:num` elements can reference the same `w:abstractNum`

### Relationship Entry in word/_rels/document.xml.rels

The document must reference the numbering.xml file:

```xml
<Relationship Id="rId5"
             Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering"
             Target="numbering.xml"/>
```

The `Id` value (e.g., "rId5") is arbitrary but must be unique within the document.

### Content Type Entry in [Content_Types].xml

```xml
<Override PartName="/word/numbering.xml"
         ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml"/>
```

---

## 2. Abstract Numbering Definitions (w:abstractNum)

### Structure

```xml
<w:abstractNum w:abstractNumId="0">
  <!-- Multilevel list definition -->
  
  <!-- Level 0 (top level) -->
  <w:lvl w:ilvl="0">
    <w:start w:val="1"/>
    <w:numFmt w:val="decimal"/>
    <w:lvlText w:val="%1."/>
    <w:lvlJc w:val="left"/>
    <w:pPr>
      <w:ind w:left="720" w:hanging="360"/>
    </w:pPr>
    <w:rPr>
      <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/>
    </w:rPr>
  </w:lvl>
  
  <!-- Level 1 (second level) -->
  <w:lvl w:ilvl="1">
    <w:start w:val="1"/>
    <w:numFmt w:val="lowerLetter"/>
    <w:lvlText w:val="%2."/>
    <w:lvlJc w:val="left"/>
    <w:pPr>
      <w:ind w:left="1440" w:hanging="360"/>
    </w:pPr>
    <w:rPr>
      <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/>
    </w:rPr>
  </w:lvl>
  
  <!-- Levels 2-8 follow the same pattern -->
  
</w:abstractNum>
```

### Level Elements (w:lvl)

Each `w:lvl` element defines the formatting for a specific indent level (0-8).

#### w:ilvl — Indent Level

```xml
<w:lvl w:ilvl="0">  <!-- Level 0 (top) -->
<w:lvl w:ilvl="1">  <!-- Level 1 (indented once) -->
<w:lvl w:ilvl="2">  <!-- Level 2 (indented twice) -->
```

Valid values: 0-8 (nine levels total).

#### w:start — Starting Number

```xml
<w:start w:val="1"/>  <!-- First item is numbered 1 -->
<w:start w:val="0"/>  <!-- First item is numbered 0 -->
<w:start w:val="10"/> <!-- First item is numbered 10 -->
```

#### w:numFmt — Number Format Type

| Format | Example | Notes |
|---|---|---|
| `decimal` | 1, 2, 3, ... | Arabic numerals |
| `lowerLetter` | a, b, c, ... | Lowercase letters |
| `upperLetter` | A, B, C, ... | Uppercase letters |
| `lowerRoman` | i, ii, iii, ... | Lowercase Roman numerals |
| `upperRoman` | I, II, III, ... | Uppercase Roman numerals |
| `bullet` | • | Bullet character |
| `none` | (no number) | No numbering |
| `ordinal` | 1st, 2nd, 3rd, ... | Ordinal numbers (English) |
| `cardinalText` | One, Two, Three, ... | Number as text |
| `decimalZero` | 01, 02, 03, ... | Zero-padded decimals |
| `hex` | 1, 2, 3, ... in hex | Hexadecimal |
| `chicago` | *, †, ‡, ... | Chicago-style footnote marks |

```xml
<w:numFmt w:val="decimal"/>
<w:numFmt w:val="lowerLetter"/>
<w:numFmt w:val="bullet"/>
```

#### w:lvlText — Display Template

The `w:lvlText` element defines how the number/bullet is displayed. Placeholders:
- `%1` — current level number
- `%2` — parent level number
- `%3` — grandparent level number
- etc.

```xml
<!-- Simple bullet -->
<w:lvlText w:val="•"/>

<!-- Numbered: "1." -->
<w:lvlText w:val="%1."/>

<!-- Numbered with parent: "1.1" -->
<w:lvlText w:val="%1.%2"/>

<!-- Three-level numbering: "1.1.1" -->
<w:lvlText w:val="%1.%2.%3"/>

<!-- Parentheses: "(1)" -->
<w:lvlText w:val="(%1)"/>

<!-- Dash: "– " -->
<w:lvlText w:val="– "/>

<!-- Arrow: "➤ " -->
<w:lvlText w:val="➤ "/>
```

#### w:lvlJc — Justification

```xml
<w:lvlJc w:val="left"/>    <!-- Left-aligned -->
<w:lvlJc w:val="center"/>  <!-- Centered -->
<w:lvlJc w:val="right"/>   <!-- Right-aligned -->
```

#### w:pPr — Paragraph Properties

Defines indentation and spacing for list items:

```xml
<w:pPr>
  <!-- Indentation -->
  <w:ind w:left="720"      <!-- Left indent in twips (1/20th point) -->
         w:hanging="360"/> <!-- Hanging indent (negative space for number) -->
  
  <!-- Optional: spacing -->
  <w:spacing w:before="0" w:after="0" w:line="240" w:lineRule="auto"/>
</w:pPr>
```

**Common indent values (in twips):**
- 720 twips = 0.5 inch = 1.27 cm (level 0)
- 1440 twips = 1.0 inch = 2.54 cm (level 1)
- 2160 twips = 1.5 inches = 3.81 cm (level 2)

**Hanging indent:** The `w:hanging` value creates space for the number/bullet. Typically 360 twips (0.25 inch).

#### w:rPr — Run Properties

Defines font and styling for the number/bullet character:

```xml
<w:rPr>
  <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri" w:cs="Calibri"/>
  <w:b/>  <!-- Bold -->
  <w:color w:val="FF0000"/>  <!-- Red -->
  <w:sz w:val="22"/>  <!-- Font size in half-points -->
</w:rPr>
```

### Complete Examples

#### Example 1: Simple Bullet List (1 Level)

```xml
<w:abstractNum w:abstractNumId="0">
  <w:lvl w:ilvl="0">
    <w:start w:val="1"/>
    <w:numFmt w:val="bullet"/>
    <w:lvlText w:val="•"/>
    <w:lvlJc w:val="left"/>
    <w:pPr>
      <w:ind w:left="720" w:hanging="360"/>
    </w:pPr>
    <w:rPr>
      <w:rFonts w:ascii="Symbol" w:hAnsi="Symbol"/>
    </w:rPr>
  </w:lvl>
  
  <!-- Levels 1-8 (unused, but required) -->
  <w:lvl w:ilvl="1">
    <w:start w:val="1"/>
    <w:numFmt w:val="bullet"/>
    <w:lvlText w:val="◦"/>
    <w:lvlJc w:val="left"/>
    <w:pPr>
      <w:ind w:left="1440" w:hanging="360"/>
    </w:pPr>
    <w:rPr>
      <w:rFonts w:ascii="Symbol" w:hAnsi="Symbol"/>
    </w:rPr>
  </w:lvl>
  
  <!-- Levels 2-8 omitted for brevity -->
</w:abstractNum>
```

#### Example 2: Simple Numbered List (1 Level)

```xml
<w:abstractNum w:abstractNumId="1">
  <w:lvl w:ilvl="0">
    <w:start w:val="1"/>
    <w:numFmt w:val="decimal"/>
    <w:lvlText w:val="%1."/>
    <w:lvlJc w:val="left"/>
    <w:pPr>
      <w:ind w:left="720" w:hanging="360"/>
    </w:pPr>
    <w:rPr>
      <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/>
    </w:rPr>
  </w:lvl>
  
  <!-- Levels 1-8 omitted -->
</w:abstractNum>
```

#### Example 3: Multi-Level Numbered List (1 → 1.1 → 1.1.1)

```xml
<w:abstractNum w:abstractNumId="2">
  <!-- Level 0: "1." -->
  <w:lvl w:ilvl="0">
    <w:start w:val="1"/>
    <w:numFmt w:val="decimal"/>
    <w:lvlText w:val="%1."/>
    <w:lvlJc w:val="left"/>
    <w:pPr>
      <w:ind w:left="720" w:hanging="360"/>
    </w:pPr>
    <w:rPr>
      <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/>
    </w:rPr>
  </w:lvl>
  
  <!-- Level 1: "1.1" -->
  <w:lvl w:ilvl="1">
    <w:start w:val="1"/>
    <w:numFmt w:val="decimal"/>
    <w:lvlText w:val="%1.%2."/>
    <w:lvlJc w:val="left"/>
    <w:pPr>
      <w:ind w:left="1440" w:hanging="360"/>
    </w:pPr>
    <w:rPr>
      <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/>
    </w:rPr>
  </w:lvl>
  
  <!-- Level 2: "1.1.1" -->
  <w:lvl w:ilvl="2">
    <w:start w:val="1"/>
    <w:numFmt w:val="decimal"/>
    <w:lvlText w:val="%1.%2.%3."/>
    <w:lvlJc w:val="left"/>
    <w:pPr>
      <w:ind w:left="2160" w:hanging="360"/>
    </w:pPr>
    <w:rPr>
      <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/>
    </w:rPr>
  </w:lvl>
  
  <!-- Levels 3-8 omitted -->
</w:abstractNum>
```

#### Example 4: Multi-Level Mixed List (Number → Bullet → Dash)

```xml
<w:abstractNum w:abstractNumId="3">
  <!-- Level 0: "1." (decimal) -->
  <w:lvl w:ilvl="0">
    <w:start w:val="1"/>
    <w:numFmt w:val="decimal"/>
    <w:lvlText w:val="%1."/>
    <w:lvlJc w:val="left"/>
    <w:pPr>
      <w:ind w:left="720" w:hanging="360"/>
    </w:pPr>
    <w:rPr>
      <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/>
    </w:rPr>
  </w:lvl>
  
  <!-- Level 1: "•" (bullet) -->
  <w:lvl w:ilvl="1">
    <w:start w:val="1"/>
    <w:numFmt w:val="bullet"/>
    <w:lvlText w:val="•"/>
    <w:lvlJc w:val="left"/>
    <w:pPr>
      <w:ind w:left="1440" w:hanging="360"/>
    </w:pPr>
    <w:rPr>
      <w:rFonts w:ascii="Symbol" w:hAnsi="Symbol"/>
    </w:rPr>
  </w:lvl>
  
  <!-- Level 2: "–" (dash) -->
  <w:lvl w:ilvl="2">
    <w:start w:val="1"/>
    <w:numFmt w:val="none"/>
    <w:lvlText w:val="–"/>
    <w:lvlJc w:val="left"/>
    <w:pPr>
      <w:ind w:left="2160" w:hanging="360"/>
    </w:pPr>
    <w:rPr>
      <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/>
    </w:rPr>
  </w:lvl>
  
  <!-- Levels 3-8 omitted -->
</w:abstractNum>
```

#### Example 5: Legal Numbering (Article 1, Section 1.1, Clause 1.1.1)

```xml
<w:abstractNum w:abstractNumId="4">
  <!-- Level 0: "Article 1." -->
  <w:lvl w:ilvl="0">
    <w:start w:val="1"/>
    <w:numFmt w:val="decimal"/>
    <w:lvlText w:val="Article %1."/>
    <w:lvlJc w:val="left"/>
    <w:pPr>
      <w:ind w:left="720" w:hanging="360"/>
    </w:pPr>
    <w:rPr>
      <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/>
      <w:b/>
    </w:rPr>
  </w:lvl>
  
  <!-- Level 1: "Section 1.1" -->
  <w:lvl w:ilvl="1">
    <w:start w:val="1"/>
    <w:numFmt w:val="decimal"/>
    <w:lvlText w:val="Section %1.%2"/>
    <w:lvlJc w:val="left"/>
    <w:pPr>
      <w:ind w:left="1440" w:hanging="360"/>
    </w:pPr>
    <w:rPr>
      <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/>
    </w:rPr>
  </w:lvl>
  
  <!-- Level 2: "Clause 1.1.1" -->
  <w:lvl w:ilvl="2">
    <w:start w:val="1"/>
    <w:numFmt w:val="decimal"/>
    <w:lvlText w:val="Clause %1.%2.%3"/>
    <w:lvlJc w:val="left"/>
    <w:pPr>
      <w:ind w:left="2160" w:hanging="360"/>
    </w:pPr>
    <w:rPr>
      <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/>
    </w:rPr>
  </w:lvl>
  
  <!-- Levels 3-8 omitted -->
</w:abstractNum>
```

---

## 3. Numbering Instances (w:num)

### Structure

```xml
<w:num w:numId="1">
  <w:abstractNumId w:val="0"/>
</w:num>

<w:num w:numId="2">
  <w:abstractNumId w:val="1"/>
</w:num>

<w:num w:numId="3">
  <w:abstractNumId w:val="0"/>
  <!-- Level overrides (optional) -->
  <w:lvlOverride w:ilvl="0">
    <w:startOverride w:val="5"/>
  </w:lvlOverride>
</w:num>
```

**Key points:**
- `w:numId` — unique identifier for this numbering instance (starts at 1, not 0)
- `w:abstractNumId w:val="X"` — references the abstract definition
- Multiple `w:num` elements can reference the same `w:abstractNum`

### Level Overrides (w:lvlOverride)

Level overrides allow you to customize a specific level for a particular numbering instance:

```xml
<w:num w:numId="3">
  <w:abstractNumId w:val="2"/>
  
  <!-- Override level 0 to start at 5 instead of 1 -->
  <w:lvlOverride w:ilvl="0">
    <w:startOverride w:val="5"/>
  </w:lvlOverride>
  
  <!-- Override level 1 to use uppercase letters instead of decimal -->
  <w:lvlOverride w:ilvl="1">
    <w:numFmt w:val="upperLetter"/>
    <w:lvlText w:val="%2)"/>
  </w:lvlOverride>
</w:num>
```

---

## 4. Referencing from document.xml

### Paragraph Numbering Reference

Paragraphs reference numbering via `w:numPr` (numbering properties):

```xml
<w:p>
  <w:pPr>
    <!-- Reference numbering instance 1, level 0 -->
    <w:numPr>
      <w:ilvl w:val="0"/>  <!-- Indent level (0-8) -->
      <w:numId w:val="1"/> <!-- Numbering instance ID -->
    </w:numPr>
  </w:pPr>
  <w:r>
    <w:t>First item</w:t>
  </w:r>
</w:p>

<w:p>
  <w:pPr>
    <!-- Same numbering instance, same level -->
    <w:numPr>
      <w:ilvl w:val="0"/>
      <w:numId w:val="1"/>
    </w:numPr>
  </w:pPr>
  <w:r>
    <w:t>Second item</w:t>
  </w:r>
</w:p>

<w:p>
  <w:pPr>
    <!-- Same numbering instance, level 1 (nested) -->
    <w:numPr>
      <w:ilvl w:val="1"/>
      <w:numId w:val="1"/>
    </w:numPr>
  </w:pPr>
  <w:r>
    <w:t>Nested item</w:t>
  </w:r>
</w:p>
```

### Indent Level (w:ilvl)

The `w:ilvl` value maps to the level definitions in the abstract numbering:

| w:ilvl | Maps To | Example |
|---|---|---|
| 0 | `w:lvl w:ilvl="0"` | "1." |
| 1 | `w:lvl w:ilvl="1"` | "1.1" |
| 2 | `w:lvl w:ilvl="2"` | "1.1.1" |

### Creating Nested Lists

To create nested lists, increment `w:ilvl`:

```xml
<!-- Top-level item: "1." -->
<w:p>
  <w:pPr>
    <w:numPr>
      <w:ilvl w:val="0"/>
      <w:numId w:val="1"/>
    </w:numPr>
  </w:pPr>
  <w:r>
    <w:t>First item</w:t>
  </w:r>
</w:p>

<!-- Nested item: "1.1" -->
<w:p>
  <w:pPr>
    <w:numPr>
      <w:ilvl w:val="1"/>
      <w:numId w:val="1"/>
    </w:numPr>
  </w:pPr>
  <w:r>
    <w:t>Nested item</w:t>
  </w:r>
</w:p>

<!-- Back to top-level: "2." -->
<w:p>
  <w:pPr>
    <w:numPr>
      <w:ilvl w:val="0"/>
      <w:numId w:val="1"/>
    </w:numPr>
  </w:pPr>
  <w:r>
    <w:t>Second item</w:t>
  </w:r>
</w:p>
```

---

## 5. Restart Behavior

### Continuous Numbering (Same Numbering Instance)

If you use the same `w:numId` for consecutive lists, they continue numbering:

```xml
<!-- List 1 -->
<w:p>
  <w:pPr>
    <w:numPr>
      <w:ilvl w:val="0"/>
      <w:numId w:val="1"/>
    </w:numPr>
  </w:pPr>
  <w:r><w:t>First item</w:t></w:r>
</w:p>

<w:p>
  <w:pPr>
    <w:numPr>
      <w:ilvl w:val="0"/>
      <w:numId w:val="1"/>
    </w:numPr>
  </w:pPr>
  <w:r><w:t>Second item</w:t></w:r>
</w:p>

<!-- Paragraph between lists (breaks numbering) -->
<w:p>
  <w:pPr/>
  <w:r><w:t>Some text here</w:t></w:r>
</w:p>

<!-- List 2 (continues from 3) -->
<w:p>
  <w:pPr>
    <w:numPr>
      <w:ilvl w:val="0"/>
      <w:numId w:val="1"/>
    </w:numPr>
  </w:pPr>
  <w:r><w:t>Third item</w:t></w:r>
</w:p>
```

Result: "1. 2. [text] 3."

### Restarting Numbering (Different Numbering Instance)

To restart numbering at 1, use a different `w:numId`:

```xml
<!-- List 1 -->
<w:p>
  <w:pPr>
    <w:numPr>
      <w:ilvl w:val="0"/>
      <w:numId w:val="1"/>
    </w:numPr>
  </w:pPr>
  <w:r><w:t>First item</w:t></w:r>
</w:p>

<w:p>
  <w:pPr>
    <w:numPr>
      <w:ilvl w:val="0"/>
      <w:numId w:val="1"/>
    </w:numPr>
  </w:pPr>
  <w:r><w:t>Second item</w:t></w:r>
</w:p>

<!-- Paragraph between lists -->
<w:p>
  <w:pPr/>
  <w:r><w:t>Some text here</w:t></w:r>
</w:p>

<!-- List 2 (restarts at 1 with different numId) -->
<w:p>
  <w:pPr>
    <w:numPr>
      <w:ilvl w:val="0"/>
      <w:numId w:val="2"/>  <!-- Different numId -->
    </w:numPr>
  </w:pPr>
  <w:r><w:t>First item</w:t></w:r>
</w:p>

<w:p>
  <w:pPr>
    <w:numPr>
      <w:ilvl w:val="0"/>
      <w:numId w:val="2"/>
    </w:numPr>
  </w:pPr>
  <w:r><w:t>Second item</w:t></w:r>
</w:p>
```

Result: "1. 2. [text] 1. 2."

### Using w:lvlOverride to Restart

Alternatively, use `w:lvlOverride` with `w:startOverride` to restart a specific level:

```xml
<!-- numbering.xml -->
<w:num w:numId="3">
  <w:abstractNumId w:val="2"/>
  <w:lvlOverride w:ilvl="0">
    <w:startOverride w:val="1"/>  <!-- Restart at 1 -->
  </w:lvlOverride>
</w:num>

<!-- document.xml -->
<w:p>
  <w:pPr>
    <w:numPr>
      <w:ilvl w:val="0"/>
      <w:numId w:val="3"/>
    </w:numPr>
  </w:pPr>
  <w:r><w:t>First item</w:t></w:r>
</w:p>
```

---

## 6. Common Bullet Characters

### Bullet Character Table

| Character | Unicode | Font | Notes |
|---|---|---|---|
| • | U+2022 | Symbol, Wingdings | Standard bullet |
| ○ | U+25CB | Symbol, Wingdings | Hollow bullet |
| ◆ | U+25C6 | Symbol, Wingdings | Diamond |
| ▪ | U+25AA | Symbol, Wingdings | Small square |
| ▫ | U+25AB | Symbol, Wingdings | Hollow square |
| ► | U+25BA | Wingdings | Right arrow |
| ➤ | U+27A4 | Wingdings | Right arrow variant |
| – | U+2013 | Calibri | En dash |
| — | U+2014 | Calibri | Em dash |
| ✓ | U+2713 | Wingdings | Check mark |
| ✗ | U+2717 | Wingdings | X mark |
| ★ | U+2605 | Wingdings | Filled star |
| ☆ | U+2606 | Wingdings | Hollow star |

### Font Selection

**Symbol font:**
- Contains many bullet and symbol characters
- Use for: •, ○, ◆, ▪, ▫

**Wingdings font:**
- Contains arrows, check marks, stars, and other symbols
- Use for: ➤, ✓, ✗, ★, ☆

**Standard fonts (Calibri, Arial, etc.):**
- Use for: –, —, and text-based bullets

### XML Examples

```xml
<!-- Standard bullet (•) -->
<w:lvl w:ilvl="0">
  <w:numFmt w:val="bullet"/>
  <w:lvlText w:val="•"/>
  <w:rPr>
    <w:rFonts w:ascii="Symbol" w:hAnsi="Symbol"/>
  </w:rPr>
</w:lvl>

<!-- Hollow bullet (○) -->
<w:lvl w:ilvl="1">
  <w:numFmt w:val="bullet"/>
  <w:lvlText w:val="○"/>
  <w:rPr>
    <w:rFonts w:ascii="Symbol" w:hAnsi="Symbol"/>
  </w:rPr>
</w:lvl>

<!-- Arrow (➤) -->
<w:lvl w:ilvl="2">
  <w:numFmt w:val="bullet"/>
  <w:lvlText w:val="➤"/>
  <w:rPr>
    <w:rFonts w:ascii="Wingdings" w:hAnsi="Wingdings"/>
  </w:rPr>
</w:lvl>

<!-- Dash (–) -->
<w:lvl w:ilvl="3">
  <w:numFmt w:val="none"/>
  <w:lvlText w:val="–"/>
  <w:rPr>
    <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/>
  </w:rPr>
</w:lvl>
```

---

## 7. Mapping to Lontar

### Lontar List Types

In Lontar's AST, lists are represented as:

```rust
pub enum Block {
    List {
        kind: ListKind,
        items: Vec<ListItem>,
    },
    // ...
}

pub enum ListKind {
    Ordered,    // Numbered list
    Unordered,  // Bullet list
}

pub struct ListItem {
    pub content: Vec<Block>,
    pub level: u8,  // 0-8 for nesting
}
```

### Mapping Strategy

| Lontar | OOXML | Notes |
|---|---|---|
| `Block::List { kind: Ordered, items }` | `w:abstractNum` with `w:numFmt="decimal"` | Create abstract definition if not exists |
| `Block::List { kind: Unordered, items }` | `w:abstractNum` with `w:numFmt="bullet"` | Create abstract definition if not exists |
| `ListItem { level: 0, ... }` | `w:numPr { w:ilvl="0", w:numId="X" }` | Top-level item |
| `ListItem { level: 1, ... }` | `w:numPr { w:ilvl="1", w:numId="X" }` | Nested item |
| Nested `Block::List` | Same `w:numId` with incremented `w:ilvl` | Reuse numbering instance |

### ID Management Strategy

**Abstract Numbering IDs (`w:abstractNumId`):**
- Start at 0
- Increment for each unique list style (ordered, unordered, custom)
- Reuse IDs for identical styles

**Numbering Instance IDs (`w:numId`):**
- Start at 1 (not 0)
- Increment for each list in the document
- Reuse IDs only if the list continues from a previous list

### Lontar Implementation Considerations

#### 1. List Immediately Following Another List

**Same type (continue numbering):**
```rust
// List 1: numbered
Block::List { kind: Ordered, items: [...] }

// List 2: numbered (continues)
Block::List { kind: Ordered, items: [...] }
```

Generate:
```xml
<!-- Both lists use same w:numId -->
<w:num w:numId="1">
  <w:abstractNumId w:val="0"/>
</w:num>
```

**Different type (restart):**
```rust
// List 1: numbered
Block::List { kind: Ordered, items: [...] }

// List 2: bullet (restarts)
Block::List { kind: Unordered, items: [...] }
```

Generate:
```xml
<!-- Different w:numId for different types -->
<w:num w:numId="1">
  <w:abstractNumId w:val="0"/>  <!-- Ordered -->
</w:num>

<w:num w:numId="2">
  <w:abstractNumId w:val="1"/>  <!-- Unordered -->
</w:num>
```

#### 2. Nested Lists with Mixed Types

```rust
Block::List {
    kind: Ordered,
    items: vec![
        ListItem {
            level: 0,
            content: vec![
                Block::Paragraph(...),
                Block::List {
                    kind: Unordered,
                    items: vec![
                        ListItem { level: 0, content: [...] },
                    ],
                },
            ],
        },
    ],
}
```

Generate:
```xml
<!-- Abstract definition with mixed levels -->
<w:abstractNum w:abstractNumId="0">
  <!-- Level 0: decimal -->
  <w:lvl w:ilvl="0">
    <w:numFmt w:val="decimal"/>
    <w:lvlText w:val="%1."/>
    <!-- ... -->
  </w:lvl>
  
  <!-- Level 1: bullet (for nested unordered) -->
  <w:lvl w:ilvl="1">
    <w:numFmt w:val="bullet"/>
    <w:lvlText w:val="•"/>
    <!-- ... -->
  </w:lvl>
</w:abstractNum>

<!-- Numbering instance -->
<w:num w:numId="1">
  <w:abstractNumId w:val="0"/>
</w:num>
```

Document:
```xml
<!-- Top-level ordered item -->
<w:p>
  <w:pPr>
    <w:numPr>
      <w:ilvl w:val="0"/>
      <w:numId w:val="1"/>
    </w:numPr>
  </w:pPr>
  <w:r><w:t>First item</w:t></w:r>
</w:p>

<!-- Nested unordered item -->
<w:p>
  <w:pPr>
    <w:numPr>
      <w:ilvl w:val="1"/>
      <w:numId w:val="1"/>
    </w:numPr>
  </w:pPr>
  <w:r><w:t>Nested bullet</w:t></w:r>
</w:p>
```

#### 3. Restart vs Continue Numbering

**Lontar decision rule:**
- If two consecutive `Block::List` elements have the same `kind`, reuse the same `w:numId` (continue)
- If they have different `kind`, use a different `w:numId` (restart)
- If there's a non-list block between them, use a different `w:numId` (restart)

```rust
// Continue numbering
Block::List { kind: Ordered, items: [...] }
Block::List { kind: Ordered, items: [...] }  // Same numId

// Restart numbering
Block::List { kind: Ordered, items: [...] }
Block::Paragraph(...)  // Non-list block
Block::List { kind: Ordered, items: [...] }  // Different numId

// Restart numbering (different type)
Block::List { kind: Ordered, items: [...] }
Block::List { kind: Unordered, items: [...] }  // Different numId
```

---

## Implementation Checklist for lontar-docx

### Phase 1 (MVP)

- [ ] Create `word/numbering.xml` with abstract definitions
- [ ] Generate `w:abstractNum` for ordered and unordered lists
- [ ] Generate `w:num` instances with correct IDs
- [ ] Implement ID management (abstractNumId, numId)
- [ ] Reference numbering from document.xml via `w:numPr`
- [ ] Support single-level lists (no nesting)
- [ ] Add relationship entry in `word/_rels/document.xml.rels`
- [ ] Add content type entry in `[Content_Types].xml`

### Phase 2

- [ ] Support nested lists (multiple levels)
- [ ] Support mixed list types (ordered parent, unordered children)
- [ ] Implement restart vs continue logic
- [ ] Support custom bullet characters
- [ ] Support custom numbering formats (Roman numerals, letters, etc.)

### Phase 3+

- [ ] Support legal numbering (Article, Section, Clause)
- [ ] Support custom indentation and spacing
- [ ] Support level overrides (w:lvlOverride)
- [ ] Support paragraph properties for list items (spacing, borders, shading)

---

## Complete Example: Multi-Level Document

### numbering.xml

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:numbering xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  
  <!-- Abstract definition for ordered lists -->
  <w:abstractNum w:abstractNumId="0">
    <w:lvl w:ilvl="0">
      <w:start w:val="1"/>
      <w:numFmt w:val="decimal"/>
      <w:lvlText w:val="%1."/>
      <w:lvlJc w:val="left"/>
      <w:pPr>
        <w:ind w:left="720" w:hanging="360"/>
      </w:pPr>
      <w:rPr>
        <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/>
      </w:rPr>
    </w:lvl>
    <w:lvl w:ilvl="1">
      <w:start w:val="1"/>
      <w:numFmt w:val="lowerLetter"/>
      <w:lvlText w:val="%2)"/>
      <w:lvlJc w:val="left"/>
      <w:pPr>
        <w:ind w:left="1440" w:hanging="360"/>
      </w:pPr>
      <w:rPr>
        <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/>
      </w:rPr>
    </w:lvl>
  </w:abstractNum>
  
  <!-- Abstract definition for unordered lists -->
  <w:abstractNum w:abstractNumId="1">
    <w:lvl w:ilvl="0">
      <w:start w:val="1"/>
      <w:numFmt w:val="bullet"/>
      <w:lvlText w:val="•"/>
      <w:lvlJc w:val="left"/>
      <w:pPr>
        <w:ind w:left="720" w:hanging="360"/>
      </w:pPr>
      <w:rPr>
        <w:rFonts w:ascii="Symbol" w:hAnsi="Symbol"/>
      </w:rPr>
    </w:lvl>
  </w:abstractNum>
  
  <!-- Numbering instances -->
  <w:num w:numId="1">
    <w:abstractNumId w:val="0"/>
  </w:num>
  
  <w:num w:numId="2">
    <w:abstractNumId w:val="1"/>
  </w:num>
  
</w:numbering>
```

### document.xml (excerpt)

```xml
<!-- Ordered list -->
<w:p>
  <w:pPr>
    <w:numPr>
      <w:ilvl w:val="0"/>
      <w:numId w:val="1"/>
    </w:numPr>
  </w:pPr>
  <w:r><w:t>First item</w:t></w:r>
</w:p>

<w:p>
  <w:pPr>
    <w:numPr>
      <w:ilvl w:val="1"/>
      <w:numId w:val="1"/>
    </w:numPr>
  </w:pPr>
  <w:r><w:t>Nested item</w:t></w:r>
</w:p>

<w:p>
  <w:pPr>
    <w:numPr>
      <w:ilvl w:val="0"/>
      <w:numId w:val="1"/>
    </w:numPr>
  </w:pPr>
  <w:r><w:t>Second item</w:t></w:r>
</w:p>

<!-- Unordered list (different numId) -->
<w:p>
  <w:pPr>
    <w:numPr>
      <w:ilvl w:val="0"/>
      <w:numId w:val="2"/>
    </w:numPr>
  </w:pPr>
  <w:r><w:t>Bullet item 1</w:t></w:r>
</w:p>

<w:p>
  <w:pPr>
    <w:numPr>
      <w:ilvl w:val="0"/>
      <w:numId w:val="2"/>
    </w:numPr>
  </w:pPr>
  <w:r><w:t>Bullet item 2</w:t></w:r>
</w:p>
```

---

## References

- ECMA-376-1:2016 — Office Open XML File Formats (Part 1: Fundamentals and Markup Language Reference)
  - Section 17.9 — Numbering
  - Section 17.9.1 — Abstract Numbering Definitions
  - Section 17.9.2 — Numbering Instances
- Microsoft Office Open XML Standard — https://www.ecma-international.org/publications-and-standards/standards/ecma-376/
