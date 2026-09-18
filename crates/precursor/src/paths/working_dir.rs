use std::env;
use std::io;
use std::path::PathBuf;

use crate::debug;

/// Returns the path to the current working directory.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - Current directory doesn't exist.
/// - User lacks permissions to access the current directory.
pub fn get_working_dir_path() -> io::Result<PathBuf> {
  let working_dir_res = env::current_dir();

  if let Err(err) = working_dir_res {
    use io::ErrorKind::{NotFound, PermissionDenied};

    let err_kind = err.kind();

    Err(match err_kind {
      NotFound => io::Error::new(err_kind, "current directory does not exist"),
      PermissionDenied => io::Error::new(
        err_kind,
        "insufficient permissions to access current directory",
      ),
      _ => {
        debug!("{err}");
        io::Error::new(err_kind, "failed to access current directory")
      }
    })
  } else {
    working_dir_res
  }
}
