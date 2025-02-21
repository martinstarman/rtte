use bevy::prelude::*;

use crate::{
  action::{Action, ActionType},
  player::Player,
  selection::Selection,
};

use super::UI_BG_COLOR;

const UI_ACTION_BG_COLOR_BASE: Color = Color::srgb(0., 0., 0.);

#[derive(Component)]
struct UiAction {
  value: ActionType,
}

#[derive(Component)]
pub struct UiActions;

pub fn ui_actions_setup(mut commands: Commands) {
  commands
    .spawn((
      UiActions,
      Node {
        width: Val::Px(75.),
        height: Val::Percent(100.),
        padding: UiRect::all(Val::Px(5.)),
        position_type: PositionType::Absolute,
        right: Val::Px(0.),
        top: Val::Px(75.),
        ..default()
      },
      BackgroundColor(UI_BG_COLOR),
      Visibility::Hidden,
    ))
    .with_children(|parent| {
      parent
        .spawn((
          UiAction {
            value: ActionType::KnifeMeleeAttack,
          },
          Node {
            width: Val::Percent(100.),
            height: Val::Px(50.),
            margin: UiRect::bottom(Val::Px(5.)),
            ..default()
          },
          BackgroundColor(UI_ACTION_BG_COLOR_BASE),
        ))
        .observe(ui_actions_action_select::<Pointer<Up>>());
    });
}

fn ui_actions_action_select<E>(
) -> impl Fn(Trigger<E>, Query<(Entity, &UiAction)>, Query<(&mut Action, &Selection), With<Player>>)
{
  move |event, ui_query, mut player_action_query| {
    for (entity, ui_action) in &ui_query {
      if entity == event.entity() {
        for (mut action, selection) in &mut player_action_query {
          if selection.active {
            if action.value.is_some() {
              if action.value.unwrap() == ui_action.value {
                action.value = None;
                return;
              }
            }
            action.value = Some(ui_action.value);
          }
        }
      }
    }
  }
}

pub fn ui_actions_visibility(
  mut visibility_query: Query<&mut Visibility, With<UiActions>>,
  selection_query: Query<&Selection, With<Player>>,
) {
  for mut visibility in &mut visibility_query {
    if selection_query.iter().any(|selection| selection.active) {
      *visibility = Visibility::Inherited;
    } else {
      *visibility = Visibility::Hidden;
    }
  }
}
