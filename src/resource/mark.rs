use bevy_ecs::system::Resource;
use macroquad::{math::Vec2, texture::Texture2D};

#[derive(Resource)]
pub struct Mark {
  pub position: Option<Vec2>,
  pub texture: Texture2D,
}
