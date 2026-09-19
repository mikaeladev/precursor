use std::fs::{self, Metadata};
use std::io::{self, ErrorKind};
use std::path::Path;

use crate::debug;
use crate::path_error_msg;

/// Queries the file system to get information about the `path`.
///
/// See the [`fs::metadata`] function for more information.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - A parent directory in `path` does not exist;
/// - User lacks permissions to access `path`.
///
/// [`fs::metadata`]: fs::metadata
pub fn get_metadata(path: impl AsRef<Path>) -> io::Result<Metadata> {
  let path = path.as_ref();

  fs::metadata(&path).map_err(|err| {
    let err_kind = err.kind();

    match err_kind {
      ErrorKind::NotFound => io::Error::new(
        err_kind,
        path_error_msg!(not_found: "file or directory", path),
      ),
      ErrorKind::PermissionDenied => {
        io::Error::new(err_kind, path_error_msg!(access_denied: path))
      }
      _ => {
        debug!("{err}");
        io::Error::new(err_kind, path_error_msg!(access_failed: path))
      }
    }
  })
}
