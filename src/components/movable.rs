use bevy_ecs::prelude::*;
use ggez::mint::Point2;

// Movable component.
#[derive(Component)]
pub struct Movable {
  /// Current movement path. This resets to default_path if empty.
  pub path: Vec<Point2<f32>>,

  /// Default movement path.
  pub path_default: Vec<Point2<f32>>,
}
