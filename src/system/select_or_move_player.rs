use crate::{
  component::{
    movement::MovementComponent,
    object::{ObjectComponent, PolygonType},
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
  q2: Query<&ObjectComponent>,
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
      let objects: Vec<&ObjectComponent> = q2
        .into_iter()
        .filter(|object| {
          object.polygon_type == PolygonType::BLOCK
            || object.polygon_type == PolygonType::TRANSPARENT
        })
        .collect();

      let mut points: Vec<Point2<f32>> = vec![Point2 {
        x: event.x,
        y: event.y,
      }];
      
      for object in &objects {
        for line in &object.polygon {
          if !points.contains(&line.0) {
            points.push(line.0);
          }
          if !points.contains(&line.1) {
            points.push(line.1);
          }
        }
      }

      for (_player, selection, position, _size, mut movement) in &mut q1 {
        if selection.active {
          let start = Point2 {
            x: position.x as i32,
            y: position.y as i32,
          };

          let path = dijkstra(
            &start,
            |&pos| {
              let mut neighbors: Vec<(Point2<i32>, usize)> = vec![];

              for point in &points {
                let mut intersections: Vec<(Point2<f32>, bool)> = vec![];

                for object in &objects {
                  for line in &object.polygon {
                    let intersection = line_segment_vs_line_segment(
                      Vec3 {
                        x: pos.x as f32,
                        y: pos.y as f32,
                        z: 0.,
                      },
                      Vec3 {
                        x: point.x,
                        y: point.y,
                        z: 0.,
                      },
                      Vec3 {
                        x: line.0.x,
                        y: line.0.y,
                        z: 0.,
                      },
                      Vec3 {
                        x: line.1.x,
                        y: line.1.y,
                        z: 0.,
                      },
                    );

                    if let Some(i) = intersection {
                      let is_polygon_point = (i.x == line.0.x && i.y == line.0.y)
                        || (i.x == line.1.x && i.y == line.1.y);

                      let value = (
                        Point2 {
                          x: i.x as f32,
                          y: i.y as f32,
                        },
                        is_polygon_point,
                      );

                      if !intersections.contains(&value) {
                        intersections.push(value);
                      }
                    }
                  }
                }

                if intersections.len() == 0 && point.x == event.x && point.y == event.y {
                  return vec![(
                    Point2 {
                      x: event.x as i32,
                      y: event.y as i32,
                    },
                    1,
                  )];
                }

                if intersections.len() == 1 && intersections.get(0).unwrap().1 == true {
                  neighbors.push((
                    Point2 {
                      x: point.x as i32,
                      y: point.y as i32,
                    },
                    1,
                  ));
                }
              }

              neighbors
            },
            |&pos| pos.x == event.x as i32 && pos.y == event.y as i32,
          );

          if let Some(path) = path {
            let mut path = path.0;

            // remove start position, we are already there
            path.remove(0);

            // path does not include destination point
            path.push(Point2 {
              x: event.x as i32,
              y: event.y as i32,
            });

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
