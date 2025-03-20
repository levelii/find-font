#![deny(clippy::all)]

mod models;
mod services;
#[cfg(test)]
mod tests;

use models::font_descriptor::FontDescriptor;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use services::font_service::FontService;

#[napi]
pub async fn all_families() -> Result<Vec<String>> {
  let service = FontService::new();
  service.all_families().await
}

#[napi]
pub async fn get_family_variants(family_name: String) -> Result<Vec<FontDescriptor>> {
  let service = FontService::new();
  service.get_family_variants(family_name).await
}
