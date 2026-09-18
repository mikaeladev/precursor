use std::fs;
use std::io;
use std::path::Path;

use crate::debug;
use crate::path_error_msg;

/// Removes the file at the provided `path`.
///
/// See the [`fs::remove_file`] function for more information.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - `path` does not exist.
/// - `path` is a directory.
/// - User lacks permissions to remove file at `path`.
///
/// [`fs::remove_file`]: fs::remove_file
pub fn remove_file<S: Into<String>>(
  path: impl AsRef<Path>,
  kind: impl Into<Option<S>>,
) -> io::Result<()> {
  use io::ErrorKind::{IsADirectory, NotFound, PermissionDenied};

  let path = path.as_ref();

  if let Err(err) = fs::remove_file(path) {
    let kind = if let Some(value) = kind.into() {
      format_args!("{} file", value.into())
    } else {
      format_args!("file")
    };

    let action = format_args!("remove {kind}");

    let err_kind = err.kind();

    Err(match err_kind {
      IsADirectory => io::Error::new(
        err_kind,
        path_error_msg!(expected_found: "a file", "a directory", path),
      ),
      NotFound => {
        io::Error::new(err_kind, path_error_msg!(not_found: kind, path))
      }
      PermissionDenied => {
        io::Error::new(err_kind, path_error_msg!(action_denied: action, path))
      }
      _ => {
        debug!("{err}");

        io::Error::new(err_kind, path_error_msg!(action_failed: action, path))
      }
    })
  } else {
    Ok(())
  }
}

/// Removes the directory at the provided `path`.
///
/// See the [`fs::remove_dir`] function for more information.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - `path` does not exist.
/// - `path` is not a directory.
/// - Directory is not empty.
/// - User lacks permissions to remove directory at `path`.
///
/// [`fs::remove_dir`]: fs::remove_dir
pub fn remove_dir<S: Into<String>>(
  path: impl AsRef<Path>,
  kind: impl Into<Option<S>>,
) -> io::Result<()> {
  use io::ErrorKind::{
    DirectoryNotEmpty, NotADirectory, NotFound, PermissionDenied,
  };

  let path = path.as_ref();

  if let Err(err) = fs::remove_dir(path) {
    let kind = if let Some(value) = kind.into() {
      format_args!("{} directory", value.into())
    } else {
      format_args!("directory")
    };

    let action = format_args!("remove {kind}");

    let err_kind = err.kind();

    Err(match err_kind {
      DirectoryNotEmpty => io::Error::new(
        err_kind,
        path_error_msg!("directory is not empty at path", path),
      ),
      NotADirectory => io::Error::new(
        err_kind,
        path_error_msg!(expected_found: "a directory", "a file", path),
      ),
      NotFound => {
        io::Error::new(err_kind, path_error_msg!(not_found: kind, path))
      }
      PermissionDenied => {
        io::Error::new(err_kind, path_error_msg!(action_denied: action, path))
      }
      _ => {
        debug!("{err}");

        io::Error::new(err_kind, path_error_msg!(action_failed: action, path))
      }
    })
  } else {
    Ok(())
  }
}

/// Removes the directory at the provided `path`, after removing all of its
/// contents.
///
/// See the [`fs::remove_dir_all`] function for more information.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - `path` does not exist.
/// - `path` is not a directory.
/// - Directory is being concurrently written to.
/// - User lacks permissions to remove directory at `path`.
///
/// [`fs::remove_dir_all`]: fs::remove_dir_all
pub fn remove_dir_all<S: Into<String>>(
  path: impl AsRef<Path>,
  kind: impl Into<Option<S>>,
) -> io::Result<()> {
  use io::ErrorKind::{
    DirectoryNotEmpty, NotADirectory, NotFound, PermissionDenied,
  };

  let path = path.as_ref();

  if let Err(err) = fs::remove_dir_all(path) {
    let kind = if let Some(value) = kind.into() {
      format_args!("{} directory", value.into())
    } else {
      format_args!("directory")
    };

    let action = format_args!("remove {kind} and its contents");

    let err_kind = err.kind();

    Err(match err_kind {
      DirectoryNotEmpty => io::Error::new(
        err_kind,
        path_error_msg!(
          format_args!(
            "failed to remove {} as it is being concurrently written to at path",
            kind
          ),
          path
        ),
      ),
      NotADirectory => io::Error::new(
        err_kind,
        path_error_msg!(expected_found: "a directory", "a file", path),
      ),
      NotFound => {
        io::Error::new(err_kind, path_error_msg!(not_found: kind, path))
      }
      PermissionDenied => {
        io::Error::new(err_kind, path_error_msg!(action_denied: action, path))
      }
      _ => {
        debug!("{err}");

        io::Error::new(err_kind, path_error_msg!(action_failed: action, path))
      }
    })
  } else {
    Ok(())
  }
}
