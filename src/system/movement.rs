use crate::{
  component::{movement::MovementComponent, position::PositionComponent},
  constants::MIN_MOVEMENT_DISTANCE,
};
use bevy_ecs::system::Query;
use maths_rs::Vec2f;

pub fn movement(mut query: Query<(&mut MovementComponent, &mut PositionComponent)>) {
  for (mut movement, mut position) in &mut query {
    if movement.path.len() > 0 {
      let next_position = movement.path[0];
      let distance = maths_rs::distance::<f32, Vec2f>(
        Vec2f::new(next_position.x, next_position.y),
        Vec2f::new(position.x, position.y),
      );

      if distance < MIN_MOVEMENT_DISTANCE {
        position.x = next_position.x;
        position.y = next_position.y;
        movement.path.remove(0);
      } else {
        position.x += (next_position.x - position.x) / distance;
        position.y += (next_position.y - position.y) / distance;
      }
    }
  }
}
