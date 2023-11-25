use bevy_ecs::{
  component::ComponentId,
  system::{Query, ResMut},
};
use maths_rs::vec::Vec2;

use crate::{
  component::{enemy::EnemyComponent, selection::SelectionComponent, view::ViewComponent},
  resource::mark::Mark,
};

pub fn run(
  mut query: Query<(&ViewComponent, &mut SelectionComponent, &EnemyComponent)>,
  mut mark: ResMut<Mark>,
) {
  let mut enemy_id: Option<ComponentId> = None;

  if let Some(position) = mark.position {
    for (view, mut selection, enemy) in &mut query {
      if maths_rs::point_inside_polygon(
        position,
        &view.polygon.iter().map(|p| Vec2::new(p.x, p.y)).collect::<Vec<Vec2<f32>>>(),
      ) {
        mark.position = None;
        selection.active = true;
        enemy_id = Some(enemy.id);
      }
    }
  }

  // deselect enemy if view mark was taken by another enemy
  if let Some(id) = enemy_id {
    for (_, mut selection, enemy) in &mut query {
      if enemy.id != id {
        selection.active = false;
      }
    }
  }
}
