//! Style types: TextStyle, ParagraphStyle, TableStyle, StyleSheet.
//!
//! Styles use `Option<T>` fields to enable cascading resolution.
//! See DD-004 in DESIGN_DECISIONS.md for rationale.

use std::collections::HashMap;

/// Text-level formatting (bold, italic, color, font, etc.)
#[derive(Debug, Clone, Default)]
pub struct TextStyle {
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
    pub strikethrough: Option<bool>,
    pub color: Option<String>,           // RGB hex: "FF0000"
    pub font_name: Option<String>,       // "Arial", "Calibri", etc.
    pub font_size: Option<f64>,          // Points
    pub superscript: Option<bool>,
    pub subscript: Option<bool>,
    pub language: Option<String>,        // BCP 47 language tag
}

/// Paragraph-level formatting (alignment, spacing, indentation)
#[derive(Debug, Clone, Default)]
pub struct ParagraphStyle {
    pub alignment: Option<Alignment>,
    pub spacing_before: Option<f64>,     // Points
    pub spacing_after: Option<f64>,      // Points
    pub line_spacing: Option<f64>,       // 1.0 = single, 1.5 = 1.5 lines, 2.0 = double
    pub indent_left: Option<f64>,        // Inches
    pub indent_right: Option<f64>,       // Inches
    pub indent_first: Option<f64>,       // Inches (first line)
    pub text_style: Option<TextStyle>,   // Paragraph-level text formatting
}

/// Alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
    Justify,
}

/// Table-level formatting (borders, shading, etc.)
#[derive(Debug, Clone, Default)]
pub struct TableStyle {
    pub border_color: Option<String>,    // RGB hex
    pub border_width: Option<f64>,       // Points
    pub header_shading: Option<String>,  // RGB hex
    pub row_shading: Option<String>,     // RGB hex (alternating rows)
    pub cell_padding: Option<f64>,       // Points
}

/// Cell-specific formatting
#[derive(Debug, Clone, Default)]
pub struct CellStyle {
    pub background_color: Option<String>, // RGB hex
    pub text_style: Option<TextStyle>,
    pub paragraph_style: Option<ParagraphStyle>,
}

/// Named style definition (e.g., "Heading1", "Normal")
#[derive(Debug, Clone)]
pub struct NamedStyle {
    pub name: String,
    pub based_on: Option<String>,        // Parent style name
    pub text_style: Option<TextStyle>,
    pub paragraph_style: Option<ParagraphStyle>,
}

/// Style sheet: collection of named styles
#[derive(Debug, Clone, Default)]
pub struct StyleSheet {
    pub styles: HashMap<String, NamedStyle>,
    pub default_style: Option<String>,   // Name of default style
    pub page_setup: Option<PageSetup>,
}

/// Page setup configuration (paper size, margins)
#[derive(Debug, Clone, Default)]
pub struct PageSetup {
    pub width_in: Option<f64>,   // Page width in inches
    pub height_in: Option<f64>,  // Page height in inches
    pub margins: Option<Margins>,
}

/// Page margins
#[derive(Debug, Clone, Default)]
pub struct Margins {
    pub top_in: Option<f64>,
    pub bottom_in: Option<f64>,
    pub left_in: Option<f64>,
    pub right_in: Option<f64>,
    pub gutter_in: Option<f64>,
}

impl StyleSheet {
    /// Create a new empty style sheet
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a named style
    pub fn add_style(&mut self, name: String, style: NamedStyle) {
        self.styles.insert(name, style);
    }

    /// Get a style by name
    pub fn get_style(&self, name: &str) -> Option<&NamedStyle> {
        self.styles.get(name)
    }

    /// Resolve style cascade: default → named → paragraph → run
    pub fn resolve_text_style(
        &self,
        named_style: Option<&str>,
        paragraph_style: Option<&ParagraphStyle>,
        run_style: Option<&TextStyle>,
    ) -> TextStyle {
        let mut result = TextStyle::default();

        // Start with default style
        if let Some(default_name) = &self.default_style {
            if let Some(style) = self.get_style(default_name) {
                if let Some(text_style) = &style.text_style {
                    result = text_style.clone();
                }
            }
        }

        // Apply named style
        if let Some(name) = named_style {
            if let Some(style) = self.get_style(name) {
                if let Some(text_style) = &style.text_style {
                    merge_text_style(&mut result, text_style);
                }
            }
        }

        // Apply paragraph style
        if let Some(para_style) = paragraph_style {
            if let Some(text_style) = &para_style.text_style {
                merge_text_style(&mut result, text_style);
            }
        }

        // Apply run style (highest priority)
        if let Some(run_style) = run_style {
            merge_text_style(&mut result, run_style);
        }

        result
    }
}

/// Merge source style into target, with source taking precedence
fn merge_text_style(target: &mut TextStyle, source: &TextStyle) {
    if source.bold.is_some() {
        target.bold = source.bold;
    }
    if source.italic.is_some() {
        target.italic = source.italic;
    }
    if source.underline.is_some() {
        target.underline = source.underline;
    }
    if source.strikethrough.is_some() {
        target.strikethrough = source.strikethrough;
    }
    if source.color.is_some() {
        target.color = source.color.clone();
    }
    if source.font_name.is_some() {
        target.font_name = source.font_name.clone();
    }
    if source.font_size.is_some() {
        target.font_size = source.font_size;
    }
    if source.superscript.is_some() {
        target.superscript = source.superscript;
    }
    if source.subscript.is_some() {
        target.subscript = source.subscript;
    }
    if source.language.is_some() {
        target.language = source.language.clone();
    }
}
