use font_kit::properties::{Properties, Stretch, Style, Weight};
use napi_derive::napi;

#[napi(object)]
#[derive(Default, Clone)]
pub struct FontDescriptor {
  pub postscript_name: Option<String>,
  pub family: Option<String>,
  pub style: Option<String>,
  pub weight: Option<i32>,
  pub width: Option<i32>,
  pub italic: Option<bool>,
  pub monospace: Option<bool>,
}

impl FontDescriptor {
  pub fn to_properties(&self) -> Properties {
    let mut props = Properties::new();

    if let Some(weight) = self.weight {
      props.weight = match weight {
        100 => Weight::THIN,
        200 => Weight::EXTRA_LIGHT,
        300 => Weight::LIGHT,
        400 => Weight::NORMAL,
        500 => Weight::MEDIUM,
        600 => Weight::SEMIBOLD,
        700 => Weight::BOLD,
        800 => Weight::EXTRA_BOLD,
        900 => Weight::BLACK,
        _ => Weight::NORMAL,
      };
    }

    if let Some(italic) = self.italic {
      props.style = if italic { Style::Italic } else { Style::Normal };
    }

    if let Some(width) = self.width {
      props.stretch = Stretch(match width {
        1 => 0.5,   // ULTRA_CONDENSED
        2 => 0.625, // EXTRA_CONDENSED
        3 => 0.75,  // CONDENSED
        4 => 0.875, // SEMI_CONDENSED
        5 => 1.0,   // NORMAL
        6 => 1.125, // SEMI_EXPANDED
        7 => 1.25,  // EXPANDED
        8 => 1.5,   // EXTRA_EXPANDED
        9 => 2.0,   // ULTRA_EXPANDED
        _ => 1.0,   // Default to NORMAL
      });
    }

    props
  }

  pub fn from_font(font: &font_kit::font::Font) -> Self {
    let properties = font.properties();
    let stretch_value = properties.stretch.0;
    Self {
      postscript_name: font.postscript_name(),
      family: Some(font.family_name()),
      style: Some(properties.style.to_string()),
      weight: Some(properties.weight.0 as i32),
      width: Some(if stretch_value <= 0.5 {
        1 // ULTRA_CONDENSED
      } else if stretch_value <= 0.625 {
        2 // EXTRA_CONDENSED
      } else if stretch_value <= 0.75 {
        3 // CONDENSED
      } else if stretch_value <= 0.875 {
        4 // SEMI_CONDENSED
      } else if stretch_value <= 1.0 {
        5 // NORMAL
      } else if stretch_value <= 1.125 {
        6 // SEMI_EXPANDED
      } else if stretch_value <= 1.25 {
        7 // EXPANDED
      } else if stretch_value <= 1.5 {
        8 // EXTRA_EXPANDED
      } else {
        9 // ULTRA_EXPANDED
      }),
      italic: Some(properties.style == Style::Italic),
      monospace: Some(font.is_monospace()),
    }
  }
}
