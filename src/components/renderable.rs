use bevy_ecs::prelude::*;
use ggez::graphics::Image;

// Renderable component.
#[derive(Component)]
pub struct Renderable {
  /// Sprite path.
  pub sprite: Image,

  /// Should be Y indexed?
  pub y_indexed: bool,
}
