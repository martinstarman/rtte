use bevy_ecs::system::Resource;
use rapier2d::{dynamics::RigidBodySet, geometry::ColliderSet, pipeline::QueryPipeline};

#[derive(Resource)]
pub struct Physics {
  pub collider_set: ColliderSet,
  pub query_pipeline: QueryPipeline,
  pub rigid_body_set: RigidBodySet,
}
