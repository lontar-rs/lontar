//! # Lontar
//!
//! A comprehensive document generation library for Rust.
//!
//! This is the umbrella crate that re-exports all Lontar components.
//! Enable only the backends you need via feature flags.
//!
//! ```toml
//! [dependencies]
//! lontar = { version = "0.1", features = ["docx", "pptx"] }
//! ```

pub use lontar_core::*;
