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
use ggez::{graphics::Image, mint::Point2, Context};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EnemyEntity {
  image: String,
  position: (f32, f32),
  path: Vec<(f32, f32)>,
  view_direction: f32,
}

impl EnemyEntity {
  pub fn into(&self, index: usize, ctx: &mut Context) -> EnemyBundle {
    let image = Image::from_path(ctx, self.image.clone()).unwrap();
    let mut path: Vec<Point2<f32>> = vec![];

    for point in &self.path {
      path.push(Point2 {
        x: point.0,
        y: point.1,
      });
    }

    EnemyBundle {
      position: PositionComponent {
        x: self.position.0,
        y: self.position.1,
      },
      size: SizeComponent {
        width: image.width() as f32,
        height: image.height() as f32,
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
        polygon: vec![],
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
