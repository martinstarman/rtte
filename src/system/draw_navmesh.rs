use crate::{
  component::{
    object::ObjectComponent,
    position::PositionComponent,
    shape::{ShapeComponent, ShapeType},
  },
  constants::{DEBUG, NAVMESH_COLOR, WINDOW_HEIGHT, WINDOW_WIDTH},
  resource::offset::Offset,
};
use bevy_ecs::{
  query::With,
  system::{Query, Res},
};
use i_float::f64_point::F64Point;
use i_overlay::core::fill_rule::FillRule;
use i_triangle::triangulation::float::FloatTriangulate;
use macroquad::shapes::draw_line;

pub fn draw_navmesh(
  query: Query<(&ShapeComponent, &PositionComponent), With<ObjectComponent>>,
  offset: Res<Offset>,
) {
  if DEBUG {
    let blocks: Vec<(&ShapeComponent, &PositionComponent)> = query
      .into_iter()
      .filter(|(shape, _)| {
        shape.r#type == ShapeType::Block || shape.r#type == ShapeType::Transparent
      })
      .collect();

    let mut shapes = vec![vec![
      F64Point::new(0., 0.),
      F64Point::new(WINDOW_WIDTH as f64, 0.),
      F64Point::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64),
      F64Point::new(0., WINDOW_HEIGHT as f64),
    ]];

    for (shape, position) in &blocks {
      let mut hole: Vec<F64Point> = vec![];

      for point in &shape.points {
        hole.push(F64Point::new((point.x + position.x) as f64, (point.y + position.y) as f64));
      }

      shapes.push(hole);
    }

    let triangulation = shapes.to_triangulation(Some(FillRule::EvenOdd), 0.);

    for i in (0..triangulation.indices.len()).step_by(3) {
      let j = triangulation.indices[i];
      let k = triangulation.indices[i + 1];
      let l = triangulation.indices[i + 2];

      let p = triangulation.points[j];
      let q = triangulation.points[k];
      let r = triangulation.points[l];

      draw_line(
        p.x as f32 - offset.x,
        p.y as f32 - offset.y,
        q.x as f32 - offset.x,
        q.y as f32 - offset.y,
        1.,
        NAVMESH_COLOR,
      );
      draw_line(
        q.x as f32 - offset.x,
        q.y as f32 - offset.y,
        r.x as f32 - offset.x,
        r.y as f32 - offset.y,
        1.,
        NAVMESH_COLOR,
      );
      draw_line(
        r.x as f32 - offset.x,
        r.y as f32 - offset.y,
        p.x as f32 - offset.x,
        p.y as f32 - offset.y,
        1.,
        NAVMESH_COLOR,
      );
    }
  }
}
