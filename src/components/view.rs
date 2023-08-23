use bevy_ecs::prelude::*;

/// View component.
#[derive(Component)]
pub struct View {
  /// View x position.
  pub x: f32,

  /// View y position.
  pub y: f32,
}
