use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Theme {
  pub name: String,
  pub comment: String,
  pub example: Option<String>,
  pub hidden: Option<bool>,
  pub locales: Option<BTreeMap<String, ThemeLocaleValue>>,
}

#[derive(Debug, Deserialize)]
pub struct ThemeLocaleValue {
  pub name: Option<String>,
  pub comment: Option<String>,
}
