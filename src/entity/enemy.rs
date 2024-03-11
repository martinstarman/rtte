use crate::{
  component::{
    enemy::{EnemyBundle, EnemyComponent},
    field_of_view::{FieldOfViewComponent, Shift},
    movement::MovementComponent,
    position::PositionComponent,
    selection::SelectionComponent,
    size::SizeComponent,
    sprite::SpriteComponent,
  },
  constants::MOVEMENT_SPEED,
};
use bevy_ecs::component::ComponentId;
use macroquad::{math::Vec2, texture::load_texture};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EnemyEntity {
  image: String,
  position: (f32, f32),
  path: Vec<(f32, f32)>,
  field_of_view_direction: f32,
}

impl EnemyEntity {
  pub async fn into(&self, index: usize) -> EnemyBundle {
    let texture = load_texture(self.image.as_str()).await.unwrap();
    let mut path: Vec<Vec2> = vec![];

    for point in &self.path {
      path.push(Vec2::new(point.0, point.1));
    }

    EnemyBundle {
      position: PositionComponent {
        x: self.position.0,
        y: self.position.1,
      },
      size: SizeComponent {
        width: texture.width(),
        height: texture.height(),
      },
      sprite: SpriteComponent {
        texture,
        ysorted: true,
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
