use bevy_ecs::component::Component;

#[derive(Component)]
pub struct SelectionComponent {
  pub active: bool,
}
