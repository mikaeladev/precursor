use std::fs::File;
use std::io::{BufReader, Result as IoResult};

use png::{
  BitDepth, ColorType, Compression, Decoder, Encoder, OutputInfo,
  Transformations,
};

use crate::RasterImage;

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

    let transform = Transformations::ALPHA | Transformations::STRIP_16;
    decoder.set_transformations(transform);

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

    let pixels = match color_type {
      ColorType::Rgba => {
        let (chunks, remainder) = frame_buffer.as_chunks::<4>();
        assert!(remainder.is_empty());

        chunks.into_iter().map(|c| *c).collect()
      }
      ColorType::GrayscaleAlpha => {
        let (chunks, remainder) = frame_buffer.as_chunks::<4>();
        assert!(remainder.is_empty());

        chunks
          .into_iter()
          .map(|c| [c[0], c[0], c[0], c[1]])
          .collect()
      }
      _ => unreachable!(),
    };

    Ok(Self::new(width, height, pixels))
  }

  fn encode_png(&self) -> IoResult<Vec<u8>> {
    let width = self.width();
    let height = self.height();

    let capacity = width as usize * height as usize * 4;

    let mut buffer = Vec::with_capacity(capacity);
    let mut encoder = Encoder::new(&mut buffer, width, height);

    encoder.set_depth(BitDepth::Eight);
    encoder.set_color(ColorType::Rgba);
    encoder.set_compression(Compression::NoCompression);

    let mut writer = encoder.write_header()?;

    writer.write_image_data(&self.to_rgba())?;
    writer.finish()?;

    Ok(buffer)
  }
}
