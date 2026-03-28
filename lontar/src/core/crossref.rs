//! Cross-reference management functionality.
//!
//! This module provides types and functions for managing cross-references
//! to labeled elements in documents, ensuring uniqueness and providing
//! resolution services.

use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur during cross-reference operations.
///
/// These errors are returned when registering or resolving cross-reference labels
/// in the `CrossRefRegistry`. They typically indicate structural problems with
/// the document's cross-reference system.
///
/// # Variants
///
/// * `DuplicateLabel` - Attempted to register a label that already exists
/// * `UnknownLabel` - Attempted to resolve a label that was never registered
///
/// # Examples
///
/// ```rust
/// use lontar::core::*;
///
/// let mut registry = CrossRefRegistry::default();
/// registry.register("fig1").unwrap();
///
/// // This will cause a duplicate label error
/// let result = registry.register("fig1");
/// assert!(matches!(result, Err(CrossRefError::DuplicateLabel(_))));
///
/// // This will cause an unknown label error
/// let result = registry.resolve("nonexistent");
/// assert!(matches!(result, Err(CrossRefError::UnknownLabel(_))));
/// ```
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CrossRefError {
    #[error("duplicate cross-reference label: {0}")]
    DuplicateLabel(String),
    #[error("unknown cross-reference label: {0}")]
    UnknownLabel(String),
}

/// Registry for cross-references to enforce uniqueness and provide resolution.
///
/// The cross-reference registry manages labels that can be referenced throughout
/// a document, such as figure numbers, table numbers, equation numbers, and section
/// references. It ensures that all labels are unique and provides sequential numbering
/// for registered labels.
///
/// Labels are assigned numbers in the order they are registered (1-based indexing).
/// This allows for consistent cross-referencing across different document formats.
///
/// # Examples
///
/// ```rust
/// use lontar::core::*;
///
/// let mut registry = CrossRefRegistry::default();
///
/// // Register labels for figures, tables, etc.
/// let fig1_num = registry.register("fig1").unwrap(); // Returns 1
/// let table1_num = registry.register("table1").unwrap(); // Returns 2
/// let fig2_num = registry.register("fig2").unwrap(); // Returns 3
///
/// // Resolve labels to their assigned numbers
/// assert_eq!(registry.resolve("fig1").unwrap(), "1");
/// assert_eq!(registry.resolve("table1").unwrap(), "2");
/// assert_eq!(registry.resolve("fig2").unwrap(), "3");
/// ```
#[derive(Debug, Clone, Default)]
pub struct CrossRefRegistry {
    labels: HashMap<String, usize>,
}

impl CrossRefRegistry {
    /// Register a label and assign a sequential number (1-based). Errors on duplicates.
    pub fn register(&mut self, label: impl Into<String>) -> Result<usize, CrossRefError> {
        let label = label.into();
        if self.labels.contains_key(&label) {
            return Err(CrossRefError::DuplicateLabel(label));
        }
        let number = self.labels.len() + 1;
        self.labels.insert(label, number);
        Ok(number)
    }

    /// Resolve a label to its number as string.
    pub fn resolve(&self, label: &str) -> Result<String, CrossRefError> {
        self.labels
            .get(label)
            .map(|n| n.to_string())
            .ok_or_else(|| CrossRefError::UnknownLabel(label.to_string()))
    }

    /// Number of registered labels.
    pub fn len(&self) -> usize {
        self.labels.len()
    }

    /// Check if registry is empty.
    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }
}

