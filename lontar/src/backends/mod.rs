#[cfg(feature = "docx")]
pub mod docx;

#[cfg(feature = "pptx")]
pub mod pptx;

#[cfg(feature = "pdf")]
pub mod pdf;

#[cfg(feature = "xlsx")]
pub mod xlsx;

#[cfg(feature = "md")]
pub mod md;

#[cfg(feature = "html")]
pub mod html;

#[cfg(feature = "txt")]
pub mod txt;
