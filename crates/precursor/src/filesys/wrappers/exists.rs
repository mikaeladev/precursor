use std::fmt;
use std::io;
use std::path::Path;

use crate::path_error_msg;

pub enum EntityKind {
  File,
  Directory,
  Either,
}

impl fmt::Display for EntityKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(match self {
      Self::File => "file",
      Self::Directory => "directory",
      Self::Either => "file or directory",
    })
  }
}

pub fn entity_exists(
  kind: EntityKind,
  path: impl AsRef<Path>,
) -> io::Result<bool> {
  use io::ErrorKind::{IsADirectory, NotADirectory, NotFound};

  let path = path.as_ref();

  match super::get_metadata(path) {
    Ok(metadata) => {
      if metadata.is_file() {
        match kind {
          EntityKind::Either | EntityKind::File => Ok(true),
          EntityKind::Directory => Err(io::Error::new(
            NotADirectory,
            path_error_msg!(expected_found: "a directory", "a file", path),
          )),
        }
      } else if metadata.is_dir() {
        match kind {
          EntityKind::Either | EntityKind::Directory => Ok(true),
          EntityKind::File => Err(io::Error::new(
            IsADirectory,
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
    Err(err) => match err.kind() {
      NotFound => Ok(false),
      _ => Err(err),
    },
  }
}
