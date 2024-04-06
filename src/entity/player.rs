use std::str::FromStr;

use crate::{
  component::{
    animation::{AnimationComponent, Walk},
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

use super::shared::{animation::Animation, direction::Direction};

#[derive(Deserialize)]
pub struct PlayerEntity {
  image: String,
  position: (f32, f32),
  animation: Animation,
}

impl PlayerEntity {
  pub async fn into(&self, index: usize) -> PlayerBundle {
    let texture = load_texture(self.image.as_str()).await.unwrap();

    let mut animation = AnimationComponent {
      active: true,
      frame: 0,
      frame_delay: self.animation.frame_delay,
      frame_height: self.animation.frame_height,
      frame_row: 0,
      frame_width: self.animation.frame_width,
      walk: Walk {
        frame_row: self.animation.walk.frame_row,
        directions: self
          .animation
          .walk
          .directions
          .iter()
          .map(|s| Direction::from_str(&s).unwrap())
          .collect(),
      },
    };
    let default_direction: Direction = Direction::from_str(&self.animation.direction).unwrap();
    animation.frame_row = animation.walk.frame_row
      + animation.walk.directions.iter().position(|d| d == &default_direction).unwrap() as i32;

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
