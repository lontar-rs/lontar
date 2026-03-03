# BibLaTeX Entry Types and Fields

## Overview

BibLaTeX defines standardized entry types and fields for bibliographic data. This document catalogs all entry types relevant to Lontar, with required and optional fields for each.

## Entry Types

### @article — Journal Article

**Required fields:**
- `author` — Author(s)
- `title` — Article title
- `journaltitle` (or `journal`) — Journal name
- `year` (or `date`) — Publication year

**Optional fields:**
- `volume` — Volume number
- `number` (or `issue`) — Issue number
- `pages` — Page range
- `doi` — Digital Object Identifier
- `url` — Web address
- `issn` — Journal ISSN
- `abstract` — Article abstract
- `keywords` — Keywords

**Example:**
```bibtex
@article{knuth1984,
  author       = {Donald E. Knuth},
  title        = {The {TeX}book},
  journaltitle = {Computers \& Typesetting},
  year         = {1984},
  volume       = {A},
  doi          = {10.1234/example}
}
```

### @book — Book

**Required fields:**
- `author` (or `editor`) — Author(s) or editor(s)
- `title` — Book title
- `year` (or `date`) — Publication year
- `publisher` — Publisher name

**Optional fields:**
- `edition` — Edition number
- `volume` — Volume number
- `series` — Series name
- `address` (or `location`) — Publisher location
- `isbn` — ISBN
- `url` — Web address
- `pages` — Total pages

**Example:**
```bibtex
@book{lamport1994,
  author    = {Leslie Lamport},
  title     = {LaTeX: A Document Preparation System},
  year      = {1994},
  edition   = {2nd},
  publisher = {Addison-Wesley},
  address   = {Reading, MA},
  isbn      = {978-0201529838}
}
```

### @inproceedings (or @conference) — Conference Paper

**Required fields:**
- `author` — Author(s)
- `title` — Paper title
- `booktitle` — Conference proceedings title
- `year` (or `date`) — Publication year

**Optional fields:**
- `editor` — Proceedings editor(s)
- `volume` — Volume number
- `pages` — Page range
- `publisher` — Publisher
- `address` (or `location`) — Conference location
- `doi` — DOI
- `url` — Web address

**Example:**
```bibtex
@inproceedings{miller2023,
  author    = {Alice Miller and Bob Chen},
  title     = {Efficient Graph Layouts for Large Diagrams},
  booktitle = {Proceedings of the Diagram Systems Conference},
  year      = {2023},
  pages     = {101--115},
  doi       = {10.1234/diag.2023.15}
}
```

### @phdthesis — PhD Thesis

**Required fields:**
- `author` — Author
- `title` — Thesis title
- `school` (or `institution`) — University name
- `year` (or `date`) — Year

**Optional fields:**
- `type` — Thesis type (defaults to "PhD thesis")
- `address` (or `location`) — University location
- `url` — Web address
- `doi` — DOI

**Example:**
```bibtex
@phdthesis{garcia2022,
  author = {Maria Garcia},
  title  = {Multi-Script Typesetting for Scientific Publishing},
  school = {University of Example},
  year   = {2022},
  url    = {https://example.edu/theses/garcia2022}
}
```

### @mastersthesis — Master's Thesis

**Required fields:**
- `author` — Author
- `title` — Thesis title
- `school` (or `institution`) — University name
- `year` (or `date`) — Year

**Optional fields:**
- Same as `@phdthesis`

**Example:**
```bibtex
@mastersthesis{smith2021,
  author = {John Smith},
  title  = {Automated Bibliography Generation},
  school = {Example University},
  year   = {2021}
}
```

### @techreport (or @report) — Technical Report

**Required fields:**
- `author` — Author(s)
- `title` — Report title
- `institution` — Institution name
- `year` (or `date`) — Year

**Optional fields:**
- `type` — Report type
- `number` — Report number
- `address` (or `location`) — Institution location
- `url` — Web address

**Example:**
```bibtex
@techreport{lee2021,
  author      = {Daniel Lee},
  title       = {Unicode Fonts for Complex Scripts},
  institution = {Example Research Lab},
  number      = {ERL-2021-05},
  year        = {2021}
}
```

### @online (or @electronic, @www) — Web Resource

**Required fields:**
- `author` (or `editor`) — Author(s) or editor(s)
- `title` — Resource title
- `year` (or `date`) — Publication/access year
- `url` — Web address

**Optional fields:**
- `urldate` — Date accessed
- `organization` — Publishing organization
- `version` — Version number

**Example:**
```bibtex
@online{webaccess2024,
  author  = {Priya Singh},
  title   = {Accessible Math on the Web: A Practical Guide},
  year    = {2024},
  url     = {https://example.com/accessible-math},
  urldate = {2024-02-01}
}
```

### @incollection — Chapter in Edited Book

