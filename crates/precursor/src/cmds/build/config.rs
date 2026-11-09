use std::io::{BufRead, BufReader};

use precursor_config::cursors::*;
use precursor_cursor::bitmap::{BitmapCursor, BitmapCursorImage};
use precursor_cursor::frames::{AnimationFrame, CursorFrames};
use precursor_error::{ErrorMessage, Result};
use precursor_filesys::{self as filesys, metapath::AsMetaPath};
use precursor_png::PngFile;

use indexmap::IndexMap;
use pixmap::Pixmap;

use super::debug_help;

pub fn make_bitmap_cursor(config: &CursorConfig) -> Result<BitmapCursor> {
  log_orphans(config);

  let mut images = IndexMap::new();

  let frames = match config.get_frames().as_ref() {
    FramesConfig::Static { bitmaps } => {
      let indexes =
        make_and_insert_images(&mut images, &config.bitmaps, &bitmaps)?;

      CursorFrames::Static(indexes)
    }
    FramesConfig::Animated(config_frames) => {
      let mut animation_frames = Vec::with_capacity(config_frames.len());

      for config_frame in config_frames {
        let indexes = make_and_insert_images(
          &mut images,
          &config.bitmaps,
          &config_frame.bitmaps,
        )?;

        animation_frames.push(AnimationFrame {
          duration: config_frame.duration,
          indexes,
        });
      }

      CursorFrames::Animated(animation_frames)
    }
  };

  let images = images.into_values().collect();

  Ok(BitmapCursor { images, frames })
}

fn log_orphans(config: &CursorConfig) {
  let orphaned_bitmaps = config.find_orphaned_bitmaps();

  if !orphaned_bitmaps.is_empty() {
    log::warn!(
      "found {} orphaned bitmap(s) in '{}' cursor: {}",
      orphaned_bitmaps.len(),
      &config.name,
      orphaned_bitmaps
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(", ")
    );
  }
}

fn make_and_insert_images(
  images: &mut IndexMap<AssetIndex, BitmapCursorImage>,
  bitmap_configs: &[AssetConfig],
  bitmap_indexes: &[AssetIndex],
) -> Result<Vec<usize>> {
  let mut image_indexes = Vec::with_capacity(bitmap_indexes.len());

  for bitmap_index in bitmap_indexes {
    if let Some(image_index) = images.get_index_of(bitmap_index) {
      image_indexes.push(image_index);
    } else {
      let mut image = make_image(&bitmap_configs[bitmap_index.index])?;

      if let scale = bitmap_index.scale.get()
        && scale != 1
      {
        image.scale_up(scale);
      }

      let (image_index, _) = images.insert_full(*bitmap_index, image);
      image_indexes.push(image_index);
    }
  }

  Ok(image_indexes)
}

fn make_image(
  AssetConfig {
    path,
    nominal,
    hotspot,
    transforms,
  }: &AssetConfig,
) -> Result<BitmapCursorImage> {
  let file = filesys::read_file(path.as_meta_path("asset file"))?;

  let mut reader = BufReader::new(file);

  let buffer = reader.fill_buf().map_err(|src| {
    ErrorMessage::new("failed to fill buffer for asset")
      .with_subheading(src.kind())
      .with_context([("path", path.display())])
      .into_error()
      .with_source(src)
  })?;

  let mut pixmap = if buffer.starts_with(PngFile::MAGIC) {
    let png = PngFile::read(&mut reader).map_err(|src| {
      ErrorMessage::new("failed to decode bitmap")
        .with_help::<&str>(debug_help())
        .with_context([("path", path.display())])
        .into_error()
        .with_source(src)
    })?;

    png.into_pixmap()
  } else {
    todo!()
  };

  drop(reader);

  if let Some(transforms) = transforms {
    if transforms.flip {
      pixmap.flip_horizontal();
    }

    if transforms.flop {
      pixmap.flip_vertical();
    }

    match transforms.rotate {
      RotateValue::Zero => (),
      RotateValue::Ninety => pixmap.rotate_90(),
      RotateValue::OneEighty => pixmap.rotate_180(),
      RotateValue::TwoSeventy => pixmap.rotate_270(),
    }
  }

  Ok(BitmapCursorImage {
    pixmap,
    nominal: *nominal,
    hotspot: *hotspot,
  })
}
