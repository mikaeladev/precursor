use std::io::{BufRead, Seek, Write};

use precursor_pixmap::{DynamicPixmap, Pixel};

use png::{
  Compression, Decoder, DecodingError, Encoder, EncodingError, Transformations,
};

pub type ReadError = DecodingError;
pub type ReadResult<T> = Result<T, ReadError>;

pub type WriteError = EncodingError;
pub type WriteResult<T> = Result<T, WriteError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitDepth {
  Eight,
}

impl From<png::BitDepth> for BitDepth {
  fn from(bit_depth: png::BitDepth) -> Self {
    match bit_depth {
      png::BitDepth::Eight => Self::Eight,
      _ => todo!("only 8-bit images are supported for now"),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorType {
  Indexed,
  Luma,
  LumaAlpha,
  Rgb,
  RgbAlpha,
}

impl From<png::ColorType> for ColorType {
  fn from(color_type: png::ColorType) -> Self {
    match color_type {
      png::ColorType::Indexed => Self::Indexed,
      png::ColorType::Grayscale => Self::Luma,
      png::ColorType::GrayscaleAlpha => Self::LumaAlpha,
      png::ColorType::Rgb => Self::Rgb,
      png::ColorType::Rgba => Self::RgbAlpha,
    }
  }
}

pub struct PngFile {
  pixmap: DynamicPixmap,
}

impl PngFile {
  pub const MAGIC: &[u8] = &[137, b'P', b'N', b'G', 13, 10, 26, 10];

  /// Constructs a new `PngFile`.
  pub fn new(pixmap: DynamicPixmap) -> Self {
    Self { pixmap }
  }

  /// Reads a PNG file from `reader`, returning the constructed `PngFile`.
  ///
  /// # Errors
  ///
  /// See the [`ReadError`] enum for details.
  pub fn read<R: BufRead + Seek>(reader: &mut R) -> ReadResult<Self> {
    let mut decoder = Decoder::new(reader);
    decoder.set_transformations(Transformations::STRIP_16);
    decoder.set_ignore_text_chunk(true); // for now

    let mut png_reader = decoder.read_info()?;
    let png_info = png_reader.info();

    if png_info.is_animated() {
      panic!("APNGs are unsupported"); // TODO: error
    }

    let width = png_info.width;
    let height = png_info.height;
    let _bit_depth = BitDepth::from(png_info.bit_depth);
    let color_type = ColorType::from(png_info.color_type);

    let indexed_palette = png_info.palette.to_owned();
    let indexed_alpha = png_info.trns.to_owned();

    if color_type == ColorType::Indexed && indexed_palette.is_none() {
      panic!("missing PLTE chunk") // TODO: error
    }

    let frame_buffer_size = png_reader // TODO: error
      .output_buffer_size()
      .expect("frame buffer length should not exceed isize::MAX");

    let mut frame_buffer = vec![0; frame_buffer_size];

    png_reader.next_frame(&mut frame_buffer)?;
    png_reader.finish()?;

    let pixmap = match color_type {
      ColorType::Indexed => {
        use precursor_pixmap::{IndexedPixel, IndexedPixmap, RgbPixel};

        let indexed_palette = indexed_palette.unwrap();
        let (chunks, []) = indexed_palette.as_chunks::<3>() else {
          unreachable!("RgbPixel buffer should always be % 3")
        };

        let pixels = frame_buffer
          .into_iter()
          .map(|i| IndexedPixel { i })
          .collect();

        let palette = chunks
          .into_iter()
          .map(|chunk| RgbPixel::from_bytes(*chunk))
          .collect();

        let trns = indexed_alpha.and_then(|bytes| Some(bytes.into_owned()));

        DynamicPixmap::Indexed(IndexedPixmap::new(
          width, height, pixels, palette, trns,
        ))
      }

      ColorType::Luma => {
        use precursor_pixmap::{LumaPixel, LumaPixmap};

        let pixels =
          frame_buffer.into_iter().map(|y| LumaPixel { y }).collect();

        DynamicPixmap::Luma(LumaPixmap::new(width, height, pixels))
      }

      ColorType::LumaAlpha => {
        use precursor_pixmap::{LumaAlphaPixel, LumaAlphaPixmap};

        let (chunks, []) = frame_buffer.as_chunks::<2>() else {
          unreachable!("LumaAlphaPixel buffer should always be % 2")
        };

        let pixels = chunks
          .into_iter()
          .map(|chunk| LumaAlphaPixel::from_bytes(*chunk))
          .collect();

        DynamicPixmap::LumaAlpha(LumaAlphaPixmap::new(width, height, pixels))
      }

      ColorType::Rgb => {
        use precursor_pixmap::{RgbPixel, RgbPixmap};

        let (chunks, []) = frame_buffer.as_chunks::<3>() else {
          unreachable!("RgbPixel buffer should always be % 3")
        };

        let pixels = chunks
          .into_iter()
          .map(|chunk| RgbPixel::from_bytes(*chunk))
          .collect();

        DynamicPixmap::Rgb(RgbPixmap::new(width, height, pixels))
      }

      ColorType::RgbAlpha => {
        use precursor_pixmap::{RgbAlphaPixel, RgbAlphaPixmap};

        let (chunks, []) = frame_buffer.as_chunks::<4>() else {
          unreachable!("RgbAlphaPixel buffer should always be % 4")
        };

        let pixels = chunks
          .into_iter()
          .map(|chunk| RgbAlphaPixel::from_bytes(*chunk))
          .collect();

        DynamicPixmap::RgbAlpha(RgbAlphaPixmap::new(width, height, pixels))
      }
    };

    Ok(Self { pixmap })
  }

  /// Writes a PNG file to `writer`.
  ///
  /// # Errors
  ///
  /// See the [`WriteError`] enum for details.
  pub fn write<W: Write>(self, writer: &mut W) -> WriteResult<()> {
    let (width, height) = self.pixmap.dimensions();

    let mut encoder = Encoder::new(writer, width, height);

    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_compression(Compression::High);

    let color_type = match &self.pixmap {
      DynamicPixmap::Luma(_) => png::ColorType::Grayscale,
      DynamicPixmap::LumaAlpha(_) => png::ColorType::GrayscaleAlpha,
      DynamicPixmap::Rgb(_) => png::ColorType::Rgb,
      DynamicPixmap::RgbAlpha(_) => png::ColorType::Rgba,
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

        png::ColorType::Indexed
      }
    };

    encoder.set_color(color_type);

    let mut png_writer = encoder.write_header()?;

    png_writer.write_image_data(&self.pixmap.concat())?;
    png_writer.finish()
  }

  /// Converts this type into a [`DynamicPixmap`].
  pub fn into_pixmap(self) -> DynamicPixmap {
    self.pixmap
  }
}
