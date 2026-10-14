use std::convert::Infallible;
use std::io::ErrorKind;
use std::path::Path;

use precursor_error::{Error, Result};

use crate::error::ErrorMessageWithPath;
use crate::macros::{action_denied, action_failed};

/// Removes the contents of the directory at the provided `path`.
///
/// This function does **not** follow symbolic links and it will simply remove
/// the symbolic link itself.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - `path` does not exist;
/// - `path` is not a directory;
/// - Directory is being concurrently written to;
/// - User lacks permissions to access and remove contents of `path`.
pub fn empty_dir<S: ToString>(
  path: impl AsRef<Path>,
  kind: impl Into<Option<S>>,
) -> Result<()> {
  let path = path.as_ref();

  let kind = if let Some(value) = kind.into() {
    format_args!("{} directory", value.to_string())
  } else {
    format_args!("directory")
  };

  for entry_res in super::read_dir(path, kind)? {
    match entry_res {
      Ok(entry) => {
        // TODO: use entry.file_type()
        let subpath = entry.path();
        let subpath_meta = super::get_metadata(&subpath, false)?;

        if subpath_meta.is_dir() {
          super::remove_dir_all::<Infallible>(subpath, None)?;
        } else {
          super::remove_file::<Infallible>(subpath, None)?;
        }
      }
      Err(err) if err.kind() == ErrorKind::NotFound => {
        break; // prevents race conditions
      }
      Err(src) => {
        let action = format_args!("read {kind} entry");

        let src_kind = src.kind();

        let msg = match src_kind {
          ErrorKind::PermissionDenied => action_denied!(action),
          _ => action_failed!(action, src_kind),
        };

        return Err(Error::new(ErrorMessageWithPath::new(msg, path), src));
      }
    }
  }

  Ok(())
}
