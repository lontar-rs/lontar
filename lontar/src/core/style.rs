//! Style and formatting types.
//!
//! This module provides types for defining and managing styles for
//! text, paragraphs, tables, and other document elements.

use std::collections::HashMap;

/// Text styling (font, size, color, etc.).
///
/// Defines the visual appearance of text runs. styles can be applied
/// directly to text elements or inherited from parent styles.
#[derive(Debug, Clone, Default)]
pub struct TextStyle {
    pub font_family: Option<String>,
    pub font_size: Option<f32>,
    pub font_weight: Option<FontWeight>,
    pub font_style: Option<FontStyle>,
    pub color: Option<Color>,
    pub background_color: Option<Color>,
    pub underline: bool,
    pub strikethrough: bool,
    pub script: Option<Script>,
}

/// Font weight options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

/// Font style options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

/// RGB color representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Script options for text positioning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Script {
    Latin,
    Arabic,
    Devanagari,
    Bengali,
    Tamil,
    Telugu,
    Kannada,
    Malayalam,
    Thai,
    Lao,
    Khmer,
    Myanmar,
    Han,
    Hiragana,
    Katakana,
    Hangul,
    Balinese,
    Hebrew,
    Cyrillic,
    Greek,
}

/// Paragraph styling (alignment, spacing, indentation, etc.).
///
/// Defines the layout and formatting of paragraphs, including alignment,
/// spacing, and indentation properties.
#[derive(Debug, Clone, Default)]
pub struct ParagraphStyle {
    pub alignment: Option<Alignment>,
    pub line_spacing: Option<f32>,
    pub space_before: Option<f32>,
    pub space_after: Option<f32>,
    pub indent_first: Option<f32>,
    pub indent_left: Option<f32>,
    pub indent_right: Option<f32>,
    pub keep_together: bool,
    pub keep_with_next: bool,
}

/// Text alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
    Justify,
}

/// Table styling options.
///
/// Defines the visual appearance of tables, including borders, spacing,
/// and header formatting.
#[derive(Debug, Clone, Default)]
pub struct TableStyle {
    pub border_width: Option<f32>,
    pub border_color: Option<Color>,
    pub cell_padding: Option<f32>,
    pub header_background: Option<Color>,
}

/// Stylesheet with named styles and cascading resolution.
///
/// Provides a collection of named styles and implements cascading
/// resolution from defaults through specific style assignments.
#[derive(Debug, Clone, Default)]
pub struct StyleSheet {
    pub text_styles: HashMap<String, TextStyle>,
    pub paragraph_styles: HashMap<String, ParagraphStyle>,
    pub table_styles: HashMap<String, TableStyle>,
    pub default_text: TextStyle,
    pub default_paragraph: ParagraphStyle,
}

impl StyleSheet {
    /// Resolve a text style by name, falling back to defaults.
    pub fn resolve_text_style(&self, name: Option<&str>) -> TextStyle {
        match name {
            Some(n) => {
                self.text_styles.get(n).cloned().unwrap_or_else(|| self.default_text.clone())
            }
            None => self.default_text.clone(),
        }
    }

    /// Resolve a paragraph style by name, falling back to defaults.
    pub fn resolve_paragraph_style(&self, name: Option<&str>) -> ParagraphStyle {
        match name {
            Some(n) => self
                .paragraph_styles
                .get(n)
                .cloned()
                .unwrap_or_else(|| self.default_paragraph.clone()),
            None => self.default_paragraph.clone(),
        }
    }

    /// Resolve a table style by name, falling back to defaults.
    pub fn resolve_table_style(&self, name: Option<&str>) -> TableStyle {
        match name {
            Some(n) => self.table_styles.get(n).cloned().unwrap_or_default(),
            None => TableStyle::default(),
        }
    }
}

impl Color {
    /// Create a new RGB color.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Create a black color.
    pub fn black() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    /// Create a white color.
    pub fn white() -> Self {
        Self { r: 255, g: 255, b: 255 }
    }

    /// Create a red color.
    pub fn red() -> Self {
        Self { r: 255, g: 0, b: 0 }
    }

    /// Create a green color.
    pub fn green() -> Self {
        Self { r: 0, g: 255, b: 0 }
    }

