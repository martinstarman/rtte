use bevy_ecs::prelude::*;

// Renderable component.
#[derive(Default, Component)]
pub struct Renderable {
  ///
  pub sprite: String,
}
