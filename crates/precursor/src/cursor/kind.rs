use std::ffi::OsStr;
use std::io::{self, Read, Seek};

use crate::args::CursorKindHint;

pub enum CursorKind {
  Ani,
  Cur,
  Svg,
  Xcur,
}

impl CursorKind {
  const ANI_EXT: &str = "ani";
  const CUR_EXT: &str = "cur";
  const SVG_EXT: &str = "svg";

  pub fn from_ext(ext: &OsStr) -> Option<Self> {
    if ext == OsStr::new(Self::ANI_EXT) {
      Some(Self::Ani)
    } else if ext == OsStr::new(Self::CUR_EXT) {
      Some(Self::Cur)
    } else if ext == OsStr::new(Self::SVG_EXT) {
      Some(Self::Svg)
    } else {
      None
    }
  }

  pub fn from_bytes(buf: &[u8]) -> Option<Self> {
    // order by highest confidence
    if buf.len() < 12 {
      None
    } else if &buf[0..3] == b"RIFF" && &buf[8..11] == b"ACON" {
      Some(Self::Ani)
    } else if &buf[0..3] == b"Xcur" {
      Some(Self::Xcur)
    } else if &buf[0..4] == &[0, 0, 2, 0] {
      Some(Self::Cur)
    } else {
      None
    }
  }

  pub fn from_reader<R: Read + Seek>(
    reader: &mut R,
  ) -> io::Result<Option<Self>> {
    let mut buf = [0; 12];
    reader.read_exact(&mut buf)?;
    reader.rewind()?;
    Ok(Self::from_bytes(&buf))
  }
}

impl From<CursorKindHint> for CursorKind {
  fn from(value: CursorKindHint) -> Self {
    match value {
      CursorKindHint::Ani => Self::Ani,
      CursorKindHint::Cur => Self::Cur,
      CursorKindHint::Svg => Self::Svg,
      CursorKindHint::Xcur => Self::Xcur,
    }
  }
}

// TODO: tests
