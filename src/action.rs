use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Action {
  pub value: Option<ActionType>,
}

#[derive(Clone, Copy)]
pub enum ActionType {
  KnifeMeleeAttack,
}
