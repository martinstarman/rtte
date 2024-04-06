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
use i_float::{f32_vec::F32Vec, fix_vec::FixVec};
use i_overlay::bool::fill_rule::FillRule;
use i_shape::fix_path::FixPath;
use i_shape::fix_shape::FixShape;
use i_triangle::triangulation::triangulate::Triangulate;
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

    let mut holes: Vec<FixPath> = vec![];

    for (shape, position) in &blocks {
      let mut hole: Vec<FixVec> = vec![];

      for point in &shape.points {
        hole.push(F32Vec::new(point.x + position.x, point.y + position.y).to_fix());
      }

      holes.push(hole);
    }

    let shape = FixShape::new_with_contour_and_holes(
      vec![
        F32Vec::new(0., 0.).to_fix(),
        F32Vec::new(WINDOW_WIDTH as f32, 0.).to_fix(),
        F32Vec::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32).to_fix(),
        F32Vec::new(0., WINDOW_HEIGHT as f32).to_fix(),
      ],
      holes,
    );

    let triangulation = shape.to_triangulation(Some(FillRule::NonZero));

    for i in (0..triangulation.indices.len()).step_by(3) {
      let j = triangulation.indices[i];
      let k = triangulation.indices[i + 1];
      let l = triangulation.indices[i + 2];

      let p = triangulation.points[j].to_f32vec();
      let q = triangulation.points[k].to_f32vec();
      let r = triangulation.points[l].to_f32vec();

      draw_line(p.x - offset.x, p.y - offset.y, q.x - offset.x, q.y - offset.y, 1., NAVMESH_COLOR);
      draw_line(q.x - offset.x, q.y - offset.y, r.x - offset.x, r.y - offset.y, 1., NAVMESH_COLOR);
      draw_line(r.x - offset.x, r.y - offset.y, p.x - offset.x, p.y - offset.y, 1., NAVMESH_COLOR);
    }
  }
}
