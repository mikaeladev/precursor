use std::fs::{self, ReadDir};
use std::io::ErrorKind;
use std::path::Path;

use precursor_error::{ErrorMessage, Result};

use crate::macros::{action_denied, action_failed};

/// Returns an iterator over the entries within a directory.
///
/// See the [`fs::read_dir`] function for more information.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - `path` does not exist;
/// - `path` is not a directory;
/// - User lacks permissions to access `path`.
///
/// [`fs::read_dir`]: fs::read_dir
pub fn read_dir<S: ToString>(
  path: impl AsRef<Path>,
  kind: impl Into<Option<S>>,
) -> Result<ReadDir> {
  let path = path.as_ref();

  fs::read_dir(path).map_err(|src| {
    let action = if let Some(value) = kind.into() {
      format_args!("read {} directory", value.to_string())
    } else {
      format_args!("read directory")
    };

    let src_kind = src.kind();

    match src_kind {
      ErrorKind::PermissionDenied => action_denied!(action),
      _ => action_failed!(action, src_kind),
    }
    .with_context([("path", path.display())])
    .into_error()
    .with_source(src)
  })
}
