use bevy_ecs::system::Resource;
use macroquad::math::Rect;

#[derive(Resource)]
pub struct TargetArea {
  pub rect: Rect,
}
