use crate::{
  component::{movement::MovementComponent, position::PositionComponent},
  constants::MIN_MOVEMENT_DISTANCE,
};
use bevy_ecs::system::Query;
use maths_rs::Vec2f;
use rapier2d::prelude::nalgebra;
use rapier2d::prelude::vector;
use rapier2d::{control::KinematicCharacterController, pipeline::QueryFilter};

pub fn movement(mut query: Query<(&mut MovementComponent, &mut PositionComponent)>) {
  for (mut movement, mut position) in &mut query {
    if movement.path.len() > 0 {
      let next_position = movement.path[0];
      // let distance = maths_rs::distance::<f32, Vec2f>(
      //   Vec2f::new(next_position.x, next_position.y),
      //   Vec2f::new(position.x, position.y),
      // );

      // if distance < MIN_MOVEMENT_DISTANCE {
      //   position.x = next_position.x;
      //   position.y = next_position.y;
      //   movement.path.remove(0);
      // } else {
      //   position.x += ((next_position.x - position.x) / distance) * movement.speed;
      //   position.y += ((next_position.y - position.y) / distance) * movement.speed;
      // }

      // The translation we would like to apply if there were no obstacles.
      let desired_translation = vector![position.x - next_position.x, position.y - next_position.y];
      // Create the character controller, here with the default configuration.
      let character_controller = KinematicCharacterController::default();
      // Calculate the possible movement.
      let corrected_movement = character_controller.move_shape(
        1. / 60.,        // The timestep length (can be set to SimulationSettings::dt).
        &bodies,         // The RigidBodySet.
        &colliders,      // The ColliderSet.
        &queries,        // The QueryPipeline.
        character_shape, // The character’s shape.
        character_pos,   // The character’s initial position.
        desired_translation,
        QueryFilter::default()
          // Make sure the the character we are trying to move isn’t considered an obstacle.
          .exclude_rigid_body(character_handle),
        |_| {}, // We don’t care about events in this example.
      );

      // TODO: apply the `corrected_movement.translation` to the rigid-body or collider based on the rules described bellow.
    }
  }
}
