use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use precursor_error::{Error, Result};

use crate::error::ErrorMessageWithPath;
use crate::macros::{action_denied, action_failed};

/// Removes the directory at the provided `path`.
///
/// See the [`fs::remove_dir`] function for more information.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - `path` does not exist;
/// - `path` is not a directory;
/// - Directory is not empty;
/// - User lacks permissions to remove directory at `path`.
pub fn remove_dir<S: ToString>(
  path: impl AsRef<Path>,
  kind: impl Into<Option<S>>,
) -> Result<()> {
  let path = path.as_ref();

  fs::remove_dir(path).map_err(|src| {
    let kind = if let Some(value) = kind.into() {
      format_args!("{} directory", value.to_string())
    } else {
      format_args!("directory")
    };

    let action = format_args!("remove {kind}");

    let src_kind = src.kind();

    let msg = match src_kind {
      ErrorKind::PermissionDenied => action_denied!(action),
      // ErrorKind::NotFound => action_failed!(action, src_kind),
      // ErrorKind::NotADirectory => action_failed!(action, src_kind),
      // ErrorKind::DirectoryNotEmpty => action_failed!(action, src_kind),
      _ => action_failed!(action, src_kind),
    };

    Error::new(ErrorMessageWithPath::new(msg, path), src)
  })
}

/// Removes the directory at the provided `path`, after removing all of its
/// contents.
///
/// This function does **not** follow symbolic links and it will simply remove
/// the symbolic link itself.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - `path` does not exist;
/// - `path` is not a directory;
/// - Directory is being concurrently written to;
/// - User lacks permissions to access and remove contents of `path`.
/// - User lacks permissions to remove directory at `path`.
pub fn remove_dir_all<S: ToString>(
  path: impl AsRef<Path>,
  kind: impl Into<Option<S>> + Clone,
) -> Result<()> {
  let path = path.as_ref();
  let path_meta = super::get_metadata(path, true)?;

  if path_meta.is_symlink() {
    let kind = if let Some(value) = kind.into() {
      format_args!("{} symlink", value.to_string())
    } else {
      format_args!("symlink")
    };

    // maybe handle behind a flag at a later date?
    super::remove_file(path, kind)
  } else {
    super::empty_dir(path, kind.clone())?;

    remove_dir(path, kind)
  }
}
