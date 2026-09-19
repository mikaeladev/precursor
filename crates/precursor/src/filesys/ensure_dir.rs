use std::convert::Infallible;
use std::io::{self, ErrorKind};
use std::path::Path;

use crate::debug;
use crate::path_error_msg;

/// Ensures a directory exists at the provided `path`, creating it if necessary.
///
/// Returns `true` if a new directory was created, and `false` otherwise.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - A parent directory in `path` does not exist;
/// - `path` exists and is not a directory;
/// - User lacks permissions to create directory at `path`.
pub fn ensure_dir<S: ToString>(
  path: impl AsRef<Path>,
  kind: impl Into<Option<S>>,
) -> io::Result<bool> {
  match super::create_new_dir(path, kind) {
    Ok(_) => Ok(true),
    Err(err) if err.kind() == ErrorKind::AlreadyExists => Ok(false),
    Err(err) => Err(err),
  }
}

/// Ensures a directory exists and is empty at the provided `path`, creating it
/// or removing its contents if necessary.
///
/// Returns `true` if a new directory was created, and `false` otherwise.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - A parent directory in `path` does not exist;
/// - `path` exists and is not a directory;
/// - `path` is being concurrently written to;
/// - User lacks permissions to create directory at `path`;
/// - User lacks permissions to access and remove contents of `path`.
pub fn ensure_dir_empty<S: ToString>(
  path: impl AsRef<Path>,
  kind: impl Into<Option<S>> + Copy,
) -> io::Result<bool> {
  let path = path.as_ref();

  if !ensure_dir(path, kind)? {
    let mut iter = super::read_dir(path, kind)?;

    while let Some(entry_res) = iter.next() {
      match entry_res {
        Ok(entry) => {
          let subpath = entry.path();
          let metadata = super::get_metadata(&subpath)?;

          if metadata.is_file() || metadata.is_symlink() {
            super::remove_file::<Infallible>(subpath, None)?;
          } else if metadata.is_dir() {
            super::remove_dir_all::<Infallible>(subpath, None)?;
          }
        }
        Err(err) => {
          debug!("{err}");

          return Err(io::Error::new(
            err.kind(),
            path_error_msg!(action_failed: "read directory", path),
          ));
        }
      }
    }

    Ok(false)
  } else {
    Ok(true)
  }
}
