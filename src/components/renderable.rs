use bevy_ecs::prelude::*;
use ggez::graphics::Image;

// Renderable component.
#[derive(Default, Component)]
pub struct Renderable {
  /// Sprite path.
  pub sprite: Option<Image>, // TODO: remove option, do not use default
}
