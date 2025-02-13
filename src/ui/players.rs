use bevy::prelude::*;

use crate::{player::Player, selection::Selection};

const UI_PLAYER_BG_COLOR_BASE: Color = Color::srgb(0., 0., 0.);
const UI_PLAYER_BG_COLOR_SELECTED: Color = Color::srgb(1., 0., 0.);

#[derive(Component)]
pub struct UiPlayers;

#[derive(Component)]
pub struct UiPlayer {
  player_entity: Entity,
}

pub fn ui_players_setup(mut commands: Commands) {
  commands.spawn((
    UiPlayers,
    Node {
      width: Val::Percent(100.0),
      height: Val::Px(75.0),
      padding: UiRect::all(Val::Px(5.)),
      ..default()
    },
    BackgroundColor(Color::srgba(0.65, 0.65, 0.65, 0.1)),
  ));
}

pub fn ui_players_player_added(
  mut commands: Commands,
  ui_query: Query<Entity, With<UiPlayers>>,
  players_query: Query<(Entity, Ref<Player>)>,
) {
  for (player_entity, player) in &players_query {
    if player.is_added() {
      let entity = ui_query.single();

      let child = commands
        .spawn((
          UiPlayer { player_entity },
          Node {
            width: Val::Px(50.),
            height: Val::Percent(100.),
            margin: UiRect::right(Val::Px(5.)),
            ..default()
          },
          BackgroundColor(UI_PLAYER_BG_COLOR_BASE),
        ))
        .observe(ui_players_player_select::<Pointer<Up>>())
        .id();

      commands.entity(entity).add_child(child);
    }
  }
}

fn ui_players_player_select<E>(
) -> impl Fn(Trigger<E>, Query<(Entity, &UiPlayer)>, Query<(Entity, &mut Selection), With<Player>>)
{
  move |event, ui_query, mut selection_query| {
    for (entity, ui_player) in &ui_query {
      if entity == event.entity() {
        for (player_entity, mut selection) in &mut selection_query {
          selection.active = ui_player.player_entity == player_entity;
        }
      }
    }
  }
}

pub fn ui_players_selection(
  mut ui_query: Query<(&UiPlayer, &mut BackgroundColor)>,
  players_query: Query<(Entity, &Selection), (With<Player>, Changed<Selection>)>,
) {
  for (ui_player, mut background_color) in &mut ui_query {
    for (player_entity, selection) in &players_query {
      if ui_player.player_entity == player_entity {
        background_color.0 = if selection.active {
          UI_PLAYER_BG_COLOR_SELECTED
        } else {
          UI_PLAYER_BG_COLOR_BASE
        }
      }
    }
  }
}
