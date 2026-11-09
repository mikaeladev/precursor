use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::num::NonZero;

use precursor_ani::AniFile;
use precursor_cur::{CurFile, CurImage};
use precursor_png::{PngFile, WriteResult};
use precursor_xcur::{XcursorChunk, XcursorFile};

use pixmap::{BgrAlphaPixmap, DynamicPixmap, IntoPixmap, Pixmap};
use point::Point;

use crate::frames::CursorFrames;

#[derive(Debug)]
pub struct BitmapCursor {
  pub images: BitmapCursorImages,
  pub frames: CursorFrames,
}

type BitmapCursorImages = Vec<BitmapCursorImage>;

#[derive(Debug, Clone)]
pub struct BitmapCursorImage {
  pub pixmap: DynamicPixmap,
  pub nominal: NonZero<u32>,
  pub hotspot: Point<u32>,
}

impl BitmapCursorImage {
  /// Scales the image by a `factor`.
  ///
  /// # Panics
  ///
  /// Panics if `factor < 2`, or if the new length exceeds `isize::MAX`.
  pub fn scale_up(&mut self, factor: usize) {
    self.pixmap.scale_up(factor);
    self.nominal = NonZero::new(self.nominal.get() * factor as u32).unwrap();
    self.hotspot *= factor as u32;
  }
}

impl BitmapCursor {
  /// Constructs a new [`AniFile`].
  pub fn to_ani_file(&self) -> WriteResult<AniFile> {
    let total_frames = match &self.frames {
      CursorFrames::Static(_) => todo!(),
      CursorFrames::Animated(frames) => frames,
    };

    let mut uniq_frames = total_frames.clone();
    uniq_frames.dedup_by(|this, other| this.indexes == other.indexes);

    let num_images = uniq_frames.len();
    let mut images = Vec::with_capacity(num_images);
    let mut image_index_map = HashMap::with_capacity(num_images);

    let num_frames = total_frames.len();
    let mut durations = Vec::with_capacity(num_frames);
    let mut sequence = Vec::with_capacity(num_frames);

    for (index, frame) in uniq_frames.into_iter().enumerate() {
      let image = make_cur_file(&self.images, &frame.indexes)?;
      let hash = calc_indexes_hash(&frame.indexes);
      images.push(image);
      image_index_map.insert(hash, index as u32);
    }

    for frame in total_frames {
      let duration = ms_to_jf(frame.duration.get());
      let hash = calc_indexes_hash(&frame.indexes);
      durations.push(duration);
      sequence.push(image_index_map[&hash]);
    }

    Ok(AniFile::new(images, durations, sequence))
  }

  /// Constructs a new [`CurFile`].
  pub fn to_cur_file(&self) -> WriteResult<CurFile> {
    let indexes = match &self.frames {
      CursorFrames::Static(indexes) => indexes,
      CursorFrames::Animated(_) => todo!(),
    };

    make_cur_file(&self.images, indexes)
  }

  /// Constructs a new [`XcursorFile`].
  pub fn to_xcursor_file(&self) -> XcursorFile {
    let mut chunks = Vec::with_capacity(match &self.frames {
      CursorFrames::Static(indexes) => indexes.len(),
      CursorFrames::Animated(frames) => frames
        .iter()
        .fold(0, |acc, frame| acc + frame.indexes.len()),
    });

    let get_pixels = |pixmap: &DynamicPixmap| {
      IntoPixmap::<BgrAlphaPixmap>::into_pixmap(pixmap.clone())
        .into_iter()
        .collect::<Vec<u8>>()
    };

    match &self.frames {
      CursorFrames::Static(indexes) => {
        for index in indexes {
          let image = &self.images[*index];
          let pixels = get_pixels(&image.pixmap);

          chunks.push(XcursorChunk::Image {
            nominal: image.nominal.get(),
            width: image.pixmap.width(),
            height: image.pixmap.height(),
            hotspot: image.hotspot,
            duration: 0,
            pixels,
          });
        }
      }
      CursorFrames::Animated(frames) => {
        for frame in frames {
          let duration = frame.duration.get();

          for index in &frame.indexes {
            let image = &self.images[*index];
            let pixels = get_pixels(&image.pixmap);

            chunks.push(XcursorChunk::Image {
              nominal: image.nominal.get(),
              width: image.pixmap.width(),
              height: image.pixmap.height(),
              hotspot: image.hotspot,
              duration,
              pixels,
            });
          }
        }
      }
    }

    XcursorFile::new(chunks)
  }
}

fn calc_indexes_hash(indexes: &[usize]) -> u64 {
  let mut hasher = DefaultHasher::new();
  indexes.hash(&mut hasher);
  hasher.finish()
}

fn make_cur_file(
  bitmaps: &BitmapCursorImages,
  indexes: &[usize],
) -> WriteResult<CurFile> {
  let mut images = Vec::with_capacity(indexes.len());

  for index in indexes {
    let image = &bitmaps[*index];

    let (width, height) = image.pixmap.dimensions();

    let hotspot = {
      let Point { x, y } = image.hotspot;
      Point::from((x as u16, y as u16))
    };

    let mut buffer = Vec::with_capacity(width as usize * height as usize);
    PngFile::new(image.pixmap.clone()).write(&mut buffer)?;

    images.push(CurImage::new(
      width as u16,
      height as u16,
      hotspot,
      buffer.into_boxed_slice(),
    ));
  }

  Ok(CurFile::new(images))
}

const JIFFY: f32 = 1000_f32 / 60_f32;

/// Converts a millisecond value to a jiffy value.
///
/// One jiffy is equal to 1/60th of a second (~16.66 ms)[^article].
///
/// [^article]: https://en.wikipedia.org/wiki/ANI_(file_format)#Timing
pub fn ms_to_jf(val: u32) -> u32 {
  (val as f32 / JIFFY) as u32
}

/// Converts a jiffy value to a millisecond value.
///
/// One jiffy is equal to 1/60th of a second (~16.66 ms)[^article].
///
/// [^article]: https://en.wikipedia.org/wiki/ANI_(file_format)#Timing
pub fn jf_to_ms(val: u32) -> u32 {
  (val as f32 * JIFFY) as u32
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn jiffy_conversion() {
    assert_eq!(ms_to_jf(200), 12);
    assert_eq!(jf_to_ms(12), 200);
  }
}
