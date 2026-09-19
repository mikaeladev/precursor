use std::fs;
use std::io::{self, ErrorKind};
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
pub fn remove_file<S: ToString>(
  path: impl AsRef<Path>,
  kind: impl Into<Option<S>>,
) -> io::Result<()> {
  let path = path.as_ref();

  fs::remove_file(path).map_err(|err| {
    let kind = if let Some(value) = kind.into() {
      format_args!("{} file", value.to_string())
    } else {
      format_args!("file")
    };

    let action = format_args!("remove {kind}");

    let err_kind = err.kind();

    match err_kind {
      ErrorKind::IsADirectory => io::Error::new(
        err_kind,
        path_error_msg!(expected_found: "a file", "a directory", path),
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
