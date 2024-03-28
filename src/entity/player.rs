use crate::{
  component::{
    animation::AnimationComponent,
    movement::MovementComponent,
    player::{PlayerBundle, PlayerComponent},
    position::PositionComponent,
    selection::SelectionComponent,
    size::SizeComponent,
    sprite::{SpriteBundle, SpriteComponent},
  },
  constants::MOVEMENT_SPEED,
};
use bevy_ecs::component::ComponentId;
use macroquad::texture::load_texture;
use serde::Deserialize;

use super::shared::animation::Animation;

#[derive(Deserialize)]
pub struct PlayerEntity {
  image: String,
  position: (f32, f32),
  animation: Option<Animation>,
}

impl PlayerEntity {
  pub async fn into(&self, index: usize) -> PlayerBundle {
    let texture = load_texture(self.image.as_str()).await.unwrap();

    let mut animation = AnimationComponent::default();

    if let Some(anim) = &self.animation {
      animation.active = true;
      animation.frame_delay = anim.frame_delay;
      animation.frame_height = anim.frame_height;
      animation.frame_row = anim.frame_row;
      animation.frame_width = anim.frame_width;
    }

    PlayerBundle {
      movement: MovementComponent {
        path: vec![],
        default_path: vec![],
        speed: MOVEMENT_SPEED,
      },
      player: PlayerComponent {
        id: ComponentId::new(index),
      },
      position: PositionComponent {
        x: self.position.0,
        y: self.position.1,
      },
      selection: SelectionComponent { active: false },
      size: SizeComponent {
        height: if animation.active { animation.frame_height as f32 } else { texture.height() },
        width: if animation.active { animation.frame_width as f32 } else { texture.width() },
      },
      sprite: SpriteBundle {
        sprite: SpriteComponent {
          texture,
          ysorted: true,
        },
        animation,
      },
    }
  }
}
