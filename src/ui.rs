use bevy::prelude::*;

pub fn ui_setup(mut commands: Commands) {
  commands
    .spawn(NodeBundle {
      style: Style {
        width: Val::Percent(100.0),
        height: Val::Px(75.0),
        padding: UiRect::all(Val::Px(5.)),
        ..default()
      },
      background_color: Color::srgba(0.65, 0.65, 0.65, 0.5).into(),
      ..default()
    })
    .with_children(|parent| {
      parent.spawn(NodeBundle {
        style: Style {
          width: Val::Px(50.),
          height: Val::Percent(100.),
          margin: UiRect::right(Val::Px(5.)),
          ..default()
        },
        background_color: Color::srgb(0., 0., 0.).into(),
        ..default()
      });

      parent.spawn(NodeBundle {
        style: Style {
          width: Val::Px(50.),
          height: Val::Percent(100.),
          margin: UiRect::right(Val::Px(5.)),
          ..default()
        },
        background_color: Color::srgb(0., 0., 0.).into(),
        ..default()
      });
    });
}
