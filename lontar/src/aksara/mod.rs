//! Text shaping, BiDi, and font management.
//!
//! This module provides the text shaping pipeline for converting logical text
//! into positioned glyphs, handling complex scripts, bidirectional text, and
//! font management.
//!
//! # Submodules
//!
//! - `test_strings` — Test strings for validating text shaping across scripts
//! - `prototype` — Prototype implementations and tests for text shaping

pub mod prototype;
pub mod test_strings;
