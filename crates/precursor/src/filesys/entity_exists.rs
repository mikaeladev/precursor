use std::fmt::{self, Display, Formatter};
use std::io::{self, ErrorKind};
use std::path::Path;

use crate::path_error_msg;

#[derive(PartialEq, Eq)]
pub enum EntityKind {
  File,
  Directory,
  Either,
}

impl Display for EntityKind {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.write_str(match self {
      Self::File => "file",
      Self::Directory => "directory",
      Self::Either => "file or directory",
    })
  }
}

// TODO: doc
pub fn entity_exists(
  kind: EntityKind,
  path: impl AsRef<Path>,
) -> io::Result<bool> {
  let path = path.as_ref();

  match super::get_metadata(path) {
    Ok(metadata) => {
      if metadata.is_file() {
        match kind {
          EntityKind::Either | EntityKind::File => Ok(true),
          EntityKind::Directory => Err(io::Error::new(
            ErrorKind::NotADirectory,
            path_error_msg!(expected_found: "a directory", "a file", path),
          )),
        }
      } else if metadata.is_dir() {
        match kind {
          EntityKind::Either | EntityKind::Directory => Ok(true),
          EntityKind::File => Err(io::Error::new(
            ErrorKind::IsADirectory,
            path_error_msg!(expected_found: "a file", "a directory", path),
          )),
        }
      } else {
        match kind {
          EntityKind::Either => Ok(true),
          _ => todo!("symlinks not yet implemented"),
        }
      }
    }
    Err(err) if err.kind() == ErrorKind::NotFound => Ok(false),
    Err(err) => Err(err),
  }
}