/// Cross-reference kind (what to display).
///
/// Defines what information should be displayed when referencing a labeled
/// element in the document. Different kinds are appropriate for different
/// contexts and document types.
///
/// # Variants
///
/// * `Auto` - Auto-generated number (e.g., "1", "Figure 1")
/// * `Page` - Page number where the label appears
/// * `Title` - Section/figure title
///
/// # Examples
///
/// ```rust
/// use lontar::core::*;
///
/// let crossref = Inline::CrossRef {
///     label: "fig1".to_string(),
///     kind: CrossRefKind::Auto,
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossRefKind {
    /// Auto-generated number (e.g., "1", "Figure 1")
    Auto,
    /// Page number
    Page,
    /// Section/figure title
    Title,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crossref_registry_register_and_resolve() {
        let mut registry = CrossRefRegistry::default();

        // Register first label
        let number1 = registry.register("fig1").unwrap();
        assert_eq!(number1, 1);

        // Register second label
        let number2 = registry.register("table1").unwrap();
        assert_eq!(number2, 2);

        // Register third label
        let number3 = registry.register("eq1").unwrap();
        assert_eq!(number3, 3);

        // Resolve labels
        assert_eq!(registry.resolve("fig1").unwrap(), "1");
        assert_eq!(registry.resolve("table1").unwrap(), "2");
        assert_eq!(registry.resolve("eq1").unwrap(), "3");
    }

    #[test]
    fn test_crossref_registry_duplicate_label() {
        let mut registry = CrossRefRegistry::default();

        // Register a label
        registry.register("label1").unwrap();

        // Try to register the same label again
        let result = registry.register("label1");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CrossRefError::DuplicateLabel("label1".to_string()));
    }

    #[test]
    fn test_crossref_registry_unknown_label() {
        let registry = CrossRefRegistry::default();

        // Try to resolve a label that doesn't exist
        let result = registry.resolve("nonexistent");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CrossRefError::UnknownLabel("nonexistent".to_string()));
    }

    #[test]
    fn test_crossref_registry_len_and_is_empty() {
        let mut registry = CrossRefRegistry::default();

        assert_eq!(registry.len(), 0);
        assert!(registry.is_empty());

        registry.register("label1").unwrap();
        assert_eq!(registry.len(), 1);
        assert!(!registry.is_empty());

        registry.register("label2").unwrap();
        assert_eq!(registry.len(), 2);
        assert!(!registry.is_empty());
    }

    #[test]
    fn test_crossref_registry_sequential_numbering() {
        let mut registry = CrossRefRegistry::default();

        // Register multiple labels and check sequential numbering
        let labels = vec!["a", "b", "c", "d", "e"];
        for (i, label) in labels.iter().enumerate() {
            let number = registry.register(*label).unwrap();
            assert_eq!(number, i + 1);
        }

        // Verify all labels resolve correctly
        for (i, label) in labels.iter().enumerate() {
            let resolved = registry.resolve(*label).unwrap();
            assert_eq!(resolved, (i + 1).to_string());
        }
    }

    #[test]
    fn test_crossref_registry_label_types() {
        let mut registry = CrossRefRegistry::default();

        // Test different types of labels that might be used in practice
        let test_labels = vec![
            "fig1",
            "figure_1",
            "table1",
            "table_1",
            "eq1",
            "equation_1",
            "sec1",
            "section_1",
            "appendix_a",
            "listing1",
            "algorithm1",
        ];

        for (i, label) in test_labels.iter().enumerate() {
            let number = registry.register(*label).unwrap();
            assert_eq!(number, i + 1);
            assert_eq!(registry.resolve(*label).unwrap(), (i + 1).to_string());
        }
    }

    #[test]
    fn test_crossref_registry_case_sensitivity() {
        let mut registry = CrossRefRegistry::default();

        // Register labels with different cases
        registry.register("Label1").unwrap();
        registry.register("label1").unwrap(); // Different case, should be allowed
        registry.register("LABEL1").unwrap(); // Different case, should be allowed

        assert_eq!(registry.resolve("Label1").unwrap(), "1");
        assert_eq!(registry.resolve("label1").unwrap(), "2");
        assert_eq!(registry.resolve("LABEL1").unwrap(), "3");
    }

    #[test]
    fn test_crossref_registry_special_characters() {
        let mut registry = CrossRefRegistry::default();

        // Test labels with special characters
        let special_labels = vec![
            "label-with-dashes",
            "label_with_underscores",
            "label.with.dots",
            "label:with:colons",
            "label123", // numbers in label
            "123label", // starting with number
        ];

        for (i, label) in special_labels.iter().enumerate() {
            let number = registry.register(*label).unwrap();
            assert_eq!(number, i + 1);
        }

        // Verify resolution
        for (i, label) in special_labels.iter().enumerate() {
            let resolved = registry.resolve(*label).unwrap();
            assert_eq!(resolved, (i + 1).to_string());
        }
    }

    #[test]
    fn test_crossref_registry_empty_string_label() {
        let mut registry = CrossRefRegistry::default();

        // Empty string should be a valid label
        let number = registry.register("").unwrap();
        assert_eq!(number, 1);
        assert_eq!(registry.resolve("").unwrap(), "1");
    }

    #[test]
    fn test_crossref_registry_unicode_labels() {
        let mut registry = CrossRefRegistry::default();

        // Test Unicode labels
        let unicode_labels = vec![
            "référence1", // French
            "ссылка1",    // Russian
            "参照1",      // Japanese
            "참조1",      // Korean
            "رابط1",      // Arabic
            "ᬦ᭄ᬢᬭ᭄ᬭᬶᬦ᭄1",  // Balinese
        ];

        for (i, label) in unicode_labels.iter().enumerate() {
            let number = registry.register(*label).unwrap();
            assert_eq!(number, i + 1);
        }

        // Verify resolution
        for (i, label) in unicode_labels.iter().enumerate() {
            let resolved = registry.resolve(*label).unwrap();
            assert_eq!(resolved, (i + 1).to_string());
        }
    }

    #[test]
    fn test_crossref_registry_long_labels() {
        let mut registry = CrossRefRegistry::default();

        // Test very long label
        let long_label = "a".repeat(1000);
        let number = registry.register(&long_label).unwrap();
        assert_eq!(number, 1);
        assert_eq!(registry.resolve(&long_label).unwrap(), "1");
    }
}
