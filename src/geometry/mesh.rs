use crate::geometry::{triangle::Triangle, vec2::Vec2};

use ggez::{graphics::Rect, mint::Point2};
use maths_rs::{deg_to_rad, distance, line_segment_vs_line_segment, rad_to_deg, Vec2f};
use pathfinding::directed::dijkstra::dijkstra;

const POINT_OF_VIEW_ANGLE: i32 = 60;

pub struct Mesh {
  pub rect: Rect,
  pub triangles: Vec<Triangle>,
  pub pov_barriers: Vec<(Vec2, Vec2)>,
}

impl Mesh {
  pub fn new(w: f32, h: f32) -> Self {
    Mesh {
      rect: Rect::new(0., 0., w, h),
      triangles: vec![],
      pov_barriers: vec![],
    }
  }

  //
  pub fn find_path(&self, start: Vec2, dest: Vec2) -> Vec<Vec2> {
    let mut is_straight = true;

    for triangle in &self.triangles {
      if triangle.is_blocking_path() && triangle.intersected(start, dest) {
        is_straight = false; // we intersect some non walkable triangle
      }
    }

    // return straight path if no nonwalkable triangle is in our path
    if is_straight {
      return vec![dest];
    }

    // dx and dy are always in map rect, so we have to find some triangle
    let triangle: &Triangle = self.triangles.iter().find(|t| t.contains(dest)).unwrap();

    if !triangle.is_blocking_path() {
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
          line_segment_vs_line_segment(pos.into(), v.into(), barrier.0.into(), barrier.1.into());

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

  //
  pub fn update_pov_barriers(&mut self) {
    let mut pov_barriers: Vec<(Vec2, Vec2)> = vec![];

    for triangle in &self.triangles {
      if triangle.is_blocking_view() {
        let a = (triangle.a, triangle.b);
        let b = (triangle.b, triangle.c);
        let c = (triangle.c, triangle.a);

        pov_barriers.push(a);
        pov_barriers.push(b);
        pov_barriers.push(c);
      }
    }

    self.pov_barriers = pov_barriers;
  }

  //
  fn get_point_neighbors(&self, point: &Point2<i32>) -> Vec<(Point2<i32>, usize)> {
    let v = Vec2::new(point.x as f32, point.y as f32);
    let mut neighbors: Vec<(Point2<i32>, usize)> = vec![];

    for triangle in &self.triangles {
      if !triangle.is_blocking_path() && triangle.contains((*point).into()) {
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
