use bevy_ecs::{
  component::ComponentId,
  system::{Query, ResMut},
};
use maths_rs::vec::Vec2;

use crate::{
  component::{
    enemy::EnemyComponent, field_of_view::FieldOfViewComponent, selection::SelectionComponent,
  },
  resource::mark::Mark,
};

pub fn run(
  mut query: Query<(&FieldOfViewComponent, &mut SelectionComponent, &EnemyComponent)>,
  mut view_mark: ResMut<Mark>,
) {
  let mut enemy_id: Option<ComponentId> = None;

  if let Some(position) = view_mark.position {
    for (view, mut selection, enemy) in &mut query {
      if maths_rs::point_inside_polygon(
        Vec2::new(position.x, position.y),
        &view.points.iter().map(|p| Vec2::new(p.x, p.y)).collect::<Vec<Vec2<f32>>>(),
      ) {
        view_mark.position = None;
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
