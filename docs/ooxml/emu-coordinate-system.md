# EMU Coordinate System

Complete guide to the English Metric Units (EMU) coordinate system used for positioning in PPTX and other OOXML formats.

## EMU Definition

**EMU (English Metric Units)** is a unit of measurement used in OOXML for precise positioning and sizing.

### Conversion Factors

| Unit | EMU | Conversion |
|------|-----|-----------|
| 1 EMU | 1 | Base unit |
| 1 point (typography) | 12,700 | 1/72 inch |
| 1 twip | 635 | 1/1440 inch (used in DOCX) |
| 1 inch | 914,400 | 1 inch |
| 1 centimeter | 360,000 | 1 cm |
| 1 millimeter | 36,000 | 1 mm |
| 1 pixel (96 DPI) | 9,525 | 1 pixel at 96 DPI |

### Derivation

EMU is defined as:
```
1 EMU = 1 / 914,400 inch
1 inch = 914,400 EMU
```

This choice allows:
- **Precise positioning** — 914,400 divisors per inch enables sub-pixel accuracy
- **Integer math** — all coordinates are integers, avoiding floating-point errors
- **Compatibility** — aligns with typography points (12,700 EMU per point)

## Conversion Formulas

### To EMU

```rust
// From inches
fn inches_to_emu(inches: f64) -> i64 {
    (inches * 914_400.0) as i64
}

// From centimeters
fn cm_to_emu(cm: f64) -> i64 {
    (cm * 360_000.0) as i64
}

// From millimeters
fn mm_to_emu(mm: f64) -> i64 {
    (mm * 36_000.0) as i64
}

// From points (typography)
fn pt_to_emu(pt: f64) -> i64 {
    (pt * 12_700.0) as i64
}

// From pixels (at 96 DPI)
fn px_to_emu(px: f64) -> i64 {
    (px * 9_525.0) as i64
}
```

### From EMU

```rust
// To inches
fn emu_to_inches(emu: i64) -> f64 {
    emu as f64 / 914_400.0
}

// To centimeters
fn emu_to_cm(emu: i64) -> f64 {
    emu as f64 / 360_000.0
}

// To millimeters
fn emu_to_mm(emu: i64) -> f64 {
    emu as f64 / 36_000.0
}

// To points (typography)
fn emu_to_pt(emu: i64) -> f64 {
    emu as f64 / 12_700.0
}

// To pixels (at 96 DPI)
fn emu_to_px(emu: i64) -> f64 {
    emu as f64 / 9_525.0
}
```

## Common Dimensions

### Standard Slide Sizes

#### Standard (4:3 Aspect Ratio)

```
Width:  10 inches = 9,144,000 EMU
Height: 7.5 inches = 6,858,000 EMU
Aspect: 4:3
```

**In other units:**
- Width: 25.4 cm = 254,000 mm
- Height: 19.05 cm = 190,500 mm

#### Widescreen (16:9 Aspect Ratio)

```
Width:  10 inches = 9,144,000 EMU
Height: 5.625 inches = 5,143,500 EMU
Aspect: 16:9
```

**In other units:**
- Width: 25.4 cm = 254,000 mm
- Height: 14.288 cm = 142,875 mm

#### Widescreen (16:10 Aspect Ratio)

```
Width:  10 inches = 9,144,000 EMU
Height: 6.25 inches = 5,715,000 EMU
Aspect: 16:10
```

### Common Page Margins

#### Standard Margins (1 inch on all sides)

```
Top:    1 inch = 914,400 EMU
Right:  1 inch = 914,400 EMU
Bottom: 1 inch = 914,400 EMU
Left:   1 inch = 914,400 EMU
```

#### Narrow Margins (0.5 inch on all sides)

```
Top:    0.5 inch = 457,200 EMU
Right:  0.5 inch = 457,200 EMU
Bottom: 0.5 inch = 457,200 EMU
Left:   0.5 inch = 457,200 EMU
```

#### Wide Margins (1.5 inch on all sides)

```
Top:    1.5 inch = 1,371,600 EMU
Right:  1.5 inch = 1,371,600 EMU
Bottom: 1.5 inch = 1,371,600 EMU
Left:   1.5 inch = 1,371,600 EMU
```

### Common Text Sizes

| Size | EMU | Notes |
|------|-----|-------|
| 8pt | 101,600 | Small text |
| 10pt | 127,000 | Body text (small) |
| 11pt | 139,700 | Body text (standard) |
| 12pt | 152,400 | Body text (large) |
| 14pt | 177,800 | Subheading |
| 16pt | 203,200 | Heading |
| 18pt | 228,600 | Large heading |
| 20pt | 254,000 | Title |
| 24pt | 304,800 | Large title |
| 28pt | 355,600 | Extra large title |
| 32pt | 406,400 | Huge title |
| 36pt | 457,200 | Massive title |

