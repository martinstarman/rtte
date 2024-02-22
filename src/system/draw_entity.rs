use crate::{
  component::{position::PositionComponent, sprite::SpriteComponent},
  resource::offset::Offset,
};
use bevy_ecs::system::{Query, Res};
use macroquad::{color::WHITE, math::Vec2, texture::draw_texture};

pub fn draw_entity(query: Query<(&PositionComponent, &SpriteComponent)>, offset: Res<Offset>) {
  let entities: Vec<_> = query.iter().filter(|(_, sprite)| sprite.ysorted == false).collect();

  for (position, sprite) in entities {
    let dest = Vec2 {
      x: position.x - offset.x,
      y: position.y - offset.y,
    };

    draw_texture(&sprite.image, dest.x, dest.y, WHITE);
  }
}
