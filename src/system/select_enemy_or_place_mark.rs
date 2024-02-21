use crate::{
  component::{
    enemy::EnemyComponent, position::PositionComponent, selection::SelectionComponent,
    size::SizeComponent,
  },
  event::select_enemy_or_place_mark::SelectEnemyOrPlaceMark,
  resource::mark::Mark,
};
use bevy_ecs::{
  component::ComponentId,
  event::EventReader,
  system::{Query, ResMut},
};
use macroquad::math::{Rect, Vec2};

pub fn run(
  mut events: EventReader<SelectEnemyOrPlaceMark>,
  mut q1: Query<(&EnemyComponent, &mut SelectionComponent, &PositionComponent, &SizeComponent)>,
  mut mark: ResMut<Mark>,
) {
  for event in events.read() {
    let mut current_selected_enemy_id: Option<ComponentId> = None;
    let mut new_enemy_selected: bool = false;

    // try to select enemy
    for (enemy, mut selection, position, size) in &mut q1 {
      let rect = Rect::new(position.x, position.y, size.width, size.height);

      if selection.active {
        current_selected_enemy_id = Some(enemy.id);
      }

      if rect.contains(Vec2::new(event.x, event.y)) && !selection.active {
        selection.active = true;
        new_enemy_selected = true;
      }
    }

    // deselect current selected enemy
    if new_enemy_selected {
      if let Some(id) = current_selected_enemy_id {
        for (enemy, mut selection, _, _) in &mut q1 {
          if enemy.id == id {
            selection.active = false;
          }
        }
      }
    } else {
      mark.position = Some(Vec2::new(event.x, event.y));
    }
  }
}
