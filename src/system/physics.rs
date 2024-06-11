use crate::resource::physics::Physics;
use bevy_ecs::system::ResMut;

pub fn physics(mut physics: ResMut<Physics>) {
  let cs = physics.collider_set.clone();
  physics.query_pipeline.update(&cs);
}
