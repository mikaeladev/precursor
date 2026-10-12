use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
  #[error("{0}")]
  IoError(#[from] std::io::Error),

  #[error("{0}")]
  ParseError(#[from] toml::de::Error),
}
