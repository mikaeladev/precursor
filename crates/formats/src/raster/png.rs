use std::fs::File;
use std::io::BufReader;

use png::{
  BitDepth, ColorType, Compression, Decoder, Encoder, OutputInfo,
  Transformations,
};

use super::{
  DynamicPixmap, IndexedPixmap, LumaAlphaPixel, LumaAlphaPixmap, LumaPixel,
  LumaPixmap, PaletteIndex, RasterError, RgbAlphaPixel, RgbAlphaPixmap,
  RgbPixel, RgbPixmap,
};

pub trait PngImage {
  /// Decodes a PNG image into `Self`.
  fn decode_png(reader: BufReader<File>) -> Result<Self, RasterError>
  where
    Self: Sized;

  /// Encodes `Self` into a PNG image.
  fn encode_png(&self) -> Result<Vec<u8>, RasterError>;

  /// Returns the associated PNG [`ColorType`].
  fn color_type(&self) -> ColorType;
}

impl PngImage for DynamicPixmap {
  fn decode_png(reader: BufReader<File>) -> Result<Self, RasterError> {
    let mut decoder = Decoder::new(reader);
    decoder.set_transformations(Transformations::STRIP_16);

    let mut png_reader = decoder.read_info()?;

    let frame_buffer_len = png_reader
      .output_buffer_size()
      .expect("buffer length should not exceed isize::MAX");

    let mut frame_buffer = vec![0; frame_buffer_len];

    let OutputInfo {
      width,
      height,
      color_type,
      ..
    } = png_reader.next_frame(&mut frame_buffer)?;

    Ok(match color_type {
      ColorType::Grayscale => {
        let pixels =
          frame_buffer.into_iter().map(|y| LumaPixel { y }).collect();

        DynamicPixmap::Luma(LumaPixmap::new(width, height, pixels)?)
      }

      ColorType::GrayscaleAlpha => {
        let (chunks, remainder) = frame_buffer.as_chunks::<2>();
        assert!(remainder.is_empty());

        let pixels = chunks
          .into_iter()
          .map(|chunk| LumaAlphaPixel::from(*chunk))
          .collect();

        DynamicPixmap::LumaAlpha(LumaAlphaPixmap::new(width, height, pixels)?)
      }

      ColorType::Rgb => {
        let (chunks, remainder) = frame_buffer.as_chunks::<3>();
        assert!(remainder.is_empty());

        let pixels = chunks
          .into_iter()
          .map(|chunk| RgbPixel::from(*chunk))
          .collect();

        DynamicPixmap::Rgb(RgbPixmap::new(width, height, pixels)?)
      }

      ColorType::Rgba => {
        let (chunks, remainder) = frame_buffer.as_chunks::<4>();
        assert!(remainder.is_empty());

        let pixels = chunks
          .into_iter()
          .map(|chunk| RgbAlphaPixel::from(*chunk))
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
            .map(|i| PaletteIndex { i })
            .collect();

          let palette = chunks
            .into_iter()
            .map(|chunk| RgbPixel::from(*chunk))
            .collect();

          let trns = match &image_info.trns {
            Some(bytes) => Some(bytes.to_vec()),
            _ => None,
          };

          DynamicPixmap::Indexed(IndexedPixmap::new(
            width, height, pixels, palette, trns,
          )?)
        } else {
          todo!()
        }
      }
    })
  }

  fn encode_png(&self) -> Result<Vec<u8>, RasterError> {
    let width = self.width();
    let height = self.height();

    let capacity = width as usize * height as usize * 4; // FIXME

    let mut buffer = Vec::with_capacity(capacity);
    let mut encoder = Encoder::new(&mut buffer, width, height);

    encoder.set_depth(BitDepth::Eight);
    encoder.set_color(self.color_type());
    encoder.set_compression(Compression::High);

    if let DynamicPixmap::Indexed(indexed_pixmap) = self {
      let palette: Vec<_> = indexed_pixmap
        .palette()
        .iter()
        .flat_map(|p| p.into_iter())
        .collect();

      encoder.set_palette(palette);

      if let Some(vec) = indexed_pixmap.trns() {
        encoder.set_trns(vec);
      }
    }

    let mut writer = encoder.write_header()?;

    writer.write_image_data(&self.pixels_concat())?;
    writer.finish()?;

    Ok(buffer)
  }

  fn color_type(&self) -> ColorType {
    use DynamicPixmap::*;

    match self {
      Luma(_) => ColorType::Grayscale,
      LumaAlpha(_) => ColorType::GrayscaleAlpha,
      Rgb(_) => ColorType::Rgb,
      RgbAlpha(_) => ColorType::Rgba,
      Indexed(_) => ColorType::Indexed,
    }
  }
}
