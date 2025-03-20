#[cfg(test)]
mod tests {
  use crate::services::font_service::FontService;

  // Test getting all font families
  #[tokio::test]
  async fn test_all_families() {
    let service = FontService::new();
    let families = service.all_families().await.unwrap();

    // Verify that the returned font family list is not empty
    assert!(!families.is_empty(), "Should find at least one font family");

    // Verify that font family names are not empty
    for family in families {
      assert!(!family.is_empty(), "Font family name should not be empty");
    }
  }

  // Test getting all fonts for a specific font family
  #[tokio::test]
  async fn test_get_font() {
    let service = FontService::new();

    // First get a known existing font family
    let families = service.all_families().await.unwrap();
    let test_family = families[0].clone();

    // Test getting all fonts for this family
    let fonts = service
      .get_family_variants(test_family.clone())
      .await
      .unwrap();
    assert!(
      !fonts.is_empty(),
      "Should find at least one font for the family"
    );

    // Verify that returned fonts belong to the specified family
    for font in fonts {
      assert_eq!(
        font.family.as_ref().unwrap(),
        &test_family,
        "Font should belong to the specified family"
      );
    }
  }

  // Test getting a nonexistent font family
  #[tokio::test]
  async fn test_get_nonexistent_font() {
    let service = FontService::new();
    let result = service
      .get_family_variants("NonexistentFont".to_string())
      .await;

    // Verify that it returns an error
    assert!(
      result.is_err(),
      "Should return error for nonexistent font family"
    );
  }
}
