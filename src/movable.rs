use bevy::prelude::*;

pub struct PathItem<T> {
  pub next: Vec2,
  pub state: T,
}

#[derive(Component, Default)]
pub struct Movable<T> {
  pub path: Vec<PathItem<T>>,
}
