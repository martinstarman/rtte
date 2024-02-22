use crate::component::{
  enemy::{EnemyBundle, EnemyComponent},
  movement::MovementComponent,
  position::PositionComponent,
  selection::SelectionComponent,
  size::SizeComponent,
  sprite::SpriteComponent,
  view::{Shift, ViewComponent},
};
use bevy_ecs::component::ComponentId;
use macroquad::{math::Vec2, texture::load_texture};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EnemyEntity {
  image: String,
  position: (f32, f32),
  path: Vec<(f32, f32)>,
  view_direction: f32,
}

impl EnemyEntity {
  pub async fn into(&self, index: usize) -> EnemyBundle {
    let image = load_texture(self.image.as_str()).await.unwrap();
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
        width: image.width(),
        height: image.height(),
      },
      sprite: SpriteComponent {
        image,
        ysorted: true,
      },
      movement: MovementComponent {
        current_path: path.clone(),
        default_path: path.clone(),
      },
      view: ViewComponent {
        points: vec![],
        current_direction: self.view_direction,
        default_direction: self.view_direction,
        shift: Shift::LEFT,
      },
      enemy: EnemyComponent {
        id: ComponentId::new(index),
      },
      selection: SelectionComponent { active: false },
    }
  }
}
