use crate::ani::acon::{ChunkIdAconExt, FromChunkError, FromChunkResult};
use crate::containers::riff::{ChunkId, ChunkValue};

#[derive(Debug, PartialEq, Eq)]
pub struct AconHeader {
  pub frame_count: u32,
  pub step_count: u32,
  pub jif_rate: u32,
}

impl AconHeader {
  const HEADER_SIZE: u32 = 36;

  /// Constructs an `AconHeader` from a byte vector.
  ///
  /// # Errors
  ///
  /// TODO
  pub const fn from_bytes(bytes: &[u8]) -> FromChunkResult<Self> {
    if let actual_len = bytes.len()
      && actual_len != Self::HEADER_SIZE as usize
    {
      return Err(FromChunkError::InvalidChunkLength(
        Self::HEADER_SIZE as usize,
        actual_len,
      ));
    }

    if let header_size =
      u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
      && header_size != Self::HEADER_SIZE
    {
      return Err(FromChunkError::InvalidHeaderSize(header_size));
    }

    let frame_count =
      u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);

    let step_count =
      u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);

    let jif_rate =
      u32::from_le_bytes([bytes[28], bytes[29], bytes[30], bytes[31]]);

    Ok(Self {
      frame_count,
      step_count,
      jif_rate,
    })
  }

  /// Converts this value into a [`ChunkValue`].
  pub fn into_chunk(self) -> ChunkValue {
    let mut data = [0_u8; 36];

    data[..4].copy_from_slice(&Self::HEADER_SIZE.to_le_bytes());
    data[4..8].copy_from_slice(&self.frame_count.to_le_bytes());
    data[8..12].copy_from_slice(&self.step_count.to_le_bytes());
    // <-- unused/legacy fields --> //
    data[28..32].copy_from_slice(&self.jif_rate.to_le_bytes());
    data[32..36].copy_from_slice(&1_u32.to_le_bytes()); // seq chunk flag

    ChunkValue::Child(ChunkId::ANIH, Vec::from(data))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const VALID_BYTES: &[u8] = &[
    36, 00, 00, 00, // header size
    04, 00, 00, 00, // frame count
    05, 00, 00, 00, // step count
    //-- START LEGACY FIELDS --//
    00, 00, 00, 00, // hotspot x
    00, 00, 00, 00, // hotspot y
    00, 00, 00, 00, // bit count
    00, 00, 00, 00, // planes
    //--  END LEGACY FIELDS  --//
    30, 00, 00, 00, // jif rate
    01, 00, 00, 00, // flags
  ];

  const EXPECTED_HEADER: AconHeader = AconHeader {
    frame_count: 4,
    step_count: 5,
    jif_rate: 30,
  };

  #[test]
  fn from_valid_bytes() {
    assert_eq!(
      AconHeader::from_bytes(VALID_BYTES).unwrap(),
      EXPECTED_HEADER
    )
  }

  #[test]
  fn from_invalid_bytes() {
    let mut invalid_bytes = VALID_BYTES.to_vec();
    invalid_bytes[0] = 0;

    assert_eq!(
      AconHeader::from_bytes(&invalid_bytes)
        .unwrap_err()
        .to_string(),
      FromChunkError::InvalidHeaderSize(0).to_string()
    )
  }

  #[test]
  fn from_empty_bytes() {
    assert_eq!(
      AconHeader::from_bytes(&[]).unwrap_err().to_string(),
      FromChunkError::InvalidChunkLength(36, 0).to_string()
    )
  }

  #[test]
  fn into_chunk() {
    assert_eq!(
      EXPECTED_HEADER.into_chunk(),
      ChunkValue::Child(ChunkId::ANIH, VALID_BYTES.to_vec())
    )
  }
}
