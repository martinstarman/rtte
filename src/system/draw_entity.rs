use crate::{
  component::{position::PositionComponent, sprite::SpriteComponent},
  resource::offset::Offset,
};
use bevy_ecs::system::{Query, Res};
use macroquad::{color::WHITE, texture::draw_texture};

pub fn draw_entity(query: Query<(&PositionComponent, &SpriteComponent)>, offset: Res<Offset>) {
  let entities: Vec<_> = query.iter().filter(|(_, sprite)| sprite.ysorted == false).collect();

  for (position, sprite) in entities {
    draw_texture(&sprite.texture, position.x - offset.x, position.y - offset.y, WHITE);
  }
}
