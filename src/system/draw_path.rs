use crate::{
  component::{movement::MovementComponent, position::PositionComponent},
  constants::{DEBUG, MOVEMENT_PATH_COLOR},
  resource::offset::Offset,
};
use bevy_ecs::system::{Query, Res};
use macroquad::shapes::draw_line;

pub fn draw_path(query: Query<(&MovementComponent, &PositionComponent)>, offset: Res<Offset>) {
  if DEBUG {
    for (movement, position) in &query {
      if movement.path.len() > 0 {
        let p = movement.path[0];
        draw_line(
          position.x - offset.x,
          position.y - offset.y,
          p.x - offset.x,
          p.y - offset.y,
          1.,
          MOVEMENT_PATH_COLOR,
        );

        for i in 0..movement.path.len() - 1 {
          let q = movement.path[i];
          let r = movement.path[i + 1];

          draw_line(
            q.x - offset.x,
            q.y - offset.y,
            r.x - offset.x,
            r.y - offset.y,
            1.,
            MOVEMENT_PATH_COLOR,
          );
        }
      }
    }
  }
}
