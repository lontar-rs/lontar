//! Error types for Lontar operations.

use thiserror::Error;

/// Errors that can occur during document operations.
#[derive(Debug, Error)]
pub enum LontarError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Style not found: {0}")]
    StyleNotFound(String),

    #[error("Invalid style: {0}")]
    InvalidStyle(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Cross-reference not found: {0}")]
    CrossRefNotFound(String),

    #[error("Duplicate label: {0}")]
    DuplicateLabel(String),

    #[error("Bibliography error: {0}")]
    Bibliography(String),

    #[error("Citation not found: {0}")]
    CitationNotFound(String),

    #[error("Bibliography parse error: {0}")]
    BibliographyParse(String),
}
