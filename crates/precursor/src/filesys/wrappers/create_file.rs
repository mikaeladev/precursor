use std::fs::File;
use std::io::{self, ErrorKind};
use std::path::Path;

use crate::debug;
use crate::path_error_msg;

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
) -> io::Result<File> {
  let path = path.as_ref();

  File::create(path).map_err(|err| {
    let action = if let Some(value) = kind.into() {
      format_args!("create {} file", value.to_string())
    } else {
      format_args!("create file")
    };

    let err_kind = err.kind();

    match err_kind {
      ErrorKind::AlreadyExists => io::Error::new(
        err_kind,
        path_error_msg!(already_exists: "directory", path),
      ),
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
) -> io::Result<File> {
  let path = path.as_ref();

  File::create_new(path).map_err(|err| {
    let action = if let Some(value) = kind.into() {
      format_args!("create new {} file", value.to_string())
    } else {
      format_args!("create new file")
    };

    let err_kind = err.kind();

    match err_kind {
      ErrorKind::AlreadyExists => io::Error::new(
        err_kind,
        path_error_msg!(already_exists: "file or directory", path),
      ),
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
