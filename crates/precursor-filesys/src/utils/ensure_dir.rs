use std::io::{self, ErrorKind};

use precursor_error::Result;

use crate::metapath::MetaPath;

/// Ensures a directory exists at the provided path, creating it if necessary.
///
/// Returns `true` if a new directory was created, and `false` otherwise.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - A parent directory in path does not exist;
/// - Path exists and is not a directory;
/// - User lacks permissions to create directory at path.
pub fn ensure_dir(meta_path: MetaPath) -> Result<bool> {
  match crate::create_new_dir(meta_path) {
    Ok(_) => Ok(true),
    Err(err)
      if let Some(src) = err.downcast::<io::Error>()
        && src.kind() == ErrorKind::AlreadyExists =>
    {
      Ok(false)
    }
    Err(err) => Err(err),
  }
}

/// Ensures a directory exists and is empty at the provided path, creating it or
/// removing its contents if necessary.
///
/// Returns `true` if a new directory was created, and `false` otherwise.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - A parent directory in path does not exist;
/// - Path exists and is not a directory;
/// - Path is being concurrently written to;
/// - User lacks permissions to remove contents of path.
/// - User lacks permissions to create directory at path;
pub fn ensure_dir_empty(meta_path: MetaPath) -> Result<bool> {
  if !ensure_dir(meta_path)? {
    crate::empty_dir(meta_path)?;
    Ok(false)
  } else {
    Ok(true)
  }
}
