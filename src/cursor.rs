use bevy::{prelude::*, window::SystemCursorIcon, winit::cursor::CursorIcon};

use crate::{action::Action, player::Player, ui::actions::UiAction};

pub fn cursor_change(
  mut commands: Commands,
  window_query: Query<Entity, With<Window>>,
  ui_action_query: Query<&UiAction>,
  player_action_query: Query<&Action, (With<Player>, Changed<Action>)>,
) {
  for action in &player_action_query {
    let mut has_custom_cursor = false;

    if action.value.is_some() {
      let ui_action = ui_action_query
        .iter()
        .find(|ui_action| ui_action.value == action.value.unwrap());

      commands
        .entity(window_query.single())
        .insert(ui_action.unwrap().cursor.clone());

      has_custom_cursor = true;
    }

    if !has_custom_cursor {
      commands
        .entity(window_query.single())
        .insert(CursorIcon::from(SystemCursorIcon::Default));
    }
  }
}
