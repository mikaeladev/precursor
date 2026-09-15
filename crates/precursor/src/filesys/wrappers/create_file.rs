use std::fs::File;
use std::io;
use std::path::Path;

use crate::{debug, path_error_msg};

/// Creates a new file at the provided `file_path`.
///
/// See the [`File::create_new`] method for more information.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - User lacks permissions to access or create file at `file_path`.
/// - A file or directory already exists at `file_path`.
/// - A parent directory in `file_path` doesn't exist.
///
/// [`File::create_new`]: File::create_new
pub fn create_new_file<S: Into<String>>(
  file_path: impl AsRef<Path>,
  file_kind: impl Into<Option<S>>,
) -> io::Result<File> {
  use io::ErrorKind::{AlreadyExists, PermissionDenied};

  let file_path = file_path.as_ref();

  let create_result = File::create_new(file_path);

  if let Err(err) = create_result {
    let action = if let Some(kind) = file_kind.into() {
      format_args!("create {} file", kind.into())
    } else {
      format_args!("create file")
    };

    let err_kind = err.kind();

    Err(match err_kind {
      AlreadyExists => io::Error::new(
        err_kind,
        path_error_msg!(already_exists: "file or directory", file_path),
      ),
      PermissionDenied => io::Error::new(
        err_kind,
        path_error_msg!(action_denied: action, file_path),
      ),
      _ => {
        debug!("{err}");

        io::Error::new(
          err_kind,
          path_error_msg!(action_failed: action, file_path),
        )
      }
    })
  } else {
    create_result
  }
}
