use std::fs::{self, ReadDir};
use std::io::{self, ErrorKind};
use std::path::Path;

use crate::debug;
use crate::path_error_msg;

/// Returns an iterator over the entries within a directory.
///
/// See the [`fs::read_dir`] function for more information.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - `path` does not exist;
/// - `path` is not a directory;
/// - User lacks permissions to access `path`.
///
/// [`fs::read_dir`]: fs::read_dir
pub fn read_dir<S: ToString>(
  path: impl AsRef<Path>,
  kind: impl Into<Option<S>>,
) -> io::Result<ReadDir> {
  let path = path.as_ref();

  fs::read_dir(path).map_err(|err| {
    let kind = if let Some(value) = kind.into() {
      format_args!("{} directory", value.to_string())
    } else {
      format_args!("directory")
    };

    let action = format_args!("read {kind}");

    let err_kind = err.kind();

    match err_kind {
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
