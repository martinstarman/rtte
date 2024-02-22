use bevy_ecs::{query::Changed, system::Query};

use crate::component::{
  movement::MovementComponent, position::PositionComponent, view::ViewComponent,
};

// TODO: smooth transition
// TODO: fov_movement_direction
pub fn run(
  mut query: Query<
    (&mut ViewComponent, &MovementComponent, &PositionComponent),
    Changed<MovementComponent>,
  >,
) {
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
