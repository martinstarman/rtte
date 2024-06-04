use crate::resource::physics::Physics;
use bevy_ecs::system::ResMut;

pub fn physics(mut physics: ResMut<Physics>) {
  let rbs = physics.rigid_body_set.clone();
  let cs = physics.collider_set.clone();
  physics.query_pipeline.update(&rbs, &cs);
}
