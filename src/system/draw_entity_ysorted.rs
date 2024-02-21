use crate::{
  component::{position::PositionComponent, size::SizeComponent, sprite::SpriteComponent},
  resource::offset::Offset,
};
use bevy_ecs::system::{Query, Res};
use macroquad::{color::WHITE, math::Vec2, texture::draw_texture};

pub fn run(
  query: Query<(&PositionComponent, &SizeComponent, &SpriteComponent)>,
  offset: Res<Offset>,
) {
  let mut entities: Vec<_> = query.iter().filter(|(_, _, sprite)| sprite.ysorted == true).collect();

  entities.sort_by(|(a_position, a_size, _), (b_position, b_size, _)| {
    (a_position.y + a_size.height).partial_cmp(&(b_position.y + b_size.height)).unwrap()
  });

  for (position, _, sprite) in entities {
    let dest = Vec2 {
      x: position.x - offset.x,
      y: position.y - offset.y,
    };

    draw_texture(&sprite.image, dest.x, dest.y, WHITE);
  }
}
