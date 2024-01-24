use crate::component::polygon::{PolygonBundle, PolygonComponent, Type};
use bevy_ecs::component::ComponentId;
use ggez::{mint::Point2, Context};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PolygonEntity {
  points: Vec<(f32, f32)>,
  r#type: String,
}

impl PolygonEntity {
  pub fn into(&self, index: usize, _ctx: &mut Context) -> PolygonBundle {
    let mut closed_polygon: Vec<(Point2<f32>, Point2<f32>)> = vec![];
    let polygon_type = match self.r#type.as_str() {
      "block" => Type::BLOCK,
      "transparent" => Type::TRANSPARENT,
      "water" => Type::WATER,
      "snow" => Type::SNOW,
      _ => Type::GROUND,
    };

    if self.points.len() >= 3 {
      for i in 0..self.points.len() - 1 {
        let curr = self.points[i];
        let next = self.points[i + 1];

        closed_polygon.push((
          Point2 {
            x: curr.0,
            y: curr.1,
          },
          Point2 {
            x: next.0,
            y: next.1,
          },
        ));
      }

      let first = self.points.first().unwrap();
      let last = self.points.last().unwrap();

      closed_polygon.push((
        Point2 {
          x: last.0,
          y: last.1,
        },
        Point2 {
          x: first.0,
          y: first.1,
        },
      ));
    }

    PolygonBundle {
      polygon: PolygonComponent {
        id: ComponentId::new(index),
        polygon: closed_polygon,
        r#type: polygon_type,
      },
    }
  }
}
