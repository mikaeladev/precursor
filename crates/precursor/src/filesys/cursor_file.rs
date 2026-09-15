use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use crate::{debug, path_error_msg};

use super::EntityKind;

pub fn prepare_cursor_file_path(
  file_path: &PathBuf,
  force: bool,
) -> io::Result<()> {
  if super::entity_exists(EntityKind::File, file_path)? {
    if force {
      remove_cursor_file(file_path)
    } else {
      Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        path_error_msg!(already_exists: "cursor file", file_path),
      ))
    }
  } else {
    Ok(())
  }
}

pub fn remove_cursor_file(file_path: &PathBuf) -> io::Result<()> {
  if let Err(err) = fs::remove_file(file_path) {
    let action = "remove cursor file";
    let err_kind = err.kind();

    Err(match err_kind {
      io::ErrorKind::PermissionDenied => io::Error::new(
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
    Ok(())
  }
}

#[cfg(target_family = "unix")]
use std::ffi::OsStr;

#[cfg(target_family = "unix")]
pub fn symlink_cursor_file(
  base_path: impl AsRef<Path>,
  cursor_name: impl AsRef<OsStr>,
  alias_name: impl AsRef<OsStr>,
) -> io::Result<()> {
  use std::os::unix::fs as unix_fs;

  use io::ErrorKind::{AlreadyExists, NotFound, PermissionDenied};

  let base_path = base_path.as_ref();
  let cursor_name = cursor_name.as_ref();
  let alias_name = alias_name.as_ref();

  let cursor_path = base_path.join(cursor_name);
  let link_path = base_path.join(alias_name);

  if !super::entity_exists(EntityKind::Either, &cursor_path)? {
    return Err(io::Error::new(
      NotFound,
      path_error_msg!(not_found: "cursor file or directory", cursor_path),
    ));
  }

  if super::entity_exists(EntityKind::Either, &link_path)? {
    return Err(io::Error::new(
      AlreadyExists,
      path_error_msg!(already_exists: "cursor file or directory", cursor_path),
    ));
  }

  if let Err(err) = unix_fs::symlink(cursor_name, &link_path) {
    let action = "symlink cursor file or directory";
    let err_kind = err.kind();

    return Err(match err_kind {
      PermissionDenied => io::Error::new(
        err_kind,
        path_error_msg!(action_denied: action, link_path),
      ),
      _ => {
        debug!("{err}");

        io::Error::new(
          err_kind,
          path_error_msg!(action_failed: action, link_path),
        )
      }
    });
  }

  Ok(())
}
