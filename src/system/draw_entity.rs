use crate::{
  component::{position::PositionComponent, size::SizeComponent, sprite::SpriteComponent},
  resource::offset::Offset,
};
use bevy_ecs::system::{Query, Res};
use macroquad::{color::WHITE, math::Vec2, texture::draw_texture};

pub fn run(
  query: Query<(&PositionComponent, &SizeComponent, &SpriteComponent)>, // TODO: remove SizeComponent
  offset: Res<Offset>,
) {
  let entities: Vec<_> = query.iter().filter(|(_, _, sprite)| sprite.ysorted == false).collect();

  for (position, _, sprite) in entities {
    let dest = Vec2 {
      x: position.x - offset.x,
      y: position.y - offset.y,
    };

    draw_texture(&sprite.image, dest.x, dest.y, WHITE);
  }
}
