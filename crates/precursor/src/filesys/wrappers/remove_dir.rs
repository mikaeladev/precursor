use std::fs;
use std::io::{self, ErrorKind};
use std::path::Path;

use crate::debug;
use crate::path_error_msg;

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
///
/// [`fs::remove_dir`]: fs::remove_dir
pub fn remove_dir<S: ToString>(
  path: impl AsRef<Path>,
  kind: impl Into<Option<S>>,
) -> io::Result<()> {
  let path = path.as_ref();

  fs::remove_dir(path).map_err(|err| {
    let kind = if let Some(value) = kind.into() {
      format_args!("{} directory", value.to_string())
    } else {
      format_args!("directory")
    };

    let action = format_args!("remove {kind}");

    let err_kind = err.kind();

    match err_kind {
      ErrorKind::DirectoryNotEmpty => io::Error::new(
        err_kind,
        path_error_msg!("directory is not empty at path", path),
      ),
      ErrorKind::NotADirectory => io::Error::new(
        err_kind,
        path_error_msg!(expected_found: "a directory", "a file", path),
      ),
      ErrorKind::NotFound => {
        io::Error::new(err_kind, path_error_msg!(not_found: kind, path))
      }
      ErrorKind::PermissionDenied => {
        io::Error::new(err_kind, path_error_msg!(action_denied: action, path))
      }
      _ => {
        debug!("{err}");

        io::Error::new(err_kind, path_error_msg!(action_failed: action, path))
      }
    }
  })
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
/// - `path` does not exist;
/// - `path` is not a directory;
/// - Directory is being concurrently written to;
/// - User lacks permissions to remove directory at `path`.
///
/// [`fs::remove_dir_all`]: fs::remove_dir_all
pub fn remove_dir_all<S: ToString>(
  path: impl AsRef<Path>,
  kind: impl Into<Option<S>>,
) -> io::Result<()> {
  let path = path.as_ref();

  fs::remove_dir_all(path).map_err(|err| {
    let kind = if let Some(value) = kind.into() {
      format_args!("{} directory", value.to_string())
    } else {
      format_args!("directory")
    };

    let action = format_args!("remove {kind} and its contents");

    let err_kind = err.kind();

    match err_kind {
      ErrorKind::DirectoryNotEmpty => io::Error::new(
        err_kind,
        path_error_msg!(
          format_args!(
            "failed to {} as it is being concurrently written to at path",
            action
          ),
          path
        ),
      ),
      ErrorKind::NotADirectory => io::Error::new(
        err_kind,
        path_error_msg!(expected_found: "a directory", "a file", path),
      ),
      ErrorKind::NotFound => {
        io::Error::new(err_kind, path_error_msg!(not_found: kind, path))
      }
      ErrorKind::PermissionDenied => {
        io::Error::new(err_kind, path_error_msg!(action_denied: action, path))
      }
      _ => {
        debug!("{err}");

        io::Error::new(err_kind, path_error_msg!(action_failed: action, path))
      }
    }
  })
}
