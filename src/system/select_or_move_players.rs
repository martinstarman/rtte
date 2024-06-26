use crate::{
  component::{
    movement::MovementComponent,
    object::ObjectComponent,
    player::PlayerComponent,
    position::PositionComponent,
    selection::SelectionComponent,
    shape::{ShapeComponent, ShapeType},
    size::SizeComponent,
  },
  constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
  event::select_or_move_player::SelectOrMovePlayer,
};
use bevy_ecs::{component::ComponentId, event::EventReader, query::With, system::Query};
use i_float::f64_point::F64Point;
use i_overlay::core::fill_rule::FillRule;
use macroquad::math::{Rect, Vec2};
use navmesh::{NavMesh, NavPathMode, NavQuery, NavTriangle, NavVec3};

use i_triangle::triangulation::float::FloatTriangulate;

pub fn select_or_move_players(
  mut events: EventReader<SelectOrMovePlayer>,
  mut query1: Query<(
    &PlayerComponent,
    &mut SelectionComponent,
    &PositionComponent,
    &SizeComponent,
    &mut MovementComponent,
  )>,
  query2: Query<(&ShapeComponent, &PositionComponent), With<ObjectComponent>>,
) {
  for event in events.read() {
    let mut selected_player_id: Option<ComponentId> = None;

    // try to select player
    for (player, mut selection, position, size, _) in &mut query1 {
      let rect = Rect::new(
        position.x - (size.width / 2.),
        position.y - (size.height / 2.),
        size.width,
        size.height,
      );

      if rect.contains(Vec2::new(event.x, event.y)) {
        selection.active = true;
        selected_player_id = Some(player.id);
      }
    }

    // deselect all players if some was selected
    if let Some(id) = selected_player_id {
      for (player, mut selection, _, _, _) in &mut query1 {
        if player.id != id {
          selection.active = false;
        }
      }
    }

    // set path to selected player when no player was selected
    if selected_player_id.is_none() {
      let blocks: Vec<(&ShapeComponent, &PositionComponent)> = query2
        .into_iter()
        .filter(|(shape, _)| {
          shape.r#type == ShapeType::Block || shape.r#type == ShapeType::Transparent
        })
        .collect();

      let to = Vec2::new(event.x, event.y);

      for (_, selection, position, _, mut movement) in &mut query1 {
        if selection.active {
          let from = Vec2::new(position.x, position.y);
          movement.path = find_path(from, to, &blocks);
        }
      }
    }
  }
}

fn find_path(
  from: Vec2,
  to: Vec2,
  blocks: &Vec<(&ShapeComponent, &PositionComponent)>,
) -> Vec<Vec2> {
  let mut shapes = vec![vec![
    F64Point::new(0., 0.),
    F64Point::new(WINDOW_WIDTH as f64, 0.),
    F64Point::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64),
    F64Point::new(0., WINDOW_HEIGHT as f64),
  ]];

  for (shape, position) in blocks {
    let mut hole: Vec<F64Point> = vec![];

    for point in &shape.points {
      hole.push(F64Point::new((point.x + position.x) as f64, (point.y + position.y) as f64));
    }

    shapes.push(hole);
  }

  let triangulation = shapes.to_triangulation(Some(FillRule::EvenOdd), 0.);

  let mut vertices: Vec<NavVec3> = vec![];
  let mut triangles: Vec<NavTriangle> = vec![];

  for i in (0..triangulation.indices.len()).step_by(3) {
    triangles.push(NavTriangle {
      first: triangulation.indices[i] as u32,
      second: triangulation.indices[i + 1] as u32,
      third: triangulation.indices[i + 2] as u32,
    })
  }

  for i in 0..triangulation.points.len() {
    vertices.push(NavVec3::new(
      triangulation.points[i].x as f32,
      triangulation.points[i].y as f32,
      0.,
    ));
  }

  let mesh = NavMesh::new(vertices, triangles).unwrap();

  let res = mesh.find_path(
    (from.x, from.y, 0.0).into(),
    (to.x, to.y, 0.0).into(),
    NavQuery::Accuracy,
    NavPathMode::Accuracy,
  );

  if let Some(path) = res {
    return path.into_iter().map(|v| Vec2::new(v.x, v.y)).collect();
  }

  vec![]
}
