use bevy::{math::CompassOctant, prelude::*};

#[derive(Component)]
pub struct Direction {
  pub value: CompassOctant,
}

impl Default for Direction {
  fn default() -> Self {
    Direction {
      value: CompassOctant::South,
    }
  }
}
