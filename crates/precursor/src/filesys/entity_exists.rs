use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::debug;
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
  subtype: &str,
  path: &PathBuf,
) -> io::Result<bool> {
  use io::ErrorKind::{
    IsADirectory, NotADirectory, NotFound, PermissionDenied,
  };

  match fs::metadata(path) {
    Ok(metadata) => {
      if metadata.is_file() {
        match kind {
          EntityKind::Either | EntityKind::File => Ok(true),
          EntityKind::Directory => Err(io::Error::new(
            NotADirectory,
            path_error_msg!(
              expected: format_args!("{subtype} {kind}"),
              found: "file",
              path
            ),
          )),
        }
      } else if metadata.is_dir() {
        match kind {
          EntityKind::Either | EntityKind::Directory => Ok(true),
          EntityKind::File => Err(io::Error::new(
            IsADirectory,
            path_error_msg!(
              expected: format_args!("{subtype} {kind}"),
              found: "directory",
              path
            ),
          )),
        }
      } else {
        match kind {
          EntityKind::Either => Ok(true),
          _ => todo!("symlinks not yet implemented"),
        }
      }
    }
    Err(err) => {
      let err_kind = err.kind();
      match err_kind {
        NotFound => Ok(false),
        PermissionDenied => Err(io::Error::new(
          err_kind,
          path_error_msg!(
            denied_action: format_args!("access {subtype} {kind}"),
            path
          ),
        )),
        _ => {
          debug!("{err}");

          Err(io::Error::new(
            err_kind,
            path_error_msg!(
              failed_action: format_args!("access {subtype} {kind}"),
              path
            ),
          ))
        }
      }
    }
  }
}
