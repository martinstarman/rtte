use crate::component::{animation::AnimationComponent, sprite::SpriteComponent};
use bevy_ecs::system::Query;

pub fn animation(mut query: Query<(&SpriteComponent, &mut AnimationComponent)>) {
  for (sprite, mut animation) in &mut query {
    if animation.active {
      animation.frame += 1;

      if animation.frame / animation.frame_delay
        >= sprite.texture.width() as i32 / animation.frame_width
      {
        animation.frame = 0;
      }
    }
  }
}
