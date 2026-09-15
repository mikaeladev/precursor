use std::fs::{self, Metadata};
use std::io;
use std::path::Path;

use crate::{debug, path_error_msg};

/// Queries the file system to get information about the `path`.
///
/// See the [`metadata`] function for more information.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - User lacks permissions to access the `path`.
/// - A parent directory in `path` doesn't exist.
///
/// [`fs::metadata`]: fs::metadata
pub fn get_metadata(path: impl AsRef<Path>) -> io::Result<Metadata> {
  let path = path.as_ref();

  let metadata_result = fs::metadata(&path);

  if let Err(err) = metadata_result {
    use io::ErrorKind::{NotFound, PermissionDenied};

    let err_kind = err.kind();

    Err(match err_kind {
      NotFound => io::Error::new(
        err_kind,
        path_error_msg!(not_found: "file or directory", path),
      ),
      PermissionDenied => {
        io::Error::new(err_kind, path_error_msg!(access_denied: path))
      }
      _ => {
        debug!("{err}");
        io::Error::new(err_kind, path_error_msg!(access_failed: path))
      }
    })
  } else {
    metadata_result
  }
}
