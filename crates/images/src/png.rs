use std::fs::File;
use std::io::{BufReader, Result as IoResult};

use png::{
  BitDepth, ColorType, Compression, Decoder, Encoder, OutputInfo,
  Transformations,
};

use crate::RgbaImage;

impl RgbaImage {
  /// Decodes a PNG image into an RGBA image.
  pub fn decode_png(reader: BufReader<File>) -> IoResult<Self> {
    let mut decoder = Decoder::new(reader);

    // strip or expand to rgba/ga
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

    let rgba = match color_type {
      ColorType::Rgba => frame_buffer,
      ColorType::GrayscaleAlpha => {
        let num_pixels = frame_buffer.len() / 2;

        let mut rgba = Vec::with_capacity(num_pixels * 4);

        for i in 0..num_pixels {
          let value = frame_buffer[2 * i];
          let alpha = frame_buffer[2 * i + 1];

          rgba.extend_from_slice(&[value, value, value, alpha]);
        }

        rgba
      }
      // see transform
      _ => unreachable!(),
    };

    Ok(RgbaImage::new(width, height, rgba))
  }

  /// Encodes an RGBA image into a PNG image.
  pub fn encode_png(&self) -> IoResult<Vec<u8>> {
    let width = self.width();
    let height = self.height();

    let capacity = width as usize * height as usize * 4;

    let mut buffer = Vec::with_capacity(capacity);
    let mut encoder = Encoder::new(&mut buffer, width, height);

    encoder.set_depth(BitDepth::Eight);
    encoder.set_color(ColorType::Rgba);
    encoder.set_compression(Compression::NoCompression);

    let mut writer = encoder.write_header()?;

    writer.write_image_data(self.as_rgba())?;
    writer.finish()?;

    Ok(buffer)
  }
}
