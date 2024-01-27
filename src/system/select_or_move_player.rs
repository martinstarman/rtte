use crate::{
  component::{
    movement::MovementComponent,
    player::PlayerComponent,
    polygon::{PolygonComponent, Type},
    position::PositionComponent,
    selection::SelectionComponent,
    size::SizeComponent,
  },
  event::select_or_move_player::SelectOrMovePlayer,
};
use bevy_ecs::{component::ComponentId, event::EventReader, system::Query};
use ggez::{graphics::Rect, mint::Point2};
use maths_rs::{distance, line_segment_vs_line_segment, vec::Vec3};
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
        .filter(|block| block.r#type == Type::BLOCK || block.r#type == Type::TRANSPARENT)
        .collect();

      let target_point = Point2 {
        x: event.x as i32,
        y: event.y as i32,
      };

      for (_, selection, position, _, mut movement) in &mut q1 {
        if selection.active {
          let start_point = Point2 {
            x: position.x as i32,
            y: position.y as i32,
          };

          let path = dijkstra(
            &start_point,
            |&point| get_neighbors(point, target_point, blocks.clone()),
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
  target: Point2<i32>,
  blocks: Vec<&PolygonComponent>,
) -> Vec<(Point2<i32>, usize)> {
  let mut neighbors: Vec<(Point2<i32>, usize)> = vec![];

  // check if it is polygon point
  let mut polygon_id: Option<ComponentId> = None;

  for block in &blocks {
    for line in &block.polygon {
      if line.0.x as i32 == point.x && line.0.y as i32 == point.y {
        polygon_id = Some(block.id);

        let neighboor = Point2 {
          x: line.1.x as i32,
          y: line.1.y as i32,
        };

        neighbors.push((
          neighboor,
          distance(
            Vec3 {
              x: point.x as f32,
              y: point.y as f32,
              z: 1.,
            },
            Vec3 {
              x: neighboor.x as f32,
              y: neighboor.y as f32,
              z: 1.,
            },
          ) as usize,
        ));
      }
    }
  }

  // test target
  let mut has_intersection = false;

  for block in &blocks {
    for line in &block.polygon {
      let intersection = line_segment_vs_line_segment(
        Vec3 {
          x: point.x as f32,
          y: point.y as f32,
          z: 0.,
        },
        Vec3 {
          x: target.x as f32,
          y: target.y as f32,
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
        if !((i.x == line.0.x && i.y == line.0.y) || (i.x == line.1.x && i.y == line.1.y)) {
          has_intersection = true;
        }
      }
    }
  }

  if !has_intersection {
    let neighbor = Point2 {
      x: target.x,
      y: target.y,
    };

    neighbors.push((
      neighbor,
      distance(
        Vec3 {
          x: point.x as f32,
          y: point.y as f32,
          z: 1.,
        },
        Vec3 {
          x: target.x as f32,
          y: target.y as f32,
          z: 1.,
        },
      ) as usize,
    ));
  }

  // test all blocks
  for block_a in &blocks {
    for line_a in &block_a.polygon {
      let mut has_intersection = false;

      for block_b in &blocks {
        for line_b in &block_b.polygon {
          let intersection = line_segment_vs_line_segment(
            Vec3 {
              x: point.x as f32,
              y: point.y as f32,
              z: 0.,
            },
            Vec3 {
              x: line_a.0.x,
              y: line_a.0.y,
              z: 0.,
            },
            Vec3 {
              x: line_b.0.x,
              y: line_b.0.y,
              z: 0.,
            },
            Vec3 {
              x: line_b.1.x,
              y: line_b.1.y,
              z: 0.,
            },
          );

          if let Some(i) = intersection {
            if !((i.x == line_b.0.x && i.y == line_b.0.y)
              || (i.x == line_b.1.x && i.y == line_b.1.y))
            {
              has_intersection = true;
            }
          }
        }
      }

      if !has_intersection {
        let neighbor = Point2 {
          x: line_a.0.x as i32,
          y: line_a.0.y as i32,
        };

        let dist = distance(
          Vec3 {
            x: point.x as f32,
            y: point.y as f32,
            z: 1.,
          },
          Vec3 {
            x: neighbor.x as f32,
            y: neighbor.y as f32,
            z: 1.,
          },
        );

        if polygon_id.is_some() {
          if polygon_id.unwrap() != block_a.id {
            neighbors.push((neighbor, dist as usize));
          }
        } else {
          neighbors.push((neighbor, dist as usize));
        }
      }
    }
  }

  neighbors
}
