use std::env;
use std::io;
use std::path::PathBuf;

use crate::debug;

pub fn get_working_dir_path() -> io::Result<PathBuf> {
  let working_dir_res = env::current_dir();

  if let Err(err) = working_dir_res {
    use io::ErrorKind::{NotFound, PermissionDenied};

    let err_kind = err.kind();
    match err_kind {
      NotFound => {
        Err(io::Error::new(err_kind, "current directory does not exist"))
      }
      PermissionDenied => Err(io::Error::new(
        err_kind,
        "insufficient permissions to access current directory",
      )),
      _ => {
        debug!("{err}");

        Err(io::Error::new(
          err_kind,
          "failed to access current directory",
        ))
      }
    }
  } else {
    working_dir_res
  }
}
