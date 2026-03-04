# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2026-03-05

### Added
- Comprehensive README.md for lontar library crate
- CLI usage guide README.md for lontar-cli binary crate
- Feature flags documentation with usage examples

### Changed
- **BREAKING**: Consolidated multi-crate architecture into single-crate with feature flags
- Moved all backend code into `lontar/src/backends/` with feature-gated modules
- Moved core AST types from `lontar-core` to `lontar/src/core/`
- Moved text shaping from `lontar-aksara` to `lontar/src/aksara/` (feature-gated)
- Moved diagram engine from `lontar-diagram` to `lontar/src/diagram/` (feature-gated)
- Moved template system from `lontar-template` to `lontar/src/template/` (feature-gated)
- Updated MSRV from 1.85 to 1.88 (required by `time` crate dependency)
- Moved `lontar-cli` from `crates/` to workspace root

### Removed
- Separate crate structure (`crates/` directory)
- Individual crates: lontar-core, lontar-aksara, lontar-docx, lontar-pptx, lontar-pdf, lontar-xlsx, lontar-md, lontar-html, lontar-txt, lontar-template, lontar-diagram

### Fixed
- `.gitignore` now properly ignores workspace root `target/` directory

## [0.1.0] - 2026-03-01

### Added
- Initial project structure and documentation
- README.md with project vision and design principles
- ARCHITECTURE.md with technical design
- TODO.md with phased roadmap
- CONTRIBUTING.md with development guidelines
- LICENSE-APACHE and LICENSE-MIT (dual license)
- Universal script support architecture (lontar-aksara crate)
- Text shaping pipeline design: rustybuzz, unicode-bidi, unicode-linebreak
- Font management with fallback chains and subsetting
- Support for 159+ Unicode scripts including Aksara Bali, Arabic, Devanagari, CJK
- Native diagram engine architecture (lontar-diagram crate)
- Diagram AST types: DiagramKind, DiagramNode, DiagramEdge, NodeShape, EdgeKind
- Format-native rendering design: DrawingML for Office, SVG for PDF/HTML, Mermaid for Markdown, ASCII for plain text
- LaTeX backend specification with complete AST mappings
- BibLaTeX citation commands documentation
- Medical/scientific journal document classes catalog
- XeLaTeX/LuaLaTeX fontspec usage for multi-script text
- TikZ diagram primitives documentation
- 16 minimal compilable LaTeX examples for each AST feature
- Citation and bibliography test fixtures (BibTeX/CSL-JSON)
- Cross-reference test documents
- PDF reference fixtures generated from LaTeX examples
- Test fixtures for expected outputs (Markdown, HTML, TXT, LaTeX)
