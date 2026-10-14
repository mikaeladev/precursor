use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use precursor_error::{Error, Result};

use crate::error::ErrorMessageWithPath;
use crate::macros::{action_denied, action_failed};

/// Creates a new directory at the provided `path`.
///
/// See the [`fs::create_dir`] function for more information.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - A parent directory in `path` does not exist;
/// - A file or directory already exists at `path`;
/// - User lacks permissions to create directory at `path`.
///
/// [`fs::create_dir`]: fs::create_dir
pub fn create_new_dir<S: ToString>(
  path: impl AsRef<Path>,
  kind: impl Into<Option<S>>,
) -> Result<()> {
  let path = path.as_ref();

  fs::create_dir(path).map_err(|src| {
    let action = if let Some(value) = kind.into() {
      format_args!("create {} directory", value.to_string())
    } else {
      format_args!("create directory")
    };

    let src_kind = src.kind();

    let msg = match src_kind {
      ErrorKind::PermissionDenied => action_denied!(action),
      // ErrorKind::AlreadyExists => action_failed!(action, src_kind),
      _ => action_failed!(action, src_kind),
    };

    Error::new(ErrorMessageWithPath::new(msg, path), src)
  })
}
