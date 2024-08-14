use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Movable {
  pub path: Vec<Vec2>,
}
