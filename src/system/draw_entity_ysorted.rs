use crate::{
  component::{position::PositionComponent, size::SizeComponent, sprite::SpriteComponent},
  resource::offset::Offset,
};
use bevy_ecs::system::{Query, Res};
use macroquad::{color::WHITE, texture::draw_texture};

pub fn draw_entity_ysorted(
  query: Query<(&PositionComponent, &SizeComponent, &SpriteComponent)>,
  offset: Res<Offset>,
) {
  let mut entities: Vec<_> = query.iter().filter(|(_, _, sprite)| sprite.ysorted == true).collect();

  entities.sort_by(|(a_position, a_size, _), (b_position, b_size, _)| {
    (a_position.y + a_size.height).partial_cmp(&(b_position.y + b_size.height)).unwrap()
  });

  for (position, _, sprite) in entities {
    draw_texture(&sprite.texture, position.x - offset.x, position.y - offset.y, WHITE);
  }
}
