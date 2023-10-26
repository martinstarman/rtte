use bevy_ecs::system::Resource;

#[derive(Resource)]
pub struct Mark {
  pub active: bool,
  pub x: f32,
  pub y: f32,
}
