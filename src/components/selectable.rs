use bevy_ecs::prelude::*;

// Selectable component.
#[derive(Component)]
pub struct Selectable {
  /// Is entity selected?
  pub selected: bool,
}
