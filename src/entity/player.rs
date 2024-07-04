use std::str::FromStr;

use crate::{
  component::{
    animation::AnimationComponent,
    body::BodyComponent,
    melee_attack::MeleeAttackComponent,
    movement::MovementComponent,
    player::{PlayerBundle, PlayerComponent},
    position::PositionComponent,
    selection::SelectionComponent,
    size::SizeComponent,
    sprite::{SpriteBundle, SpriteComponent},
  },
  constants::MOVEMENT_SPEED,
  shared::{direction::Direction, movement::Movement},
};
use bevy_ecs::component::ComponentId;
use macroquad::texture::load_texture;
use rapier2d::prelude::nalgebra;
use rapier2d::{dynamics::RigidBodyType, prelude::vector};
use rapier2d::{
  dynamics::{RigidBodyBuilder, RigidBodySet},
  geometry::{ColliderBuilder, ColliderSet},
};
use serde::Deserialize;

use super::shared::animation::Animation;

#[derive(Deserialize)]
pub struct PlayerEntity {
  image: String,
  position: (f32, f32),
  animation: Animation,
}

impl PlayerEntity {
  pub async fn into(
    &self,
    index: usize,
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
  ) -> PlayerBundle {
    let texture = load_texture(self.image.as_str()).await.unwrap();

    let animation = AnimationComponent {
      active: true,
      frame: 0,
      frame_delay: self.animation.frame_delay,
      frame_height: self.animation.frame_height,
      frame_width: self.animation.frame_width,
      movements: self.animation.movements.iter().map(|m| Movement::from_str(m).unwrap()).collect(),
      directions: self
        .animation
        .directions
        .iter()
        .map(|s| Direction::from_str(&s).unwrap())
        .collect(),
    };

    let rigid_body = RigidBodyBuilder::new(RigidBodyType::KinematicPositionBased)
      .position(vector![self.position.0, self.position.1].into())
      .build();

    let rigid_body_handle = rigid_body_set.insert(rigid_body);

    let collider = ColliderBuilder::capsule_y(6., 6.).build(); // TODO: width and height

    let collider_handle =
      collider_set.insert_with_parent(collider, rigid_body_handle, rigid_body_set);

    let direction: Direction = Direction::from_str(&self.animation.default_direction).unwrap();

    PlayerBundle {
      body: BodyComponent {
        collider_handle,
        rigid_body_handle,
      },
      melee_attack: MeleeAttackComponent {
        active: false,
        enemy_id: None,
      },
      movement: MovementComponent {
        default_path: vec![],
        direction,
        movement: Movement::Idling,
        path: vec![],
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
