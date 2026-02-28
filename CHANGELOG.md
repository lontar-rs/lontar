# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
