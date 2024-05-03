use bevy_ecs::component::Component;
use rapier2d::{dynamics::RigidBodyHandle, geometry::Collider};

#[derive(Component)]
pub struct BodyComponent {
  pub collider: Collider,
  pub rigid_body_handle: RigidBodyHandle,
}