### Common Shape Sizes

#### Small Icon

```
Width:  0.5 inch = 457,200 EMU
Height: 0.5 inch = 457,200 EMU
```

#### Medium Icon

```
Width:  1 inch = 914,400 EMU
Height: 1 inch = 914,400 EMU
```

#### Large Icon

```
Width:  2 inch = 1,828,800 EMU
Height: 2 inch = 1,828,800 EMU
```

#### Small Image

```
Width:  2 inch = 1,828,800 EMU
Height: 1.5 inch = 1,371,600 EMU
```

#### Medium Image

```
Width:  4 inch = 3,657,600 EMU
Height: 3 inch = 2,743,200 EMU
```

#### Large Image

```
Width:  6 inch = 5,486,400 EMU
Height: 4.5 inch = 4,114,800 EMU
```

## Positioning in PPTX

### Coordinate System

PPTX uses a standard Cartesian coordinate system:
- **Origin (0, 0)** — top-left corner of the slide
- **X-axis** — increases to the right
- **Y-axis** — increases downward

### Shape Positioning

All shapes are positioned using the `<p:xfrm>` (transform) element:

```xml
<p:xfrm>
  <a:off x="914400" y="914400"/>      <!-- Offset (position) -->
  <a:ext cx="4572000" cy="1828800"/>  <!-- Extent (size) -->
</p:xfrm>
```

**Attributes:**
- `x` — horizontal offset from left edge (EMU)
- `y` — vertical offset from top edge (EMU)
- `cx` — width (EMU)
- `cy` — height (EMU)

### Example Positions

#### Top-Left Corner (1" from edges)

```xml
<p:xfrm>
  <a:off x="914400" y="914400"/>
  <a:ext cx="3657600" cy="2743200"/>
</p:xfrm>
```

**Interpretation:**
- Position: 1" from left, 1" from top
- Size: 4" wide, 3" tall

#### Center of Standard Slide

For a standard 10" × 7.5" slide, to center a 4" × 2" shape:

```
Slide width:  9,144,000 EMU
Slide height: 6,858,000 EMU
Shape width:  3,657,600 EMU (4")
Shape height: 1,828,800 EMU (2")

X position: (9,144,000 - 3,657,600) / 2 = 2,743,200 EMU
Y position: (6,858,000 - 1,828,800) / 2 = 2,514,600 EMU
```

```xml
<p:xfrm>
  <a:off x="2743200" y="2514600"/>
  <a:ext cx="3657600" cy="1828800"/>
</p:xfrm>
```

#### Bottom-Right Corner (0.5" from edges)

For a standard 10" × 7.5" slide:

```
X position: 9,144,000 - 457,200 - 3,657,600 = 5,029,200 EMU
Y position: 6,858,000 - 457,200 - 1,828,800 = 4,572,000 EMU
```

```xml
<p:xfrm>
  <a:off x="5029200" y="4572000"/>
  <a:ext cx="3657600" cy="1828800"/>
</p:xfrm>
```

## Layout Calculations

### Two-Column Layout

For a standard 10" × 7.5" slide with 0.5" margins and 0.25" gap:

```
Slide width:     9,144,000 EMU
Left margin:       457,200 EMU
Right margin:      457,200 EMU
Gap:               228,600 EMU (0.25")
Available width: 9,144,000 - 457,200 - 457,200 = 8,229,600 EMU

Column width: (8,229,600 - 228,600) / 2 = 4,000,500 EMU ≈ 4.37"

Left column:
  x = 457,200
  cx = 4,000,500

Right column:
  x = 457,200 + 4,000,500 + 228,600 = 4,686,300
  cx = 4,000,500
```

### Three-Column Layout

```
Available width: 8,229,600 EMU
Gaps (2):        457,200 EMU (0.5" total)
Column width: (8,229,600 - 457,200) / 3 = 2,590,800 EMU ≈ 2.83"

Left column:
  x = 457,200
  cx = 2,590,800

Middle column:
  x = 457,200 + 2,590,800 + 228,600 = 3,276,600
  cx = 2,590,800

Right column:
  x = 3,276,600 + 2,590,800 + 228,600 = 6,096,000
  cx = 2,590,800
```

## Text Box Positioning

Text boxes in PPTX use the same positioning system:

