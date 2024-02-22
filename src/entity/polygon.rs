use crate::component::polygon::{PolygonBundle, PolygonComponent, Type};
use bevy_ecs::component::ComponentId;
use macroquad::math::Vec2;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PolygonEntity {
  points: Vec<(f32, f32)>,
  r#type: String,
}

impl PolygonEntity {
  pub fn into(&self, index: usize) -> PolygonBundle {
    let mut closed_polygon: Vec<(Vec2, Vec2)> = vec![]; // TODO: polygon
    
    let polygon_type = match self.r#type.as_str() { // TODO: type
      "transparent" => Type::TRANSPARENT,
      "water" => Type::WATER,
      "snow" => Type::SNOW,
      _ => Type::BLOCK,
    };

    if self.points.len() >= 3 {
      for i in 0..self.points.len() - 1 {
        let curr = self.points[i];
        let next = self.points[i + 1];

        closed_polygon.push((Vec2::new(curr.0, curr.1), Vec2::new(next.0, next.1)));
      }

      let first = self.points.first().unwrap();
      let last = self.points.last().unwrap();

      closed_polygon.push((Vec2::new(last.0, last.1), Vec2::new(first.0, first.1)));
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
