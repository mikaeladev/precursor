use std::fs;
use std::io::ErrorKind;

use precursor_error::{ErrorMessage, Result};

use crate::macros::{action_denied, action_failed};
use crate::metapath::MetaPath;

/// Removes the directory at the provided path.
///
/// See the [`fs::remove_dir`] function for more information.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - Path does not exist;
/// - Path is not a directory;
/// - Directory is not empty;
/// - User lacks permissions to remove directory at path.
pub fn remove_dir(MetaPath { path, kind }: MetaPath) -> Result<()> {
  fs::remove_dir(path).map_err(|src| {
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

/// Removes the directory at the provided path, after removing all of its
/// contents.
///
/// This function does **not** follow symbolic links and will simply remove any
/// symlinks it comes across.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - Path does not exist;
/// - Path is not a directory;
/// - Directory is being concurrently written to;
/// - User lacks permissions to remove contents of path;
/// - User lacks permissions to remove directory at path.
pub fn remove_dir_all(meta_path: MetaPath) -> Result<()> {
  let metadata = crate::get_metadata(meta_path.path, true)?;

  if metadata.is_symlink() {
    // maybe handle behind a flag at a later date?
    crate::remove_file(meta_path)
  } else {
    crate::empty_dir(meta_path)?;
    remove_dir(meta_path)
  }
}
