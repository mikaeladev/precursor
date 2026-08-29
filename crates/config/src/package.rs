use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct PackageConfig {
  pub name: String,
  pub comment: String,
  pub example: Option<String>,
  pub hidden: Option<bool>,
  pub locales: Option<BTreeMap<String, PackageLocaleConfig>>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct PackageLocaleConfig {
  pub name: Option<String>,
  pub comment: Option<String>,
}
