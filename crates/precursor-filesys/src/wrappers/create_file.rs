use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;

use precursor_error::{ErrorMessage, Result};

use crate::macros::{action_denied, action_failed};

/// Creates a file at the provided `path`.
///
/// See the [`File::create`] method for more information.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - A parent directory in `path` does not exist;
/// - A directory already exists at `path`;
/// - User lacks permissions to create file at `path`.
pub fn create_file<S: ToString>(
  path: impl AsRef<Path>,
  kind: impl Into<Option<S>>,
) -> Result<File> {
  let path = path.as_ref();

  File::create(path).map_err(|src| {
    let action = if let Some(value) = kind.into() {
      format_args!("create {} file", value.to_string())
    } else {
      format_args!("create file")
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

/// Creates a new file at the provided `path`.
///
/// See the [`File::create_new`] method for more information.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - A parent directory in `path` does not exist;
/// - A file or directory already exists at `path`;
/// - User lacks permissions to create file at `path`.
pub fn create_new_file<S: ToString>(
  path: impl AsRef<Path>,
  kind: impl Into<Option<S>>,
) -> Result<File> {
  let path = path.as_ref();

  File::create_new(path).map_err(|src| {
    let action = if let Some(value) = kind.into() {
      format_args!("create new {} file", value.to_string())
    } else {
      format_args!("create new file")
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
