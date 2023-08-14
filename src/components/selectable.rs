use bevy_ecs::prelude::*;

// Selectable component.
#[derive(Default, Component)]
pub struct Selectable {
  ///
  pub selected: bool,
}