```xml
<p:sp>
  <p:nvSpPr>
    <p:cNvPr id="2" name="Text Box 1"/>
    <p:cNvSpPr txBody="1"/>
    <p:nvPr/>
  </p:nvSpPr>
  <p:spPr>
    <a:xfrm>
      <a:off x="914400" y="914400"/>      <!-- 1" from left, 1" from top -->
      <a:ext cx="7315200" cy="2743200"/>  <!-- 8" wide, 3" tall -->
    </a:xfrm>
  </p:spPr>
  <p:txBody>
    <a:bodyPr/>
    <a:lstStyle/>
    <a:p>
      <a:r>
        <a:rPr lang="en-US" sz="2400"/>
        <a:t>Text content</a:t>
      </a:r>
    </a:p>
  </p:txBody>
</p:sp>
```

## Image Sizing

When embedding images, maintain aspect ratio:

```rust
fn scale_image(original_width: f64, original_height: f64, max_width: f64, max_height: f64) -> (f64, f64) {
    let width_ratio = max_width / original_width;
    let height_ratio = max_height / original_height;
    let scale = width_ratio.min(height_ratio);

    (original_width * scale, original_height * scale)
}

// Example: 800×600 image, max 4"×3"
let (w, h) = scale_image(800.0, 600.0, 4.0, 3.0);
// Result: 4.0 × 3.0 (scaled to fit)

let width_emu = inches_to_emu(w);   // 3,657,600 EMU
let height_emu = inches_to_emu(h);  // 2,743,200 EMU
```

## Table Sizing

Table column widths are specified in EMU:

```xml
<a:tblGrid>
  <a:gridCol w="2286000"/>  <!-- Column 1: 2.5" -->
  <a:gridCol w="2286000"/>  <!-- Column 2: 2.5" -->
  <a:gridCol w="2286000"/>  <!-- Column 3: 2.5" -->
</a:tblGrid>
```

**Total width:** 2,286,000 × 3 = 6,858,000 EMU = 7.5"

## Line Spacing

Line spacing in PPTX is specified in EMU:

```xml
<a:lnSpc>
  <a:spcPts val="1200"/>  <!-- 12 points = 152,400 EMU -->
</a:lnSpc>
```

**Common line spacings:**
- Single (1.0): 1,200 points = 152,400 EMU
- 1.5 lines: 1,800 points = 228,600 EMU
- Double (2.0): 2,400 points = 304,800 EMU

## Angle Measurements

Angles in OOXML are measured in 60,000ths of a degree:

```
1 degree = 60,000 units
90 degrees = 5,400,000 units
180 degrees = 10,800,000 units
270 degrees = 16,200,000 units
360 degrees = 21,600,000 units
```

**Conversion:**
```rust
fn degrees_to_ooxml(degrees: f64) -> i64 {
    (degrees * 60_000.0) as i64
}

fn ooxml_to_degrees(units: i64) -> f64 {
    units as f64 / 60_000.0
}
```

**Example (gradient angle):**
```xml
<a:lin ang="2700000" scaled="1"/>  <!-- 45 degrees (2,700,000 / 60,000) -->
```

## Opacity/Alpha

Opacity is specified as a percentage in 1,000ths:

```
0% = 0
50% = 50,000
100% = 100,000
```

**Conversion:**
```rust
fn percent_to_ooxml(percent: f64) -> i64 {
    (percent * 1_000.0) as i64
}

fn ooxml_to_percent(units: i64) -> f64 {
    units as f64 / 1_000.0
}
```

**Example (50% transparency):**
```xml
<a:alphaModFix amt="50000"/>  <!-- 50% opacity -->
```

## Summary

| Measurement | EMU Value | Formula |
|-------------|-----------|---------|
| 1 inch | 914,400 | base unit |
| 1 cm | 360,000 | inch × 360,000 / 2.54 |
| 1 mm | 36,000 | cm / 10 |
| 1 point | 12,700 | inch / 72 |
| 1 twip | 635 | point / 20 |
| 1 pixel (96 DPI) | 9,525 | inch × 96 |

**Key takeaways:**
- All positioning uses integer EMU values
- EMU enables sub-pixel precision without floating-point errors
- Standard slide is 9,144,000 × 6,858,000 EMU (10" × 7.5")
- Origin is top-left; X increases right, Y increases down
- Margins, padding, and spacing all use EMU
- Angles use 60,000ths of a degree
- Opacity uses 1,000ths of a percent

## Implementation Checklist

When implementing EMU positioning:
- [ ] Use integer arithmetic (no floating-point)
- [ ] Validate coordinates are non-negative
- [ ] Validate shapes don't exceed slide boundaries
- [ ] Account for margins when positioning
- [ ] Maintain aspect ratios for images
- [ ] Use consistent units throughout (convert to EMU once)
- [ ] Test with standard and widescreen slide sizes
- [ ] Verify positioning in Microsoft Office applications
