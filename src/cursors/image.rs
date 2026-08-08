use std::fs::File;
use std::io::{BufReader, Result as IoResult};

use png::{
  BitDepth, ColorType, Compression, Decoder, Encoder, Transformations,
};

#[derive(Clone)]
pub struct CursorImage {
  /// Width/Height in pixels.
  size: u16,
  /// Hotspot co-ordinates.
  hotspot: (u16, u16),
  /// Image buffer.
  buffer: Vec<u8>,
}

impl CursorImage {
  /// Returns the cursor hotspot co-ordinates.
  pub const fn hotspot(&self) -> (u16, u16) {
    self.hotspot
  }

  /// Returns a slice to the underlying image buffer.
  pub const fn buffer(&self) -> &[u8] {
    self.buffer.as_slice()
  }

  /// Returns the width/height of the image in pixels.
  pub const fn size(&self) -> u16 {
    self.size
  }

  /// Consumes the struct and returns the underlying image buffer.
  pub fn into_buffer(self) -> Vec<u8> {
    self.buffer
  }

  /// Creates a cursor image from a PNG file.
  pub fn from_png(
    size: u16,
    hotspot: (u16, u16),
    buf_reader: BufReader<File>,
  ) -> IoResult<Self> {
    let rgba = Self::decode_png(buf_reader)?;
    let buffer = Self::encode_png(size as u32, &rgba)?;

    Ok(Self {
      size,
      hotspot,
      buffer,
    })
  }

  /// Decodes a PNG file to an 8-bit RGBA buffer.
  fn decode_png(buf_reader: BufReader<File>) -> IoResult<Vec<u8>> {
    let mut decoder = Decoder::new(buf_reader);

    // strip or expand to 8-bit rgba/ga
    let transform = Transformations::ALPHA | Transformations::STRIP_16;
    decoder.set_transformations(transform);

    let mut reader = decoder.read_info()?;

    let frame_buffer_len = reader
      .output_buffer_size()
      .expect("buffer length should not exceed isize::MAX");

    let mut frame_buffer = vec![0u8; frame_buffer_len];

    reader.next_frame(&mut frame_buffer)?;

    match reader.output_color_type().0 {
      ColorType::Rgba => Ok(frame_buffer),
      ColorType::GrayscaleAlpha => {
        let pixel_count = frame_buffer.len() / 2;

        let mut rgba = vec![0u8; pixel_count * 4];

        for i in 0..pixel_count {
          let value = frame_buffer[2 * i];
          let alpha = frame_buffer[2 * i + 1];
          rgba.extend_from_slice(&[value, value, value, alpha]);
        }

        Ok(rgba)
      }
      _ => unreachable!(), // see transform
    }
  }

  /// Encodes an 8-bit RGBA buffer into a PNG file.
  fn encode_png(size: u32, rgba: &[u8]) -> IoResult<Vec<u8>> {
    let mut buffer = Vec::with_capacity((size as usize) ^ 2 * 4);
    let mut encoder = Encoder::new(&mut buffer, size, size);

    encoder.set_depth(BitDepth::Eight);
    encoder.set_color(ColorType::Rgba);
    encoder.set_compression(Compression::NoCompression);

    let mut writer = encoder.write_header()?;

    writer.write_image_data(rgba)?;
    writer.finish()?;

    Ok(buffer)
  }
}
