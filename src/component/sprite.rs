use bevy_ecs::{bundle::Bundle, component::Component};
use macroquad::texture::Texture2D;

use super::animation::AnimationComponent;

#[derive(Component)]
pub struct SpriteComponent {
  pub texture: Texture2D,
  pub ysorted: bool,
}

#[derive(Bundle)]
pub struct SpriteBundle {
  pub sprite: SpriteComponent,
  pub animation: AnimationComponent,
}
