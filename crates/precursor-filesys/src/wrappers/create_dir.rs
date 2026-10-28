use std::fs;
use std::io::ErrorKind;

use precursor_error::{ErrorMessage, Result};

use crate::macros::{action_denied, action_failed};
use crate::metapath::MetaPath;

/// Creates a new directory at the provided path.
///
/// See the [`fs::create_dir`] function for more information.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - A parent directory in path does not exist;
/// - A file or directory already exists at path;
/// - User lacks permissions to create directory at path.
///
/// [`fs::create_dir`]: fs::create_dir
pub fn create_new_dir(MetaPath { path, kind }: MetaPath) -> Result<()> {
  fs::create_dir(path).map_err(|src| {
    let action = format_args!("create new {kind}");

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
