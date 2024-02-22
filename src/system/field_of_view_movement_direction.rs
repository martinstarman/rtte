use bevy_ecs::{query::Changed, system::Query};

use crate::component::{
  field_of_view::FieldOfViewComponent, movement::MovementComponent, position::PositionComponent,
};

pub fn field_of_view_movement_direction(
  mut query: Query<
    (&mut FieldOfViewComponent, &MovementComponent, &PositionComponent),
    Changed<MovementComponent>,
  >,
) {
  for (mut view, movement, position) in &mut query {
    if movement.current_path.len() > 0 {
      let rad = f32::atan2(
        movement.current_path[0].y - position.y,
        movement.current_path[0].x - position.x,
      );

      view.movement_direction = rad;
      view.direction = rad;
    }
  }
}
