use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum InputArg {
  Path(PathBuf),
  #[cfg(target_family = "unix")]
  Stdin,
}

#[cfg(target_family = "unix")]
impl Default for InputArg {
  fn default() -> Self {
    Self::Stdin
  }
}

impl From<OsString> for InputArg {
  fn from(value: OsString) -> Self {
    #[cfg(target_family = "unix")]
    if value == "-" {
      return Self::Stdin;
    }

    Self::Path(PathBuf::from(value))
  }
}
