use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Selection {
  pub active: bool,
}
