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
  event::select_or_move_player::SelectOrMovePlayer,
};
use bevy_ecs::{component::ComponentId, event::EventReader, query::With, system::Query};
use i_float::{f32_vec::F32Vec, fix_vec::FixVec};
use i_overlay::bool::fill_rule::FillRule;
use i_shape::fix_path::FixPath;
use i_shape::fix_shape::FixShape;
use i_triangle::triangulation::triangulate::Triangulate;
use macroquad::math::{Rect, Vec2};
use navmesh::{NavMesh, NavPathMode, NavQuery, NavTriangle, NavVec3};

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
      let rect = Rect::new(position.x, position.y, size.width, size.height);

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
          shape.r#type == ShapeType::BLOCK || shape.r#type == ShapeType::TRANSPARENT
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
  let mut holes: Vec<FixPath> = vec![];

  for (shape, position) in blocks {
    let mut hole: Vec<FixVec> = vec![];

    for point in &shape.points {
      hole.push(F32Vec::new(point.x + position.x, point.y + position.y).to_fix());
    }

    holes.push(hole);
  }

  let shape = FixShape::new_with_contour_and_holes(
    vec![
      F32Vec::new(0., 0.).to_fix(),
      F32Vec::new(800., 0.).to_fix(),
      F32Vec::new(800., 600.).to_fix(),
      F32Vec::new(0., 600.).to_fix(),
    ],
    holes, // vec![
           //   vec![
           //     F32Vec::new(100., 100.).to_fix(),
           //     F32Vec::new(150., 100.).to_fix(),
           //     F32Vec::new(150., 150.).to_fix(),
           //     F32Vec::new(100., 150.).to_fix(),
           //   ],
           //   vec![
           //     F32Vec::new(50., 50.).to_fix(),
           //     F32Vec::new(80., 50.).to_fix(),
           //     F32Vec::new(80., 80.).to_fix(),
           //     F32Vec::new(50., 80.).to_fix(),
           //   ],
           //   vec![
           //     F32Vec::new(150., 100.).to_fix(),
           //     F32Vec::new(200., 100.).to_fix(),
           //     F32Vec::new(200., 125.).to_fix(),
           //     F32Vec::new(150., 125.).to_fix(),
           //   ],
           //   vec![
           //     F32Vec::new(100., 150.).to_fix(),
           //     F32Vec::new(115., 150.).to_fix(),
           //     F32Vec::new(115., 175.).to_fix(),
           //     F32Vec::new(100., 175.).to_fix(),
           //   ],
           // ],
  );

  let triangulation = shape.to_triangulation(Some(FillRule::NonZero));
  let mut vertices: Vec<NavVec3> = vec![];
  let mut triangles: Vec<NavTriangle> = vec![];

  for i in (0..triangulation.indices.len()).step_by(3) {
    println!(
      "({}, {}, {})",
      triangulation.indices[i],
      triangulation.indices[i + 1],
      triangulation.indices[i + 2]
    );
    triangles.push(NavTriangle {
      first: triangulation.indices[i] as u32,
      second: triangulation.indices[i + 1] as u32,
      third: triangulation.indices[i + 2] as u32,
    })
  }

  for i in 0..triangulation.points.len() {
    println!(
      "({}x{})",
      triangulation.points[i].to_f32vec().x,
      triangulation.points[i].to_f32vec().y,
    );
    vertices.push(NavVec3::new(
      triangulation.points[i].to_f32vec().x,
      triangulation.points[i].to_f32vec().y,
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
