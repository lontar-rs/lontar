//! # lontar-diagram
//!
//! Native diagramming engine for the Lontar document generation library.
//!
//! Provides structured diagram definitions (flowcharts, sequence diagrams,
//! trees, network graphs) with automatic layout algorithms and format-native
//! rendering. Diagrams are defined as nodes and edges, laid out automatically,
//! and rendered as:
//!
//! - DrawingML shapes in DOCX and PPTX (native vector, editable)
//! - Vector paths in PDF and SVG (scalable, resolution-independent)
//! - Inline SVG in HTML
//! - Mermaid code blocks in Markdown (renders on GitHub)
//! - ASCII art in plain text
//!
//! # Example
//!
//! ```rust
//! use lontar_diagram::*;
//!
//! let diagram = Diagram::flowchart()
//!     .node("start", "Start")
//!     .node("process", "Process Data")
//!     .node("end", "End")
//!     .edge("start", "process")
//!     .edge("process", "end")
//!     .build();
//! ```

pub mod layout;
pub mod model;
pub mod render;

// TODO: Phase 6 — Implement diagram model, layout engine, and renderers
