use bevy_ecs::component::Component;

#[derive(Component)]
pub struct AnimationComponent {
  pub active: bool,
  pub frame: i32,
  pub frame_delay: i32,
  pub frame_height: i32,
  pub frame_row: i32,
  pub frame_width: i32,
}

impl Default for AnimationComponent {
  fn default() -> Self {
    AnimationComponent {
      active: false,
      frame: 0,
      frame_delay: 0,
      frame_height: 0,
      frame_row: 0,
      frame_width: 0,
    }
  }
}
