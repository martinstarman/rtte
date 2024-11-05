use bevy::{math::bounding::Aabb2d, prelude::*};
use std::collections::HashMap;

use crate::{
  animation::{Animation, AnimationAtlasConfig},
  bounding_box::BoundingBox,
  direction::{Direction, Directions},
  movable::Speed::Slow,
  movable::{Movable, PathItem},
  utils::timer_from_fps,
  ysort::YSort,
};

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Default)]
pub struct EnemyState {
  pub value: EnemyStates,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum EnemyStates {
  #[default]
  Idle = 1,
  Walk = 2,
  Run = 3,
}

pub fn enemy_setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
  let mut atlas_config = HashMap::new();
  let texture = asset_server.load("enemy/export.png");
  let tile_size = UVec2::new(16, 32);
  let directions = vec![
    Directions::East,
    Directions::NorthEast,
    Directions::North,
    Directions::NorthWest,
    Directions::West,
    Directions::SouthWest,
    Directions::South,
    Directions::SouthEast,
  ];

  let mut layouts = HashMap::new();

  for (i, direction) in directions.iter().enumerate() {
    let offset = Some(UVec2::new(0, i as u32 * tile_size.y));
    let atlas = TextureAtlasLayout::from_grid(tile_size, 4, 1, None, offset);
    let handle = atlases.add(atlas);
    layouts.insert(direction.clone(), handle);
  }

  let config = AnimationAtlasConfig {
    fps: 10,
    frame_count: 4,
    layouts,
  };

  atlas_config.insert(EnemyStates::Idle, config);

  let mut layouts = HashMap::new();

  for (i, direction) in directions.iter().enumerate() {
    let offset = Some(UVec2::new(0, (i as u32 * tile_size.y) + 256));
    let atlas = TextureAtlasLayout::from_grid(tile_size, 4, 1, None, offset);
    let handle = atlases.add(atlas);
    layouts.insert(direction.clone(), handle);
  }

  let config = AnimationAtlasConfig {
    fps: 10,
    frame_count: 4,
    layouts,
  };

  atlas_config.insert(EnemyStates::Walk, config);

  let mut layouts = HashMap::new();

  for (i, direction) in directions.iter().enumerate() {
    let offset = Some(UVec2::new(0, (i as u32 * tile_size.y) + 512));
    let atlas = TextureAtlasLayout::from_grid(tile_size, 4, 1, None, offset);
    let handle = atlases.add(atlas);
    layouts.insert(direction.clone(), handle);
  }

  let config = AnimationAtlasConfig {
    fps: 10,
    frame_count: 4,
    layouts,
  };

  atlas_config.insert(EnemyStates::Run, config);

  let default_fps = atlas_config.get(&EnemyStates::Idle).unwrap().fps;
  let default_layout = atlas_config
    .clone()
    .get(&EnemyStates::Idle)
    .unwrap()
    .layouts
    .get(&Directions::South)
    .unwrap()
    .clone();

  commands.insert_resource(Animation {
    frame_timer: timer_from_fps(default_fps),
    atlas_config,
  });

  let path = vec![
    PathItem {
      position: Vec2::new(100., 100.),
      speed: Slow,
    },
    PathItem {
      position: Vec2::new(100., 0.),
      speed: Slow,
    },
    PathItem {
      position: Vec2::new(0., 0.),
      speed: Slow,
    },
  ];

  commands.spawn((
    Enemy,
    Movable {
      path: path.clone(),
      default_path: path.clone(),
    },
    EnemyState::default(),
    Direction::default(),
    SpriteBundle {
      texture,
      ..default()
    },
    TextureAtlas::from(default_layout),
    YSort { height: 32 },
    BoundingBox {
      value: Aabb2d::new(Vec2::new(0., 0.), Vec2::new(8., 16.)),
    },
  ));
}

pub fn enemy_atlas_layout(
  mut query: Query<
    (&EnemyState, &Direction, &mut TextureAtlas),
    (With<Enemy>, Or<(Changed<Direction>, Changed<EnemyState>)>),
  >,
  animation: Res<Animation<EnemyStates>>,
) {
  for (enemy_state, direction, mut atlas) in &mut query {
    atlas.layout = animation
      .atlas_config
      .get(&enemy_state.value)
      .unwrap()
      .layouts
      .get(&direction.value)
      .unwrap()
      .clone();
  }
}

pub fn enemy_state(mut query: Query<(&mut EnemyState, &Movable), Changed<Movable>>) {
  for (mut enemy_state, movable) in &mut query {
    if movable.path.len() == 0 && enemy_state.value != EnemyStates::Idle {
      enemy_state.value = EnemyStates::Idle;
    }

    if movable.path.len() > 0 && enemy_state.value != EnemyStates::Walk {
      enemy_state.value = EnemyStates::Walk;
    }
  }
}
