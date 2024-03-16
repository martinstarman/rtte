use crate::component::{
  object::{ObjectBundle, ObjectComponent},
  position::PositionComponent,
  shape::{ShapeComponent, ShapeType},
  size::SizeComponent,
  sprite::SpriteComponent,
};
use bevy_ecs::component::ComponentId;
use macroquad::{math::Vec2, texture::load_texture};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ObjectEntity {
  image: String,
  position: (f32, f32),
  shape: Vec<(f32, f32)>,
  r#type: String,
  ysorted: bool,
}

impl ObjectEntity {
  pub async fn into(&self, index: usize) -> ObjectBundle {
    let texture = load_texture(self.image.as_str()).await.unwrap();
    let mut points: Vec<Vec2> = vec![];
    let mut lines: Vec<(Vec2, Vec2)> = vec![];

    if self.shape.len() > 2 {
      for i in 0..self.shape.len() - 1 {
        let p = self.shape[i];
        let q = self.shape[i + 1];

        points.push(p.into());
        lines.push((p.into(), q.into()));
      }

      // close line
      let first = self.shape.first().unwrap();
      let last = self.shape.last().unwrap();

      lines.push((Vec2::new(last.0, last.1), Vec2::new(first.0, first.1)));
    }

    let r#type = match self.r#type.as_str() {
      "none" => ShapeType::NONE,
      "transparent" => ShapeType::TRANSPARENT,
      "water" => ShapeType::WATER,
      "snow" => ShapeType::SNOW,
      _ => ShapeType::BLOCK,
    };

    ObjectBundle {
      object: ObjectComponent {
        id: ComponentId::new(index),
      },
      position: PositionComponent {
        x: self.position.0,
        y: self.position.1,
      },
      shape: ShapeComponent {
        lines,
        points,
        r#type,
      },
      size: SizeComponent {
        height: texture.height(),
        width: texture.width(),
      },
      sprite: SpriteComponent {
        texture,
        ysorted: self.ysorted,
      },
    }
  }
}
