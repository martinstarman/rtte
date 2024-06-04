use crate::{
  component::{body::BodyComponent, movement::MovementComponent, position::PositionComponent},
  constants::MIN_MOVEMENT_DISTANCE,
  resource::physics::Physics,
};
use bevy_ecs::system::{Query, ResMut};
use maths_rs::Vec2f;
use rapier2d::prelude::nalgebra;
use rapier2d::prelude::vector;
use rapier2d::{control::KinematicCharacterController, pipeline::QueryFilter};

pub fn movement(
  mut query: Query<(&mut MovementComponent, &mut PositionComponent, &mut BodyComponent)>,
  mut physics: ResMut<Physics>,
) {
  for (mut movement, mut position, body) in &mut query {
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
        let x = ((next_position.x - position.x) / distance) * movement.speed;
        let y = ((next_position.y - position.y) / distance) * movement.speed;

        let collider = &physics.collider_set[body.collider_handle];
        let desired_translation = vector![x, y];
        let character_controller = KinematicCharacterController::default();

        let corrected_movement = character_controller.move_shape(
          1. / 60., // TODO
          &physics.rigid_body_set,
          &physics.collider_set,
          &physics.query_pipeline,
          collider.shape(),
          &vector![position.x, position.y].into(),
          desired_translation,
          QueryFilter::default().exclude_rigid_body(body.rigid_body_handle),
          |_| {},
        );

        position.x += corrected_movement.translation.x;
        position.y += corrected_movement.translation.y;

        let rigid_body = &mut physics.rigid_body_set[body.rigid_body_handle];
        rigid_body.set_next_kinematic_position(vector![position.x, position.y].into());

        // TODO: set_next_kinematic_position should update collider position
        let collider = &mut physics.collider_set[body.collider_handle];
        collider.set_position(vector![position.x, position.y].into());
      }
    }
  }
}
