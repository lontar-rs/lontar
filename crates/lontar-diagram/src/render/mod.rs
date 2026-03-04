//! Format-specific diagram renderers.
//!
//! Each renderer converts a PositionedDiagram into format-native output:
//! DrawingML for OOXML, vector paths for PDF/SVG, Mermaid for Markdown, ASCII for text.

pub mod ascii;
pub mod drawingml;
pub mod mermaid;
pub mod svg;

// TODO: Phase 6 — Implement renderers
