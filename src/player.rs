use bevy::{prelude::*, window::PrimaryWindow};
use std::{collections::HashMap, time::Duration};

use crate::{camera::MainCamera, direction::Direction};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerAnimationConfig {
  pub fps: u8,
  pub frame_timer: Timer,
}

#[derive(Resource)]
pub struct PlayerAtlasConfig {
  map: HashMap<Direction, Handle<TextureAtlasLayout>>,
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
  let texture = asset_server.load("player_walk.png");

  let tile_size = UVec2::new(256, 256);
  let mut atlas_config = HashMap::new();

  let atlas = TextureAtlasLayout::from_grid(tile_size, 4, 3, None, None);
  let handle = atlases.add(atlas);
  atlas_config.insert(Direction::North, handle);

  let atlas = TextureAtlasLayout::from_grid(tile_size, 4, 3, None, Some(UVec2::new(1024, 0)));
  let handle = atlases.add(atlas);
  atlas_config.insert(Direction::NorthEast, handle);

  let atlas = TextureAtlasLayout::from_grid(tile_size, 4, 3, None, Some(UVec2::new(2048, 0)));
  let handle = atlases.add(atlas);
  atlas_config.insert(Direction::East, handle);

  let atlas = TextureAtlasLayout::from_grid(tile_size, 4, 3, None, Some(UVec2::new(3072, 0)));
  let handle = atlases.add(atlas);
  atlas_config.insert(Direction::SouthEast, handle);

  let atlas = TextureAtlasLayout::from_grid(tile_size, 4, 3, None, Some(UVec2::new(4096, 0)));
  let handle = atlases.add(atlas);
  atlas_config.insert(Direction::South, handle);

  let atlas = TextureAtlasLayout::from_grid(tile_size, 4, 3, None, Some(UVec2::new(5120, 0)));
  let handle = atlases.add(atlas);
  atlas_config.insert(Direction::SouthWest, handle);

  let atlas = TextureAtlasLayout::from_grid(tile_size, 4, 3, None, Some(UVec2::new(6144, 0)));
  let handle = atlases.add(atlas);
  atlas_config.insert(Direction::West, handle);

  let atlas = TextureAtlasLayout::from_grid(tile_size, 4, 3, None, Some(UVec2::new(7168, 0)));
  let handle = atlases.add(atlas);
  atlas_config.insert(Direction::NorthWest, handle);

  commands.insert_resource(PlayerAtlasConfig {
    map: atlas_config.clone(),
  });

  commands.spawn((
    Player,
    SpriteBundle {
      texture,
      ..default()
    },
    TextureAtlas::from(atlas_config.get(&Direction::South).unwrap().clone()),
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
  mut atlas_q: Query<&mut TextureAtlas, With<Player>>,
  atlas_config: Res<PlayerAtlasConfig>,
) {
  let window = windows_q.single();

  if let Some(cursor_position) = window.cursor_position() {
    let (camera, global_transform) = camera_q.single();

    if let Some(position) = camera.viewport_to_world_2d(global_transform, cursor_position) {
      let angle = position.to_angle();
      let direction = Direction::try_from(angle).unwrap();

      let mut atlas = atlas_q.single_mut();
      atlas.layout = atlas_config.map.get(&direction).unwrap().clone();
    }
  }
}
