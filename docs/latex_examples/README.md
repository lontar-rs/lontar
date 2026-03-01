# LaTeX Examples

Minimal compilable LaTeX examples demonstrating each AST feature.

## Compilation

All examples require XeLaTeX or LuaLaTeX (not pdfLaTeX):

```bash
xelatex example.tex
```

For examples with citations (09_citations.tex):

```bash
xelatex 09_citations.tex
biber 09_citations
xelatex 09_citations.tex
xelatex 09_citations.tex
```

For examples with table of contents (15_toc.tex):

```bash
xelatex 15_toc.tex
xelatex 15_toc.tex
```

## Examples

| File | AST Feature | Packages Required |
|------|-------------|-------------------|
| `01_minimal.tex` | Minimal document | `fontspec`, `babel` |
| `02_headings.tex` | `Heading` (levels 1-6) | `fontspec`, `babel` |
| `03_styled_text.tex` | `Text` with `TextStyle` | `fontspec`, `babel`, `xcolor`, `soul` |
| `04_table.tex` | `Table` | `fontspec`, `babel`, `booktabs` |
| `05_image.tex` | `Image` | `fontspec`, `babel`, `graphicx`, `mwe` |
| `06_lists.tex` | `List` (ordered/unordered) | `fontspec`, `babel` |
| `07_code_block.tex` | `CodeBlock` | `fontspec`, `babel`, `listings` |
| `08_math.tex` | `Math`, `Equation` | `fontspec`, `babel`, `amsmath`, `amssymb` |
| `09_citations.tex` | `Citation`, `Bibliography` | `fontspec`, `babel`, `biblatex` |
| `10_crossrefs.tex` | `CrossRef` | `fontspec`, `babel`, `hyperref`, `cleveref` |
| `11_hyperlinks.tex` | `Link` | `fontspec`, `babel`, `hyperref` |
| `12_tikz.tex` | `Diagram` (TikZ) | `fontspec`, `babel`, `tikz` |
| `13_blockquote.tex` | `BlockQuote` | `fontspec`, `babel` |
| `14_pagebreak.tex` | `PageBreak` | `fontspec`, `babel` |
| `15_toc.tex` | `TableOfContents` | `fontspec`, `babel`, `hyperref` |
| `16_multiscript.tex` | Multi-script text | `fontspec`, `babel`, Noto fonts |

## Font Requirements

For multi-script example (16_multiscript.tex), install Noto fonts:

```bash
# Ubuntu/Debian
sudo apt-get install fonts-noto fonts-noto-cjk fonts-noto-extra

# macOS
brew tap homebrew/cask-fonts
brew install --cask font-noto-serif font-noto-sans

# Or download from https://fonts.google.com/noto
```

## Package Installation

Ensure you have a complete TeX Live installation:

```bash
# Ubuntu/Debian
sudo apt-get install texlive-full

# macOS
brew install --cask mactex

# Or use TeX Live installer from https://tug.org/texlive/
```

## Verification

To verify all examples compile correctly:

```bash
#!/bin/bash
for file in *.tex; do
    echo "Compiling $file..."
    xelatex -interaction=nonstopmode "$file" > /dev/null 2>&1
    if [ $? -eq 0 ]; then
        echo "✓ $file compiled successfully"
    else
        echo "✗ $file failed to compile"
    fi
done
```

## Notes

- All examples use XeLaTeX/LuaLaTeX for Unicode support
- pdfLaTeX is not supported (cannot handle multi-script text)
- Examples use minimal preambles with only required packages
- Output PDFs demonstrate the expected rendering for each AST feature
