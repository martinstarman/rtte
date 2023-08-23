use crate::vec2::Vec2;
use bevy_ecs::prelude::*;

// Movable component.
#[derive(Component)]
pub struct Movable {
  /// Current movement path. This resets to default_path if empty.
  pub path: Vec<Vec2>,

  /// Default movement path.
  pub path_default: Vec<Vec2>,
}
