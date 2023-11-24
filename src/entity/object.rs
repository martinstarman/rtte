use crate::component::{
  object::{ObjectBundle, ObjectComponent, PolygonType},
  position::PositionComponent,
  size::SizeComponent,
  sprite::SpriteComponent,
};
use bevy_ecs::component::ComponentId;
use ggez::{graphics::Image, mint::Point2, Context};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ObjectEntity {
  image: String,
  position: (f32, f32),
  polygon: Vec<(f32, f32)>,
  polygon_type: String,
}

impl ObjectEntity {
  pub fn into(&self, index: usize, ctx: &mut Context) -> ObjectBundle {
    let image = Image::from_path(ctx, self.image.clone()).unwrap();
    let mut closed_polygon: Vec<(Point2<f32>, Point2<f32>)> = vec![];
    let polygon_type = match self.polygon_type.as_str() {
      "block" => PolygonType::BLOCK,
      "transparent" => PolygonType::TRANSPARENT,
      "water" => PolygonType::WATER,
      "snow" => PolygonType::SNOW,
      _ => PolygonType::GROUND,
    };

    if self.polygon.len() >= 3 {
      for i in 0..self.polygon.len() - 1 {
        let curr = self.polygon[i];
        let next = self.polygon[i + 1];

        closed_polygon.push((
          Point2 {
            x: curr.0 + self.position.0,
            y: curr.1 + self.position.1,
          },
          Point2 {
            x: next.0 + self.position.0,
            y: next.1 + self.position.1,
          },
        ));
      }

      let first = self.polygon.first().unwrap();
      let last = self.polygon.last().unwrap();

      closed_polygon.push((
        Point2 {
          x: last.0 + self.position.0,
          y: last.1 + self.position.1,
        },
        Point2 {
          x: first.0 + self.position.0,
          y: first.1 + self.position.1,
        },
      ));
    }

    ObjectBundle {
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
        ysorted: polygon_type != PolygonType::GROUND,
      },
      object: ObjectComponent {
        id: ComponentId::new(index),
        polygon: closed_polygon,
        polygon_type,
      },
    }
  }
}
