use bevy_ecs::prelude::*;

/// Position component.
#[derive(Default, Component)]
pub struct Position {
  /// X position.
  pub x: f32,

  /// Y position.
  pub y: f32,
}
