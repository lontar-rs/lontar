# Citation & Bibliography Fixtures

This directory provides shared bibliography inputs for citation tests across backends.

## Files
- `sample.bib` — BibTeX entries (article, book, inproceedings, thesis, report, webpage)
- `sample.json` — CSL-JSON equivalents of the same entries

## Citation Modes Covered
- Parenthetical: \parencite{knuth1984}
- Narrative: \textcite{knuth1984}
- Year-only: \cite*{knuth1984}
- Suppress author: \parencite*{knuth1984}
- Multiple citations: \parencite{knuth1984,lamport1994}

## Suggested Test Document
Use `tests/fixtures/reference_docs/citations.md` as the source document; backends should render citations per their conventions using `sample.bib` or `sample.json`.