**Required fields:**
- `author` — Chapter author(s)
- `title` — Chapter title
- `booktitle` — Book title
- `publisher` — Publisher
- `year` (or `date`) — Year

**Optional fields:**
- `editor` — Book editor(s)
- `pages` — Page range
- `chapter` — Chapter number
- `edition` — Edition
- `volume` — Volume number

**Example:**
```bibtex
@incollection{jones2020,
  author    = {Sarah Jones},
  title     = {Typography in the Digital Age},
  booktitle = {Handbook of Modern Publishing},
  editor    = {Robert Brown},
  publisher = {Academic Press},
  year      = {2020},
  pages     = {45--78}
}
```

### @misc — Miscellaneous

**Required fields:**
- `author` (or `editor`) — Author(s) or editor(s)
- `title` — Title
- `year` (or `date`) — Year

**Optional fields:**
- `howpublished` — How it was published
- `note` — Additional notes
- `url` — Web address

**Example:**
```bibtex
@misc{dataset2023,
  author       = {Data Consortium},
  title        = {Multi-Script Typography Dataset},
  year         = {2023},
  howpublished = {Zenodo},
  doi          = {10.5281/zenodo.1234567}
}
```

## Common Fields Reference

### Person Names

| Field | Description | Example |
|---|---|---|
| `author` | Author(s) | `Donald E. Knuth` |
| `editor` | Editor(s) | `Leslie Lamport` |
| `translator` | Translator(s) | `John Smith` |

**Multiple authors:** Separate with `and`
```bibtex
author = {Alice Miller and Bob Chen and Carol Davis}
```

**Name formats:**
- `First Last` → `Donald E. Knuth`
- `Last, First` → `Knuth, Donald E.`
- `von Last, First` → `von Neumann, John`

### Titles

| Field | Description |
|---|---|
| `title` | Main title |
| `subtitle` | Subtitle |
| `booktitle` | Book/proceedings title |
| `journaltitle` | Journal name |

**Capitalization:** Use `{...}` to preserve case
```bibtex
title = {The {TeX}book}  % TeX stays capitalized
```

### Publication Info

| Field | Description | Format |
|---|---|---|
| `year` | Year | `2024` |
| `date` | Full date | `2024-03-15` |
| `month` | Month | `mar` or `3` |
| `publisher` | Publisher | `Addison-Wesley` |
| `address` / `location` | Place | `Reading, MA` |

### Identifiers

| Field | Description | Example |
|---|---|---|
| `doi` | Digital Object Identifier | `10.1234/example` |
| `isbn` | Book identifier | `978-0201529838` |
| `issn` | Journal identifier | `1234-5678` |
| `url` | Web address | `https://example.com` |
| `urldate` | Access date | `2024-02-01` |

### Pagination

| Field | Description | Format |
|---|---|---|
| `pages` | Page range | `101--115` |
| `page` | Single page | `42` |
| `chapter` | Chapter number | `5` |
| `volume` | Volume | `12` |
| `number` / `issue` | Issue | `3` |

## Field Type Reference

### Required vs Optional

- **Required:** Entry is invalid without these
- **Optional:** Recommended but not required

### Field Data Types

| Type | Description | Example |
|---|---|---|
| Literal | Plain text | `Addison-Wesley` |
| Name list | Person names | `Donald E. Knuth and Leslie Lamport` |
| Date | ISO 8601 date | `2024-03-15` |
| Integer | Number | `1984` |
| Range | Page/year range | `101--115` |
| Key | Citation key | `knuth1984` |

## Special Characters

### Escaping

| Character | BibTeX | Output |
|---|---|---|
| `&` | `\&` | & |
| `%` | `\%` | % |
| `$` | `\$` | $ |
| `#` | `\#` | # |
| `_` | `\_` | _ |
| `{` | `\{` | { |
| `}` | `\}` | } |
| `~` | `\~{}` | ~ |
| `^` | `\^{}` | ^ |
| `\` | `\textbackslash{}` | \ |

### Accents

| Accent | BibTeX | Output |
|---|---|---|
| Acute | `{\'e}` | é |
| Grave | `{\`e}` | è |
| Circumflex | `{\^e}` | ê |
| Umlaut | `{\"o}` | ö |
| Tilde | `{\~n}` | ñ |
| Cedilla | `{\c c}` | ç |

## Validation Rules

1. **Citation key:** Must be unique, alphanumeric + `-_`
2. **Required fields:** Must be present for entry type
3. **Author names:** Must parse correctly
4. **Dates:** Must be valid ISO 8601 or year only
5. **URLs:** Must be valid HTTP(S) URLs
6. **DOIs:** Must match DOI format `10.xxxx/yyyy`

## References

- [BibLaTeX Documentation](https://ctan.org/pkg/biblatex)
- [BibLaTeX Entry Types](https://www.overleaf.com/learn/latex/Bibliography_management_with_biblatex)
- [BibTeX Format](http://www.bibtex.org/Format/)
