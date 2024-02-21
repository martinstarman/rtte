use bevy_ecs::component::Component;

#[derive(Component)]
pub struct SizeComponent {
  pub height: f32,
  pub width: f32,
}
