//! # lontar-xlsx
//!
//! XLSX backend for the Lontar document generation library.
//!
//! This crate provides a thin integration layer around the excellent
//! `rust_xlsxwriter` crate, mapping Lontar's unified document AST to
//! Excel spreadsheet output. Tables become worksheets, headings become
//! sheet names, and text content flows into cells.
//!
//! For advanced spreadsheet features (formulas, conditional formatting,
//! pivot tables), use `rust_xlsxwriter` directly.

// TODO: Phase 4 — Implement XlsxWriter wrapping rust_xlsxwriter
