use bevy_ecs::system::Resource;
use maths_rs::vec::Vec2;

#[derive(Resource)]
pub struct Mark {
  pub position: Option<Vec2<f32>>,
}
