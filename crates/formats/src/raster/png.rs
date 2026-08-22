use std::fs::File;
use std::io::{BufReader, Result as IoResult};

use png::{
  BitDepth, ColorType, Compression, Decoder, Encoder, OutputInfo,
  Transformations,
};

use super::{
  ConcatPixmap, GrayscaleAlphaPixel, GrayscalePixel, IndexedPixmap, Pixmap,
  RasterImage, RgbAlphaPixel, RgbPixel,
};

pub trait PngImage {
  /// Decodes a PNG image into `Self`.
  fn decode_png(reader: BufReader<File>) -> IoResult<Self>
  where
    Self: Sized;

  /// Encodes `Self` into a PNG image.
  fn encode_png(&self) -> IoResult<Vec<u8>>;
}

impl PngImage for RasterImage {
  fn decode_png(reader: BufReader<File>) -> IoResult<Self> {
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

    let pixmap = match color_type {
      ColorType::Grayscale => Pixmap::Grayscale(
        frame_buffer
          .into_iter()
          .map(|chunk| GrayscalePixel::from(chunk))
          .collect(),
      ),

      ColorType::GrayscaleAlpha => {
        let (chunks, remainder) = frame_buffer.as_chunks::<2>();
        assert!(remainder.is_empty());

        Pixmap::GrayscaleAlpha(
          chunks
            .into_iter()
            .map(|chunk| GrayscaleAlphaPixel::from(*chunk))
            .collect(),
        )
      }

      ColorType::Indexed => {
        let image_info = png_reader.info();

        if let Some(palette_buf) = &image_info.palette {
          let (chunks, remainder) = palette_buf.as_chunks::<3>();
          assert!(remainder.is_empty());

          let palette = chunks
            .into_iter()
            .map(|chunk| RgbPixel::from(*chunk))
            .collect();

          let trns = match &image_info.trns {
            Some(bytes) => Some(bytes.to_vec()),
            _ => None,
          };

          Pixmap::Indexed(IndexedPixmap {
            pixels: frame_buffer,
            palette,
            trns,
          })
        } else {
          todo!()
        }
      }

      ColorType::Rgb => {
        let (chunks, remainder) = frame_buffer.as_chunks::<3>();
        assert!(remainder.is_empty());

        Pixmap::Rgb(
          chunks
            .into_iter()
            .map(|chunk| RgbPixel::from(*chunk))
            .collect(),
        )
      }

      ColorType::Rgba => {
        let (chunks, remainder) = frame_buffer.as_chunks::<4>();
        assert!(remainder.is_empty());

        Pixmap::RgbAlpha(
          chunks
            .into_iter()
            .map(|chunk| RgbAlphaPixel::from(*chunk))
            .collect(),
        )
      }
    };

    Ok(Self::new(width, height, pixmap))
  }

  fn encode_png(&self) -> IoResult<Vec<u8>> {
    let width = self.width();
    let height = self.height();
    let pixmap = self.pixmap();

    let capacity = width as usize * height as usize * 4; // FIXME

    let mut buffer = Vec::with_capacity(capacity);
    let mut encoder = Encoder::new(&mut buffer, width, height);

    let color_type = pixmap.color_type();

    encoder.set_depth(BitDepth::Eight);
    encoder.set_color(color_type);
    encoder.set_compression(Compression::High);

    if let Pixmap::Indexed(indexed_pixmap) = pixmap {
      let palette: Vec<_> = indexed_pixmap
        .palette
        .iter()
        .flat_map(|p| p.into_iter())
        .collect();

      encoder.set_palette(palette);

      if let Some(vec) = &indexed_pixmap.trns {
        encoder.set_trns(vec);
      }
    }

    let mut writer = encoder.write_header()?;

    writer.write_image_data(&self.pixmap().concat())?;
    writer.finish()?;

    Ok(buffer)
  }
}
