use crate_pixmap::DynamicPixmap;
use crate_pixmap::pixels::IntoBytes;

use png::{BitDepth, ColorType, Compression, Encoder};

use crate::EncodeResult;

/// Encodes `pixmap` as a static PNG image.
///
/// # Errors
///
/// Fails with an [`EncodeError`] if the operation fails. See the [PNG
/// documentation] for details.
///
/// [`EncodeError`]: crate::EncodeError
/// [PNG documentation]: png::EncodingError
pub fn encode(pixmap: DynamicPixmap, buffer: &mut Vec<u8>) -> EncodeResult<()> {
  let (width, height) = pixmap.dimensions();

  let mut encoder = Encoder::new(buffer, width, height);

  encoder.set_depth(BitDepth::Eight);
  encoder.set_compression(Compression::High);

  let color_type = match &pixmap {
    DynamicPixmap::Luma(_) => ColorType::Grayscale,
    DynamicPixmap::LumaAlpha(_) => ColorType::GrayscaleAlpha,
    DynamicPixmap::Rgb(_) => ColorType::Rgb,
    DynamicPixmap::RgbAlpha(_) => ColorType::Rgba,
    DynamicPixmap::Indexed(pixmap) => {
      let palette: Vec<_> = pixmap
        .palette()
        .into_iter()
        .flat_map(|p| p.into_bytes())
        .collect();

      encoder.set_palette(palette);

      if let Some(vec) = pixmap.trns() {
        encoder.set_trns(vec);
      }

      ColorType::Indexed
    }
  };

  encoder.set_color(color_type);

  let mut writer = encoder.write_header()?;

  writer.write_image_data(&pixmap.concat())?;
  writer.finish()
}

// TODO: tests
