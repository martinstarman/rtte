use crate::{
  geometry::{line_segment::LineSegment, triangle::Triangle, vec2::Vec2},
  Mode, State,
};

use delaunator::Point;
use ggez::{
  graphics::{self, Canvas, Color, DrawMode, DrawParam, Rect},
  mint::Point2,
  Context,
};
use maths_rs::{deg_to_rad, distance, line_segment_vs_line_segment, rad_to_deg, Vec2f};
use pathfinding::directed::dijkstra::dijkstra;
use serde::{Deserialize, Serialize};

const POINT_OF_VIEW_ANGLE: i32 = 60;

#[derive(Serialize, Deserialize)]
pub struct Mesh {
  pub rect: Rect,
  pub vectors: Vec<Vec2>,
  pub triangles: Vec<Triangle>,
  pub pov_barriers: Vec<LineSegment>,
}

impl Mesh {
  pub fn new(w: f32, h: f32) -> Self {
    let v1 = Vec2::new(0., 0.);
    let v2 = Vec2::new(w, 0.);
    let v3 = Vec2::new(w, h);
    let v4 = Vec2::new(0., h);

    let mut mesh = Mesh {
      rect: Rect::new(0., 0., w, h),
      vectors: vec![v1, v2, v3, v4],
      triangles: vec![],
      pov_barriers: vec![],
    };

    mesh.triangulate();

    mesh
  }

  //
  pub fn add_vec2(&mut self, v: Vec2) {
    self.vectors.push(v);
    self.triangulate();
  }

  //
  pub fn find_path(&self, start: Vec2, dest: Vec2) -> Vec<Vec2> {
    let mut is_straight = true;

    for triangle in &self.triangles {
      if triangle.is_blocking_path && triangle.intersected(LineSegment::new(start, dest)) {
        is_straight = false; // we intersect some non walkable triangle
      }
    }

    // return straight path if no nonwalkable triangle is in our path
    if is_straight {
      return vec![dest];
    }

    // dx and dy are always in map rect, so we have to find some triangle
    let triangle: &Triangle = self.triangles.iter().find(|t| t.contains(dest)).unwrap();

    if !triangle.is_blocking_path {
      let path = dijkstra(
        &start.into(),
        |&point| self.get_point_neighbors(&point),
        |&point| triangle.contains(point.into()),
      );

      if let Some(path) = path {
        let mut path = path.0;

        // remove start position, we are already there
        path.remove(0);

        // path does not include destination point
        path.push(dest.clone().into());

        return path.into_iter().map(|p| p.into()).collect::<Vec<Vec2>>();
      }
    }

    vec![]
  }

  //
  pub fn get_pov(&self, pos: Vec2, dest: Vec2) -> Vec<Vec2> {
    let mut point_of_view: Vec<Vec2> = vec![pos];

    let d = dest - pos;
    let current_angle = rad_to_deg(f32::atan2(d.y, d.x)) as i32;
    let min_angle = current_angle - (POINT_OF_VIEW_ANGLE / 2);
    let max_angle = min_angle + POINT_OF_VIEW_ANGLE;

    for angle in min_angle..max_angle {
      let rad = deg_to_rad(angle as f32);

      let mut v = Vec2::new(
        f32::cos(rad) * d.x - f32::sin(rad) * d.y + pos.x,
        f32::sin(rad) * d.x + f32::cos(rad) * d.y + pos.y,
      );

      let mut min_distance = distance::<f32, Vec2f>(pos.into(), v.into());

      for barrier in &self.pov_barriers {
        let intersection =
          line_segment_vs_line_segment(pos.into(), v.into(), barrier.a.into(), barrier.b.into());

        if intersection.is_some() {
          let intersection = intersection.unwrap();
          let w = Vec2::new(intersection.x, intersection.y);
          let distance = distance::<f32, Vec2f>(pos.into(), w.into());

          if distance < min_distance {
            v = w;
            min_distance = distance;
          }
        }
      }

      point_of_view.push(v);
    }

    point_of_view
  }

  // draw mesh
  pub fn draw(&self, canvas: &mut Canvas, ctx: &mut Context, state: &State) {
    if state.mode == Mode::Edit {
      // draw triangles
      for triangle in &self.triangles {
        let points = vec![triangle.a, triangle.b, triangle.c];

        // bg
        if triangle.is_blocking_path || triangle.is_blocking_view {
          let mesh =
            graphics::Mesh::new_polygon(ctx, DrawMode::fill(), &points[..], Color::GREEN).unwrap();
          canvas.draw(&mesh, DrawParam::new().offset(state.offset).scale(state.scale));
        }

        // outline
        let color = if triangle.is_selected {
          Color::WHITE
        } else {
          Color::BLACK
        };
        let mesh =
          graphics::Mesh::new_polygon(ctx, DrawMode::stroke(1.), &points[..], color).unwrap();
        canvas.draw(&mesh, DrawParam::new().offset(state.offset).scale(state.scale));
      }
    }
  }

  //
  pub fn update_pov_barriers(&mut self) {
    let mut pov_barriers: Vec<LineSegment> = vec![];

    for triangle in &self.triangles {
      if triangle.is_blocking_view {
        let a = LineSegment::new(triangle.a, triangle.b);
        let b = LineSegment::new(triangle.b, triangle.c);
        let c = LineSegment::new(triangle.c, triangle.a);

        pov_barriers.push(a);
        pov_barriers.push(b);
        pov_barriers.push(c);
      }
    }

    self.pov_barriers = pov_barriers;
  }

  //
  fn triangulate(&mut self) {
    let mut triangles: Vec<Triangle> = vec![];
    let points: Vec<Point> = self.vectors.iter().map(|&v| v.into()).collect();
    let res = delaunator::triangulate(&points);

    for i in (0..res.triangles.len()).step_by(3) {
      let a = self.vectors[res.triangles[i]];
      let b = self.vectors[res.triangles[i + 1]];
      let c = self.vectors[res.triangles[i + 2]];

      triangles.push(Triangle::new(a, b, c));
    }

    self.triangles = triangles;
    self.update_pov_barriers();
  }

  //
  fn get_point_neighbors(&self, point: &Point2<i32>) -> Vec<(Point2<i32>, usize)> {
    let v = Vec2::new(point.x as f32, point.y as f32);
    let mut neighbors: Vec<(Point2<i32>, usize)> = vec![];

    for triangle in &self.triangles {
      if !triangle.is_blocking_path && triangle.contains((*point).into()) {
        neighbors
          .push((triangle.a.into(), distance::<f32, Vec2f>(v.into(), triangle.a.into()) as usize));
        neighbors
          .push((triangle.b.into(), distance::<f32, Vec2f>(v.into(), triangle.b.into()) as usize));
        neighbors
          .push((triangle.c.into(), distance::<f32, Vec2f>(v.into(), triangle.c.into()) as usize));
      }
    }

    neighbors
  }
}

// TODO: tests
// #[cfg(test)]
// mod test {
//   use super::*;

//   #[test]
//   fn default() {}
// }
