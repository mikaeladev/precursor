use std::fs::{self, Metadata};
use std::io::ErrorKind;
use std::path::Path;

use precursor_error::{ErrorMessage, Result};

use crate::macros::{action_denied, action_failed};

/// Queries the file system for information about the `path`.
///
/// This function will only traverse symlinks when `follow_symlinks` is `true`.
///
/// See the [`fs::metadata`] function for more information.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - A parent directory in `path` does not exist;
/// - User lacks permissions to access `path`.
///
/// [`fs::metadata`]: fs::metadata
pub fn get_metadata(
  path: impl AsRef<Path>,
  follow_symlinks: bool,
) -> Result<Metadata> {
  let path = path.as_ref();

  let metadata = if follow_symlinks {
    fs::metadata(&path)
  } else {
    fs::symlink_metadata(&path)
  };

  metadata.map_err(|src| {
    let action = "query metadata";

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
