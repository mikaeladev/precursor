use std::io;
use std::path::Path;

use crate::filesys::EntityKind;
use crate::path_error_msg;

pub fn prepare_file<S: Into<String>>(
  path: impl AsRef<Path>,
  kind: impl Into<Option<S>>,
  force: bool,
) -> io::Result<()> {
  let path = path.as_ref();

  if super::entity_exists(EntityKind::File, path)? {
    if force {
      super::remove_file(path, kind)
    } else {
      let kind = if let Some(value) = kind.into() {
        format_args!("{} file", value.into())
      } else {
        format_args!("file")
      };

      Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        path_error_msg!(already_exists: kind, path),
      ))
    }
  } else {
    Ok(())
  }
}
