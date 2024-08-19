use bevy::prelude::*;

use crate::ysort::YSort;

#[derive(Component)]
pub struct Building;

pub fn building_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let texture = asset_server.load("building.png");

  commands.spawn((
    Building,
    SpriteBundle {
      texture,
      transform: Transform {
        translation: Vec3 {
          x: 100.,
          y: 100.,
          z: 0.,
        },
        ..default()
      },
      ..default()
    },
    YSort { height: 270 },
  ));
}
