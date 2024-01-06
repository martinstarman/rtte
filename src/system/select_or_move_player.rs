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
use maths_rs::vec::Vec2;

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
      let in_object: bool = q2
        .into_iter()
        .filter(|object| {
          object.polygon_type == PolygonType::BLOCK
            || object.polygon_type == PolygonType::TRANSPARENT
        })
        .find(|object| {
          maths_rs::point_inside_polygon(
            maths_rs::vec::Vec2 {
              x: event.x,
              y: event.y,
            },
            &object.polygon.iter().map(|(p, _)| Vec2::new(p.x, p.y)).collect::<Vec<Vec2<f32>>>(),
          )
        })
        .is_some();

      if !in_object {
        for (_, selection, _, _, mut movement) in &mut q1 {
          if selection.active {
            movement.current_path = vec![Point2 {
              x: event.x,
              y: event.y,
            }];
          }
        }
      }
    }
  }
}
