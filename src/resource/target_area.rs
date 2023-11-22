use bevy_ecs::system::Resource;
use ggez::graphics::Rect;

#[derive(Resource)]
pub struct TargetArea {
  pub rect: Rect,
}
