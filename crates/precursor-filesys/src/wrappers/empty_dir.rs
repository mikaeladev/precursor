use std::io::ErrorKind;

use precursor_error::{ErrorMessage, Result};

use crate::macros::{action_denied, action_failed};
use crate::metapath::{AsMetaPath, MetaPath};

/// Removes the contents of the directory at the provided path.
///
/// This function does **not** follow symbolic links and will simply remove any
/// symlinks it comes across.
///
/// # Errors
///
/// Fails with an error in the following (non-exhaustive) situations:
///
/// - Path does not exist;
/// - Path is not a directory;
/// - Directory is being concurrently written to;
/// - User lacks permissions to remove contents of path.
pub fn empty_dir(meta_path: MetaPath) -> Result<()> {
  let MetaPath { path, kind } = meta_path;

  for entry_res in super::read_dir(meta_path)? {
    match entry_res {
      Ok(entry) => {
        // TODO: use entry.file_type()
        let subpath = entry.path();
        let subpath_meta = crate::get_metadata(&subpath, false)?;

        if subpath_meta.is_dir() {
          super::remove_dir_all(
            subpath.as_meta_path(&format!("subdirectory of {kind}")),
          )?;
        } else {
          super::remove_file(
            subpath.as_meta_path(&format!("file within {kind}")),
          )?;
        }
      }
      Err(err) if err.kind() == ErrorKind::NotFound => {
        break; // prevent race conditions
      }
      Err(src) => {
        let action = format_args!("read entry within {kind}");

        let src_kind = src.kind();

        return Err(
          match src_kind {
            ErrorKind::PermissionDenied => action_denied!(action),
            _ => action_failed!(action, src_kind),
          }
          .with_context([("path", path.display())])
          .into_error()
          .with_source(src),
        );
      }
    }
  }

  Ok(())
}
