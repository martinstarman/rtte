use crate::component::{animation::AnimationComponent, sprite::SpriteComponent};
use bevy_ecs::system::Query;

// TODO: slow down
pub fn animation(mut query: Query<(&SpriteComponent, &mut AnimationComponent)>) {
  for (sprite, mut animation) in &mut query {
    let mut next_frame = animation.frame + 1;

    if next_frame as f32 > sprite.texture.width() / animation.frame_width {
      next_frame = 0;
    }

    animation.frame = next_frame;
  }
}
