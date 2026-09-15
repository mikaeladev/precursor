use std::fs;
use std::io;
use std::path::Path;

use crate::{debug, path_error_msg};

use super::EntityKind;

/// Creates a new directory at the provided `dir_path`.
///
/// See the [`create_dir`] function for more information.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - User lacks permissions to access or create directory at `dir_path`.
/// - A file or directory already exists at `dir_path`.
/// - A parent directory in `dir_path` doesn't exist.
///
/// [`create_dir`]: fs::create_dir
pub fn create_new_dir<S: Into<String>>(
  dir_path: impl AsRef<Path>,
  dir_kind: impl Into<Option<S>>,
) -> io::Result<()> {
  use io::ErrorKind::{AlreadyExists, PermissionDenied};

  let dir_path = dir_path.as_ref();

  if let Err(err) = fs::create_dir(dir_path) {
    let action = if let Some(kind) = dir_kind.into() {
      format_args!("create {} directory", kind.into())
    } else {
      format_args!("create directory")
    };

    let err_kind = err.kind();

    Err(match err_kind {
      AlreadyExists => io::Error::new(
        err_kind,
        path_error_msg!(already_exists: "file or directory", dir_path),
      ),
      PermissionDenied => io::Error::new(
        err_kind,
        path_error_msg!(action_denied: action, dir_path),
      ),
      _ => {
        debug!("{err}");

        io::Error::new(
          err_kind,
          path_error_msg!(action_failed: action, dir_path),
        )
      }
    })
  } else {
    Ok(())
  }
}

/// Ensures a directory exists at the provided `dir_path`, creating it if
/// necessary.
///
/// Returns `true` if a new directory was created, and `false` otherwise.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - User lacks permissions to access or create directory at `dir_path`.
/// - A parent directory in `dir_path` doesn't exist.
pub fn ensure_dir<S: Into<String>>(
  dir_path: impl AsRef<Path>,
  dir_kind: impl Into<Option<S>>,
) -> io::Result<bool> {
  if !super::entity_exists(EntityKind::Directory, &dir_path)? {
    create_new_dir(dir_path, dir_kind)?;
    Ok(true)
  } else {
    Ok(false)
  }
}
