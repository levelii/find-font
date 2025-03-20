use crate::models::font_descriptor::FontDescriptor;
use font_kit::source::SystemSource;
use napi::bindgen_prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct FontService {
  source: SystemSource,
  font_cache: Arc<Mutex<HashMap<String, Vec<FontDescriptor>>>>,
}

impl FontService {
  pub fn new() -> Self {
    Self {
      source: SystemSource::new(),
      font_cache: Arc::new(Mutex::new(HashMap::new())),
    }
  }

  pub async fn all_families(&self) -> Result<Vec<String>> {
    self
      .source
      .all_families()
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  pub async fn get_family_variants(&self, family_name: String) -> Result<Vec<FontDescriptor>> {
    let mut cache = self.font_cache.lock().await;

    // Check if the font family is already in cache
    if let Some(cached_fonts) = cache.get(&family_name) {
      return Ok(cached_fonts.clone());
    }

    let mut fonts = Vec::new();

    // Load fonts from system
    match self.source.select_family_by_name(&family_name) {
      Ok(family_handles) => {
        for font_handle in family_handles.fonts() {
          match font_handle.load() {
            Ok(font) => {
              fonts.push(FontDescriptor::from_font(&font));
            }
            Err(_e) => {}
          }
        }

        // Cache all fonts of the family
        cache.insert(family_name, fonts.clone());
        Ok(fonts)
      }
      Err(e) => Err(Error::from_reason(e.to_string())),
    }
  }
}
