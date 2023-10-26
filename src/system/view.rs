use crate::{
  component::{
    enemy::Enemy,
    movement::Movement,
    object::{Object, PolygonType},
    position::Position,
    selection::Selection,
    view::{Shift, View},
  },
  resource::mark::Mark,
};
use bevy_ecs::{
  component::ComponentId,
  query::Changed,
  system::{Query, ResMut},
};
use ggez::mint::Point2;
use maths_rs::{vec::Vec2, Vec2f, Vec3f};

const RADIAN: f32 = std::f32::consts::PI / 180.;
const DISTANCE: f32 = 150.;
const INNER_ANGLE: f32 = 60. * RADIAN;
const SHIFT_ANGLE: f32 = 30. * RADIAN;

pub fn update_current_direction(mut query: Query<&mut View>) {
  for mut view in &mut query {
    if view.shift == Shift::LEFT {
      view.current_direction += RADIAN;
    } else {
      view.current_direction -= RADIAN;
    }
  }
}

// TODO: smooth transition
pub fn update_default_direction(mut query: Query<(&mut View, &Movement, &Position), Changed<Movement>>) {
  for (mut view, movement, position) in &mut query {
    if movement.current_path.len() > 0 {
      let rad = f32::atan2(
        movement.current_path[0].y - position.y,
        movement.current_path[0].x - position.x,
      );

      view.default_direction = rad;
      view.current_direction = rad;
    }
  }
}

pub fn update_shift(mut query: Query<&mut View>) {
  for mut view in &mut query {
    let rad = view.current_direction - view.default_direction;

    if rad > SHIFT_ANGLE {
      view.shift = Shift::RIGHT;
    }

    if rad < -SHIFT_ANGLE {
      view.shift = Shift::LEFT;
    }
  }
}

pub fn mark_in_view(mut query: Query<(&View, &mut Selection, &Enemy)>, mut mark: ResMut<Mark>) {
  let mut enemy_id: Option<ComponentId> = None;

  if mark.active {
    for (view, mut selection, enemy) in &mut query {
      if maths_rs::point_inside_polygon(
        Vec2::new(mark.x, mark.y),
        &view.polygon.iter().map(|p| Vec2::new(p.x, p.y)).collect::<Vec<Vec2<f32>>>(),
      ) {
        mark.active = false;
        selection.active = true;
        enemy_id = Some(enemy.id);
      }
    }
  }

  // deselect enemy if view mark was taken by another enemy
  if let Some(id) = enemy_id {
    for (_, mut selection, enemy) in &mut query {
      if enemy.id != id {
        selection.active = false;
      }
    }
  }
}

pub fn update(mut query: Query<(&mut View, &Position)>, blocks_query: Query<(&Object, &Position)>) {
  let blocks: Vec<(&Object, &Position)> =
    blocks_query.iter().filter(|(object, _)| object.polygon_type == PolygonType::BLOCK).collect();

  for (mut view, position) in &mut query {
    let mut points: Vec<Point2<f32>> = vec![];
    let mut rad = view.current_direction - (INNER_ANGLE / 2.);

    while rad < view.current_direction + (INNER_ANGLE / 2.) {
      let mut min_distance = DISTANCE;
      let mut point =
        Vec2f::new(f32::cos(rad) * DISTANCE + position.x, f32::sin(rad) * DISTANCE + position.y);

      for (object, object_position) in &blocks {
        // test all objects polygon lines vs ray (from entity position to view_point)
        for line in &object.polygon {
          if let Some(intersection) = maths_rs::line_segment_vs_line_segment(
            Vec3f::new(position.x, position.y, 0.),
            point.into(),
            Vec3f::new(line.0.x + object_position.x, line.0.y + object_position.y, 0.),
            Vec3f::new(line.1.x + object_position.x, line.1.y + object_position.y, 0.),
          ) {
            // ray was intersected by some line
            let distance = maths_rs::distance::<f32, Vec2f>(
              Vec2f::new(position.x, position.y),
              intersection.into(),
            );

            // save the point if the intersection is closer to entity
            if distance < min_distance {
              point = intersection.into();
              min_distance = distance;
            }
          }
        }
      }

      // add closest point to entity
      points.push(Point2 {
        x: point.x,
        y: point.y,
      });

      rad += RADIAN;
    }

    // close view polygon
    points.push(Point2 {
      x: position.x,
      y: position.y,
    });

    view.polygon = points;
  }
}
