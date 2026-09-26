use std::io::{Read, Seek, Write};
use std::path::Path;

use crate_formats::cur::{self, CurFile, CurIcon};
use crate_pixmap_png::PNG_MAGIC;

use crate::args::{ExtractArgs, InputArg};
use crate::cursor::CursorKind;
use crate::error::{PrecursorError, PrecursorResult};
use crate::filesys;
use crate::paths;

pub fn extract(
  ExtractArgs {
    cursor_file_input,
    target_dir_path,
    kind_hint,
    empty,
    force,
  }: ExtractArgs,
) -> PrecursorResult {
  let working_dir_path = paths::get_working_dir_path()?;

  let target_dir_path = working_dir_path.join(target_dir_path);

  if empty {
    filesys::ensure_dir_empty(&target_dir_path, "target")?;
  } else {
    filesys::ensure_dir(&target_dir_path, "target")?;
  }

  let mut cursor_kind = kind_hint.and_then(|hint| Some(hint.into()));

  if cursor_kind.is_none()
    && let InputArg::Path(path) = &cursor_file_input
    && let Some(ext) = path.extension()
    && let Some(kind) = CursorKind::from_ext(ext)
  {
    cursor_kind = Some(kind)
  }

  let mut reader =
    filesys::get_cursor_reader(working_dir_path, cursor_file_input)?;

  if cursor_kind.is_none() {
    match CursorKind::from_reader(&mut reader)? {
      Some(kind) => cursor_kind = Some(kind),
      None => return Err(PrecursorError::UnrecognisedCursorFormat),
    }
  }

  let num_frames = match cursor_kind.unwrap() {
    CursorKind::Cur => extract_cur(&mut reader, target_dir_path, force)?,
    _ => todo!("extract is not yet implemented for this format"),
  };

  if num_frames == 0 {
    println!("failed to extract any frames")
  } else {
    println!("extracted {num_frames} frame(s)");
  }

  Ok(())
}

fn extract_cur<R: Read + Seek>(
  reader: &mut R,
  base_path: impl AsRef<Path>,
  force: bool,
) -> cur::ReadResult<usize> {
  let base_path = base_path.as_ref();

  let icons = CurFile::read(reader)?.into_icons();

  let mut icon_index = 0;

  for CurIcon { buffer, .. } in icons {
    let file_ext = if buffer.starts_with(PNG_MAGIC) {
      "png"
    } else {
      "bmp"
    };

    let file_name = format!("{icon_index}.{file_ext}");
    let file_path = base_path.join(file_name);

    let mut file = if force {
      filesys::create_file(file_path, file_ext)
    } else {
      filesys::create_new_file(file_path, file_ext)
    }?;

    file.write_all(&buffer)?;

    icon_index += 1;
  }

  Ok(icon_index)
}
