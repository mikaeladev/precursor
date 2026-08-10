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
  /// RGBA buffer.
  rgba: Vec<u8>,
}

impl CursorImage {
  /// Returns the cursor hotspot co-ordinates.
  pub const fn hotspot(&self) -> (u16, u16) {
    self.hotspot
  }

  /// Returns a slice to the underlying RGBA buffer.
  pub const fn rgba(&self) -> &[u8] {
    self.rgba.as_slice()
  }

  /// Returns the width/height of the image in pixels.
  pub const fn size(&self) -> u16 {
    self.size
  }

  /// Consumes the struct and returns the underlying RGBA buffer.
  pub fn into_rgba(self) -> Vec<u8> {
    self.rgba
  }

  /// Consumes the struct and returns a BGRA buffer.
  pub fn into_bgra(self) -> Vec<u8> {
    let mut bgra = self.into_rgba();

    for chunk in bgra.as_chunks_mut::<4>().0 {
      chunk.swap(0, 2);
    }

    bgra
  }

  /// Consumes the struct and returns a PNG file buffer.
  pub fn into_png(self) -> IoResult<Vec<u8>> {
    Self::encode_png(self.size as u32, &self.rgba)
  }

  /// Creates a cursor image from a PNG file.
  pub fn from_png(
    size: u16,
    hotspot: (u16, u16),
    buf_reader: BufReader<File>,
  ) -> IoResult<Self> {
    let rgba = Self::decode_png(buf_reader)?;

    Ok(Self {
      size,
      hotspot,
      rgba,
    })
  }

  /// Decodes a PNG file to an RGBA buffer.
  fn decode_png(buf_reader: BufReader<File>) -> IoResult<Vec<u8>> {
    let mut decoder = Decoder::new(buf_reader);

    // strip or expand to rgba/ga
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
        let num_pixels = frame_buffer.len() / 2;

        let mut rgba = Vec::with_capacity(num_pixels * 4);

        for i in 0..num_pixels {
          let value = frame_buffer[2 * i];
          let alpha = frame_buffer[2 * i + 1];

          rgba.extend_from_slice(&[value, value, value, alpha]);
        }

        Ok(rgba)
      }
      _ => unreachable!(), // see transform
    }
  }

  /// Encodes an RGBA buffer as a PNG file.
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
