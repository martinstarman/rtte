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
  for (mut field_of_view, movement, position) in &mut query {
    if movement.path.len() > 0 {
      let angle = f32::atan2(movement.path[0].y - position.y, movement.path[0].x - position.x);

      field_of_view.movement_direction = angle;
      field_of_view.direction = angle;
    }
  }
}
