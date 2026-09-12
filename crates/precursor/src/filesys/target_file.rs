use std::fs;
use std::io;
use std::path::PathBuf;

use crate::debug;
use crate::path_error_msg;

use super::EntityKind;

pub fn remove_target_file(file_path: &PathBuf) -> io::Result<()> {
  if let Err(err) = fs::remove_file(file_path) {
    let action = "remove target file";
    let err_kind = err.kind();

    Err(match err_kind {
      io::ErrorKind::PermissionDenied => io::Error::new(
        err_kind,
        path_error_msg!(denied_action: action, file_path),
      ),
      _ => {
        debug!("{err}");

        io::Error::new(
          err_kind,
          path_error_msg!(failed_action: action, file_path),
        )
      }
    })
  } else {
    Ok(())
  }
}

pub fn prepare_target_file_path(
  file_path: &PathBuf,
  force: bool,
) -> io::Result<()> {
  if super::entity_exists(EntityKind::File, "target", file_path)? {
    if force {
      remove_target_file(file_path)
    } else {
      Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        path_error_msg!("target file already exists at path", file_path),
      ))
    }
  } else {
    Ok(())
  }
}
