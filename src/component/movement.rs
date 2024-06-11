use bevy_ecs::component::Component;
use macroquad::math::Vec2;

use crate::shared::{direction::Direction, movement::Movement};

#[derive(Component)]
pub struct MovementComponent {
  pub default_path: Vec<Vec2>,
  pub direction: Direction,
  pub movement: Movement,
  pub path: Vec<Vec2>,
  pub speed: f32,
}

impl Default for MovementComponent {
  fn default() -> Self {
    MovementComponent {
      default_path: vec![],
      direction: Direction::North,
      movement: Movement::Idling,
      path: vec![],
      speed: 0.,
    }
  }
}
