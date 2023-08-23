use super::{
  movable::Movable, position::Position, renderable::Renderable, selectable::Selectable, size::Size,
  view::View,
};
use bevy_ecs::{prelude::*, component::ComponentId};

/// Enemy component.
#[derive(Component)]
pub struct Enemy {
    /// Enemy id.
    pub id: ComponentId,
}

/// Enemy bundle.
#[derive(Bundle)]
pub struct EnemyBundle {
  /// Enemy.
  pub enemy: Enemy,

  /// Enemy movement.
  pub movable: Movable,

  /// Enemy position.
  pub position: Position,

  /// Enemy rendering.
  pub renderable: Renderable,

  /// Enemy selection.
  pub selectable: Selectable,

  /// Enemy size.
  pub size: Size,

  /// Point of view.
  pub view: View,
}
