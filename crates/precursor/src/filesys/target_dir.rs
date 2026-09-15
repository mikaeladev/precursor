use std::io;
use std::path::PathBuf;

use crate::path_error_msg;

use super::EntityKind;

pub fn check_target_dir_path(dir_path: &PathBuf) -> io::Result<()> {
  match super::entity_exists(EntityKind::Directory, dir_path)? {
    true => Ok(()),
    false => Err(io::Error::new(
      io::ErrorKind::NotFound,
      path_error_msg!(not_found: "target directory", dir_path),
    )),
  }
}
