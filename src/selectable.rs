use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Selectable {
  pub selected: bool,
}
