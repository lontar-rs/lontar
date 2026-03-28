//! Document writer trait and error types.
//!
//! This module defines the backend-agnostic interface that all format-specific
//! writers must implement, along with error types and feature degradation reporting.

use crate::core::Document;
use std::fmt;

/// Result type for document writing operations.
pub type WriteResult<T> = Result<T, WriteError>;

/// Errors that can occur during document writing.
#[derive(Debug, Clone)]
pub enum WriteError {
    /// Citation key not found in bibliography
    CitationNotFound {
        key: String,
        available_keys: Vec<String>,
    },
    /// Bibliography style not supported by backend
    BibliographyStyleNotSupported {
        style: String,
        supported: Vec<String>,
    },
    /// Cross-reference label not found
    CrossRefNotFound { label: String },
    /// Resource (image, etc.) not found
    ResourceNotFound { id: String },
    /// Feature not supported by backend
    FeatureNotSupported { feature: String, backend: String },
    /// Invalid document structure
    InvalidDocument { reason: String },
    /// I/O error
    IoError { message: String },
    /// Other error
    Other { message: String },
}

impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WriteError::CitationNotFound {
                key,
                available_keys,
            } => {
                write!(
                    f,
                    "Citation key '{}' not found. Available keys: {}",
                    key,
                    available_keys.join(", ")
                )
            }
            WriteError::BibliographyStyleNotSupported { style, supported } => {
                write!(
                    f,
                    "Bibliography style '{}' not supported. Supported styles: {}",
                    style,
                    supported.join(", ")
                )
            }
            WriteError::CrossRefNotFound { label } => {
                write!(f, "Cross-reference label '{label}' not found")
            }
            WriteError::ResourceNotFound { id } => {
                write!(f, "Resource '{id}' not found")
            }
            WriteError::FeatureNotSupported { feature, backend } => {
                write!(f, "{backend} backend does not support: {feature}")
            }
            WriteError::InvalidDocument { reason } => {
                write!(f, "Invalid document: {reason}")
            }
            WriteError::IoError { message } => {
                write!(f, "I/O error: {message}")
            }
            WriteError::Other { message } => {
                write!(f, "{message}")
            }
        }
    }
}

impl std::error::Error for WriteError {}

/// Feature degradation report.
///
/// When a backend encounters unsupported features, it can report them
/// in a WriteReport instead of failing. This allows graceful degradation.
#[derive(Debug, Clone, Default)]
pub struct WriteReport {
    /// Features that were skipped or degraded
    pub degradations: Vec<Degradation>,
    /// Warnings about the output
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Degradation {
    pub feature: String,
    pub reason: String,
    pub fallback: Option<String>,
}

impl WriteReport {
    /// Create a new empty report.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a degradation.
    pub fn degrade(mut self, feature: impl Into<String>, reason: impl Into<String>) -> Self {
        self.degradations.push(Degradation {
            feature: feature.into(),
            reason: reason.into(),
            fallback: None,
        });
        self
    }

    /// Add a degradation with fallback.
    pub fn degrade_with_fallback(
        mut self,
        feature: impl Into<String>,
        reason: impl Into<String>,
        fallback: impl Into<String>,
    ) -> Self {
        self.degradations.push(Degradation {
            feature: feature.into(),
            reason: reason.into(),
            fallback: Some(fallback.into()),
        });
        self
    }

    /// Add a warning.
    pub fn warn(mut self, message: impl Into<String>) -> Self {
        self.warnings.push(message.into());
        self
    }

    /// Check if there are any degradations.
    pub fn has_degradations(&self) -> bool {
        !self.degradations.is_empty()
    }

    /// Check if there are any warnings.
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

/// Backend-agnostic document writer trait.
///
/// All format-specific writers (PDF, DOCX, LaTeX, etc.) must implement this trait.
pub trait DocumentWriter {
    /// Write a document to the specified output.
    ///
    /// Returns a WriteReport with any degradations or warnings.
    fn write(&mut self, document: &Document) -> WriteResult<WriteReport>;

    /// Get the name of this writer (e.g., "PDF", "DOCX", "LaTeX").
    fn name(&self) -> &'static str;

    /// Get the list of supported bibliography styles.
    fn supported_bibliography_styles(&self) -> Vec<&'static str> {
        vec![]
    }

    /// Check if a feature is supported.
    fn supports_feature(&self, feature: &str) -> bool {
        let _ = feature;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_error_display() {
        let err = WriteError::CitationNotFound {
            key: "key2024".to_string(),
            available_keys: vec!["key2023".to_string(), "key2022".to_string()],
        };

        let msg = err.to_string();
        assert!(msg.contains("key2024"));
        assert!(msg.contains("key2023"));
    }

    #[test]
    fn test_write_report() {
        let report = WriteReport::new()
            .degrade("ligatures", "Not supported in this backend")
            .degrade_with_fallback(
                "equations",
                "Complex equations not supported",
                "Rendered as images",
            )
            .warn("Document contains unsupported features");

        assert!(report.has_degradations());
        assert!(report.has_warnings());
        assert_eq!(report.degradations.len(), 2);
        assert_eq!(report.warnings.len(), 1);
    }
}
