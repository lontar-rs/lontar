//! # lontar-core
//!
//! Core document model for the Lontar document generation library.
//!
//! This crate defines the unified AST (Abstract Syntax Tree) that all format
//! backends consume. Build a [`Document`] using the [`DocumentBuilder`], then
//! pass it to any backend's `DocumentWriter` implementation.
//!
//! # Example
//!
//! ```rust
//! use lontar_core::prelude::*;
//!
//! let doc = DocumentBuilder::new("My Report")
//!     .heading(1, "Introduction")
//!     .paragraph("Hello from Lontar.")
//!     .build();
//! ```

pub mod ast;
pub mod bibliography;
pub mod builder;
pub mod error;
pub mod resource;
pub mod style;
pub mod writer;

pub mod prelude {
    //! Convenience re-exports for common usage.
    pub use crate::ast::*;
    pub use crate::bibliography::{BibAuthor, BibEntry, BibEntryKind, BibError, BibliographyStore};
    pub use crate::builder::DocumentBuilder;
    pub use crate::error::LontarError;
    pub use crate::resource::{ImageSource, ResourceStore};
    pub use crate::style::*;
    pub use crate::writer::{DocumentWriter, WriteReport};
}
