//! Core error types for Lontar.
//!
//! Provides a unified `LontarError` used across builders and writers.

use crate::core::{BibliographyError, CrossRefError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LontarError {
    #[error("citation not found: {0}")]
    CitationNotFound(String),

    #[error("bibliography error: {0}")]
    Bibliography(#[from] BibliographyError),

    #[error("cross-reference error: {0}")]
    CrossRef(#[from] CrossRefError),

    #[error("invalid document: {0}")]
    InvalidDocument(String),

    #[error("I/O error: {0}")]
    Io(String),

    #[error("{0}")]
    Other(String),
}

impl From<std::io::Error> for LontarError {
    fn from(err: std::io::Error) -> Self {
        LontarError::Io(err.to_string())
    }
}
