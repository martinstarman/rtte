use bevy::{math::bounding::Aabb2d, prelude::*, window::PrimaryWindow};
use std::collections::HashMap;
use vleue_navigator::NavMesh;

use crate::{
  animation::{Animation, AnimationAtlasConfig},
  bounding_box::BoundingBox,
  camera::MainCamera,
  direction::{Direction, Directions},
  movable::{Movable, PathItem, Speed},
  utils::timer_from_fps,
  ysort::YSort,
};

#[derive(Component, PartialEq, Eq, Hash)]
pub struct Player;

#[derive(Component, Default)]
pub struct PlayerState {
  pub value: PlayerStates,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PlayerStates {
  #[default]
  Idle = 1,
  Walk = 2,
  Run = 3,
}

pub fn player_setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
  let mut atlas_config = HashMap::new();
  let texture = asset_server.load("player/export.png");
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

  atlas_config.insert(PlayerStates::Idle, config);

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

  atlas_config.insert(PlayerStates::Walk, config);

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

  atlas_config.insert(PlayerStates::Run, config);

  let default_fps = atlas_config.get(&PlayerStates::Idle).unwrap().fps;
  let default_layout = atlas_config
    .clone()
    .get(&PlayerStates::Idle)
    .unwrap()
    .layouts
    .get(&Directions::South)
    .unwrap()
    .clone();

  commands.insert_resource(Animation {
    frame_timer: timer_from_fps(default_fps),
    atlas_config,
  });

  commands.spawn((
    Player,
    Movable::default(),
    PlayerState::default(),
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

pub fn player_animation(
  mut query: Query<(&PlayerState, &mut TextureAtlas), With<Player>>,
  mut animation: ResMut<Animation<PlayerStates>>,
  time: Res<Time>,
) {
  for (player_state, mut atlas) in &mut query {
    animation.frame_timer.tick(time.delta());

    if animation.frame_timer.just_finished() {
      let atlas_config = animation.atlas_config.get(&player_state.value).unwrap();
      atlas.index = (atlas.index + 1) % (atlas_config.frame_count as usize - 1);
      animation.frame_timer = timer_from_fps(atlas_config.fps);
    }
  }
}

pub fn player_atlas_layout(
  mut query: Query<
    (&PlayerState, &Direction, &mut TextureAtlas),
    (With<Player>, Or<(Changed<Direction>, Changed<PlayerState>)>),
  >,
  animation: Res<Animation<PlayerStates>>,
) {
  for (player_state, direction, mut atlas) in &mut query {
    atlas.layout = animation
      .atlas_config
      .get(&player_state.value)
      .unwrap()
      .layouts
      .get(&direction.value)
      .unwrap()
      .clone();
  }
}

pub fn player_path(
  mut query: Query<(&mut Movable, &Transform), With<Player>>,
  navmeshes: Res<Assets<NavMesh>>,
  navmesh: Query<&Handle<NavMesh>>,
  buttons: Res<ButtonInput<MouseButton>>,
  windows: Query<&Window, With<PrimaryWindow>>,
  camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
  keys: Res<ButtonInput<KeyCode>>,
) {
  if buttons.just_pressed(MouseButton::Left) {
    let window = windows.single();

    if let Some(cursor_position) = window.cursor_position() {
      let (camera, global_transform) = camera_q.single();

      if let Some(position) = camera.viewport_to_world_2d(global_transform, cursor_position) {
        for (mut movable, transform) in &mut query {
          let Some(navmesh) = navmeshes.get(navmesh.single()) else {
            continue;
          };

          let Some(path) = navmesh.transformed_path(
            transform.translation.xyz(),
            navmesh.transform().transform_point(position.extend(0.)),
          ) else {
            break;
          };

          movable.path = path
            .path
            .iter()
            .map(|v| PathItem {
              position: v.xy(),
              speed: if keys.pressed(KeyCode::ShiftLeft) {
                Speed::Fast
              } else {
                Speed::Slow
              },
            })
            .collect();
        }
      }
    }
  }

  if buttons.just_pressed(MouseButton::Right) {
    for (mut movable, _) in &mut query {
      movable.path = vec![];
    }
  }
}

pub fn player_state(mut query: Query<(&mut PlayerState, &Movable), Changed<Movable>>) {
  for (mut player_state, movable) in &mut query {
    if movable.path.len() == 0 && player_state.value != PlayerStates::Idle {
      player_state.value = PlayerStates::Idle;
    }

    if movable.path.len() > 0 {
      player_state.value = if movable.path[0].speed == Speed::Slow {
        PlayerStates::Walk
      } else {
        PlayerStates::Run
      };
    }
  }
}
