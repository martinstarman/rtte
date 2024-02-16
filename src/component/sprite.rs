use bevy_ecs::prelude::*;
use macroquad::texture::Texture2D;

#[derive(Component)]
pub struct SpriteComponent {
  pub image: Texture2D,
  pub ysorted: bool,
}
