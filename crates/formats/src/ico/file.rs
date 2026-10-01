use std::io::{self, Read, Seek, Write};

use thiserror::Error;

use crate::ico::dir::IconDirEntry;

use super::dir::IconDir;
use super::macros::wrap_u8;
use super::traits::IcoDir;

/// The error type for ICO reads.
#[derive(Debug, Error)]
pub enum ReadError {
  #[error("{0}")]
  IoError(#[from] io::Error),

  #[error("missing reserved byte in file header")]
  HeaderMissingReserved,

  #[error("expected resource type to be {0}, was {1}")]
  InvalidResourceType(u16, u16),

  #[error("missing reserved byte in entry {0}")]
  EntryMissingReserved(u16),
}

pub type ReadResult<T> = Result<T, ReadError>;

pub struct IcoFile(IconDir);

impl IcoFile {
  /// Constructs a new `IcoFile`.
  pub fn new(images: impl IntoIterator<Item = IcoImage>) -> Self {
    let images = images.into_iter().map(From::from).collect();
    Self(IconDir::new(images))
  }

  /// Reads an ICO file from `reader`, returning the constructed `IcoFile`.
  ///
  /// # Errors
  ///
  /// TODO
  pub fn read<R: Read + Seek>(reader: &mut R) -> ReadResult<Self> {
    Ok(Self(IconDir::read(reader)?))
  }

  /// Writes an ICO file to `writer`, returning how many bytes were written.
  ///
  /// # Errors
  ///
  /// This method returns the same errors as [`Write::write_all`].
  pub fn write<W: Write>(self, writer: &mut W) -> io::Result<usize> {
    self.0.write(writer)
  }

  /// Returns how many bytes will be written by [`write`].
  ///
  /// [`write`]: Self::write
  pub fn exact_size(&self) -> usize {
    self.0.exact_size()
  }
}

pub struct IcoImage {
  pub width: u16,
  pub height: u16,
  pub color_count: u16,
  pub color_planes: u16,
  pub bit_depth: u16,
  pub buffer: Box<[u8]>,
}

impl From<IcoImage> for (IconDirEntry, Box<[u8]>) {
  fn from(image: IcoImage) -> Self {
    let entry = IconDirEntry {
      width: wrap_u8!(image.width),
      height: wrap_u8!(image.height),
      color_count: wrap_u8!(image.color_count),
      color_planes: image.color_planes,
      bit_depth: image.bit_depth,
    };
    (entry, image.buffer)
  }
}
