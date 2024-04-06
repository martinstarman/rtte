use std::str::FromStr;

use serde::Deserialize;

#[derive(Deserialize, Copy, Clone, PartialEq)]
pub enum Direction {
  North,
  NorthEast,
  East,
  SouthEast,
  South,
  SouthWest,
  West,
  NorthWest,
}

impl FromStr for Direction {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "N" => Ok(Direction::North),
      "NE" => Ok(Direction::NorthEast),
      "E" => Ok(Direction::East),
      "SE" => Ok(Direction::SouthEast),
      "S" => Ok(Direction::South),
      "SW" => Ok(Direction::SouthWest),
      "W" => Ok(Direction::West),
      "NW" => Ok(Direction::NorthWest),
      _ => Err(()),
    }
  }
}
