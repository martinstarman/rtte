use bevy_ecs::component::Component;

use crate::shared::{direction::Direction, movement::Movement};

#[derive(Component)]
pub struct AnimationComponent {
  pub active: bool,
  pub frame: i32,
  pub frame_delay: i32,
  pub frame_height: i32,
  pub frame_width: i32,
  pub movements: Vec<Movement>,
  pub directions: Vec<Direction>,
}

impl Default for AnimationComponent {
  fn default() -> Self {
    AnimationComponent {
      active: false,
      frame: 0,
      frame_delay: 0,
      frame_height: 0,
      frame_width: 0,
      movements: vec![],
      directions: vec![],
    }
  }
}
