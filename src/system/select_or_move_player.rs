use crate::{
  component::{
    movement::MovementComponent,
    polygon::{PolygonComponent, Type},
    player::PlayerComponent,
    position::PositionComponent,
    selection::SelectionComponent,
    size::SizeComponent,
  },
  event::select_or_move_player::SelectOrMovePlayer,
};
use bevy_ecs::{component::ComponentId, event::EventReader, system::Query};
use ggez::{graphics::Rect, mint::Point2};
use maths_rs::{line_segment_vs_line_segment, vec::Vec3};
use pathfinding::directed::dijkstra::dijkstra;

pub fn run(
  mut events: EventReader<SelectOrMovePlayer>,
  mut q1: Query<(
    &PlayerComponent,
    &mut SelectionComponent,
    &PositionComponent,
    &SizeComponent,
    &mut MovementComponent,
  )>,
  q2: Query<&PolygonComponent>,
) {
  for event in events.read() {
    let mut selected_player_id: Option<ComponentId> = None;

    // try to select player
    for (player, mut selection, position, size, _) in &mut q1 {
      let rect = Rect::new(position.x, position.y, size.width, size.height);

      if rect.contains(Point2 {
        x: event.x,
        y: event.y,
      }) {
        selection.active = true;
        selected_player_id = Some(player.id);
      }
    }

    // deselect all players if some was selected
    if let Some(id) = selected_player_id {
      for (player, mut selection, _, _, _) in &mut q1 {
        if player.id != id {
          selection.active = false;
        }
      }
    }

    // set path to selected player when no player was selected
    if selected_player_id.is_none() {
      let blocks: Vec<&PolygonComponent> = q2
        .into_iter()
        .filter(|block| {
          block.r#type == Type::BLOCK || block.r#type == Type::TRANSPARENT
        })
        .collect();

      let target_point = Point2 {
        x: event.x as i32,
        y: event.y as i32,
      };

      let mut unique_points: Vec<Point2<f32>> = vec![];
      let mut unique_lines: Vec<(Point2<f32>, Point2<f32>)> = vec![];

      for block in &blocks {
        for line in &block.polygon {
          if !unique_points.contains(&line.0) {
            unique_points.push(line.0);
          }

          if !unique_points.contains(&line.1) {
            unique_points.push(line.1);
          }

          if !unique_lines.contains(&(line.0, line.1)) && !unique_lines.contains(&(line.1, line.0))
          {
            unique_lines.push(line.clone());
          }
        }
      }

      for (_, selection, position, _, mut movement) in &mut q1 {
        if selection.active {
          let start_point = Point2 {
            x: position.x as i32,
            y: position.y as i32,
          };

          let mut unique_points_with_player_position = unique_points.clone();

          unique_points_with_player_position.push(Point2 {
            x: position.x,
            y: position.y,
          });

          let target_neighbors =
            get_neighbors(target_point, &unique_points_with_player_position, &unique_lines);

          let path = dijkstra(
            &start_point,
            |&point| {
              if target_neighbors.contains(&(point, 1)) {
                return vec![(target_point, 1)];
              }

              get_neighbors(point, &unique_points, &unique_lines)
            },
            |&point| point.x == target_point.x && point.y == target_point.y,
          );

          if let Some(path) = path {
            let mut path = path.0;

            // remove start position, we are already there
            path.remove(0);

            // path does not include destination point
            path.push(target_point);

            movement.current_path = path
              .into_iter()
              .map(|p| Point2 {
                x: p.x as f32,
                y: p.y as f32,
              })
              .collect::<Vec<Point2<f32>>>();
          }
        }
      }
    }
  }
}

fn get_neighbors(
  point: Point2<i32>,
  unique_points: &Vec<Point2<f32>>,
  unique_lines: &Vec<(Point2<f32>, Point2<f32>)>,
) -> Vec<(Point2<i32>, usize)> {
  let mut neighbors: Vec<(Point2<i32>, usize)> = vec![];

  for unique_point in unique_points {
    let mut has_intersection = false;

    for unique_line in unique_lines {
      let intersection = line_segment_vs_line_segment(
        Vec3 {
          x: point.x as f32,
          y: point.y as f32,
          z: 0.,
        },
        Vec3 {
          x: unique_point.x,
          y: unique_point.y,
          z: 0.,
        },
        Vec3 {
          x: unique_line.0.x,
          y: unique_line.0.y,
          z: 0.,
        },
        Vec3 {
          x: unique_line.1.x,
          y: unique_line.1.y,
          z: 0.,
        },
      );

      if let Some(i) = intersection {
        if !((i.x == unique_line.0.x && i.y == unique_line.0.y)
          || (i.x == unique_line.1.x && i.y == unique_line.1.y))
        {
          has_intersection = true;
        }
      }
    }

    if !has_intersection {
      let neighbor = Point2 {
        x: unique_point.x as i32,
        y: unique_point.y as i32,
      };

      if !neighbors.contains(&(neighbor, 1)) {
        neighbors.push((neighbor, 1));
      }
    }
  }

  neighbors
}
