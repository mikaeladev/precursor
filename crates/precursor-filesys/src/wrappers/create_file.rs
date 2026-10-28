use std::fs::File;
use std::io::ErrorKind;

use precursor_error::{ErrorMessage, Result};

use crate::macros::{action_denied, action_failed};
use crate::metapath::MetaPath;

/// Creates or overwrites the file at the provided path.
///
/// See the [`File::create`] method for more information.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - A parent directory in path does not exist;
/// - A directory already exists at path;
/// - User lacks permissions to create file at path.
pub fn create_file(MetaPath { path, kind }: MetaPath) -> Result<File> {
  File::create(path).map_err(|src| {
    let action = format_args!("create {kind}");

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

/// Creates a new file at the provided path.
///
/// See the [`File::create_new`] method for more information.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - A parent directory in path does not exist;
/// - A file or directory already exists at path;
/// - User lacks permissions to create file at path.
pub fn create_new_file(MetaPath { path, kind }: MetaPath) -> Result<File> {
  File::create_new(path).map_err(|src| {
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
