//! Format-specific diagram renderers.
//!
//! Each renderer converts a PositionedDiagram into format-native output:
//! DrawingML for OOXML, vector paths for PDF/SVG, Mermaid for Markdown, ASCII for text.

pub mod drawingml;
pub mod svg;
pub mod mermaid;
pub mod ascii;

// TODO: Phase 6 — Implement renderers
