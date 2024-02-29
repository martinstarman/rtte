use bevy_ecs::{query::Changed, system::Query};

use crate::component::movement::MovementComponent;

pub fn reset_path(mut query: Query<&mut MovementComponent, Changed<MovementComponent>>) {
  for mut movement in &mut query {
    if movement.path.len() == 0 && movement.default_path.len() > 0 {
      movement.path = movement.default_path.clone();
    }
  }
}
