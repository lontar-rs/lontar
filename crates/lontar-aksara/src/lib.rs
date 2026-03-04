//! # lontar-aksara
//!
//! Text shaping pipeline for universal script support.
//!
//! This crate integrates text shaping (rustybuzz), bidirectional text
//! resolution (unicode-bidi), script-aware line breaking (unicode-linebreak),
//! and font management into a unified pipeline consumed by format backends.
//!
//! Supports all 159+ Unicode scripts including complex scripts like
//! Aksara Bali, Devanagari, Arabic, and CJK.

pub mod bidi;
pub mod font;
pub mod lang;
pub mod linebreak;
pub mod script;
pub mod shaping;
