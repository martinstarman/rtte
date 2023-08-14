use crate::geometry::vec2::Vec2;
use bevy_ecs::prelude::*;

// Movable component.
#[derive(Default, Component)]
pub struct Movable {
  pub path: Vec<Vec2>,
}
