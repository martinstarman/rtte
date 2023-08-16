use bevy_ecs::prelude::*;

// Selectable component.
#[derive(Default, Component)]
pub struct Selectable {
  /// Is entity selected?
  pub selected: bool,
}
