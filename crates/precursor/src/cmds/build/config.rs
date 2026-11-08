use std::io::{BufRead, BufReader};

use precursor_config::cursors::*;
use precursor_cursor::{AnimationFrame, CursorData, CursorFrames, CursorImage};
use precursor_error::{ErrorMessage, Result};
use precursor_filesys::{self as filesys, metapath::AsMetaPath};
use precursor_png::PngFile;

use indexmap::IndexMap;
use pixmap::Pixmap;

use super::debug_help;

pub fn make_cursor(config: &CursorConfig) -> Result<CursorData> {
  if let orphans = config.find_orphans()
    && !orphans.is_empty()
  {
    log::warn!(
      "found {} orphaned asset(s) in '{}' cursor: {}",
      orphans.len(),
      &config.name,
      orphans
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(", ")
    );
  }

  let mut image_map = IndexMap::new();

  let cursor_frames = match config.get_frames().as_ref() {
    FramesConfig::Static(asset_indexes) => {
      let image_indexes =
        insert_images(&mut image_map, &config.assets, &asset_indexes)?;

      CursorFrames::Static(image_indexes)
    }
    FramesConfig::Animated(config_frames) => {
      let mut animation_frames = Vec::with_capacity(config_frames.len());

      for config_frame in config_frames {
        let image_indexes =
          insert_images(&mut image_map, &config.assets, &config_frame.assets)?;

        animation_frames.push(AnimationFrame {
          duration: config_frame.duration,
          indexes: image_indexes,
        });
      }

      CursorFrames::Animated(animation_frames)
    }
  };

  Ok(CursorData {
    images: image_map.into_values().collect(),
    frames: cursor_frames,
  })
}

fn insert_images(
  map: &mut IndexMap<AssetIndex, CursorImage>,
  assets: &[AssetConfig],
  asset_indexes: &[AssetIndex],
) -> Result<Vec<usize>> {
  let mut image_indexes = Vec::with_capacity(asset_indexes.len());

  for asset_index in asset_indexes {
    let asset_index = asset_index.into_struct();

    if let Some(image_index) = map.get_index_of(&asset_index) {
      image_indexes.push(image_index);
    } else {
      let AssetIndex::Struct { index, scale } = asset_index else {
        unreachable!() // into_struct() was called
      };

      let mut image = make_cursor_image(&assets[index])?;

      if let prim_scale = scale.get()
        && prim_scale != 1
      {
        image.scale_up(prim_scale);
      }

      let (image_index, _) = map.insert_full(asset_index, image);
      image_indexes.push(image_index);
    }
  }

  Ok(image_indexes)
}

fn make_cursor_image(
  AssetConfig {
    path,
    nominal,
    hotspot,
    transforms,
  }: &AssetConfig,
) -> Result<CursorImage> {
  let file = filesys::read_file(path.as_meta_path("asset file"))?;

  let mut reader = BufReader::new(file);

  let buffer = reader.fill_buf().map_err(|src| {
    ErrorMessage::new("failed to fill icon buffer for asset")
      .with_subheading(src.kind())
      .with_context([("path", path.display())])
      .into_error()
      .with_source(src)
  })?;

  let mut pixmap = if buffer.starts_with(PngFile::MAGIC) {
    let png = PngFile::read(&mut reader).map_err(|src| {
      ErrorMessage::new("failed to decode asset")
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

  Ok(CursorImage {
    pixmap,
    nominal: *nominal,
    hotspot: *hotspot,
  })
}
