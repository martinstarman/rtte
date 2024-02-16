use bevy_ecs::system::Resource;
use macroquad::math::Vec2;

#[derive(Resource)]
pub struct Mark {
  pub position: Option<Vec2>,
}
