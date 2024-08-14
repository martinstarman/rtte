use bevy::prelude::*;
use std::convert::TryFrom;

#[derive(Component)]
pub struct Direction {
  pub value: Directions,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Directions {
  East = 0,
  NorthEast = 1,
  North = 2,
  NorthWest = 3,
  West = 4,
  SouthWest = 5,
  South = 6,
  SouthEast = 7,
}

impl TryFrom<f32> for Directions {
  type Error = ();

  fn try_from(value: f32) -> Result<Self, Self::Error> {
    // see https://gamedev.stackexchange.com/a/49300
    let octant = (8. * value / (2. * std::f32::consts::PI) + 8.).round() as i32 % 8;

    match octant {
      0 => Ok(Directions::East),
      1 => Ok(Directions::NorthEast),
      2 => Ok(Directions::North),
      3 => Ok(Directions::NorthWest),
      4 => Ok(Directions::West),
      5 => Ok(Directions::SouthWest),
      6 => Ok(Directions::South),
      7 => Ok(Directions::SouthEast),
      _ => Err(()),
    }
  }
}
