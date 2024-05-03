use bevy_ecs::component::Component;
use rapier2d::{dynamics::RigidBodyHandle, geometry::ColliderHandle};

#[derive(Component)]
pub struct BodyComponent {
  pub collider_handle: ColliderHandle,
  pub rigid_body_handle: RigidBodyHandle,
}
