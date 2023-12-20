use crate::{
  component::{movement::MovementComponent, position::PositionComponent},
  constants::MIN_MOVEMENT_DISTANCE,
};
use bevy_ecs::system::Query;
use maths_rs::Vec2f;

pub fn run(mut query: Query<(&mut MovementComponent, &mut PositionComponent)>) {
  for (mut movement, mut current_position) in &mut query {
    // move within path
    if movement.current_path.len() > 0 {
      let next_position = movement.current_path[0];
      let distance = maths_rs::distance::<f32, Vec2f>(
        Vec2f::new(next_position.x, next_position.y),
        Vec2f::new(current_position.x, current_position.y),
      );

      if distance < MIN_MOVEMENT_DISTANCE {
        current_position.x = next_position.x;
        current_position.y = next_position.y;
        movement.current_path.remove(0);
      } else {
        current_position.x += (next_position.x - current_position.x) / distance;
        current_position.y += (next_position.y - current_position.y) / distance;
      }
    }

    // reset path if any
    if movement.current_path.len() == 0 && movement.default_path.len() > 0 {
      movement.current_path = movement.default_path.clone();
    }
  }
}