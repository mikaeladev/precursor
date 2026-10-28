use std::fs;
use std::io::ErrorKind;

use precursor_error::{ErrorMessage, Result};

use crate::macros::{action_denied, action_failed};
use crate::metapath::MetaPath;

/// Removes the file at the provided path.
///
/// See the [`fs::remove_file`] function for more information.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - Path does not exist;
/// - Path is a directory;
/// - User lacks permissions to remove file at path.
///
/// [`fs::remove_file`]: fs::remove_file
pub fn remove_file(MetaPath { path, kind }: MetaPath) -> Result<()> {
  fs::remove_file(path).map_err(|src| {
    let action = format_args!("remove {kind}");

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
