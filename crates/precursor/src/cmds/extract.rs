use std::io::{self, Read, Seek, Write};
use std::path::Path;

use precursor_formats::ani::{self, AniFile};
use precursor_formats::cur::{self, CurFile, CurImage};
use precursor_pixmap_png::PNG_MAGIC;

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

  let base_path = working_dir_path.join(target_dir_path);

  if empty {
    filesys::ensure_dir_empty(&base_path, "target")?;
  } else {
    filesys::ensure_dir(&base_path, "target")?;
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
    CursorKind::Ani => extract_ani(&mut reader, base_path, force)?,
    CursorKind::Cur => extract_cur(&mut reader, base_path, force)?,
    _ => todo!("extract is not yet implemented for this format"),
  };

  if num_frames == 0 {
    println!("failed to extract any frames")
  } else {
    println!("extracted {num_frames} frame(s)");
  }

  Ok(())
}

fn extract_ani<R: Read>(
  reader: &mut R,
  base_path: impl AsRef<Path>,
  force: bool,
) -> ani::ReadResult<usize> {
  let base_path = base_path.as_ref();

  let ani_file = AniFile::read(reader)?;

  let mut total_icons = 0;
  let mut frame_index = 0;

  for frame in ani_file.into_frames() {
    let mut icon_index = 0;

    for CurImage { buffer, .. } in frame.into_icons() {
      let file_name = format!("{}-{}", frame_index, icon_index);
      write_cur_icon(base_path, file_name, buffer, force)?;

      icon_index += 1;
    }

    total_icons += icon_index;
    frame_index += 1;
  }

  Ok(total_icons)
}

fn extract_cur<R: Read + Seek>(
  reader: &mut R,
  base_path: impl AsRef<Path>,
  force: bool,
) -> cur::ReadResult<usize> {
  let base_path = base_path.as_ref();

  let mut icon_index = 0;

  for CurImage { buffer, .. } in CurFile::read(reader)?.into_icons() {
    let file_name = icon_index.to_string();
    write_cur_icon(base_path, file_name, buffer, force)?;

    icon_index += 1;
  }

  Ok(icon_index)
}

fn write_cur_icon(
  base_path: impl AsRef<Path>,
  file_name: impl AsRef<Path>,
  buffer: Box<[u8]>,
  force: bool,
) -> io::Result<()> {
  let file_ext = if buffer.starts_with(PNG_MAGIC) {
    "png"
  } else {
    todo!("bmp is not yet implemented")
  };

  let file_path = base_path
    .as_ref()
    .join(file_name.as_ref().with_extension(file_ext));

  let mut file = if force {
    filesys::create_file(file_path, file_ext)
  } else {
    filesys::create_new_file(file_path, file_ext)
  }?;

  file.write_all(&buffer)
}
