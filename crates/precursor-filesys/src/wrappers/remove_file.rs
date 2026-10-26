use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use precursor_error::{ErrorMessage, Result};

use crate::macros::{action_denied, action_failed};

/// Removes the file at the provided `path`.
///
/// See the [`fs::remove_file`] function for more information.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - `path` does not exist.
/// - `path` is a directory.
/// - User lacks permissions to remove file at `path`.
///
/// [`fs::remove_file`]: fs::remove_file
pub fn remove_file<S: ToString>(
  path: impl AsRef<Path>,
  kind: impl Into<Option<S>>,
) -> Result<()> {
  let path = path.as_ref();

  fs::remove_file(path).map_err(|src| {
    let action = if let Some(value) = kind.into() {
      format_args!("remove {} file", value.to_string())
    } else {
      format_args!("remove file")
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
