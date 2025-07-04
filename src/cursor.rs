use bevy::{prelude::*, window::SystemCursorIcon, winit::cursor::CursorIcon};

use crate::{action::Action, player::Player, ui::actions::UiAction};

pub fn change_cursor_on_action_select(
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
        .entity(window_query.single().unwrap())
        .insert(ui_action.unwrap().cursor.clone());

      has_custom_cursor = true;
    }

    if !has_custom_cursor {
      commands
        .entity(window_query.single().unwrap())
        .insert(CursorIcon::from(SystemCursorIcon::Default));
    }
  }
}
