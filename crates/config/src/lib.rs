mod cursors;
mod package;

pub use cursors::*;
pub use package::*;

use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
  pub package: PackageConfig,
  pub cursors: BTreeMap<String, CursorConfig>,
}

#[cfg(test)]
mod tests {
  use super::*;

  use std::path::PathBuf;
  use std::str::FromStr;

  use crate_cursor::{CursorDuration, CursorHotspot};

  #[test]
  fn test_deserialize_from_document() {
    let raw_value = r#"
      [package]
      name = "test"
      comment = "testing"

      # static #

      [cursors.hand1]
      nominal = 24
      hotspot = [4, 4]
      asset = "testing/hand.png"

      [cursors.hand2]
      nominal = 24
      hotspot = [4, 4]
      asset = "testing/hand.png"
      aliases = ["linux:foo", "windows:bar"]

      # animated #

      [cursors.wait1]
      nominal = 24
      hotspot = [4, 4]
      sequence = [0, 1, 2, 3, 0]
      duration = 1000
      assets = [
        "testing/wait-01.png",
        "testing/wait-02.png",
        "testing/wait-03.png",
        "testing/wait-04.png",
      ]

      [cursors.wait2]
      nominal = 24
      hotspot = [4, 4]
      sequence = [0, 1, 2, 3, 0]
      durations = [200, 200, 200, 200, 200]
      assets = [
        "testing/wait-01.png",
        "testing/wait-02.png",
        "testing/wait-03.png",
        "testing/wait-04.png",
      ]
    "#;

    let value = toml::from_str::<Config>(raw_value).unwrap();

    let hand1 = CursorConfig::from(ScaledStaticCursorConfig {
      nominal: 24,
      hotspot: CursorHotspot { x: 4, y: 4 },
      asset: PathBuf::from("testing/hand.png").into(),
      aliases: None,
    });

    let hand2 = CursorConfig::from(ScaledStaticCursorConfig {
      nominal: 24,
      hotspot: CursorHotspot { x: 4, y: 4 },
      asset: PathBuf::from("testing/hand.png").into(),
      aliases: Some(Vec::from([
        PlatformAlias::Linux("foo".into()),
        PlatformAlias::Windows("bar".into()),
      ])),
    });

    let wait1 = CursorConfig::from(ScaledAnimatedCursorConfig {
      nominal: 24,
      hotspot: CursorHotspot { x: 4, y: 4 },
      duration: Some(CursorDuration::from_milliseconds(1000)),
      durations: None,
      sequence: Vec::from([
        ScaledCursorFrameConfig::from(0),
        ScaledCursorFrameConfig::from(1),
        ScaledCursorFrameConfig::from(2),
        ScaledCursorFrameConfig::from(3),
        ScaledCursorFrameConfig::from(0),
      ]),
      assets: Vec::from([
        PathBuf::from("testing/wait-01.png").into(),
        PathBuf::from("testing/wait-02.png").into(),
        PathBuf::from("testing/wait-03.png").into(),
        PathBuf::from("testing/wait-04.png").into(),
      ]),
      aliases: None,
    });

    let wait2 = CursorConfig::from(ScaledAnimatedCursorConfig {
      nominal: 24,
      hotspot: CursorHotspot { x: 4, y: 4 },
      duration: None,
      durations: Some(Vec::from([
        CursorDuration::from_milliseconds(200),
        CursorDuration::from_milliseconds(200),
        CursorDuration::from_milliseconds(200),
        CursorDuration::from_milliseconds(200),
        CursorDuration::from_milliseconds(200),
      ])),
      sequence: Vec::from([
        ScaledCursorFrameConfig::from(0),
        ScaledCursorFrameConfig::from(1),
        ScaledCursorFrameConfig::from(2),
        ScaledCursorFrameConfig::from(3),
        ScaledCursorFrameConfig::from(0),
      ]),
      assets: Vec::from([
        PathBuf::from("testing/wait-01.png").into(),
        PathBuf::from("testing/wait-02.png").into(),
        PathBuf::from("testing/wait-03.png").into(),
        PathBuf::from("testing/wait-04.png").into(),
      ]),
      aliases: None,
    });

    let expected = Config {
      package: PackageConfig {
        name: LocaleString::from_str("test").unwrap(),
        comment: LocaleString::from_str("testing").unwrap(),
        hidden: None,
        example: None,
      },
      cursors: BTreeMap::from([
        (String::from("hand1"), hand1),
        (String::from("hand2"), hand2),
        (String::from("wait1"), wait1),
        (String::from("wait2"), wait2),
      ]),
    };

    assert_eq!(value, expected)
  }
}
