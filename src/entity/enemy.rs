use crate::component::{
  enemy::{Enemy, EnemyBundle},
  movement::Movement,
  position::Position,
  selection::Selection,
  size::Size,
  sprite::Sprite,
  view::{Shift, View},
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
  pub fn to_component(&self, index: usize, ctx: &mut Context) -> EnemyBundle {
    let image = Image::from_path(ctx, self.image.clone()).unwrap();
    let mut path: Vec<Point2<f32>> = vec![];

    for point in &self.path {
      path.push(Point2 {
        x: point.0,
        y: point.1,
      });
    }

    EnemyBundle {
      position: Position {
        x: self.position.0,
        y: self.position.1,
      },
      size: Size {
        width: image.width() as f32,
        height: image.height() as f32,
      },
      sprite: Sprite {
        image,
        ysorted: true,
      },
      movement: Movement {
        current_path: path.clone(),
        default_path: path.clone(),
      },
      view: View {
        polygon: vec![],
        current_direction: self.view_direction,
        default_direction: self.view_direction,
        shift: Shift::LEFT,
      },
      enemy: Enemy {
        id: ComponentId::new(index),
      },
      selection: Selection { active: false },
    }
  }
}
