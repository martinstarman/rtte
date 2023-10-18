use bevy_ecs::prelude::*;
use ggez::graphics::Image;

#[derive(Component)]
pub struct Sprite {
  pub image: Image,
  pub ysorted: bool,
}
