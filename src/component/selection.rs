use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct SelectionComponent {
  pub active: bool,
}
