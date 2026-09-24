use std::io::{BufRead, Seek};

use crate_pixmap::pixels::*;
use crate_pixmap::*;

use png::{BitDepth, ColorType, Decoder, OutputInfo, Transformations};

use crate::DecodeResult;

/// Decodes a PNG image from `reader`, returning the constructed
/// [`DynamicPixmap`].
///
/// # Errors
///
/// Fails with a [`DecodeError`] if the operation fails. See the [PNG
/// documentation] for details.
///
/// [`DecodeError`]: crate::DecodeError
/// [PNG documentation]: png::DecodingError
pub fn decode<R: BufRead + Seek>(
  reader: &mut R,
) -> DecodeResult<DynamicPixmap> {
  let mut decoder = Decoder::new(reader);
  decoder.set_transformations(Transformations::STRIP_16);

  let mut png_reader = decoder.read_info()?;

  let frame_buffer_len = png_reader // TODO: handle gracefully
    .output_buffer_size()
    .expect("buffer length should not exceed isize::MAX");

  let mut frame_buffer = vec![0; frame_buffer_len];

  let OutputInfo {
    width,
    height,
    color_type,
    bit_depth,
    ..
  } = png_reader.next_frame(&mut frame_buffer)?;

  if bit_depth != BitDepth::Eight {
    unimplemented!() // TODO: implement or error
  }

  // FIXME: decompress frame_buffer first

  Ok(match color_type {
    ColorType::Grayscale => {
      let pixels = frame_buffer.into_iter().map(|y| LumaPixel { y }).collect();

      DynamicPixmap::Luma(LumaPixmap::new(width, height, pixels)?)
    }

    ColorType::GrayscaleAlpha => {
      let (chunks, remainder) = frame_buffer.as_chunks::<2>();
      assert!(remainder.is_empty());

      let pixels = chunks
        .into_iter()
        .map(|chunk| LumaAlphaPixel::from_bytes(*chunk))
        .collect();

      DynamicPixmap::LumaAlpha(LumaAlphaPixmap::new(width, height, pixels)?)
    }

    ColorType::Rgb => {
      let (chunks, remainder) = frame_buffer.as_chunks::<3>();
      assert!(remainder.is_empty());

      let pixels = chunks
        .into_iter()
        .map(|chunk| RgbPixel::from_bytes(*chunk))
        .collect();

      DynamicPixmap::Rgb(RgbPixmap::new(width, height, pixels)?)
    }

    ColorType::Rgba => {
      let (chunks, remainder) = frame_buffer.as_chunks::<4>();
      assert!(remainder.is_empty());

      let pixels = chunks
        .into_iter()
        .map(|chunk| RgbAlphaPixel::from_bytes(*chunk))
        .collect();

      DynamicPixmap::RgbAlpha(RgbAlphaPixmap::new(width, height, pixels)?)
    }

    ColorType::Indexed => {
      let image_info = png_reader.info();

      if let Some(palette_buf) = &image_info.palette {
        let (chunks, remainder) = palette_buf.as_chunks::<3>();
        assert!(remainder.is_empty());

        let pixels = frame_buffer
          .into_iter()
          .map(|i| IndexedPixel { i })
          .collect();

        let palette = chunks
          .into_iter()
          .map(|chunk| RgbPixel::from_bytes(*chunk))
          .collect();

        let trns = match &image_info.trns {
          Some(bytes) => Some(bytes.to_vec()),
          _ => None,
        };

        DynamicPixmap::Indexed(IndexedPixmap::new(
          width, height, pixels, palette, trns,
        )?)
      } else {
        // TODO: return an error
        todo!()
      }
    }
  })
}

// TODO: tests
