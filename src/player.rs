use bevy::{prelude::*, window::PrimaryWindow};
use std::{collections::HashMap, time::Duration};

use crate::{camera::MainCamera, direction::Direction};

#[derive(Component)]
pub struct Player {
  state: PlayerState,
}

#[derive(Component)]
pub struct PlayerAnimationConfig {
  pub fps: u8,
  pub frame_timer: Timer,
}

#[derive(Resource)]
pub struct PlayerAtlasConfig {
  map: HashMap<PlayerState, HashMap<Direction, Handle<TextureAtlasLayout>>>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum PlayerState {
  Idle = 1,
  Walk = 2,
}

impl PlayerAnimationConfig {
  fn new(fps: u8) -> Self {
    Self {
      fps,
      frame_timer: Self::timer_from_fps(fps),
    }
  }

  fn timer_from_fps(fps: u8) -> Timer {
    Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
  }
}

pub fn player_setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
  let texture = asset_server.load("player_atlas.png");

  let tile_size = UVec2::new(256, 256);
  let mut atlas_config = HashMap::new();

  let states = [PlayerState::Idle, PlayerState::Walk];

  let directions = [
    Direction::North,
    Direction::NorthEast,
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
  ];

  for (is, state) in states.iter().enumerate() {
    let mut direction_config = HashMap::new();

    for (id, direction) in directions.iter().enumerate() {
      let offset = Some(UVec2::new(id as u32 * 1024, is as u32 * 1024));
      let atlas = TextureAtlasLayout::from_grid(tile_size, 4, 3, None, offset);
      let handle = atlases.add(atlas);
      direction_config.insert(direction.clone(), handle);
    }

    atlas_config.insert(state.clone(), direction_config);
  }

  commands.insert_resource(PlayerAtlasConfig {
    map: atlas_config.clone(),
  });

  commands.spawn((
    Player {
      state: PlayerState::Idle,
    },
    SpriteBundle {
      texture,
      ..default()
    },
    TextureAtlas::from(
      atlas_config
        .get(&PlayerState::Idle)
        .unwrap()
        .clone()
        .get(&Direction::South)
        .unwrap()
        .clone(),
    ),
    PlayerAnimationConfig::new(10),
  ));
}

pub fn player_animation(
  time: Res<Time>,
  mut animation_q: Query<(&mut TextureAtlas, &mut PlayerAnimationConfig), With<Player>>,
) {
  for (mut atlas, mut animation_config) in &mut animation_q {
    animation_config.frame_timer.tick(time.delta());

    if animation_config.frame_timer.just_finished() {
      atlas.index = (atlas.index + 1) % 8;

      animation_config.frame_timer = PlayerAnimationConfig::timer_from_fps(animation_config.fps);
    }
  }
}

pub fn player_direction(
  windows_q: Query<&Window, With<PrimaryWindow>>,
  camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
  mut player_atlas_q: Query<(&mut TextureAtlas, &Player)>,
  atlas_config: Res<PlayerAtlasConfig>,
) {
  let window = windows_q.single();

  if let Some(cursor_position) = window.cursor_position() {
    let (camera, global_transform) = camera_q.single();

    if let Some(position) = camera.viewport_to_world_2d(global_transform, cursor_position) {
      let angle = position.to_angle();
      let direction = Direction::try_from(angle).unwrap();

      let (mut atlas, player) = player_atlas_q.single_mut();
      atlas.layout = atlas_config
        .map
        .get(&player.state)
        .unwrap()
        .clone()
        .get(&direction)
        .unwrap()
        .clone();
    }
  }
}
