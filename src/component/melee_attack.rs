use bevy_ecs::component::{Component, ComponentId};

#[derive(Component)]
pub struct MeleeAttackComponent {
  pub active: bool,
  pub enemy_id: Option<ComponentId>,
  // TODO: type?
}
