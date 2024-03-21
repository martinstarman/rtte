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

// TODO: this will be the same for enemy
#[derive(Deserialize)]
pub struct Animation {
  width: f32,
  height: f32,
}

#[derive(Deserialize)]
pub struct PlayerEntity {
  image: String,
  position: (f32, f32),
  animation: Option<Animation>,
}

impl PlayerEntity {
  pub async fn into(&self, index: usize) -> PlayerBundle {
    let texture = load_texture(self.image.as_str()).await.unwrap();

    let mut frame_width = 0.;
    let mut frame_height = 0.;

    if let Some(anim) = &self.animation {
      frame_width = anim.width;
      frame_height = anim.height;
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
        width: texture.width(),
        height: texture.height(),
      },
      sprite: SpriteBundle {
        sprite: SpriteComponent {
          texture,
          ysorted: true,
        },
        animation: AnimationComponent {
          is_animated: true,
          frame: 0,
          frame_width,
          frame_height,
        },
      },
    }
  }
}
