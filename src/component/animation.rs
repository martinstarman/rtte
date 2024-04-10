use bevy_ecs::component::Component;

use crate::entity::shared::direction::Direction;

#[derive(Component)]
pub struct AnimationComponent {
  pub active: bool, // TODO: remove me...
  pub frame: i32,
  pub frame_delay: i32,
  pub frame_height: i32,
  pub frame_row: i32,
  pub frame_width: i32,
  pub walk: Walk,
}

pub struct Walk {
  pub frame_row: i32,             // TODO: atlas_first_row?
  pub directions: Vec<Direction>, // TODO: direction_offset?
}

// TODO: remove me
impl Default for AnimationComponent {
  fn default() -> Self {
    AnimationComponent {
      active: false,
      frame: 0,
      frame_delay: 0,
      frame_height: 0,
      frame_row: 0,
      frame_width: 0,
      walk: Walk {
        frame_row: 0,
        directions: vec![],
      },
    }
  }
}