    /// Create a blue color.
    pub fn blue() -> Self {
        Self { r: 0, g: 0, b: 255 }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::black()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_style_creation() {
        let style = TextStyle {
            font_family: Some("Arial".to_string()),
            font_size: Some(12.0),
            font_weight: Some(FontWeight::Bold),
            color: Some(Color::red()),
            ..Default::default()
        };

        assert_eq!(style.font_family, Some("Arial".to_string()));
        assert_eq!(style.font_size, Some(12.0));
        assert_eq!(style.font_weight, Some(FontWeight::Bold));
        assert_eq!(style.color, Some(Color::red()));
        assert!(!style.underline);
    }

    #[test]
    fn test_paragraph_style_creation() {
        let style = ParagraphStyle {
            alignment: Some(Alignment::Center),
            line_spacing: Some(1.5),
            space_before: Some(12.0),
            ..Default::default()
        };

        assert_eq!(style.alignment, Some(Alignment::Center));
        assert_eq!(style.line_spacing, Some(1.5));
        assert_eq!(style.space_before, Some(12.0));
        assert!(!style.keep_together);
    }

    #[test]
    fn test_table_style_creation() {
        let style = TableStyle {
            border_width: Some(1.0),
            border_color: Some(Color::black()),
            cell_padding: Some(8.0),
            header_background: Some(Color::new(200, 200, 200)),
        };

        assert_eq!(style.border_width, Some(1.0));
        assert_eq!(style.cell_padding, Some(8.0));
    }

    #[test]
    fn test_stylesheet_resolution() {
        let mut stylesheet = StyleSheet::default();

        // Add custom text style
        stylesheet.text_styles.insert(
            "emphasis".to_string(),
            TextStyle { font_style: Some(FontStyle::Italic), ..Default::default() },
        );

        // Add custom paragraph style
        stylesheet.paragraph_styles.insert(
            "centered".to_string(),
            ParagraphStyle { alignment: Some(Alignment::Center), ..Default::default() },
        );

        // Test text style resolution
        let resolved = stylesheet.resolve_text_style(Some("emphasis"));
        assert_eq!(resolved.font_style, Some(FontStyle::Italic));

        // Test fallback to default
        let resolved = stylesheet.resolve_text_style(Some("nonexistent"));
        assert_eq!(resolved.font_style, stylesheet.default_text.font_style);

        // Test paragraph style resolution
        let resolved = stylesheet.resolve_paragraph_style(Some("centered"));
        assert_eq!(resolved.alignment, Some(Alignment::Center));
    }

    #[test]
    fn test_color_creation() {
        let black = Color::black();
        assert_eq!(black.r, 0);
        assert_eq!(black.g, 0);
        assert_eq!(black.b, 0);

        let white = Color::white();
        assert_eq!(white.r, 255);
        assert_eq!(white.g, 255);
        assert_eq!(white.b, 255);

        let red = Color::red();
        assert_eq!(red.r, 255);
        assert_eq!(red.g, 0);
        assert_eq!(red.b, 0);

        let custom = Color::new(128, 64, 192);
        assert_eq!(custom.r, 128);
        assert_eq!(custom.g, 64);
        assert_eq!(custom.b, 192);
    }

    #[test]
    fn test_font_weight_variants() {
        let weights = vec![
            FontWeight::Thin,
            FontWeight::ExtraLight,
            FontWeight::Light,
            FontWeight::Normal,
            FontWeight::Medium,
            FontWeight::SemiBold,
            FontWeight::Bold,
            FontWeight::ExtraBold,
            FontWeight::Black,
        ];

        // Test that all font weights are distinct
        for (i, weight1) in weights.iter().enumerate() {
            for (j, weight2) in weights.iter().enumerate() {
                if i != j {
                    assert_ne!(weight1, weight2);
                }
            }
        }
    }

    #[test]
    fn test_font_style_variants() {
        let _styles = vec![FontStyle::Normal, FontStyle::Italic, FontStyle::Oblique];

        assert_ne!(FontStyle::Normal, FontStyle::Italic);
        assert_ne!(FontStyle::Normal, FontStyle::Oblique);
        assert_ne!(FontStyle::Italic, FontStyle::Oblique);
    }

    #[test]
    fn test_alignment_variants() {
        let alignments =
            vec![Alignment::Left, Alignment::Center, Alignment::Right, Alignment::Justify];

        // Test that all alignments are distinct
        for (i, align1) in alignments.iter().enumerate() {
            for (j, align2) in alignments.iter().enumerate() {
                if i != j {
                    assert_ne!(align1, align2);
                }
            }
        }
    }

    #[test]
    fn test_default_styles() {
        let default_text = TextStyle::default();
        assert_eq!(default_text.font_family, None);
        assert_eq!(default_text.font_size, None);
        assert_eq!(default_text.font_weight, None);
        assert!(!default_text.underline);
        assert!(!default_text.strikethrough);

        let default_paragraph = ParagraphStyle::default();
        assert_eq!(default_paragraph.alignment, None);
        assert_eq!(default_paragraph.line_spacing, None);
        assert!(!default_paragraph.keep_together);
        assert!(!default_paragraph.keep_with_next);

        let default_table = TableStyle::default();
        assert_eq!(default_table.border_width, None);
        assert_eq!(default_table.cell_padding, None);
    }
}
