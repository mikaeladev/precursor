use std::fs::File;
use std::io::{ErrorKind, Read};

use precursor_error::{ErrorMessage, Result};

use crate::macros::{action_denied, action_failed};
use crate::metapath::MetaPath;

/// Opens a file in read-only mode.
///
/// See the [`File::open`] method for more information.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - Path does not exist;
/// - Path is a directory;
/// - User lacks permissions to read file at path.
pub fn read_file(MetaPath { path, kind }: MetaPath) -> Result<File> {
  File::open(&path).map_err(|src| {
    let action = format_args!("open {kind}");

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

/// Reads all bytes in a file and returns the contents as a [`String`].
///
/// See the [`File::read_to_string`] method for more information.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - Path does not exist;
/// - Path is a directory;
/// - User lacks permissions to read file at path;
/// - File content is not valid UTF-8.
pub fn read_file_to_string(meta_path: MetaPath) -> Result<String> {
  let MetaPath { path, kind } = meta_path;

  let mut file = crate::read_file(meta_path)?;
  let capacity = crate::get_metadata(path, true)?.len() as usize;

  let mut buffer = String::with_capacity(capacity);

  file.read_to_string(&mut buffer).map_err(|src| {
    action_failed!(format_args!("read {} to string", kind), src.kind())
      .with_context([("path", path.display())])
      .into_error()
      .with_source(src)
  })?;

  Ok(buffer)
}
