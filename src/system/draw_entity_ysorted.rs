use crate::{
  component::{
    animation::AnimationComponent, position::PositionComponent, size::SizeComponent,
    sprite::SpriteComponent,
  },
  resource::offset::Offset,
};
use bevy_ecs::system::{Query, Res};
use macroquad::{
  color::WHITE,
  math::{Rect, Vec2},
  texture::{draw_texture, draw_texture_ex, DrawTextureParams},
};

pub fn draw_entity_ysorted(
  query: Query<(&PositionComponent, &SizeComponent, &SpriteComponent, &AnimationComponent)>,
  offset: Res<Offset>,
) {
  let mut entities: Vec<_> =
    query.iter().filter(|(_, _, sprite, _)| sprite.ysorted == true).collect();

  entities.sort_by(|(a_position, a_size, _, _), (b_position, b_size, _, _)| {
    (a_position.y + a_size.height).partial_cmp(&(b_position.y + b_size.height)).unwrap()
  });

  for (position, _, sprite, animation) in entities {
    if animation.active {
      draw_texture_ex(
        &sprite.texture,
        position.x - offset.x,
        position.y - offset.y,
        WHITE,
        DrawTextureParams {
          dest_size: Some(Vec2::new(animation.frame_width as f32, animation.frame_height as f32)),
          source: Some(Rect::new(
            ((animation.frame / animation.frame_delay) * animation.frame_width) as f32,
            (animation.frame_row * animation.frame_height) as f32,
            animation.frame_width as f32,
            animation.frame_height as f32,
          )),
          ..Default::default()
        },
      );
    } else {
      draw_texture(&sprite.texture, position.x - offset.x, position.y - offset.y, WHITE);
    }
  }
}
