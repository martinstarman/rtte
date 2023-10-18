use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Selection {
  pub active: bool,
}
