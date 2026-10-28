use std::fs::{self, ReadDir};
use std::io::ErrorKind;

use precursor_error::{ErrorMessage, Result};

use crate::macros::{action_denied, action_failed};
use crate::metapath::MetaPath;

/// Returns an iterator over the entries within a directory.
///
/// See the [`fs::read_dir`] function for more information.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - Path does not exist;
/// - Path is not a directory;
/// - User lacks permissions to access path.
///
/// [`fs::read_dir`]: fs::read_dir
pub fn read_dir(MetaPath { path, kind }: MetaPath) -> Result<ReadDir> {
  fs::read_dir(path).map_err(|src| {
    let action = format_args!("read {kind}");

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
