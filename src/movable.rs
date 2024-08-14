use bevy::prelude::*;

#[derive(Component)]
pub struct Movable {
  pub path: Vec<Vec2>,
}
