# Expected PDF Fixtures

This directory holds reference PDF outputs for visual comparison. PDFs are generated from the LaTeX examples in `docs/latex_examples/`.

## Prerequisites
- XeLaTeX (recommended) or LuaLaTeX
- `biber` (for `09_citations.tex`)
- TeX packages: `fontspec`, `babel`, `graphicx`, `booktabs`, `amsmath`, `amssymb`, `listings`, `hyperref`, `cleveref`, `tikz`, `biblatex`, `xcolor`, `soul`

On Ubuntu/Debian (full install):
```bash
sudo apt-get install texlive-full
```

## Generate all PDFs
From repo root:
```bash
chmod +x scripts/generate_pdf_fixtures.sh
./scripts/generate_pdf_fixtures.sh
```
Outputs will be written here as `*.pdf` matching each `.tex` example.

## Notes
- Uses XeLaTeX by default.
- Multi-script example (`16_multiscript.tex`) requires Noto fonts (see `docs/latex_examples/README.md`).
- Citations example (`09_citations.tex`) requires running `biber`.
- PDFs are for visual regression only; source of truth for expected text/HTML/LaTeX/TXT remains in `tests/fixtures/expected_{md,html,txt,tex}/`.
