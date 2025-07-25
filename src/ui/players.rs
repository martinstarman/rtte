use bevy::prelude::*;

use crate::{action::Action, player::Player, selection::Selection};

use super::{UI_BG_COLOR, UI_ITEM_BG_COLOR_BASE, UI_ITEM_BG_COLOR_SELECTED};

#[derive(Component)]
pub struct UiPlayers;

#[derive(Component)]
pub struct UiPlayer {
  player_entity: Entity,
}

pub fn ui_players_init(mut commands: Commands) {
  commands.spawn((
    UiPlayers,
    Node {
      width: Val::Percent(100.),
      height: Val::Px(75.),
      padding: UiRect::all(Val::Px(5.)),
      ..default()
    },
    BackgroundColor(UI_BG_COLOR),
  ));
}

pub fn ui_update_players_on_player_added(
  mut commands: Commands,
  ui_query: Query<Entity, With<UiPlayers>>,
  players_query: Query<(Entity, Ref<Player>)>,
) {
  for (player_entity, player) in &players_query {
    if player.is_added() {
      let entity = ui_query.single().unwrap();

      let child = commands
        .spawn((
          UiPlayer { player_entity },
          Node {
            width: Val::Px(50.),
            height: Val::Percent(100.),
            margin: UiRect::right(Val::Px(5.)),
            ..default()
          },
          BackgroundColor(UI_ITEM_BG_COLOR_BASE),
        ))
        .observe(ui_select_player::<Pointer<Pressed>>())
        .id();

      commands.entity(entity).add_child(child);
    }
  }
}

fn ui_select_player<E>() -> impl Fn(
  Trigger<E>,
  Query<(Entity, &UiPlayer)>,
  Query<(Entity, &mut Selection, &mut Action), With<Player>>,
  ResMut<ButtonInput<MouseButton>>,
) {
  move |event, ui_query, mut selection_query, mut mouse| {
    mouse.clear_just_pressed(MouseButton::Left);

    for (entity, ui_player) in &ui_query {
      if entity == event.target() {
        for (player_entity, mut selection, mut action) in &mut selection_query {
          if ui_player.player_entity == player_entity {
            let is_selection_active = !selection.active;
            selection.active = is_selection_active;

            if !is_selection_active {
              action.value = None;
            }
          } else {
            selection.active = false;
            action.value = None;
          }
        }
      }
    }
  }
}

pub fn ui_draw_players(
  mut ui_query: Query<(&UiPlayer, &mut BackgroundColor)>,
  players_query: Query<(Entity, &Selection), (With<Player>, Changed<Selection>)>,
) {
  for (ui_player, mut background_color) in &mut ui_query {
    for (player_entity, selection) in &players_query {
      if ui_player.player_entity == player_entity {
        background_color.0 = if selection.active {
          UI_ITEM_BG_COLOR_SELECTED
        } else {
          UI_ITEM_BG_COLOR_BASE
        }
      }
    }
  }
}
