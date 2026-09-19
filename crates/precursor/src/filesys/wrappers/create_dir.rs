use std::fs;
use std::io::{self, ErrorKind};
use std::path::Path;

use crate::debug;
use crate::path_error_msg;

/// Creates a new directory at the provided `path`.
///
/// See the [`fs::create_dir`] function for more information.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - A parent directory in `path` does not exist;
/// - A file or directory already exists at `path`;
/// - User lacks permissions to create directory at `path`.
///
/// [`fs::create_dir`]: fs::create_dir
pub fn create_new_dir<S: ToString>(
  path: impl AsRef<Path>,
  kind: impl Into<Option<S>>,
) -> io::Result<()> {
  let path = path.as_ref();

  fs::create_dir(path).map_err(|err| {
    let action = if let Some(value) = kind.into() {
      format_args!("create {} directory", value.to_string())
    } else {
      format_args!("create directory")
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
