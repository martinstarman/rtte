use std::f32::consts::PI;

use crate::{
  component::{movement::MovementComponent, position::PositionComponent},
  shared::direction::Direction,
};
use bevy_ecs::system::Query;

pub fn direction(mut query: Query<(&PositionComponent, &mut MovementComponent)>) {
  for (position, mut movement) in &mut query {
    if movement.path.len() > 0 {
      let dy = movement.path[0].y - position.y;
      let dx = movement.path[0].x - position.x;

      let a = dy.atan2(dx) * 180.0 / PI;

      let mut direction: Direction = Direction::North;

      if a >= 0.0 && a < 22.5 {
        direction = Direction::East;
      } else if a >= 22.5 && a < 67.5 {
        direction = Direction::SouthEast;
      } else if a >= 67.5 && a < 112.5 {
        direction = Direction::South;
      } else if a >= 112.5 && a < 157.5 {
        direction = Direction::SouthWest;
      } else if a >= 157.5 && a <= 180.0 {
        direction = Direction::West;
      }

      if a <= 0.0 && a > -22.5 {
        direction = Direction::East;
      } else if a <= -22.5 && a > -67.5 {
        direction = Direction::NorthEast;
      } else if a <= -67.5 && a > -112.5 {
        direction = Direction::North;
      } else if a <= -112.5 && a > -157.5 {
        direction = Direction::NorthWest;
      } else if a <= -157.5 && a >= -180.0 {
        direction = Direction::West;
      }

      if movement.direction != direction {
        movement.direction = direction;
      }
    }
  }
}
