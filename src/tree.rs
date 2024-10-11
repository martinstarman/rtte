use bevy::prelude::*;

use crate::ysort::YSort;

#[derive(Component)]
pub struct Tree;

pub fn tree_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let texture = asset_server.load("tree.png");

  commands.spawn((
    Tree,
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
    YSort { height: 116 },
  ));
}
