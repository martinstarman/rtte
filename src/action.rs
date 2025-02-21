use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Action {
  pub value: Option<ActionType>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ActionType {
  KnifeMeleeAttack,
}
