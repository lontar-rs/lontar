# Multi-Script Fixtures

This folder holds representative strings, font manifest, reference generators, and shaping notes for multi-script testing.

## Contents
- `strings.txt` — sample text per script category
- `fonts_manifest.txt` — Noto Sans font list + download URLs (one per category)
- `generate_refs.py` — generate DOCX and PPTX reference files from `strings.txt`
- `reference_template.tex` — XeLaTeX template for LaTeX reference doc
- `shaping_notes.md` — expected shaping behaviors per script

## Usage
1. Install dependencies (Python):
   ```bash
   pip install python-docx python-pptx
   ```
2. Download fonts (see `fonts_manifest.txt`) into `tests/fixtures/scripts/fonts/`.
3. Generate references:
   ```bash
   python generate_refs.py
   ```
   - Outputs: `reference.docx`, `reference.pptx` in this folder.
4. Build LaTeX reference (optional):
   ```bash
   xelatex reference_template.tex
   ```

## Notes
- DOCX/PPTX generation uses `strings.txt` to include all script categories.
- LaTeX template expects XeLaTeX and Noto fonts present on the system or in the TEXMF path.
