use bevy_ecs::system::Resource;

#[derive(Resource)]
pub struct Offset {
  pub x: f32,
  pub y: f32,
}
