//! DocumentWriter trait and WriteReport.

use crate::ast::Document;
use crate::error::LontarError;
use std::path::Path;

/// Options for writing documents (backend-specific)
pub trait WriteOptions: Default {}

/// Report of what features were written vs. degraded
#[derive(Debug, Clone, Default)]
pub struct WriteReport {
    /// Features that were successfully written
    pub written_features: Vec<String>,
    /// Features that were degraded or skipped
    pub degraded_features: Vec<String>,
    /// Warnings during writing
    pub warnings: Vec<String>,
}

impl WriteReport {
    /// Create a new empty report
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a written feature
    pub fn add_written(&mut self, feature: String) {
        self.written_features.push(feature);
    }

    /// Add a degraded feature
    pub fn add_degraded(&mut self, feature: String) {
        self.degraded_features.push(feature);
    }

    /// Add a warning
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    /// Check if there were any degradations
    pub fn has_degradations(&self) -> bool {
        !self.degraded_features.is_empty()
    }
}

/// Trait that all format backends implement
pub trait DocumentWriter {
    /// Backend-specific options type
    type Options: WriteOptions;

    /// Write a document to a byte buffer
    fn write(doc: &Document, options: &Self::Options) -> Result<Vec<u8>, LontarError>;

    /// Write a document to a file
    fn write_to_file(
        doc: &Document,
        path: impl AsRef<Path>,
        options: &Self::Options,
    ) -> Result<WriteReport, LontarError> {
        let bytes = Self::write(doc, options)?;
        std::fs::write(path, bytes)?;
        Ok(WriteReport::new())
    }
}
