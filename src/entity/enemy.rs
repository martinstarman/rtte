use crate::{
  component::{
    animation::AnimationComponent,
    body::BodyComponent,
    enemy::{EnemyBundle, EnemyComponent},
    field_of_view::{FieldOfViewComponent, Shift},
    movement::MovementComponent,
    position::PositionComponent,
    selection::SelectionComponent,
    size::SizeComponent,
    sprite::{SpriteBundle, SpriteComponent},
  },
  constants::MOVEMENT_SPEED,
};
use bevy_ecs::component::ComponentId;
use macroquad::{math::Vec2, texture::load_texture};
use rapier2d::{
  dynamics::{RigidBodyBuilder, RigidBodySet},
  geometry::{ColliderBuilder, ColliderSet},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EnemyEntity {
  image: String,
  position: (f32, f32),
  path: Vec<(f32, f32)>,
  field_of_view_direction: f32,
}

impl EnemyEntity {
  pub async fn into(
    &self,
    index: usize,
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
  ) -> EnemyBundle {
    let texture = load_texture(self.image.as_str()).await.unwrap();
    let mut path: Vec<Vec2> = vec![];
    let animation = AnimationComponent::default(); // TODO: implement me

    for point in &self.path {
      path.push(Vec2::new(point.0, point.1));
    }

    let rigid_body = RigidBodyBuilder::kinematic_position_based().build();
    let rigid_body_handle = rigid_body_set.insert(rigid_body);

    let collider = ColliderBuilder::ball(24.).build(); // TODO: capsule
    let collider_handle =
      collider_set.insert_with_parent(collider, rigid_body_handle, rigid_body_set);

    EnemyBundle {
      body: BodyComponent {
        collider_handle,
        rigid_body_handle,
      },
      position: PositionComponent {
        x: self.position.0,
        y: self.position.1,
      },
      size: SizeComponent {
        width: texture.width(),
        height: texture.height(),
      },
      sprite: SpriteBundle {
        sprite: SpriteComponent {
          texture,
          ysorted: true,
        },
        animation,
      },
      movement: MovementComponent {
        path: path.clone(),
        default_path: path.clone(),
        speed: MOVEMENT_SPEED,
      },
      field_of_view: FieldOfViewComponent {
        points: vec![],
        direction: self.field_of_view_direction,
        movement_direction: self.field_of_view_direction,
        shift: Shift::LEFT,
      },
      enemy: EnemyComponent {
        id: ComponentId::new(index),
      },
      selection: SelectionComponent { active: false },
    }
  }
}
