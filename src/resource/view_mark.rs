use bevy_ecs::system::Resource;
use macroquad::math::Vec2;

#[derive(Resource)]
pub struct ViewMark {
  pub position: Option<Vec2>,
}
