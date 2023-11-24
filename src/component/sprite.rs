use bevy_ecs::prelude::*;
use ggez::graphics::Image;

#[derive(Component)]
pub struct SpriteComponent {
  pub image: Image,
  pub ysorted: bool,
}
