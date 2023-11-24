use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct SizeComponent {
  pub height: f32,
  pub width: f32,
}
