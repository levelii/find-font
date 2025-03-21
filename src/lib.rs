#![deny(clippy::all)]

mod models;
mod services;
#[cfg(test)]
mod tests;

use models::font_descriptor::FontDescriptor;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use services::font_service::FontService;
use std::sync::Arc;

#[napi]
pub async fn all_families() -> Result<Vec<String>> {
  let service = Arc::new(FontService::new());
  service.all_families().await
}

#[napi]
pub async fn get_family_variants(family_name: String) -> Result<Vec<FontDescriptor>> {
  let service = Arc::new(FontService::new());
  service.get_family_variants(family_name).await
}
