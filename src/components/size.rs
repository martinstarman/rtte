use bevy_ecs::prelude::*;

// Size component.
#[derive(Default, Component)]
pub struct Size {
  /// Width.
  pub w: f32,

  /// Height.
  pub h: f32,
}
