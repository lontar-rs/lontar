//! Lontar — comprehensive document generation for Rust.
//!
//! # Feature flags
//!
//! | Feature    | Enables                                      |
//! |------------|----------------------------------------------|
//! | `aksara`   | Text shaping, BiDi, font management          |
//! | `diagram`  | Native diagram engine with auto-layout       |
//! | `docx`     | DOCX output (implies `aksara`)               |
//! | `pptx`     | PPTX output (implies `aksara`)               |
//! | `pdf`      | PDF output  (implies `aksara`)               |
//! | `xlsx`     | XLSX output (wraps rust_xlsxwriter)          |
//! | `md`       | Markdown output                              |
//! | `html`     | HTML output                                  |
//! | `txt`      | Plain text output                            |
//! | `template` | Declarative document templates               |
//! | `full`     | All of the above                             |

pub mod core;

#[cfg(feature = "aksara")]
pub mod aksara;

#[cfg(feature = "diagram")]
pub mod diagram;

#[cfg(feature = "template")]
pub mod template;

pub mod backends;

pub mod prelude {
    pub use crate::core::{
        Block, Document, DocumentBuilder, DocumentMetadata, Inline, PageSetup, ParagraphStyle,
        ResourceStore, StyleSheet, TableStyle, TextStyle,
    };

    #[cfg(feature = "docx")]
    pub use crate::backends::docx::DocxWriter;

    #[cfg(feature = "pptx")]
    pub use crate::backends::pptx::PptxWriter;

    #[cfg(feature = "pdf")]
    pub use crate::backends::pdf::PdfWriter;

    #[cfg(feature = "xlsx")]
    pub use crate::backends::xlsx::XlsxWriter;

    #[cfg(feature = "md")]
    pub use crate::backends::md::MarkdownWriter;

    #[cfg(feature = "html")]
    pub use crate::backends::html::HtmlWriter;

    #[cfg(feature = "txt")]
    pub use crate::backends::txt::PlainTextWriter;
}
