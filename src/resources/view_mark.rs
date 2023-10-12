use bevy_ecs::system::Resource;

#[derive(Resource)]
pub struct ViewMark {
  pub active: bool,
  pub x: f32,
  pub y: f32,
}
