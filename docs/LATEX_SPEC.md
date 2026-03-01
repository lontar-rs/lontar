# LaTeX Backend Specification

This document specifies the complete mapping from Lontar's AST to LaTeX output, including package dependencies, command mappings, and compilation requirements.

## Table of Contents

1. [Package Dependencies](#package-dependencies)
2. [AST to LaTeX Mapping](#ast-to-latex-mapping)
3. [BibLaTeX Citation Commands](#biblatex-citation-commands)
4. [Journal Document Classes](#journal-document-classes)
5. [XeLaTeX/LuaLaTeX Font Specification](#xelatexlualatex-font-specification)
6. [TikZ Diagram Primitives](#tikz-diagram-primitives)
7. [Special Character Escaping](#special-character-escaping)
8. [Minimal Examples](#minimal-examples)

---

## Package Dependencies

The LaTeX backend uses **conditional package loading** — only packages required by the document's AST features are included in the preamble.

### Core Packages (Always Loaded)

| Package | Purpose | Requirement |
|---------|---------|-------------|
| `fontspec` | Unicode font support via system fonts | **Required** (XeLaTeX/LuaLaTeX only) |
| `babel` | Language support and hyphenation | **Required** (language from `DocumentMetadata.language`) |
| `geometry` | Page layout and margins | **Required** (default: A4, 1in margins) |

### Conditional Packages (Feature-Dependent)

| Package | AST Trigger | Purpose | Default Options |
|---------|-------------|---------|-----------------|
| `graphicx` | `Block::Image` | Image inclusion | `[]{graphicx}` |
| `booktabs` | `Block::Table` | Professional table rules | `[]{booktabs}` |
| `longtable` | `Block::Table` (multi-page) | Tables spanning multiple pages | `[]{longtable}` |
| `amsmath` | `Inline::Math`, `Block::Equation` | Math typesetting | `[]{amsmath}` |
| `amssymb` | `Inline::Math`, `Block::Equation` | Math symbols | `[]{amssymb}` |
| `listings` | `Block::CodeBlock` | Code syntax highlighting | `[basicstyle=\ttfamily]{listings}` |
| `minted` | `Block::CodeBlock` (advanced) | Advanced syntax highlighting (requires Pygments) | `[]{minted}` |
| `hyperref` | `Inline::Link`, `Inline::CrossRef` | Hyperlinks and cross-references | `[colorlinks=true,linkcolor=blue,urlcolor=blue]{hyperref}` |
| `cleveref` | `Inline::CrossRef` | Smart cross-references ("Figure 3", "Table 2") | `[]{cleveref}` |
| `tikz` | `Block::Diagram` | Vector graphics and diagrams | `[]{tikz}` |
| `biblatex` | `Inline::Citation`, `Block::Bibliography` | Bibliography management | `[backend=biber,style=<style>]{biblatex}` |
| `fancyhdr` | `DocumentMetadata` (headers/footers) | Custom headers and footers | `[]{fancyhdr}` |
| `enumitem` | `Block::List` (custom styling) | List customization | `[]{enumitem}` |
| `xcolor` | `TextStyle` (color) | Text and background colors | `[]{xcolor}` |
| `soul` | `TextStyle` (strikethrough, underline) | Text decoration | `[]{soul}` |

### Package Loading Order

Packages must be loaded in the correct order to avoid conflicts:

```latex
\documentclass{article}
% 1. Font and encoding (must be first for XeLaTeX/LuaLaTeX)
\usepackage{fontspec}
\usepackage[english]{babel}

% 2. Page layout
\usepackage[a4paper,margin=1in]{geometry}

% 3. Graphics and color
\usepackage{graphicx}
\usepackage{xcolor}

% 4. Tables
\usepackage{booktabs}
\usepackage{longtable}

% 5. Math
\usepackage{amsmath}
\usepackage{amssymb}

% 6. Code
\usepackage{listings}  % or minted

% 7. TikZ (before hyperref)
\usepackage{tikz}

% 8. Bibliography (before hyperref)
\usepackage[backend=biber,style=numeric]{biblatex}
\addbibresource{references.bib}

% 9. Hyperref (must be near end)
\usepackage{hyperref}

% 10. Cleveref (must be after hyperref)
\usepackage{cleveref}

% 11. Headers/footers (after hyperref)
\usepackage{fancyhdr}
```

---

## AST to LaTeX Mapping

Complete mapping of AST nodes to LaTeX commands and environments.

### Block-Level Elements

| AST Node | LaTeX Output | Package Required | Notes |
|----------|--------------|------------------|-------|
| `Heading { level: 1 }` | `\section{...}` | None | Numbered by default |
| `Heading { level: 2 }` | `\subsection{...}` | None | |
| `Heading { level: 3 }` | `\subsubsection{...}` | None | |
| `Heading { level: 4 }` | `\paragraph{...}` | None | Inline heading |
| `Heading { level: 5 }` | `\subparagraph{...}` | None | Inline heading |
| `Heading { level: 6+ }` | `\textbf{...}\par` | None | Fallback for deep nesting |
| `Paragraph` | Text + `\par` | None | Blank line separation |
| `Table` | `\begin{table}...\end{table}` + `tabular` | `booktabs` | See table mapping below |
| `Image` | `\begin{figure}...\end{figure}` + `\includegraphics` | `graphicx` | See image mapping below |
| `List { ordered: false }` | `\begin{itemize}...\end{itemize}` | None | Unordered list |
| `List { ordered: true }` | `\begin{enumerate}...\end{enumerate}` | None | Ordered list |
| `CodeBlock` | `\begin{lstlisting}...\end{lstlisting}` | `listings` | Or `minted` |
| `BlockQuote` | `\begin{quote}...\end{quote}` | None | Indented block |
| `PageBreak` | `\newpage` | None | Hard page break |
| `HorizontalRule` | `\hrulefill` or `\rule{\linewidth}{0.4pt}` | None | Horizontal line |
| `Equation` | `\begin{equation}...\end{equation}` | `amsmath` | Numbered equation |
| `Diagram` | `\begin{tikzpicture}...\end{tikzpicture}` | `tikz` | Vector graphics |
| `Bibliography` | `\printbibliography` | `biblatex` | Reference list |
| `TableOfContents` | `\tableofcontents` | None | Auto-generated TOC |

### Inline Elements

| AST Node | LaTeX Output | Package Required | Notes |
|----------|--------------|------------------|-------|
| `Text { style: TextStyle::default() }` | Plain text | None | Escaped special chars |
| `Text { style.bold: true }` | `\textbf{...}` | None | Bold text |
| `Text { style.italic: true }` | `\textit{...}` | None | Italic text |
| `Text { style.bold + italic }` | `\textbf{\textit{...}}` | None | Nested styles |
| `Text { style.underline: true }` | `\underline{...}` | None | Underlined text |
| `Text { style.strikethrough: true }` | `\st{...}` | `soul` | Strikethrough |
| `Text { style.code: true }` | `\texttt{...}` | None | Monospace |
| `Text { style.superscript: true }` | `\textsuperscript{...}` | None | Superscript |
| `Text { style.subscript: true }` | `\textsubscript{...}` | None | Subscript |
| `Text { style.color: Some(...) }` | `\textcolor{rgb}{...}` | `xcolor` | RGB color |
| `Link { url, text }` | `\href{url}{text}` | `hyperref` | Clickable hyperlink |
| `InlineImage` | `\includegraphics[height=1em]{...}` | `graphicx` | Inline image |
| `Citation { mode: Parenthetical }` | `\parencite{key}` | `biblatex` | (Author, Year) or [1] |
| `Citation { mode: Narrative }` | `\textcite{key}` | `biblatex` | Author (Year) |
| `Citation { mode: YearOnly }` | `\cite*{key}` | `biblatex` | (Year) only |
| `Citation { mode: SuppressAuthor }` | `\parencite*{key}` | `biblatex` | (Year) |
| `Citation { mode: Full }` | `\fullcite{key}` | `biblatex` | Full reference inline |
| `CrossRef { kind: Auto }` | `\cref{label}` | `cleveref` | "Figure 3", "Table 2" |
| `CrossRef { kind: Number }` | `\ref{label}` | `hyperref` | "3" |
| `CrossRef { kind: Page }` | `\pageref{label}` | `hyperref` | "12" |
| `CrossRef { kind: Title }` | `\nameref{label}` | `hyperref` | Section title |
| `FootnoteRef` | `\footnote{...}` | None | Footnote |
| `LineBreak` | `\\` or `\newline` | None | Line break |
| `NonBreakingSpace` | `~` | None | Non-breaking space |
| `Tab` | `\quad` | None | Horizontal space |
| `Math { display: false }` | `$...$` | `amsmath` | Inline math |
| `Math { display: true }` | `\[...\]` or `\begin{equation*}` | `amsmath` | Display math |
| `PageNumber` | `\thepage` | None | Current page number |
| `PageCount` | `\pageref{LastPage}` | `lastpage` | Total pages |
| `CurrentDate` | `\today` | None | Current date |

### Table Mapping

```latex
% AST: Table { headers, rows, style, caption }
\begin{table}[h]
\centering
\caption{Caption text}
\label{table:label}
\begin{tabular}{lll}  % column alignment from style
\toprule
Header 1 & Header 2 & Header 3 \\
\midrule
Cell 1 & Cell 2 & Cell 3 \\
Cell 4 & Cell 5 & Cell 6 \\
\bottomrule
\end{tabular}
\end{table}
```

**Column alignment:**
- `TextAlign::Left` → `l`
- `TextAlign::Center` → `c`
- `TextAlign::Right` → `r`

**Cell spanning:**
- `colspan: Some(n)` → `\multicolumn{n}{c}{content}`
- `rowspan: Some(n)` → `\multirow{n}{*}{content}` (requires `multirow` package)

### Image Mapping

```latex
% AST: Image { resource_id, alt_text, width, height, label, caption }
\begin{figure}[h]
\centering
\includegraphics[width=0.8\textwidth]{media/image.png}
\caption{Caption text}
\label{fig:label}
\end{figure}
```

**Size specification:**
- `width: Some(w)` → `width=w\textwidth` (if w < 1.0) or `width=wcm` (if w > 1.0)
- `height: Some(h)` → `height=hcm`
- Both specified → `width=...,height=...,keepaspectratio`

---

## BibLaTeX Citation Commands

Mapping from `CitationMode` enum to BibLaTeX commands.

### Citation Mode Mapping

| `CitationMode` | BibLaTeX Command | Numeric Style Output | Author-Year Output | Notes |
|----------------|------------------|----------------------|--------------------|-------|
| `Parenthetical` | `\parencite{key}` | `[1]` | `(Smith, 2024)` | Default citation |
| `Narrative` | `\textcite{key}` | `Smith [1]` | `Smith (2024)` | Author in text |
| `YearOnly` | `\cite*{key}` | `2024` | `2024` | Year only (rare in numeric) |
| `SuppressAuthor` | `\parencite*{key}` | `[1]` | `(2024)` | Suppress author |
| `Full` | `\fullcite{key}` | Full reference | Full reference | Complete citation inline |

### Multiple Citations

```latex
% AST: Citation { keys: ["key1", "key2"], ... }
\parencite{key1,key2}  % Output: [1, 2] or (Smith, 2024; Jones, 2023)
```

### Citation with Prefix/Suffix

```latex
% AST: Citation { keys: ["key"], prefix: Some("see"), suffix: Some("p. 42") }
\parencite[see][p. 42]{key}  % Output: [see 1, p. 42] or (see Smith, 2024, p. 42)
```

### Bibliography Styles

| `BibliographyStyle` | BibLaTeX Style Option | Common Use Case |
|---------------------|----------------------|-----------------|
| `Numeric` | `style=numeric` | Computer science, physics |
| `AuthorYear` | `style=authoryear` | Social sciences, humanities |
| `Superscript` | `style=numeric,autocite=superscript` | Medical journals |
| `Vancouver` | `style=vancouver` | Medical journals (BMJ, Lancet) |
| `Apa7` | `style=apa` | Psychology, education |
| `Named(s)` | `style=s` | Custom CSL or BibLaTeX style |

### Bibliography Output

```latex
% AST: Block::Bibliography
\printbibliography[title={References}]
```

**Filtering options:**
```latex
\printbibliography[type=article,title={Journal Articles}]
\printbibliography[keyword=primary,title={Primary Sources}]
```

---

## Journal Document Classes

Common medical and scientific journal document classes and their LaTeX packages.

### Medical Journals

| Journal | Document Class | Package/Template | Notes |
|---------|----------------|------------------|-------|
| **The Lancet** | `elsarticle` | Elsevier template | `\documentclass[review]{elsarticle}` |
| **BMJ** | `bmj` | Custom BMJ class | `\documentclass{bmj}` |
| **JAMA** | `jama` | AMA Manual of Style | Custom class, strict formatting |
| **NEJM** | `nejm` | Custom NEJM class | `\documentclass{nejm}` |
| **PLOS ONE** | `plos2015` | PLOS template | `\documentclass{plos2015}` |
| **Nature** | `nature` | Nature template | `\documentclass{nature}` |
| **Cell** | `cell` | Cell Press template | `\documentclass{cell}` |

### Scientific Publishers

| Publisher | Document Class | Package | Notes |
|-----------|----------------|---------|-------|
| **Elsevier** | `elsarticle` | `elsarticle.cls` | Supports multiple journal formats |
| **Springer** | `svjour3` | Springer template | `\documentclass{svjour3}` |
| **IEEE** | `IEEEtran` | `IEEEtran.cls` | Engineering, computer science |
| **ACM** | `acmart` | `acmart.cls` | Computer science |
| **AIP** | `aip-cp` | AIP Conference Proceedings | Physics |
| **ACS** | `achemso` | American Chemical Society | Chemistry |

### Generic Academic Classes

| Class | Purpose | Package | Notes |
|-------|---------|---------|-------|
| `article` | Short papers | Built-in | Default LaTeX class |
| `report` | Long reports, theses | Built-in | Chapters, no `\part` |
| `book` | Books | Built-in | Chapters, parts, front/back matter |
| `memoir` | Flexible book/article | `memoir.cls` | Highly customizable |
| `scrartcl` | KOMA-Script article | `scrartcl.cls` | European typography |
| `amsart` | AMS article | `amsart.cls` | Mathematics |

### Document Class Options

```latex
% Common options
\documentclass[
    12pt,              % Font size: 10pt, 11pt, 12pt
    a4paper,           % Paper size: a4paper, letterpaper
    twocolumn,         % Two-column layout
    twoside,           % Two-sided printing
    draft,             % Draft mode (show overfull boxes)
    final              % Final mode (default)
]{article}
```

---

## XeLaTeX/LuaLaTeX Font Specification

Using `fontspec` for Unicode and multi-script text support.

### Basic Font Setup

```latex
\usepackage{fontspec}

% Set main font (serif, for body text)
\setmainfont{Linux Libertine O}

% Set sans-serif font (for headings, if needed)
\setsansfont{Linux Biolinum O}

% Set monospace font (for code)
\setmonofont{Fira Code}[
    Scale=MatchLowercase,
    Contextuals=Alternate  % Enable ligatures like -> => !=
]
```

### Multi-Script Support

```latex
% Latin + Cyrillic
\setmainfont{DejaVu Serif}

% Latin + Greek
\setmainfont{GFS Didot}

% Latin + Arabic
\setmainfont{Amiri}[
    Script=Arabic,
    Language=Arabic
]

% Latin + Devanagari (Hindi, Sanskrit)
\setmainfont{Noto Serif Devanagari}

% Latin + CJK (Chinese, Japanese, Korean)
\usepackage{xeCJK}
\setCJKmainfont{Noto Serif CJK SC}  % Simplified Chinese
```

### Font Features

```latex
\setmainfont{TeX Gyre Pagella}[
    Ligatures=TeX,           % Enable TeX ligatures (`` '' -- ---)
    Numbers=OldStyle,        % Old-style numerals (1234 vs 1234)
    SmallCapsFont={* SC},    % Small caps variant
    ItalicFont={* Italic},
    BoldFont={* Bold},
    BoldItalicFont={* Bold Italic}
]
```

### Fallback Fonts for Missing Glyphs

```latex
% If main font lacks certain glyphs, specify fallback
\setmainfont{Linux Libertine O}[
    Extension=.otf,
    UprightFont=*,
    BoldFont=*Bold,
    ItalicFont=*Italic,
    BoldItalicFont=*BoldItalic,
    % Fallback for missing glyphs
    RawFeature={fallback=DejaVuSerif}
]
```

### Balinese Script Example

```latex
\usepackage{fontspec}
\newfontfamily\balinesefont{Noto Serif Balinese}[
    Script=Balinese,
    Language=Balinese
]

% In document:
{\balinesefont ᬅᬓ᭄ᬱᬭ ᬩᬮᬶ}  % Aksara Bali (Balinese script)
```

### Font Selection Strategy

For Lontar's multi-script support:

1. **Default font:** Use a comprehensive Unicode font like `Noto Serif` or `DejaVu Serif`
2. **Script-specific fonts:** Detect script in text and switch fonts as needed
3. **Fallback chain:** Specify multiple fonts to cover all Unicode blocks

```latex
\setmainfont{Noto Serif}[
    Scale=1.0,
    Ligatures=TeX,
    % Automatic fallback for missing glyphs
    UprightFeatures={FallBack=DejaVu Serif},
    BoldFeatures={FallBack=DejaVu Serif Bold},
    ItalicFeatures={FallBack=DejaVu Serif Italic}
]
```

---

## TikZ Diagram Primitives

TikZ commands needed for rendering `Block::Diagram` nodes.

### Basic Shapes

```latex
\begin{tikzpicture}
% Rectangle
\draw (0,0) rectangle (2,1);
\filldraw[fill=blue!20] (0,0) rectangle (2,1);

% Circle
\draw (0,0) circle (1cm);
\filldraw[fill=red!20] (0,0) circle (1cm);

% Ellipse
\draw (0,0) ellipse (2cm and 1cm);

% Line
\draw (0,0) -- (2,1);

% Polyline
\draw (0,0) -- (1,1) -- (2,0) -- cycle;  % cycle closes the path

% Bezier curve
\draw (0,0) .. controls (1,1) and (2,1) .. (3,0);

% Arc
\draw (0,0) arc (0:90:1cm);
\end{tikzpicture}
```

### Node (Text Box)

```latex
\begin{tikzpicture}
% Simple node
\node at (0,0) {Text};

% Styled node
\node[draw, rectangle, fill=blue!20, minimum width=2cm, minimum height=1cm] at (0,0) {Box};

% Node with anchor
\node[anchor=north west] at (0,0) {Top-left aligned};

% Named node for connections
\node (A) at (0,0) {Node A};
\node (B) at (2,0) {Node B};
\draw[->] (A) -- (B);
\end{tikzpicture}
```

### Arrows and Connections

```latex
\begin{tikzpicture}
% Arrow
\draw[->] (0,0) -- (2,0);

% Double arrow
\draw[<->] (0,0) -- (2,0);

% Thick arrow
\draw[->, thick] (0,0) -- (2,0);

% Dashed line
\draw[dashed] (0,0) -- (2,0);

% Curved arrow
\draw[->, bend left] (0,0) to (2,0);
\end{tikzpicture}
```

### Flowchart Example

```latex
\begin{tikzpicture}[
    node distance=2cm,
    box/.style={rectangle, draw, fill=blue!20, minimum width=2cm, minimum height=1cm},
    decision/.style={diamond, draw, fill=yellow!20, aspect=2}
]
\node[box] (start) {Start};
\node[decision, below of=start] (decision) {Decision?};
\node[box, below left of=decision] (yes) {Yes};
\node[box, below right of=decision] (no) {No};
\node[box, below right of=yes] (end) {End};

\draw[->] (start) -- (decision);
\draw[->] (decision) -- node[left] {yes} (yes);
\draw[->] (decision) -- node[right] {no} (no);
\draw[->] (yes) -- (end);
\draw[->] (no) -- (end);
\end{tikzpicture}
```

### Required TikZ Libraries

```latex
\usepackage{tikz}
\usetikzlibrary{
    shapes.geometric,  % Diamond, ellipse, etc.
    arrows.meta,       % Arrow tips
    positioning,       % above of=, below of=
    calc,              % Coordinate calculations
    backgrounds,       % Background layers
    fit,               % Fit node around others
    decorations.pathmorphing  % Wavy lines, etc.
}
```

### Coordinate Systems

```latex
% Cartesian coordinates
\draw (0,0) -- (1,1);

% Polar coordinates
\draw (0:1cm) -- (90:1cm);

% Relative coordinates
\draw (0,0) -- ++(1,0) -- ++(0,1);  % ++ is relative to last point

% Named coordinates
\coordinate (A) at (0,0);
\coordinate (B) at (2,1);
\draw (A) -- (B);
```

---

## Special Character Escaping

LaTeX reserves certain characters for commands and syntax. These must be escaped when appearing in document text.

### Reserved Characters

| Character | LaTeX Meaning | Escape Sequence | Example Input | Example Output |
|-----------|---------------|-----------------|---------------|----------------|
| `\` | Command prefix | `\textbackslash{}` or `$\backslash$` | `C:\Users` | `C:\textbackslash{}Users` |
| `{` | Group start | `\{` | `{code}` | `\{code\}` |
| `}` | Group end | `\}` | `{code}` | `\{code\}` |
| `$` | Math mode | `\$` | `$100` | `\$100` |
| `&` | Table column separator | `\&` | `AT&T` | `AT\&T` |
| `%` | Comment | `\%` | `50%` | `50\%` |
| `#` | Macro parameter | `\#` | `#hashtag` | `\#hashtag` |
| `_` | Subscript | `\_` | `file_name` | `file\_name` |
| `^` | Superscript | `\^{}` or `\textasciicircum{}` | `x^2` | `x\^{}2` |
| `~` | Non-breaking space | `\~{}` or `\textasciitilde{}` | `~user` | `\~{}user` |

### Escaping Function

```rust
fn escape_latex(text: &str) -> String {
    text.replace('\\', r"\textbackslash{}")
        .replace('{', r"\{")
        .replace('}', r"\}")
        .replace('$', r"\$")
        .replace('&', r"\&")
        .replace('%', r"\%")
        .replace('#', r"\#")
        .replace('_', r"\_")
        .replace('^', r"\^{}")
        .replace('~', r"\~{}")
}
```

### Special Cases

**URLs:** Use `\url{...}` or `\href{...}{...}` from `hyperref` — no escaping needed inside:
```latex
\url{https://example.com/path?query=value&other=123}  % No escaping
```

**Code blocks:** Use `lstlisting` or `minted` — no escaping needed:
```latex
\begin{lstlisting}
int main() {
    printf("Hello, world!\n");  // No escaping needed
    return 0;
}
\end{lstlisting}
```

**Math mode:** Different escaping rules apply:
```latex
$x_1 + x_2 = y$  % Underscore is subscript, not escaped
```

**Verbatim text:** Use `\verb|...|` for inline verbatim:
```latex
\verb|C:\Users\file_name.txt|  % No escaping needed
```

---

## Minimal Examples

Minimal compilable `.tex` files demonstrating each AST feature.

### 1. Minimal Document

```latex
\documentclass{article}
\usepackage{fontspec}
\usepackage[english]{babel}

\begin{document}
Hello, world!
\end{document}
```

**Compile:** `xelatex minimal.tex`

### 2. Headings

```latex
\documentclass{article}
\usepackage{fontspec}
\usepackage[english]{babel}

\begin{document}
\section{Heading 1}
\subsection{Heading 2}
\subsubsection{Heading 3}
\paragraph{Heading 4}
\subparagraph{Heading 5}
\end{document}
```

### 3. Styled Text

```latex
\documentclass{article}
\usepackage{fontspec}
\usepackage[english]{babel}
\usepackage{xcolor}
\usepackage{soul}

\begin{document}
This is \textbf{bold} text.

This is \textit{italic} text.

This is \textbf{\textit{bold and italic}} text.

This is \st{strikethrough} text.

This is \texttt{code} text.

This is \textcolor{red}{colored} text.
\end{document}
```

### 4. Table

```latex
\documentclass{article}
\usepackage{fontspec}
\usepackage[english]{babel}
\usepackage{booktabs}

\begin{document}
\begin{table}[h]
\centering
\caption{Sample Table}
\label{tab:sample}
\begin{tabular}{lll}
\toprule
Header 1 & Header 2 & Header 3 \\
\midrule
Cell 1 & Cell 2 & Cell 3 \\
Cell 4 & Cell 5 & Cell 6 \\
\bottomrule
\end{tabular}
\end{table}
\end{document}
```

### 5. Image

```latex
\documentclass{article}
\usepackage{fontspec}
\usepackage[english]{babel}
\usepackage{graphicx}

\begin{document}
\begin{figure}[h]
\centering
\includegraphics[width=0.5\textwidth]{example-image}
\caption{Sample Image}
\label{fig:sample}
\end{figure}
\end{document}
```

**Note:** Requires `example-image.pdf` or use `\usepackage{mwe}` for demo images.

### 6. Lists

```latex
\documentclass{article}
\usepackage{fontspec}
\usepackage[english]{babel}

\begin{document}
\begin{itemize}
\item Bullet 1
\item Bullet 2
    \begin{itemize}
    \item Nested 2.1
    \item Nested 2.2
    \end{itemize}
\item Bullet 3
\end{itemize}

\begin{enumerate}
\item Item 1
\item Item 2
    \begin{enumerate}
    \item Nested 2.1
    \item Nested 2.2
    \end{enumerate}
\item Item 3
\end{enumerate}
\end{document}
```

### 7. Code Block

```latex
\documentclass{article}
\usepackage{fontspec}
\usepackage[english]{babel}
\usepackage{listings}

\lstset{
    basicstyle=\ttfamily,
    breaklines=true,
    frame=single
}

\begin{document}
\begin{lstlisting}[language=Python]
def hello():
    print("Hello, world!")
\end{lstlisting}
\end{document}
```

### 8. Math and Equations

```latex
\documentclass{article}
\usepackage{fontspec}
\usepackage[english]{babel}
\usepackage{amsmath}

\begin{document}
Inline math: $E = mc^2$

Display math:
\[
\int_0^\infty e^{-x^2} dx = \frac{\sqrt{\pi}}{2}
\]

Numbered equation:
\begin{equation}
\label{eq:quadratic}
x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}
\end{equation}
\end{document}
```

### 9. Citations and Bibliography

```latex
\documentclass{article}
\usepackage{fontspec}
\usepackage[english]{babel}
\usepackage[backend=biber,style=numeric]{biblatex}
\addbibresource{references.bib}

\begin{document}
According to \textcite{knuth1984}, TeX is a typesetting system.

Multiple citations \parencite{knuth1984,lamport1994}.

\printbibliography
\end{document}
```

**references.bib:**
```bibtex
@book{knuth1984,
    author = {Donald E. Knuth},
    title = {The TeXbook},
    year = {1984},
    publisher = {Addison-Wesley}
}

@book{lamport1994,
    author = {Leslie Lamport},
    title = {LaTeX: A Document Preparation System},
    year = {1994},
    publisher = {Addison-Wesley}
}
```

**Compile:** `xelatex example.tex && biber example && xelatex example.tex`

### 10. Cross-References

```latex
\documentclass{article}
\usepackage{fontspec}
\usepackage[english]{babel}
\usepackage{hyperref}
\usepackage{cleveref}

\begin{document}
\section{Introduction}
\label{sec:intro}

See \cref{sec:methods} for details.

\section{Methods}
\label{sec:methods}

As mentioned in \cref{sec:intro}, we use...

Refer to \cref{fig:results} on page \pageref{fig:results}.

\begin{figure}[h]
\centering
\rule{5cm}{3cm}  % Placeholder
\caption{Results}
\label{fig:results}
\end{figure}
\end{document}
```

### 11. Hyperlinks

```latex
\documentclass{article}
\usepackage{fontspec}
\usepackage[english]{babel}
\usepackage{hyperref}

\begin{document}
Visit \href{https://example.com}{Example Website}.

Or use the URL directly: \url{https://example.com}
\end{document}
```

### 12. TikZ Diagram

```latex
\documentclass{article}
\usepackage{fontspec}
\usepackage[english]{babel}
\usepackage{tikz}
\usetikzlibrary{shapes.geometric,arrows.meta,positioning}

\begin{document}
\begin{tikzpicture}[
    box/.style={rectangle, draw, fill=blue!20, minimum width=2cm, minimum height=1cm},
    arrow/.style={->, >=Stealth, thick}
]
\node[box] (A) {Start};
\node[box, right=of A] (B) {Process};
\node[box, right=of B] (C) {End};

\draw[arrow] (A) -- (B);
\draw[arrow] (B) -- (C);
\end{tikzpicture}
\end{document}
```

---

## Compilation Workflow

### Standard Compilation

```bash
# Single pass (no citations/cross-refs)
xelatex document.tex

# With citations (BibLaTeX)
xelatex document.tex
biber document
xelatex document.tex
xelatex document.tex  # Second pass for cross-refs

# With citations (BibTeX - legacy)
xelatex document.tex
bibtex document
xelatex document.tex
xelatex document.tex
```

### LuaLaTeX Alternative

```bash
lualatex document.tex
biber document
lualatex document.tex
lualatex document.tex
```

### Makefile Example

```makefile
MAIN = document
TEX = xelatex
BIB = biber

all: $(MAIN).pdf

$(MAIN).pdf: $(MAIN).tex references.bib
	$(TEX) $(MAIN)
	$(BIB) $(MAIN)
	$(TEX) $(MAIN)
	$(TEX) $(MAIN)

clean:
	rm -f *.aux *.log *.bbl *.blg *.bcf *.run.xml *.out *.toc

.PHONY: all clean
```

---

## Error Handling

Common LaTeX errors and how to handle them in the backend.

### Missing Package

**Error:** `! LaTeX Error: File 'package.sty' not found.`

**Solution:** Ensure package is installed via TeX Live or MiKTeX. Backend should check for required packages before compilation.

### Undefined Control Sequence

**Error:** `! Undefined control sequence.`

**Solution:** Command not defined. Check package loading order or typo in command name.

### Overfull/Underfull Boxes

**Warning:** `Overfull \hbox (10.0pt too wide)`

**Solution:** Text doesn't fit in line. LaTeX handles this automatically in final output. Backend can ignore these warnings.

### Missing \item

**Error:** `! LaTeX Error: Something's wrong--perhaps a missing \item.`

**Solution:** List environment (`itemize`, `enumerate`) requires `\item` for each entry. Backend must ensure all list items have `\item` prefix.

### File Not Found

**Error:** `! LaTeX Error: File 'image.png' not found.`

**Solution:** Image file missing. Backend must copy all referenced images to output directory.

---

## Performance Considerations

### Preamble Optimization

Only load packages that are actually used:

```rust
fn generate_preamble(doc: &Document) -> String {
    let mut packages = vec!["fontspec", "babel", "geometry"];
    
    if doc.has_images() {
        packages.push("graphicx");
    }
    if doc.has_tables() {
        packages.push("booktabs");
    }
    if doc.has_math() {
        packages.push("amsmath");
        packages.push("amssymb");
    }
    // ... etc
    
    packages.iter()
        .map(|p| format!("\\usepackage{{{}}}", p))
        .collect::<Vec<_>>()
        .join("\n")
}
```

### Compilation Speed

- **XeLaTeX:** Slower than pdfLaTeX, but required for Unicode
- **LuaLaTeX:** Similar speed to XeLaTeX, better for complex documents
- **Incremental compilation:** Use `latexmk` for automatic recompilation

### Output Size

- **Vector graphics:** Use PDF for images when possible (smaller, scalable)
- **Raster graphics:** Compress PNG/JPEG before inclusion
- **Fonts:** Embedded fonts increase PDF size

---

## References

- [LaTeX2e Documentation](https://www.latex-project.org/help/documentation/)
- [BibLaTeX Manual](https://ctan.org/pkg/biblatex)
- [TikZ & PGF Manual](https://ctan.org/pkg/pgf)
- [fontspec Documentation](https://ctan.org/pkg/fontspec)
- [Comprehensive LaTeX Symbol List](https://ctan.org/pkg/comprehensive)
