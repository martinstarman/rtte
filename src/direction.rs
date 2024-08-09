use std::convert::TryFrom;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Direction {
  East = 0,
  NorthEast = 1,
  North = 2,
  NorthWest = 3,
  West = 4,
  SouthWest = 5,
  South = 6,
  SouthEast = 7,
}

impl TryFrom<f32> for Direction {
  type Error = ();

  fn try_from(value: f32) -> Result<Self, Self::Error> {
    // see https://gamedev.stackexchange.com/a/49300
    let octant = (8. * value / (2. * std::f32::consts::PI) + 8.).round() as i32 % 8;

    match octant {
      0 => Ok(Direction::East),
      1 => Ok(Direction::NorthEast),
      2 => Ok(Direction::North),
      3 => Ok(Direction::NorthWest),
      4 => Ok(Direction::West),
      5 => Ok(Direction::SouthWest),
      6 => Ok(Direction::South),
      7 => Ok(Direction::SouthEast),
      _ => Err(()),
    }
  }
}
