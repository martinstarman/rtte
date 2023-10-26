use crate::component::{
  object::{Object, ObjectBundle, PolygonType},
  position::Position,
  size::Size,
  sprite::Sprite,
};
use ggez::{graphics::Image, mint::Point2};

pub fn new(
  position: Position,
  image: Image,
  polygon: Vec<Point2<f32>>,
  polygon_type: PolygonType,
) -> ObjectBundle {
  let mut closed_polygon: Vec<(Point2<f32>, Point2<f32>)> = vec![];

  if polygon.len() >= 3 {
    for i in 0..polygon.len() - 1 {
      let curr = polygon[i];
      let next = polygon[i + 1];

      closed_polygon.push((curr, next));
    }

    let first = polygon.first().unwrap();
    let last = polygon.last().unwrap();

    closed_polygon.push((*last, *first))
  }

  ObjectBundle {
    position,
    size: Size {
      width: image.width() as f32,
      height: image.height() as f32,
    },
    sprite: Sprite {
      image,
      ysorted: polygon_type != PolygonType::GROUND,
    },
    object: Object {
      polygon: closed_polygon,
      polygon_type,
    },
  }
}
