mod args;

use std::io::{Read, Seek, Write};
use std::path::Path;

use precursor_cursor::CursorKind;
use precursor_error::{ErrorMessage, Result};
use precursor_filesys::{self as filesys, InputArg};
use precursor_formats::ani::{self, AniFile};
use precursor_formats::cur::{self, CurFile, CurImage};
use precursor_formats::png::PngFile;
use precursor_paths as paths;

pub use args::*;

const CONSIDER_KIND_ARG: &str = "consider passing the '--kind' argument";

const CONSIDER_DEBUG_FLAG: &str =
  "consider passing the '--debug' flag to see the underlying error";

pub fn extract(
  ExtractArgs {
    cursor_file_input,
    mut cursor_kind,
    target_dir_path,
    empty,
    force,
  }: ExtractArgs,
) -> Result<()> {
  paths::set_target_dir(target_dir_path);

  let target_dir_path = paths::target_dir();

  if !filesys::ensure_dir(&target_dir_path, "target")? && empty {
    filesys::empty_dir(&target_dir_path, "target")?;
  }

  if cursor_kind.is_none()
    && let InputArg::Path(path) = &cursor_file_input
    && let Some(ext) = path.extension()
    && let Some(kind) = CursorKind::from_ext(ext)
  {
    cursor_kind = Some(kind)
  }

  let mut reader = filesys::get_cursor_reader(cursor_file_input)?;

  if cursor_kind.is_none() {
    let kind = CursorKind::from_reader(&mut reader).map_err(|src| {
      ErrorMessage::new("failed to read cursor data")
        .with_subheading(src.kind())
        .into_error()
        .with_source(src)
    })?;

    if kind.is_none() {
      return Err(
        ErrorMessage::new("unrecognised cursor format")
          .with_help(CONSIDER_KIND_ARG)
          .into_error(),
      );
    }

    cursor_kind = kind
  }

  let num_frames = match cursor_kind.unwrap() {
    CursorKind::Ani => extract_ani(&mut reader, force)?,
    CursorKind::Cur => extract_cur(&mut reader, force)?,
    _ => todo!("extract is not yet implemented for this format"),
  };

  // TODO: use log macros
  if num_frames == 0 {
    println!("failed to extract any frames")
  } else {
    println!("extracted {num_frames} frame(s)");
  }

  Ok(())
}

fn extract_ani<R: Read>(reader: &mut R, force: bool) -> Result<usize> {
  let mut total_icons = 0;
  let mut frame_index = 0;

  let ani_file = AniFile::read(reader).map_err(|src| {
    let message = ErrorMessage::new("failed to read ANI file");

    match &src {
      ani::ReadError::IoError(inner) => message.with_subheading(inner.kind()),
      ani::ReadError::FromChunkError(_) => message
        .with_subheading("malformed or invalid chunk(s)")
        .with_help(CONSIDER_DEBUG_FLAG),
    }
    .into_error()
    .with_source(src)
  })?;

  for frame in ani_file.into_frames() {
    let mut icon_index = 0;

    for CurImage { buffer, .. } in frame.into_icons() {
      let file_name = format!("{}-{}", frame_index, icon_index);
      write_cur_icon(file_name, buffer, force)?;

      icon_index += 1;
    }

    total_icons += icon_index;
    frame_index += 1;
  }

  Ok(total_icons)
}

fn extract_cur<R: Read + Seek>(reader: &mut R, force: bool) -> Result<usize> {
  let mut icon_index = 0;

  let cur_file = CurFile::read(reader).map_err(|src| {
    let message = ErrorMessage::new("failed to read CUR file");

    match &src {
      cur::ReadError::IoError(inner) => message.with_subheading(inner.kind()),
      _ => message.with_subheading(src.to_string()),
    }
    .into_error()
    .with_source(src)
  })?;

  for CurImage { buffer, .. } in cur_file.into_icons() {
    let file_name = icon_index.to_string();
    write_cur_icon(file_name, buffer, force)?;

    icon_index += 1;
  }

  Ok(icon_index)
}

fn write_cur_icon(
  file_name: impl AsRef<Path>,
  buffer: Box<[u8]>,
  force: bool,
) -> Result<()> {
  let file_ext = if buffer.starts_with(PngFile::MAGIC) {
    "png"
  } else {
    todo!("bmp is not yet implemented")
  };

  let file_path = paths::target_dir().join(file_name).with_extension(file_ext);

  let mut file = if force {
    filesys::create_file(&file_path, file_ext)
  } else {
    filesys::create_new_file(&file_path, file_ext)
  }?;

  file.write_all(&buffer).map_err(|src| {
    ErrorMessage::new("failed to write CUR icon")
      .with_subheading(src.kind())
      .with_context([("path", file_path.display())])
      .into_error()
      .with_source(src)
  })?;

  Ok(())
}
