use bevy_ecs::component::Component;

#[derive(Component)]
pub struct AnimationComponent {
  pub is_animated: bool,
  pub frame: i32, // TODO: row, col
  pub frame_width: f32,
  pub frame_height: f32,
}
