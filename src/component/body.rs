use bevy_ecs::component::Component;
use rapier2d::{
  dynamics::{RigidBody, RigidBodyHandle},
  geometry::Collider,
};

#[derive(Component)]
pub struct BodyComponent {
  pub rigid_body: RigidBody,
  pub collider: Collider,
  pub rigid_body_handle: RigidBodyHandle,
}
