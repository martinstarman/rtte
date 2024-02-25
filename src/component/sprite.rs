use bevy_ecs::component::Component;
use macroquad::texture::Texture2D;

#[derive(Component)]
pub struct SpriteComponent {
  pub texture: Texture2D,
  pub ysorted: bool,
}
