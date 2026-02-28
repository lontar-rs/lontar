# Reference Document Generator

Scripts to generate reference DOCX and PPTX files for Lontar's Phase 0 spec analysis.

## Usage

```bash
# Install dependencies
pip install -r tools/gen_reference/requirements.txt

# Generate reference DOCX files
python3 tools/gen_reference/minimal_docx.py

# Generate reference PPTX files
python3 tools/gen_reference/minimal_pptx.py

# Extract XML from generated files for inspection
python3 tools/gen_reference/unzip_references.py
```

## Output

- `tests/fixtures/reference_docs/` — Generated .docx and .pptx files
- `tests/fixtures/reference_docs/assets/` — Test images (auto-generated)
- `tests/fixtures/expected_xml/` — Extracted XML from each document

## Generated Documents

### DOCX
| File | Content |
|---|---|
| `minimal.docx` | Single "Hello World" paragraph |
| `styled.docx` | Bold, italic, underline, strikethrough, superscript, subscript, colors, fonts |
| `headings.docx` | All 6 heading levels |
| `tables.docx` | Simple, styled, horizontal merge, vertical merge |
| `lists.docx` | Bullet, numbered, nested bullet, nested numbered, mixed nesting |
| `code_blocks.docx` | Monospace code block paragraphs |
| `images.docx` | Inline and floating images |
| `layout.docx` | Headers, footers, page numbers, page breaks, section breaks, block quotes, TOC |

### PPTX
| File | Content |
|---|---|
| `minimal.pptx` | Title slide |
| `content.pptx` | Title + bullet list with sub-bullets |
| `styled.pptx` | Bold, italic, color, underline, font variations |
| `table.pptx` | Table with headers and data |
| `image.pptx` | Embedded image |
| `charts.pptx` | Bar chart, line chart, pie chart |
| `notes.pptx` | Slide with speaker notes |
| `two_column.pptx` | Two-column layout |
