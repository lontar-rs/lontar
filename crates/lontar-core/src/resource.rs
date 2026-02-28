//! Resource store for images and binary assets.

use std::collections::HashMap;
use std::path::PathBuf;

/// Source of an image resource
#[derive(Debug, Clone)]
pub enum ImageSource {
    /// Embedded image data (bytes)
    Embedded(Vec<u8>),
    /// Path to image file
    File(PathBuf),
    /// External URL
    Url(String),
}

/// Image metadata
#[derive(Debug, Clone)]
pub struct Image {
    pub source: ImageSource,
    pub alt_text: Option<String>,
    pub width: Option<f64>,              // Inches
    pub height: Option<f64>,             // Inches
    pub mime_type: Option<String>,       // "image/png", "image/jpeg", etc.
}

/// Resource store for managing images and other assets
#[derive(Debug, Clone, Default)]
pub struct ResourceStore {
    images: HashMap<String, Image>,
    next_id: usize,
}

impl ResourceStore {
    /// Create a new empty resource store
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an image resource and return its ID
    pub fn add_image(&mut self, image: Image) -> String {
        let id = format!("image_{}", self.next_id);
        self.next_id += 1;
        self.images.insert(id.clone(), image);
        id
    }

    /// Get an image by ID
    pub fn get_image(&self, id: &str) -> Option<&Image> {
        self.images.get(id)
    }

    /// Get all images
    pub fn images(&self) -> impl Iterator<Item = (&String, &Image)> {
        self.images.iter()
    }

    /// Remove an image by ID
    pub fn remove_image(&mut self, id: &str) -> Option<Image> {
        self.images.remove(id)
    }

    /// Clear all resources
    pub fn clear(&mut self) {
        self.images.clear();
        self.next_id = 0;
    }
}
