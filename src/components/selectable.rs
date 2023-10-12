use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Selectable {
  pub selected: bool,
}
